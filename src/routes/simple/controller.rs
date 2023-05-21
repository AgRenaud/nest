use std::sync::Arc;

use crate::{simple_api};

#[derive(Clone)]
pub struct SimpleController {
    pub store: Arc<dyn simple_api::SimpleStore>,
}
