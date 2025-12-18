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
