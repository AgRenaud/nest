use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{routing::get, Router};
use axum_login::tower_sessions::{Expiry, SessionManagerLayer};
use axum_login::AuthManagerLayerBuilder;
use axum_template::engine::Engine;

use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use object_store::local::LocalFileSystem;
use tokio::net::TcpListener as TokioTcpListener;

use tokio::signal;
use tokio::task::AbortHandle;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::request_id::MakeRequestUuid;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tower_http::{trace::TraceLayer, ServiceBuilderExt};
use tower_sessions::cookie::Key;
use tower_sessions::session_store::ExpiredDeletion;
use tower_sessions_sqlx_store::PostgresStore;
use tracing::Level;

use crate::authentication::Backend;
use crate::front;
use crate::greeting;
use crate::healthcheck::healthcheck;
use crate::settings;
use crate::simple::{self, store::Store};
use crate::state::AppState;
use sqlx::postgres::PgPoolOptions;

pub struct Application {
    app: Router,
    session_store: PostgresStore,
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
            .acquire_timeout(std::time::Duration::from_secs(2))
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

        let key = Key::generate();

        let session_store = PostgresStore::new(db_pool.clone());
        session_store
            .migrate()
            .await
            .expect("Unable to create sessions");

        let session_layer = SessionManagerLayer::new(session_store.clone())
            .with_secure(false)
            .with_http_only(true)
            .with_signed(key)
            .with_expiry(Expiry::OnInactivity(time::Duration::days(15)));

        let backend = Backend::new(db_pool.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        let app_state = AppState {
            engine: Engine::from(jinja),
            store: simple_store,
        };

        let app = Router::new()
            .nest("/", front::router().layer(auth_layer))
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

        Application {
            app,
            session_store,
            listener,
        }
    }

    pub async fn run(self) {
        greeting::greets(&self.address());

        let app = self.app.clone();
        let listener = TokioTcpListener::from_std(self.listener).unwrap();

        let deletion_task = tokio::task::spawn(
            self.session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await
            .unwrap();
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
