use axum_template::RenderHtml;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_typed_multipart::TypedMultipart;
use hyper::{header, StatusCode};
use serde::Serialize;

pub mod models;
pub mod package;
pub mod simple_api;
pub mod store;

use crate::{authentication::auth, engine::AppEngine, state::AppState};
use models::RequestData;
use package::Distribution;

use self::simple_api::SimpleState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(upload))
        .route_layer(axum::middleware::from_fn(auth))
        .route("/", get(list_packages))
        .route("/:project/", get(list_dists))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .route("/:project/:distribution", get(download_package))
}

#[tracing::instrument(
        name = "Simple::Upload a package",
        skip(store, data),
        fields(
            project = %data.name,
            project_version = %data.version
        )
    )]
async fn upload(
    State(store): State<SimpleState>,
    TypedMultipart(data): TypedMultipart<RequestData>,
) {
    let distribution: Distribution = data.into();

    if (store.upload_package(distribution).await).is_err() {
        tracing::error!("Failed to upload package");
    } else {
        tracing::info!("Package has been added to index");
    }
}

#[derive(Serialize)]
struct Dists {
    dists: Vec<String>,
}

#[tracing::instrument(
        name = "Simple::Get distributions list",
        skip(engine, store, project),
        fields(
            project = %project
        )
    )]
async fn list_dists(
    engine: AppEngine,
    Path(project): Path<String>,
    State(store): State<SimpleState>,
) -> impl IntoResponse {
    let dists = store.get_dists(&project).await.unwrap();
    let dists: Vec<String> = dists.iter().map(|d| d.filename.to_owned()).collect();

    RenderHtml("simple/dists.jinja", engine, Dists { dists })
}

#[derive(Serialize)]
struct Projects {
    projects: Vec<String>,
}

#[tracing::instrument(name = "Simple::List package", skip(engine, store))]
async fn list_packages(engine: AppEngine, State(store): State<SimpleState>) -> impl IntoResponse {
    let projects = store.get_projects().await.unwrap();
    let projects: Vec<String> = projects.iter().map(|p| p.name.to_owned()).collect();

    RenderHtml("simple/packages.jinja", engine, Projects { projects })
}

#[tracing::instrument(
        name = "Simple::Get distributions list",
        skip(store, project, distribution),
        fields(
            project = %project,
            distribution = %distribution
        )
    )]
async fn download_package(
    State(store): State<SimpleState>,
    Path((project, distribution)): Path<(String, String)>,
) -> impl IntoResponse {
    let file = store.get_dist_file(&project, &distribution).await;

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
