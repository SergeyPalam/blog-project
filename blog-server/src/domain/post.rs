use chrono::{DateTime, Utc};
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub fn new(title: String, content: String, author_id: i64) -> Self {
        let current = Utc::now();
        Self {
            id: 0,
            title,
            content,
            author_id,
            created_at: current.clone(),
            updated_at: current
        }
    }
}