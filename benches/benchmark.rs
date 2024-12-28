// allow unused imports for criterion
#![allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_cancer_screening::api::create_futures;
use llm_cancer_screening::mock_server::start_mock_server;
use llm_cancer_screening::csv_reader;
use std::sync::Arc;
use tokio::runtime::Runtime;

// create benchmark for read_csv
fn benchmark_read_csv(c: &mut Criterion) {
    c.bench_function("read_csv", |b| {
        b.iter(|| {
            let file_path = black_box("tests/data/cancer_text_data.csv");
            let _ = csv_reader::read_csv(file_path).expect("Failed to read CSV");
        });
    });
}

// fn benchmark_create_futures(c: &mut Criterion) {
//     // Setup a mock server, api_key, api_url, df, and text_inputs
//     let server = start_mock_server();
//     let api_url = Arc::new(format!("{}/v1/chat/completions", server.base_url()));
//     let api_key = Arc::new("test_api_key".to_string());
//     let df = read_csv("tests/data/cancer_10_records.csv").expect("Failed to read CSV");
//     let text_inputs = extract_text_inputs(&df).expect("Failed to extract Text_Input column");

//     // Create a Tokio runtime for running async code
//     let rt = Runtime::new().unwrap();
//     let handle = rt.handle().clone();

//     // create_futures benchmark
//     c.bench_function("create_futures", |b| {
//         b.to_async(handle).iter(|| async {
//             let futures = create_futures(Arc::clone(&api_url), Arc::clone(&api_key), text_inputs.clone());
//             let results: Vec<_> = futures.into_iter().map(|f| {
//                 let f = f;
//                 async move { black_box(f.await) }
//             }).collect::<Vec<_>>();
//             for result in results {
//                 let _ = black_box(result.await);
//             }
//         });
//     });
// }

criterion_group!(
    benches,
    benchmark_read_csv,
    // benchmark_create_futures
);
criterion_main!(benches);