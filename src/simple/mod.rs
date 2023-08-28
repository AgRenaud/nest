use std::sync::Arc;

use maud::{html, Markup, DOCTYPE};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use axum_typed_multipart::TypedMultipart;
use hyper::{header, StatusCode};
use sqlx::PgPool;

pub mod models;
pub mod package;
pub mod simple_api;
pub mod store;

use crate::authentication::auth;
use models::RequestData;
use package::Distribution;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[derive(Clone)]
pub struct SimpleState {
    pub store: Arc<dyn simple_api::SimpleStore>,
}

pub fn router(state: SimpleState, pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(pool))
        .into_inner();

    Router::new()
        .route("/", post(upload))
        .route_layer(axum::middleware::from_fn(auth))
        .route("/", get(list_packages))
        .route("/:project/", get(list_dists))
        .layer(axum::extract::DefaultBodyLimit::disable())
        .route("/:project/:distribution", get(download_package))
        .with_state(state)
        .layer(middleware)
}

#[tracing::instrument(
        name = "Simple::Upload a package",
        skip(state, data),
        fields(
            project = %data.name,
            project_version = %data.version
        )
    )]
async fn upload(
    State(state): State<SimpleState>,
    TypedMultipart(data): TypedMultipart<RequestData>,
) {
    let distribution: Distribution = data.into();

    if (state.store.upload_package(distribution).await).is_err() {
        tracing::error!("Failed to upload package");
    } else {
        tracing::info!("Package has been added to index");
    }
}

#[tracing::instrument(
        name = "Simple::Get distributions list",
        skip(state, project),
        fields(
            project = %project
        )
    )]
async fn list_dists(Path(project): Path<String>, State(state): State<SimpleState>) -> Markup {
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

#[tracing::instrument(name = "Simple::List package", skip(state))]
async fn list_packages(State(state): State<SimpleState>) -> Markup {
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

#[tracing::instrument(
        name = "Simple::Get distributions list",
        skip(state, project, distribution),
        fields(
            project = %project,
            distribution = %distribution
        )
    )]
async fn download_package(
    State(state): State<SimpleState>,
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
