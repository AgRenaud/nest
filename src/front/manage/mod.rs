use axum::{
    routing::{get, post},
    Router,
};

use crate::state::AppState;

pub mod sign_in;
pub mod sign_up;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/login", post(sign_in::login))
        .route("/create_user", post(sign_up::create_user))
        .route("/sign_up", get(sign_up::sign_up))
}
