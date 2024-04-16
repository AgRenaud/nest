use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_template::RenderHtml;
use minijinja::context;
use serde::Deserialize;
use sqlx::PgPool;
use crate::simple::package::CoreMetadata;

use crate::{engine::AppEngine, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/:project/:version", get(documentation))
}

async fn get_version(pool: &PgPool, project: &str, version: &str) -> String {
    match version {
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
            .fetch_one(pool)
            .await
            .expect("Unable to get latest version");

            latest_version.version
        }
        v => v.to_string(),
    }
}

async fn documentation_content(pool: &PgPool, project: &str, version: &str) -> String {
    let version = get_version(pool, project, version).await;

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
    .fetch_one(pool)
    .await
    .expect("Unable to fetch html content")
    .html
}

#[derive(Deserialize)]
pub struct ReleaseInfo {
    pub version: String,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub license: Option<String>,
    pub home_page: Option<String>,
    keywords: Option<String>
}

impl ReleaseInfo {
    pub fn keywords_list(&self) -> Vec<String> {
        if let Some(keywords) = &self.keywords {
            keywords.split(",")
                .into_iter()
                .map(|kw| kw.to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
}

async fn package_meta(pool: &PgPool, project: &str, version: &str) -> ReleaseInfo {
    let version = get_version(pool, project, version).await;

    sqlx::query_as!(ReleaseInfo, r#"
        WITH selected_project AS (
            select p.id as project_id
            from projects p
            where p.normalized_name = normalize_pep426_name($1)
        )
        SELECT
            r.version AS version,
            r.author AS author,
            r.author_email AS author_email,
            r.home_page AS home_page,
            r.license AS license,
            r.keywords AS keywords
        FROM releases r
        JOIN selected_project sp
        ON sp.project_id = r.project_id
        WHERE r.version = $2"#,
        project, version)
            .fetch_one(pool)
            .await
            .unwrap()
}

pub async fn documentation(
    engine: AppEngine,
    Extension(pool): Extension<PgPool>,
    Path((project, version)): Path<(String, String)>,
) -> impl IntoResponse {
    let doc = documentation_content(&pool, &project, &version).await;
    let info = package_meta(&pool, &project, &version).await;

    RenderHtml(
        "documentation.jinja",
        engine,
        context! {
            package_name => project,
            content => doc,
            keywords => info.keywords_list(),
            home_page => info.home_page,
            author => info.author
        },
    )
}
