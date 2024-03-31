use axum::extract::FromRef;
use std::sync::Arc;

use crate::{engine::AppEngine, simple::simple_api};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: AppEngine,
    pub store: Arc<dyn simple_api::SimpleStore>,
}
