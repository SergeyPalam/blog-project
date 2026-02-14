use chrono::{DateTime, Utc};

pub struct User {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}