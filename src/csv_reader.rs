// filepath: /Users/kevinkurek/Desktop/github/llm_cancer_screening/src/csv_reader.rs
use polars::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn read_csv(file_path: &str) -> Result<DataFrame> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let df = CsvReader::new(reader)
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    Ok(df)
}

pub fn extract_text_inputs(df: &DataFrame) -> Result<Vec<String>> {
    let text_input_series = df.column("Text_Input")?;
    let text_inputs = text_input_series.utf8()?
        .into_iter()
        .filter_map(|opt| opt.map(|s| s.to_string()))
        .collect();
    Ok(text_inputs)
}

pub fn add_column_and_write_csv(df: &mut DataFrame, column_name: &str, values: Vec<Option<String>>, output_path: &str) -> Result<()> {
    let series = Series::new(column_name, values);
    df.with_column(series)?;
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);
    CsvWriter::new(writer)
        .has_header(true)
        .finish(df)?;
    Ok(())
}