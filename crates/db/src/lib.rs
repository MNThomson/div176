#![allow(async_fn_in_trait)]

use std::time::Duration;

use postgresql_embedded::PostgreSQL;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod users;
use crate::users::Users;

#[cfg_attr(test, unreachable_macro::with_unreachable_defaults)]
pub trait Database {
    async fn healthcheck(&self) -> Result<(), ()>;
}

#[derive(Clone)]
pub struct DB {
    pool: PgPool,
    pub users: Users,
}
pub const INIT_SQL: &str = include_str!("../sql/init.sql");
pub const DUMP_SQL: &str = include_str!("../sql/dump.sql");

pub async fn embedded_db() -> (String, PostgreSQL) {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await.unwrap();
    postgresql.start().await.unwrap();

    let database_name = "sja";
    postgresql.create_database(database_name).await.unwrap();
    let settings = postgresql.settings();
    let database_url = settings.url(database_name);
    (database_url, postgresql)
}

impl DB {
    pub async fn init(connection_string: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(Duration::from_secs(3))
            .connect(connection_string)
            .await?;

        let db = DB {
            pool: pool.clone(),
            users: Users::init(pool.clone()),
        };

        #[cfg(debug_assertions)]
        {
            sqlx::raw_sql("DROP SCHEMA public CASCADE; CREATE SCHEMA public; GRANT ALL ON SCHEMA public TO sja; GRANT ALL ON SCHEMA public TO public;")
                .execute(&db.pool)
                .await
                .expect("to reset database");

            sqlx::raw_sql(INIT_SQL)
                .execute(&db.pool)
                .await
                .expect("to init db structure");

            sqlx::raw_sql(DUMP_SQL)
                .execute(&db.pool)
                .await
                .expect("to seed db with data");
        }

        Ok(db)
    }
}

impl Database for DB {
    async fn healthcheck(&self) -> Result<(), ()> {
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .unwrap_or((0,));
        if row.0 == 1 { Ok(()) } else { Err(()) }
    }
}
