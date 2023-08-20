use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use serde::Serialize;

use super::package::{CoreMetadata, DistHashes, Distribution, File};

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
    pub action: String,
    pub protocol_version: String,

    // identify release
    pub name: String,
    pub version: String,

    // file content
    pub filetype: Option<String>,
    pub pyversion: Option<String>,

    // additional meta-data
    pub metadata_version: String,
    pub summary: Option<String>,
    pub home_page: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_email: Option<String>,
    pub license: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub platform: Option<String>,
    pub classifiers: Vec<String>,
    pub download_url: Option<String>,
    pub platforms: Vec<String>,
    pub supported_platform: Option<String>,
    // pub comment: Option<String>,
    pub md5_digest: String,
    pub sha256_digest: String,
    pub blake2_256_digest: String,
    pub description_content_type: Option<String>,

    // PEP 314
    pub provides: Vec<String>,
    pub requires: Option<String>,
    pub obsoletes: Option<String>,
    pub provides_extra: Vec<String>,

    // Metadata 1.2
    pub project_urls: Option<String>,
    pub provides_dist: Option<String>,
    pub obsoletes_dist: Option<String>,
    pub requires_dist: Option<String>,
    pub requires_external: Option<String>,
    pub requires_python: Option<String>,
    pub content: FieldData<Bytes>,
}

// Traits impl
impl From<RequestData> for Distribution {
    fn from(val: RequestData) -> Self {
        fn parse_string(s: Option<String>) -> Vec<String> {
            match s {
                Some(elt) => elt.split("\r\n").map(|e| e.to_string()).collect(),
                _ => Vec::new(),
            }
        }

        let filename = val.content.metadata.file_name.expect("No filename");
        let content = val.content.contents;

        let core_metadata = CoreMetadata {
            metadata_version: val.metadata_version,
            name: val.name,
            version: val.version,
            platforms: val.platforms,
            supported_platforms: parse_string(val.supported_platform),
            summary: val.summary,
            description: val.description,
            description_content_type: val.description_content_type,
            keywords: parse_string(val.keywords),
            home_page: val.home_page,
            download_url: val.download_url,
            author: val.author,
            author_email: val.author_email,
            maintainer: val.maintainer,
            maintainer_email: val.maintainer_email,
            license: val.license,
            classifiers: val.classifiers,
            requires_dists: parse_string(val.requires_dist),
            requires_python: val.requires_python,
            requires_externals: parse_string(val.requires_external),
            project_urls: parse_string(val.project_urls),
            provides_extras: val.provides_extra,
            provides_dists: parse_string(val.provides_dist),
            obsoletes_dists: parse_string(val.obsoletes_dist),
        };

        let file = File { filename, content };

        let hashes = DistHashes {
            md5_digest: val.md5_digest,
            sha256_digest: val.sha256_digest,
            blake2_256_digest: val.blake2_256_digest,
        };

        let python_version = val.pyversion;

        Distribution {
            core_metadata,
            file,
            hashes,
            python_version,
        }
    }
}
