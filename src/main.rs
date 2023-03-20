use hyper::{header, StatusCode, };
use std::path::Path;

use axum::{
    body::{self, Empty, Full},
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

mod simple;
mod app_state;

use simple::{upload, list_packages};
use app_state::AppState;


#[tokio::main]
async fn main() {
    let static_dir = String::from("./static");
    let index_dir = String::from("./simple-index");

    let state = AppState { static_dir, index_dir };

    let app = Router::new()
        .route("/simple", post(upload))
        .route("/simple", get(list_packages))
        .route("/", get(index_html))
        .route("/index.js", get(index_js))
        .with_state(state);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_html(State(state): State<AppState>) -> impl IntoResponse {
    let path = state.static_dir;
    let path = Path::new(path.as_str()).join("index.html");

    if path.is_file() {
        let file = std::fs::read(path);

        match file {
            Ok(file) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str("text/html").unwrap(),
                )
                .body(body::boxed(Full::from(file)))
                .unwrap(),
            Err(e) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}

async fn index_js(State(state): State<AppState>) -> impl IntoResponse {
    let path = state.static_dir;
    let path = Path::new(path.as_str()).join("index.js");

    if path.is_file() {
        let file = std::fs::read(path);

        match file {
            Ok(file) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str("text/javascript").unwrap(),
                )
                .body(body::boxed(Full::from(file)))
                .unwrap(),
            Err(_e) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))
                .unwrap(),
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}