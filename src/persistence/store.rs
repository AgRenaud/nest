use crate::package;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
pub struct Project {
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
    async fn get_projects(&self) -> Result<Vec<Project>, PackageError>;
    async fn get_dists(&self, project: &String) -> Result<Vec<PkgDist>, PackageError>;
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
        log::debug!(
            "Uploading package {} - {}",
            &distribution.core_metadata.name,
            &distribution.core_metadata.version
        );

        let core_metadata = distribution.core_metadata;
        let filename = distribution.file.filename;

        let file_path = Path::from_iter(["simple-index", filename.as_str()]);
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
                self.store.delete(&file_path).await;
                Err(PackageError)
            }
        }
    }

    async fn get_projects(&self) -> Result<Vec<Project>, PackageError> {
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
}
