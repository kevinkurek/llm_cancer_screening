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

pub fn extract_first_text_input(df: &DataFrame) -> Result<Option<String>> {
    let text_input_series = df.column("Text_Input")?;
    let first_text_input = text_input_series.utf8()?.get(0).map(|s| s.to_string());
    Ok(first_text_input)
}