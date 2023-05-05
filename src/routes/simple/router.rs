use crate::package::Distribution;

use axum::routing::{get, post};
use axum::Router;
use axum::{
    extract::{Path, State},
    response::Html,
};
use axum_typed_multipart::TypedMultipart;
use liquid;

use super::models::RequestData;
use super::SimpleController;

pub fn router(state: SimpleController) -> Router {
    Router::new()
        .route("/simple/", post(upload).get(list_packages))
        .route("/simple/:project", get(list_dists))
        .with_state(state)
}

async fn upload(State(state): State<SimpleController>, data: TypedMultipart<RequestData>) {
    let distribution: Distribution = data.0.into();

    let _query = state.store.upload_package(distribution).await;
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

#[axum_macros::debug_handler]
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
