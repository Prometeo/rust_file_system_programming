use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;

fn validate_user_input(input_path_str: &str) {
    let path = Path::new(input_path_str);
    // We perform a series of checks to understand the target.
    if path.exists() {
        println!("The path exists on the file system.");
        if path.is_file() {
            println!("Confirmed: The target is a normal file ready for processing.");
        } else if path.is_dir() {
            println!("Error: You provided a directory. This tool only processes individual files.");
        } else {
            println!("Warning: The target exists but is neither a standard file nor a directory.");
            println!("It might be a broken symbolic link or a special device file.");
        }
    } else {
        println!("Error: The specified path does not exist. Please check your spelling.");
    }
}

fn resolve_complex_path(user_input: &str) -> io::Result<()> {
    let messy_path = Path::new(user_input);
    println!("The user provided the path: {:?}", messy_path);
    // We call canonicalize. This requires disk access and verifies existence.
    // The ? operator ensures we return an error if the path is invalid.
    let absolute_path = fs::canonicalize(&messy_path)?;
    println!("The resolved, absolute path is: {:?}", absolute_path);
    Ok(())
}

fn locate_application_configuration() -> PathBuf {
    // We start with a base path.
    // In a real app, this might come from an environment variable.
    let mut config_path = PathBuf::from("application_root");
    // We push each component individually.
    // We NEVER use slashes in these strings.
    config_path.push("internal");
    config_path.push("settings");
    config_path.push("database_config.toml");
    // The resulting PathBuf is perfectly adapted to the target operating system.
    config_path
}

fn display_system_separator() {
println!("On this operating system, the path separator is: {}", MAIN_SEPARATOR);
 // You can also use the MAIN_SEPARATOR_STR constant if you need it as a string slice.
use std::path::MAIN_SEPARATOR_STR;
    println!("As a string slice, it looks like this: {}", MAIN_SEPARATOR_STR);
}

fn main() {
    println!("Hello, world!");
}
