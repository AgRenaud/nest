use axum::response::IntoResponse;



use crate::{authentication::AuthSession, engine::AppEngine};

pub async fn profile(_engine: AppEngine, _auth_session: AuthSession) -> impl IntoResponse {}
