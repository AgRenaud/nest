mod api;
mod db;
mod greeting;
mod package;

use std::sync::Arc;

use object_store::local::LocalFileSystem;
use surrealdb::opt::auth::Root;
use surrealdb::{engine::remote::ws::Ws, Surreal};

use axum::Router;

use api::{simple_routes, SimpleController};
use db::Store;

#[tokio::main]
async fn main() {
    env_logger::init();

    let static_dir = String::from("./static");
    println!("{}", greeting::LOGO);
    let server_addr = "127.0.0.1:8080".parse().unwrap();

    log::info!("Serve API at {}", server_addr);

    let storage =
        LocalFileSystem::new_with_prefix("simple-index").expect("Unable to set up local index.");
    let store = Arc::new(storage);

    let db = Surreal::new::<Ws>("localhost:8000")
        .await
        .expect("Unable to reach db");

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Unable to connect to db.");
    db.use_ns("global")
        .use_db("packages")
        .await
        .expect("Unable to get namespace and database");

    let db = Arc::new(db);

    let store = Arc::new(Store::new(db, store));

    let state = SimpleController { static_dir, store };

    let app = Router::new().merge(simple_routes(state.clone()));

    axum::Server::bind(&server_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
