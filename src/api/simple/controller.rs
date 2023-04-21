use std::sync::Arc;

use crate::db;

#[derive(Clone)]
pub struct SimpleController {
    pub store: Arc<dyn db::SimpleStore>,
}
