use std::fs;
use std::io;
fn read_text_resiliently(path: &str) -> io::Result<()> {
    // Instead of using read_to_string, we read the file as raw, unopinionated bytes.
    let raw_bytes = fs::read(path)?;
    // We pass the raw bytes to the lossy conversion function.
    // This function returns a Cow (Clone-on-Write) smart pointer.
    // For beginners, you can simply call .into_owned() to turn it into a standard String.
    let recovered_text = String::from_utf8_lossy(&raw_bytes).into_owned();
    // We can now print or process the text.
    println!("Recovered Text Output:");
    println!("{}", recovered_text);
    // We can also check if any data was lost during the conversion.
    // If the resulting string contains the special replacement character,
    // we know the file was not perfect UTF-8.
    if recovered_text.contains('\u{FFFD}') {
        println!("Warning: The file contained invalid text formatting.");
        println!("Some characters were replaced with the unknown symbol.");
    }
    Ok(())
}

fn main() {
    // .into_owned() and then use .contains('\u{FFFD}') to check if there were errors. This is highly inefficient.
    // Calling .contains() forces the program to scan the entire string a second time from start to finish.
    // When you call from_utf8_lossy, Rust scans the bytes. If the bytes are 100% perfect UTF-8, it returns a Cow::Borrowed pointer,
    // doing zero allocations. If it finds an error, it allocates memory to insert the replacement characters and returns a Cow::Owned
    // You can check for errors instantly, with zero extra scanning, simply by looking at the Cow type itself.
    println!("Hello, world!");
}
