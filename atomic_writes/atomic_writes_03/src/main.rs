use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use tempfile::NamedTempFile;


/// Standard approach
fn save_data_atomically(target_path_str: &str, data: &str) -> io::Result<()> {
    let target_path = Path::new(target_path_str);
    let parent_dir = target_path.parent().unwrap_or_else(|| Path::new("."));

    // 1. Create a secure, randomized temporary file in the target directory.
    // If anything fails after this point, the temp file is automatically deleted when it drops.
    let mut temp_file = NamedTempFile::new_in(parent_dir)?;

    // 2. Write the data to the temporary file.
    temp_file.write_all(data.as_bytes())?;

    // 3. Force the data to the physical disk.
    temp_file.as_file().sync_all()?;

    // 4. Atomically move the temp file to the final target path.
    // persist() performs the atomic rename and consumes the temp file so it isn't deleted.
    temp_file.persist(target_path).map_err(|e| e.error)?;

    // 5. Sync the parent directory to guarantee the file metadata survives a sudden power loss.
    let dir = OpenOptions::new().read(true).open(parent_dir)?;
    dir.sync_all()?;

    Ok(())
}

fn main() {
    if let Err(e) = save_data_atomically("config.txt", "production data") {
        eprintln!("Save failed: {}", e);
    } else {
        println!("Save succeeded!");
    }
}
