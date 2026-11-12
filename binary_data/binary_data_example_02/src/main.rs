use memmap2::Mmap;
use std::fs::File;

fn process_file_with_memory_map() -> Result<(), Box<dyn std::error::Error>> {
    // We open a standard read-only file handle.
    let file = File::open("massive_database.db")?;
    // We instruct the memmap2 crate to map the file into memory.
    // Notice the unsafe keyword. We will explain this immediately after the code.
    let memory_map = unsafe { Mmap::map(&file)? };
    // The memory map behaves exactly like a standard slice of bytes &[u8].
    // We can check its total size instantly.
    println!(
        "File mapped successfully. Total size: {} bytes",
        memory_map.len()
    );
    // We can access any byte randomly with incredible speed.
    // If the file has at least 10,000 bytes, we print the byte at that index.
    if memory_map.len() > 10_000 {
        let specific_byte = memory_map[10_000];
        println!("The byte at index 10,000 is: {}", specific_byte);
    }
    // When the memory_map variable goes out of scope, the mapping is
    // automatically unmapped by the operating system cleanly.
    Ok(())
}

/// When Memory Mapping is the Right Choice
/// - Random Access on Massive Files: If you are building a database, a search index, or querying a 50 GB dataset where you need to constantly jump around to read tiny pieces of data out of order.
/// - Zero-Copy Binary Parsing: If you are reading complex, fixed-size binary formats (like a 3D model, a video game asset, or an executable file) where you want to instantly
///   map complex C-style structs directly over the bytes without copying them into a secondary buffer.
/// - Inter-Process Communication (IPC): Memory mapping is the fastest way for two completely separate programs running on the same computer to share data. Both programs map the same
///   file into their virtual memory, allowing them to read and write to the exact same bytes simultaneously at RAM speeds.

fn main() {
    println!("Hello, world!");
}
