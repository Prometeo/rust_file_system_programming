use std::fs::File;
use std::io::{self, BufWriter, Write};

fn generate_massive_datase() -> io::Result<()> {
    let path = "generated_dataset.csv";

    let file = File::create(path)?;

    // Wrap the file in a BufWriter
    let mut writer = BufWriter::new(file);

    // write data in a tight loop
    for i in 0..100_000 {
        let row = format!("{}, item_name,active\n", i);

        // We write to the BufWriter
        // it holds data in memory and only talks to the disk occasionally.
        writer.write_all(row.as_bytes())?;
    }

    // Critical step: Flushing the buffer
    // When the bufer hows out of scope, it attempts to flush automatically.
    // Howerver, if the final flush fails, the error is silently ignored during drop.
    // Calling flush manually allows you to catch and handle any final disk errors.
    writer.flush()?;

    println!("Dataset generated successfully.");
    Ok(())


}

fn main() {
    println!("Hello, world!");
}
