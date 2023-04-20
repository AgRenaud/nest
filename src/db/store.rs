use crate::package::{CoreMetadata, Package};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use async_trait::async_trait;
use object_store::ObjectStore;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Error, Surreal};

pub struct PackageError;

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
}

#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

impl From<CoreMetadata> for PkgMetadata {
    fn from(value: CoreMetadata) -> Self {
        PkgMetadata {
            metadata_version: value.metadata_version,
            name: value.name,
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
    async fn upload_package(&self, package: Package) -> Result<(), PackageError>;
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
        log::info!("Uploading package");

        let project_name = package.metadata.name.clone();
        let classifiers = package.metadata.classifiers.clone();
        let pkg_metadata: PkgMetadata = package.metadata.into();

        if !self.project_exists(&project_name).await {
            log::info!("Creating project {}", &project_name);
            let _project: Result<Project, Error> = self
                .db
                .create(("projects", &project_name))
                .content(Project {
                    name: project_name.clone(),
                })
                .await;
        }

        for classifier in classifiers.into_iter() {
            let classifier_value = classifier.0;

            let classifier: Option<Classifier> = self
                .db
                .query("SELECT * FROM classifiers WHERE name=$value")
                .bind(("value", &classifier_value))
                .await
                .unwrap()
                .take(0)
                .unwrap();

            if classifier.is_none() {
                let _classifier: Result<Classifier, Error> = self
                    .db
                    .create("classifiers")
                    .content(Classifier {
                        name: classifier_value,
                    })
                    .await;
            } else {
                log::info!("Classfier '{}' already exists", classifier_value);
            }
        }

        let metadata_record: Record = self
            .db
            .create("pkg_metadata")
            .content(pkg_metadata)
            .await
            .unwrap();

        println!("{:?}", metadata_record);

        let relation_record = self
            .db
            .query("RELATE pkg_metadata:$metadata->projects:$name RETURN NONE")
            .bind(("metadata", metadata_record.id.id))
            .bind(("name", project_name))
            .await
            .unwrap();

        Ok(())
    }
}

impl Store {
    async fn project_exists(&self, project_name: &str) -> bool {
        let project: Option<Project> = self.db.select(("projects", project_name)).await.unwrap();

        project.is_some()
    }
}
