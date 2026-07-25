use std::fs;
use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::TempDir;

use flate2::read::ZlibDecoder;
use sha1_smol::Sha1;
use std::io::Read;

fn mgit_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("mgit").expect("mgit binary not found");
    cmd.current_dir(temp_dir.path());
    cmd
}

fn count_stored_objects(objects_dir: &std::path::Path) -> std::io::Result<usize> {
    let mut count = 0;
    if objects_dir.is_dir() {
        for entry in fs::read_dir(objects_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                for sub_entry in fs::read_dir(path)? {
                    if sub_entry?.path().is_file() {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

fn decompress_blob(objects_dir: &std::path::Path, hash: &str) -> std::io::Result<Vec<u8>> {
    let (dir, file) = hash.split_at(2);
    let blob_path = objects_dir.join(dir).join(file);

    let compressed_bytes = fs::read(blob_path)?;
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

fn calculate_raw_sha1(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.digest().to_string()
}

#[test]
fn test_complex_churn_and_nested_structures() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let dot_mgit = temp_dir.path().join(".mgit");
    let objects_dir = dot_mgit.join("objects");

    let src_utils_dir = temp_dir.path().join("src/utils");
    let tests_dir = temp_dir.path().join("tests");
    let assets_dir = temp_dir.path().join("assets");
    fs::create_dir_all(&src_utils_dir)?;
    fs::create_dir_all(&tests_dir)?;
    fs::create_dir_all(&assets_dir)?;

    let readme_path = temp_dir.path().join("README.md");
    let main_rs_path = temp_dir.path().join("src/main.rs");
    let math_rs_path = src_utils_dir.join("math.rs");
    let test_rs_path = tests_dir.join("common.rs");
    let logo_png_path = assets_dir.join("logo.png");

    let readme_content_v1 = "Project Draft v1";
    let main_rs_content = "fn main() { println!(\"mgit core\"); }";
    let math_rs_content = "pub fn add(a: i32, b: i32) -> i32 { a + b }";
    let test_rs_content = "use common_test_helpers;";
    let logo_png_content: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82];

    fs::write(&readme_path, readme_content_v1)?;
    fs::write(&main_rs_path, main_rs_content)?;
    fs::write(&math_rs_path, math_rs_content)?;
    fs::write(&test_rs_path, test_rs_content)?;
    fs::write(&logo_png_path, &logo_png_content)?;

    mgit_cmd(&temp_dir).args(&["add", "README.md"]).assert().success();
    mgit_cmd(&temp_dir).args(&["add", "src/main.rs"]).assert().success();
    mgit_cmd(&temp_dir).args(&["commit", "-m", "commit 1: core structure"]).assert().success();

    // 2 file blobs + 1 commit object
    assert_eq!(count_stored_objects(&objects_dir)?, 3);

    let readme_content_v2 = "Project Draft v2 - Updated and Expanded with more info.";
    fs::write(&readme_path, readme_content_v2)?;
    mgit_cmd(&temp_dir).args(&["add", "README.md"]).assert().success();

    // 3 unique file blobs + 1 commit object
    assert_eq!(count_stored_objects(&objects_dir)?, 4);

    mgit_cmd(&temp_dir).args(&["add", "src/utils/math.rs"]).assert().success();
    mgit_cmd(&temp_dir).args(&["add", "tests/common.rs"]).assert().success();
    mgit_cmd(&temp_dir).args(&["add", "assets/logo.png"]).assert().success();
    mgit_cmd(&temp_dir).args(&["commit", "-m", "commit 2: added math, tests, and logo asset"]).assert().success();

    // 6 unique file blobs + 2 commit objects
    assert_eq!(count_stored_objects(&objects_dir)?, 8);

    let test_matrix = vec![
        ("README.md", readme_content_v2.as_bytes()),
        ("src/main.rs", main_rs_content.as_bytes()),
        ("src/utils/math.rs", math_rs_content.as_bytes()),
        ("tests/common.rs", test_rs_content.as_bytes()),
        ("assets/logo.png", logo_png_content.as_slice()),
    ];

    for (relative_path, expected_bytes) in test_matrix {
        let hash = calculate_raw_sha1(expected_bytes);
        let decompressed = decompress_blob(&objects_dir, &hash)?;

        let null_index = decompressed.iter().position(|&b| b == 0).expect("Null terminator missing");
        let header = std::str::from_utf8(&decompressed[..null_index])?;
        let data = &decompressed[null_index + 1..];

        assert_eq!(header, format!("blob {}", expected_bytes.len()), "Mismatch in header of {}", relative_path);
        assert_eq!(data, expected_bytes, "Mismatch in content data of {}", relative_path);
    }

    mgit_cmd(&temp_dir)
    .arg("log")
    .assert()
    .success()
    .stdout(predicate::str::contains("commit 2: added math, tests, and logo asset"))
    .stdout(predicate::str::contains("parent:"))
    .stdout(predicate::str::contains("commit 1: core structure"));

    Ok(())
}

#[test]
fn test_help_and_version() {
    let temp_dir = TempDir::new().unwrap();

    mgit_cmd(&temp_dir)
    .arg("--help")
    .assert()
    .success()
    .stdout(predicate::str::contains("USAGE:"))
    .stdout(predicate::str::contains("init"));

    mgit_cmd(&temp_dir)
    .arg("--version")
    .assert()
    .success()
    .stdout(predicate::str::contains("mgit: v"));
}

#[test]
fn test_invalid_command() {
    let temp_dir = TempDir::new().unwrap();
    mgit_cmd(&temp_dir)
    .arg("unknowncommand")
    .assert()
    .success()
    .stderr(predicate::str::contains("Error: Unknown command 'unknowncommand'"));
}

#[test]
fn test_add_missing_file_argument() {
    let temp_dir = TempDir::new().unwrap();
    mgit_cmd(&temp_dir)
    .arg("add")
    .assert()
    .success()
    .stderr(predicate::str::contains("Error: 'mgit add' requires a file path."));
}

#[test]
fn test_commit_missing_message_argument() {
    let temp_dir = TempDir::new().unwrap();

    mgit_cmd(&temp_dir)
    .arg("commit")
    .assert()
    .success()
    .stderr(predicate::str::contains("Error: 'mgit commit' requires a message."));

    mgit_cmd(&temp_dir)
    .args(&["commit", "-m"])
    .assert()
    .success()
    .stderr(predicate::str::contains("Error: Please provide a message after -m."));
}

#[test]
fn test_full_repository_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;

    mgit_cmd(&temp_dir).arg("init").assert().success();

    let dot_mgit = temp_dir.path().join(".mgit");
    assert!(dot_mgit.is_dir());
    assert!(dot_mgit.join("objects").is_dir());
    assert!(dot_mgit.join("refs/heads").is_dir());
    assert!(dot_mgit.join("HEAD").is_file());

    let main_c_path = temp_dir.path().join("main.c");
    let test_txt_path = temp_dir.path().join("test.txt");
    fs::write(&main_c_path, "int main(){\n    return 0;\n}\n")?;
    fs::write(&test_txt_path, "hello world")?;

    mgit_cmd(&temp_dir)
    .arg("status")
    .assert()
    .success()
    .stdout(predicate::str::contains("Untracked files:"));

    let objects_dir = dot_mgit.join("objects");
    assert_eq!(count_stored_objects(&objects_dir)?, 0);

    mgit_cmd(&temp_dir).args(&["add", "main.c"]).assert().success();
    assert_eq!(count_stored_objects(&objects_dir)?, 1);

    mgit_cmd(&temp_dir).args(&["commit", "-m", "initial commit"]).assert().success();

    mgit_cmd(&temp_dir)
    .arg("status")
    .assert()
    .success()
    .stdout(predicate::str::contains("main.c").not());

    mgit_cmd(&temp_dir)
    .arg("log")
    .assert()
    .success()
    .stdout(predicate::str::contains("initial commit"));

    Ok(())
}

#[test]
fn test_file_modification_flow() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let main_c_path = temp_dir.path().join("main.c");
    fs::write(&main_c_path, "init main(){\n    return 0;\n}\n")?;
    mgit_cmd(&temp_dir).args(&["add", "main.c"]).assert().success();
    mgit_cmd(&temp_dir).args(&["commit", "-m", "initial draft"]).assert().success();

    fs::write(&main_c_path, "int main(){\n    return 0;\n}\n")?;

    mgit_cmd(&temp_dir)
    .arg("status")
    .assert()
    .success()
    .stdout(predicate::str::contains("Changes not staged for commit:"));

    Ok(())
}

#[test]
fn test_subdirectories_and_multi_commit_history() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let main_c_path = temp_dir.path().join("main.c");
    fs::write(&main_c_path, "int main() { return 0; }")?;
    mgit_cmd(&temp_dir).args(&["add", "main.c"]).assert().success();
    mgit_cmd(&temp_dir).args(&["commit", "-m", "add main.c"]).assert().success();

    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir)?;
    let server_c_path = src_dir.join("server.c");
    fs::write(&server_c_path, "hi")?;

    mgit_cmd(&temp_dir).args(&["add", "src/server.c"]).assert().success();
    mgit_cmd(&temp_dir).args(&["commit", "-m", "add server.c"]).assert().success();

    mgit_cmd(&temp_dir)
    .arg("log")
    .assert()
    .success()
    .stdout(predicate::str::contains("add server.c"))
    .stdout(predicate::str::contains("parent:"))
    .stdout(predicate::str::contains("add main.c"));

    Ok(())
}

#[test]
fn test_blob_text_integrity() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let file_path = temp_dir.path().join("sample.txt");
    let content = "Hello from the mgit test suite!";
    fs::write(&file_path, content)?;

    let hash = calculate_raw_sha1(content.as_bytes());
    mgit_cmd(&temp_dir).args(&["add", "sample.txt"]).assert().success();

    let objects_dir = temp_dir.path().join(".mgit/objects");
    let decompressed = decompress_blob(&objects_dir, &hash)?;

    let null_index = decompressed.iter().position(|&b| b == 0).unwrap();
    let header = std::str::from_utf8(&decompressed[..null_index])?;
    let data = &decompressed[null_index + 1..];

    assert_eq!(header, format!("blob {}", content.len()));
    assert_eq!(data, content.as_bytes());

    Ok(())
}

#[test]
fn test_blob_binary_integrity() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let file_path = temp_dir.path().join("binary.bin");
    let content: Vec<u8> = vec![0, 1, 2, 0, 255, 128, 0, 64];
    fs::write(&file_path, &content)?;

    let hash = calculate_raw_sha1(&content);
    mgit_cmd(&temp_dir).args(&["add", "binary.bin"]).assert().success();

    let objects_dir = temp_dir.path().join(".mgit/objects");
    let decompressed = decompress_blob(&objects_dir, &hash)?;

    let null_index = decompressed.iter().position(|&b| b == 0).unwrap();
    let header = std::str::from_utf8(&decompressed[..null_index])?;
    let data = &decompressed[null_index + 1..];

    assert_eq!(header, format!("blob {}", content.len()));
    assert_eq!(data, content);

    Ok(())
}

#[test]
fn test_blob_empty_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    mgit_cmd(&temp_dir).arg("init").assert().success();

    let file_path = temp_dir.path().join("empty.txt");
    fs::write(&file_path, "")?;

    let hash = calculate_raw_sha1(&[]);
    mgit_cmd(&temp_dir).args(&["add", "empty.txt"]).assert().success();

    let objects_dir = temp_dir.path().join(".mgit/objects");
    let decompressed = decompress_blob(&objects_dir, &hash)?;

    let null_index = decompressed.iter().position(|&b| b == 0).unwrap();
    let header = std::str::from_utf8(&decompressed[..null_index])?;
    let data = &decompressed[null_index + 1..];

    assert_eq!(header, "blob 0");
    assert!(data.is_empty());

    Ok(())
}
