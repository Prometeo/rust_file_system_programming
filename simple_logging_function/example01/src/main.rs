use std::fs::OpenOptions;
use std::io::{self, Write};

fn append_to_system_log(message: &str) -> io::Result<()> {
    let log_path = "system_activity.log";
    // We instantiate the OpenOptions builder.
    let mut file = OpenOptions::new()
        // We want to create the file if it is missing.
        .create(true)
        // We want to append data to the end, preserving history.
        .append(true)
        // Finally, we call open with our target path.
        .open(log_path)?;
    // Now we use the write_all method from the std::io::Write trait.
    // We convert our string message into a slice of raw bytes.
    file.write_all(message.as_bytes())?;
    // We add a newline character to keep the log readable.
    file.write_all(b"\n")?;
    Ok(())
}

fn main() {
    ()
}
