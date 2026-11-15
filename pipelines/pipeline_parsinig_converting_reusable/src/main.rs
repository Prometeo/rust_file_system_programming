use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;

// This struct represents the exact columns in the CSV file.
#[derive(Debug, Deserialize)]
struct CsvInputRecord {
    username: String,
    email: String,
    age_in_years: u32,
}
// This struct represents the final shape we want in our JSON file.
#[derive(Debug, Serialize)]
struct JsonOutputRecord {
    user_handle: String,
    contact_email: String,
    estimated_birth_year: u32,
}

// We define our pure transformation function.
// It takes the input struct and returns the output struct.
// This function knows absolutely nothing about files or pipelines.
fn transform_user_record(input: CsvInputRecord) -> JsonOutputRecord {
    let calculated_year = 2024 - input.age_in_years;
    JsonOutputRecord {
        user_handle: input.username,
        contact_email: input.email.to_lowercase(), // Let us add a new business rule here.
        estimated_birth_year: calculated_year,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Initialize `csv_reader` first (replace "users.csv" with your actual file path)
    let mut csv_reader = Reader::from_path("users.csv")?;

    // 2. Build the iterator pipeline
    let record_stream = csv_reader
        .deserialize::<CsvInputRecord>()
        .filter_map(|result| result.ok())
        .map(transform_user_record);

    // 3. Consume the iterator so the pipeline actually runs
    let output_records: Vec<JsonOutputRecord> = record_stream.collect();
    println!("{:#?}", output_records);

    Ok(())
}

// We write a quick test to prove our logic works perfectly.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record_transformation() {
        let raw_input = CsvInputRecord {
            username: String::from("TestUser"),
            email: String::from("USER@Example.com"),
            age_in_years: 30,
        };
        let result = transform_user_record(raw_input);
        // We verify the math is correct.
        assert_eq!(result.estimated_birth_year, 1994);
        // We verify our new lowercase business rule is applied.
        assert_eq!(result.contact_email, "user@example.com");
    }
}
