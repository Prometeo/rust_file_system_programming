use std::fs;
use std::io;
use std::time::{Duration, SystemTime};

fn analyze_file_age(file_path: &str) -> io::Result<()> {
    let metadata = fs::metadata(file_path)?;
    // We attempt to retrieve the modified time.
    // This returns a Result because some obscure or network file systems
    // might not support tracking modification times.
    if let Ok(modified_time) = metadata.modified() {
        // We get the current time from the computer's clock.
        let current_time = SystemTime::now();
        // We calculate the difference between now and the modified time.
        // The duration_since method returns a Result. It will fail if the
        // modified time is somehow in the future, which can happen if the
        // user's system clock was changed manually.
        match current_time.duration_since(modified_time) {
            Ok(duration) => {
                // The duration object allows us to extract the time in seconds.
                let seconds_old = duration.as_secs();
                let days_old = seconds_old / 86400; // 86400 seconds in a day.
                println!("This file was modified {} days ago.", days_old);
                if days_old > 30 {
                    println!("Notice: This file is older than 30");
                }
            }
            Err(_) => {
                println!("Error: The file's modified time is in the future.");
                println!("Please check your system clock synchronization.");
            }
        }
    } else {
        println!("This file system does not support modification timestamps.");
    }
    // We can also retrieve the creation time.
    // Creation time is less reliable across platforms. Some older Linux
    // file systems do not track file birth times at all.
    if let Ok(created_time) = metadata.created() {
        println!("Successfully retrieved the creation timestamp.");
    // We could perform similar duration calculations here.
    } else {
        println!("Creation time is not available on this specific file system.");
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
