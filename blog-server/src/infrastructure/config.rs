use anyhow::Result;
use std::env;
pub struct DbConfig {
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: u16,
    pub pass: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

pub struct LogConfig {
    pub level: String,
}

pub struct SecretConfig {
    pub jwt_secret: String,
}

pub struct Config {
    pub db_config: DbConfig,
    pub log_config: LogConfig,
    pub secret_config: SecretConfig,
}

impl Config {
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            db_config: DbConfig {
                name: env::var("DB_NAME")?,
                user: env::var("DB_USER")?,
                host: env::var("DB_HOST")?,
                port: env::var("DB_PORT")?.parse::<u16>()?,
                pass: env::var("DB_PASS")?,
                max_connections: env::var("DB_MAX_CONN")?.parse::<u32>()?,
                min_connections: env::var("DB_MIN_CONN")?.parse::<u32>()?,
            },
            log_config: LogConfig {
                level: env::var("LOG_LEVEL")?,
            },
            secret_config: SecretConfig {
                jwt_secret: env::var("JWT_SECRET")?,
            },
        })
    }
}
