use std::error::Error;
// We import serde for our data struct.
use serde::Deserialize;
// We import the Figment builder and the Toml provider.
use figment::{
    Figment,
    providers::{Format, Toml},
};

// We define our application settings.
// We must derive Deserialize so figment can populate the fields.
#[derive(Debug, Deserialize)]
struct AppSettings {
    max_threads: u32,
    output_directory: String,
    enable_telemetry: bool,
}
fn load_basic_configuration() -> Result<(), Box<dyn Error>> {
    println!("Loading settings from configuration file...");
    // We instantiate the Figment builder.
    // We instruct it to merge the data from a file named 'Settings.toml'.
    let config_result: Result<AppSettings, figment::Error> =
        Figment::new().merge(Toml::file("Settings.toml")).extract();
    // We handle the result. If the file is missing or malformed, figment provides
    // a detailed error message explaining exactly which field caused the issue.
    match config_result {
        Ok(settings) => {
            println!("Settings loaded successfully.");
            println!("Threads: {}", settings.max_threads);
            println!("Output: {}", settings.output_directory);
        }
        Err(e) => {
            eprintln!("Failed to load configuration. Using system defaults.");
            eprintln!("Error details: {}", e);
            // In a real application, you might exit the program here or fall back
            // to a hardcoded default configuration.
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
