//! Undo command handler

use std::fs;

use anyhow::Result;
use colored::*;

use crate::logger::{History, OperationType};

/// Undo the last operation
pub fn run() -> Result<()> {
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
