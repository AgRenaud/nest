use crate::package::Package;

use axum::routing::post;
use axum::Router;
use axum::{extract::State, response::Json};
use axum_typed_multipart::TypedMultipart;

use super::models::{RequestData, SimpleIndex};
use super::SimpleController;

pub fn routes(state: SimpleController) -> Router {
    Router::new()
        .route("/simple", post(upload).get(list_packages))
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
