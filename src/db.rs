use async_trait::async_trait;
use polars::prelude::*;
use tokio_postgres::{Client, NoTls};
use crate::storage::{DataStorage, WriteDataParams};
use std::error::Error;

pub struct DbStorage {
    connection_string: String,
}

#[allow(dead_code)]
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

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[async_trait]
impl DataStorage for DbStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn std::error::Error>> {
        !todo!("Implement the read_data method for DbStorage")
    }

    async fn write_data(&self, params: WriteDataParams) -> Result<(), Box<dyn std::error::Error>> {
        !todo!("Implement the write_data method for DbStorage")
    }
}