use axum::response::IntoResponse;
use axum_template::RenderHtml;
use minijinja::context;

use crate::{authentication::AuthSession, engine::AppEngine};

pub async fn home(engine: AppEngine, auth_session: AuthSession) -> impl IntoResponse {
    if let Some(_uid) = auth_session.user {
        RenderHtml(
            "home/home.jinja",
            engine,
            context! { is_authenticated => true },
        )
    } else {
        RenderHtml(
            "home/home.jinja",
            engine,
            context! { is_authenticated => false },
        )
    }
}
