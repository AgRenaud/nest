use maud::{html, Markup};
use serde::Deserialize;
use axum::{
    Form,
    routing::post,
    extract::Extension,
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
        .route("/", post(search))
        .layer(middleware)
}

pub fn search_bar() -> Markup {
    html!{
        div class="w-100% relative flex-content-center" {
            input class="p5px h20px w100% b-3px b-rd-2 b-s-solid outline-none"
                type="search"
                placeholder="Search package.."
                hx-post="/search"
                name="search"
                hx-trigger="keyup changed delay:500ms, search"
                hx-target="#results"
                hx-swap="innerHTML"
                hx-indicator="#spinner";
            span id="spinner" class="htmx-indicator" { "Searching.." }
            div id="results" {}
        }
    }
}


#[derive(Deserialize)]
pub struct Query {
    pub search: String
}


#[derive(Deserialize)]
struct Package {
    name: String
}

pub async fn search(Extension(pool): Extension<PgPool>, Form(query): Form<Query>) -> Markup {

    let query = query.search;

    let packages = sqlx::query_as!(Package, r#"
        SELECT name FROM projects
        WHERE normalized_name LIKE (normalize_pep426_name($1) || '%')
        "#,
        query)
        .fetch_all(&pool)
        .await;

    match packages {
        Ok(packages) => html! {
            ul {
                @for package in packages {
                    li { (package.name) }
                }
            }
        },
        Err(_) => html! {
            p class="red" { "Oups. There is an error" }
        }
    }
}
