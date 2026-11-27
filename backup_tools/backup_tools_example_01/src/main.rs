use blake3::Hasher;
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tar::Builder;
use walkdir::WalkDir;
use chrono::Local;

// We define a truct to hold the results of our difference calculation.
#[derive(Debug)]
pub struct BackupDiff {
    pub added_files: Vec<PathBuf>,
    pub modified_files: Vec<PathBuf>,
    pub deleted_files: Vec<PathBuf>,
}

pub fn compatre_manifest(
    old_manifest: &BackupManifest,
    new_manifest: &BackupManifest,
) -> BackupDiff {
    println!("Comparing current system state against previous backup manifest...");
    println!("Comparing current system state against previous backup manifest...");
    let mut diff = BackupDiff {
        added_files: Vec::new(),
        modified_files: Vec::new(),
        deleted_files: Vec::new(),
    };

    // Step 1: We convert the old maifest into a high-speed HashMap
    // The key is the file path. The value is a rederence to the FileRecord.
    let mut old_files_map: HashMap<&PathBuf, &FileRecord> = HashMap::new();

    for old_record in &old_manifest.files {
        old_files_map.insert(&old_record.path, old_record);
    }

    // Step 2: We iterate through every file in the NEW manifest.
    for new_record in &new_manifest.files {
        // We ask the old HasjMap if it containes the path of this new file.
        match old_files_map.remove(&new_record.path) {
            // Case A: The  path exists in the old manifest.
            Some(old_record) => {
                // The file existed yesterday, and it exists today.
                // We muts check if the contents have changed by comparing the hashes.
                if old_record.content_hash != new_record.content_hash {
                    // the hashes are different. The files has been modified.
                    diff.modified_files.push(new_record.path.clone());
                }

                // If the hashes are identical, we do absolutely nothing.
                // The file is unchanged and does not need to be backed up again
            }

            // Case B: The path does NOT exist in the old manifest.
            None => {
                // The file exists today, but the HashMap has no record of it.
                // Therefore, it must be a newly created file.
                diff.added_files.push(new_record.path.clone());
            }
        }
    }

    // Step 3: Identify Deleted Files.
    // During Step 2, we used the remove() method on the HashMap when  we found a match.
    // This means that by the end of step 2, any records left sitting inside
    // the old_files_map must be files that existed yesterday, but were never
    // found during todays's scan. Therefore, they have been deleted.

    for (deleted_path, _) in old_files_map {
        diff.deleted_files.push(deleted_path.clone());
    }

    println!(
        "Diff complete. {} added, {} modified, {} deleted.",
        diff.added_files.len(),
        diff.modified_files.len(),
        diff.deleted_files.len()
    );
    diff
}

// We define a struct to represent a single file in our backup source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    // The relative path of the file from the root of our backup directory.
    pub path: PathBuf,
    // The exact size of the file in bytes.
    pub size: u64,
    // The modification time, stored as the number of seconds since the Unix Epoch
    // We use a u64 integer here instead of System Time because integers are
    // much easier to serialize and deserialize to JSON
    pub modified_seconds: u64,

    // We will leave room for a cryptographic hash, which we will implement shortly.
    pub content_hash: String,
}

// We define a struct to represent the entire snapshot.
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    // A timestamp indicating when this backup scan was performed.
    pub scan_timestamp: u64,
    // A list pf all the files we discovered during the scan.
    pub files: Vec<FileRecord>,
}

// A helper function to get the current time as a simple integer
fn current_time_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn calculate_file_hash(file_path: &Path) -> io::Result<String> {
    // We open the file for reading
    let mut file = File::open(file_path)?;

    // We create a new, empty BLAKE3 hasher object.
    let mut hasher = Hasher::new();

    // We define a memory buffer on the stack.
    // A 64-kilobyte buffer is highly efficient for disk reading and hashing.
    let mut buffer = [0u8; 65536];
    loop {
        // We read a chunk of data from the hard drive into our memory buffer.
        let bytes_read = file.read(&mut buffer)?;

        // If we read zero bytes, we have reached the end of the file.
        if bytes_read == 0 {
            break;
        }

        // We isolate the actual data we just read.
        let valid_data = &buffer[..bytes_read];

        // We feed the data chunk into the hasher.
        // The hasher updates its internal mathematical state and waits for more data.
        hasher.update(valid_data);
    }

    // Once the entire file has been streamed through the hasher, we ask for the final result.
    let final_hash = hasher.finalize();

    // We convert the binary hash output into a standard hexadecimal string.
    // This allows us to easily save it to our JSON manifest later.
    let hash_string = final_hash.to_hex().to_string();
    Ok(hash_string)

    // We enter ourt standard stream processing loop.
}

// A helper function to get a file's modified time as integer
fn extract_modified_seconds(metadata: &fs::Metadata) -> u64 {
    metadata
        .modified()
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn generate_manifest(source_directory: &str) -> BackupManifest {
    println!("Scanning source directory: {}", source_directory);

    let mut recorded_files = Vec::new();

    // We begin our recursive directory walk.
    for entry_result in WalkDir::new(source_directory) {
        // We gracefully skip any files or folders we do not have permission to read.
        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: Skipping inaccessible path. Details: {}", err);
                continue;
            }
        };

        // We only want to record files, not directory folders.
        // The folders will be recreated automatically when restoring the files.
        if !entry.file_type().is_file() {
            continue;
        }

        // We attempt to read the metadata for the file.
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                eprintln!(
                    "Warning: Could not read metadata for {:?}. Error: {}",
                    entry.path(),
                    err
                );
                continue;
            }
        };

        let file_path = entry.path();

        // We attempt to calculate the hash.
        // If the hashing fails, we handle it gracefully by assigning a fallback string.
        let hash_value = match calculate_file_hash(file_path) {
            Ok(h) => h,
            Err(e) => {
                eprintln!("Warning: Could not hash file {:?}. Error: {}", file_path, e);
                String::from("UNHASHABLE_DUE_TO_ERROR")
            }
        };

        // We create a new FileRecord for this specific file.
        let record = FileRecord {
            // We store the full path.
            path: entry.path().to_path_buf(),
            size: metadata.len(),
            modified_seconds: extract_modified_seconds(&metadata),
            content_hash: hash_value,
        };

        // We add the record to our growing inventory list.
        recorded_files.push(record);
    }

    // We construct the final manifest object after completing the directory walk.
    let manifest = BackupManifest {
        scan_timestamp: current_time_seconds(),
        files: recorded_files,
    };

    println!("Scan complete. Indexed {} files.", manifest.files.len());

    manifest
}

pub fn create_tar_archive(
    output_archive_path: &str,
    source_root_directory: &str,
    files_to_backup: Vec<PathBuf>,
) -> io::Result<()> {
    println!("Begining archiving process to {}...", output_archive_path);

    // 1. We create the final output file where the archive will be stored.
    let archive_file = File::create(output_archive_path)?;

    // 2. We wrap the output file where the archvive will be stored.
    // The builder takes ownership of the file handle and will manage the
    // complex header writing process for us
    let mut tar_builder = Builder::new(archive_file);

    let source_root_path = Path::new(source_root_directory);

    // 3. We iterate through the specific list of files we identified as changed or new.
    let mut files_archived = 0;

    for absolute_file_path in files_to_backup {
        // Before we put the file in the archive, we must calculate its relative path.
        // If the absolute path is /home/user/project/data.txt, and the root is /home/usr/project,
        // we only want to store "data.txt" in the archive.
        // if we store absolute path in an archive, extracting them on a different computer
        // can overwrite sensitive system folders.

        let relative_path = match absolute_file_path.strip_prefix(source_root_path) {
            Ok(rel) => rel,
            Err(_) => {
                // If the path stripping fails, it means the file is somehow outside of our root.
                // We log the erroor and skip the file for safety.
                eprintln!(
                    "Error: Path {:?} is outside the source root. Skipping.",
                    absolute_file_path
                );
                continue;
            }
        };

        // 4. We open the physical file on the hard drive for reading.
        let mut source_file = match File::open(&absolute_file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "Warning: Could not open file for archiving: {:?}. Error: {}",
                    absolute_file_path, e
                );
                continue;
            }
        };
        // 5. We instruct the tar builder to append the file to the archive.
        // We provide the safe relative path for the archive header, and a reference
        // to the open file handle. The builder will stream the data from the soruce file
        // direwctly into the archive efficiently.
        if let Err(e) = tar_builder.append_file(relative_path, &mut source_file) {
            eprintln!("Error appending {:?} to archive {}", relative_path, e);
        } else {
            files_archived += 1;
        }
    }

    // 6.We finalize the archive.
    // This writes the closing markers required by the tar specification.
    tar_builder.into_inner()?;

    println!(
        "Archiving complet. Successfully bundled {} files.",
        files_archived
    );
    Ok(())
}

pub fn create_compressed_archive(
    output_archive_path: &str,
    source_root_directory: &str,
    files_to_backup: Vec<PathBuf>,
) -> io::Result<()> {
    println!("Beginning compressed archiving process...");

    // 1. Create the final output file.
    // We update th extension to tar.gz to indicate it is a compressed tarball.
    let file_path_with_ext = format!("{}.tar.gz", output_archive_path);
    let archive_file = File::create(&file_path_with_ext)?;

    // 2. We wrap the physical file handle in a Gzip Encoder
    // We set the compression level to Defult, which balances speed and size.
    let gzip_encoder = GzEncoder::new(archive_file, Compression::default());

    // 3. We wrap the Gzip Encoder inside the tar Builder.
    // Because GzEncoder implements the Write trait, the tar builder accepts it
    // exactly as if it were a normal file.
    let mut tar_builder = Builder::new(gzip_encoder);

    let source_root_path = Path::new(source_root_directory);
    let mut files_archived = 0;

    for absolute_file_path in files_to_backup {
        if let Ok(relative_path) = absolute_file_path.strip_prefix(source_root_path) {
            if let Ok(mut source_file) = File::open(&absolute_file_path) {
                // The append process remains exactly the same as the previos function
                // The data flows through the builder, into the encodder, and onto the disk.
                if tar_builder
                    .append_file(relative_path, &mut source_file)
                    .is_ok()
                {
                    files_archived += 1;
                }
            }
        }
    }

    // 4. Finalization
    // Callin into_inner on the tar builder finishes the tar format and returns the encoder.
    let encoder_result = tar_builder.into_inner()?;

    //Calling finish on the encoder finalizes the gzip format and ensures everythin is written.
    encoder_result.finish()?;

    println!(
        "Compresed archiving complete. Bundled {} files.",
        files_archived
    );
    Ok(())
}

pub fn execute_backup_run(source_dir: &str, destination_dir: &str, previous_manifest_path: Option<&str>) -> io::Result<()> {
    // 1. Ensure the destination directory exists safely.
    fs::create_dir_all(destination_dir)?;

    // 2. Generate a unique version string based on the current date and time.
    // Example format: 2024-10-25_14-30-00
    let timestamp_string = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();

    // 3. Coonstruct the organized output paths based on the timestamp.
    let base_name = format!("backup_{}", timestamp_string);

    let mut manifest_output_path = PathBuf::from(destination_dir);
    manifest_output_path.push(format!("{}_manifest.json", base_name));

    let mut archive_output_path = PathBuf::from(destination_dir);
    archive_output_path.push(&base_name); // The compression function adds .tar.gz

    // 4. Generate the current snapshot manifest.
    let current_manifest = generate_manifest(source_dir);

    // 5. Determine which files to back up.
    let files_to_backup = if let Some(old_manifest_path) = previous_manifest_path {
        // If we have a previous manifest, we are performing an Incremental backup.
        // We would  load the old manifest using serde_json here ...
        // Let us assume we have loaded it into `old_manifest`.
        // let old_manifest = load_manifest(old_manifest_path)?;

        // We use our diff engine to find the changes.
        // let diff = compare_manifest(&old_manifest, &current_manifest);

        // We bundle the added and modified files together.
        // let mut required_files = diff.added_files;
        // required_files.extend(diff.modified_files);
        // required_files

        // p;aceholder for the compilation
        Vec::new()

    } else {
        // If there is no previous manifest, we are performing a Full backup.
        // We must extract every single path from the current manifest.
        current_manifest.files.iter().map(|record| record.path.clone()).collect()
    };

    // 6. Execute the physical archiving process.
    if !files_to_backup.is_empty() {
        create_compressed_archive(archive_output_path.to_str().unwrap(), source_dir, files_to_backup)?;
    } else {
        println!("No changes detected. Skipping archive creation.");
    }

    // 7. Save the new manifest to the destination folder so it can act
    // as the "previous_manifest" for tomorrow's run.

    // current_manifest.save_to_file(&manifestr_ooutput_path)?;
    println!("Backup run complete. Manifest saved as {:?}", manifest_output_path);
    Ok(())


}

fn main() {
    println!("Hello, world!");
}
