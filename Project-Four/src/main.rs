// Import the file system module for reading and writing files.
use std::fs;
// Import Path and PathBuf types from the standard library for handling file and directory paths.
use std::path::{PathBuf};
// Import HashSet from the standard library, a collection type that ensures uniqueness of its elements.
use std::collections::HashSet;
// Import the WalkDir type from the walkdir crate, which allows efficient traversal of directory trees.
use walkdir::WalkDir;
// Import Serialize and Deserialize traits from serde, which are needed for (de)serializing our custom data structures.
use serde::{Serialize, Deserialize};
// Import the io module for handling I/O related errors and results.
use std::io;

// Define a custom data structure to map the structure of the settings.json file.
#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    // This attribute renames the Rust field to match the JSON property name when (de)serializing.
    #[serde(rename = "rust-analyzer.linkedProjects")]
    linked_projects: Vec<String>,  // A vector to store paths to the Cargo.toml files.
}

// The main function, which is the entry point of our program.
fn main() -> io::Result<()> {

    // Define Variables to be used later
    let root_name: &str = "/workspaces/rust/";
    let settings_name: &str = ".vscode/settings.json";
    let target_file_name: &str = "Cargo.toml";

    // Define the root directory from which the search will start.
    let root_directory = PathBuf::from(&root_name);

    // Define the path to the settings.json file we'll be updating.
    let settings_path: PathBuf = root_directory.join(&settings_name);

    // Initialize an empty HashSet to store unique paths to discovered Cargo.toml files.
    let mut discovered_toml_files = HashSet::new();

    // Iterate over each entry in the directory tree starting from the root_directory.
    for entry in WalkDir::new(&root_directory) {
        // Unwrap the Result; if there's an error, it will be propagated up via the `?` operator.
        let entry = entry?;

        // Check if the current entry corresponds to a file named "Cargo.toml".
        if entry.file_name() == target_file_name {
            // Compute the relative path from the current entry to the directory containing settings.json.
            let relative_path = pathdiff::diff_paths(entry.path(), settings_path.parent().unwrap())
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to compute relative path"))?
                .display()
                .to_string();

            println!("{:?}", relative_path);
            // Adjust the computed path to be relative to the settings.json directory.
            let adjusted_path = format!(".{}", relative_path.trim_start_matches(".."));

            // Insert the adjusted path into the HashSet, ensuring uniqueness.
            discovered_toml_files.insert(adjusted_path);
        }
    }

    // Read the current contents of the settings.json file into a String.
    let settings_data = fs::read_to_string(&settings_path)?;

    // Deserialize the JSON string into our custom Settings struct.
    let mut settings: Settings = serde_json::from_str(&settings_data)?;

    // Iterate over each discovered path.
    for toml_file in &discovered_toml_files {
        // Check if the path already exists in the current settings. If not, add it.
        if !settings.linked_projects.contains(toml_file) {
            settings.linked_projects.push(toml_file.clone());
        }
    }

    // Serialize our updated Settings struct back into a JSON formatted string.
    let updated_settings = serde_json::to_string_pretty(&settings)?;

    // Write the updated JSON string back to the settings.json file.
    fs::write(settings_path, updated_settings)?;

    // Signal successful execution.
    Ok(())
}
