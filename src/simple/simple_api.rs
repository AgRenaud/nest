use std::sync::Arc;

use super::package;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use async_trait::async_trait;

#[derive(Debug)]
pub struct PackageError;

#[derive(Serialize, Deserialize)]
pub struct ProjectName {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct PkgDist {
    pub filename: String,
    pub path: String,
}

#[async_trait]
pub trait SimpleStore: Send + Sync + 'static {
    async fn upload_package(&self, distribution: package::Distribution)
        -> Result<(), PackageError>;
    async fn get_projects(&self) -> Result<Vec<ProjectName>, PackageError>;
    async fn get_dists(&self, project: &str) -> Result<Vec<PkgDist>, PackageError>;
    async fn get_dist_file(&self, project: &str, dist: &str)
        -> Result<package::File, PackageError>;
    async fn get_dist_metadata(
        &self,
        project: &str,
        dist: &str,
    ) -> Result<package::CoreMetadata, PackageError>;
}

pub type SimpleState = Arc<dyn SimpleStore>;
