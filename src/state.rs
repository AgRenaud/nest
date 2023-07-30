use std::sync::Arc;

use crate::simple_api;


pub struct AppState {
    pub store: Arc<dyn simple_api::SimpleStore>,
}