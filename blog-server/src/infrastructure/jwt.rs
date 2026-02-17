use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, TimeDelta};
use tracing::{error};

use super::config::SecretConfig;
use crate::domain::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub email: String,
    pub id: i64,
    exp: usize,
}

pub struct JwtService {
    enc_key: EncodingKey,
    dec_key: DecodingKey,
    header: Header,
    validation: Validation,
}

impl JwtService {
    pub fn new(secret_config: &SecretConfig) -> Self {
        let header = Header::new(Algorithm::HS256);
        let validation = Validation::new(Algorithm::HS256);
        let enc_key = EncodingKey::from_secret(secret_config.jwt_secret.as_bytes());
        let dec_key = DecodingKey::from_secret(secret_config.jwt_secret.as_bytes());
        Self {
            enc_key,
            dec_key,
            header,
            validation
        }
    }

    pub fn generate_token(&self, username: &str, email: &str, user_id: i64) -> Result<String, AppError> {
        let expiration = if let Some(val) = Utc::now().checked_add_signed(TimeDelta::hours(24)){
            val.timestamp()
        }else{
            return Err(AppError::InternalError("Can't generate timestamp of jwt token".to_string()));
        };
        let claims = Claims {
            username: username.to_string(),
            email: email.to_string(),
            id: user_id,
            exp: expiration as usize,
        };
        let token = match encode(&self.header, &claims, &self.enc_key){
            Ok(val) => val,
            Err(e) => {
                error!("{e}");
                return Err(AppError::InternalError("Can't create jwt token".to_string()));
            }
        };
        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Option<Claims> {
        match decode::<Claims>(token, &self.dec_key, &self.validation) {
            Ok(data) => {
                Some(data.claims)
            }
            Err(_) => {
                None
            }
        }
    }
}