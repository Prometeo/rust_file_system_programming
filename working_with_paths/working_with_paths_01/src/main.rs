use std::path::{Path, PathBuf};

fn construct_output_path() {
    // We create a new, empty PathBuf.
    // This allocates a small amount of memory on the heap to store our path.
    let mut current_path = PathBuf::new();
    // We use the push method to add components to our path.
    // Rust automatically inserts the correct separator character for the operating system.
    current_path.push("workspace");
    current_path.push("data_exports");
    current_path.push("monthly_report.csv");
    // We can print the resulting path.
    // We must use the {:?} debug formatter because paths might contain non-standard characters.
    println!("The constructed path is: {:?}", current_path);
}

fn demonstrate_push_behavior() {
    // We start with a base directory.
    let mut my_path = PathBuf::from("/var/log/application");
    // We push a relative folder name.
    // The path becomes: /var/log/application/metrics
    my_path.push("metrics");
    println!("After pushing relative folder: {:?}", my_path);
    // Now, we push an absolute path that starts with a root slash.
    // Rust assumes we want to start over completely from this new absolute location.
    // The previous contents of the PathBuf are completely erased.
    // The path becomes: /etc/system_config
    my_path.push("/etc/system_config");
    println!("After pushing absolute folder: {:?}", my_path);
}

fn join_paths_immutably() {
    // We create an immutable Path reference pointing to a base directory.
    let base_directory = Path::new("user_profiles");
    // We use the join method to create a new PathBuf.
    // The original base_directory is not changed.
    let profile_picture = base_directory.join("avatar.png");
    let profile_settings = base_directory.join("settings.json");
    println!("Picture path: {:?}", profile_picture);
    println!("Settings path: {:?}", profile_settings);
}

fn prepare_backup_path(original_path_str: &str) {
    let path = Path::new(original_path_str);
    println!("Analyzing path: {:?}", path);
    // 1. Extract the parent directory.
    // If the path has no parent (for example, it is just "report.txt" in the current directory),
    // parent() might return an empty path.
    let parent_dir = path.parent().unwrap_or_else(|| Path::new(""));
    // 2. Extract the file stem.
    // If the path ends in a directory name or is invalid, this might be None.
    if let Some(stem) = path.file_stem() {
        // 3. Extract the file extension.
        if let Some(extension) = path.extension() {
            // At this point, stem and extension are of type &OsStr.
            // OsStr represents an operating system string, which might not be valid UTF-8.
            // To format them into a new standard string, we must convert them losslessly.
            let stem_string = stem.to_string_lossy();
            let extension_string = extension.to_string_lossy();
            // We construct our new backup file name.
            let backup_file_name = format!("{}_backup.{}", stem_string, extension_string);
            // We join the parent directory with our new file name to create the final path.
            let mut final_backup_path = PathBuf::from(parent_dir);
            final_backup_path.push(backup_file_name);
            println!(
                "The backup file should be saved to: {:?}",
                final_backup_path
            );
        } else {
            println!("This file does not have an extension. Cannot proceed with backup logic.");
        }
    } else {
        println!("Could not determine the file stem. The path might be a root directory.");
    }
}

fn main() {
    construct_output_path();
}
