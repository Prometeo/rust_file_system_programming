use serde::Deserialize;
use std::error::Error;
use std::fs;
// We define a nested structure to hold our configuration.
// The outer struct represents the entire file.
#[derive(Debug, Deserialize)]
struct BackupConfiguration {
    tool_name: String,
    maximum_retries: u32,
    // We can nest structs to represent TOML sections.
    paths: PathSettings,
}
// The inner struct represents a specific section in the TOML file.
#[derive(Debug, Deserialize)]
struct PathSettings {
    source_directory: String,
    destination_directory: String,
    ignore_hidden_files: bool,
}
fn load_tool_configuration() -> Result<(), Box<dyn Error>> {
    // 1. We define the path to our settings file.
    let config_path = "backup_settings.toml";
    // 2. We read the entire file into a standard string.
    // For small configuration files, fs::read_to_string is perfectly safe.
    let toml_content = fs::read_to_string(config_path)?;
    // 3. We ask the toml crate to deserialize the string into our Rust struct.
    // The type annotation : BackupConfiguration tells Serde exactly what shape to expect.
    let parsed_config: BackupConfiguration = toml::from_str(&toml_content)?;
    println!("Successfully loaded configuration:");
    println!("Tool Name: {}", parsed_config.tool_name);
    println!("Source: {}", parsed_config.paths.source_directory);
    // We can use the parsed booleans directly in our logic.
    if parsed_config.paths.ignore_hidden_files {
        println!("The tool will ignore hidden files during the backup process.");
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
