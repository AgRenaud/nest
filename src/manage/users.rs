use async_trait::async_trait;

pub enum Role {
    Admin,
    Contributor,
}

pub struct User {
    pub name: String,
    password: String,
    email: String,
    pub role: Role,
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
        role: Role,
    ) -> Result<(), UserError>;
}
