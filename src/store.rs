use crate::pypa::{self, CoreMetadata, Package};

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
        if let Err(e) = self.create_project(package.metadata.name.clone()).await {
            return Err(e);
        };

        if let Err(e) = self.add_classifiers(package.metadata.classifiers.clone()).await {
            return Err(e);
        }

        if let Err(e) = self.add_pkg_metadata(package.metadata).await {
            return Err(e)
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
        let pkg_metadata = PkgMetadata {
            metadata_version: metadata.metadata_version,
            name: metadata.name,
            version: metadata.version,
            platforms: metadata.platforms,
            supported_platforms: metadata.supported_platforms,
            summary: metadata.summary.unwrap_or_default(),
            description: metadata.description.unwrap_or_default(),
            description_content_type: metadata.description_content_type.unwrap_or_default(),
            keywords: metadata.keywords,
            home_page: metadata.home_page.unwrap_or_default(),
            download_url: metadata.download_url.unwrap_or_default(),
            author: metadata.author.unwrap_or_default(),
            author_email: metadata.author_email.unwrap_or_default(),
            maintainer: metadata.maintainer.unwrap_or_default(),
            maintainer_email: metadata.maintainer_email.unwrap_or_default(),
            license: metadata.license.unwrap_or_default(),
            requires_dists: metadata.requires_dists.into_iter().map(|req| req.0).collect(),
            requires_python: metadata.requires_python.unwrap_or_default(),
            requires_externals: metadata.requires_externals.into_iter().map(|req| req.0).collect(),
            project_urls: metadata.project_urls.into_iter().map(|url| url.0).collect(),
            provides_dists: metadata.provides_dists.into_iter().map(|dist| dist.0).collect(),
            provides_extras: metadata.provides_extras.into_iter().map(|ext| ext.0).collect(),
            obsoletes_dists: metadata.obsoletes_dists.into_iter().map(|obs| obs.0).collect(),
        };

        let pkg_metadata: Result<PkgMetadata, Error> = self
            .db
            .create("pkg_metadata")
            .content(pkg_metadata)
            .await;

        Ok(())
    }
}
