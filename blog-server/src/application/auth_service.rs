
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString};
use argon2::Argon2;
use chrono::Utc;

use std::sync::Arc;

use crate::infrastructure::jwt::JwtService;
use crate::data::user_repository::UserRepository;
use crate::domain::error::AppError;
use crate::domain::user::User;

#[derive(Deserialize)]
pub struct RegisterUserReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Default)]
pub struct RegisteredUser {
    pub token: String,
}

#[derive(Deserialize)]
pub struct LoginUserReq {
    pub username: String,
    pub password: String,
}

pub struct AuthService {
    jwt_service: Arc<JwtService>,
    user_repo: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(jwt_service: Arc<JwtService>, user_repo: Arc<UserRepository>) -> Self {
        Self {
            jwt_service,
            user_repo,
        }
    }
    pub async fn register(&self, reg_req: RegisterUserReq) -> Result<RegisteredUser, AppError>{
        let mut new_user = User::default();
        new_user.username = reg_req.username;
        new_user.email = reg_req.email;
        new_user.password_hash = Self::hash_password(&reg_req.password)?;
        new_user.created_at = Utc::now();

        let id = self.user_repo.add_new_user(&new_user).await?;
        let new_token = self.jwt_service.generate_token(&new_user.username, &new_user.email, id)?;
        
        Ok(RegisteredUser{token: new_token})
    }

    pub async fn login(&self, log_req: LoginUserReq) -> Result<RegisteredUser, AppError> {
        let user_name = log_req.username;
        let hash_password = Self::hash_password(&log_req.password)?;

        let user = self.user_repo.get_user(&user_name).await?;
        if user.password_hash != hash_password {
            info!("Attempt to log with wrong credentials: {user_name}");
            return Err(AppError::Unauthorized(user_name.to_string()));
        }

        let new_token = self.jwt_service.generate_token(&user.username, &user.email, user.id)?;
        Ok(RegisteredUser{token: new_token})
    }

    fn hash_password(password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(res) => res,
            Err(e) => {
                error!("{e}");
                return Err(AppError::InternalError(format!("Can't hash password")));
            }
        };
        Ok(password_hash.to_string())
    }
}