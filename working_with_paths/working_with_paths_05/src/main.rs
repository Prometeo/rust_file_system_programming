use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
fn resolve_secure_path(base_dir_str: &str, user_input_str: &str) -> io::Result<PathBuf> {
    // 1. Resolve the secure base directory.
    let base_dir = Path::new(base_dir_str);
    let absolute_base = fs::canonicalize(&base_dir)?;
    // 2. Combine the base directory with the user's input.
    // We use join() to create the combined path.
    let user_path = Path::new(user_input_str);
    let combined_path = absolute_base.join(user_path);
    // 3. Resolve the combined path.
    // Canonicalize will process all the malicious `..` components.
    // If the file does not exist, canonicalize will fail here.
    let absolute_combined = fs::canonicalize(&combined_path)?;
    // 4. The Security Check.
    // We verify that the absolute combined path still starts with the absolute base path.
    if absolute_combined.starts_with(&absolute_base) {
        // The path is safe. The user is asking for a file inside the allowed zone.
        println!("Security Check Passed. Path is securely isolated.");
        Ok(absolute_combined)
    } else {
        // The path is dangerous. The user has broken out of the base directory.
        println!("SECURITY ALERT: Directory traversal attempt blocked!");
        // We return a custom PermissionDenied error to reject the operation safely.
        Err(Error::new(
            ErrorKind::PermissionDenied,
            "Access to paths outside the designated base directory is strictly forbidden.",
        ))
    }
}

fn main() {
    println!("Hello, world!");
}
