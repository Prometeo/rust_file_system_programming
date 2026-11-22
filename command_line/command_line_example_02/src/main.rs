use clap::{Parser, Subcommand};
// We define our top-level application struct.
#[derive(Parser, Debug)]
#[command(name = "filetoolkit")]
#[command(about = "A comprehensive file manipulation tool.")]
struct ToolkitCli {
    // This is a Flag. We use the #[arg(short, long)] attribute.
    // 'short' allows the user to type -v.
    // 'long' allows the user to type --verbose.
    // Because the type is bool, clap knows this is a toggle switch.
    #[arg(short, long)]
    verbose: bool,
    // This is an Option. It has a short (-c) and long (--config) format.
    // Because the type is Option<String>, it is not strictly required.
    // If the user does not provide it, the value will be None.
    #[arg(short, long)]
    config_file: Option<String>,
    // This field holds our subcommands.
    // We use the #[command(subcommand)] attribute to tell clap to look at our enum.
    #[command(subcommand)]
    action: ToolkitCommand,
}
// We define an enum to represent the different subcommands.
#[derive(Subcommand, Debug)]
enum ToolkitCommand {
    // The first subcommand mode is 'search'.
    Search {
        // This is a Positional Argument. It has no short or long attributes.
        // It is strictly required.
        keyword: String,
        // This is another Positional Argument.
        // We use a default value, which automatically makes it an optional argument
        // even though it is positional. If the user omits it, it defaults to the current directory.
        #[arg(default_value = ".")]
        directory: String,
    },
    // The second subcommand mode is 'compress'.
    Compress {
        // This is a Positional Argument for the file to compress.
        target_file: String,
        // This is a Flag specific only to the compress subcommand.
        #[arg(long)]
        fast_mode: bool,
    },
}
fn execute_complex_cli() {
    let cli = ToolkitCli::parse();
    // We check the global verbose flag.
    if cli.verbose {
        println!("Verbose mode is enabled. Logging all operations.");
    }
    // We use pattern matching to determine which subcommand the user chose.
    match &cli.action {
        ToolkitCommand::Search { keyword, directory } => {
            println!("Executing Search Mode.");
            println!(
                "Looking for '{}' inside the '{}' directory.",
                keyword, directory
            );
            // Call the search logic from Chapter 11 here.
        }
        ToolkitCommand::Compress {
            target_file,
            fast_mode,
        } => {
            println!("Executing Compression Mode.");
            println!("Target file: {}", target_file);
            if *fast_mode {
                println!("Fast compression algorithm selected.");
            }
            // Call the compression logic here.
        }
    }
}

fn main() {
    execute_complex_cli();
}
