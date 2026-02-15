
use std::sync::Arc;

use crate::infrastructure::jwt::JwtService;

pub struct AuthService {
    jwt_service: Arc<JwtService>,
}

impl AuthService {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self {
            jwt_service,
        }
    }
    pub fn register(user_name: String, email: String, password: String) -> String{
        todo!();
    }

    pub fn login(user_name: String, password: String) -> String {
        todo!();
    }
}