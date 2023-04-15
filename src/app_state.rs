use bytes::Bytes;
use std::sync::Arc;

use crate::store;

#[derive(Clone)]
pub struct AppState {
    pub static_dir: String,
    pub store: Arc<dyn store::SimpleStore>,
}
