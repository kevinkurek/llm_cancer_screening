use llm_cancer_screening::storage::DataStorage;
use llm_cancer_screening::csv_storage::CsvStorage;

#[tokio::test]
async fn test_read_csv() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "tests/data/cancer_text_data.csv";
    let storage = CsvStorage::new(file_path);

    // Read the CSV file using the CsvStorage implementation
    let df = storage.read_data().await?;

    // Print the DataFrame to verify its contents
    println!("{:?}", df);

    // Assert the column names
    if file_path == "tests/data/sample.csv" {
        // Add assertions to verify the DataFrame's contents
        assert_eq!(df.shape(), (3, 2)); // Example: 3 rows, 2 columns
        assert_eq!(df.get_column_names(), ["name", "age"]);
    } else {
        // Add assertions to verify the DataFrame has N rows and 2 columns
        assert_eq!(df.width(), 2); // Example: N rows, 2 columns
        assert_eq!(df.get_column_names(), ["Index", "Text_Input"]);
    }

    Ok(())
}