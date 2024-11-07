#![feature(associated_type_defaults)]
#![allow(async_fn_in_trait)]

use std::{fs::File, time::Duration};

use anyhow::{bail, Context, Result};
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool};

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

const PATH: &str = "data.db";

impl DB {
    pub async fn init() -> Result<Self> {
        File::open(PATH).or_else(|_| File::create(PATH)).unwrap();
        let db = DB {
            pool: SqlitePoolOptions::new()
                .max_connections(50)
                .acquire_timeout(Duration::from_secs(3))
                .connect(format!("sqlite://{}", PATH).as_str())
                .await
                .context("Could not connect to database (with URL)")?,
        };
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
