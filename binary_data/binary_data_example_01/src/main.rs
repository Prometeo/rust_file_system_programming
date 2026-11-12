use std::fs::File;
use std::io::{self, Read};
// We define a struct to hold the parsed information.
#[derive(Debug)]
struct BinaryFileHeader {
    signature_value: [u8; 4],
    version: u16,
}
fn parse_binary_header() -> io::Result<()> {
    let mut file = File::open("custom_archive.bin")?;
    // Step 1: Read the 4-byte identification number.
    // We create a precise 4-byte array on the stack.
    let mut signature_buffer = [0u8; 4];
    // We demand exactly 4 bytes from the file.
    file.read_exact(&mut signature_buffer)?;
    // Step 2: Read the 2-byte version number.
    // We create a precise 2-byte array.
    let mut version_buffer = [0u8; 2];
    file.read_exact(&mut version_buffer)?;
    // We use a standard library method to convert the two raw bytes
    // into a proper 16-bit unsigned integer. We assume the bytes are
    // stored in little-endian format, which is standard on most computer processors.
    let version_number = u16::from_le_bytes(version_buffer);
    // We populate our struct.
    let header = BinaryFileHeader {
        signature_value: signature_buffer,
        version: version_number,
    };
    println!("Successfully parsed header: {:?}", header);
    Ok(())
}

/// Better with standard library
fn parse_binary_header_efficiently() -> io::Result<()> {
    let mut file = File::open("custom_archive.bin")?;

    // Create a single buffer for the entire 6-byte header
    let mut header_buffer = [0u8; 6];

    // Fetch all 6 bytes in ONE system call
    file.read_exact(&mut header_buffer)?;

    // Slice the bytes locally in RAM (zero overhead)
    let signature: [u8; 4] = header_buffer[0..4].try_into().unwrap();
    let version_bytes: [u8; 2] = header_buffer[4..6].try_into().unwrap();

    let header = BinaryFileHeader {
        signature_value: signature,
        version: u16::from_le_bytes(version_bytes),
    };

    println!("Successfully parsed header: {:?}", header);
    Ok(())
}


/// Even Better
use std::fs;
use binrw::{binread, BinRead, io::Cursor};
// The macro automatically generates the byte-parsing logic,
// assuming Little Endian format by default.
#[binread]
#[br(little)]
#[derive(Debug)]
struct BinaryFileHeader2 {
    signature_value: [u8; 4],
    version: u16,
}

fn parse_with_binrw() -> binrw::BinResult<()> {
    // Read the file into RAM
    let bytes = fs::read("custom_archive.bin")?;
    let mut reader = Cursor::new(bytes);

    // Magically parse the entire struct directly from the bytes
    let header = BinaryFileHeader2::read(&mut reader)?;

    println!("Successfully parsed: {:?}", header);
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
