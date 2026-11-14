use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
// We define a complex application state.
// We use the exact same Serialize and Deserialize traits we used for JSON.
#[derive(Debug, Serialize, Deserialize)]
struct ApplicationState {
    user_session_token: String,
    active_project_id: u32,
    recent_files: Vec<String>,
}
fn save_and_load_state() -> Result<(), Box<dyn Error>> {
    // We create our state object.
    let current_state = ApplicationState {
        user_session_token: String::from("TOKEN_XYZ_789"),
        active_project_id: 1045,
        recent_files: vec![String::from("report.pdf"), String::from("data.csv")],
    };
    // 1. Serializing to binary.
    // We ask bincode to serialize our struct into a raw Vector of bytes.
    let encoded_bytes: Vec<u8> = bincode::serialize(&current_state)?;
    // We write those raw bytes to a file.
    // Notice how small the resulting file will be compared to JSON.
    fs::write("app_state.bin", &encoded_bytes)?;
    println!("Application state securely saved to binary cache.");
    // 2. Deserializing from binary.
    // Later, when the application restarts, we read the raw bytes back into memory.
    let loaded_bytes = fs::read("app_state.bin")?;
    // We ask bincode to reconstruct the struct.
    let restored_state: ApplicationState = bincode::deserialize(&loaded_bytes)?;
    println!("Application state restored successfully:");
    println!("Active Project: {}", restored_state.active_project_id);
    println!("Recent Files Count: {}", restored_state.recent_files.len());
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
