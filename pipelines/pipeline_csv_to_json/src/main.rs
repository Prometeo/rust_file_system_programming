use serde::{Deserialize, Serialize};
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
// Now,let us build the pipeline function.
use std::error::Error;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{BufWriter, Write};
fn convert_csv_to_jsonl() -> Result<(), Box<dyn Error>> {
    // 1. Establish the Source.
    let input_file = File::open("user_database.csv")?;
    // We configure the CSV reader.
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input_file);
    // 2. Establish the Sink.
    let output_file = File::create("user_export.jsonl")?;
    let mut json_writer = BufWriter::new(output_file);
    println!("Beginning structural conversion pipeline...");
    // 3. Define the Transformers.
    // The csv_reader.deserialize() method returns an iterator of Result<CsvInputRecord>.
    let record_stream = csv_reader
        .deserialize::<CsvInputRecord>()
        // We filter out any malformed CSV rows.
        .filter_map(|result| result.ok())
        // The core Transformer: We map the input struct into the output struct.
        .map(|input_record| {
            // We implement our specific business logic transformation.
            // We assume the current year is 2024 for this simple calculation.
            let calculated_year = 2024 - input_record.age_in_years;
            // We construct and return the brand new output struct.
            // We move the string data from the old struct to the new one to avoid copying memory.
            JsonOutputRecord {
                user_handle: input_record.username,
                contact_email: input_record.email,
                estimated_birth_year: calculated_year,
            }
        });
    // 4. Consume the stream and feed the Sink.
    let mut success_count = 0;
    for final_record in record_stream {
        // We use serde_json to turn our Rust struct into a JSON string.
        // The to_string function returns a Result, which we handle safely.
        match serde_json::to_string(&final_record) {
            Ok(json_string) => {
                // We write the JSON string, followed by a newline, to our Sink.
                writeln!(json_writer, "{}", json_string)?;
                success_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to serialize a record to JSON: {}", e);
            }
        }
    }
    json_writer.flush()?;
    println!(
        "Conversion complete. {} records exported securely.",
        success_count
    );
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
