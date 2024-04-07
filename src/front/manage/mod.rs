use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};

use crate::{authentication::AuthSession, state::AppState};

pub mod sign_in;
pub mod sign_up;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign_in", get(sign_in::sign_in))
        .route("/login", post(sign_in::login))
        .route("/create_user", post(sign_up::create_user))
        .route("/sign_up", get(sign_up::sign_up))
        .route(
            "/logout",
            get(|mut auth_session: AuthSession| async move {
                auth_session
                    .logout()
                    .await
                    .expect("Unable to retrieve user session");
                Redirect::to("/")
            }),
        )
}
