use crate::package;
use crate::simple_api::{PackageError, PkgDist, ProjectName, SimpleStore};

use anyhow::Result;
use sqlx::PgPool;
use std::sync::Arc;

use async_trait::async_trait;
use object_store::{path::Path, ObjectStore};

#[derive(Clone)]
pub struct Store {
    db: PgPool,
    store: Arc<dyn ObjectStore>,
}

impl Store {
    pub fn new(db: PgPool, store: Arc<dyn ObjectStore>) -> Store {
        Store { db, store }
    }

    async fn create_project(&self, project_name: &String) -> Result<(), PackageError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO projects (name, normalized_name) 
            VALUES ($1, normalize_pep426_name($2))
            "#,
            project_name,
            project_name,
        )
        .execute(&self.db)
        .await;

        if query.is_err() {
            return Err(PackageError);
        }

        Ok(())
    }

    async fn project_exists(&self, project_name: &String) -> Result<bool, PackageError> {
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
}

#[async_trait]
impl SimpleStore for Store {
    async fn upload_package(
        &self,
        distribution: package::Distribution,
    ) -> Result<(), PackageError> {
        let core_metadata = distribution.core_metadata;
        let filename = distribution.file.filename;

        let project_exists = self
            .project_exists(&core_metadata.name)
            .await
            .expect("Unable to check wether project exists or not.");

        if !project_exists {
            log::info!("Create project {}", &core_metadata.name);
            self.create_project(&core_metadata.name)
                .await
                .expect("Unable to create project");
        }

        let file_path = Path::from_iter([
            "simple-index",
            (core_metadata.name.as_str()),
            filename.as_str(),
        ]);
        let r = self.store.put(&file_path, distribution.file.content).await;

        match r {
            Ok(_) => Ok(()),
            Err(_) => return Err(PackageError),
        }

        // let upload_package_query = include_str!("./query/upload_package.srql");


        // match transaction {
        //     Ok(_) => Ok(()),
        //     Err(_) => {
        //         self.store
        //             .delete(&file_path)
        //             .await
        //             .expect("Unable to delete a file on aborted transaction.");
        //         Err(PackageError)
        //     }
        // }
    }

    async fn get_projects(&self) -> Result<Vec<ProjectName>, PackageError> {
        // match self.db.select("projects").await {
        //     Ok(p) => Ok(p),
        //     Err(_e) => Err(PackageError),
        // }

        todo!();
    }

    async fn get_dists(&self, project: &String) -> Result<Vec<PkgDist>, PackageError> {
        // let get_dists_query = include_str!("./query/get_dists.srql");

        let result = todo!();

        // match result {
        //     Ok(mut r) => {
        //         let dists: Option<Dists> = r.take(1).unwrap();
        //         let dists = dists.unwrap();
        //         Ok(dists.dists)
        //     }
        //     Err(_) => Err(PackageError),
        // }
    }

    async fn get_dist_file(
        &self,
        project: &String,
        dist: &String,
    ) -> Result<package::File, PackageError> {
        let file_path = Path::from_iter(["simple-index", project.as_str(), dist.as_str()]);
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
        _project: &String,
        _dist: &String,
    ) -> Result<package::CoreMetadata, PackageError> {
        todo!()
    }
}
