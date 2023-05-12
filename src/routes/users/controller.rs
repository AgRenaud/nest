use std::sync::Arc;

use crate::persistence;

#[derive(Clone)]
pub struct SimpleController {
    pub store: Arc<dyn persistence::SimpleStore>,
}
