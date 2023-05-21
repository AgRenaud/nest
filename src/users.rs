use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    password: String,
    email: String
}

pub struct EmailAlreadyExists;
pub struct NameAlreadyExists;

pub enum UserError {
    EmailAlreadyExists,
    NameAlreadyExists
}

#[async_trait]
pub trait UserStore: Send + Sync + 'static {
    async fn add_user(&self, name: String, password: String, email: String) -> Result<(), UserError>;
}