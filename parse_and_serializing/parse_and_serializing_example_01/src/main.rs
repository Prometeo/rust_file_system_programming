use std::error::Error;
use std::fs::File;
// We import the Reader from the csv crate
use csv::Reader;
// We import the Deserialize trait, which we will explain deeply in the next section.
use serde::Deserialize;
// We define a struct that represents a single row in our CSV file.
// The #[derive(Deserialize)] line tells the compiler to automatically generate
// the code needed to map CSV columns into these specific fields.
#[derive(Debug, Deserialize)]
struct InventoryRecord {
    item_id: u32,
    item_name: String,
    quantity: u32,
}

fn process_inventory_file() -> Result<(), Box<dyn Error>> {
    // We open a standard file handle.
    let file = File::open("warehouse_inventory.csv")?;
    // We pass the file handle into the CSV Reader.
    // The Reader automatically wraps the file in a BufReader for high performance.
    let mut csv_reader = Reader::from_reader(file);
    println!("Starting inventory processing...");
    // The deserialize method creates an iterator that yields our custom structs.
    for result in csv_reader.deserialize() {
        // We must handle errors. A row might be missing a column,
        // or a quantity might be text instead of a number.
        match result {
            Ok(record) => {
                // We have a perfectly formed InventoryRecord struct.
                // We define the type explicitly here to help the compiler,
                // though Rust can usually infer it.
                let valid_record: InventoryRecord = record;
                println!(
                    "Found Item {}: {} (Quantity: {})",
                    valid_record.item_id, valid_record.item_name, valid_record.quantity
                );
            }
            Err(e) => {
                // If a single row is corrupted, we log the error and continue.
                // This is the batch aggregation pattern we learned in Chapter 3.
                eprintln!("Warning: Could not parse a row. Details: {}", e);
            }
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
