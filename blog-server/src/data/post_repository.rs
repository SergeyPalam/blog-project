use sqlx::PgPool;
use tracing::info;

use crate::domain::error::AppError;
use crate::domain::post::Post;

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn next_post_id(&self) -> Result<i64, AppError> {
        let query = sqlx::query! {
            r#"
             SELECT NEXTVAL('posts_id_seq')
            "#
        };

        let next_post_id = match query.fetch_one(&self.pool).await {
            Ok(row) => {
                if let Some(val) = row.nextval {
                    val
                } else {
                    info!("Can't generate post id");
                    return Err(AppError::InternalError(format!("DB error")));
                }
            }
            Err(e) => {
                info!("{e}");
                return Err(AppError::InternalError(format!("DB error")));
            }
        };

        Ok(next_post_id)
    }

    pub async fn add_new_post(&self, post: &Post) -> Result<(), AppError> {
        let query = sqlx::query! {
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

        if let Err(e) = query.execute(&self.pool).await {
            info!("{e}");
            return Err(AppError::InternalError(format!("DB error")));
        };

        Ok(())
    }

    pub async fn get_post_author_id(&self, post_id: i64) -> Result<i64, AppError> {
        let query = sqlx::query! {
            r#"
             SELECT author_id FROM posts
             WHERE id = $1
            "#,
            post_id
        };

        let id = match query.fetch_one(&self.pool).await {
            Ok(row) => row.author_id,
            Err(e) => {
                info!("{e}");
                if let sqlx::error::Error::RowNotFound = e {
                    return Err(AppError::PostNotFound(post_id.to_string()));
                } else {
                    return Err(AppError::InternalError(format!("DB error")));
                }
            }
        };

        Ok(id)
    }

    pub async fn get_post(&self, post_id: i64) -> Result<Post, AppError> {
        let query = sqlx::query_as! {
            Post,
            r#"
             SELECT *
             FROM posts
             WHERE posts.id = $1
            "#,
            post_id
        };

        let post = match query.fetch_one(&self.pool).await {
            Ok(row) => row,
            Err(e) => {
                info!("{e}");
                if let sqlx::error::Error::RowNotFound = e {
                    return Err(AppError::PostNotFound(post_id.to_string()));
                } else {
                    return Err(AppError::InternalError(format!("DB error")));
                }
            }
        };

        Ok(post)
    }

    pub async fn update_post(
        &self,
        post_id: i64,
        new_title: Option<String>,
        new_content: Option<String>,
    ) -> Result<Post, AppError> {
        let mut post = self.get_post(post_id).await?;
        post.update(new_title, new_content);
        let query = sqlx::query! {
            r#"
             UPDATE posts
             SET title = $1, content = $2, updated_at = $3
             WHERE id = $4
            "#,
            post.title,
            post.content,
            post.updated_at,
            post_id
        };

        if let Err(e) = query.execute(&self.pool).await {
            info!("{e}");
            return Err(AppError::InternalError(format!("DB error")));
        };

        Ok(post)
    }

    pub async fn delete_post(&self, post_id: i64) -> Result<(), AppError> {
        let query = sqlx::query! {
            r#"
             DELETE FROM posts
             WHERE id = $1
            "#,
            post_id
        };

        if let Err(e) = query.execute(&self.pool).await {
            info!("{e}");
            if let sqlx::error::Error::RowNotFound = e {
                return Err(AppError::PostNotFound(post_id.to_string()));
            } else {
                return Err(AppError::InternalError(format!("DB error")));
            }
        };

        Ok(())
    }

    pub async fn get_posts(&self, offset: i64, limit: i64) -> Result<Vec<Post>, AppError> {
        let query = sqlx::query_as! {
            Post,
            r#"
             SELECT *
             FROM posts
             ORDER BY updated_at DESC
             LIMIT $1 OFFSET $2
            "#,
            offset,
            limit
        };

        let posts = match query.fetch_all(&self.pool).await {
            Ok(records) => records,
            Err(e) => {
                info!("{e}");
                return Err(AppError::InternalError(format!("DB error")));
            }
        };

        Ok(posts)
    }
}
