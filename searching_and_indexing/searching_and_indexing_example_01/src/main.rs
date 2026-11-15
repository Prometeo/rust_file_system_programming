use std::path::PathBuf;
use walkdir::WalkDir;

fn find_files_by_name(starting_directory: &str, search_term: &str) -> Vec<PathBuf> {
    // We create a Vector to hold the paths of the files we find.
    let mut matched_files: Vec<PathBuf> = Vec::new();
    // We want our search to be case-insensitive.
    // Finding "Report.txt" should work even if the user searches for "report".
    // We convert the search term to lowercase once, before the loop begins,
    // to save processing time.
    let search_term_lower = search_term.to_lowercase();
    println!(
        "Starting search for '{}' in {}",
        search_term, starting_directory
    );
    // We initialize the WalkDir iterator to scan the directory recursively.
    for entry_result in WalkDir::new(starting_directory).into_iter() {
        // We must handle potential permission errors gracefully.
        let entry = match entry_result {
            Ok(valid_entry) => valid_entry,
            Err(error) => {
                // If we cannot read a specific folder, we log a warning and continue.
                eprintln!(
                    "Warning: Could not read a directory entry. Error: {}",
                    error
                );
                continue;
            }
        };
        // We are only interested in files, not directories that might match the name.
        if !entry.file_type().is_file() {
            continue;
        }
        // We extract the file name.
        // File names are OS strings, so we must safely convert them to standard strings.
        if let Some(file_name_os) = entry.file_name().to_str() {
            // We convert the current file name to lowercase for comparison.
            let file_name_lower = file_name_os.to_lowercase();
            // We use the standard string 'contains' method to check for a match.
            if file_name_lower.contains(&search_term_lower) {
                // If it matches, we clone the path and add it to our results list.
                // We use into_path() to convert the DirEntry into a standalone PathBuf.
                matched_files.push(entry.into_path());
            }
        }
    }
    matched_files
}

fn execute_name_search() {
    // We call our search function, looking for any file containing "config".
    let results = find_files_by_name(".", "config");
    println!("Search complete. Found {} matching files.", results.len());
    for path in results {
        println!("- {:?}", path);
    }
}

fn main() {
    println!("Hello, world!");
}
