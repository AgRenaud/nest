use axum::{extract::{Extension, Path}, routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use sqlx::PgPool;

use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use crate::components::header;


pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(db_pool))
        .into_inner();

    Router::new()
        .route("/:project/:version", get(documentation))
        .layer(middleware)
}


pub async fn documentation_content(project: &str, version: &str) -> Markup {
    html! {
        p { "Hello World " (project) " - " (version) }
    }
}


pub async fn documentation(Path((project, version)): Path<(String, String)>) -> Markup {

    let doc = documentation_content(&project, &version).await;

    html!(
        (DOCTYPE)
        head {
            meta charset="utf-8";
            script src="https://unpkg.com/htmx.org@1.9.3" {};
            script src="https://cdn.jsdelivr.net/npm/@unocss/runtime" {};
            title { "Nest" }
        }
        body class="m0 p0 font-sans" {
            (header())
            (doc)
        }
    )
}

