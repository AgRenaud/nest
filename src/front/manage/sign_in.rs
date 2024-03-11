#![allow(dead_code)]
#![allow(unused_variables)]

use axum::response::IntoResponse;
use axum_template::RenderHtml;
use serde::Deserialize;

use crate::engine::AppEngine;

pub async fn sign_in(engine: AppEngine) -> impl IntoResponse {
    RenderHtml("sign_in/sign_in.jinja", engine, &())
}

#[derive(Deserialize)]
pub struct SignIn {
    username: String,
    password: String,
}
