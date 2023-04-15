use crate::pypa::{self, Package, CoreMetadata};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use async_trait::async_trait;
use object_store::ObjectStore;
use surrealdb::{engine::remote::ws::Client, Error, Surreal};

pub struct PackageError;

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Classifier {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct PkgMetadata {
    metadata_version: String,
    name: String,
    version: String,
    platforms: Vec<String>,
    supported_platforms: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    description_content_type: Option<String>,
    keywords: Vec<String>,
    home_page: Option<String>,
    download_url: Option<String>,
    author: Option<String>,
    author_email: Option<String>,
    maintainer: Option<String>,
    maintainer_email: Option<String>,
    license: Option<String>,
    // classifiers: Vec<Classifier>, Node to Classifiers
    requires_dists: Vec<String>,
    requires_python: Option<String>,
    requires_externals: Vec<String>,
    project_urls: Vec<String>,
    provides_extras: Vec<String>,
    provides_dists: Vec<String>,
    obsoletes_dists: Vec<String>,
}

#[async_trait]
pub trait SimpleStore: Send + Sync + 'static {
    async fn upload_package(&self, package: Package) -> Result<(), PackageError>;
    async fn add_classifiers(&self, classifiers: Vec<pypa::Classifier>)
        -> Result<(), PackageError>;
    async fn create_project(&self, project: String) -> Result<(), PackageError>;
    async fn add_pkg_metadata(&self, metadata: CoreMetadata) -> Result<(), PackageError>;
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
    async fn upload_package(&self, package: Package) -> Result<(), PackageError> {
        if let Err(e) = self.create_project(package.metadata.name).await {
            return Err(e);
        };

        if let Err(e) = self.add_classifiers(package.metadata.classifiers).await {
            return Err(e);
        }

        Ok(())
    }

    async fn add_classifiers(
        &self,
        classifiers: Vec<pypa::Classifier>,
    ) -> Result<(), PackageError> {
        for classifier in classifiers.into_iter() {
            let new_classifier: Result<Classifier, Error> = self
                .db
                .create("classifiers")
                .content(Classifier { name: classifier.0 })
                .await;

            if let Err(e) = new_classifier {
                return Err(PackageError);
            };
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
            Err(e) => Err(PackageError),
        }
    }

    async fn add_pkg_metadata(&self, metadata: CoreMetadata) -> Result<(), PackageError> {
        todo!();
    }
}
