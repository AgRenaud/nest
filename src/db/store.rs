use crate::package;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use object_store::{path::Path, ObjectStore};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Error, Surreal};

#[derive(Debug)]
pub struct PackageError;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Classifier {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PkgDist {
    pub filename: String,
}

#[derive(Deserialize)]
pub struct Dists {
    dists: Vec<PkgDist>,
}

#[derive(Serialize, Deserialize)]
struct PkgVersion {
    metadata_version: String,
    name: String,
    version: String,
    platforms: Vec<String>,
    supported_platforms: Vec<String>,
    summary: String,
    description: String,
    description_content_type: String,
    keywords: Vec<String>,
    home_page: String,
    download_url: String,
    author: String,
    author_email: String,
    maintainer: String,
    maintainer_email: String,
    license: String,
    // classifiers: Vec<Classifier>, Node to Classifiers
    requires_dists: Vec<String>,
    requires_python: String,
    requires_externals: Vec<String>,
    project_urls: Vec<String>,
    provides_extras: Vec<String>,
    provides_dists: Vec<String>,
    obsoletes_dists: Vec<String>,
}

impl From<package::CoreMetadata> for PkgVersion {
    fn from(value: package::CoreMetadata) -> Self {
        PkgVersion {
            metadata_version: value.metadata_version,
            name: value.name, // Thing { tb: String::from("projects"),  id: Id::String(value.name) },
            version: value.version,
            platforms: value.platforms,
            supported_platforms: value.supported_platforms,
            summary: value.summary.unwrap_or_default(),
            description: value.description.unwrap_or_default(),
            description_content_type: value.description_content_type.unwrap_or_default(),
            keywords: value.keywords,
            home_page: value.home_page.unwrap_or_default(),
            download_url: value.download_url.unwrap_or_default(),
            author: value.author.unwrap_or_default(),
            author_email: value.author_email.unwrap_or_default(),
            maintainer: value.maintainer.unwrap_or_default(),
            maintainer_email: value.maintainer_email.unwrap_or_default(),
            license: value.license.unwrap_or_default(),
            requires_dists: value.requires_dists.into_iter().map(|req| req.0).collect(),
            requires_python: value.requires_python.unwrap_or_default(),
            requires_externals: value
                .requires_externals
                .into_iter()
                .map(|req| req.0)
                .collect(),
            project_urls: value.project_urls.into_iter().map(|url| url.0).collect(),
            provides_dists: value
                .provides_dists
                .into_iter()
                .map(|dist| dist.0)
                .collect(),
            provides_extras: value.provides_extras.into_iter().map(|ext| ext.0).collect(),
            obsoletes_dists: value.obsoletes_dists.into_iter().map(|obs| obs.0).collect(),
        }
    }
}

#[async_trait]
pub trait SimpleStore: Send + Sync + 'static {
    async fn upload_package(&self, package: package::Package) -> Result<(), PackageError>;
    async fn get_projects(&self) -> Result<Vec<Project>, PackageError>;
    async fn get_dists(&self, project: String) -> Result<Vec<PkgDist>, PackageError>;
}

#[derive(Clone)]
pub struct Store {
    store: Arc<dyn ObjectStore>,
    db: Arc<Surreal<Client>>,
}

impl Store {
    pub fn new(db: Arc<Surreal<Client>>, store: Arc<dyn ObjectStore>) -> Store {
        Store { db, store }
    }
}

#[async_trait]
#[warn(unused_must_use)]
impl SimpleStore for Store {
    async fn upload_package(&self, package: package::Package) -> Result<(), PackageError> {
        log::debug!(
            "Uploading package {} - {}",
            &package.metadata.name,
            &package.metadata.version
        );

        let project_name = package.metadata.name.clone();
        let classifiers = package.metadata.classifiers.clone();
        let pkg_metadata = package.metadata;
        let pkg_file = package.pkg_file;

        let project = self.add_or_select_project(project_name).await;
        let pkg_version = self
            .add_or_update_pkg_version(&project.id, pkg_metadata)
            .await;
        let _pkg_dist = self
            .add_or_update_pkg_dist(&pkg_version.id, &pkg_file)
            .await;
        let _classifiers = self
            .add_or_select_classifiers(&pkg_version.id, classifiers)
            .await;

        Ok(())
    }

    async fn get_projects(&self) -> Result<Vec<Project>, PackageError> {
        let projects: Vec<Project> = self.db.select("projects").await.unwrap();

        Ok(projects)
    }

    async fn get_dists(&self, project: String) -> Result<Vec<PkgDist>, PackageError> {
        let project = Thing {
            tb: String::from("projects"),
            id: surrealdb::sql::Id::String(project),
        };
        let mut result = self
            .db
            .query(r#"SELECT ->has_versions->pkg_versions->has_dists->pkg_dists.* AS dists FROM $project_name;"#)
            .bind(("project_name", project))
            .await
            .unwrap();

        let dists: Option<Dists> = result.take(0).unwrap();
        let dists = dists.unwrap();

        Ok(dists.dists)
    }
}

impl Store {
    async fn add_or_select_project(&self, name: String) -> Record {
        let project: Option<Record> = self.db.select(("projects", &name)).await.unwrap();

        match project {
            Some(p) => p,
            None => {
                let project: Result<Record, Error> = self
                    .db
                    .create(("projects", &name))
                    .content(Project { name: name.clone() })
                    .await;
                project.unwrap()
            }
        }
    }

    async fn add_or_update_pkg_version(
        &self,
        project_id: &Thing,
        metadata: package::CoreMetadata,
    ) -> Record {
        let record: Option<Record> = self
            .db
            .query("SELECT id FROM pkg_versions WHERE name=$name AND version=$version")
            .bind(("name", metadata.name.clone()))
            .bind(("version", metadata.version.clone()))
            .await
            .unwrap()
            .take(0)
            .unwrap();

        match record {
            Some(r) => {
                let pkg_metadata: PkgVersion = metadata.into();

                let record = self
                    .db
                    .update(("pkg_versions", r.id))
                    .content(pkg_metadata)
                    .await;
                record.unwrap()
            }
            None => {
                let pkg_metadata: PkgVersion = metadata.into();

                let record: Option<Record> = self
                    .db
                    .query(
                        "
                    BEGIN TRANSACTION;

                    LET $new_version = (CREATE pkg_versions CONTENT $pkg_metadata);
                    LET $new_version_id = $new_version.id;
                    LET $new_version_id=type::thing('pkg_versions', $new_version_id);

                    RELATE $project_id->has_versions->$new_version_id;

                    RETURN $new_version;

                    COMMIT TRANSACTION;
                ",
                    )
                    .bind(("pkg_metadata", pkg_metadata))
                    .bind(("project_id", project_id))
                    .await
                    .unwrap()
                    .take(0)
                    .unwrap();

                record.unwrap()
            }
        }
    }

    async fn add_or_select_classifiers(
        &self,
        pkg_version: &Thing,
        classifiers: Vec<package::Classifier>,
    ) -> Vec<Record> {
        let records = classifiers
            .iter()
            .map(|c| self.add_or_select_classifier(pkg_version, c));

        future::join_all(records).await
    }

    async fn add_or_select_classifier(
        &self,
        _pkg_version: &Thing,
        classifier: &package::Classifier,
    ) -> Record {
        let mut req = self
            .db
            .query("SELECT id FROM classifiers WHERE name=$value")
            .bind(("value", &classifier.0))
            .await
            .unwrap();

        let record: Option<Record> = req.take(0).unwrap();

        match record {
            Some(r) => r,
            None => {
                let classifier = self
                    .db
                    .create("classifiers")
                    .content(Classifier {
                        name: classifier.0.clone(),
                    })
                    .await;
                classifier.unwrap()
            }
        }
    }

    async fn add_or_update_pkg_dist(&self, pkg_version: &Thing, file: &package::PkgFile) -> Record {
        let filename = &file.filename;
        let content = &file.content;

        let mut path = String::from("global/repository/");
        path.push_str(filename.as_str());
        let path = Path::from(path);

        self.store.put(&path, content.to_owned()).await.unwrap();

        let pkg_dist = PkgDist {
            filename: filename.to_owned(),
        };

        let record: Option<Record> = self
            .db
            .query(
                "
            BEGIN TRANSACTION;
            LET $new_dist = (CREATE pkg_dists CONTENT $pkg_dist);
            LET $new_dist_id = $new_dist.id;
            LET $new_dist_id=type::thing('pkg_dists', $new_dist_id);
            RELATE $pkg_version->has_dists->$new_dist;

            RETURN $new_dist;

            COMMIT TRANSACTION;
        ",
            )
            .bind(("pkg_dist", pkg_dist))
            .bind(("pkg_version", pkg_version))
            .await
            .unwrap()
            .take(0)
            .unwrap();

        log::debug!("{:?}", &record);

        record.unwrap()
    }
}
