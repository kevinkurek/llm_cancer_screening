use dotenv::dotenv;
use llm_cancer_screening::csv_reader::CsvStorage;
use llm_cancer_screening::db::DbStorage;
use llm_cancer_screening::storage::DataStorage;
use llm_cancer_screening::api::create_futures;
use llm_cancer_screening::mock_server::start_mock_server;
use std::env;
use std::sync::Arc;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Define file_path and output_path constants
    const FILE_PATH: &str = "tests/data/cancer_10_records.csv";
    const OUTPUT_PATH: &str = "output.csv";
    let use_mock_server = false; // Set to false to use the real API
    let use_database = true; // Set to true to use the database

    // Define the API URL and API key
    let (api_url, api_key) = if use_mock_server {
        // Start the mock server
        let server = start_mock_server();
        let api_url = format!("{}/v1/chat/completions", server.base_url());
        let api_key = "test_api_key".to_string();
        (api_url, api_key)
    } else {
        let api_url = "https://api.openai.com/v1/chat/completions".to_string();
        let api_key = env::var("OPENAI_API_KEY")?;
        (api_url, api_key)
    };

    // Select the appropriate storage backend
    let storage: Arc<dyn DataStorage + Send + Sync> = if use_database {
        Arc::new(DbStorage::new("your_database_connection_string"))
    } else {
        Arc::new(CsvStorage::new(FILE_PATH, OUTPUT_PATH))
    };

    // Read data
    let df = storage.read_data().await?;
    let text_inputs = extract_text_inputs(&df)?;

    // Create a vector of futures for the API calls
    let futures = create_futures(storage, text_inputs).await;

    // Wait for all futures to complete
    let results: Vec<Option<String>> = join_all(futures).await
                                        .into_iter()
                                        .map(|res| res.unwrap())
                                        .collect();

    // Write data
    storage.write_data(&df).await?;

    Ok(())
}