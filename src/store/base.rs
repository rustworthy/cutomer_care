#![allow(dead_code)]

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

#[derive(Debug, Clone)]
pub struct Db {
    pub connection: PgPool,
}

impl Db {
    pub async fn run_migrations(&self) {
        if let Err(e) = sqlx::migrate!().run(&self.clone().connection).await {
            panic!("Failed to run migrations: {}", e)
        }
    }

    async fn build(conn_string: &str) -> Self {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(conn_string)
            .await
        {
            Ok(connection) => Self { connection },
            Err(err) => panic!("Couldn't establish DB connection: {}", err),
        }
    }

    pub async fn new(usr: &str, pass: &str, host: &str, port: &str, db_name: &str) -> Self {
        let db_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            usr, pass, host, port, db_name
        );
        Self::build(&db_string).await
    }

    pub async fn from_env() -> Self {
        let db_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            &env::var("POSTGRES_USER").expect("POSTGRES_USER environment variable not set!"),
            &env::var("POSTGRES_PASSWORD")
                .expect("POSTGRES_PASSWORD environment variable not set!"),
            &env::var("POSTGRES_HOST").expect("POSTGRES_HOST environment variable not set!"),
            &env::var("POSTGRES_PORT").expect("POSTGRES_PORT environment variable not set!"),
            &env::var("POSTGRES_DB").expect("POSTGRES_DB environment variable not set!"),
        );
        Self::build(&db_string).await
    }
}
