use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, BufWriter};

fn stream_and_mask_data_safely() -> io::Result<()> {
    let input_file = File::open("raw_system_data.txt")?;
    let output_file = File::create("sanitized_system_data.txt")?;

    // Use BufReader and BufWriter for high performance
    let mut reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    // Allocate a single String to reuse, avoiding massive memory allocations
    let mut line_buffer = String::new();

    loop {
        // read_line reads exactly up to the next '\n' boundary.
        // It guarantees we never split a word or a UTF-8 character.
        let bytes_read = reader.read_line(&mut line_buffer)?;

        if bytes_read == 0 {
            break; // EOF
        }

        // Apply the transformation safely
        let sanitized_text = line_buffer.replace("PASSWORD", "********");

        // Write it out
        writer.write_all(sanitized_text.as_bytes())?;

        // Clear the buffer for the next line, but keep the memory allocated
        line_buffer.clear();
    }

    // Ensure final bytes are pushed to disk
    writer.flush()?;

    Ok(())
}

fn main() {
    ()
}
