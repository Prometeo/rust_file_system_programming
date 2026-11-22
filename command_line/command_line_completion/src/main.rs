use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use std::io;

#[derive(Parser, Debug)]
#[command(name = "file_indexer")]
struct IndexerCli {
    #[command(subcommand)]
    action: IndexerAction,
}

#[derive(Subcommand, Debug)]
enum IndexerAction {
    Build {
        target_directory: String,
    },
    Search {
        query: String,
    },

    // We add a special subcommand solely for generating the script.
    // The shell argument will let the user choose to automatically convert
    GenerateCompletions {
        // We use clap's built-in value parsing to automatically convert
        // the user's string input into the correct Shell enum variant
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn handle_cli_execution() {
    let cli = IndexerCli::parse();
    match cli.action {
        IndexerAction::Build { target_directory } => {
            println!("Building index for {}", target_directory);
        }
        IndexerAction::Search { query } => {
            println!("Searching  index for '{}'", query);
        }
        IndexerAction::GenerateCompletions { shell } => {
            // Step 1: We extract raw internal Command object from our struct.
            // The command() method is provided by the CommandFactory trait.
            let mut application_command: Command = IndexerCli::command();

            // step 2: We define the name of the executable.
            let binary_name = "file_indexer";

            // Step 3: We call the generate function.
            // We pass the chosen shell, our command object, the binary name,
            // and a mutable reference to standard output.
            // The function will generate the script and print it directly to the terminal.
            generate(
                shell,
                &mut application_command,
                binary_name,
                &mut io::stdout(),
            );
            // We exit immediately after generating the script.
            println!("Completion script generated successfully");
        }
    }
}

fn main() {
    // To install the completion on linux type in the terminal:
    // file_indexer GenerateCompletions bash > ~/.local/share/bash-completions/completions/file_indexer
    println!("Hello, world!");
}
