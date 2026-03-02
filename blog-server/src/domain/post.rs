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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let post = Post::create(5, "new_title".to_string(), "new_content".to_string(), 6);
        assert_eq!(post.id, 5);
        assert_eq!(post.title, "new_title");
        assert_eq!(post.content, "new_content");
        assert_eq!(post.author_id, 6);
        assert_eq!(post.created_at, post.updated_at);
    }

    #[test]
    fn test_update() {
        let mut post = Post::create(5, "new_title".to_string(), "new_content".to_string(), 6);
        post.update(None, None);

        assert_eq!(post.id, 5);
        assert_eq!(post.title, "new_title");
        assert_eq!(post.content, "new_content");
        assert_eq!(post.author_id, 6);
        assert_eq!(post.created_at, post.updated_at);

        post.update(Some("updated_title".to_string()), None);
        assert_eq!(post.id, 5);
        assert_eq!(post.title, "updated_title");
        assert_eq!(post.content, "new_content");
        assert_eq!(post.author_id, 6);
        assert!(post.updated_at > post.created_at);

        let prev_updated = post.updated_at;
        post.update(None, Some("updated_content".to_string()));
        assert_eq!(post.id, 5);
        assert_eq!(post.title, "updated_title");
        assert_eq!(post.content, "updated_content");
        assert_eq!(post.author_id, 6);
        assert!(post.updated_at > prev_updated);
    }
}
