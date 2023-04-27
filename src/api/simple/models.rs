use crate::package::{
    Classifier, CoreMetadata, ObsoletesDist, Package, PkgFile, ProjectURL, ProvidesDist,
    ProvidesExtra, RequiresDist, RequiresExternal,
};

use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleIndex {
    pub packages: Vec<String>,
}

#[derive(Serialize)]
pub struct ProjectDists {
    pub dists: Vec<String>,
}

#[derive(TryFromMultipart)]
pub struct RequestData {
    #[form_data(field_name = ":action")]
    action: String,
    protocol_version: String,

    // identify release
    name: String,
    version: String,

    // file content
    filetype: Option<String>,
    pyversion: Option<String>,

    // additional meta-data
    metadata_version: String,
    summary: Option<String>,
    home_page: Option<String>,
    author: Option<String>,
    author_email: Option<String>,
    maintainer: Option<String>,
    maintainer_email: Option<String>,
    license: Option<String>,
    description: Option<String>,
    keywords: Option<String>,
    platform: Option<String>,
    classifiers: Vec<String>,
    download_url: Option<String>,
    platforms: Vec<String>,
    supported_platform: Option<String>,
    comment: Option<String>,
    md5_digest: Option<String>,
    sha256_digest: Option<String>,
    blake2_256_digest: Option<String>,
    description_content_type: Option<String>,

    // PEP 314
    provides: Vec<String>,
    requires: Option<String>,
    obsoletes: Option<String>,
    provides_extra: Vec<String>,

    // Metadata 1.2
    project_urls: Option<String>,
    provides_dist: Option<String>,
    obsoletes_dist: Option<String>,
    requires_dist: Option<String>,
    requires_external: Option<String>,
    requires_python: Option<String>,
    content: FieldData<Bytes>,
}

// Traits impl
impl Into<Package> for RequestData {
    fn into(self) -> Package {
        fn parse_string(s: Option<String>) -> Vec<String> {
            match s {
                Some(elt) => elt
                    .split("\r\n")
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect(),
                _ => Vec::new(),
            }
        }

        let filename = self.content.metadata.file_name.expect("No filename");
        let content = self.content.contents;
        let pkg_file = PkgFile { filename, content };

        let metadata = CoreMetadata {
            metadata_version: self.metadata_version,
            name: self.name,
            version: self.version,
            platforms: self.platforms,
            supported_platforms: parse_string(self.supported_platform),
            summary: self.summary,
            description: self.description,
            description_content_type: self.description_content_type,
            keywords: parse_string(self.keywords),
            home_page: self.home_page,
            download_url: self.download_url,
            author: self.author,
            author_email: self.author_email,
            maintainer: self.maintainer,
            maintainer_email: self.maintainer_email,
            license: self.license,
            classifiers: self
                .classifiers
                .iter()
                .map(|c| Classifier(c.to_string()))
                .collect(),
            requires_dists: parse_string(self.requires_dist)
                .iter()
                .map(|d| RequiresDist(d.to_string()))
                .collect(),
            requires_python: self.requires_python,
            requires_externals: parse_string(self.requires_external)
                .iter()
                .map(|e| RequiresExternal(e.to_string()))
                .collect(),
            project_urls: parse_string(self.project_urls)
                .iter()
                .map(|s| ProjectURL(s.to_string()))
                .collect(),
            provides_extras: self
                .provides_extra
                .iter()
                .map(|s| ProvidesExtra(s.to_string()))
                .collect(),
            provides_dists: parse_string(self.provides_dist)
                .iter()
                .map(|s| ProvidesDist(s.to_string()))
                .collect(),
            obsoletes_dists: parse_string(self.obsoletes_dist)
                .iter()
                .map(|s| ObsoletesDist(s.to_string()))
                .collect(),
        };

        Package { metadata, pkg_file }
    }
}

impl std::fmt::Debug for RequestData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RequestData")
            .field("action", &self.action)
            .field("protocol_version", &self.protocol_version)
            .field("name", &self.name)
            .field("version", &self.version)
            .field("filetype", &self.filetype)
            .field("pyversion", &self.pyversion)
            .field("metadata_version", &self.metadata_version)
            .field("summary", &self.summary)
            .field("home_page", &self.home_page)
            .field("author", &self.author)
            .field("author_email", &self.author_email)
            .field("maintainer", &self.maintainer)
            .field("maintainer_email", &self.maintainer_email)
            .field("license", &self.license)
            .field("description", &self.description)
            .field("keywords", &self.keywords)
            .field("platform", &self.platform)
            .field("classifiers", &self.classifiers)
            .field("download_url", &self.download_url)
            .field("supported_platform", &self.supported_platform)
            .field("comment", &self.comment)
            .field("md5_digest", &self.md5_digest)
            .field("sha256_digest", &self.sha256_digest)
            .field("blake2_256_digest", &self.blake2_256_digest)
            .field("provides", &self.provides)
            .field("requires", &self.requires)
            .field("obsoletes", &self.obsoletes)
            .field("project_urls", &self.project_urls)
            .field("provides_dist", &self.provides_dist)
            .field("obsoletes_dist", &self.obsoletes_dist)
            .field("requires_dist", &self.requires_dist)
            .field("requires_external", &self.requires_external)
            .field("requires_python", &self.requires_python)
            .finish()
    }
}
