
use actix_web::HttpMessage;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use chrono::Utc;

use std::sync::Arc;

use crate::infrastructure::jwt::{JwtService};
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

#[derive(Serialize)]
pub struct RegisteredUser {
    pub token: String,
}

pub struct AuthService {
    jwt_service: Arc<JwtService>,
    user_repo: Arc<UserRepository>,
}

impl AuthService {
    fn build_reg_user(&self, user: User) -> Result<RegisteredUser, AppError> {
        let new_token = self.jwt_service.generate_token(&user.username, &user.email, user.id)?;
        Ok(RegisteredUser{
            token: new_token,
        })
    }
    pub fn new(jwt_service: Arc<JwtService>, user_repo: Arc<UserRepository>) -> Self {
        Self {
            jwt_service,
            user_repo,
        }
    }
    pub async fn register(&self, reg_req: RegisterUserReq) -> Result<RegisteredUser, AppError>{
        let user_id = self.user_repo.next_user_id().await?;
        let new_user = User::create(user_id, reg_req.username, reg_req.email, reg_req.password)?;

        self.user_repo.add_new_user(&new_user).await?;
        self.build_reg_user(new_user)
    }

    pub async fn login(&self, log_req: LoginUserReq) -> Result<RegisteredUser, AppError> {
        let user_name = log_req.username;
        let user = self.user_repo.get_user(&user_name).await?;
        user.verify_user(&log_req.password)?;
        self.build_reg_user(user)
    }
}