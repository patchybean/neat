//! Watch command handler

use std::path::Path;

use anyhow::Result;

use crate::config::Config as NeatConfig;
use crate::organizer::OrganizeMode;
use crate::watcher;

/// Watch a directory and auto-organize new files
pub fn run(
    path: &Path,
    _by_type: bool,
    by_date: bool,
    by_extension: bool,
    config_path: Option<std::path::PathBuf>,
    auto: bool,
) -> Result<()> {
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
