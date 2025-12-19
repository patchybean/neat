//! Neat - A smart CLI tool to organize and clean up messy directories

mod classifier;
mod cleaner;
mod cli;
mod config;
mod duplicates;
mod error;
mod logger;
mod metadata;
mod organizer;
mod scanner;
mod tui;
mod watcher;

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;

use crate::cli::{Cli, Commands, ConfigAction};
use crate::duplicates::{display_duplicates, find_duplicates};
use crate::logger::{History, OperationType};
use crate::organizer::{execute_moves, plan_moves, preview_moves, print_results, OrganizeMode};
use crate::scanner::{format_size, scan_directory, total_size, ScanOptions};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Organize {
            path,
            by_type,
            by_date,
            by_extension,
            by_camera,
            by_date_taken,
            dry_run,
            execute,
            ignore,
        } => {
            cmd_organize(
                &path,
                by_type,
                by_date,
                by_extension,
                by_camera,
                by_date_taken,
                dry_run,
                execute,
                cli.verbose,
                ignore,
            )?;
        }

        Commands::Clean {
            path,
            older_than,
            empty_folders,
            dry_run,
            execute,
            trash,
        } => {
            cmd_clean(&path, older_than, empty_folders, dry_run, execute, trash)?;
        }

        Commands::Duplicates {
            path,
            delete,
            dry_run,
            execute,
            trash,
        } => {
            cmd_duplicates(&path, delete, dry_run, execute, trash)?;
        }

        Commands::Stats { path } => {
            cmd_stats(&path)?;
        }

        Commands::Undo => {
            cmd_undo()?;
        }

        Commands::History => {
            cmd_history()?;
        }

        Commands::Watch {
            path,
            by_type,
            by_date,
            by_extension,
            config,
            auto,
        } => {
            cmd_watch(&path, by_type, by_date, by_extension, config, auto)?;
        }

        Commands::Config { action } => {
            cmd_config(action)?;
        }

        Commands::Tui { path } => {
            tui::run_tui(&path)?;
        }

        Commands::Completions { shell } => {
            use clap::CommandFactory;
            use clap_complete::generate;
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "neatcli", &mut std::io::stdout());
        }
    }

    Ok(())
}

/// Organize command handler
#[allow(clippy::too_many_arguments)]
fn cmd_organize(
    path: &Path,
    _by_type: bool,
    by_date: bool,
    by_extension: bool,
    by_camera: bool,
    by_date_taken: bool,
    dry_run: bool,
    execute: bool,
    verbose: bool,
    ignore: Vec<String>,
) -> Result<()> {
    // Determine mode
    let mode = if by_date {
        OrganizeMode::ByDate
    } else if by_extension {
        OrganizeMode::ByExtension
    } else if by_camera {
        OrganizeMode::ByCamera
    } else if by_date_taken {
        OrganizeMode::ByDateTaken
    } else {
        OrganizeMode::ByType // Default
    };

    let mode_name = match mode {
        OrganizeMode::ByType => "type",
        OrganizeMode::ByDate => "date",
        OrganizeMode::ByExtension => "extension",
        OrganizeMode::ByCamera => "camera",
        OrganizeMode::ByDateTaken => "date taken",
    };

    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    println!(
        "{} Scanning {} (organizing by {})...",
        "→".cyan(),
        canonical_path.display().to_string().bold(),
        mode_name.cyan()
    );

    // Load ignore patterns from .neatignore file and CLI
    let mut ignore_patterns = scanner::load_ignore_patterns(&canonical_path);
    ignore_patterns.extend(ignore);

    // Scan directory
    let options = ScanOptions {
        include_hidden: false,
        max_depth: Some(1), // Only immediate children
        follow_symlinks: false,
        ignore_patterns,
    };

    let files = scan_directory(&canonical_path, &options)?;

    if files.is_empty() {
        println!("{}", "No files found to organize.".yellow());
        return Ok(());
    }

    if verbose {
        println!(
            "  Found {} files ({})",
            files.len(),
            format_size(total_size(&files))
        );
    }

    // Plan moves
    let moves = plan_moves(&files, &canonical_path, mode);

    if moves.is_empty() {
        println!("{}", "All files are already organized.".green());
        return Ok(());
    }

    // Dry-run is default if --execute is not specified
    if execute && !dry_run {
        let result = execute_moves(&moves, &format!("organize --by-{}", mode_name))?;
        print_results(&result);
    } else {
        preview_moves(&moves, &canonical_path);
    }

    Ok(())
}

/// Clean command handler
fn cmd_clean(
    path: &Path,
    older_than: Option<String>,
    empty_folders: bool,
    dry_run: bool,
    execute: bool,
    use_trash: bool,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

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

/// Duplicates command handler
fn cmd_duplicates(
    path: &Path,
    delete: bool,
    dry_run: bool,
    execute: bool,
    use_trash: bool,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    println!(
        "{} Scanning {} for duplicate files...",
        "→".cyan(),
        canonical_path.display().to_string().bold()
    );

    let options = ScanOptions {
        include_hidden: false,
        max_depth: None,
        follow_symlinks: false,
        ignore_patterns: Vec::new(),
    };

    let files = scan_directory(&canonical_path, &options)?;
    println!("  Found {} files to analyze", files.len());

    let duplicates = find_duplicates(&files)?;
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

/// Stats command handler
fn cmd_stats(path: &Path) -> Result<()> {
    use crate::classifier::Classifier;
    use std::collections::HashMap;

    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    println!(
        "{} Analyzing {}...\n",
        "→".cyan(),
        canonical_path.display().to_string().bold()
    );

    let options = ScanOptions {
        include_hidden: false,
        max_depth: None,
        follow_symlinks: false,
        ignore_patterns: Vec::new(),
    };

    let files = scan_directory(&canonical_path, &options)?;

    if files.is_empty() {
        println!("{}", "No files found.".yellow());
        return Ok(());
    }

    let classifier = Classifier::new();

    // Group by category
    let mut by_category: HashMap<String, (usize, u64)> = HashMap::new();
    for file in &files {
        let category = classifier.classify(file.extension.as_deref());
        let entry = by_category
            .entry(category.folder_name().to_string())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += file.size;
    }

    // Sort by count
    let mut categories: Vec<_> = by_category.into_iter().collect();
    categories.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    println!("{}", "Files by Type:".bold());
    println!("{}", "─".repeat(50));
    for (category, (count, size)) in &categories {
        let bar_len = (*count as f64 / files.len() as f64 * 30.0) as usize;
        let bar = "█".repeat(bar_len);
        println!(
            "  {:12} {:>5} files {:>10}  {}",
            category.cyan(),
            count,
            format_size(*size).dimmed(),
            bar.green()
        );
    }

    // Top 10 largest files
    let mut sorted_files = files.clone();
    sorted_files.sort_by(|a, b| b.size.cmp(&a.size));

    println!("\n{}", "Largest Files:".bold());
    println!("{}", "─".repeat(50));
    for file in sorted_files.iter().take(10) {
        println!(
            "  {:>10}  {}",
            format_size(file.size).yellow(),
            file.name.dimmed()
        );
    }

    // Top 10 oldest files
    sorted_files.sort_by(|a, b| a.modified.cmp(&b.modified));

    println!("\n{}", "Oldest Files:".bold());
    println!("{}", "─".repeat(50));
    for file in sorted_files.iter().take(10) {
        let age = file
            .modified
            .elapsed()
            .map(|d| {
                let days = d.as_secs() / 86400;
                if days > 365 {
                    format!("{}y ago", days / 365)
                } else if days > 30 {
                    format!("{}mo ago", days / 30)
                } else {
                    format!("{}d ago", days)
                }
            })
            .unwrap_or_else(|_| "unknown".to_string());

        println!("  {:>10}  {}", age.yellow(), file.name.dimmed());
    }

    // Summary
    println!("\n{}", "─".repeat(50));
    println!(
        "{}: {} files, {}",
        "Total".bold(),
        files.len().to_string().cyan(),
        format_size(total_size(&files)).cyan()
    );

    Ok(())
}

/// Undo command handler
fn cmd_undo() -> Result<()> {
    let mut history = History::load()?;

    if history.is_empty() {
        println!("{}", "No operations to undo.".yellow());
        return Ok(());
    }

    let batch = history.pop_last().unwrap();

    println!(
        "{} Undoing '{}' ({} operations)...",
        "→".cyan(),
        batch.command.bold(),
        batch.operations.len()
    );

    let mut undone = 0;
    let mut errors = 0;

    for op in batch.operations.iter().rev() {
        match op.operation_type {
            OperationType::Move => {
                // Reverse the move
                if op.to.exists() {
                    // Create parent directory if needed
                    if let Some(parent) = op.from.parent() {
                        fs::create_dir_all(parent).ok();
                    }

                    match fs::rename(&op.to, &op.from) {
                        Ok(_) => undone += 1,
                        Err(e) => {
                            errors += 1;
                            eprintln!(
                                "{} Failed to restore {}: {}",
                                "✗".red(),
                                op.from.display(),
                                e
                            );
                        }
                    }
                }
            }
            OperationType::Delete => {
                // Cannot undo deletes
                eprintln!(
                    "{} Cannot restore deleted file: {}",
                    "⚠".yellow(),
                    op.from.display()
                );
                errors += 1;
            }
        }
    }

    if undone > 0 {
        history.save()?;
        println!(
            "\n{} Restored {} files",
            "✓".green(),
            undone.to_string().green()
        );
    }

    if errors > 0 {
        println!(
            "{} {} operations could not be undone",
            "⚠".yellow(),
            errors.to_string().yellow()
        );
    }

    Ok(())
}

/// History command handler
fn cmd_history() -> Result<()> {
    let history = History::load()?;

    if history.is_empty() {
        println!("{}", "No operation history.".yellow());
        return Ok(());
    }

    println!("{}", "Operation History:".bold());
    println!("{}", "─".repeat(60));

    for (i, batch) in history.batches.iter().rev().enumerate() {
        if i >= 10 {
            println!("... and {} more operations", history.batches.len() - 10);
            break;
        }

        let timestamp = batch.timestamp.format("%Y-%m-%d %H:%M:%S");
        println!(
            "  {} {} ({} files)",
            timestamp.to_string().dimmed(),
            batch.command.cyan(),
            batch.operations.len()
        );
    }

    Ok(())
}

/// Watch command handler
fn cmd_watch(
    path: &Path,
    _by_type: bool,
    by_date: bool,
    by_extension: bool,
    config_path: Option<std::path::PathBuf>,
    auto: bool,
) -> Result<()> {
    use crate::config::Config as NeatConfig;

    // Determine mode
    let mode = if by_date {
        OrganizeMode::ByDate
    } else if by_extension {
        OrganizeMode::ByExtension
    } else {
        OrganizeMode::ByType // Default
    };

    // Load config if specified
    let config = if let Some(cfg_path) = config_path {
        Some(NeatConfig::load(&cfg_path)?)
    } else {
        NeatConfig::load_default()?
    };

    watcher::watch_directory(path, mode, config.as_ref(), auto)
}

/// Config command handler
fn cmd_config(action: ConfigAction) -> Result<()> {
    use crate::config::Config as NeatConfig;

    match action {
        ConfigAction::Init { path } => {
            let config_path = path.unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(".neat")
                    .join("config.toml")
            });

            if config_path.exists() {
                println!(
                    "{} Config file already exists: {}",
                    "⚠".yellow(),
                    config_path.display()
                );

                let overwrite = dialoguer::Confirm::new()
                    .with_prompt("Overwrite?")
                    .default(false)
                    .interact()?;

                if !overwrite {
                    println!("{}", "Cancelled.".yellow());
                    return Ok(());
                }
            }

            NeatConfig::create_sample(&config_path)?;
            println!(
                "{} Created config file: {}",
                "✓".green(),
                config_path.display().to_string().cyan()
            );
            println!("\nSample rules:");
            println!(
                "  {} Invoices: *invoice*.pdf → Documents/Invoices/{{year}}",
                "•".dimmed()
            );
            println!(
                "  {} Screenshots: Screenshot*.png → Images/Screenshots/{{year}}-{{month}}",
                "•".dimmed()
            );
        }

        ConfigAction::Show { path } => {
            let config_path = path.unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(".neat")
                    .join("config.toml")
            });

            if !config_path.exists() {
                println!(
                    "{} Config file not found: {}",
                    "✗".red(),
                    config_path.display()
                );
                println!(
                    "  {} Run {} to create one",
                    "ℹ".blue(),
                    "neat config init".yellow()
                );
                return Ok(());
            }

            let config = NeatConfig::load(&config_path)?;

            println!(
                "{} Config: {}\n",
                "→".cyan(),
                config_path.display().to_string().bold()
            );

            if config.rules.is_empty() {
                println!("{}", "No rules defined.".yellow());
            } else {
                println!("{}", "Rules:".bold());
                println!("{}", "─".repeat(60));

                for rule in &config.rules {
                    println!(
                        "  {} {} (priority: {})",
                        "•".cyan(),
                        rule.name.bold(),
                        rule.priority
                    );
                    println!("    Pattern: {}", rule.pattern.yellow());
                    println!("    Dest:    {}", rule.destination.green());
                    println!();
                }
            }

            println!("{}", "Settings:".bold());
            println!("{}", "─".repeat(60));
            println!("  Include hidden: {}", config.settings.include_hidden);
            println!("  Follow symlinks: {}", config.settings.follow_symlinks);
            println!("  Default mode: {}", config.settings.default_organize_mode);
        }
    }

    Ok(())
}
