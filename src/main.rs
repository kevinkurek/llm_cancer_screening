// filepath: /Users/kevinkurek/Desktop/github/llm_cancer_screening/src/main.rs
use dotenv::dotenv;
use futures::future::join_all;
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;
use llm_cancer_screening::csv_reader::{read_csv, extract_text_inputs, add_column_and_write_csv};
use std::env;
use std::sync::Arc;
use serde_json::Value;

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

    // wrap api_url and api_key in Arc to share across threads
    let api_url = Arc::new(api_url);
    let api_key = Arc::new(api_key);

    // Use a relative path for the CSV file
    let mut df = read_csv(FILE_PATH).expect("Failed to read CSV");
    let text_inputs = extract_text_inputs(&df).expect("Failed to extract Text_Input column");
    println!("Text inputs: {:?}", text_inputs);

    // Create a vector of futures for the API calls
    let futures: Vec<_> = text_inputs.into_iter().map(|text_input| {
        let api_url = Arc::clone(&api_url);
        let api_key = Arc::clone(&api_key);
        let prompt = format!("Is there an indication of cancer in this text? Please answer with one word: True or False. {}", text_input);
        tokio::spawn(async move {
            match call_api(&api_url, &api_key, &prompt).await {
                Ok(response) => {
                    println!("Response: {}", response);
                    // Parse the response to extract the "content" value
                    let response_json: Value = serde_json::from_str(&response).expect("Failed to parse response");
                    let content = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
                    Some(content)
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    None
                },
            }
        })
    }).collect();

    // Wait for all futures to complete
    let results: Vec<Option<String>> = join_all(futures).await.into_iter().map(|res| res.unwrap()).collect();

    // Add the results to the new column and write to the output CSV
    add_column_and_write_csv(&mut df, "Cancer_Detected", results, OUTPUT_PATH).expect("Failed to write CSV");
}