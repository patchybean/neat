//! Clean old files from directories

use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::{bail, Result};
use colored::*;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};

use crate::logger::Logger;
use crate::scanner::{format_size, FileInfo};

/// Parse a duration string (e.g., "30d", "7d", "1w")
pub fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim().to_lowercase();

    if s.is_empty() {
        bail!("Duration cannot be empty");
    }

    let (num_str, unit) = if s.ends_with('d') {
        (&s[..s.len() - 1], 'd')
    } else if s.ends_with('w') {
        (&s[..s.len() - 1], 'w')
    } else if s.ends_with('h') {
        (&s[..s.len() - 1], 'h')
    } else {
        // Default to days
        (s.as_str(), 'd')
    };

    let num: u64 = num_str.parse().map_err(|_| {
        anyhow::anyhow!(
            "Invalid duration format: {}. Use formats like 30d, 7d, 1w",
            s
        )
    })?;

    let seconds = match unit {
        'h' => num * 3600,
        'd' => num * 86400,
        'w' => num * 604800,
        _ => num * 86400,
    };

    Ok(Duration::from_secs(seconds))
}

/// Find files older than the specified duration
pub fn find_old_files(files: &[FileInfo], max_age: Duration) -> Vec<&FileInfo> {
    let now = SystemTime::now();
    let cutoff = now - max_age;

    files.iter().filter(|f| f.modified < cutoff).collect()
}

/// Preview files to be cleaned
pub fn preview_clean(files: &[&FileInfo], duration_str: &str) {
    if files.is_empty() {
        println!(
            "{} No files older than {} found.",
            "✓".green(),
            duration_str.cyan()
        );
        return;
    }

    let total_size: u64 = files.iter().map(|f| f.size).sum();

    println!(
        "\n{}",
        format!("Files older than {}:", duration_str)
            .bold()
            .yellow()
    );
    println!("{}", "─".repeat(60));

    for (i, file) in files.iter().enumerate() {
        if i >= 20 {
            println!("  ... and {} more files", files.len() - 20);
            break;
        }

        let age = file
            .modified
            .elapsed()
            .map(format_age)
            .unwrap_or_else(|_| "unknown".to_string());

        println!(
            "  {} {} ({}, {})",
            "○".yellow(),
            file.path.display(),
            format_size(file.size).dimmed(),
            age.dimmed()
        );
    }

    println!("\n{}", "─".repeat(60));
    println!(
        "\n{}: {} files ({}) would be deleted",
        "Summary".bold(),
        files.len().to_string().yellow(),
        format_size(total_size).red()
    );
    println!(
        "\n{} Use {} to delete these files.",
        "⚠".yellow(),
        "--execute".yellow()
    );
}

/// Execute file deletion with confirmation
pub fn execute_clean(files: &[&FileInfo], force: bool, use_trash: bool) -> Result<(usize, u64)> {
    if files.is_empty() {
        return Ok((0, 0));
    }

    let action = if use_trash { "Move to trash" } else { "Delete" };

    // Confirm with user unless forced
    if !force {
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "{} {} files ({})?",
                action,
                files.len(),
                format_size(files.iter().map(|f| f.size).sum())
            ))
            .default(false)
            .interact()?;

        if !confirmed {
            println!("{}", "Operation cancelled.".yellow());
            return Ok((0, 0));
        }
    }

    let pb = ProgressBar::new(files.len() as u64);
    let template = if use_trash {
        "{spinner:.green} Moving to trash [{bar:40.yellow/white}] {pos}/{len}"
    } else {
        "{spinner:.green} Deleting [{bar:40.red/white}] {pos}/{len}"
    };
    pb.set_style(
        ProgressStyle::default_bar()
            .template(template)
            .unwrap()
            .progress_chars("█▓░"),
    );

    let mut deleted = 0;
    let mut total_size = 0u64;
    let mut logger = Logger::new(if use_trash { "clean --trash" } else { "clean" });

    for file in files {
        pb.inc(1);
        let result = if use_trash {
            trash::delete(&file.path).map_err(|e| anyhow::anyhow!("{}", e))
        } else {
            fs::remove_file(&file.path).map_err(Into::into)
        };

        match result {
            Ok(_) => {
                deleted += 1;
                total_size += file.size;
                logger.log_delete(file.path.clone());
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to {} {}: {}",
                    "✗".red(),
                    if use_trash { "trash" } else { "delete" },
                    file.path.display(),
                    e
                );
            }
        }
    }

    pb.finish_and_clear();
    logger.save()?;

    let action_past = if use_trash {
        "Moved to trash"
    } else {
        "Deleted"
    };
    println!(
        "\n{} {} {} files ({})",
        "✓".green(),
        action_past,
        deleted.to_string().green(),
        format_size(total_size).green()
    );

    Ok((deleted, total_size))
}

/// Find empty directories
pub fn find_empty_dirs(path: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut empty_dirs = Vec::new();
    find_empty_dirs_recursive(path, &mut empty_dirs)?;
    Ok(empty_dirs)
}

fn find_empty_dirs_recursive(
    path: &Path,
    empty_dirs: &mut Vec<std::path::PathBuf>,
) -> Result<bool> {
    if !path.is_dir() {
        return Ok(false);
    }

    let entries: Vec<_> = fs::read_dir(path)?.filter_map(|e| e.ok()).collect();

    if entries.is_empty() {
        empty_dirs.push(path.to_path_buf());
        return Ok(true);
    }

    let entries_count = entries.len();
    let mut all_empty = true;
    for entry in &entries {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            if !find_empty_dirs_recursive(&entry_path, empty_dirs)? {
                all_empty = false;
            }
        } else {
            all_empty = false;
        }
    }

    if all_empty && entries_count > 0 {
        empty_dirs.push(path.to_path_buf());
    }

    Ok(all_empty)
}

/// Format age as human-readable string
fn format_age(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else if secs < 604800 {
        format!("{}d ago", secs / 86400)
    } else {
        format!("{}w ago", secs / 604800)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_duration_days() {
        let d = parse_duration("30d").unwrap();
        assert_eq!(d, Duration::from_secs(30 * 86400));
    }

    #[test]
    fn test_parse_duration_weeks() {
        let d = parse_duration("1w").unwrap();
        assert_eq!(d, Duration::from_secs(7 * 86400));
    }

    #[test]
    fn test_parse_duration_hours() {
        let d = parse_duration("24h").unwrap();
        assert_eq!(d, Duration::from_secs(24 * 3600));
    }

    #[test]
    fn test_parse_duration_no_unit_defaults_to_days() {
        let d = parse_duration("5").unwrap();
        assert_eq!(d, Duration::from_secs(5 * 86400));
    }

    #[test]
    fn test_parse_duration_with_whitespace() {
        let d = parse_duration("  7d  ").unwrap();
        assert_eq!(d, Duration::from_secs(7 * 86400));
    }

    #[test]
    fn test_parse_duration_empty_fails() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("   ").is_err());
    }

    #[test]
    fn test_parse_duration_invalid_fails() {
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("d").is_err());
    }

    #[test]
    fn test_find_old_files_empty() {
        let files: Vec<FileInfo> = vec![];
        let old = find_old_files(&files, Duration::from_secs(86400));
        assert!(old.is_empty());
    }

    #[test]
    fn test_find_old_files_filters_correctly() {
        let now = SystemTime::now();
        let old_time = now - Duration::from_secs(86400 * 10); // 10 days ago
        let new_time = now - Duration::from_secs(3600); // 1 hour ago

        let files = vec![
            FileInfo {
                name: "old.txt".to_string(),
                path: std::path::PathBuf::from("/tmp/old.txt"),
                size: 100,
                extension: Some("txt".to_string()),
                modified: old_time,
                created: None,
            },
            FileInfo {
                name: "new.txt".to_string(),
                path: std::path::PathBuf::from("/tmp/new.txt"),
                size: 100,
                extension: Some("txt".to_string()),
                modified: new_time,
                created: None,
            },
        ];

        let old_files = find_old_files(&files, Duration::from_secs(86400 * 5)); // 5 days
        assert_eq!(old_files.len(), 1);
        assert_eq!(old_files[0].name, "old.txt");
    }

    #[test]
    fn test_find_empty_dirs_empty_directory() {
        let dir = tempdir().unwrap();
        let empty_dir = dir.path().join("empty");
        fs::create_dir(&empty_dir).unwrap();

        let result = find_empty_dirs(dir.path()).unwrap();
        assert!(result.contains(&empty_dir));
    }

    #[test]
    fn test_find_empty_dirs_non_empty() {
        let dir = tempdir().unwrap();
        let non_empty = dir.path().join("has_file");
        fs::create_dir(&non_empty).unwrap();
        fs::write(non_empty.join("file.txt"), "content").unwrap();

        let result = find_empty_dirs(dir.path()).unwrap();
        assert!(!result.contains(&non_empty));
    }

    #[test]
    fn test_find_empty_dirs_nested() {
        let dir = tempdir().unwrap();
        let parent = dir.path().join("parent");
        let child = parent.join("child");
        fs::create_dir_all(&child).unwrap();

        let result = find_empty_dirs(dir.path()).unwrap();
        // Both child and parent should be marked empty
        assert!(result.contains(&child));
        assert!(result.contains(&parent));
    }

    #[test]
    fn test_format_age_minutes() {
        let age = format_age(Duration::from_secs(1800)); // 30 minutes
        assert_eq!(age, "30m ago");
    }

    #[test]
    fn test_format_age_hours() {
        let age = format_age(Duration::from_secs(7200)); // 2 hours
        assert_eq!(age, "2h ago");
    }

    #[test]
    fn test_format_age_days() {
        let age = format_age(Duration::from_secs(172800)); // 2 days
        assert_eq!(age, "2d ago");
    }

    #[test]
    fn test_format_age_weeks() {
        let age = format_age(Duration::from_secs(1209600)); // 2 weeks
        assert_eq!(age, "2w ago");
    }

    // ==================== Additional parse_duration edge cases ====================

    #[test]
    fn test_parse_duration_zero() {
        let d = parse_duration("0d").unwrap();
        assert_eq!(d.as_secs(), 0);
    }

    #[test]
    fn test_parse_duration_uppercase() {
        // Should be case-insensitive
        let d = parse_duration("7D").unwrap();
        assert_eq!(d, Duration::from_secs(7 * 86400));
    }

    #[test]
    fn test_parse_duration_large_value() {
        let d = parse_duration("365d").unwrap();
        assert_eq!(d.as_secs(), 365 * 86400);
    }

    #[test]
    fn test_parse_duration_negative_fails() {
        assert!(parse_duration("-7d").is_err());
    }

    // ==================== Additional format_age edge cases ====================

    #[test]
    fn test_format_age_zero() {
        let age = format_age(Duration::from_secs(0));
        assert_eq!(age, "0m ago");
    }

    #[test]
    fn test_format_age_one_second() {
        let age = format_age(Duration::from_secs(1));
        assert_eq!(age, "0m ago");
    }

    #[test]
    fn test_format_age_boundary_hour() {
        // Just under 1 hour
        let age = format_age(Duration::from_secs(3599));
        assert_eq!(age, "59m ago");
        // Exactly 1 hour
        let age = format_age(Duration::from_secs(3600));
        assert_eq!(age, "1h ago");
    }

    #[test]
    fn test_format_age_boundary_day() {
        // Just under 1 day
        let age = format_age(Duration::from_secs(86399));
        assert_eq!(age, "23h ago");
        // Exactly 1 day
        let age = format_age(Duration::from_secs(86400));
        assert_eq!(age, "1d ago");
    }
}

