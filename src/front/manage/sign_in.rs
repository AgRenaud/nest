#![allow(dead_code)]
#![allow(unused_variables)]

use axum::{response::IntoResponse, Form};
use axum_login::{tower_sessions::Session, AuthUser};
use axum_template::RenderHtml;
use hyper::StatusCode;

use crate::{
    authentication::{AuthSession, Credentials, UserSession, USER_SESSION_KEY},
    engine::AppEngine,
};

pub async fn sign_in(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("sign_in/sign_in.jinja", engine, &())
}

pub async fn login(
    mut auth_session: AuthSession,
    session: Session,
    Form(credentials): Form<Credentials>,
) -> impl IntoResponse {
    if let Ok(auth) = auth_session.authenticate(credentials).await {
        if let Some(user) = auth {
            // Create a session in db;
            //
            // Append Cookie to response
            //
            // Return template with cookie
            // add hx-redirect to header (redirect to home )

            if auth_session.login(&user).await.is_ok() {
                session
                    .insert(USER_SESSION_KEY, UserSession { user_id: user.id() })
                    .await
                    .unwrap();

                tracing::info!("{:?}", user.id());

                (StatusCode::OK, [("HX-Redirect", "/")])
            } else {
                todo!()
            }
        } else {
            tracing::info!("Login error, unable to find a valid user");
            todo!()
        }
    } else {
        // Return a 401 bad credentials error*
        // Must be materialized by a modal message on html
        tracing::info!("Login error, unable to use authenticate");
        todo!()
    }
}
