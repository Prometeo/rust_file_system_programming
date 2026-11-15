use serde::Serialize;
use std::path::PathBuf;

// We derive Serialize so this struct can be converted directly to JSON.
#[derive(Debug, Serialize)]
struct SearchOutput {
    total_files_scanned: usize,
    total_matches_found: usize,
    query_term: String,
    // We store the matched paths as a simple vector of strings.
    // PathBuf handles OS specifics, but strings are required for clean JSON export.
    results: Vec<String>,
}
fn present_search_results(
    query: &str,
    scanned_count: usize,
    matched_paths: Vec<PathBuf>,
    output_as_json: bool,
) {
    // 1. We prepare our data structure.
    let mut formatted_results = Vec::new();
    for path in matched_paths {
        // We convert the PathBuf to a String safely using lossy conversion.
        formatted_results.push(path.to_string_lossy().into_owned());
    }
    let output_data = SearchOutput {
        total_files_scanned: scanned_count,
        total_matches_found: formatted_results.len(),
        query_term: query.to_string(),
        results: formatted_results,
    };
    // 2. We route the data based on the requested output format.
    if output_as_json {
        // Machine-Readable Mode.
        // We use to_string_pretty to create nicely indented JSON, though standard
        // to_string works perfectly for pure machine-to-machine communication.
        match serde_json::to_string_pretty(&output_data) {
            Ok(json_string) => {
                // We print ONLY the JSON. No conversational text. No headers.
                // This allows the user to pipe the output directly into another tool like `jq`.
                println!("{}", json_string);
            }
            Err(e) => {
                // We use standard error for failures so we don't corrupt the standard output pipe.
                eprintln!("Error formatting JSON output: {}", e);
            }
        }
    } else {
        // Human-Readable Mode.
        // We print conversational text and structured lists.
        println!("========================================");
        println!("Search Complete for query: '{}'", output_data.query_term);
        println!("Files Scanned: {}", output_data.total_files_scanned);
        println!("Matches Found: {}", output_data.total_matches_found);
        println!("========================================");
        if output_data.results.is_empty() {
            println!("No results matched your criteria.");
        } else {
            println!("Matching Files:");
            for (index, file_path) in output_data.results.iter().enumerate() {
                // We print a numbered list.
                println!("  [{}] {}", index + 1, file_path);
            }
        }
        println!("========================================");
    }
}

fn main() {
    println!("Hello, world!");
}
