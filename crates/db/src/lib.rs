use anyhow::Result;
use sqlx::{Sqlite, SqlitePool};

pub type DbType = Sqlite;
pub type DbPool = SqlitePool;

#[cfg_attr(test, unreachable_macro::with_unreachable_defaults)]
pub trait Database {
    fn healthcheck(&self) -> Result<()>;
}

#[derive(Clone)]
pub struct DB {}

impl Database for DB {
    fn healthcheck(&self) -> Result<()> {
        Ok(())
    }
}
