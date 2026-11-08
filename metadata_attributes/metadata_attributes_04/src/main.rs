#[cfg(unix)]
mod unix_permissions {
    use std::fs;
    use std::io;
    // We import the PermissionsExt trait to interact with the Unix mode.
    use std::os::unix::fs::PermissionsExt;
    pub fn secure_sensitive_file(file_path: &str) -> io::Result<()> {
        // 1. We retrieve the current metadata and extract the permissions object.
        let metadata = fs::metadata(file_path)?;
        let mut permissions = metadata.permissions();
        // 2. We use the .mode() method provided by PermissionsExt to get the numeric mode.
        // The mode contains extra bits at the beginning for file type information,
        // so we use a bitwise AND operation (& 0o777) to strip away everything except
        // the bottom nine permission flags we care about.
        let current_mode = permissions.mode() & 0o777;
        // We format the number as an octal string for printing using {:o}.
        println!("Current Unix permissions mode: {:o}", current_mode);
        // 3. We define our target secure mode.
        // 0o600 means Read (4) + Write (2) for the owner (6),
        // and nothing (0) for group, and nothing (0) for others.
        let target_secure_mode = 0o600;
        if current_mode != target_secure_mode {
            println!("File is not secure. Updating permissions...");
            // 4. We use the .set_mode() method to update the permissions object in memory.
            permissions.set_mode(target_secure_mode);
            // 5. We apply the modified permissions object back to the actual file system
            // using the fs::set_permissions function.
            fs::set_permissions(file_path, permissions)?;
            println!("Permissions successfully updated to 600.");
        } else {
            println!("File is already secure.");
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
