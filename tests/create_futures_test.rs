// tests/create_futures_test.rs
use std::sync::Arc;
use llm_cancer_screening::mock_server::start_mock_server;
use llm_cancer_screening::api::create_futures;
use tokio::task::JoinHandle;

#[tokio::test]
async fn test_create_futures() {
    // Start the mock server
    let server = start_mock_server();

    // Mock server details
    let api_url = Arc::new(format!("{}/v1/chat/completions", server.base_url()));
    let api_key = Arc::new("test_api_key".to_string());

    // Define test inputs
    let text_inputs = vec![
        "Patient shows no symptoms".to_string(),
        "MRI scan indicates a possible tumor".to_string(),
    ];

    // Expected responses
    let expected_responses = vec![
        Some("This is a mock response.".to_string()), // Response from the mock server
        Some("This is a mock response.".to_string()), // Same response for each input as per the mock
    ];

    // Call create_futures
    let futures: Vec<JoinHandle<Option<String>>> = create_futures(api_url, api_key, text_inputs).await;

    // Await all futures
    let results: Vec<Option<String>> = futures::future::join_all(futures)
        .await
        .into_iter()
        .map(|handle| handle.expect("Task panicked"))
        .collect();

    // Assert that the results match the expected responses
    assert_eq!(results, expected_responses);
}