use super::config::DbConfig;
use sqlx::{PgPool, migrate, postgres::PgPoolOptions};

fn db_url_from_params(db_config: &DbConfig) -> String {
    format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_config.user, db_config.pass, db_config.host, db_config.port, db_config.name
    )
}

pub async fn create_pool(db_config: &DbConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .min_connections(db_config.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&db_url_from_params(&db_config))
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    migrate!("./migrations").run(pool).await?;
    Ok(())
}
