use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::headers::authorization::Basic;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use hyper::{Request, StatusCode};

use super::users::{AuthSession, Credentials};

pub async fn auth(
    auth_session: AuthSession,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let username = auth.username().to_string();
    let password = auth.password().to_string();

    let credentials = Credentials { username, password };

    if let Ok(user) = auth_session.authenticate(credentials).await {
        if let Some(current_user) = user {
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
