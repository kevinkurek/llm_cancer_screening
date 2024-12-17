// src/mock_server.rs
use httpmock::MockServer;
use httpmock::Method::POST;
use serde_json::json;

pub fn start_mock_server() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
        when.method(POST)
            .path("/v1/chat/completions")
            .header("Authorization", "Bearer test_api_key");
        then.status(200)
            .json_body(json!({
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "This is a mock response."
                    }
                }]
            }));
    });

    server
}