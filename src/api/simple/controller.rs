use std::sync::Arc;

use crate::db;

#[derive(Clone)]
pub struct SimpleController {
    pub static_dir: String,
    pub store: Arc<dyn db::SimpleStore>,
}
