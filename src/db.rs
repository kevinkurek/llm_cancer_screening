use async_trait::async_trait;
use polars::prelude::*;
use crate::storage::{DataStorage, WriteDataParams};
use sqlx::{Pool, Postgres, Sqlite, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
use std::error::Error;

pub struct DbStorage {
    connection_string: String,
    pool: Option<Pool<Postgres>>,
    sqlite_pool: Option<Pool<Sqlite>>,
}

impl DbStorage {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            pool: None,
            sqlite_pool: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.connection_string.starts_with("postgres://") {
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&self.connection_string)
                .await?;
            self.pool = Some(pool);
        } else if self.connection_string.starts_with("sqlite://") {
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&self.connection_string)
                .await?;
            self.sqlite_pool = Some(pool);
        } else {
            return Err("Unsupported database type".into());
        }
        Ok(())
    }
}

#[async_trait]
impl DataStorage for DbStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>> {
        if let Some(pool) = &self.pool {
            // Implement PostgreSQL read logic
            let rows = sqlx::query!("SELECT * FROM your_table")
                .fetch_all(pool)
                .await?;
            // Convert rows to DataFrame
            // ...
            Ok(DataFrame::default()) // Replace with actual DataFrame conversion
        } else if let Some(pool) = &self.sqlite_pool {
            // Implement SQLite read logic
            let rows = sqlx::query!("SELECT * FROM your_table")
                .fetch_all(pool)
                .await?;
            // Convert rows to DataFrame
            // ...
            Ok(DataFrame::default()) // Replace with actual DataFrame conversion
        } else {
            Err("No database connection".into())
        }
    }

    async fn write_data(&self, params: WriteDataParams) -> Result<(), Box<dyn Error>> {
        if let Some(pool) = &self.pool {
            // Implement PostgreSQL write logic
            // ...
            Ok(())
        } else if let Some(pool) = &self.sqlite_pool {
            // Implement SQLite write logic
            // ...
            Ok(())
        } else {
            Err("No database connection".into())
        }
    }
}