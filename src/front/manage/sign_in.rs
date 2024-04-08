#![allow(dead_code)]
#![allow(unused_variables)]

use axum::{body::Body, response::IntoResponse, Form};
use axum_login::{tower_sessions::Session, AuthUser};
use axum_template::RenderHtml;
use hyper::{HeaderMap, StatusCode};
use minijinja::context;

use crate::{
    authentication::{AuthSession, Credentials, UserSession, USER_SESSION_KEY},
    engine::AppEngine,
};

pub async fn sign_in(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("sign_in/sign_in.jinja", engine, &())
}

pub async fn login(
    engine: AppEngine,
    mut auth_session: AuthSession,
    session: Session,
    Form(credentials): Form<Credentials>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    match auth_session.authenticate(credentials).await {
        Ok(Some(user)) => {
            if auth_session.login(&user).await.is_ok() {
                session
                    .insert(USER_SESSION_KEY, UserSession { user_id: user.id() })
                    .await
                    .unwrap();
                tracing::info!("{:?}", user.id());
                headers.insert("HX-Redirect", "/".parse().unwrap());
                (StatusCode::OK, headers, Body::empty())
            } else {
                (
                StatusCode::INTERNAL_SERVER_ERROR,
                headers,
                RenderHtml("sign_in/components/sign_in_error.jinja", engine, context! { message => "Internal server error. Contact your administrator." }).into_response().into_body()
            )
            }
        }
        Ok(None) => {
            tracing::info!("Login error, unable to find a valid user");
            (
                StatusCode::UNAUTHORIZED,
                headers,
                RenderHtml(
                    "sign_in/components/sign_in_error.jinja",
                    engine,
                    context! { message => "Wrong credentials. Check your username and password." },
                )
                .into_response()
                .into_body(),
            )
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            headers,
            RenderHtml(
                "sign_in/components/sign_in_error.jinja",
                engine,
                context! { message => "Wrong credentials. Check your username and password." },
            )
            .into_response()
            .into_body(),
        ),
    }
}
