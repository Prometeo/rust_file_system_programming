use std::fs;
use std::io;
use std::path::Path;


fn inspect_basic_metadata(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    // We ask the operating system for the metadata record of this path.
    // The ? operator ensures that if the file is missing or inaccessible,
    // the error is returned gracefully to the caller.
    let metadata_record = fs::metadata(&path)?;
    println!("Successfully retrieved metadata for: {:?}", path);
    // We can check if the item is a normal file.
    if metadata_record.is_file() {
        println!("Type: This is a standard file.");
    }
        // We can check if the item is a directory.
        if metadata_record.is_dir() {
            println!("Type: This is a directory.");
        }
        // We can retrieve the exact size in bytes.
        let file_size = metadata_record.len();
        println!("Size: {} bytes", file_size);
        // We can check the basic, cross-platform read-only permission flag.
        let permissions = metadata_record.permissions();
        if permissions.readonly() {
            println!("Status: This file is marked as read-only.");
        } else {
            println!("Status: This file can be modified.");
        }
        Ok(())
}

fn main() {
    println!("Hello, world!");
}
