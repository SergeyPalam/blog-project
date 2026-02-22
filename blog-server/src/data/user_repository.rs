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

    pub async fn next_user_id(&self) -> Result<i64, AppError> {
        let query = sqlx::query!{
            r#"
             SELECT NEXTVAL('users_id_seq')
            "#
        };

        let next_user_id = match query.fetch_one(&self.pool).await{
            Ok(row) => {
                if let Some(val) = row.nextval{
                    val
                }else{
                    info!("Can't generate post id");
                    return Err(AppError::InternalError(format!("DB error")));
                }
            },
            Err(e) => {
                info!("{e}");
                return Err(AppError::InternalError(format!("DB error")));
            }
        };
        
        Ok(next_user_id)
    }

    pub async fn add_new_user(&self, user: &User) -> Result<(), AppError> {
        let query = sqlx::query!{
            r#"
             INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.created_at
        };

        if let Err(e) = query.execute(&self.pool).await {
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
        };
        
        Ok(())
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