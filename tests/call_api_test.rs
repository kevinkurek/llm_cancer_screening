// tests/call_api_test.rs
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;

#[tokio::test]
async fn test_call_api() {
    // Start the mock server
    let server = start_mock_server();

    // Mock server details
    let api_url = format!("{}/v1/chat/completions", server.base_url());
    let api_key = "test_api_key";
    let prompt = "Test prompt";

    // Call the API function
    let result = call_api(&api_url, api_key, prompt).await;

    // Assert that the result is Ok
    assert!(result.is_ok(), "API call failed: {:?}", result);

    // Parse the response
    let response_text = result.unwrap();
    let response_json: serde_json::Value = serde_json::from_str(&response_text)
        .expect("Failed to parse JSON response");
    
    // Verify the response content
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .expect("Response content missing");
    
    assert_eq!(content, "This is a mock response.");
}