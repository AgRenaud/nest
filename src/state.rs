use std::sync::Arc;

use axum::extract::FromRef;

use crate::{engine::AppEngine, simple::simple_api};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: AppEngine,
    pub store: Arc<dyn simple_api::SimpleStore>,
}
