use crate::package::{CoreMetadata, Distribution, File};

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

        Distribution {
            core_metadata,
            file,
        }
    }
}
