/**************************************************************************/
/*                                index.rs                                */
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

/// make the index key-value store
///  ||
///  \/
/// file mode
/// timestamps
/// permissions
/// SHA-1
/// filename
/// cache entries
// use std::fs::{self, File, OpenOptions};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// pub fn create_index() {
//     match File::create(".mgit/index") {
//         Ok(_) => {
//             if cfg!(debug_assertions) {
//                 println!("[DEBUG] index created...");
//             }
//         }
//         Err(error) => {
//             eprintln!("Could not create index. Details: {}", error);
//             std::process::exit(1);
//         }
//     };
// }

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
