// src/main.rs
use dotenv::dotenv;
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;
use llm_cancer_screening::csv_reader::{read_csv, extract_first_text_input};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // If you want to put in .env, for now we will use the default value
    // let use_mock_server = env::var("USE_MOCK_SERVER").unwrap_or_else(|_| "false".to_string()) == "true";
    let use_mock_server=false; 
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

    // Use an absolute path for testing
    let file_path = "tests/data/cancer_text_data.csv";
    let df = read_csv(file_path).expect("Failed to read CSV");
    if let Some(text_input) = extract_first_text_input(&df).expect("Failed to extract Text_Input column") {
        let prompt = format!("Is there an indication of cancer in this text? {}", text_input);

        match call_api(&api_url, &api_key, &prompt).await {
            Ok(response) => println!("Response: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        eprintln!("No text input found in the CSV file.");
    }
}