/**************************************************************************/
/*                                 log.rs                                 */
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

use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

use flate2::read::ZlibDecoder;

// Terminal styling configurations
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GRAY: &str = "\x1b[90m";
const COLOR_RESET: &str = "\x1b[0m";

/// Resolves the current branch path and name from HEAD
fn get_active_branch_ref() -> Option<(PathBuf, String)> {
    let head_path = Path::new(".mgit/HEAD");
    if !head_path.exists() {
        return None;
    }
    let content = fs::read_to_string(head_path).ok()?;
    let content = content.trim();
    if content.starts_with("ref: ") {
        let ref_sub_path = &content[5..];
        let ref_path = Path::new(".mgit").join(ref_sub_path);
        let branch_name = ref_sub_path.rsplit('/').next()?.to_string();
        Some((ref_path, branch_name))
    } else {
        None
    }
}

/// Reads and decompresses a stored commit object
fn read_commit_object(hash: &str) -> Option<String> {
    let (dir, file) = hash.split_at(2);
    let file_path = format!(".mgit/objects/{}/{}", dir, file);
    let compressed_bytes = fs::read(&file_path).ok()?;
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).ok()?;

    let null_index = decompressed.iter().position(|&b| b == 0)?;
    String::from_utf8(decompressed[null_index + 1..].to_vec()).ok()
}

pub fn run() {
    let Some((ref_path, branch_name)) = get_active_branch_ref() else {
        eprintln!("Error: Cannot read HEAD ref.");
        return;
    };

    if !ref_path.exists() {
        println!("fatal: your current branch '{}' does not have any commits yet", branch_name);
        return;
    }

    let mut current_hash = match fs::read_to_string(&ref_path) {
        Ok(hash) => hash.trim().to_string(),
        Err(_) => {
            eprintln!("Error: Cannot read branch reference.");
            return;
        }
    };

    if current_hash.is_empty() {
        println!("fatal: your current branch '{}' does not have any commits yet", branch_name);
        return;
    }

    // Traverse the parent history back to the initial commit
    while !current_hash.is_empty() {
        let Some(commit_content) = read_commit_object(&current_hash) else {
            eprintln!("Error: Commit object {} is missing or corrupt.", current_hash);
            break;
        };

        let mut parent: Option<String> = None;
        let mut message = String::new();

        for line in commit_content.lines() {
            if line.starts_with("parent ") {
                parent = Some(line["parent ".len()..].trim().to_string());
            } else if line.starts_with("message ") {
                message = line["message ".len()..].trim().to_string();
            }
        }

        let short_hash = if current_hash.len() >= 7 { &current_hash[..7] } else { &current_hash };

        // Print timeline entry with color highlights
        println!("{}●{}  [{}{}{}] - {}", COLOR_YELLOW, COLOR_RESET, COLOR_CYAN, short_hash, COLOR_RESET, message);

        if let Some(ref p) = parent {
            let short_parent = if p.len() >= 7 { &p[..7] } else { p };
            println!("│  {}parent: {}{}", COLOR_GRAY, short_parent, COLOR_RESET);
            println!("│");
            current_hash = p.clone();
        } else {
            current_hash.clear(); // End of graph traversal
        }
    }
}
