// filepath: /Users/kevinkurek/Desktop/github/llm_cancer_screening/src/main.rs
use dotenv::dotenv;
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;
use llm_cancer_screening::csv_reader::{read_csv, extract_text_inputs, add_column_and_write_csv};
use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let use_mock_server = env::var("USE_MOCK_SERVER").unwrap_or_else(|_| "false".to_string()) == "true";
    let use_mock_server = false; // Set to false to use the real API


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

    // Use a relative path for the CSV file
    let file_path = "tests/data/cancer_10_records.csv";
    let mut df = read_csv(file_path).expect("Failed to read CSV");
    let text_inputs = extract_text_inputs(&df).expect("Failed to extract Text_Input column");

    let mut results = Vec::new();
    for text_input in text_inputs {
        let prompt = format!("Is there an indication of cancer in this text? Please answer with one word: True or False. {}", text_input);

        match call_api(&api_url, &api_key, &prompt).await {
            Ok(response) => {
                println!("Response: {}", response);
                // Parse the response to extract the "content" value
                let response_json: Value = serde_json::from_str(&response).expect("Failed to parse response");
                let content = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
                results.push(Some(content));
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                results.push(None);
            },
        }
    }

    // Add the results to the new column and write to the output CSV
    add_column_and_write_csv(&mut df, "Cancer_Detected", results, "output_test.csv").expect("Failed to write CSV");
}