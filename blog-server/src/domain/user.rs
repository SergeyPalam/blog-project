use chrono::{DateTime, Utc};
use std::fmt::Display;
use derive_more::Debug;

#[derive(Debug)]
pub struct User {
    id: i64,
    username: String,
    email: String,
    #[debug(skip)]
    password_hash: String,
    created_at: DateTime<Utc>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: name: {}, email: {}", self.username, self.email)
    }
}