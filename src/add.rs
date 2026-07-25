/**************************************************************************/
/*                                add.rs                                  */
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
use std::path::Path;
use std::io::{Read, Write};

use sha1_smol::Sha1;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

fn check_repo(){
    let verify = vec![
        Path::new(".mgit/"),
        // Path::new(".mgit/hooks/"),
        // Path::new(".mgit/info/"),
        // Path::new(".mgit/info/exclude"),
        Path::new(".mgit/objects"),
        // Path::new(".mgit/objects/info/"),
        // Path::new(".mgit/objects/pak/"),
        Path::new(".mgit/refs/"),
        Path::new(".mgit/refs/heads/"),
        Path::new(".mgit/refs/tags/"),
        // Path::new(".mgit/config"),
        // Path::new(".mgit/description"),
        Path::new(".mgit/HEAD"),
    ];

    for path in &verify {
        if !path.exists() {
            println!("Missing: {:?}",path);
        } else {
            if cfg!(debug_assertions) {
                println!("[OK] {:?}",path);
            }
        }
    }
}

fn read_file(file: &str) -> Option<Vec<u8>> {
    match fs::read(file) {
        Ok(data) => {
            if cfg!(debug_assertions) {
                println!("[DEBUG] Read {} bytes from {}", data.len(), file);
            }
            Some(data)
        },
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            None
        }
    }
}

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.digest().to_string()
}

/// Stores the blob in .mgit/objects/<hash[0..2]>/<hash[2..]>
fn store_blob(data: &[u8], hash: &str) -> std::io::Result<()> {
    let (dir, file) = hash.split_at(2);
    let dir = format!(".mgit/objects/{}", dir);
    let file = format!("{}/{}", dir, file);
    let _ = fs::create_dir_all(dir);

    let header = format!("blob {}\0", data.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(data);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&store)?;
    let compressed = encoder.finish()?;

    match fs::write(&file, compressed) {
        Ok(_) => if cfg!(debug_assertions) {
            println!("[DEBUG] Stored blob at {:?}", &file);
        },
        Err(e) => eprintln!("Failed to write blob file: {}", e),
    };

    Ok(())
}

/// Read the stored blob
fn read_blob(hash: &str) -> Option<Vec<u8>> {
    let (dir, file) = hash.split_at(2);
    let file_path = format!(".mgit/objects/{}/{}", dir, file);

    let compressed_bytes = fs::read(&file_path).ok()?;

    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).ok()?;

    let null_index = decompressed.iter().position(|&b| b == 0)?;
    Some(decompressed[null_index + 1..].to_vec())
}

pub fn run(file: String){
    check_repo();

    // Normalize path by stripping leading "./" to prevent duplicate index entries
    let mut normalized_file = file;
    if normalized_file.starts_with("./") {
        normalized_file = normalized_file[2..].to_string();
    }

    if let Some(data) = read_file(&normalized_file) {
        let hash = hash_bytes(&data);
        if cfg!(debug_assertions) {
            println!("[DEBUG] SHA-1 Hash: {}", hash);
        }

        if let Err(e) = store_blob(&data, &hash) {
            eprintln!("Failed to store blob: {}", e);
        }

        // Call the localized index updater
        update_index(&hash, &normalized_file);

        let blob = read_blob(&hash);
        if cfg!(debug_assertions) {
            match blob {
                Some(ref bytes) => {
                    match String::from_utf8_lossy(bytes) {
                        s if s.is_empty() => println!("[DEBUG] blob content: (empty)"),
                        s => println!("[DEBUG] blob content: {}", s),
                    }
                },
                None => println!("[DEBUG] blob content: None (file not found or invalid)"),
            }
        }
    }
}

pub fn update_index(hash: &str, file: &str) {
    let index_path = ".mgit/index";

    let mut normalized_file = file.to_string();
    if normalized_file.starts_with("./") {
        normalized_file = normalized_file[2..].to_string();
    }

    let new_entry = format!("{} | {}", hash, normalized_file);
    let mut entries: Vec<String> = Vec::new();

    if Path::new(index_path).exists() {
        if let Ok(contents) = fs::read_to_string(index_path) {
            entries = contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            // Normalize and compare both entries to discard the old matching file entry
            .filter(|line| {
                let parts: Vec<&str> = line.split(" | ").collect();
                if parts.len() == 2 {
                    let mut existing_file = parts[1].to_string();
                    if existing_file.starts_with("./") {
                        existing_file = existing_file[2..].to_string();
                    }
                    existing_file != normalized_file
                } else {
                    true
                }
            })
            .collect();
        }
    }

    entries.insert(0, new_entry);

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
