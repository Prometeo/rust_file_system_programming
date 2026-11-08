use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;
// 1. We define a struct to represent the state of a file at a specific moment.
// We derive Debug so we can print it easily, and PartialEq so we can use the == operator.
#[derive(Debug, PartialEq)]
pub struct FileSnapshot {
    size_in_bytes: u64,
    last_modified: SystemTime,
}
impl FileSnapshot {
    // 2. We build a function to generate a snapshot from a live file.
    pub fn capture_from_path(file_path: &Path) -> io::Result<FileSnapshot> {
        // We use metadata (or symlink_metadata depending on our tool's design).
        let metadata = fs::metadata(file_path)?;
        // We extract the size.
        let current_size = metadata.len();
        // We attempt to extract the modified time.
        // We use a fallback just in case the file system does not support it.
        // Returning the epoch (start of time) is a safe default for broken timestamps.
        let current_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        // We construct and return our snapshot struct.
        Ok(FileSnapshot {
            size_in_bytes: current_size,
            last_modified: current_time,
        })
    }
    // 3. We build a method to compare this snapshot against another snapshot.
    // It returns true if the file appears to have been altered.
    pub fn has_changed_since(&self, previous_snapshot: &FileSnapshot) -> bool {
        // If the size is different, the file has definitely changed.
        if self.size_in_bytes != previous_snapshot.size_in_bytes {
            return true;
        }
        // If the modified time is different, the file has likely changed.
        // We use the != operator, which works cleanly on SystemTime objects.
        if self.last_modified != previous_snapshot.last_modified {
            return true;
        }
        // If both match, we consider the file unchanged.
        false
    }
}
// 4. We simulate using the snapshot system in a workflow.
fn run_synchronization_workflow() {
    let target_file = Path::new("important_database.db");
    // Phase A: The initial run. We capture the state and save it.
    // In a real utility, you would save this snapshot to a database or a tracking file.
    println!("Initial scan...");
    let initial_state_result = FileSnapshot::capture_from_path(&target_file);
    // We assume the initial capture was successful for the sake of the simulation.
    if let Ok(initial_state) = initial_state_result {
        println!("Initial snapshot recorded: {:?}", initial_state);
        // ... Time passes. The user might or might not modify the file ...
        // Phase B: The next run. We want to know if we need to back up the file again.
        println!("Second scan...");
        if let Ok(current_state) = FileSnapshot::capture_from_path(&target_file) {
            // We use our clean comparison logic.
            if current_state.has_changed_since(&initial_state) {
                println!("Action required: The file has been modified.");
                println!("We will begin backing up the new version...");
            // Execute backup logic...
            } else {
                println!("No action required: The file is identical.");
                println!("Skipping backup operation to save time and bandwidth.");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
