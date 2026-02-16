use sqlx::PgPool;
pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
}