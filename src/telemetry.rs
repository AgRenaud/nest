use std::net::SocketAddr;

use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};


use axum::extract::ConnectInfo;
use hyper::Request;

use tokio::task::JoinHandle;
use tracing::{field::Empty};


pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync    
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);    

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}        

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}    


// From https://github.com/mattiapenati/zero2prod-dev/blob/main/src/trace.rs

#[derive(Clone, Copy, Debug)]
pub struct MakeSpan;

impl<B> tower_http::trace::MakeSpan<B> for MakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> tracing::Span {
        let ConnectInfo(socket_addr) = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .unwrap();

        let request_id = request
            .headers()
            .get("x-request-id")
            .unwrap()
            .to_str()
            .unwrap();

        let http_target = match request.uri().path_and_query() {
            Some(http_target) => http_target.as_str(),
            None => request.uri().path(),
        };

        tracing::info_span!(
            "HTTP request",
            http.method = %request.method(),
            http.target = %http_target,
            http.status_code = Empty,
            net.sock.host.addr = %socket_addr.ip(),
            net.sock.host.port = socket_addr.port(),
            request_id = %request_id,
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OnResponse;

impl<B> tower_http::trace::OnResponse<B> for OnResponse {
    fn on_response(
        self,
        response: &hyper::Response<B>,
        _latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        span.record("http.status_code", response.status().as_u16());
    }
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}