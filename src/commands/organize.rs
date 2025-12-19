//! Organize command handler

use std::path::Path;

use anyhow::{Context, Result};
use colored::*;

use crate::organizer::{
    execute_copies, execute_moves, plan_moves, preview_moves, print_results, OrganizeMode,
};
use crate::scanner::{
    format_size, parse_date, parse_size, scan_directory, total_size, ScanOptions,
};

/// Organize files in a directory by type, date, extension, or metadata
#[allow(clippy::too_many_arguments)]
pub fn run(
    path: &Path,
    _by_type: bool,
    by_date: bool,
    by_extension: bool,
    by_camera: bool,
    by_date_taken: bool,
    by_artist: bool,
    by_album: bool,
    dry_run: bool,
    execute: bool,
    verbose: bool,
    ignore: Vec<String>,
    min_size: Option<String>,
    max_size: Option<String>,
    after: Option<String>,
    before: Option<String>,
    copy: bool,
    recursive: bool,
    startswith: Option<String>,
    endswith: Option<String>,
    contains: Option<String>,
    regex: Option<String>,
    mime: Option<String>,
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
    } else if by_artist {
        OrganizeMode::ByArtist
    } else if by_album {
        OrganizeMode::ByAlbum
    } else {
        OrganizeMode::ByType // Default
    };

    let mode_name = match mode {
        OrganizeMode::ByType => "type",
        OrganizeMode::ByDate => "date",
        OrganizeMode::ByExtension => "extension",
        OrganizeMode::ByCamera => "camera",
        OrganizeMode::ByDateTaken => "date taken",
        OrganizeMode::ByArtist => "artist",
        OrganizeMode::ByAlbum => "album",
    };

    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    let action = if copy { "copying" } else { "organizing" };
    let recursive_msg = if recursive { " (recursive)" } else { "" };

    println!(
        "{} Scanning {} ({} by {}{})...",
        "→".cyan(),
        canonical_path.display().to_string().bold(),
        action,
        mode_name.cyan(),
        recursive_msg
    );

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

    // Load ignore patterns from .neatignore file and CLI
    let mut ignore_patterns = crate::scanner::load_ignore_patterns(&canonical_path);
    ignore_patterns.extend(ignore);

    // Scan directory
    let options = ScanOptions {
        include_hidden: false,
        max_depth: if recursive { None } else { Some(1) },
        follow_symlinks: false,
        ignore_patterns,
        min_size: min_size_bytes,
        max_size: max_size_bytes,
        after_date,
        before_date,
        name_startswith: startswith,
        name_endswith: endswith,
        name_contains: contains,
        regex_pattern: regex,
        mime_filter: mime,
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
        if copy {
            let result = execute_copies(&moves, &format!("copy --by-{}", mode_name))?;
            print_results(&result);
        } else {
            let result = execute_moves(&moves, &format!("organize --by-{}", mode_name))?;
            print_results(&result);
        }
    } else {
        preview_moves(&moves, &canonical_path);
    }

    Ok(())
}
