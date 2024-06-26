use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::Extension, Form};
use axum_template::RenderHtml;
use minijinja::context;
use serde::Deserialize;

use password_auth::generate_hash;

use crate::engine::AppEngine;

use sqlx::PgPool;

pub async fn sign_up(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("sign_up/sign_up.jinja", engine, &())
}

#[derive(Deserialize)]
pub struct SignUp {
    username: String,
    password: String,
    confirm_password: String,
}

#[tracing::instrument(name = "Manage::Create user", skip(engine, pool, form))]
pub async fn create_user(
    engine: AppEngine,
    Extension(pool): Extension<PgPool>,
    Form(form): Form<SignUp>,
) -> impl IntoResponse {
    if form.password != form.confirm_password {
        let error_message = String::from("Password are not the same. Please check your password.");

        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            [
                (header::CONTENT_TYPE, "text/plain"),
                (header::CONTENT_ENCODING, "utf-8"),
            ],
            RenderHtml(
                "sign_up/components/sign_up_error.jinja",
                engine,
                context! {
                    message => error_message
                },
            ),
        );
    }

    let password = form.password;
    let password_hash = generate_hash(password);

    let user_created = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1::TEXT::CITEXT, $2)
        "#,
        &form.username,
        password_hash,
    )
    .execute(&pool)
    .await;

    match user_created {
        Ok(_) => {
            let message = format!("User {} has been succefully created", &form.username);

            (
                StatusCode::CREATED,
                [
                    (header::CONTENT_TYPE, "text/plain"),
                    (header::CONTENT_ENCODING, "utf-8"),
                ],
                RenderHtml(
                    "sign_up/components/sign_up_success.jinja",
                    engine,
                    context! { message => message },
                ),
            )
        }
        Err(e) => {
            let err = e.into_database_error();

            let error_kind: &sqlx::error::ErrorKind = &err
                .as_ref()
                .map_or(sqlx::error::ErrorKind::Other, |err| err.kind());

            match &error_kind {
                sqlx::error::ErrorKind::UniqueViolation => {
                    let error_message = format!("User {} already exists.", &form.username);

                    (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        [
                            (header::CONTENT_TYPE, "text/plain"),
                            (header::CONTENT_ENCODING, "utf-8"),
                        ],
                        RenderHtml(
                            "sign_up/components/sign_up_error.jinja",
                            engine,
                            context! { message => error_message },
                        ),
                    )
                }
                _ => {
                    let error_message = &"Unexpected error !";
                    (
                        StatusCode::UNPROCESSABLE_ENTITY,
                        [
                            (header::CONTENT_TYPE, "text/plain"),
                            (header::CONTENT_ENCODING, "utf-8"),
                        ],
                        RenderHtml(
                            "sign_up/components/sign_up_error.jinja",
                            engine,
                            context! { message => error_message },
                        ),
                    )
                }
            }
        }
    }
}
