use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_template::RenderHtml;
use minijinja::context;
use sqlx::PgPool;

use crate::{engine::AppEngine, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/:project/:version", get(documentation))
}

pub async fn documentation_content(pool: PgPool, project: &str, version: &str) -> String {
    let version = match version {
        v if v.eq("latest") => {
            let latest_version = sqlx::query!(
                r#"
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
                project
            )
            .fetch_one(&pool)
            .await
            .expect("Unable to get latest version");

            latest_version.version
        }
        v => v.to_string(),
    };

    sqlx::query!(
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
        version
    )
    .fetch_one(&pool)
    .await
    .expect("Unable to fetch html content")
    .html
}

pub async fn documentation(
    engine: AppEngine,
    Extension(pool): Extension<PgPool>,
    Path((project, version)): Path<(String, String)>,
) -> impl IntoResponse {
    let doc = documentation_content(pool, &project, &version).await;
    RenderHtml(
        "documentation.jinja",
        engine,
        context! { package_name => project, content => doc },
    )
}
