//! Duplicate detection using SHA256 content hashing

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};

use crate::scanner::{format_size, FileInfo};

/// A group of duplicate files
#[derive(Debug)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<FileInfo>,
    pub size: u64,
}

impl DuplicateGroup {
    /// Get the wasted space (all but one file)
    pub fn wasted_space(&self) -> u64 {
        if self.files.len() > 1 {
            self.size * (self.files.len() as u64 - 1)
        } else {
            0
        }
    }
}

/// Find duplicate files by content
pub fn find_duplicates(files: &[FileInfo]) -> Result<Vec<DuplicateGroup>> {
    if files.is_empty() {
        return Ok(Vec::new());
    }

    // Step 1: Group by size (files with different sizes can't be duplicates)
    let mut by_size: HashMap<u64, Vec<&FileInfo>> = HashMap::new();
    for file in files {
        if file.size > 0 {
            // Skip empty files
            by_size.entry(file.size).or_default().push(file);
        }
    }

    // Filter to only groups with potential duplicates
    let potential_dups: Vec<_> = by_size
        .into_values()
        .filter(|group| group.len() > 1)
        .collect();

    if potential_dups.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2: Hash files with same size
    let total_files: usize = potential_dups.iter().map(|g| g.len()).sum();
    
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Hashing files [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▓░"),
    );

    let mut by_hash: HashMap<String, Vec<FileInfo>> = HashMap::new();

    for group in potential_dups {
        for file in group {
            pb.inc(1);
            if let Ok(hash) = hash_file(&file.path) {
                by_hash.entry(hash).or_default().push(file.clone());
            }
        }
    }

    pb.finish_and_clear();

    // Step 3: Build duplicate groups
    let duplicates: Vec<DuplicateGroup> = by_hash
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| {
            let size = files.first().map(|f| f.size).unwrap_or(0);
            DuplicateGroup { hash, files, size }
        })
        .collect();

    Ok(duplicates)
}

/// Hash a file using SHA256
fn hash_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// Display duplicate groups
pub fn display_duplicates(groups: &[DuplicateGroup]) {
    if groups.is_empty() {
        println!("{}", "No duplicate files found.".green());
        return;
    }

    let total_wasted: u64 = groups.iter().map(|g| g.wasted_space()).sum();
    let total_count: usize = groups.iter().map(|g| g.files.len() - 1).sum();

    println!("\n{}", "Duplicate Files Found:".bold().yellow());
    println!("{}", "─".repeat(60));

    for (i, group) in groups.iter().enumerate() {
        if i >= 10 {
            println!("\n... and {} more duplicate groups", groups.len() - 10);
            break;
        }

        println!(
            "\n  {} ({}) - {} copies:",
            format!("Group {}", i + 1).cyan().bold(),
            format_size(group.size).dimmed(),
            group.files.len()
        );

        for (j, file) in group.files.iter().enumerate() {
            let marker = if j == 0 { "●".green() } else { "○".yellow() };
            println!("    {} {}", marker, file.path.display());
        }
    }

    println!("\n{}", "─".repeat(60));
    println!(
        "\n{}: {} duplicate files in {} groups",
        "Summary".bold(),
        total_count.to_string().yellow(),
        groups.len().to_string().cyan()
    );
    println!(
        "{}: {} could be recovered by removing duplicates",
        "Wasted space".bold(),
        format_size(total_wasted).red()
    );
    println!(
        "\n{} Use {} to remove duplicates (keeps first file in each group).",
        "ℹ".blue(),
        "--delete --execute".yellow()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use tempfile::tempdir;

    fn make_file_info(path: PathBuf, size: u64) -> FileInfo {
        FileInfo {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            extension: path.extension().map(|e| e.to_string_lossy().to_string()),
            path,
            size,
            modified: SystemTime::now(),
            created: None,
        }
    }

    #[test]
    fn test_wasted_space_single_file() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![make_file_info(PathBuf::from("/a.txt"), 100)],
            size: 100,
        };
        assert_eq!(group.wasted_space(), 0);
    }

    #[test]
    fn test_wasted_space_two_files() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![
                make_file_info(PathBuf::from("/a.txt"), 100),
                make_file_info(PathBuf::from("/b.txt"), 100),
            ],
            size: 100,
        };
        assert_eq!(group.wasted_space(), 100); // 1 duplicate
    }

    #[test]
    fn test_wasted_space_three_files() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![
                make_file_info(PathBuf::from("/a.txt"), 500),
                make_file_info(PathBuf::from("/b.txt"), 500),
                make_file_info(PathBuf::from("/c.txt"), 500),
            ],
            size: 500,
        };
        assert_eq!(group.wasted_space(), 1000); // 2 duplicates * 500
    }

    #[test]
    fn test_find_duplicates_empty() {
        let files: Vec<FileInfo> = vec![];
        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_duplicates_no_duplicates() {
        let dir = tempdir().unwrap();
        
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        
        let mut f1 = File::create(&file1).unwrap();
        write!(f1, "content A").unwrap();
        
        let mut f2 = File::create(&file2).unwrap();
        write!(f2, "content B").unwrap();
        
        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];
        
        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty()); // Different content, no duplicates
    }

    #[test]
    fn test_find_duplicates_with_duplicates() {
        let dir = tempdir().unwrap();
        
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        
        let mut f1 = File::create(&file1).unwrap();
        write!(f1, "same content").unwrap();
        
        let mut f2 = File::create(&file2).unwrap();
        write!(f2, "same content").unwrap();
        
        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];
        
        let result = find_duplicates(&files).unwrap();
        assert_eq!(result.len(), 1); // One duplicate group
        assert_eq!(result[0].files.len(), 2);
    }

    #[test]
    fn test_find_duplicates_empty_files_skipped() {
        let dir = tempdir().unwrap();
        
        let file1 = dir.path().join("empty1.txt");
        let file2 = dir.path().join("empty2.txt");
        
        File::create(&file1).unwrap(); // Empty file
        File::create(&file2).unwrap(); // Empty file
        
        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];
        
        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty()); // Empty files are skipped
    }

    #[test]
    fn test_hash_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        
        let mut file = File::create(&file_path).unwrap();
        write!(file, "hello world").unwrap();
        
        let hash = hash_file(&file_path).unwrap();
        
        // SHA256 of "hello world" should be consistent
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA256 hex is 64 chars
    }
}
