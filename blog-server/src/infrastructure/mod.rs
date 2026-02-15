pub mod database;
pub mod jwt;
pub mod logging;
pub mod config;

use anyhow::Result;
use sqlx::PgPool;
use dotenv::dotenv;

use std::sync::Arc;

use config::Config;
use database::{create_pool, run_migrations};
use logging::init_logging;
use jwt::JwtService;

pub struct AppState {
    pub config: Config,
    pub db_pool: PgPool,
    pub jwt_service: Arc<JwtService>,
}

pub async fn init() -> Result<AppState> {
    dotenv().ok();
    let config = Config::from_environment()?;
    init_logging(&config.log_config);
    let db_pool = create_pool(&config.db_config).await?;
    tracing::info!("Db pool connections has created");
    tracing::info!("Run migrations...");
    run_migrations(&db_pool).await?;
    tracing::info!("Migration finished");
    let jwt_service = JwtService::new(&config.secret_config);
    Ok(AppState{
        config,
        db_pool,
        jwt_service: Arc::new(jwt_service),
    })
}