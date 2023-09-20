extern crate pathdiff;
extern crate walkdir;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::path::Path;
use std::collections::HashSet;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    #[serde(rename = "rust-analyzer.linkedProjects")]
    linked_projects: Vec<String>,
}

fn main() {
    // Define the root directory and settings file paths
    let root_directory = "..";
    let settings_path = "../.vscode/settings.json";

    // Search for Cargo.toml files
    let mut toml_files = HashSet::new();
    for entry in WalkDir::new(root_directory) {
        let entry = entry.unwrap();
        if entry.file_name() == "Cargo.toml" {
            let relative_path = pathdiff::diff_paths(entry.path(), Path::new(settings_path).parent().unwrap())
                .expect("Failed to compute relative path")
                .to_string_lossy()
                .to_string();
            
            // Adjust the path to be relative to the settings.json directory
            let adjusted_path = format!(".{}", relative_path.trim_start_matches(".."));

            toml_files.insert(adjusted_path);
        }
    }
    

    // Load the settings.json file
    let settings_data = fs::read_to_string(settings_path).expect("Unable to read settings.json");
    let mut settings: Settings = serde_json::from_str(&settings_data).expect("Error parsing settings.json");

    // Convert HashSet to Vec and update the rust-analyzer.linkedProjects field
    let unique_toml_files: Vec<_> = toml_files.into_iter().collect();
    for toml_file in unique_toml_files {
        if !settings.linked_projects.contains(&toml_file){
            settings.linked_projects.push(toml_file);
        }
    }
    

    // Write the updated settings back to settings.json
    let updated_settings = serde_json::to_string_pretty(&settings).expect("Error serializing settings");
    fs::write(settings_path, updated_settings).expect("Unable to write to settings.json");
}
