use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn save_data_atomically(target_path_str: &str, data: &str) -> io::Result<()> {
    let target_path = Path::new(target_path_str);
    // 1. Create a path for our temporary file.
    // We add an extension like .tmp so it is clear what this file is.
    // Crucially, this temporary file must be in the same directory as the target.
    // Moving files across different hard drives is not atomic.
    let temp_path = target_path.with_extension("tmp");
    // 2. Open the temporary file for writing.
    let temp_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(temp_file);
    // 3. Write our data to the temporary file.
    // We handle errors locally so we can clean up if something goes wrong.
    if let Err(write_error) = writer.write_all(data.as_bytes()) {
        // If writing fails (e.g., disk full), we drop the writer and delete the temp file.
        drop(writer);
        let _ = fs::remove_file(&temp_path);
        return Err(write_error);
    }
    // 4. Force the operating system to push the data to the physical disk platter.
    // We must call flush, and then we must call sync_all on the underlying file.
    // This guarantees the data is safe even if power is lost immediately after this line.
    writer.flush()?;
    let temp_file_handle = writer.into_inner().map_err(|e| e.into_error())?;
    temp_file_handle.sync_all()?;
    // 5. The Atomic Rename.
    // This single system call replaces the original target file with our complete temp file.
    // This is the atomic moment.
    fs::rename(&temp_path, target_path)?;

    println!("Data saved securely with atomic guarantees.");
    Ok(())
}

fn main() {
    println!("Hello, world!");
    // 1. The parent directory is never synced (Durability risk)
    // When you call fs::rename, you are modifying the parent directory's metadata (removing the old file pointer
    // and adding the new one). If a power failure occurs immediately after fs::rename, the directory changes might only exist in RAM
    // Upon reboot, the OS might roll back the directory state, and your new file will be lost. To guarantee the file survives a crash,
    // you must open the parent directory and call sync_all() on it after the rename.

    // 2. Deterministic temporary filenames (Concurrency risk)
    // Using target_path.with_extension("tmp") creates a race condition. If two threads or two instances of your
    // program try to save the same file at the exact same time, they will both write to file.tmp simultaneously,
    // interleaving their data and corrupting the file. Temporary files should use randomized names.

    // 3. BufWriter is redundant here (Performance)
    // You are writing a single string slice (data.as_bytes()). BufWriter is designed to batch many tiny writes into larger chunks.
    // For a single write_all call, BufWriter just adds unnecessary memory allocation and copying overhead.
}
