use async_trait::async_trait;
use polars::prelude::*;
use crate::storage::{DataStorage, WriteDataParams};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions, Row};
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

        if let Some(pool) = &self.pool {
            let sql_path = Path::new("./src/sql/read_data.sql");
            let sql = fs::read_to_string(sql_path)?;
            let rows = sqlx::query(&sql)
                .fetch_all(pool)
                .await?;
            
            // Collect data into vectors
            let mut text_inputs = Vec::new();

            for row in rows {
                text_inputs.push(row.get::<String, _>("Text_Input"));
            }

            // Create DataFrame from vectors
            let df = DataFrame::new(vec![
                Series::new(PlSmallStr::from("Text_Input"), text_inputs).into(),
            ])?;

            println!("{:?}", df);

            Ok(df)
        } else {
            Err("No database connection".into())
        }
    }

    async fn write_data(&self, params: WriteDataParams) -> Result<(), Box<dyn Error>> {
        if let Some(pool) = &self.pool {
            // Add the Cancer_Detected column if it doesn't exist
            sqlx::query("ALTER TABLE responses ADD COLUMN Cancer_Detected TEXT;")
                .execute(pool)
                .await?;

            for i in 0..params.df.height() {
                let text_input: &str = params.df.column("Text_Input")?.str()?.get(i).unwrap();
                let cancer_detected: &str = params.values[i].as_deref().unwrap_or("");
                sqlx::query("UPDATE responses SET Cancer_Detected = ? WHERE Text_Input = ?;")
                    .bind(cancer_detected)
                    .bind(text_input)
                    .execute(pool)
                    .await?;
            }
            Ok(())
        } else {
            Err("No database connection".into())
        }
    }
}