// src/api.rs
use reqwest::Client;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use serde_json::Value;
use tokio::task::JoinHandle;
// use crate::storage::{DataStorage, WriteDataParams};
// use polars::prelude::*;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

pub async fn call_api(api_url: &str, api_key: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let request_body = RequestBody {
        model: "gpt-4".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens: 100, // Adjust as needed
    };

    let response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}

pub async fn create_futures(api_url: Arc<String>, api_key: Arc<String>, text_inputs: Vec<String>) -> Vec<JoinHandle<Option<String>>> {
    text_inputs.into_iter().map(|text_input| {
        let api_url = Arc::clone(&api_url);
        let api_key = Arc::clone(&api_key);
        let prompt = format!("Is there an indication of cancer in this text? Please answer with one word: True or False. {}", text_input);
        tokio::spawn(async move {
            match call_api(&api_url, &api_key, &prompt).await {
                Ok(response) => {
                    println!("Response: {}", response);
                    // Parse the response to extract the "content" value
                    let response_json: Value = serde_json::from_str(&response).expect("Failed to parse response");
                    let content = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();
                    Some(content)
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    None
                },
            }
        })
    }).collect()
}

// pub async fn create_futures<T: DataStorage + 'static>(
//     api_url: Arc<String>,
//     api_key: Arc<String>,
//     text_inputs: Vec<String>,
//     storage: Option<Arc<T>>,
// ) -> Vec<JoinHandle<Option<String>>> {
//     text_inputs
//         .into_iter()
//         .map(|text_input| {
//             let api_url = Arc::clone(&api_url);
//             let api_key = Arc::clone(&api_key);
//             let storage = storage.clone(); // Clone storage for async tasks
//             let prompt = format!(
//                 "Is there an indication of cancer in this text? Please answer with one word: True or False. {}",
//                 text_input
//             );

//             tokio::spawn(async move {
//                 match call_api(&api_url, &api_key, &prompt).await {
//                     Ok(response) => {
//                         println!("Response: {}", response);

//                         // Parse the response to extract the "content" value
//                         let response_json: Value =
//                             serde_json::from_str(&response).expect("Failed to parse response");
//                         let content = response_json["choices"][0]["message"]["content"]
//                             .as_str()
//                             .unwrap_or("")
//                             .to_string();

//                         // Save data using WriteDataParams if storage is provided
//                         if let Some(storage) = storage {
//                             let df = DataFrame::new(vec![
//                                 Series::new("input", vec![text_input.clone()]),
//                                 Series::new("output", vec![content.clone()]),
//                             ])
//                             .expect("Failed to create DataFrame");

//                             let params = WriteDataParams {
//                                 df,
//                                 column_name: "output".to_string(),
//                                 values: vec![Some(content.clone())],
//                                 output_path: "".to_string(), // Fill in path for CSV or leave blank for DB
//                             };

//                             if let Err(e) = storage.write_data(params).await {
//                                 eprintln!("Failed to save data: {}", e);
//                             }
//                         }

//                         Some(content)
//                     }
//                     Err(e) => {
//                         eprintln!("Error: {}", e);
//                         None
//                     }
//                 }
//             })
//         })
//         .collect()
// }