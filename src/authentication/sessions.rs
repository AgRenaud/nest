use serde::{Deserialize, Serialize};

pub const USER_SESSION_KEY: &str = "user_session";

#[derive(Serialize, Deserialize, Default)]
pub struct UserSession {
    pub user_id: uuid::Uuid,
}
