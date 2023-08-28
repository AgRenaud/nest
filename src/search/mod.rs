use maud::{html, Markup};

use crate::components::header;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;


pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new() 
        .layer(AddExtensionLayer::new(db_pool)) 
        .into_inner();

    Router::new()
        .route("/", get(search)) <- (path, method_router, handler) 
        .layer(middleware)
}

pub fn search_bar() -> Markup {
    html!{

    }
}

pub async fn sign_up() -> Markup {
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
            div class="ma w-100" {
                (sign_up_form(None))
            }
        }
    )
  }