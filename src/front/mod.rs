use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

mod documentation;
mod home;
mod manage;
mod search;
mod serve_static;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home::home))
        .route("/search", post(search::search_package))
        .route("/search/doc/:project", get(search::show_documentation))
        .nest("/manage", manage::router())
        .nest("/packages", documentation::router())
        .nest_service("/static", serve_static::static_router("static"))
}
