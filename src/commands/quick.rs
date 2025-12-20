//! Quick action commands for common workflows

use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::*;

use crate::cli::QuickAction;
use crate::organizer::{
    execute_moves, plan_moves, preview_moves, print_results, ConflictStrategy, OrganizeMode,
};
use crate::scanner::{scan_directory, ScanOptions};

/// Expand ~ to home directory
fn expand_home(path: &std::path::Path) -> PathBuf {
    let path_str = path.to_string_lossy();
    if let Some(stripped) = path_str.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped);
        }
    }
    path.to_path_buf()
}

/// Run a quick action
pub fn run(action: QuickAction) -> Result<()> {
    match action {
        QuickAction::Downloads { dry_run } => {
            let path = dirs::download_dir().context("Could not find Downloads directory")?;
            organize_by_type(&path, !dry_run, "downloads")
        }

        QuickAction::Desktop { dry_run } => {
            let path = dirs::desktop_dir().context("Could not find Desktop directory")?;
            organize_by_type(&path, !dry_run, "desktop")
        }

        QuickAction::Photos { path, dry_run } => {
            let expanded = expand_home(&path);
            let canonical = expanded
                .canonicalize()
                .with_context(|| format!("Path does not exist: {:?}", expanded))?;
            organize_photos(&canonical, !dry_run)
        }

        QuickAction::Music { path, dry_run } => {
            let expanded = expand_home(&path);
            let canonical = expanded
                .canonicalize()
                .with_context(|| format!("Path does not exist: {:?}", expanded))?;
            organize_music(&canonical, !dry_run)
        }

        QuickAction::Cleanup {
            days,
            trash,
            dry_run,
        } => {
            let path = dirs::download_dir().context("Could not find Downloads directory")?;
            cleanup_old_files(&path, days, trash, !dry_run)
        }
    }
}

/// Organize files by type
fn organize_by_type(path: &std::path::Path, execute: bool, name: &str) -> Result<()> {
    println!(
        "{} Quick action: Organize {} by type",
        "→".cyan(),
        name.bold()
    );
    println!("  Path: {}", path.display().to_string().dimmed());
    println!();

    let options = ScanOptions {
        include_hidden: false,
        max_depth: Some(1),
        follow_symlinks: false,
        ignore_patterns: vec![],
        min_size: None,
        max_size: None,
        after_date: None,
        before_date: None,
        ..Default::default()
    };

    let files = scan_directory(path, &options)?;

    if files.is_empty() {
        println!("{}", "No files found.".yellow());
        return Ok(());
    }

    let moves = plan_moves(&files, path, OrganizeMode::ByType);

    if moves.is_empty() {
        println!("{}", "All files are already organized.".green());
        return Ok(());
    }

    if execute {
        let result = execute_moves(&moves, &format!("quick {}", name), ConflictStrategy::Rename)?;
        print_results(&result);
    } else {
        preview_moves(&moves, path);
    }

    Ok(())
}

/// Organize photos by date taken
fn organize_photos(path: &std::path::Path, execute: bool) -> Result<()> {
    println!("{} Quick action: Organize photos by date taken", "→".cyan());
    println!("  Path: {}", path.display().to_string().dimmed());
    println!();

    let options = ScanOptions {
        include_hidden: false,
        max_depth: None, // Recursive
        follow_symlinks: false,
        ignore_patterns: vec![],
        min_size: None,
        max_size: None,
        after_date: None,
        before_date: None,
        ..Default::default()
    };

    let files = scan_directory(path, &options)?;
    let moves = plan_moves(&files, path, OrganizeMode::ByDateTaken);

    if moves.is_empty() {
        println!("{}", "All photos are already organized.".green());
        return Ok(());
    }

    if execute {
        let result = execute_moves(&moves, "quick photos", ConflictStrategy::Rename)?;
        print_results(&result);
    } else {
        preview_moves(&moves, path);
    }

    Ok(())
}

/// Organize music by album
fn organize_music(path: &std::path::Path, execute: bool) -> Result<()> {
    println!(
        "{} Quick action: Organize music by artist/album",
        "→".cyan()
    );
    println!("  Path: {}", path.display().to_string().dimmed());
    println!();

    let options = ScanOptions {
        include_hidden: false,
        max_depth: None, // Recursive
        follow_symlinks: false,
        ignore_patterns: vec![],
        min_size: None,
        max_size: None,
        after_date: None,
        before_date: None,
        ..Default::default()
    };

    let files = scan_directory(path, &options)?;
    let moves = plan_moves(&files, path, OrganizeMode::ByAlbum);

    if moves.is_empty() {
        println!("{}", "All music is already organized.".green());
        return Ok(());
    }

    if execute {
        let result = execute_moves(&moves, "quick music", ConflictStrategy::Rename)?;
        print_results(&result);
    } else {
        preview_moves(&moves, path);
    }

    Ok(())
}

/// Clean up old files
fn cleanup_old_files(
    path: &std::path::Path,
    days: u32,
    use_trash: bool,
    execute: bool,
) -> Result<()> {
    use std::time::{Duration, SystemTime};

    println!(
        "{} Quick action: Clean files older than {} days",
        "→".cyan(),
        days.to_string().bold()
    );
    println!("  Path: {}", path.display().to_string().dimmed());
    println!();

    let threshold = SystemTime::now() - Duration::from_secs(days as u64 * 24 * 60 * 60);

    let options = ScanOptions {
        include_hidden: false,
        max_depth: Some(1),
        follow_symlinks: false,
        ignore_patterns: vec![],
        min_size: None,
        max_size: None,
        after_date: None,
        before_date: Some(threshold),
        ..Default::default()
    };

    let files = scan_directory(path, &options)?;

    if files.is_empty() {
        println!("{}", "No old files found.".green());
        return Ok(());
    }

    println!(
        "{} Found {} files older than {} days:",
        "→".yellow(),
        files.len(),
        days
    );
    for file in files.iter().take(10) {
        println!("  {} {}", "○".dimmed(), file.name);
    }
    if files.len() > 10 {
        println!("  ... and {} more", files.len() - 10);
    }
    println!();

    if execute {
        let action = if use_trash { "Move to trash" } else { "Delete" };
        println!(
            "{} {} {} files? (this action is not undoable)",
            "⚠".yellow(),
            action,
            files.len()
        );
        println!("  Use the 'clean' command with --trash for safer deletion.");
    } else {
        println!(
            "{} Run without {} to execute this cleanup.",
            "ℹ".blue(),
            "--dry-run".yellow()
        );
    }

    Ok(())
}
