use serde::Deserialize;
use std::error::Error;
use std::io::Read;

#[derive(Debug, Deserialize, PartialEq)]
struct UserProfile {
    username: String,
    access_level: u8,
}
// This function does not know about files or hard drives.
// It only knows how to extract a UserProfile from a stream of data.
// We use generics (<R: Read>) to accept any valid readable source.
fn parse_profile_from_stream<R: Read>(data_stream: R) -> Result<UserProfile, Box<dyn Error>> {
    // We use serde_json's from_reader, passing the generic stream directly.
    let profile: UserProfile = serde_json::from_reader(data_stream)?;
    Ok(profile)
}

/// This design is infinitely extensible. When your utility runs in production, you can open a real file from the hard drive and pass the file handle into the function. // Production usage:
/// use std::fs::File;
///  fn run_production() {
/// We open a real file.
/// if let Ok(file) = File::open("production_data.json") {
/// We pass the File handle (which implements Read) to our parser.
/// let _profile = parse_profile_from_stream(file);
/// }
///  }


/// Testing usage:
/// use std::io::Cursor;
///  fn run_test() {
/// We define our test data as a raw string in memory
/// let test_data = r#"
/// {
/// "username": "admin_user",
/// "access_level": 5
/// }
/// "#;
/// We wrap the string in a Cursor so it behaves like a readable file.
/// let memory_stream = Cursor::new(test_data);
/// We pass the memory stream into the exact same parsing function.
/// let result = parse_profile_from_stream(memory_stream);
/// We can confidently assert that our parsing logic works perfectly
/// without ever touching the physical file system.
/// assert!(result.is_ok());
/// let parsed_profile = result.unwrap();
///      assert_eq!(parsed_profile.username, "admin_user");
/// }


fn main() {
    println!("Hello, world!");
}
