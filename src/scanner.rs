//! File scanner - traverse directories and collect file information

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{Context, Result};
use walkdir::WalkDir;

/// Information about a scanned file
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub modified: SystemTime,
    pub created: Option<SystemTime>,
}

impl FileInfo {
    /// Create FileInfo from a path
    pub fn from_path(path: &Path) -> Result<Self> {
        let metadata = fs::metadata(path)
            .with_context(|| format!("Failed to read metadata for {:?}", path))?;

        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase());

        Ok(FileInfo {
            path: path.to_path_buf(),
            name,
            extension,
            size: metadata.len(),
            modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            created: metadata.created().ok(),
        })
    }
}

/// Scanner configuration
#[derive(Debug, Default)]
pub struct ScanOptions {
    /// Include hidden files (starting with .)
    pub include_hidden: bool,
    /// Maximum depth to scan (None = unlimited)
    pub max_depth: Option<usize>,
    /// Follow symlinks
    pub follow_symlinks: bool,
}

/// Scan a directory and return file information
pub fn scan_directory(path: &Path, options: &ScanOptions) -> Result<Vec<FileInfo>> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }

    if !path.is_dir() {
        anyhow::bail!("Not a directory: {:?}", path);
    }

    let mut walker = WalkDir::new(path).follow_links(options.follow_symlinks);

    if let Some(depth) = options.max_depth {
        walker = walker.max_depth(depth);
    }

    let files: Vec<FileInfo> = walker
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            if options.include_hidden {
                true
            } else {
                !entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with('.')
            }
        })
        .filter_map(|entry| FileInfo::from_path(entry.path()).ok())
        .collect();

    Ok(files)
}

/// Count total size of files
pub fn total_size(files: &[FileInfo]) -> u64 {
    files.iter().map(|f| f.size).sum()
}

/// Format bytes into human-readable string
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
