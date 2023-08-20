use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use std::time::Duration;

use axum::routing::get;
use axum::Router;

use object_store::local::LocalFileSystem;

use tower::ServiceBuilder;
use tower_http::{request_id::MakeRequestUuid, trace::TraceLayer, ServiceBuilderExt};

use crate::greeting;
use crate::persistence::Store;
use crate::routes::healthcheck::healthcheck;
use crate::routes::home::home;
use crate::routes::simple::{self, SimpleState};
use crate::routes::manage;
use crate::settings;
use crate::telemetry::{MakeSpan, OnResponse};
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

impl Application {
    pub async fn build(config: settings::Settings) -> Self {
        let storage =
            LocalFileSystem::new_with_prefix(config.persistence.object_storage.path.clone())
                .expect("Unable to set up local index.");
        let store = Arc::new(storage);

        let db_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_lazy_with(config.persistence.database.with_db());

        let simple_store = Store::new(db_pool.clone(), store);
        let simple_store = Arc::new(simple_store);

        let simple_state = SimpleState {
            store: simple_store,
        };

        let app = Router::new()
            .nest("/simple", simple::router(simple_state))
            .nest("/manage", manage::router(db_pool.clone()))
            .route("/healthcheck", get(healthcheck))
            .route("/", get(home));

        let addr = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(addr).unwrap();

        Application { app, listener }
    }

    pub async fn run(self) -> Result<(), hyper::Error> {
        greeting::greets(&self.address());

        let middleware = ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(MakeSpan)
                    .on_response(OnResponse),
            )
            .propagate_x_request_id()
            .into_inner();

        hyper::Server::from_tcp(self.listener)?
            .serve(
                self.app
                    .layer(middleware)
                    .into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
