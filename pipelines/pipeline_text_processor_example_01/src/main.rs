use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn clean_reporting_data() -> io::Result<()> {
    // 1. Establish the Source.
    let input_file = File::open("messy_report_data.txt")?;
    let reader = BufReader::new(input_file);
    // 2. Establish the Sink.
    // We create the output file and wrap it in a BufWriter for efficient saving.
    let output_file = File::create("clean_report_data.txt")?;
    let mut writer = BufWriter::new(output_file);
    println!("Starting text transformation pipeline...");
    // 3. Define the Transformers as an iterator chain.
    let clean_stream = reader
        .lines()
        // Safely extract successful lines, dropping any underlying read errors.
        .filter_map(|result| result.ok())
        // Transformer A: Remove empty lines.
        // We use the trim() method to remove invisible whitespace before checking
        // if the string is empty.
        .filter(|line| !line.trim().is_empty())
        // Transformer B: Remove debug messages.
        // If the line starts with "DEBUG:", it returns true. We negate it with !
        // so that debug lines return false and are destroyed.
        .filter(|line| !line.starts_with("DEBUG:"))
        // Transformer C: Standardize formatting.
        // We map the surviving clean string into a new uppercase string.
        .map(|line| line.to_uppercase());
    // 4. Consume the stream and feed the Sink.
    // The for loop pulls items from our configured iterator chain one by one.
    let mut lines_processed = 0;
    for transformed_line in clean_stream {
        // We write the transformed string to the output file.
        // The writeln! macro automatically adds the newline character back in.
        // We use the ? operator to handle any disk full or permission errors instantly.
        writeln!(writer, "{}", transformed_line)?;
        lines_processed += 1;
    }
    // 5. Secure the Sink.
    // We must flush the BufWriter to ensure the final block of data is saved.
    writer.flush()?;
    println!(
        "Pipeline finished. {} pristine lines saved to output.",
        lines_processed
    );
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
