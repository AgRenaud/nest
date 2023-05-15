use std::net::{TcpListener, SocketAddr};
use std::sync::Arc;

use axum::Router;
use axum::routing::get;

use object_store::local::LocalFileSystem;
use surrealdb::opt::auth::Root;
use surrealdb::{engine::remote::ws::Ws, Surreal};

use tower::ServiceBuilder;
use tower_http::{
    ServiceBuilderExt,
    request_id::MakeRequestUuid,
    trace::TraceLayer
};

use crate::greeting;
use crate::persistence::Store;
use crate::routes::healthcheck::healthcheck;
use crate::routes::home::home;
use crate::routes::simple;
use crate::settings;
use crate::telemetry::{MakeSpan, OnResponse};

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

        let db = Surreal::new::<Ws>(config.persistence.database.address.clone())
            .await
            .expect("Unable to reach db ! Please check your configuration.");

        db.signin(Root {
            username: config.persistence.database.user.as_str(),
            password: config.persistence.database.password.as_str(),
        })
        .await
        .expect("Unable to connect to db.");

        // TODO: Move use of namespace and repository to api user.
        db.use_ns("global")
            .use_db("repository")
            .await
            .expect("Unable to get namespace and database");

        let db = Arc::new(db);
        let store = Arc::new(Store::new(db, store));
        let state = simple::SimpleController { store };

        let app = Router::new()
            .nest("/", simple::router(state))
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
            .serve(self.app
                .layer(middleware)
                .into_make_service_with_connect_info::<SocketAddr>())
            .await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
