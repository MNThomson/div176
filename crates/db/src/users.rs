use sqlx::PgPool;
use types::Error;

#[derive(Clone)]
pub struct Users {
    pool: PgPool,
}

impl Users {
    pub(crate) fn init(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_userid_password_from_username(
        &self,
        username: &str,
    ) -> Result<Option<(i32, String)>, Error> {
        let result = sqlx::query!(
            "SELECT uid, password_hash FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|f| (f.uid, f.password_hash));

        Ok(result)
    }

    pub async fn create_session(&self, user_id: i32, token: &str) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user_sessions (uid, token) VALUES ($1, $2)",
            user_id,
            token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
