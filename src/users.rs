use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
    email: String,
}

pub struct EmailAlreadyExists;
pub struct NameAlreadyExists;

pub enum UserError {
    EmailAlreadyExists,
    NameAlreadyExists,
}

#[async_trait]
pub trait UserStore: Send + Sync + 'static {
    async fn create_user(
        &self,
        name: String,
        password: String,
        email: String,
    ) -> Result<(), UserError>;
}
