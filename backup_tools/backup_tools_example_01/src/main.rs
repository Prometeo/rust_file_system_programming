use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

// We define a struct to represent a single file in our backup source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    // The relative path of the file from the root of our backup directory.
    pub path: PathBuf,
    // The exact size of the file in bytes.
    pub size: u64,
    // The modification time, stored as the number of seconds since the Unix Epoch
    // We use a u64 integer here instead of System Time because integers are
    // much easier to serialize and deserialize to JSON
    pub modified_seconds: u64,

    // We will leave room for a cryptographic hash, which we will implement shortly.
    pub content_hash: String,
}

// We define a struct to represent the entire snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    // A timestamp indicating when this backup scan was performed.
    pub scan_timestamp: u64,
    // A list pf all the files we discovered during the scan.
    pub files: Vec<FileRecord>,
}

// A helper function to get the current time as a simple integer
fn current_time_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn calculate_file_hash(file_path: &Path) -> io::Result<String> {
    // We open the file for reading
    let mut file = File::open(file_path)?;

    // We create a new, empty BLAKE3 hasher object.
    let mut hasher = Hasher::new();

    // We define a memory buffer on the stack.
    // A 64-kilobyte buffer is highly efficient for disk reading and hashing.
    let mut buffer = [0u8; 65536];
    loop {
        // We read a chunk of data from the hard drive into our memory buffer.
        let bytes_read = file.read(&mut buffer)?;

        // If we read zero bytes, we have reached the end of the file.
        if bytes_read == 0 {
            break;
        }

        // We isolate the actual data we just read.
        let valid_data = &buffer[..bytes_read];

        // We feed the data chunk into the hasher.
        // The hasher updates its internal mathematical state and waits for more data.
        hasher.update(valid_data);
    }

    // Once the entire file has been streamed through the hasher, we ask for the final result.
    let final_hash = hasher.finalize();

    // We convert the binary hash output into a standard hexadecimal string.
    // This allows us to easily save it to our JSON manifest later.
    let hash_string = final_hash.to_hex().to_string();
    Ok(hash_string)

    // We enter ourt standard stream processing loop.
}

// A helper function to get a file's modified time as integer
fn extract_modified_seconds(metadata: &fs::Metadata) -> u64 {
    metadata
        .modified()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn generate_manifest(source_directory: &str) -> BackupManifest {
    println!("Scanning source directory: {}", source_directory);

    let mut recorded_files = Vec::new();

    // We begin our recursive directory walk.
    for entry_result in WalkDir::new(source_directory) {
        // We gracefully skip any files or folders we do not have permission to read.
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: Skipping inaccessible path. Details: {}", err);
                continue;
            }
        };

        // We only want to record files, not directory folders.
        // The folders will be recreated automatically when restoring the files.
        if !entry.file_type().is_file() {
            continue;
        }

        // We attempt to read the metadata for the file.
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                eprintln!(
                    "Warning: Could not read metadata for {:?}. Error: {}",
                    entry.path(),
                    err
                );
                continue;
            }
        };

        let file_path = entry.path();

        // We attempt to calculate the hash.
        // If the hashing fails, we handle it gracefully by assigning a fallback string.
        let hash_value = match calculate_file_hash(file_path) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Warning: Could not hash file {:?}. Error: {}", file_path, e);
                String::from("UNHASHABLE_DUE_TO_ERROR")
            }
        };

        // We create a new FileRecord for this specific file.
        let record = FileRecord {
            // We store the full path.
            path: entry.path().to_path_buf(),
            size: metadata.len(),
            modified_seconds: extract_modified_seconds(&metadata),
            content_hash: hash_value,
        };

        // We add the record to our growing inventory list.
        recorded_files.push(record);
    }

    // We construct the final manifest object after completing the directory walk.
    let manifest = BackupManifest {
        scan_timestamp: current_time_seconds(),
        files: recorded_files,
    };

    println!("Scan complete. Indexed {} files.", manifest.files.len());

    manifest
}

fn main() {
    println!("Hello, world!");
}
