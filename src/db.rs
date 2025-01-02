use async_trait::async_trait;
use polars::prelude::*;
use crate::storage::{DataStorage, WriteDataParams};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct DbStorage {
    connection_string: String,
    pool: Option<Pool<Sqlite>>,
}

impl DbStorage {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            pool: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        // Print a message to indicate that we are connecting to the database
        println!("Connecting to database: {}", self.connection_string);
        
        // Create a connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&self.connection_string)
            .await?;
        self.pool = Some(pool);
        Ok(())
    }

    pub async fn initialize(&self) -> Result<(), Box<dyn Error>> {
        if let Some(pool) = &self.pool {
            let sql_path = Path::new("./src/sql/initialize.sql");
            let sql = fs::read_to_string(sql_path)?;
            sqlx::query(&sql)
                .execute(pool)
                .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl DataStorage for DbStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>> {
        !todo!("Implement the read_data method for DbStorage");
    }

    async fn write_data(&self, params: WriteDataParams) -> Result<(), Box<dyn Error>> {
        if let Some(pool) = &self.pool {
            for i in 0..params.df.height() {
                let column1: &str = params.df.column("Text_Input")?.str()?.get(i).unwrap();
                let column2: &str = params.df.column("Cancer_Detected")?.str()?.get(i).unwrap();
                let sql_path = Path::new("./src/sql/write_data.sql");
                let sql = fs::read_to_string(sql_path)?;
                sqlx::query(&sql)
                    .bind(column1)
                    .bind(column2)
                .execute(pool)
                .await?;
            }
            Ok(())
        } else {
            Err("No database connection".into())
        }
    }
}