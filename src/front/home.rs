use axum::response::IntoResponse;
use axum_template::RenderHtml;

use crate::engine::AppEngine;

pub async fn home(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("home/home.jinja", engine, &())
}
