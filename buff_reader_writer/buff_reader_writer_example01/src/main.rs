use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn process_log_file_line_by_line() -> io::Result<()> {
    let path = "server_request.log";
    let file = File::open(path)?;

    // Wrap the File handle in a BufReader.
    // The BufReader takes ownership of the file handle and manages it
    let reader = BufReader::new(file);

    // The lines() method creates an iterator that yields one line of text at a time.
    for line_result in reader.lines() {
        // We must unwrap the Result because reading the disk could fail mid-file
        let line = line_result?;
    // We can now process the single line of text
    // Even if the file is ten gigabytes, we only have one line in memory at a time
        if line.contains("ERROR") {
            println!("Found an error:{}", line);
        }
    }
    // lines() -> Each successful iteration produces an independently owned, heap-allocated string.
    // if one line is 500 MB, .lines() may need to allocate a 500 MB String.
    // Owns each returned line
    // May allocate for each line

    Ok(())
}

fn process_log_file2() -> io::Result<()> {
     // read_line() doesn't necessarily eliminate allocations, but it can avoid repeatedly allocating new strings when the existing capacity is sufficient.
    // Reuses your existing String
    // Writes into a mutable buffer you provide
        // Can reuse allocated capacity
    let file = File::open("server_requests.log")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        if line.contains("ERROR") {
            println!("{}", line.trim_end());
        }
    }

    Ok(())
}


fn main() {
    println!("Hello, world!");
}
