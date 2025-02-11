use async_trait::async_trait;
use polars::prelude::*;
use std::error::Error;

#[async_trait]
pub trait DataStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>>;
    async fn write_data(&self, params: WriteDataParams) -> Result<(), Box<dyn Error>>;
}

pub struct WriteDataParams {
    pub df: DataFrame,
    pub column_name: String,
    pub values: Vec<Option<String>>,
    pub output_path: String,
}