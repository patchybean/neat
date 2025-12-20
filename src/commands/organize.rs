//! Organize command handler

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use colored::*;

use crate::organizer::{
    execute_copies, execute_moves, plan_moves, plan_moves_with_template, preview_moves,
    print_results, ConflictStrategy, OrganizeMode,
};
use crate::scanner::{
    format_size, parse_date, parse_size, scan_directory, total_size, ScanOptions,
};

/// Organize files in directories by type, date, extension, or metadata
#[allow(clippy::too_many_arguments)]
pub fn run(
    paths: &[PathBuf],
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
    template: Option<String>,
    on_conflict: ConflictStrategy,
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

    // Parse size filters once (shared across all paths)
    let min_size_bytes = min_size
        .map(|s| parse_size(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let max_size_bytes = max_size
        .map(|s| parse_size(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    // Parse date filters once (shared across all paths)
    let after_date = after
        .map(|s| parse_date(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    let before_date = before
        .map(|s| parse_date(&s))
        .transpose()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    // Process each path
    for path in paths {
        organize_single_path(
            path,
            mode,
            mode_name,
            dry_run,
            execute,
            verbose,
            &ignore,
            min_size_bytes,
            max_size_bytes,
            after_date,
            before_date,
            copy,
            recursive,
            startswith.clone(),
            endswith.clone(),
            contains.clone(),
            regex.clone(),
            mime.clone(),
            template.clone(),
            on_conflict,
        )?;
    }

    Ok(())
}

/// Process a single directory
#[allow(clippy::too_many_arguments)]
fn organize_single_path(
    path: &Path,
    mode: OrganizeMode,
    mode_name: &str,
    dry_run: bool,
    execute: bool,
    verbose: bool,
    ignore: &[String],
    min_size_bytes: Option<u64>,
    max_size_bytes: Option<u64>,
    after_date: Option<std::time::SystemTime>,
    before_date: Option<std::time::SystemTime>,
    copy: bool,
    recursive: bool,
    startswith: Option<String>,
    endswith: Option<String>,
    contains: Option<String>,
    regex: Option<String>,
    mime: Option<String>,
    template: Option<String>,
    on_conflict: ConflictStrategy,
) -> Result<()> {
    let canonical_path = path
        .canonicalize()
        .with_context(|| format!("Path does not exist: {:?}", path))?;

    let action = if copy { "copying" } else { "organizing" };
    let recursive_msg = if recursive { " (recursive)" } else { "" };

    let template_display = if let Some(ref t) = template {
        format!(" with template '{}'", t)
    } else {
        format!(" by {}", mode_name.cyan())
    };

    println!(
        "{} Scanning {} ({}{}{})...",
        "→".cyan(),
        canonical_path.display().to_string().bold(),
        action,
        template_display,
        recursive_msg
    );

    // Load ignore patterns from .neatignore file and CLI
    let mut ignore_patterns = crate::scanner::load_ignore_patterns(&canonical_path);
    ignore_patterns.extend(ignore.iter().cloned());

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

    // Plan moves - use template if provided, otherwise use mode
    let moves = if let Some(ref t) = template {
        plan_moves_with_template(&files, &canonical_path, t)
    } else {
        plan_moves(&files, &canonical_path, mode)
    };

    if moves.is_empty() {
        println!("{}", "All files are already organized.".green());
        return Ok(());
    }

    // Dry-run is default if --execute is not specified
    if execute && !dry_run {
        if copy {
            let result = execute_copies(&moves, &format!("copy --by-{}", mode_name), on_conflict)?;
            print_results(&result);
        } else {
            let result =
                execute_moves(&moves, &format!("organize --by-{}", mode_name), on_conflict)?;
            print_results(&result);
        }
    } else {
        preview_moves(&moves, &canonical_path);
    }

    Ok(())
}
