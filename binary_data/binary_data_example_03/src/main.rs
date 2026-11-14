use std::fs::File;
use std::io::{self, BufWriter, Write};
// We import the WriteBytesExt trait and the LittleEndian standard from the byteorder crate.
use byteorder::{LittleEndian, WriteBytesExt};
fn export_binary_metrics() -> io::Result<()> {
    let file = File::open("sensor_cache.bin")?;
    let mut writer = BufWriter::new(file);
    println!("Writing raw binary data to disk...");
    for i in 0..10_000 {
        // We write a 32-bit unsigned integer using explicit LittleEndian ordering.
        writer.write_u32::<LittleEndian>(i)?;
        // We calculate a floating-point value and write it as a 64-bit float.
        let sensor_value = (i as f64) * 3.14;
        writer.write_f64::<LittleEndian>(sensor_value)?;
    }
    // Always flush buffered writers.
    writer.flush()?;
    println!("Binary export complete.");
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
