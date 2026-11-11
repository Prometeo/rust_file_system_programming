use std::fs::File;
use std::io::{self, BufReader, Read, BufRead};
use std::time::Instant;
fn compare_read_performance() -> io::Result<()> {
    // We assume we have a medium-sized file available for testing.
    let target_file = "sample_dataset.dat";
    // We ensure the file exists before we start our test.
    if !std::path::Path::new(target_file).exists() {
        println!(
            "Please create a file named {} to run this test.",
            target_file
        );
        return Ok(());
    }
    // Test 1: Reading without buffering.
    // We open a raw file handle.
    let mut raw_file = File::open(target_file)?;
    // We create a tiny one-byte array on the stack.
    let mut single_byte_buffer = [0u8; 1];
    let mut raw_bytes_read = 0;
    println!("Starting unbuffered read...");
    let start_raw = Instant::now();
    // We read the file one byte at a time.
    // Every single iteration of this loop triggers a system call.
    while raw_file.read(&mut single_byte_buffer)? > 0 {
        raw_bytes_read += 1;
    }
    let duration_raw = start_raw.elapsed();
    // When Test 1 runs, the operating system has to physically spin up the hard drive or query the SSD to fetch the file. As it does this,
    // the OS silently copies that file into its own ultra-fast RAM (the Page Cache).
    println!("Unbuffered read took: {:?}", duration_raw);
    // Test 2: Reading with buffering.
    // We open a fresh raw file handle.
    let buffered_file = File::open(target_file)?;
    // We wrap the raw handle in a BufReader.
    // The BufReader creates a large internal array automatically.
    let mut reader = BufReader::new(buffered_file);
    let mut buffered_bytes_read = 0;
    // When Test 2 runs a millisecond later, the file is already sitting in the OS's RAM. It doesn't touch the physical disk at all.
    // Therefore, Test 2 will look artificially faster not just because of BufReader, but because it is reading from RAM while Test 1 had to read from the physical disk.
    println!("Starting buffered read...");
    let start_buffered = Instant::now();
    // We still ask for data one byte at a time.
    // However, the BufReader intercepts the request. It only makes a
    // system call when its internal array is completely empty.
    while reader.read(&mut single_byte_buffer)? > 0 {
        buffered_bytes_read += 1;
    }
    let duration_buffered = start_buffered.elapsed();
    println!("Buffered read took: {:?}", duration_buffered);
    println!("Total bytes processed in each test: {}", raw_bytes_read);
    Ok(())
}


/// Better approach
fn process_bytes_idiomatically() -> io::Result<()> {
    let file = File::open("sample_dataset.dat")?;
    let mut reader = BufReader::new(file);
    let mut total_bytes_processed = 0;

    loop {
        // 1. Get a direct reference to the data sitting in the buffer.
        // This makes a system call ONLY if the buffer is empty.
        let buffer = reader.fill_buf()?;

        // If the buffer is empty after trying to fill it, we hit End of File.
        let length = buffer.len();
        if length == 0 {
            break;
        }

        // 2. Iterate directly over the slice in memory.
        // This is incredibly fast. Zero function call overhead per byte, zero copying.
        for &byte in buffer {
            // Do your processing here.
            // e.g., if byte == b'\n' { ... }
        }

        total_bytes_processed += length;

        // 3. Tell the BufReader we have finished looking at those bytes
        // so it can discard them and load the next chunk on the next loop.
        reader.consume(length);
    }

    println!("Total bytes processed: {}", total_bytes_processed);
    Ok(())
}

fn read_with_custom_buffer(path: &str) -> io::Result<()> {
    // bumping the capacity to 64 KB, 128 KB, or even 1 MB will often yield a massive performance increase, as it aligns better with
    // the operating system's internal read-ahead mechanisms and the SSD's block size.

    // If your application frequently uses .seek() to jump to different parts of a file (like querying a database index or reading a zip archive),
    // a large BufReader can actually destroy your performance.

    // For random access, the standard approaches are:
    // - Use a very small BufReader (or no BufReader at all).
    // - Use OS-specific atomic read functions (like pread on Linux/macOS or seek_read on Windows) which read bytes from a specific offset without altering the file cursor or maintaining a buffer.
    // - Use Memory-Mapped Files (mmap), which hands the entire buffering strategy over to the operating system's virtual memory manager. This is what real database engines (like SQLite or PostgreSQL) do.
    let file = File::open(path)?;

    // Create a 128 KB buffer instead of the default 8 KB
    let capacity = 128 * 1024;
    let mut reader = BufReader::with_capacity(capacity, file);

    // Process the file...
    Ok(())
}

fn main() {
    // To write a valid benchmark, you must "warm up" the cache by reading the file once before starting any timers,
    // so both tests are reading from RAM. That isolates the actual overhead of the system calls.

    // Even with BufReader, calling .read(&mut single_byte_buffer) one million times for a 1MB file is terrible practice.
    // BufReader does prevent a system call for every byte, but you are still incurring the cost of a Rust function call, buffer bounds checking,
    // and memory copying for every single byte. It is horribly inefficient.

    // Instead, you use the BufRead trait's fill_buf() and consume() methods. This allows you to look directly at the BufReader's internal memory array without copying the bytes out of it one by one.
    println!("Hello, world!");
}
