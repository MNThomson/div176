#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
postgresql_embedded = { version = "0.18.5" }
tokio = { version = "1.41", features = ["full"] }
sqlx = { version = "0.8", features = ["chrono", "macros", "postgres", "runtime-tokio"] }
---
use std::fs;

use postgresql_embedded::PostgreSQL;
use sqlx::{Executor, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    println!("= Starting DB");

    let (postgresql, database_url) = {
        let mut postgresql = PostgreSQL::default();
        postgresql.setup().await.unwrap();
        postgresql.start().await.unwrap();

        let database_name = "sja";
        postgresql.create_database(database_name).await.unwrap();
        postgresql.database_exists(database_name).await.unwrap();
        let settings = postgresql.settings();
        let database_url = settings.url(database_name);
        (postgresql, database_url)
    };

    {
        println!("= Connecting to DB");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .unwrap();

        let sql = fs::read_to_string("./sql/init.sql").expect("Failed to read init.sql");
        for statement in sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                pool.execute(statement).await.unwrap();
            }
        }

        pool.close().await;
    }

    {
        println!("= Starting SQLX Prepare");
        let output = std::process::Command::new("cargo")
            .args(["sqlx", "prepare", "-D", &database_url])
            .output()
            .unwrap();

        if !output.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("{}", String::from_utf8_lossy(&output.stdout));
        }
    };

    postgresql.stop().await.unwrap();
}
