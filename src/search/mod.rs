use axum::{extract::{Extension, Path}, routing::{post, get}, Form, Router, response::{Response, IntoResponse}};
use hyper::StatusCode;
use maud::{html, Markup};
use serde::Deserialize;
use sqlx::PgPool;

use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(db_pool))
        .into_inner();

    Router::new()
        .route("/", post(search))
        .route("/documentation/:project", get(show_documentation))
        .layer(middleware)
}

pub fn search_bar() -> Markup {
    html! {
        div class="w-100% relative flex-content-center" {
            h1 { "Search Package" }
            input class="p1 h10 w100% b-3px b-rd-2 b-s-solid outline-none transition-all"
                type="search"
                placeholder="Search package.."
                hx-post="/search"
                name="search"
                hx-trigger="keyup changed delay:250ms, search"
                hx-target="#results"
                hx-swap="innerHTML"
                hx-indicator="#search-indicator";
            a href="#" { "Show all (simple index)" }
            span id="search-indicator" class="htmx-indicator" { "Searching.." }
            div id="results" {}
        }
    }
}

pub async fn show_documentation(Path(package): Path<String>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("HX-Redirect", format!("/packages/{}/latest", &package))],
    )
}

#[derive(Deserialize)]
pub struct Query {
    pub search: String,
}

#[derive(Deserialize)]
struct Package {
    pub name: String,
    pub has_docs: bool,
}

pub async fn search(Extension(pool): Extension<PgPool>, Form(query): Form<Query>) -> Markup {
    let query = query.search;

    if query.trim().is_empty() {
        return html! {};
    }

    let packages = sqlx::query_as!(
        Package,
        r#"
        SELECT name, has_docs FROM projects
        WHERE normalized_name LIKE (normalize_pep426_name($1) || '%')
        "#,
        query
    )
    .fetch_all(&pool)
    .await;


    match packages {
        Ok(packages) => {
            html! {
                ul class="min-w300px m-auto list-none p0" {
                    @for package in packages {
                        li
                            hx-get=(format!("/search/documentation/{}", &package.name))
                            class="
                            flex justify-between center bg-#f0f0f0
                            b-1px b-s-solid b-#ccc b-rd-5px
                            shadow-2px
                            p10px
                            m-b-10px
                            transition-opacity
                            hover-bg-black hover-color-white hover-cursor-pointer" {
                            p class="m0" { (&package.name) }
                            div class="flex justify" {
                                @if package.has_docs { p class="m1px font-20px" { "🗎" } }
                                @else { p class="m1px font-20px" { "" } }
                            }
                        }
                    }
                }
            }
        },
        Err(_) => html! {
            p class="red" { "Oups. There is an error" }
        },
    }
}
