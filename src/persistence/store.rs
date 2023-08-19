use crate::package::{self, CoreMetadata};
use crate::simple_api::{PackageError, PkgDist, ProjectName, SimpleStore};

use anyhow::Result;
use bytes::Bytes;
use regex::Regex;
use sqlx::PgPool;
use std::sync::Arc;

use async_trait::async_trait;
use object_store::{path::Path, ObjectStore};

#[derive(sqlx::Type)]
#[sqlx(type_name = "packagetype")]
#[sqlx(rename_all = "snake_case")]
enum PackageType {
    BdistDmg,
    BdistDumb,
    BdistEgg,
    BdistMsi,
    BdistRpm,
    BdistWheel,
    BdistWininst,
    Sdist,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "dependency_kind")]
#[sqlx(rename_all = "snake_case")]
enum DependencyKind {
    Requires,
    Provides,
    Obsoletes,
    RequiresDist,
    ProvidesDist,
    ObsoletesDist,
    RequiresExternal,
}

impl sqlx::postgres::PgHasArrayType for DependencyKind {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_dependency_kind")
    }
}

struct Dependency {
    pub kind: DependencyKind,
    pub specifier: String,
}

impl package::CoreMetadata {
    fn get_dependencies(self) -> Vec<Dependency> {
        // TODO: Check if Requires and Provides are part of PEP specifications.

        let mut dependencies = Vec::new();

        for value in &self.requires_dists {
            dependencies.push(Dependency {
                kind: DependencyKind::RequiresDist,
                specifier: value.clone(),
            })
        }
        for value in &self.provides_dists {
            dependencies.push(Dependency {
                kind: DependencyKind::ProvidesDist,
                specifier: value.clone(),
            })
        }

        for value in &self.obsoletes_dists {
            dependencies.push(Dependency {
                kind: DependencyKind::ObsoletesDist,
                specifier: value.clone(),
            })
        }

        for value in &self.requires_externals {
            dependencies.push(Dependency {
                kind: DependencyKind::RequiresExternal,
                specifier: value.clone(),
            })
        }

        dependencies
    }
}

fn canonicalize_version(version: &str, strip_trailing_zero: bool) -> String {
    let mut parts = Vec::new();

    let re = Regex::new(r"^(\d+):?((\d+)(\.(\d+))*)([.-].+)?$").unwrap();
    if let Some(captures) = re.captures(version) {
        if let Some(epoch) = captures.get(1) {
            let epoch_value = epoch.as_str();
            if !epoch_value.is_empty() {
                parts.push(format!("{}!", epoch_value));
            }
        }

        if let Some(release) = captures.get(2) {
            let release_value = release.as_str();
            if strip_trailing_zero {
                let re_trailing_zero = Regex::new(r"(\.0)+$").unwrap();

                let normalized_release = re_trailing_zero.replace_all(release_value, "");
                parts.push(normalized_release.to_string());
            } else {
                parts.push(release_value.to_string());
            }
        }

        if let Some(pre_release) = captures.get(6) {
            let pre_release_value = pre_release.as_str();
            parts.push(pre_release_value.to_string());
        }
    } else {
        // Invalid version format, return the original input
        return version.to_string();
    }

    parts.join("")
}

#[derive(Clone)]
pub struct Store {
    db: PgPool,
    store: Arc<dyn ObjectStore>,
}

impl Store {
    pub fn new(db: PgPool, store: Arc<dyn ObjectStore>) -> Store {
        Store { db, store }
    }

    async fn create_project(&self, project_name: &str) -> Result<(), PackageError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO projects (name, normalized_name)
            VALUES ($1, normalize_pep426_name($1))
            "#,
            project_name,
        )
        .execute(&self.db)
        .await;

        if query.is_err() {
            return Err(PackageError);
        }

        Ok(())
    }

    async fn project_exists(&self, project_name: &str) -> Result<bool, PackageError> {
        let query = sqlx::query!(
            r#"
            SELECT name, normalized_name
            FROM projects
            WHERE normalized_name = normalize_pep426_name($1)
            "#,
            project_name,
        )
        .fetch_optional(&self.db)
        .await;

        if let Ok(q) = query {
            match q {
                Some(_) => Ok(true),
                _ => Ok(false),
            }
        } else {
            return Err(PackageError);
        }
    }

    async fn save_file_distribution(
        &self,
        project_name: &str,
        dist_name: &str,
        dist_content: &Bytes,
    ) -> Result<(), PackageError> {
        let file_path = Path::from_iter(["simple-index", project_name, dist_name]);

        let query = self.store.put(&file_path, dist_content.to_owned()).await;

        if query.is_err() {
            return Err(PackageError);
        }

        Ok(())
    }
}

#[async_trait]
impl SimpleStore for Store {
    async fn upload_package(
        &self,
        distribution: package::Distribution,
    ) -> Result<(), PackageError> {
        let core_metadata = distribution.core_metadata;
        let hashes = distribution.hashes;

        let filename = distribution.file.filename;

        let project_exists = self
            .project_exists(&core_metadata.name)
            .await
            .expect("Unable to check wether project exists or not.");

        if !project_exists {
            tracing::info!("Create project {}", &core_metadata.name);
            self.create_project(&core_metadata.name)
                .await
                .expect("Unable to create project");
        }

        let project = sqlx::query!(
            r#"
            SELECT id FROM projects WHERE name = $1
            "#,
            &core_metadata.name
        )
        .fetch_one(&self.db)
        .await
        .expect("Unable to get record.");

        let save_file = self
            .save_file_distribution(&core_metadata.name, &filename, &distribution.file.content)
            .await;

        if save_file.is_err() {
            return Err(PackageError {});
        }

        let size = distribution.file.content.len() as i32;

        let file_path = Path::from_iter(["simple-index", &core_metadata.name, &filename]);

        let mut tx = self.db.begin().await.expect("Unable to start transaction");
        let release = sqlx::query!(
            r#"
            INSERT INTO releases(
            version, canonical_version, is_prerelease, author, author_email, maintainer, maintainer_email, home_page, license, summary, keywords, platform, download_url, requires_python, project_id)
            VALUES
                ($1, $2, pep440_is_prerelease($1), $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING id
            "#,
            &core_metadata.version,
            canonicalize_version(&core_metadata.version, false),
            &core_metadata.author.as_deref().unwrap_or(""),
            &core_metadata.author_email.as_deref().unwrap_or(""),
            &core_metadata.maintainer.as_deref().unwrap_or(""),
            &core_metadata.maintainer_email.as_deref().unwrap_or(""),
            &core_metadata.home_page.as_deref().unwrap_or(""),
            &core_metadata.license.as_deref().unwrap_or(""),
            &core_metadata.summary.as_deref().unwrap_or(""),
            &core_metadata.keywords.join(","),
            &core_metadata.platforms.join(","),
            &core_metadata.download_url.as_deref().unwrap_or(""),
            &core_metadata.requires_python.as_deref().unwrap_or(""),
            &project.id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();

        let release_id = release.id;

        let _ = sqlx::query!(r#"
            INSERT INTO release_files(
                python_version, requires_python, packagetype, filename, path, size, md5_digest, sha256_digest, blake2_256_digest, release_id
            )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, lower($8), lower($9), $10)
            "#,
            &distribution.python_version.as_deref().unwrap_or(""),
            &core_metadata.requires_python.as_deref().unwrap_or(""),
            PackageType::BdistWheel as _,
            &filename,
            &file_path.to_string(),
            &size,
            &hashes.md5_digest,
            &hashes.sha256_digest,
            &hashes.blake2_256_digest,
            &release_id,
            )
            .execute(&mut *tx)
            .await
            .expect("Unable to add release file");

        if let Some(desc) = &core_metadata.description {
            tracing::info!("Receive description");
            let description_type = &core_metadata.description_content_type;
            tracing::info!("Description type: {:?}", description_type);

            let _ = sqlx::query!(
                r#"
                INSERT INTO release_descriptions (
                    content_type, raw, html, release_id
                )
                VALUES
                    ($1, $2, $3, $4)
                "#,
                description_type.as_deref().unwrap_or("text/rst"),
                desc,
                "",
                &release_id
            )
            .execute(&mut *tx)
            .await;
        }

        let deps = core_metadata.get_dependencies();
        let deps_number = deps.len() as i32;

        if deps_number > 0 {
            let (deps_kind, deps_specifier): (Vec<_>, Vec<_>) = deps
                .into_iter()
                .map(|Dependency { kind, specifier }| (kind, specifier.to_owned()))
                .unzip();

            let deps_release = vec![release_id, deps_number];

            let _ = sqlx::query!(
                r#"
            INSERT INTO release_dependencies (
                kind, specifier, release_id)
            SELECT * FROM UNNEST($1::"dependency_kind"[], $2::text[], $3::int[])
            "#,
                &deps_kind as _,
                &deps_specifier,
                &deps_release
            )
            .execute(&mut *tx)
            .await;
        }

        let tx = tx.commit().await;

        if tx.is_err() {
            tracing::info!("Transaction failed, about to delete the file.");

            // TODO: Should we handle this error better ???
            self.store
                .delete(&file_path)
                .await
                .expect("Unable to delete file on aborted transactions.");
            return Err(PackageError {});
        }

        Ok(())
    }

    async fn get_projects(&self) -> Result<Vec<ProjectName>, PackageError> {
        let projects = sqlx::query_as!(
            ProjectName,
            r#"
            SELECT name FROM projects
            ORDER BY name ASC
            "#
        )
        .fetch_all(&self.db)
        .await;

        match projects {
            Ok(p) => Ok(p),
            Err(_e) => Err(PackageError),
        }
    }

    async fn get_dists(&self, project: &str) -> Result<Vec<PkgDist>, PackageError> {
        let pkg_dists = sqlx::query_as!(
            PkgDist,
            r#"
            WITH SelectedProject AS (
                SELECT id
                FROM projects
                WHERE normalized_name = normalize_pep426_name($1)
            )
            SELECT rf.filename as filename, rf.path as path
            FROM SelectedProject sr
            JOIN releases r ON sr.id = r.project_id
            JOIN release_files rf ON r.id = rf.release_id;
            "#,
            project
        )
        .fetch_all(&self.db)
        .await;

        if let Ok(pkg_dists) = pkg_dists {
            Ok(pkg_dists)
        } else {
            Err(PackageError {})
        }
    }

    async fn get_dist_file(
        &self,
        project: &str,
        dist: &str,
    ) -> Result<package::File, PackageError> {
        let file_path = Path::from_iter(["simple-index", project, dist]);
        let file = self.store.get(&file_path).await;

        match file {
            Ok(file) => {
                let content = file.bytes().await.expect("Unable to decode wheel content");
                let filename = dist.to_owned();

                Ok(package::File { filename, content })
            }
            _ => {
                todo!()
            }
        }
    }

    async fn get_dist_metadata(
        &self,
        _project: &str,
        _dist: &str,
    ) -> Result<package::CoreMetadata, PackageError> {
        todo!()
    }
}
