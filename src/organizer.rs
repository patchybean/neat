//! Organizer - move files to organized locations

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{Datelike, TimeZone, Utc};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::classifier::{Category, Classifier};
use crate::logger::Logger;
use crate::scanner::{format_size, FileInfo};

/// Organization mode
#[derive(Debug, Clone, Copy)]
pub enum OrganizeMode {
    ByType,
    ByDate,
    ByExtension,
}

/// A planned file move
#[derive(Debug, Clone)]
pub struct PlannedMove {
    pub from: PathBuf,
    pub to: PathBuf,
    pub size: u64,
}

/// Result of organizing
#[derive(Debug, Default)]
pub struct OrganizeResult {
    pub moved: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub total_size: u64,
}

/// Plan file moves based on the organization mode
pub fn plan_moves(
    files: &[FileInfo],
    base_path: &Path,
    mode: OrganizeMode,
) -> Vec<PlannedMove> {
    let classifier = Classifier::new();
    let mut moves = Vec::new();

    for file in files {
        let destination = match mode {
            OrganizeMode::ByType => {
                let category = classifier.classify(file.extension.as_deref());
                base_path.join(category.folder_name()).join(&file.name)
            }
            OrganizeMode::ByDate => {
                let datetime = file.modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| Utc.timestamp_opt(d.as_secs() as i64, 0).unwrap())
                    .unwrap_or_else(|_| Utc::now());
                
                let year = datetime.year().to_string();
                let month = format!("{:02}", datetime.month());
                
                base_path.join(year).join(month).join(&file.name)
            }
            OrganizeMode::ByExtension => {
                let ext = file.extension.as_deref().unwrap_or("no_extension");
                base_path.join(ext.to_uppercase()).join(&file.name)
            }
        };

        // Skip if file is already in the right place
        if file.path != destination {
            moves.push(PlannedMove {
                from: file.path.clone(),
                to: destination,
                size: file.size,
            });
        }
    }

    moves
}

/// Preview planned moves (dry-run)
pub fn preview_moves(moves: &[PlannedMove], base_path: &Path) {
    if moves.is_empty() {
        println!("{}", "No files to move.".yellow());
        return;
    }

    println!("\n{}", "Preview:".bold().cyan());
    println!("{}", "─".repeat(60));

    // Group by destination folder
    let mut by_folder: HashMap<PathBuf, Vec<&PlannedMove>> = HashMap::new();
    for mv in moves {
        let folder = mv.to.parent().unwrap_or(base_path).to_path_buf();
        by_folder.entry(folder).or_default().push(mv);
    }

    // Sort folders
    let mut folders: Vec<_> = by_folder.keys().collect();
    folders.sort();

    for folder in folders {
        let files = &by_folder[folder];
        let folder_name = folder.strip_prefix(base_path).unwrap_or(folder);
        println!("\n  {} ({} files)", folder_name.display().to_string().green().bold(), files.len());
        
        // Show first 5 files in each folder
        for mv in files.iter().take(5) {
            let from_name = mv.from.file_name().unwrap_or_default().to_string_lossy();
            println!("    {} {}", "→".dimmed(), from_name);
        }
        
        if files.len() > 5 {
            println!("    {} ... and {} more", "→".dimmed(), files.len() - 5);
        }
    }

    let total_size: u64 = moves.iter().map(|m| m.size).sum();
    println!("\n{}", "─".repeat(60));
    println!(
        "\n{}: {} files to move ({})",
        "Summary".bold(),
        moves.len().to_string().cyan(),
        format_size(total_size).cyan()
    );
    println!(
        "\n{} Use {} to execute these changes.",
        "ℹ".blue(),
        "--execute".yellow()
    );
}

/// Execute planned moves
pub fn execute_moves(moves: &[PlannedMove], command_name: &str) -> Result<OrganizeResult> {
    if moves.is_empty() {
        return Ok(OrganizeResult::default());
    }

    let pb = ProgressBar::new(moves.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▓░"),
    );

    let mut result = OrganizeResult::default();
    let mut logger = Logger::new(command_name);

    for mv in moves {
        pb.inc(1);

        // Create parent directory if needed
        if let Some(parent) = mv.to.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Failed to create directory: {:?}", parent))?;
            }
        }

        // Handle name conflicts
        let final_dest = resolve_conflict(&mv.to);

        // Move the file
        match fs::rename(&mv.from, &final_dest) {
            Ok(_) => {
                result.moved += 1;
                result.total_size += mv.size;
                logger.log_move(mv.from.clone(), final_dest);
            }
            Err(e) => {
                result.skipped += 1;
                result.errors.push(format!("{}: {}", mv.from.display(), e));
            }
        }
    }

    pb.finish_and_clear();
    logger.save()?;

    Ok(result)
}

/// Resolve filename conflicts by adding a number suffix
fn resolve_conflict(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }

    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let extension = path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let parent = path.parent().unwrap_or(Path::new("."));

    let mut counter = 1;
    loop {
        let new_name = format!("{}_{}{}", stem, counter, extension);
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

/// Print organize results
pub fn print_results(result: &OrganizeResult) {
    println!("\n{}", "Results:".bold().green());
    println!("{}", "─".repeat(40));
    
    if result.moved > 0 {
        println!(
            "  {} {} files moved ({})",
            "✓".green(),
            result.moved.to_string().green(),
            format_size(result.total_size).dimmed()
        );
    }

    if result.skipped > 0 {
        println!(
            "  {} {} files skipped",
            "⚠".yellow(),
            result.skipped.to_string().yellow()
        );
    }

    if !result.errors.is_empty() {
        println!("\n  {}", "Errors:".red());
        for error in result.errors.iter().take(5) {
            println!("    {} {}", "✗".red(), error);
        }
        if result.errors.len() > 5 {
            println!("    ... and {} more errors", result.errors.len() - 5);
        }
    }
}
