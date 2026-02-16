
use std::sync::Arc;

use actix_web::App;

use crate::infrastructure::jwt::JwtService;
use crate::data::user_repository::UserRepository;
use crate::domain::error::AppError;

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
    pub fn register(user_name: String, email: String, password: String) -> Result<String, AppError>{
        todo!();
    }

    pub fn login(user_name: String, password: String) -> Result<String, AppError> {
        todo!();
    }
}