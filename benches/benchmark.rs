// allow unused imports for criterion
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llm_cancer_screening::csv_reader;

// create benchmark for read_csv
fn benchmark_read_csv(c: &mut Criterion) {
    c.bench_function("read_csv", |b| {
        b.iter(|| {
            let file_path = black_box("tests/data/cancer_text_data.csv");
            let _ = csv_reader::read_csv(file_path).expect("Failed to read CSV");
        });
    });
}

criterion_group!(
    benches,
    benchmark_read_csv,
);
criterion_main!(benches);