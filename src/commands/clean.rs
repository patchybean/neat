//! Clean command handler

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use colored::*;

use crate::cleaner;
use crate::scanner::{parse_date, parse_size, scan_directory, ScanOptions};

/// Clean old files and empty folders
#[allow(clippy::too_many_arguments)]
pub fn run(
    path: &Path,
    older_than: Option<String>,
    empty_folders: bool,
    dry_run: bool,
    execute: bool,
    use_trash: bool,
    min_size: Option<String>,
    max_size: Option<String>,
    after: Option<String>,
    before: Option<String>,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    // Parse size filters
    let min_size_bytes = min_size
        .map(|s| parse_size(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let max_size_bytes = max_size
        .map(|s| parse_size(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    // Parse date filters
    let after_date = after
        .map(|s| parse_date(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let before_date = before
        .map(|s| parse_date(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    if let Some(duration_str) = older_than {
        let duration = cleaner::parse_duration(&duration_str)?;

        println!(
            "{} Scanning {} for files older than {}...",
            "→".cyan(),
            canonical_path.display().to_string().bold(),
            duration_str.cyan()
        );

        let options = ScanOptions {
            include_hidden: false,
            max_depth: None,
            follow_symlinks: false,
            ignore_patterns: Vec::new(),
            min_size: min_size_bytes,
            max_size: max_size_bytes,
            after_date,
            before_date,
            ..Default::default()
        };

        let files = scan_directory(&canonical_path, &options)?;
        let old_files = cleaner::find_old_files(&files, duration);

        if execute && !dry_run {
            cleaner::execute_clean(&old_files, false, use_trash)?;
        } else {
            cleaner::preview_clean(&old_files, &duration_str);
        }
    }

    if empty_folders {
        println!(
            "{} Scanning for empty folders in {}...",
            "→".cyan(),
            canonical_path.display().to_string().bold()
        );

        let empty_dirs = cleaner::find_empty_dirs(&canonical_path)?;

        if empty_dirs.is_empty() {
            println!("{}", "No empty folders found.".green());
        } else {
            println!("\n{}", "Empty folders:".yellow().bold());
            for dir in &empty_dirs {
                println!("  {} {}", "○".yellow(), dir.display());
            }
            println!(
                "\n{} {} empty folders found",
                "Summary:".bold(),
                empty_dirs.len()
            );

            if execute && !dry_run {
                for dir in empty_dirs {
                    if let Err(e) = fs::remove_dir(&dir) {
                        eprintln!("{} Failed to remove {}: {}", "✗".red(), dir.display(), e);
                    }
                }
            }
        }
    }

    Ok(())
}
