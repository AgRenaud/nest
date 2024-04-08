use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use axum_login::login_required;

use crate::{
    authentication::{AuthSession, Backend},
    state::AppState,
};

pub mod sign_in;
pub mod sign_up;

pub fn router() -> Router<AppState> {
    Router::new()
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
        .route_layer(login_required!(Backend, login_url = "/manage/sign_in"))
        .route("/sign_in", get(sign_in::sign_in))
        .route("/login", post(sign_in::login))
        .route("/create_user", post(sign_up::create_user))
        .route("/sign_up", get(sign_up::sign_up))
}
