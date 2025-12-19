//! File scanner - traverse directories and collect file information

use std::fs::{self, File};
use std::io::{BufRead, BufReader};
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
    #[allow(dead_code)]
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

        let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase());

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
#[derive(Debug, Default, Clone)]
pub struct ScanOptions {
    /// Include hidden files (starting with .)
    pub include_hidden: bool,
    /// Maximum depth to scan (None = unlimited)
    pub max_depth: Option<usize>,
    /// Follow symlinks
    pub follow_symlinks: bool,
    /// Patterns to ignore (glob patterns like .gitignore)
    pub ignore_patterns: Vec<String>,
    /// Minimum file size in bytes (None = no minimum)
    pub min_size: Option<u64>,
    /// Maximum file size in bytes (None = no maximum)
    pub max_size: Option<u64>,
}

/// Load ignore patterns from .neatignore file in the given directory
pub fn load_ignore_patterns(dir: &Path) -> Vec<String> {
    let ignore_file = dir.join(".neatignore");
    if !ignore_file.exists() {
        return Vec::new();
    }

    let file = match File::open(&ignore_file) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

/// Scan a directory and return file information
pub fn scan_directory(path: &Path, options: &ScanOptions) -> Result<Vec<FileInfo>> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {:?}", path);
    }

    if !path.is_dir() {
        anyhow::bail!("Not a directory: {:?}", path);
    }

    // Compile ignore patterns
    let ignore_patterns: Vec<glob::Pattern> = options
        .ignore_patterns
        .iter()
        .filter_map(|p| glob::Pattern::new(p).ok())
        .collect();

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
                !entry.file_name().to_string_lossy().starts_with('.')
            }
        })
        .filter(|entry| {
            // Check if file matches any ignore pattern
            let file_name = entry.file_name().to_string_lossy();
            let file_path = entry.path().to_string_lossy();
            !ignore_patterns
                .iter()
                .any(|pattern| pattern.matches(&file_name) || pattern.matches(&file_path))
        })
        .filter_map(|entry| FileInfo::from_path(entry.path()).ok())
        // Apply size filters
        .filter(|file| {
            if let Some(min) = options.min_size {
                if file.size < min {
                    return false;
                }
            }
            if let Some(max) = options.max_size {
                if file.size > max {
                    return false;
                }
            }
            true
        })
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

/// Parse a human-readable size string to bytes
/// Examples: "10MB", "1.5GB", "500KB", "1024", "100B"
pub fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim().to_uppercase();

    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    // Try to find the unit suffix
    let (num_str, multiplier) = if s.ends_with("TB") {
        (&s[..s.len() - 2], TB)
    } else if s.ends_with("GB") {
        (&s[..s.len() - 2], GB)
    } else if s.ends_with("MB") {
        (&s[..s.len() - 2], MB)
    } else if s.ends_with("KB") {
        (&s[..s.len() - 2], KB)
    } else if s.ends_with("B") {
        (&s[..s.len() - 1], 1)
    } else if s.ends_with("T") {
        (&s[..s.len() - 1], TB)
    } else if s.ends_with("G") {
        (&s[..s.len() - 1], GB)
    } else if s.ends_with("M") {
        (&s[..s.len() - 1], MB)
    } else if s.ends_with("K") {
        (&s[..s.len() - 1], KB)
    } else {
        (s.as_str(), 1) // No suffix, assume bytes
    };

    let num: f64 = num_str
        .trim()
        .parse()
        .map_err(|_| format!("Invalid size format: {}", s))?;

    if num < 0.0 {
        return Err("Size cannot be negative".to_string());
    }

    Ok((num * multiplier as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn test_format_size_kb() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(10240), "10.00 KB");
    }

    #[test]
    fn test_format_size_mb() {
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 5), "5.00 MB");
    }

    #[test]
    fn test_format_size_gb() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 2), "2.00 GB");
    }

    #[test]
    fn test_total_size() {
        let files = vec![
            FileInfo {
                path: PathBuf::from("/test/a.txt"),
                name: "a.txt".to_string(),
                extension: Some("txt".to_string()),
                size: 100,
                modified: SystemTime::now(),
                created: None,
            },
            FileInfo {
                path: PathBuf::from("/test/b.txt"),
                name: "b.txt".to_string(),
                extension: Some("txt".to_string()),
                size: 200,
                modified: SystemTime::now(),
                created: None,
            },
        ];
        assert_eq!(total_size(&files), 300);
    }

    #[test]
    fn test_total_size_empty() {
        let files: Vec<FileInfo> = vec![];
        assert_eq!(total_size(&files), 0);
    }

    #[test]
    fn test_scan_directory_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let options = ScanOptions::default();
        let result = scan_directory(dir.path(), &options).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "test.txt");
        assert_eq!(result[0].extension, Some("txt".to_string()));
    }

    #[test]
    fn test_scan_directory_hidden_files() {
        let dir = tempdir().unwrap();

        // Create regular file
        File::create(dir.path().join("visible.txt")).unwrap();
        // Create hidden file
        File::create(dir.path().join(".hidden")).unwrap();

        // Without hidden files
        let options = ScanOptions {
            include_hidden: false,
            ..Default::default()
        };
        let result = scan_directory(dir.path(), &options).unwrap();
        assert_eq!(result.len(), 1);

        // With hidden files
        let options = ScanOptions {
            include_hidden: true,
            ..Default::default()
        };
        let result = scan_directory(dir.path(), &options).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_scan_directory_max_depth() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        File::create(dir.path().join("root.txt")).unwrap();
        File::create(subdir.join("nested.txt")).unwrap();

        // Depth 1 (only root)
        let options = ScanOptions {
            max_depth: Some(1),
            ..Default::default()
        };
        let result = scan_directory(dir.path(), &options).unwrap();
        assert_eq!(result.len(), 1);

        // Depth 2 (includes subdir)
        let options = ScanOptions {
            max_depth: Some(2),
            ..Default::default()
        };
        let result = scan_directory(dir.path(), &options).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_scan_directory_nonexistent() {
        let options = ScanOptions::default();
        let result = scan_directory(Path::new("/nonexistent/path"), &options);
        assert!(result.is_err());
    }

    #[test]
    fn test_file_info_from_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.pdf");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "content").unwrap();

        let info = FileInfo::from_path(&file_path).unwrap();
        assert_eq!(info.name, "test.pdf");
        assert_eq!(info.extension, Some("pdf".to_string()));
        assert_eq!(info.size, 7); // "content" = 7 bytes
    }

    #[test]
    fn test_file_info_no_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("Makefile");
        File::create(&file_path).unwrap();

        let info = FileInfo::from_path(&file_path).unwrap();
        assert_eq!(info.name, "Makefile");
        assert_eq!(info.extension, None);
    }
}
