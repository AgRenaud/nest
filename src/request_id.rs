// Take a look at the following links :
// - https://github.com/mattiapenati/zero2prod/blob/main/src/request_id.rs
// - https://github.com/tower-rs/tower-http/blob/master/tower-http/src/trace/make_span.rs#L11
// - https://github.com/tokio-rs/axum/discussions/1261

use std::{fmt, ops::Deref};

use axum::response::{IntoResponse, Response};
use axum::http:: Request;
use tracing::{Level, Span};
use uuid::Uuid;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RequestId(Uuid);


#[derive(Debug)]
pub struct MissingRequestId;

impl IntoResponse for MissingRequestId {
    fn into_response(self) -> Response {
        hyper::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl Deref for RequestId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl RequestId {
    #[inline]
    fn new() -> Self {
        RequestId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AddRequestIdLayer;

impl<S> tower::Layer<S> for AddRequestIdLayer {
    type Service = AddRequestId<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AddRequestId { inner }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AddRequestId<S> {
    inner: S,
}

impl<B, S> tower::Service<Request<B>> for AddRequestId<S>
where
    S: tower::Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        req.extensions_mut().insert(RequestId::new());
        self.inner.call(req)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UseRequestId;

impl tower_http::request_id::MakeRequestId for UseRequestId {
    fn make_request_id<B>(
        &mut self,
        req: &Request<B>,
    ) -> Option<tower_http::request_id::RequestId> {
        req.extensions().get::<RequestId>().map(|request_id| {
            hyper::http::HeaderValue::from_str(&request_id.to_string())
                .unwrap()
                .into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct MakeSpanWithRequestId {
    level: Level,
}

impl MakeSpanWithRequestId {
    pub fn new() -> Self {
        MakeSpanWithRequestId {
            level: Level::DEBUG,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl Default for MakeSpanWithRequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl<B> tower_http::trace::MakeSpan<B> for MakeSpanWithRequestId {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        macro_rules! make_span {
            ($level:expr) => {
                if let Some(request_id) = req.extensions().get::<RequestId>() {
                    tracing::span!(
                        $level,
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                        request_id = %request_id,
                    )
                } else {
                    tracing::span!(
                        $level,
                        "request",
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                    )
                }
            }
        }

        match self.level {
            Level::ERROR => {
                make_span!(Level::ERROR)
            }
            Level::WARN => {
                make_span!(Level::WARN)
            }
            Level::INFO => {
                make_span!(Level::INFO)
            }
            Level::DEBUG => {
                make_span!(Level::DEBUG)
            }
            Level::TRACE => {
                make_span!(Level::TRACE)
            }
        }
    }
}