//! Duplicates command handler

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use colored::*;

use crate::duplicates::{display_duplicates, find_duplicates};
use crate::export;
use crate::scanner::{parse_date, parse_size, scan_directory, ScanOptions};

/// Find and optionally delete duplicate files
#[allow(clippy::too_many_arguments)]
pub fn run(
    path: &Path,
    delete: bool,
    dry_run: bool,
    execute: bool,
    use_trash: bool,
    min_size: Option<String>,
    max_size: Option<String>,
    after: Option<String>,
    before: Option<String>,
    json: bool,
    csv: bool,
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

    if !json && !csv {
        println!(
            "{} Scanning {} for duplicate files...",
            "→".cyan(),
            canonical_path.display().to_string().bold()
        );
    }

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
    if !json && !csv {
        println!("  Found {} files to analyze", files.len());
    }

    let duplicates = find_duplicates(&files)?;

    // Handle export formats
    if json {
        export::export_duplicates_json(&duplicates, &mut std::io::stdout())?;
        return Ok(());
    }
    if csv {
        export::export_duplicates_csv(&duplicates, &mut std::io::stdout())?;
        return Ok(());
    }

    display_duplicates(&duplicates);

    if delete && execute && !dry_run && !duplicates.is_empty() {
        let action = if use_trash { "Move to trash" } else { "Delete" };
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!(
                "{} duplicate files (keeping first in each group)?",
                action
            ))
            .default(false)
            .interact()?;

        if confirmed {
            let mut deleted = 0;
            for group in &duplicates {
                // Skip the first file (the one we keep)
                for file in group.files.iter().skip(1) {
                    let result = if use_trash {
                        trash::delete(&file.path).map_err(|e| anyhow::anyhow!("{}", e))
                    } else {
                        fs::remove_file(&file.path).map_err(Into::into)
                    };

                    match result {
                        Ok(_) => deleted += 1,
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
            }
            let action_past = if use_trash {
                "Moved to trash"
            } else {
                "Deleted"
            };
            println!(
                "\n{} {} {} duplicate files",
                "✓".green(),
                action_past,
                deleted.to_string().green()
            );
        }
    }

    Ok(())
}
