use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
// We define a custom error type specifically for validation failures.
#[derive(Debug)]
struct ValidationError {
    details: String,
}
// We implement the standard Display trait so our error can be printed nicely.
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration Validation Failed: {}", self.details)
    }
}
// We implement the Error trait to make it a standard Rust error.
impl Error for ValidationError {}
#[derive(Debug, Deserialize)]
struct NetworkConfig {
    hostname: String,
    port: u32,
    max_connections: u32,
}

impl NetworkConfig {
    // We create a method specifically for checking internal logic rules.
    fn validate(&self) -> Result<(), ValidationError> {
        // Rule 1: The hostname cannot be completely empty.
        if self.hostname.trim().is_empty() {
            return Err(ValidationError {
                details: String::from("The hostname field cannot be empty."),
            });
        }
        // Rule 2: The port must be within the valid network range.
        if self.port == 0 || self.port > 65535 {
            return Err(ValidationError {
                details: format!(
                    "Port {} is invalid. Must be between 1 and 65535.",
                    self.port
                ),
            });
        }
        // Rule 3: We enforce a safe operational limit on connections.
        if self.max_connections > 1000 {
            return Err(ValidationError {
                details: format!(
                    "Max connections ({}) exceeds the safe limit of 1000.",
                    self.max_connections
                ),
            });
        }
        // If all checks pass, we return Ok.
        Ok(())
    }
}

fn load_and_validate_config() -> Result<NetworkConfig, Box<dyn Error>> {
    let file_content = fs::read_to_string("network_settings.json")?;
    // Step 1: Structural Parsing.
    // Serde ensures the JSON is properly formatted and contains the right fields.
    let config: NetworkConfig = serde_json::from_str(&file_content)?;
    // Step 2: Logical Validation.
    // We immediately call our validation method before letting the rest
    // of the application see the data.
    config.validate()?;
    // If we reach this line, the data is structurally and logically perfect.
    println!("Configuration loaded and validated successfully.");
    Ok(config)
}

fn main() {
    println!("Hello, world!");
}
