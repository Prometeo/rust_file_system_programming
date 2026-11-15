use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_largest_megabyte_value() -> io::Result<()> {
    let file = File::open("file_size_records.txt")?;
    let reader = BufReader::new(file);
    // We build the pipeline.
    let largest_size_mb: f64 = reader
        .lines()
        .filter_map(|line_result| line_result.ok())
        // Transformer: Map the text string into a parsed floating-point number.
        // The parse() method returns a Result. If the text is not a valid number,
        // it returns an Error. We use .ok() to convert the Result into an Option.
        // filter_map automatically drops any None values, meaning invalid text
        // lines are safely ignored and removed from the stream.
        .filter_map(|text_line| text_line.parse::<f64>().ok())
        // Transformer: Map the raw bytes into megabytes.
        .map(|bytes| bytes / (1024.0 * 1024.0))
        // The Sink: We use fold to find the maximum value.
        // The first argument (0.0) is the starting value of our accumulator.
        // The closure takes the current accumulator and the new incoming item.
        .fold(0.0, |current_maximum, incoming_megabytes| {
            // We use the standard f64::max method to compare the two.
            // Whichever is larger becomes the new accumulator for the next round.
            current_maximum.max(incoming_megabytes)
        });
    println!(
        "The largest file recorded is {:.2} Megabytes.",
        largest_size_mb
    );
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
