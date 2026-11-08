use std::io;
use std::path::Path;
// This entire block of code is only compiled when building for Unix systems.
#[cfg(unix)]
fn is_hidden(file_path: &Path) -> io::Result<bool> {
    // On Unix, we do not need to read the disk metadata.
    // We only need to look at the file name string.
    // We extract the file name from the path.
    let is_hidden = file_path
        .file_name()
        // We convert the OS-string to a standard string slice if possible.
        .and_then(|name| name.to_str())
        // We check if the string begins with a period.
        .map(|name_str| name_str.starts_with('.'))
        // If there is no valid file name, we default to false.
        .unwrap_or(false);
    // We return the boolean wrapped in Ok to match the signature.
    Ok(is_hidden)
}
// This entire block of code is only compiled when building for Windows systems.
#[cfg(windows)]
fn is_hidden(file_path: &Path) -> io::Result<bool> {
    use std::fs;
    use std::os::windows::fs::MetadataExt;
    // We define the constant for the hidden attribute.
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;
    // On Windows, we must ask the disk for the file's metadata.
    let metadata = fs::metadata(file_path)?;
    let attributes = metadata.file_attributes();
    // We use bitwise logic to check if the hidden flag is active.
    let is_hidden = (attributes & FILE_ATTRIBUTE_HIDDEN) != 0;
    Ok(is_hidden)
}
// Now we can use our function in cross-platform logic without worrying
// about which version the compiler actually selected.
fn calculate_visible_size(directory: &str) -> io::Result<()> {
    println!("Scanning directory: {}", directory);
    // In a real application, you would use walkdir here.
    // We assume file_path is obtained from an iterator.
    let file_path = Path::new("example_path.txt");
    // We call our conditionally compiled function.
    match is_hidden(&file_path) {
        Ok(true) => {
            println!("Skipping hidden file: {:?}", file_path);
            // We would continue to the next item in the loop.
        }
        Ok(false) => {
            println!("Processing visible file: {:?}", file_path);
            // We would calculate the size and add it to our total.
        }
        Err(e) => {
            println!("Could not determine hidden status: {}", e);
        }
    }
    Ok(())
}
fn main() {
    println!("Hello, world!");
}
