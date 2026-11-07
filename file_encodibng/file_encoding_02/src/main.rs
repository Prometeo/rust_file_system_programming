use std::borrow::Cow;
use std::fs;
use std::io;

fn read_text_resiliently(path: &str) -> io::Result<()> {
    let raw_bytes = fs::read(path)?;

    // We keep the Cow enum instead of immediately calling into_owned()
    let lossy_text = String::from_utf8_lossy(&raw_bytes);

    // We can instantly know if there were errors by checking the Cow variant.
    // This requires zero extra scanning of the text.
    match lossy_text {
        Cow::Borrowed(_) => {
            println!("File is perfect UTF-8.");
        }
        Cow::Owned(_) => {
            println!("Warning: The file contained invalid text formatting.");
            println!("Some characters were replaced with the unknown symbol.");
        }
    }

    // Now we can convert it to a standard String if we need ownership
    let recovered_text = lossy_text.into_owned();

    println!("Recovered Text Output:\n{}", recovered_text);

    Ok(())
}

fn main() {
    ()
}
