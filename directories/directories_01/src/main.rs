use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn setup_simple_folder() -> io::Result<()> {
    // We attempt to create a single directory named "exports".
    fs::create_dir("exports")?;
    println!("Successfully created the exports directory.");
    Ok(())
}

fn setup_complex_workspace() -> io::Result<()> {
    // We define a deeply nested path.
    let workspace_path = "application/data/user_profiles/backups";
    // This single call creates 'application', then 'data',
    // then 'user_profiles', and finally 'backups'.
    fs::create_dir_all(workspace_path)?;
    println!("The entire workspace path is guaranteed to exist.");
    Ok(())
}

fn reorganize_folders() -> io::Result<()> {
    // We ensure our starting folders exist for the
    fs::create_dir_all("old_workspace")?;
    fs::create_dir_all("archive")?;
    // We move and rename the folder in a single operation.
    // The folder "old_workspace" will disappear, and its contents
    // will now be located at "archive/project_alpha".
    fs::rename("old_workspace", "archive/project_alpha")?;
    println!("Folder successfully moved and renamed.");
    Ok(())
}

fn inspect_current_directory() -> io::Result<()> {
    let target_folder = "."; // The dot represents the current working directory.
    // Stage 1: We attempt to open the directory.
    // This returns an Iterator, or an error if the folder cannot be opened.
    let directory_iterator = fs::read_dir(target_folder)?;
    println!("Contents of the current directory:");
    // We loop over the iterator.
    for entry_result in directory_iterator {
        // Stage 2: We must handle potential errors for each individual entry.
        let entry = entry_result?;
        // Once we have a valid entry, we can extract its information.
        // We get the file name as an OS-specific string.
        let file_name = entry.file_name();
        // We can print it. We use the {:?} debug formatter because
        // file names are not guaranteed to be valid UTF-8 text.
        println!("- {:?}", file_name);
    }
    Ok(())
}

fn categorize_directory_contents(folder_path: &str) -> io::Result<()> {
    let entries = fs::read_dir(folder_path)?;
    let mut file_count = 0;
    let mut dir_count = 0;
    for entry_result in entries {
        let entry = entry_result?;
        // We fetch the cached file type information.
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            dir_count += 1;
            println!("Directory: {:?}", entry.file_name());
        } else if file_type.is_file() {
            file_count += 1;
            println!("File: {:?}", entry.file_name());
        } else {
            println!("Other (e.g., Symlink): {:?}", entry.file_name());
        }
    }

    println!("Summary: {} files, {} directories.", file_count, dir_count);
    Ok(())
}

// Our function takes a path to start from. (DFS)
fn depth_first_walk(dir_path: &Path) -> io::Result<()> {
    // We open the current directory.
    let entries = fs::read_dir(dir_path)?;
    for entry_result in entries {
        let entry = entry_result?;
        let current_path = entry.path();
        // We print the path we are currently looking at.
        println!("Found: {:?}", current_path);
        // If the item we just found is a directory...
        if current_path.is_dir() {
            // We call our own function again, passing in this new directory.
            // This pauses the current loop and plunges deeper into the new folder.
            depth_first_walk(&current_path)?;
        }
    }
    Ok(())
}

fn breadth_first_walk(start_dir: &str) -> io::Result<()> {
    // We create a Vector to act as our work queue.
    let mut folders_to_visit: Vec<PathBuf> = Vec::new();
    // We push our starting directory into the queue.
    folders_to_visit.push(PathBuf::from(start_dir));
    // We keep looping as long as there is work in the queue.
    // The pop() method removes and returns the last item in the Vector.
    // To act strictly like a breadth-first queue, we should remove from the front,
    // but taking from the back is much faster in Rust Vectors and works fine for avoiding recursion.
    while let Some(current_dir) = folders_to_visit.pop() {
        // We try to read the directory.
        let entries = match fs::read_dir(&current_dir) {
            Ok(e) => e,
            Err(err) => {
                // If we get a permission denied on one folder, we log it
                // and continue processing the rest of the queue.
                eprintln!("Could not read directory {:?}: {}", current_dir, err);
                continue;
            }
        };
        for entry_result in entries {
            let entry = entry_result?;
            let path = entry.path();
            println!("Found: {:?}", path);
            // If we find a new directory, we add it to our list of places to visit later.
            if path.is_dir() {
                folders_to_visit.push(path);
            }
        }
    }
    Ok(())
}


fn main() {
    println!("Hello, world!");
}
