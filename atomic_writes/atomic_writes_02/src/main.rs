use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// better approach
fn save_data_atomically(target_path_str: &str, data: &str) -> io::Result<()> {
    let target_path = Path::new(target_path_str);
    let parent_dir = target_path.parent().unwrap_or_else(|| Path::new("."));

    // 1. Generate a pseudo-random temporary filename to prevent collisions
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let temp_name = format!("{}.{}.tmp", target_path.file_name().unwrap().to_string_lossy(), timestamp);
    let temp_path = parent_dir.join(temp_name);

    // 2. Open file. (BufWriter removed as it's unneeded for a single bulk write)
    let mut temp_file = File::create(&temp_path)?;

    // 3. Write data. Clean up manually if it fails.
    if let Err(write_error) = temp_file.write_all(data.as_bytes()) {
        let _ = fs::remove_file(&temp_path);
        return Err(write_error);
    }

    // 4. Flush file data to physical disk
    temp_file.sync_all()?;

    // 5. Atomic Rename
    if let Err(rename_error) = fs::rename(&temp_path, target_path) {
        let _ = fs::remove_file(&temp_path); // Cleanup on rename failure
        return Err(rename_error);
    }

    // 6. NEW: Sync the parent directory to persist the rename operation
    // Note: Opening a directory requires read permissions
    let dir = OpenOptions::new().read(true).open(parent_dir)?;
    dir.sync_all()?;

    println!("Data saved securely with atomic and durable guarantees.");
    Ok(())
}

fn main() {
    if let Err(e) = save_data_atomically("config.txt", "my sensitive data") {
        eprintln!("Failed to save: {}", e);
    }
}
