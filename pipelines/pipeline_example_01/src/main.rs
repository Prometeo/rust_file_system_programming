use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn high_performance_pipeline() -> io::Result<()> {
    let file = File::open("system_events.log")?;
    let reader = BufReader::new(file);
    // We build a continuous pipeline using iterators.
    let total_length: usize = reader
        // 1. The Source: We create an iterator of lines.
        .lines()
        // 2. Transformer A: We handle potential file reading errors.
        // The filter_map adapter drops any lines that caused an I/O error,
        // allowing us to focus only on successful strings.
        .filter_map(|line_result| line_result.ok())
        // 3. Transformer B: We filter out lines that do not contain our keyword.
        // The line is passed in as a reference. If the closure returns true,
        // the line moves to the next step. If false, it is immediately discarded.
        .filter(|line| line.contains("Error"))
        // 4. Transformer C: We map the string into a completely different type.
        // We take the text string and transform it into an integer representing its length.
        .map(|line| line.len())
        // 5. The Sink: We aggregate the stream of integers.
        // The sum adapter consumes the entire iterator stream and adds all the numbers together.
        .sum();
    println!("Total length of all error messages: {}", total_length);
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
