use crate::package::Package;

use axum::routing::{get, post};
use axum::Router;
use axum::{
    extract::{Path, State},
    response::Json,
};
use axum_typed_multipart::TypedMultipart;

use super::models::{ProjectDists, RequestData, SimpleIndex};
use super::SimpleController;

pub fn routes(state: SimpleController) -> Router {
    Router::new()
        .route("/simple", post(upload).get(list_packages))
        .route("/simple/:project", get(list_dists))
        .with_state(state)
}

async fn upload(State(state): State<SimpleController>, data: TypedMultipart<RequestData>) {
    let package: Package = data.0.into();

    let _query = state.store.upload_package(package).await;
}

async fn list_packages(State(state): State<SimpleController>) -> Json<SimpleIndex> {
    let projects = state.store.get_projects().await.unwrap();

    let packages = projects.iter().map(|p| p.name.to_owned()).collect();

    Json(SimpleIndex { packages })
}

async fn list_dists(
    Path(project): Path<String>,
    State(state): State<SimpleController>,
) -> Json<ProjectDists> {
    let dists = state.store.get_dists(project).await.unwrap();

    let dists = dists.iter().map(|d| d.filename.to_owned()).collect();

    Json(ProjectDists { dists })
}
