use std::fs;
use std::io;

fn safely_inspect_item(path: &str) -> io::Result<()> {
    // We use symlink_metadata to guarantee we are looking at the exact item
    // specified by the path, without any automatic redirection.
    let metadata_record = fs::symlink_metadata(path)?;
    // The file_type() method returns a FileType struct that allows us to
    // check for the symlink status explicitly.
    let file_type = metadata_record.file_type();
    if file_type.is_symlink() {
        println!("Attention: This item is a symbolic link.");
        println!("Link file size: {} bytes", metadata_record.len());
        // If we want to know where it points, we use fs::read_link.
        if let Ok(target) = fs::read_link(path) {
            println!("This link points to: {:?}", target);
        }
    } else {
        println!("This item is a normal file or directory.");
        println!("Actual size: {} bytes", metadata_record.len());
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
