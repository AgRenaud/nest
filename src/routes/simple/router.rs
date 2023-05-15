use crate::package::Distribution;

use liquid;

use axum::{
    extract::{Path, State},
    headers::{authorization::Basic, Authorization},
    response::{Html, IntoResponse},
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

async fn list_dists(
    Path(project): Path<String>,
    State(state): State<SimpleController>,
) -> Html<String> {
    let html_template = include_str!("./templates/simple-package.html");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(html_template)
        .unwrap();

    let dists = state.store.get_dists(&project).await.unwrap();
    let dists: Vec<String> = dists.iter().map(|d| d.filename.to_owned()).collect();

    let html = template
        .render(&liquid::object!({"project": project, "dists": dists}))
        .unwrap();

    Html(html)
}

async fn list_packages(State(state): State<SimpleController>) -> Html<String> {
    let html_template = include_str!("./templates/simple.html");

    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(html_template)
        .unwrap();

    let projects = state.store.get_projects().await.unwrap();
    let projects: Vec<String> = projects.iter().map(|p| p.name.to_owned()).collect();

    let html = template
        .render(&liquid::object!({ "projects": projects }))
        .unwrap();

    Html(html)
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
