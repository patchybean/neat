//! Integration tests for neatcli

#![allow(deprecated)] // cargo_bin is deprecated but still works fine for standard setups

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use tempfile::tempdir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("--help").assert().success();
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("neatcli"));
}

#[test]
fn test_organize_by_extension() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("test.txt");
    let file2 = dir.path().join("image.jpg");

    File::create(&file1).unwrap();
    File::create(&file2).unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("organize")
        .arg(dir.path())
        .arg("--execute")
        .assert()
        .success();

    assert!(!file1.exists());
    assert!(!file2.exists());
    assert!(dir.path().join("Documents").join("test.txt").exists());
    assert!(dir.path().join("Images").join("image.jpg").exists());
}

#[test]
fn test_organize_dry_run() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("test.txt");
    File::create(&file1).unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("organize")
        .arg(dir.path())
        .arg("--dry-run")
        .assert()
        .success();

    // File should still be in original place
    assert!(file1.exists());
    assert!(!dir.path().join("Documents").join("test.txt").exists());
}

#[test]
fn test_duplicates_detection() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("dup1.txt");
    let file2 = dir.path().join("dup2.txt");

    // Create duplicates with same content
    fs::write(&file1, "duplicate content").unwrap();
    fs::write(&file2, "duplicate content").unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("duplicates")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("dup1.txt").or(predicate::str::contains("dup2.txt")));
}

#[test]
fn test_duplicates_json_export() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("dup1.txt");
    let file2 = dir.path().join("dup2.txt");

    fs::write(&file1, "duplicate content").unwrap();
    fs::write(&file2, "duplicate content").unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("duplicates")
        .arg(dir.path())
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"hash\""));
}

#[test]
fn test_duplicates_csv_export() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("dup1.txt");
    let file2 = dir.path().join("dup2.txt");

    fs::write(&file1, "duplicate content").unwrap();
    fs::write(&file2, "duplicate content").unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("duplicates")
        .arg(dir.path())
        .arg("--csv")
        .assert()
        .success()
        .stdout(predicate::str::contains("group,hash,path,size"));
}

#[test]
fn test_stats_command() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("file.txt")).unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("stats")
        .arg(dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Files by Type"));
}

#[test]
fn test_stats_json_export() {
    let dir = tempdir().unwrap();
    File::create(dir.path().join("file.txt")).unwrap();

    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("stats")
        .arg(dir.path())
        .arg("--json")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"total_files\""));
}

#[test]
fn test_size_filter() {
    let dir = tempdir().unwrap();
    let small_file = dir.path().join("small.txt");
    let large_file = dir.path().join("large.bin");

    File::create(&small_file).unwrap();
    let f = File::create(&large_file).unwrap();
    f.set_len(1024 * 1024 * 2).unwrap(); // 2MB

    // Organize only small files (< 1MB)
    let mut cmd = Command::cargo_bin("neatcli").unwrap();
    cmd.arg("organize")
        .arg(dir.path())
        .arg("--execute")
        .arg("--max-size")
        .arg("1MB")
        .assert()
        .success();

    // Small file moved, large file stays
    assert!(dir.path().join("Documents").join("small.txt").exists());
    assert!(large_file.exists());
}
