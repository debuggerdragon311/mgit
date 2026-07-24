use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::collections::BTreeSet;
use sha1_smol::Sha1;

// Configuration
const INDEX_PATH: &str = ".mgit/index";
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[31m";   // Untracked
// const COLOR_YELLOW: &str = "\x1b[33m"; // Modified (ready for future use)

// Helpers

fn hash_file(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.digest().to_string()
}

fn find_hash(index_path: &str, target_file: &str) -> Option<String> {
    let file = File::open(index_path).ok()?;
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(" | ").collect();
            if parts.len() == 2 && parts[1] == target_file {
                return Some(parts[0].to_string());
            }
        }
    }
    None
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            // Skip .mgit directory
            if path.file_name().map_or(false, |n| n == ".mgit") {
                continue;
            }
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}


pub fn run() {
    let mut files_to_check = Vec::new();

    // 1. Collect all files
    if collect_files(Path::new("."), &mut files_to_check).is_err() {
        eprintln!("Error traversing directory");
        return;
    }

    let mut untracked_items = BTreeSet::new();
    let mut has_untracked = false;

    for file_path in files_to_check {
        let Ok(data) = fs::read(&file_path) else { continue };

        let current_hash = hash_file(&data);
        let mut path_str = file_path.to_string_lossy().to_string();

        // Normalize: Strip leading "./" to match index format
        if path_str.starts_with("./") {
            path_str = path_str[2..].to_string();
        }

        // 2. Check Status
        if let Some(stored_hash) = find_hash(INDEX_PATH, &path_str) {
            if stored_hash != current_hash {
                // TODO: Add logic here to track "Modified" files with COLOR_YELLOW
            }
        } else {
            // 3. Handle Untracked
            has_untracked = true;

            // Determine display name (group by directory or show file)
            let display_name = if let Some(parent) = file_path.parent() {
                let p = parent.to_string_lossy();
                let norm_p = p.strip_prefix("./").unwrap_or(&p);

                if norm_p != "." {
                    // It's inside a folder: show "folder/"
                    format!("{}/", norm_p)
                } else {
                    // It's in root: show "filename"
                    path_str.clone()
                }
            } else {
                path_str.clone()
            };

            // Apply Color and Add to Set
            untracked_items.insert(format!("{}{}{}", COLOR_RED, display_name, COLOR_RESET));
        }
    }

    // 4. Output
    if has_untracked {
        println!("Untracked files:");
        println!("  (use \"mgit add <file>...\" to include in what will be committed)");
        for item in untracked_items {
            println!("        {}", item);
        }
        println!("\nnothing added to commit but untracked files present (use \"mgit add\" to track)");
    } else {
        println!("On branch main\n\nnothing to commit, working tree clean");
    }
}
