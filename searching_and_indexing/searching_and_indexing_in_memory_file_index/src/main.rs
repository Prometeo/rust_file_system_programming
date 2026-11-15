use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// We define our index structure.
// It holds a HashMap where the Key is a String (a word).
// The Value is a Vector of PathBufs (the files containing that word).
pub struct FileIndex {
    word_map: HashMap<String, Vec<PathBuf>>,
}

impl FileIndex {
    // We create a function to initialize a new, empty index.
    pub fn new() -> Self {
        FileIndex {
            word_map: HashMap::new(),
        }
    }
    // This method walks a directory and populates the index.
    pub fn build_from_directory(&mut self, root_path: &str) {
        println!("Building in-memory index for directory: {}", root_path);
        for entry_result in WalkDir::new(root_path).into_iter() {
            if let Ok(entry) = entry_result {
                let path = entry.path();
                // We only index standard files.
                if path.is_file() {
                    // We attempt to index the file. If it fails (e.g., it is a binary file
                    // that cannot be read as text), we quietly ignore it and continue.
                    let _ = self.index_single_file(path);
                }
            }
        }
        println!(
            "Index build complete. Tracking {} unique words.",
            self.word_map.len()
        );
    }
    // A private helper method to read the words from a single file.
    fn index_single_file(&mut self, file_path: &Path) -> std::io::Result<()> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let line = line_result?;
            // We convert the line to lowercase for case-insensitive searching.
            let line_lower = line.to_lowercase();
            // The split_whitespace method breaks the line into individual words.
            for word in line_lower.split_whitespace() {
                // We clean the word by removing common punctuation attached to it.
                // This ensures "error," and "error" both index as the same word.
                let clean_word = word
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string();
                if !clean_word.is_empty() {
                    // This is the core logic of the index.
                    // We check the HashMap for the current word.
                    // If the word does not exist in the map yet, we insert a new, empty Vector.
                    // Then, we push the current file path into that Vector.
                    self.word_map
                        .entry(clean_word)
                        .or_insert_with(Vec::new)
                        .push(file_path.to_path_buf());
                }
            }
        }
        Ok(())
    }
    // This method allows the user to query the index instantly.
    pub fn query(&self, search_term: &str) -> Option<&Vec<PathBuf>> {
        let term_lower = search_term.to_lowercase();
        // We retrieve the Vector of paths associated with the word, if it exists.
        self.word_map.get(&term_lower)
    }
}

fn main() {
    println!("Hello, world!");
}
