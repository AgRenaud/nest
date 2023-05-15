use crate::package;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;
use object_store::{path::Path, ObjectStore};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

#[derive(Debug)]
pub struct PackageError;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectName {
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

#[async_trait]
pub trait SimpleStore: Send + Sync + 'static {
    async fn upload_package(&self, distribution: package::Distribution)
        -> Result<(), PackageError>;
    async fn get_projects(&self) -> Result<Vec<ProjectName>, PackageError>;
    async fn get_dists(&self, project: &String) -> Result<Vec<PkgDist>, PackageError>;
    async fn get_dist_file(
        &self,
        project: &String,
        dist: &String,
    ) -> Result<package::File, PackageError>;
    async fn get_dist_metadata(
        &self,
        project: &String,
        dist: &String,
    ) -> Result<package::CoreMetadata, PackageError>;
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
impl SimpleStore for Store {
    async fn upload_package(
        &self,
        distribution: package::Distribution,
    ) -> Result<(), PackageError> {
        let core_metadata = distribution.core_metadata;
        let filename = distribution.file.filename;

        let file_path = Path::from_iter([
            "simple-index",
            (core_metadata.name.as_str()),
            filename.as_str(),
        ]);
        let r = self.store.put(&file_path, distribution.file.content).await;

        match r {
            Ok(_) => {}
            Err(_) => return Err(PackageError),
        }

        let upload_package_query = include_str!("./query/upload_package.srql");

        let transaction = self
            .db
            .query(upload_package_query)
            .bind(("core_metadata", &core_metadata))
            .bind(("filename", &filename))
            .await;

        match transaction {
            Ok(_) => Ok(()),
            Err(_) => {
                self.store
                    .delete(&file_path)
                    .await
                    .expect("Unable to delete a file on aborted transaction.");
                Err(PackageError)
            }
        }
    }

    async fn get_projects(&self) -> Result<Vec<ProjectName>, PackageError> {
        match self.db.select("projects").await {
            Ok(p) => Ok(p),
            Err(_e) => Err(PackageError),
        }
    }

    async fn get_dists(&self, project: &String) -> Result<Vec<PkgDist>, PackageError> {
        let get_dists_query = include_str!("./query/get_dists.srql");

        let result = self
            .db
            .query(get_dists_query)
            .bind(("project_name", project))
            .await;

        match result {
            Ok(mut r) => {
                let dists: Option<Dists> = r.take(1).unwrap();
                let dists = dists.unwrap();
                Ok(dists.dists)
            }
            Err(_) => Err(PackageError),
        }
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
