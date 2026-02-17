pub mod database;
pub mod jwt;
pub mod logging;
pub mod config;
pub mod hash;

use anyhow::Result;
use dotenv::dotenv;

use std::sync::Arc;

use config::Config;
use database::{create_pool, run_migrations};
use logging::init_logging;
use jwt::JwtService;
use super::application::{auth_service::AuthService, blog_service::BlogService};
use super::data::{post_repository::PostRepository, user_repository::UserRepository};
pub struct AppState {
    pub config: Config,
    pub jwt_service: Arc<JwtService>,
    pub auth_service: Arc<AuthService>,
    pub blog_service: Arc<BlogService>,
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
    let jwt_service = Arc::new(JwtService::new(&config.secret_config));
    let post_repo = Arc::new(PostRepository::new(db_pool.clone()));
    let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
    let auth_service = Arc::new(AuthService::new(jwt_service.clone(), user_repo.clone()));
    let blog_service = Arc::new(BlogService::new(post_repo.clone()));
    Ok(AppState{
        config,
        jwt_service,
        auth_service,
        blog_service,
    })
}