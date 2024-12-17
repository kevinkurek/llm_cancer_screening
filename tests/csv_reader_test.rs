// tests/csv_reader_test.rs
use llm_cancer_screening::csv_reader::read_csv;

#[test]
fn test_read_csv() {
    let file_path = "tests/data/cancer_text_data.csv";
    let df = read_csv(file_path).expect("Failed to read CSV");

    // Print the DataFrame to verify its contents
    println!("{:?}", df);



    // assert the column names
    if file_path == "tests/data/sample.csv" {
        // Add assertions to verify the DataFrame's contents
        assert_eq!(df.shape(), (3, 2)); // Example: 3 rows, 2 columns
        assert_eq!(df.get_column_names(), ["name", "age"]);
    }
    else {
        // Add assertions to verify the DataFrame has N rows and 2 columns
        assert_eq!(df.width(), 2); // Example: N rows, 2 columns
        assert_eq!(df.get_column_names(), ["Index", "Text_Input"]);
    }
}