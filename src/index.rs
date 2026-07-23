/// make the index key-value store
///  ||
///  \/
/// file mode
/// timestamps
/// permissions
/// SHA-1
/// filename
/// cache entries
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn create_index() {
    match File::create(".mgit/index") {
        Ok(_) => {
            if cfg!(debug_assertions) {
                println!("[DEBUG] index created...");
            }
        }
        Err(error) => {
            eprintln!("Could not create index. Details: {}", error);
            std::process::exit(1);
        }
    };
}

pub fn update_index(hash: &str, file: &str) {
    let index_path = ".mgit/index";
    let new_entry = format!("{} | {}", hash, file);
    let mut entries: Vec<String> = Vec::new();

    // Read existing entries and filter out any previous reference to this specific file
    if Path::new(index_path).exists() {
        if let Ok(contents) = fs::read_to_string(index_path) {
            entries = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            // Prevent duplicate files by removing lines that end with the same filename
            .filter(|line| !line.ends_with(&format!(" | {}", file)))
            .collect();
        }
    }

    // Insert the brand new or updated entry at the very top (index 0)
    entries.insert(0, new_entry);

    // Rewrite the index file completely with the new ordered list
    match File::create(index_path) {
        Ok(mut f) => {
            for entry in entries {
                if let Err(e) = writeln!(f, "{}", entry) {
                    eprintln!("Failed to write entry: {}", e);
                }
            }
            if cfg!(debug_assertions) {
                println!("[DEBUG] Index successfully updated.");
            }
        }
        Err(e) => eprintln!("Failed to open index file: {}", e),
    }
}
