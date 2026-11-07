use std::fs::File;
use std::io::{self, Read, Write};
use std::fs;

fn stream_large_file_copy(source_path: &str, destination_path: &str) -> io::Result<()> {
    let mut source = File::open(source_path)?;
    let mut destination = File::create(destination_path)?;
    // 64 Kilobytes is a generally excellent chunk size for modern hard drives.
    // The syntax [0; 65536] creates an array of 65,536 zeros.
    let mut buffer = [0u8; 65536];
    loop {
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let valid_data = &buffer[..bytes_read];
        destination.write_all(valid_data)?;
    }
    println!("Large file successfully copied using a constant memory footprint.");
    Ok(())
}

///better version ->
fn copy_file(
    source_path: &str,
    destination_path: &str,
) -> io::Result<u64> {
    // still need to validate source and destination aren't the same file
    // this would truncate the source file
    let mut source = File::open(source_path)?;
    let mut destination = File::create(destination_path)?;

    io::copy(&mut source, &mut destination)
}

fn same_file(source: &str, destination: &str) -> io::Result<bool> {
    // Validating they are not the same file, validate even symbolic links
    let source_metadata = fs::metadata(source)?;
    let destination_metadata = match fs::metadata(destination) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(false);
        }
        Err(error) => return Err(error),
    };

    use std::os::unix::fs::MetadataExt;

    Ok(
        source_metadata.dev() == destination_metadata.dev()
            && source_metadata.ino() == destination_metadata.ino()
    )
}

fn main() {
    println!("Hello, world!");
}
