use dotenv::dotenv;
use llm_cancer_screening::csv_storage::{CsvStorage, extract_text_inputs};
use llm_cancer_screening::db::DbStorage;
use llm_cancer_screening::storage::{DataStorage, WriteDataParams};
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
    let use_mock_server = true; // Set to false to use the real API
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
        let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL env var");
        let mut db_storage = DbStorage::new(&database_url);
        db_storage.connect().await?;
        db_storage.initialize().await?;
        Arc::new(db_storage)
    } else {
        Arc::new(CsvStorage::new(FILE_PATH))
    };

    // Read data
    let df = storage.read_data().await?;
    let text_inputs = extract_text_inputs(&df)?;

    // // Create a vector of futures for the API calls
    let api_url = Arc::new(api_url);
    let api_key = Arc::new(api_key);
    let futures = create_futures(api_url, api_key, text_inputs).await;

    // Wait for all futures to complete
    let results: Vec<Option<String>> = join_all(futures)
                                        .await
                                        .into_iter()
                                        .map(|res| res.unwrap()).collect();

    // Write data
    let params = WriteDataParams {
        df,
        column_name: "Cancer_Detected".to_string(),
        values: results,
        output_path: OUTPUT_PATH.to_string(),
    };
    storage.write_data(params).await?;

    Ok(())
}