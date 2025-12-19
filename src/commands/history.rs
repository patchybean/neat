//! History command handler

use anyhow::Result;
use colored::*;

use crate::logger::History;

/// Show operation history
pub fn run() -> Result<()> {
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
