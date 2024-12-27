// use polars::prelude::CsvReadOptions;
// use polars_core::prelude::*;
// use polars::prelude::*;
use polars::prelude::*;
use std::fs::File;

pub fn read_csv(file_path: &str) -> PolarsResult<DataFrame> {
    let df = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(file_path.into()))
            .unwrap()
            .finish()
            .unwrap();
    println!("{:?}", df);
    Ok(df)
}

pub fn extract_text_inputs(df: &DataFrame) -> PolarsResult<Vec<String>> {

    // get the series of the column "Text_Input"
    let text_input_col = df.column("Text_Input")?;

    // Convert to a Vec<String>
    let text_inputs: Vec<String> = text_input_col
            .str() // Access the Series as a String type
            .expect("Column is not a String type") // Ensure the column is of type String
            .into_iter() // Get an iterator over the column values
            .map(|opt_name| opt_name.unwrap_or("").to_string()) // Handle Option<&str> and convert to String
            .collect(); // Collect the iterator into a Vec<String>
    // println!("{:?}", text_inputs);

    Ok(text_inputs)
}

pub fn add_column_and_write_csv(df: &mut DataFrame, column_name: &str, values: Vec<Option<String>>, output_path: &str) -> Result<(), PolarsError> {
    let series = Series::new(column_name.into(), values);
    df.with_column(series)?;
    let mut file = File::create(output_path)?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .finish(df)?;
    Ok(())
}