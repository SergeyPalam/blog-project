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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_env(key: &str, val: &str) {
        unsafe {
            env::set_var(key, val);
        }
    }

    #[test]
    fn test_from_environment() {
        set_env("DB_NAME", "db");
        set_env("DB_USER", "user");
        set_env("DB_HOST", "localhost");
        set_env("DB_PORT", "5432");
        set_env("DB_PASS", "pass");
        set_env("DB_MAX_CONN", "20");
        set_env("DB_MIN_CONN", "5");
        set_env("LOG_LEVEL", "info");
        set_env("JWT_SECRET", "secret");

        let config = Config::from_environment().unwrap();
        assert_eq!(config.db_config.name, "db");
        assert_eq!(config.db_config.user, "user");
        assert_eq!(config.db_config.host, "localhost");
        assert_eq!(config.db_config.port, 5432);
        assert_eq!(config.db_config.pass, "pass");
        assert_eq!(config.db_config.max_connections, 20);
        assert_eq!(config.db_config.min_connections, 5);
        assert_eq!(config.log_config.level, "info");
        assert_eq!(config.secret_config.jwt_secret, "secret");
    }
}