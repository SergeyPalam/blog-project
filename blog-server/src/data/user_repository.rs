use prost::Message;
use sqlx::PgPool;
use tracing::{info, error, warn, debug};

use crate::domain::user::User;
use crate::domain::error::AppError;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
    pub async fn add_new_user(&self, user: &User) -> Result<i64, AppError> {
        let query = sqlx::query!{
            r#"
             INSERT INTO users (username, email, password_hash, created_at)
             VALUES ($1, $2, $3, $4) RETURNING id
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.created_at
        };
        let id = match query.fetch_one(&self.pool).await {
            Ok(row) => row.id,
            Err(e) => {
                info!("{e}");
                let Some(e) = e.into_database_error() else{
                    return Err(AppError::InternalError(format!("DB error")));
                };

                let kind = e.kind();
                if let sqlx::error::ErrorKind::UniqueViolation = kind {
                    return Err(AppError::AlreadyExists(format!("{user}")));
                } else {
                    return Err(AppError::InternalError(format!("DB error")));
                }
            }
        };
        
        Ok(id)
    }

    pub async fn get_user(&self, username: &str) -> Result<User, AppError> {
        let query = sqlx::query_as!{
            User,
            r#"
             SELECT * from users where username = $1
            "#,
            username
        };
        
        let user =
        match query.fetch_one(&self.pool).await{
            Ok(row) => row,
            Err(e) => {
                info!("{e}");
                if let sqlx::error::Error::RowNotFound = e {
                    return Err(AppError::UserNotFound(username.to_string()));
                } else {
                    return Err(AppError::InternalError(format!("DB error")));
                }
            }
        };

        Ok(user)
    }
}