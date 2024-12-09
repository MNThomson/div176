#![feature(associated_type_defaults)]
#![allow(async_fn_in_trait)]

use std::{
    fs::{File, remove_file},
    time::Duration,
};

use anyhow::{Context, Result, bail};
use sqlx::{Sqlite, SqlitePool, sqlite::SqlitePoolOptions};

pub type DbType = Sqlite;
pub type DbPool = SqlitePool;

#[cfg_attr(test, unreachable_macro::with_unreachable_defaults)]
pub trait Database {
    async fn healthcheck(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct DB {
    pool: DbPool,
}

pub const INIT_SQL: &str = include_str!("../sql/init.sql");

const PATH: &str = "data.db";

impl DB {
    pub async fn init() -> Result<Self> {
        #[cfg(debug_assertions)]
        let _ = remove_file(PATH);

        File::open(PATH).or_else(|_| File::create(PATH)).unwrap();
        let db = DB {
            pool: SqlitePoolOptions::new()
                .max_connections(50)
                .acquire_timeout(Duration::from_secs(3))
                .connect(format!("sqlite://{}", PATH).as_str())
                .await
                .context("Could not connect to database (with URL)")?,
        };

        #[cfg(debug_assertions)]
        let _ = sqlx::query(INIT_SQL).execute(&db.pool).await;

        Ok(db)
    }
}

impl Database for DB {
    async fn healthcheck(&self) -> Result<()> {
        let row: (i64,) = sqlx::query_as("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .unwrap_or((0,));
        if row.0 == 1 {
            Ok(())
        } else {
            bail!("not healthy")
        }
    }
}
