use sqlx::PgPool;
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
}