/**************************************************************************/
/*                                status.rs                               */
/**************************************************************************/
/*                         This file is part of:                          */
/*                                  mgit                                  */
/**************************************************************************/
/*                  Copyright (C) 2026  Soumyajit Bala                    */
/*                                                                        */
/* This program is free software: you can redistribute it and/or modify   */
/* it under the terms of the GNU Affero General Public License as         */
/* published by the Free Software Foundation, either version 3 of the     */
/* License, or (at your option) any later version.                        */
/*                                                                        */
/* This program is distributed in the hope that it will be useful,        */
/* but WITHOUT ANY WARRANTY; without even the implied warranty of         */
/* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          */
/* GNU Affero General Public License for more details.                    */
/*                                                                        */
/* You should have received a copy of the GNU Affero General Public       */
/* License along with this program.  If not, see                          */
/* <https://www.gnu.org/licenses/>.                                       */
/**************************************************************************/

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::collections::BTreeSet;

use sha1_smol::Sha1;

// Configuration
const INDEX_PATH: &str = ".mgit/index";
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_YELLOW: &str = "\x1b[33m";

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

    if collect_files(Path::new("."), &mut files_to_check).is_err() {
        eprintln!("Error traversing directory");
        return;
    }

    let mut untracked_items = BTreeSet::new();
    let mut modified_items = BTreeSet::new();
    let mut has_untracked = false;
    let mut has_modified = false;

    for file_path in files_to_check {
        let Ok(data) = fs::read(&file_path) else { continue };

        let current_hash = hash_file(&data);
        let mut path_str = file_path.to_string_lossy().to_string();

        if path_str.starts_with("./") {
            path_str = path_str[2..].to_string();
        }

        // Compare against the index
        if let Some(stored_hash) = find_hash(INDEX_PATH, &path_str) {
            if stored_hash != current_hash {
                has_modified = true;
                modified_items.insert(format!("{}modified:   {}{}", COLOR_YELLOW, path_str, COLOR_RESET));
            }
        } else {
            has_untracked = true;

            // Simplify directory paths to match git-like behavior
            let display_name = if let Some(parent) = file_path.parent() {
                let p = parent.to_string_lossy();
                let norm_p = p.strip_prefix("./").unwrap_or(&p);

                if norm_p != "." {
                    format!("{}/", norm_p)
                } else {
                    path_str.clone()
                }
            } else {
                path_str.clone()
            };

            untracked_items.insert(format!("{}{}{}", COLOR_RED, display_name, COLOR_RESET));
        }
    }

    // Display modified files
    if has_modified {
        println!("Changes not staged for commit:");
        println!("  (use \"mgit add <file>...\" to update what will be committed)");
        for item in &modified_items {
            println!("        {}", item);
        }
        println!();
    }

    // Display untracked files
    if has_untracked {
        println!("Untracked files:");
        println!("  (use \"mgit add <file>...\" to include in what will be committed)");
        for item in &untracked_items {
            println!("        {}", item);
        }
        println!();
    }

    // Status summary footer
    if has_untracked || has_modified {
        if has_modified && !has_untracked {
            println!("no changes added to commit (use \"mgit add\" to stage)");
        } else if has_untracked && !has_modified {
            println!("nothing added to commit but untracked files present (use \"mgit add\" to track)");
        } else {
            println!("no changes added to commit (use \"mgit add\" to stage)");
        }
    } else {
        println!("On branch main\n\nnothing to commit, working tree clean");
    }
}
