use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};
use std::error::Error;
// 1. We define our CLI struct. We make the fields optional so we know
// if the user actually typed them or not.
#[derive(Parser, Debug)]
#[command(name = "DataProcessor")]
struct CommandLineArgs {
    #[arg(short, long)]
    threads: Option<u32>,
    #[arg(short, long)]
    output: Option<String>,
}
// 2. We define our final Configuration struct.
// We derive both Serialize and Deserialize. Serialize is needed so we can
// treat this struct as our base layer of default values.
#[derive(Debug, Serialize, Deserialize)]
struct FinalConfiguration {
    max_threads: u32,
    output_directory: String,
}
// 3. We implement a method to generate the lowest-priority default values.
impl Default for FinalConfiguration {
    fn default() -> Self {
        FinalConfiguration {
            max_threads: 2,
            output_directory: String::from("./default_output"),
        }
    }
}
fn load_layered_configuration() -> Result<FinalConfiguration, Box<dyn Error>> {
    // We parse the immediate terminal arguments first.
    let cli_input = CommandLineArgs::parse();
    // We begin building the layered Figment configuration.
    let mut config_builder = Figment::new();
    // Layer 1: The absolute base defaults.
    // We use Serialized::defaults to load our hardcoded Default implementation.
    config_builder = config_builder.merge(Serialized::defaults(FinalConfiguration::default()));
    // Layer 2: The Configuration File.
    // If Settings.toml exists, any values inside it will overwrite the defaults.
    // If the file does not exist, figment quietly ignores it and keeps the defaults.
    config_builder = config_builder.merge(Toml::file("Settings.toml"));
    // Layer 3: Environment Variables.
    // We use the Env provider. We set a prefix of "PROCESSOR_".
    // If the operating system has an environment variable named PROCESSOR_MAX_THREADS=8,
    // figment will automatically grab it and overwrite the max_threads field.
    config_builder = config_builder.merge(Env::prefixed("PROCESSOR_"));
    // Layer 4: Command-Line Overrides.
    // We manually check our parsed clap struct. If the user provided a value,
    // we inject it into the figment builder using Serialized::defaults at the very end.
    // Because it is added last, it has the absolute highest priority.
    // We create a temporary struct to hold any CLI overrides.
    #[derive(Serialize)]
    struct CliOverrides {
        max_threads: Option<u32>,
        output_directory: Option<String>,
    }
    let overrides = CliOverrides {
        max_threads: cli_input.threads,
        output_directory: cli_input.output,
    };
    // We merge the overrides. Figment is smart enough to ignore the None values
    // and only apply the Some values.
    config_builder = config_builder.merge(Serialized::defaults(overrides));
    // Finally, we extract the fully merged configuration into our final struct.
    let finalized_settings: FinalConfiguration = config_builder.extract()?;
    Ok(finalized_settings)
}

fn main() {
    println!("Hello, world!");
}
