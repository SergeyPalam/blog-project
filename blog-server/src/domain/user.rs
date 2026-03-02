use tracing::{error, info};

use chrono::{DateTime, Utc};
use derive_more::Debug;
use std::fmt::Display;

use super::error::AppError;
use crate::infrastructure::hash::{hash_password, verify_password};

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[debug(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn create(
        id: i64,
        username: String,
        email: String,
        password: String,
    ) -> Result<Self, AppError> {
        let password_hash = match hash_password(&password) {
            Ok(val) => val,
            Err(e) => {
                error!("{e}");
                return Err(AppError::InternalError("Can't hash password".to_string()));
            }
        };

        Ok(Self {
            id,
            username,
            email,
            password_hash,
            created_at: Utc::now(),
        })
    }

    pub fn verify_user(&self, password: &str) -> Result<(), AppError> {
        if let Err(e) = verify_password(password, &self.password_hash) {
            info!("Attempt to log with wrong credentials: {e} for user {}", self.username);
            return Err(AppError::Unauthorized(self.username.to_owned()));
        };

        Ok(())
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: name: {}, email: {}", self.username, self.email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::AppError;

    #[test]
    fn test_create() {
        let user = User::create(5, "name".to_string(), "mail".to_string(), "pass".to_string()).unwrap();
        assert_eq!(user.id, 5);
        assert_eq!(user.username, "name");
        assert_eq!(user.email, "mail");
        assert!(!user.password_hash.is_empty());
    }

    #[test]
    fn test_verify() {
        let user = User::create(5, "name".to_string(), "mail".to_string(), "pass".to_string()).unwrap();
        assert!(user.verify_user("pass").is_ok());
        let err = user.verify_user("wrong_pass").err().unwrap();
        match err {
            AppError::Unauthorized(msg) => {
                assert!(!msg.is_empty());
            }
            _ => {
                panic!("Wrong type error");
            }
        }
    }
}

