use axum::http::StatusCode;

pub async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

#[cfg(test)]
mod tests {
    // Just for example purpose this tests is useless and will be remove later
    use crate::healthcheck::healthcheck;

    #[tokio::test]
    async fn healthcheck_succeeds() {
        let response = healthcheck().await;

        assert!(response.is_success())
    }
}
