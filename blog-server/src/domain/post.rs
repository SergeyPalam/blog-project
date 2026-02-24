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
    pub fn create(id: i64, title: String, content: String, author_id: i64) -> Self {
        let current = Utc::now();
        Self {
            id,
            title,
            content,
            author_id,
            created_at: current,
            updated_at: current,
        }
    }

    pub fn update(&mut self, new_title: Option<String>, new_content: Option<String>) {
        if new_title.is_some() || new_content.is_some() {
            self.updated_at = Utc::now();
        }
        if let Some(title) = new_title {
            self.title = title;
        }
        if let Some(content) = new_content {
            self.content = content;
        }
    }
}
