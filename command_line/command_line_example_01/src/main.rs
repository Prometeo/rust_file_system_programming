// We import the Parser trait from the clap crate.
use clap::Parser;
// We define a struct to represent our desired command-line inputs.
// The #[derive(Parser)] attribute tells the clap crate to automatically generate
// all the complex string parsing logic for us behind the scenes.

#[derive(Parser, Debug)]
// The #[command(...)] attributes allow us to add metadata to our tool.
// This metadata is used to generate the automatic help menu.
#[command(name = "renamer")]
#[command(version = "1.0")]
#[command(about = "A utility for renaming files in bulk.", long_about = None)]
struct RenamerCli {
    // This field represents the path to the file the user wants to rename.
    // By simply defining it as a String, clap knows to expect text from the user.
    source_path: String,
    // This field represents the new name for the file.
    target_path: String,
}
fn initialize_simple_cli() {
    // We call the parse method provided by the Parser trait.
    // This reads the raw terminal input, validates it, and returns our populated struct.
    // If the user provided the wrong arguments, parse() will automatically print
    // a highly detailed error message and terminate the program cleanly.
    let cli_arguments = RenamerCli::parse();
    println!("Ready to rename.");
    println!("Source: {}", cli_arguments.source_path);
    println!("Target: {}", cli_arguments.target_path);
}

fn main() {
    initialize_simple_cli();
}
