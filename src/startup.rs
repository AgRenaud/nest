use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::{routing::get, Router};

use axum_template::engine::Engine;

use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use object_store::local::LocalFileSystem;
use tokio::net::TcpListener as TokioTcpListener;

use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::request_id::MakeRequestUuid;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tower_http::{trace::TraceLayer, ServiceBuilderExt};
use tracing::Level;

use crate::front;
use crate::greeting;
use crate::healthcheck::healthcheck;
use crate::settings;
use crate::simple::{self, store::Store};
use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

impl Application {
    pub async fn build(config: settings::Settings) -> Self {
        std::fs::create_dir_all(config.persistence.object_storage.path.clone())
            .expect("Unable to create folder for wheels");
        let storage =
            LocalFileSystem::new_with_prefix(config.persistence.object_storage.path.clone())
                .expect("Unable to set up local index.");
        let store = Arc::new(storage);

        let db_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_lazy_with(config.persistence.database.with_db());

        tracing::info!("Run migrations on {}", &config.persistence.database.host);
        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Unable to run migrations");

        let simple_store = Store::new(db_pool.clone(), store);
        let simple_store = Arc::new(simple_store);

        let jinja = AutoReloader::new(move |notifier| {
            let template_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");

            let mut env = Environment::new();
            env.set_loader(path_loader(&template_path));
            notifier.set_fast_reload(true);
            notifier.watch_path(&template_path, true);
            Ok(env)
        });

        let app_state = AppState {
            engine: Engine::from(jinja),
            store: simple_store,
        };

        let app = Router::new()
            .nest("/", front::router())
            .nest("/simple", simple::router())
            .with_state(app_state)
            .route("/healthcheck", get(healthcheck));

        let db_middleware = ServiceBuilder::new()
            .layer(AddExtensionLayer::new(db_pool))
            .into_inner();

        let trace_middleware = ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new()
                            .include_headers(true)
                            .level(Level::INFO),
                    )
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            .propagate_x_request_id()
            .into_inner();

        let app = app.layer(db_middleware).layer(trace_middleware);

        let addr = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        Application { app, listener }
    }

    pub async fn run(self) {
        greeting::greets(&self.address());

        let app = self.app.clone();
        let listener = TokioTcpListener::from_std(self.listener).unwrap();

        axum::serve(listener, app).await.unwrap();
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
