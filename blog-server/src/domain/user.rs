use chrono::{DateTime, Utc};
use std::fmt::Display;
use derive_more::Debug;

#[derive(Debug, Default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[debug(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: name: {}, email: {}", self.username, self.email)
    }
}