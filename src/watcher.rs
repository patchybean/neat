//! Watch mode - monitor directory for changes and auto-organize

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::{Context, Result};
use colored::*;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

use crate::classifier::Classifier;
use crate::config::Config as NeatConfig;
use crate::organizer::{execute_moves, plan_moves, ConflictStrategy, OrganizeMode, PlannedMove};
use crate::scanner::FileInfo;

/// Watch a directory and auto-organize new files
pub fn watch_directory(
    path: &Path,
    mode: OrganizeMode,
    config: Option<&NeatConfig>,
    auto_execute: bool,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    println!(
        "{} Watching {} for new files...",
        "👁".cyan(),
        canonical_path.display().to_string().bold()
    );
    println!("{}", "Press Ctrl+C to stop.".dimmed());
    println!();

    let (tx, rx) = channel();

    // Create a debouncer to avoid processing the same file multiple times
    let mut debouncer =
        new_debouncer(Duration::from_secs(2), tx).context("Failed to create file watcher")?;

    debouncer
        .watcher()
        .watch(&canonical_path, RecursiveMode::NonRecursive)
        .context("Failed to watch directory")?;

    let _classifier = Classifier::new();

    loop {
        match rx.recv() {
            Ok(Ok(events)) => {
                for event in events {
                    if event.kind == DebouncedEventKind::Any {
                        let file_path = &event.path;

                        // Skip directories and hidden files
                        if file_path.is_dir() {
                            continue;
                        }

                        if let Some(name) = file_path.file_name() {
                            if name.to_string_lossy().starts_with('.') {
                                continue;
                            }
                        }

                        // Skip if file no longer exists (was moved/deleted)
                        if !file_path.exists() {
                            continue;
                        }

                        // Get file info
                        if let Ok(file_info) = FileInfo::from_path(file_path) {
                            println!(
                                "{} New file detected: {}",
                                "→".cyan(),
                                file_info.name.bold()
                            );

                            // Check custom rules first
                            let destination = if let Some(cfg) = config {
                                if let Some(rule) = cfg.find_matching_rule(&file_info.name) {
                                    println!(
                                        "  {} Matched rule: {}",
                                        "✓".green(),
                                        rule.name.cyan()
                                    );
                                    Some(rule.get_destination(
                                        &canonical_path,
                                        &file_info.name,
                                        file_info.extension.as_deref(),
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            // Use standard organization if no rule matched
                            let moves = if let Some(dest) = destination {
                                vec![PlannedMove {
                                    from: file_info.path.clone(),
                                    to: dest,
                                    size: file_info.size,
                                }]
                            } else {
                                plan_moves(std::slice::from_ref(&file_info), &canonical_path, mode)
                            };

                            if moves.is_empty() {
                                println!("  {} Already organized", "✓".green());
                                continue;
                            }

                            let mv = &moves[0];
                            let dest_folder = mv
                                .to
                                .parent()
                                .map(|p| p.strip_prefix(&canonical_path).unwrap_or(p))
                                .map(|p| p.display().to_string())
                                .unwrap_or_default();

                            if auto_execute {
                                // Get the matched rule to check for post_action
                                let matched_rule = config.and_then(|cfg| cfg.find_matching_rule(&file_info.name));
                                
                                match execute_moves(&moves, "watch", ConflictStrategy::Rename) {
                                    Ok(_) => {
                                        println!(
                                            "  {} Moved to {}",
                                            "✓".green(),
                                            dest_folder.cyan()
                                        );
                                        
                                        // Execute post_action hook if configured
                                        if let Some(rule) = matched_rule {
                                            if let Some(ref hook_cmd) = rule.post_action {
                                                use crate::hooks::execute_hook;
                                                let mv = &moves[0];
                                                if let Err(e) = execute_hook(hook_cmd, &mv.from, &mv.to) {
                                                    println!("  {} Hook failed: {}", "⚠".yellow(), e);
                                                } else {
                                                    println!("  {} Hook executed", "⚡".cyan());
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("  {} Failed: {}", "✗".red(), e);
                                    }
                                }
                            } else {
                                println!(
                                    "  {} Would move to: {}",
                                    "→".yellow(),
                                    dest_folder.cyan()
                                );
                                println!(
                                    "    {} Add {} flag to auto-move files",
                                    "ℹ".blue(),
                                    "--auto".yellow()
                                );
                            }

                            println!();
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("{} Watch error: {:?}", "⚠".yellow(), e);
            }
            Err(e) => {
                eprintln!("{} Channel error: {:?}", "✗".red(), e);
                break;
            }
        }
    }

    Ok(())
}
