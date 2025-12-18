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
        anyhow::anyhow!("Invalid duration format: {}. Use formats like 30d, 7d, 1w", s)
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

    files
        .iter()
        .filter(|f| f.modified < cutoff)
        .collect()
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

    println!("\n{}", format!("Files older than {}:", duration_str).bold().yellow());
    println!("{}", "─".repeat(60));

    for (i, file) in files.iter().enumerate() {
        if i >= 20 {
            println!("  ... and {} more files", files.len() - 20);
            break;
        }

        let age = file.modified
            .elapsed()
            .map(|d| format_age(d))
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
pub fn execute_clean(files: &[&FileInfo], force: bool) -> Result<(usize, u64)> {
    if files.is_empty() {
        return Ok((0, 0));
    }

    // Confirm with user unless forced
    if !force {
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "Delete {} files ({})?",
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
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Deleting [{bar:40.red/white}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▓░"),
    );

    let mut deleted = 0;
    let mut total_size = 0u64;
    let mut logger = Logger::new("clean");

    for file in files {
        pb.inc(1);
        match fs::remove_file(&file.path) {
            Ok(_) => {
                deleted += 1;
                total_size += file.size;
                logger.log_delete(file.path.clone());
            }
            Err(e) => {
                eprintln!("{} Failed to delete {}: {}", "✗".red(), file.path.display(), e);
            }
        }
    }

    pb.finish_and_clear();
    logger.save()?;

    println!(
        "\n{} Deleted {} files ({})",
        "✓".green(),
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

fn find_empty_dirs_recursive(path: &Path, empty_dirs: &mut Vec<std::path::PathBuf>) -> Result<bool> {
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
}
