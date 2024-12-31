use polars::prelude::*;
use std::fs::File;
use async_trait::async_trait;
use crate::storage::{DataStorage, WriteDataParams};
use std::error::Error;

pub struct CsvStorage {
    file_path: String,
    output_path: String,
}

impl CsvStorage {
    pub fn new(file_path: &str, output_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
        }
    }
}

#[async_trait]
impl DataStorage for CsvStorage {
    async fn read_data(&self) -> Result<DataFrame, Box<dyn Error>> {
        let df = CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some(self.file_path.clone().into()))?
            .finish()?;
        Ok(df)
    }

    async fn write_data(&self, mut params: WriteDataParams) -> Result<(), Box<dyn Error>> {
        let series = Series::new(params.column_name.into(), params.values);
        params.df.with_column(series)?;
        let file = File::create(&params.output_path)?;
        CsvWriter::new(file)
            .include_header(true)
            .finish(&mut params.df);
        Ok(())
    }
}

// pub fn read_csv(file_path: &str) -> PolarsResult<DataFrame> {
//     let df = CsvReadOptions::default()
//             .with_has_header(true)
//             .try_into_reader_with_file_path(Some(file_path.into()))?
//             .finish()?;
//     Ok(df)
// }

// pub fn extract_text_inputs(df: &DataFrame) -> PolarsResult<Vec<String>> {

//     // get the series of the column "Text_Input"
//     let text_input_col = df.column("Text_Input")?.str()?;

//     // Convert to a Vec<String>
//     let text_inputs: Vec<String> = text_input_col
//             .into_iter() // Get an iterator over the column values
//             .map(|val| val.unwrap_or("").to_string()) // Convert the Option<&str> to a String
//             .collect(); // Collect the iterator into a Vec<String>

//     Ok(text_inputs)
// }

// pub fn add_column_and_write_csv(df: &mut DataFrame, column_name: &str, values: Vec<Option<String>>, output_path: &str) -> Result<(), PolarsError> {
//     let series = Series::new(column_name.into(), values);
//     df.with_column(series)?;
//     let mut file = File::create(output_path)?;
//     CsvWriter::new(&mut file)
//         .include_header(true)
//         .finish(df)?;
//     Ok(())
// }