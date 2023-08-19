use crate::package::Distribution;

use maud::{html, Markup, DOCTYPE};

use axum::{
    extract::{Path, State},
    headers::{authorization::Basic, Authorization},
    response::IntoResponse,
    routing::{get, post},
    Router, TypedHeader,
};
use axum_typed_multipart::TypedMultipart;
use hyper::{header, StatusCode};

use super::models::RequestData;
use super::SimpleController;

pub fn router(state: SimpleController) -> Router {
    Router::new()
        .route("/simple/", post(upload).get(list_packages))
        .route("/simple/:project/", get(list_dists))
        .route("/simple/:project/:distribution", get(download_package))
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

    if let Err(_) = state.store.upload_package(distribution).await {
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
            let content_disposition = format!("attachment; filename=\"{}\"", &(*filename));

            let headers = axum::response::AppendHeaders([
                (header::CONTENT_TYPE, content_type),
                (header::CONTENT_DISPOSITION, content_disposition),
            ]);

            Ok((headers, body))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "File not found !")),
    }
}
