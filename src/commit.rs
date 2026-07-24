/**************************************************************************/
/*                                commit.rs                               */
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

/// mgit commit "initial commit"
/// read index <hash> | <file name>
/// create commit object
/// ||
/// \/
/// Message: "initial commit"
/// Files:
///    <hash> | <file name>
///
/// hash it and encode compress it as before
/// store as same as blobs
/// update and make a branch  .mgit/refs/heads/master --> <hash of the commit object>
/// update the HEAD to ref: refs/heads/master
/// later commits only update the hash stored in ref/heads/master nothing else

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Read, Write};

use sha1_smol::Sha1;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

const INDEX_PATH: &str = ".mgit/index";

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.digest().to_string()
}

fn get_active_branch_ref() -> (PathBuf, String) {
    let head_path = Path::new(".mgit/HEAD");
    let mut ref_path = PathBuf::from(".mgit/refs/heads/master");
    let mut branch_name = "master".to_string();

    if let Ok(content) = fs::read_to_string(head_path) {
        let content = content.trim();
        if content.starts_with("ref: ") {
            let ref_sub_path = &content[5..];
            ref_path = Path::new(".mgit").join(ref_sub_path);
            if let Some(name) = ref_sub_path.rsplit('/').next() {
                branch_name = name.to_string();
            }
        }
    }
    (ref_path, branch_name)
}

fn store_commit_object(data: &[u8], hash: &str) -> std::io::Result<()> {
    let (dir, file) = hash.split_at(2);
    let dir_path = format!(".mgit/objects/{}", dir);
    let file_path = format!("{}/{}", dir_path, file);
    fs::create_dir_all(&dir_path)?;

    let header = format!("commit {}\0", data.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(data);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&store)?;
    let compressed = encoder.finish()?;

    fs::write(&file_path, compressed)?;
    Ok(())
}

fn read_commit_object(hash: &str) -> Option<Vec<u8>> {
    let (dir, file) = hash.split_at(2);
    let file_path = format!(".mgit/objects/{}/{}", dir, file);

    let compressed_bytes = fs::read(&file_path).ok()?;
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).ok()?;

    let null_index = decompressed.iter().position(|&b| b == 0)?;
    Some(decompressed[null_index + 1..].to_vec())
}

pub fn run(message: String) {
    let index_path = Path::new(INDEX_PATH);
    if !index_path.exists() {
        eprintln!("Error: Nothing to commit (index file missing).");
        return;
    }

    let index_contents = match fs::read_to_string(index_path) {
        Ok(content) if !content.trim().is_empty() => content,
        _ => {
            eprintln!("Error: Nothing to commit (index is empty).");
            return;
        }
    };

    let (ref_path, branch_name) = get_active_branch_ref();

    let parent_hash = if ref_path.exists() {
        fs::read_to_string(&ref_path)
        .map(|s| s.trim().to_string())
        .ok()
        .filter(|s| !s.is_empty())
    } else {
        None
    };

    let mut commit_content = String::new();
    if let Some(ref parent) = parent_hash {
        commit_content.push_str(&format!("parent {}\n", parent));
    }
    commit_content.push_str(&format!("message {}\n", message));
    commit_content.push_str("files:\n");
    for line in index_contents.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            commit_content.push_str(&format!("  {}\n", trimmed));
        }
    }

    let commit_bytes = commit_content.as_bytes();
    let commit_hash = hash_bytes(commit_bytes);

    if let Err(e) = store_commit_object(commit_bytes, &commit_hash) {
        eprintln!("Error: Failed to store commit object: {}", e);
        return;
    }

    if cfg!(debug_assertions) {
        println!("[DEBUG] SHA-1 Hash: {}", commit_hash);
        match read_commit_object(&commit_hash) {
            Some(ref bytes) => {
                println!("[DEBUG] Commit Object Content:");
                println!("{}", String::from_utf8_lossy(bytes));
            }
            None => println!("[DEBUG] Commit Content: None (file not found or invalid)"),
        }
    }

    if let Some(parent_dir) = ref_path.parent() {
        let _ = fs::create_dir_all(parent_dir);
    }
    if let Err(e) = fs::write(&ref_path, format!("{}\n", commit_hash)) {
        eprintln!("Error: Failed to update branch reference: {}", e);
        return;
    }

    let head_path = Path::new(".mgit/HEAD");
    if let Err(e) = fs::write(head_path, format!("ref: refs/heads/{}\n", branch_name)) {
        eprintln!("Error: Failed to update HEAD: {}", e);
        return;
    }

    let short_hash = &commit_hash[..7];
    println!("[{} {}] {}", branch_name, short_hash, message);
}
