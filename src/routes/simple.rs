use std::sync::Arc;

use maud::{html, Markup, DOCTYPE};

use axum::{
    extract::{Path, State},
    headers::{authorization::Basic, Authorization},
    response::IntoResponse,
    routing::{get, post},
    Router, TypedHeader,
};
use hyper::{header, StatusCode};

use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::Serialize;

use crate::package::{CoreMetadata, DistHashes, Distribution, File};
use crate::simple_api;

#[derive(Clone)]
pub struct SimpleController {
    pub store: Arc<dyn simple_api::SimpleStore>,
}

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
            python_version
        }
    }
}

pub fn router(state: SimpleController) -> Router {
    Router::new()
        .route("/", post(upload).get(list_packages))
        .route("/:project/", get(list_dists))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .route("/:project/:distribution", get(download_package))
        .with_state(state)
}

#[tracing::instrument(
        name = "Upload a package",
        skip(state, auth, data),
        fields(
            project = %data.name,
            project_version = %data.version
        )
    )]
async fn upload(
    State(state): State<SimpleController>,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    TypedMultipart(data): TypedMultipart<RequestData>,
) {
    let distribution: Distribution = data.into();

    if (state.store.upload_package(distribution).await).is_err() {
        tracing::error!("Failed to upload package");
    } else {
        tracing::info!("Package has been added to index");
    }
}

async fn list_dists(Path(project): Path<String>, State(state): State<SimpleController>) -> Markup {
    let dists = state.store.get_dists(&project).await.unwrap();
    let dists: Vec<String> = dists.iter().map(|d| d.filename.to_owned()).collect();

    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="pypi:repository-version" content="1.1";
        title { "Links for " (&project) }
        body {
            h1 { "Links for " (&project) }
            @for dist in &dists {
                a href={"/simple/" (&project) "/" (&dist) } { (&dist) } br;
            }
        }
    }
}

async fn list_packages(State(state): State<SimpleController>) -> Markup {
    let projects = state.store.get_projects().await.unwrap();
    let projects: Vec<String> = projects.iter().map(|p| p.name.to_owned()).collect();

    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="pypi:repository-version" content="1.1";
        title { "Simple index" }
        body {
            @for project in &projects {
                a href={"/simple/" (&project) "/"} { (&project) } br;
            }
        }
    }
}

async fn download_package(
    State(state): State<SimpleController>,
    Path((project, distribution)): Path<(String, String)>,
) -> impl IntoResponse {
    let file = state.store.get_dist_file(&project, &distribution).await;

    match file {
        Ok(file) => {
            let content = file.content;
            let filename = file.filename.as_str();

            let body = content;

            let content_type = String::from("octet/stream; charset=utf-8");
            let content_disposition = format!("attachment; filename=\"{}\"", filename);

            let headers = axum::response::AppendHeaders([
                (header::CONTENT_TYPE, content_type),
                (header::CONTENT_DISPOSITION, content_disposition),
            ]);

            Ok((headers, body))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "File not found !")),
    }
}
