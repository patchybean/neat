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
    /// Only include files modified after this date (None = no filter)
    pub after_date: Option<std::time::SystemTime>,
    /// Only include files modified before this date (None = no filter)
    pub before_date: Option<std::time::SystemTime>,
    /// Name filter: files starting with
    pub name_startswith: Option<String>,
    /// Name filter: files ending with
    pub name_endswith: Option<String>,
    /// Name filter: files containing
    pub name_contains: Option<String>,
    /// Regex pattern to match filename
    pub regex_pattern: Option<String>,
    /// MIME type filter (e.g., "image/*", "application/pdf")
    pub mime_filter: Option<String>,
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
        // Apply date filters
        .filter(|file| {
            if let Some(after) = options.after_date {
                if file.modified < after {
                    return false;
                }
            }
            if let Some(before) = options.before_date {
                if file.modified > before {
                    return false;
                }
            }
            true
        })
        // Apply name filters
        .filter(|file| {
            use crate::core::filters::NameFilter;
            let filter = NameFilter {
                startswith: options.name_startswith.clone(),
                endswith: options.name_endswith.clone(),
                contains: options.name_contains.clone(),
                case_insensitive: true,
            };
            if filter.is_empty() {
                return true;
            }
            filter.matches(&file.name)
        })
        // Apply regex filter
        .filter(|file| {
            if let Some(ref pattern) = options.regex_pattern {
                crate::core::filters::matches_regex(&file.name, pattern).unwrap_or_default()
            } else {
                true
            }
        })
        // Apply MIME filter
        .filter(|file| {
            if let Some(ref mime_filter) = options.mime_filter {
                crate::core::filters::matches_mime(&file.path, mime_filter)
            } else {
                true
            }
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

/// Parse a date string to SystemTime
/// Supports formats: "YYYY-MM-DD", "YYYY/MM/DD"
pub fn parse_date(s: &str) -> Result<std::time::SystemTime, String> {
    use chrono::{NaiveDate, TimeZone, Utc};

    let s = s.trim();

    // Try YYYY-MM-DD format
    let date = if s.contains('-') {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
    } else if s.contains('/') {
        NaiveDate::parse_from_str(s, "%Y/%m/%d")
    } else {
        return Err(format!(
            "Invalid date format: {}. Use YYYY-MM-DD or YYYY/MM/DD",
            s
        ));
    };

    match date {
        Ok(d) => {
            let datetime = Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap());
            Ok(datetime.into())
        }
        Err(_) => Err(format!("Invalid date: {}. Use YYYY-MM-DD or YYYY/MM/DD", s)),
    }
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

    // ==================== parse_size tests ====================

    #[test]
    fn test_parse_size_valid_units() {
        assert_eq!(parse_size("10MB").unwrap(), 10 * 1024 * 1024);
        assert_eq!(
            parse_size("1.5GB").unwrap(),
            (1.5 * 1024.0 * 1024.0 * 1024.0) as u64
        );
        assert_eq!(parse_size("500KB").unwrap(), 500 * 1024);
        assert_eq!(parse_size("100B").unwrap(), 100);
        assert_eq!(
            parse_size("1TB").unwrap(),
            1024_u64 * 1024 * 1024 * 1024
        );
    }

    #[test]
    fn test_parse_size_short_units() {
        assert_eq!(parse_size("10M").unwrap(), 10 * 1024 * 1024);
        assert_eq!(parse_size("1G").unwrap(), 1024 * 1024 * 1024);
        assert_eq!(parse_size("500K").unwrap(), 500 * 1024);
        assert_eq!(parse_size("1T").unwrap(), 1024_u64 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_edge_cases() {
        // Empty string
        assert!(parse_size("").is_err());
        // Invalid string
        assert!(parse_size("invalid").is_err());
        // Negative number
        assert!(parse_size("-10MB").is_err());
        // Whitespace handling
        assert_eq!(parse_size("  10MB  ").unwrap(), 10 * 1024 * 1024);
        // No unit (defaults to bytes)
        assert_eq!(parse_size("1024").unwrap(), 1024);
        // Zero
        assert_eq!(parse_size("0").unwrap(), 0);
        assert_eq!(parse_size("0MB").unwrap(), 0);
        // Lowercase
        assert_eq!(parse_size("10mb").unwrap(), 10 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_float_values() {
        assert_eq!(parse_size("0.5GB").unwrap(), (0.5 * 1024.0 * 1024.0 * 1024.0) as u64);
        assert_eq!(parse_size("2.5MB").unwrap(), (2.5 * 1024.0 * 1024.0) as u64);
    }

    // ==================== parse_date tests ====================

    #[test]
    fn test_parse_date_dash_format() {
        let result = parse_date("2024-12-25");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_date_slash_format() {
        let result = parse_date("2024/12/25");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_date_invalid_format() {
        // Unsupported format
        assert!(parse_date("2024.12.25").is_err());
        assert!(parse_date("25-12-2024").is_err());
        assert!(parse_date("12/25/2024").is_err());
    }

    #[test]
    fn test_parse_date_invalid_dates() {
        // Empty string
        assert!(parse_date("").is_err());
        // Not a date
        assert!(parse_date("not-a-date").is_err());
        // Invalid month
        assert!(parse_date("2024-13-01").is_err());
        // Invalid day
        assert!(parse_date("2024-02-30").is_err());
        assert!(parse_date("2024-04-31").is_err());
    }

    #[test]
    fn test_parse_date_leap_year() {
        // Valid leap year date
        assert!(parse_date("2024-02-29").is_ok());
        // Invalid non-leap year date
        assert!(parse_date("2023-02-29").is_err());
    }

    #[test]
    fn test_parse_date_whitespace() {
        assert!(parse_date("  2024-12-25  ").is_ok());
    }
}
