// src/csv_reader.rs
use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn read_csv(file_path: &str) -> Result<DataFrame> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let df = CsvReader::new(reader)
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    Ok(df)
}