use axum::{
    extract::{Extension, Path},
    routing::get,
    Router,
};
use maud::{html, Markup, DOCTYPE, PreEscaped};
use sqlx::PgPool;

use crate::components::header;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

pub fn router(db_pool: PgPool) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(AddExtensionLayer::new(db_pool))
        .into_inner();

    Router::new()
        .route("/:project/:version", get(documentation))
        .layer(middleware)
}

pub async fn documentation_content(pool: PgPool, project: &str, version: &str) -> Markup {

    let version = match version {
        v if v.eq("latest") => {
            let latest_version = sqlx::query!(r#"
                WITH selected_project AS (
                    SELECT id
                    FROM projects
                    WHERE normalized_name = normalize_pep426_name($1)
                )
                SELECT r.version AS version
                FROM selected_project sp
                    JOIN releases r
                        ON sp.id = r.project_id
                ORDER BY r.version DESC
                LIMIT 1
                "#,
                project)
                .fetch_one(&pool)
                .await
                .expect("Unable to get latest version");

            latest_version.version
        },
        v => v.to_string()
    };

    let html_content = sqlx::query!(
        r#"
        WITH selected_project AS (
            SELECT id
            FROM projects
            WHERE normalized_name = normalize_pep426_name($1)
        )
        SELECT rd.html AS html
        FROM selected_project sp
            JOIN releases r
                ON sp.id = r.project_id
            JOIN release_descriptions rd
                ON r.id = rd.release_id
            WHERE r.version = $2
            LIMIT 1
        "#,
        project,
        version)
        .fetch_one(&pool)
        .await
        .expect("Unable to fetch html content")
        .html;

    html! {
        div class="border-10 max-w-2xl m-auto" {
            (PreEscaped(html_content))
        }
    }
}

pub async fn documentation(
    Extension(pool): Extension<PgPool>,
    Path((project, version)): Path<(String, String)>,
) -> Markup {
    let doc = documentation_content(pool, &project, &version).await;

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
