use std::fs;
use std::path::Path;
use std::io::{Read, Write};

use sha1_smol::Sha1;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

use crate::index;

fn check_repo(){
    let verify = vec![
        Path::new(".mgit/"),
        Path::new(".mgit/hooks/"),
        Path::new(".mgit/info/"),
        Path::new(".mgit/info/exclude"),
        Path::new(".mgit/objects"),
        Path::new(".mgit/objects/info/"),
        Path::new(".mgit/objects/pak/"),
        Path::new(".mgit/refs/"),
        Path::new(".mgit/refs/heads/"),
        Path::new(".mgit/refs/tags/"),
        Path::new(".mgit/config"),
        Path::new(".mgit/description"),
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
fn store_blob(data: &[u8],hash:&str) -> std::io::Result<()>{
    let (dir, file) = hash.split_at(2);
    let dir = format!(".mgit/objects/{}", dir);
    let file = format!("{}/{}", dir,file);
    let _ = fs::create_dir_all(dir);

    // convert the data to blob
    let header = format!("blob {}\0",data.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(data);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&store)?;
    let compressed = encoder.finish()?;

    match fs::write(&file,compressed){
        Ok(_) => if cfg!(debug_assertions) {
                    println!("[DEBUG] Stored blob at {:?}", &file);
                },
        Err(e) => eprintln!("Failed to read file: {}", e),
    };

    Ok(())
}

/// Read the stored blob
fn read_blob(hash: &str) -> Option<Vec<u8>> {
    let (dir, file) = hash.split_at(2);
    let file_path = format!(".mgit/objects/{}/{}", dir, file);

    // Read compressed bytes from disk
    let compressed_bytes = fs::read(&file_path).ok()?;

    // DECOMPRESS first
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).ok()?;

    // find the null byte in the DECOMPRESSED data
    let null_index = decompressed.iter().position(|&b| b == 0)?;

    // Return content after the null byte
    Some(decompressed[null_index + 1..].to_vec())
}

pub fn run(file: String){
    check_repo();

    if let Some(data) = read_file(&file) {
        let hash = hash_bytes(&data);
        if cfg!(debug_assertions) {
            println!("[DEBUG] SHA-1 Hash: {}", hash);
        }

        // Store the blob in the objects directory
        if let Err(e) = store_blob(&data, &hash) {
            eprintln!("Failed to store blob: {}", e);
        }

        // index::create_index();
        index::update_index(&hash, &file);

        // test blob read
        let blob = read_blob(&hash);
        if cfg!(debug_assertions) {
            match blob {
                Some(ref bytes) => {
                    // Try to print as text (if it's a text file)
                    match String::from_utf8_lossy(bytes) {
                        s if s.is_empty() => println!("[DEBUG] blob content: (empty)"),
                        s => println!("[DEBUG] blob content: {}", s),
                    }

                    // If it might be binary, print hex instead
                    // println!("[DEBUG] blob hex: {:x?}", bytes);
                },
                None => println!("[DEBUG] blob content: None (file not found or invalid)"),
            }
        }
    }
}
