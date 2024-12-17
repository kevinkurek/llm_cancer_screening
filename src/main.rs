// src/main.rs
use dotenv::dotenv;
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let use_mock_server = env::var("USE_MOCK_SERVER").unwrap_or_else(|_| "false".to_string()) == "true";
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

    let prompt = "Once upon a time";

    match call_api(&api_url, &api_key, &prompt).await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}