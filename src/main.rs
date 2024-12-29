use dotenv::dotenv;
use llm_cancer_screening::csv_reader::{read_csv, extract_text_inputs, add_column_and_write_csv};
use llm_cancer_screening::api::create_futures;
use llm_cancer_screening::mock_server::start_mock_server;
use std::env;
use std::sync::Arc;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // define file_path constant
    const FILE_PATH: &str = "tests/data/cancer_10_records.csv";
    const OUTPUT_PATH: &str = "output.csv";
    let use_mock_server = false; // Set to false to use the real API

    // Define the API URL and API key
    let (api_url, api_key) = if use_mock_server {
        // Start the mock server
        let server = start_mock_server();
        let api_url = format!("{}/v1/chat/completions", server.base_url());
        let api_key = "test_api_key".to_string();
        (api_url, api_key)
    } else {
        let api_url = "https://api.openai.com/v1/chat/completions".to_string();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        (api_url, api_key)
    };

    // Handle reading the CSV file
    let mut df = match read_csv(FILE_PATH) {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Error reading CSV into dataframe: {}", e);
            return;
        }
    };
    println!("First 5 dataframe values: {}", df.head(Some(5)));

    // Extract the text inputs from the dataframe
    let text_inputs = match extract_text_inputs(&df) {
        Ok(text_inputs) => text_inputs,
        Err(e) => {
            eprintln!("Error extracting text inputs from dataframe: {}", e);
            return;
        }
    };
    println!("First 5 Example Text inputs: {:?}", &text_inputs[..5]);

    // Create a vector of futures for the API calls: wrap api_url and api_key in Arc to share across threads
    let api_url = Arc::new(api_url);
    let api_key = Arc::new(api_key);
    let futures = create_futures(api_url, api_key, text_inputs);

    // Wait for all futures to complete
    let results: Vec<Option<String>> = join_all(futures).await.into_iter().map(|res| res.unwrap()).collect();

    // Add the results to the new column and write to the output CSV
    if let Err(e) = add_column_and_write_csv(&mut df, "Cancer_Detected", results, OUTPUT_PATH){
        eprintln!("Error adding column and writing to CSV: {}", e);
        return;
    };
}