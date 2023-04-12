use crate::pypa::{Classifier, CoreMetadata};

use bytes::Bytes;
use std::sync::Arc;

use async_trait::async_trait;
use object_store::ObjectStore;
use surrealdb::{engine::remote::ws::Client, Surreal};

struct PackageError;

#[async_trait]
pub trait SimpleStore {
    async fn upload_package(
        &self,
        metadata: CoreMetadata,
        archives: Bytes,
    ) -> Result<(), PackageError>;
    async fn add_classifier(&self, classifiers: &[Classifier]) -> Result<(), PackageError>;
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
impl SimpleStore for Store {
    async fn upload_package(
        &self,
        metadata: CoreMetadata,
        archives: Bytes,
    ) -> Result<(), PackageError> {
        match self.add_classifier(&metadata.classifiers).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    async fn add_classifier(&self, classifiers: &[Classifier]) -> Result<(), PackageError> {
        for classifier in classifiers.iter() {
            let result = &self
                .db
                .query("CREATE classifiers SET name=$classifier")
                .bind(("classifier", classifier.0.as_str()))
                .await;

            match result {
                Ok(response) => {
                    todo!()
                }
                Err(e) => {
                    todo!()
                }
            }
        }
        Ok(())
    }

    async fn create_project(&self, project: String) -> Result<(), PackageError> {
        todo!()
    }
}
