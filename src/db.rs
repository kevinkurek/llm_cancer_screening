use async_trait::async_trait;
use polars::prelude::*;
use tokio_postgres::{Client, NoTls};
use crate::storage::DataStorage;
use std::error::Error;

pub struct DbStorage {
    connection_string: String,
}

impl DbStorage {
    pub fn new(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
        }
    }

    async fn get_client(&self) -> Result<Client, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(client)
    }
}

#[async_trait]
impl DataStorage for DbStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>> {
        let client = self.get_client().await?;
        let rows = client.query("SELECT * FROM your_table", &[]).await?;
        // Convert rows to DataFrame
        // ...
        Ok(DataFrame::default()) // Replace with actual DataFrame conversion
    }

    async fn write_data(&self, df: &DataFrame) -> Result<(), Box<dyn Error>> {
        let client = self.get_client().await?;
        // Write DataFrame to database
        // ...
        Ok(())
    }
}