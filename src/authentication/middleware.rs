use axum::middleware::Next;
use axum::{
    extract::Extension,
    headers::{authorization::Basic, Authorization},
    response::Response,
    TypedHeader,
};
use hyper::{Request, StatusCode};
use sqlx::PgPool;

use crate::authentication::{self, Credentials};

pub async fn auth<B>(
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    Extension(pool): Extension<PgPool>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let username = auth.username().to_string();
    let password = secrecy::Secret::new(auth.password().to_string());
    let credentials = Credentials { username, password };

    if let Ok(current_user) = authentication::validate_credentials(credentials, &pool).await {
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
