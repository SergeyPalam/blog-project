pub mod database;
pub mod jwt;
pub mod logging;
pub mod config;

use anyhow::Result;
use sqlx::PgPool;
use dotenv::dotenv;

use config::Config;
use database::{create_pool, run_migrations};
use logging::init_logging;

pub struct AppState {
    config: Config,
    db_pool: PgPool,    
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
    Ok(AppState{
        config,
        db_pool,
    })
}