// We import the glob function from the glob crate.
use glob::glob;
use std::path::PathBuf;

fn collect_all_data_files(search_root: &str) -> Vec<PathBuf> {
    // We construct our glob pattern string.
    // We combine the root directory with the recursive wildcard ** and the specific extension.
    let search_pattern = format!("{}/**/*.csv", search_root);
    println!("Searching file system using pattern: {}", search_pattern);
    let mut found_files = Vec::new();
    // The glob() function parses the pattern and begins searching the disk.
    // It returns an iterator of Results. The initial call can fail if the pattern
    // syntax is invalid, so we use expect() to handle developer errors.
    let matching_entries = glob(&search_pattern).expect("Failed to read the glob pattern");
    // We iterate through the results as they are found on the hard drive.
    for entry_result in matching_entries {
        match entry_result {
            Ok(path) => {
                // If a match is found and readable, we add it to our list.
                println!("Match found: {:?}", path);
                found_files.push(path);
            }
            Err(error) => {
                // The glob operation can fail on specific files if we lack permission
                // to read the directory they are inside. We log the error.
                eprintln!("Permission error while matching: {:?}", error.path());
            }
        }
    }
    println!("Total data files located: {}", found_files.len());
    found_files
}

fn main() {
    println!("Hello, world!");
}
