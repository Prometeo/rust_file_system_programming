use std::time::{Duration, SystemTime};
use std::{fs, io};
use walkdir::WalkDir;

fn robust_directory_scan(target_path: &str) {
    println!("Scanning directory using walkdir...");
    // We instantiate WalkDir with our starting path.
    // The into_iter() method starts the recursive walk.
    for entry_result in WalkDir::new(target_path).into_iter() {
        // Walkdir returns a Result for every entry, just like fs::read_dir.
        // It catches permission errors and provides them here.
        match entry_result {
            Ok(entry) => {
                // The entry object is similar to fs::DirEntry.
                // We can easily get the path.
                let path = entry.path();
                println!("Scanned: {:?}", path);
            }
            Err(error) => {
                // If a specific folder denies access, walkdir catches it.
                // We can log the error, and the iterator will automatically
                // skip that folder and continue the walk safely.
                eprintln!("Encountered an error: {}", error);
            }
        }
    }
}

fn find_recent_large_text_files(scan_directory: &str) {
    // We calculate the exact time it was 24 hours ago.
    // SystemTime::now() gets the current computer clock time.
    // Duration::from_secs(86400) creates a duration of exactly one day.
    let one_day_ago = SystemTime::now() - Duration::from_secs(86400);
    // We start our robust walk.
    for entry_result in WalkDir::new(scan_directory).into_iter() {
        // We skip any entries that caused errors (like permission denied).
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        // Condition 1: Must be a file, not a directory.
        if !path.is_file() {
            continue;
        }
        // Condition 2: Must have a .txt extension.
        // The extension() method returns an Option, so we use 'if let' to extract it safely.
        let is_txt = if let Some(ext) = path.extension() {
            ext == "txt"
        } else {
            false
        };
        if !is_txt {
            continue;
        }
        // Condition 3: Check metadata for size and time.
        // We request the metadata for this specific file.
        if let Ok(metadata) = entry.metadata() {
            // We check if the size is greater than 100,000 bytes (100 KB).
            if metadata.len() > 100_000 {
                // We check the last modification time.
                if let Ok(modified_time) = metadata.modified() {
                    // We compare the file's modified time to our 24-hour cutoff.
                    // The >= operator works perfectly on SystemTime structs.
                    if modified_time >= one_day_ago {
                        // If all conditions are met, we print the file.
                        println!("Target found: {:?}", path);
                        println!("Size: {} bytes", metadata.len());
                    }
                }
            }
        }
    }
}


/// symlinks
fn inspect_links(path: &str) -> io::Result<()> {
    // We use symlink_metadata to look at the actual item at the path.
    let metadata = fs::symlink_metadata(path)?;
    // We can check if this specific item is a symlink.
    if metadata.file_type().is_symlink() {
        println!("This path is a symbolic link.");
        // If it is a symlink, we can read the text inside it to see
        // exactly where it is pointing.
        let target_path = fs::read_link(path)?;
        println!("It points to: {:?}", target_path);
    } else {
        println!("This is a normal file or directory.");
        println!("Size: {} bytes", metadata.len());
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
