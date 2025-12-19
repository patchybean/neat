//! Stats command handler

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use colored::*;

use crate::classifier::Classifier;
use crate::export;
use crate::scanner::{format_size, scan_directory, total_size, ScanOptions};

/// Show statistics about a directory
pub fn run(path: &Path, json: bool) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    if !json {
        println!(
            "{} Analyzing {}...\n",
            "→".cyan(),
            canonical_path.display().to_string().bold()
        );
    }

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

    if files.is_empty() {
        if json {
            println!("{{\"total_files\": 0, \"total_size\": 0, \"categories\": []}}");
        } else {
            println!("{}", "No files found.".yellow());
        }
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

    // Handle JSON export
    if json {
        let stats = export::ExportStats {
            total_files: files.len(),
            total_size: total_size(&files),
            categories: categories
                .iter()
                .map(|(name, (count, size))| export::CategoryStats {
                    name: name.clone(),
                    count: *count,
                    size: *size,
                })
                .collect(),
        };
        export::export_stats_json(&stats, &mut std::io::stdout())?;
        return Ok(());
    }

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
