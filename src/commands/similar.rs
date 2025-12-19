//! Similar images command handler

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use colored::*;
use dialoguer::Confirm;

use crate::duplicates;
use crate::scanner::{scan_directory, ScanOptions};

/// Find visually similar images using perceptual hashing
pub fn run(
    path: &Path,
    threshold: u32,
    delete: bool,
    dry_run: bool,
    execute: bool,
    use_trash: bool,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    println!(
        "{} Scanning {} for similar images (threshold: {})...",
        "→".cyan(),
        canonical_path.display().to_string().bold(),
        threshold
    );

    let options = ScanOptions {
        include_hidden: false,
        max_depth: None,
        follow_symlinks: false,
        ignore_patterns: Vec::new(),
        min_size: None,
        max_size: None,
        after_date: None,
        before_date: None,
        ..Default::default()
    };

    let files = scan_directory(&canonical_path, &options)?;
    let similar = duplicates::find_similar_images(&files, threshold)?;

    duplicates::display_similar_images(&similar);

    // Delete similar images if requested
    if delete && execute && !dry_run {
        if similar.is_empty() {
            return Ok(());
        }

        let action = if use_trash { "trash" } else { "delete" };
        let files_to_remove: Vec<_> = similar
            .iter()
            .flat_map(|g| g.similar.iter().map(|(f, _)| f))
            .collect();

        if files_to_remove.is_empty() {
            return Ok(());
        }

        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to {} {} similar images?",
                action,
                files_to_remove.len()
            ))
            .default(false)
            .interact()?;

        if confirm {
            let mut deleted = 0;
            for file in files_to_remove {
                let result: Result<()> = if use_trash {
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
            let action_past = if use_trash {
                "Moved to trash"
            } else {
                "Deleted"
            };
            println!(
                "\n{} {} {} similar images",
                "✓".green(),
                action_past,
                deleted.to_string().green()
            );
        }
    }

    Ok(())
}
