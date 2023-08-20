use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod sign_in;
mod sign_up;

pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(db_pool))
        .into_inner();

    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/create_user", post(sign_up::create_user))
        .route("/sign_up", get(sign_up::sign_up))
        .layer(middleware)
}
