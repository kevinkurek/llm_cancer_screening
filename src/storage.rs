use async_trait::async_trait;
use polars::prelude::*;
use std::error::Error;

#[async_trait]
pub trait DataStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>>;
    async fn write_data(&self, df: &DataFrame) -> Result<(), Box<dyn Error>>;
}