use sqlx::PgPool;
use tracing::info;
use crate::domain::post::Post;
use crate::domain::error::AppError;
pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
    pub async fn add_new_post(&self, post: Post) -> Result<String, AppError> {
        let query = sqlx::query!{
            r#"
             INSERT INTO posts (title, content, author_id, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)
            "#,
            post.title,
            post.content,
            post.author_id,
            post.created_at,
            post.updated_at
        };
        if let Err(e) = query.fetch_one(&self.pool).await {
            info!("{e}");
            return Err(AppError::InternalError(format!("DB error")));
        };
        
        Ok(post.content)
    } 
}