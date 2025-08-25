use sqlx::PgPool;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub(crate) fn init(pool: PgPool) -> Self {
        Self { pool }
    }
}
