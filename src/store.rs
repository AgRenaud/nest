use crate::pypa::{self, Package};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use async_trait::async_trait;
use object_store::ObjectStore;
use surrealdb::{engine::remote::ws::Client, Surreal, Error};

struct PackageError;

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Classifier {
    name: String,
}

#[async_trait]
pub trait SimpleStore {
    async fn upload_package(&self, package: Package) -> Result<(), PackageError>;
    async fn add_classifiers(&self, classifiers: Vec<pypa::Classifier>) -> Result<(), PackageError>;
    async fn create_project(&self, project: String) -> Result<(), PackageError>;
}

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
    async fn upload_package(&self, package: Package) -> Result<(), PackageError> {
        if let Err(e) = self.create_project(package.metadata.name).await {
            return Err(e)
        };

        if let Err(e) = self.add_classifiers(package.metadata.classifiers).await {
            return Err(e)
        }

        Ok(())
    }

    async fn add_classifiers(&self, classifiers: Vec<pypa::Classifier>) -> Result<(), PackageError> {
        for classifier in classifiers.into_iter() {
            let new_classifier: Result<Classifier, Error> = self
                .db
                .create("classifiers")
                .content(Classifier {
                    name: classifier.0
                })
                .await;

            if let Err(e) = new_classifier { return Err(PackageError) };
        }
        Ok(())
    }

    async fn create_project(&self, project: String) -> Result<(), PackageError> {
        let project: Result<Project, Error> = self
            .db
            .create("projects")
            .content(Project { name: project })
            .await;
        
        match project {
            Ok(_) => Ok(()),
            Err(e) => Err(PackageError)
        }
    }
}
