use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
// We import the prelude to get access to the par_iter methods.
use rayon::prelude::*;
use walkdir::WalkDir;

fn parallel_content_search(root_directory: &str, target_word: &str) -> Vec<PathBuf> {
    println!("Scanning directory structure sequentially...");
    let mut all_file_paths: Vec<PathBuf> = Vec::new();
    // Step 1: Sequential Traversal.
    // We walk the directory tree on a single thread to collect the paths.
    // Parallelizing disk directory traversal is often counterproductive because
    // it causes the hard drive head to thrash wildly. Gathering paths sequentially
    // into memory first is usually the optimal strategy.
    for entry_result in WalkDir::new(root_directory).into_iter() {
        if let Ok(entry) = entry_result {
            if entry.file_type().is_file() {
                all_file_paths.push(entry.into_path());
            }
        }
    }
    println!(
        "Found {} files to inspect. Beginning parallel scanning...",
        all_file_paths.len()
    );
    let keyword_lower = target_word.to_lowercase();
    // Step 2: Parallel Content Scanning.
    // We transition from a standard iterator to a Rayon parallel iterator.
    // Rayon takes the massive Vector of paths and divides it among the CPU cores automatically.
    let matching_files: Vec<PathBuf> = all_file_paths
        .into_par_iter()
        // This filter closure is now running simultaneously on multiple cores.
        // Core 1 might be scanning file A, while Core 2 is scanning file B.
        .filter(|path| {
            // We isolate the disk read and memory logic cleanly inside the closure.
            // Each core opens its own assigned file independently.
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line_result in reader.lines() {
                    if let Ok(line) = line_result {
                        if line.to_lowercase().contains(&keyword_lower) {
                            // If we find the word, we return true to keep this path.
                            return true;
                        }
                    }
                }
            }
            // If the file fails to open, or the word is not found, we discard the path.
            false
        })
        // Step 3: Safe Collection.
        // Rayon safely merges the results from all the independent processor cores
        // back into a single, unified Vector.
        .collect();
    matching_files
}

fn main() {
    println!("Hello, world!");
}
