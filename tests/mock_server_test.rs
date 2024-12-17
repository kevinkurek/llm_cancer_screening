// tests/mock_server_test.rs
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;

#[tokio::test]
async fn test_call_api_with_mock_server() {
    // Start the mock server
    let server = start_mock_server();
    let api_url = &format!("{}/v1/engines/gpt-4/completions", server.base_url());
    let api_key = "test_api_key";
    let prompt = "Once upon a time";

    let response = call_api(&api_url, &api_key, &prompt).await.expect("API call failed");
    assert_eq!(response, "{\"choices\":[{\"text\":\"This is a mock response.\"}]}");
}