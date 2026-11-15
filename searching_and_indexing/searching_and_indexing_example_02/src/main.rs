use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
// We import the Regex type from the regex crate.
use regex::Regex;
fn scan_for_pattern(file_path: &Path, compiled_pattern: &Regex) -> io::Result<Vec<String>> {
    let mut extracted_matches: Vec<String> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    // We process the file line by line for memory efficiency.
    for line_result in reader.lines() {
        let line_text = line_result?;
        // The find_iter method searches the line for every instance of our pattern.
        // A single line might contain multiple email addresses, so we must iterate
        // over all the matches found on this specific line.
        for found_match in compiled_pattern.find_iter(&line_text) {
            // The found_match object knows exactly where the pattern starts and ends.
            // We use the as_str() method to extract the actual text of the email address,
            // convert it to an owned String, and push it to our results vector.
            extracted_matches.push(found_match.as_str().to_string());
        }
    }
    Ok(extracted_matches)
}
fn execute_regex_search() {
    let target_file = Path::new("customer_feedback_dump.txt");
    // Step 1: We define our regular expression pattern as a raw string.
    // The r"..." syntax in Rust is a raw string. It prevents Rust from trying to
    // interpret backslashes as escape characters, which is essential for writing
    // clean regex patterns.
    // This pattern broadly matches text formatted like an email address.
    let pattern_string = r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}";
    // Step 2: We compile the regular expression.
    // Compiling can fail if your syntax is invalid. We use expect() here because
    // this is a hardcoded string that we know is valid. If it were user input,
    // we would handle the Result gracefully.
    let email_regex = Regex::new(pattern_string).expect("Failed to compile regex");
    println!("Scanning file for email addresses...");
    // Step 3: We pass a reference to the compiled regex into our scanner function.
    match scan_for_pattern(target_file, &email_regex) {
        Ok(emails) => {
            println!("Scan complete. Found {} email addresses.", emails.len());
            for email in emails {
                println!("- {}", email);
            }
        }
        Err(e) => {
            eprintln!("Failed to read the target file: {}", e);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
