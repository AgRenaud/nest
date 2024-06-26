use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    Form,
};
use axum_template::RenderHtml;
use hyper::StatusCode;
use minijinja::context;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::engine::AppEngine;

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

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    pub name: String,
    pub has_docs: bool,
}

pub async fn search_package(
    Extension(pool): Extension<PgPool>,
    engine: AppEngine,
    Form(query): Form<Query>,
) -> impl IntoResponse {
    let query = query.search;

    let mut package_list = Vec::new();

    if !query.trim().is_empty() {
        let mut packages = sqlx::query_as!(
            Package,
            r#"
        SELECT name, has_docs FROM projects
        WHERE normalized_name LIKE (normalize_pep426_name($1) || '%')
        "#,
            query
        )
        .fetch_all(&pool)
        .await;

        if let Ok(packages) = &mut packages {
            package_list.append(packages);
        }
    };

    let ctx = context! {
        packages => package_list
    };

    RenderHtml("home/components/package-card.jinja", engine, ctx)
}
