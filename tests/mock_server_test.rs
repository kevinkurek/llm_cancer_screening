// tests/mock_server_test.rs
use llm_cancer_screening::api::call_api;
use llm_cancer_screening::mock_server::start_mock_server;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[tokio::test]
async fn test_call_api_with_mock_server() {
    // Start the mock server
    let server = start_mock_server();
    let api_url = &format!("{}/v1/chat/completions", server.base_url());
    let api_key = "test_api_key";
    let prompt = "Once upon a time";

    let response = call_api(&api_url, &api_key, &prompt).await.expect("API call failed");

    // Parse the response
    let parsed_response: ApiResponse = serde_json::from_str(&response).expect("Failed to parse response");

    // Define the expected response
    let expected_response = ApiResponse {
        choices: vec![Choice {
            message: Message {
                role: "assistant".to_string(),
                content: "This is a mock response.".to_string(),
            },
        }],
    };

    // Compare the parsed response with the expected response
    assert_eq!(parsed_response, expected_response);
}