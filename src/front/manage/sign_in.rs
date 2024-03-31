#![allow(dead_code)]
#![allow(unused_variables)]

use axum::{response::IntoResponse, Form};
use axum_login::{tower_sessions::Session, AuthUser};
use axum_template::RenderHtml;
use serde::Deserialize;

use crate::{
    authentication::{AuthSession, Credentials, UserSession, USER_SESSION_KEY},
    engine::AppEngine,
};

pub async fn sign_in(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("sign_in/sign_in.jinja", engine, &())
}

#[derive(Deserialize)]
pub struct SignInForm {
    username: String,
    password: String,
}

pub async fn login(mut auth_session: AuthSession, session: Session, Form(form): Form<SignInForm>) {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };

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

                println!("{:?}", user.id());
            }
        } else {
            println!("Login error, unable to find a valid user");
            todo!()
        }
    } else {
        // Return a 401 bad credentials error*
        // Must be materialized by a modal message on html
        println!("Login error, unable to use authenticate");
        todo!()
    }
}
