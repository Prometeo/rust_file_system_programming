use std::fs;
use std::path::PathBuf;
// We import ProjectDirs from the directories crate.
use directories::ProjectDirs;
fn locate_global_configuration() -> Option<PathBuf> {
    // We instantiate ProjectDirs.
    // It requires three strings: a qualifier, an organization name, and the application name.
    // For open-source tools, leaving the first two blank is standard practice.
    if let Some(proj_dirs) = ProjectDirs::from("", "", "MyAwesomeUtility") {
        // The config_dir() method returns the exact, platform-correct path.
        let config_folder = proj_dirs.config_dir();
        println!("The correct configuration folder is: {:?}", config_folder);
        // Crucial Step: The folder might not exist yet if this is the first time
        // the user has run the application. We must ensure it exists.
        // We use create_dir_all from Chapter 5 to build it safely.
        if let Err(e) = fs::create_dir_all(config_folder) {
            eprintln!("Failed to create global configuration directory: {}", e);
            // If we cannot create the directory, we cannot load or save settings.
            return None;
        }

        // We combine the folder path with our specific file name.
        let mut final_config_file = PathBuf::from(config_folder);
        final_config_file.push("settings.toml");

        // We return the full, platform-perfect path.
        return Some(final_config_file);
    }

    // If ProjectDirs fails (which is rare, but possible on deeply customized systems),
    // we return None.
    None
}

fn main() {
    println!("Hello, world!");
}
