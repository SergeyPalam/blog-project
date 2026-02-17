
use actix_web::HttpMessage;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use chrono::Utc;

use std::sync::Arc;

use crate::infrastructure::jwt::{JwtService};
use crate::infrastructure::hash::hash_password;
use crate::data::user_repository::UserRepository;
use crate::domain::error::AppError;
use crate::domain::user::User;

#[derive(Deserialize)]
pub struct RegisterUserReq {
    pub username: String,
    pub email: String,
    pub password: String,
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
    pub async fn register(&self, reg_req: RegisterUserReq) -> Result<String, AppError>{
        let mut new_user = User::default();
        new_user.username = reg_req.username;
        new_user.email = reg_req.email;
        new_user.password_hash = match hash_password(&reg_req.password){
            Ok(val) => val,
            Err(e) => {
                error!("{e}");
                return Err(AppError::InternalError("Can't hash password".to_string()));
            }
        };
        new_user.created_at = Utc::now();

        let id = self.user_repo.add_new_user(&new_user).await?;
        let new_token = self.jwt_service.generate_token(&new_user.username, &new_user.email, id)?;
        
        Ok(new_token)
    }

    pub async fn login(&self, log_req: LoginUserReq) -> Result<String, AppError> {
        let user_name = log_req.username;
        let hash_password = match hash_password(&log_req.password){
            Ok(val) => val,
            Err(e) => {
                error!("{e}");
                return Err(AppError::InternalError("Can't hash password".to_string()));
            }
        };

        let user = self.user_repo.get_user(&user_name).await?;
        if user.password_hash != hash_password {
            info!("Attempt to log with wrong credentials: {user_name}");
            return Err(AppError::Unauthorized(user_name.to_string()));
        }

        let new_token = self.jwt_service.generate_token(&user.username, &user.email, user.id)?;
        Ok(new_token)
    }
}