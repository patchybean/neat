//! CLI definitions using clap

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Neat - A smart CLI tool to organize and clean up messy directories
#[derive(Parser)]
#[command(name = "neatcli")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Organize files by type or date
    Organize {
        /// Target directory to organize
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Organize files by their type (Images, Documents, Videos, etc.)
        #[arg(long, group = "organize_mode")]
        by_type: bool,

        /// Organize files by date (YYYY/MM structure)
        #[arg(long, group = "organize_mode")]
        by_date: bool,

        /// Organize files by extension
        #[arg(long, group = "organize_mode")]
        by_extension: bool,

        /// Preview changes without executing (default behavior)
        #[arg(long, short = 'n')]
        dry_run: bool,

        /// Actually execute the changes
        #[arg(long, short)]
        execute: bool,
    },

    /// Clean old files from a directory
    Clean {
        /// Target directory to clean
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Delete files older than duration (e.g., 30d, 7d, 1w)
        #[arg(long)]
        older_than: Option<String>,

        /// Remove empty folders
        #[arg(long)]
        empty_folders: bool,

        /// Preview changes without executing
        #[arg(long, short = 'n')]
        dry_run: bool,

        /// Actually execute the changes
        #[arg(long, short)]
        execute: bool,
    },

    /// Find duplicate files by content
    Duplicates {
        /// Target directory to scan
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Delete duplicates (keeps the first file in each group)
        #[arg(long)]
        delete: bool,

        /// Preview changes without executing
        #[arg(long, short = 'n')]
        dry_run: bool,

        /// Actually execute the changes
        #[arg(long, short)]
        execute: bool,
    },

    /// Show statistics about a directory
    Stats {
        /// Target directory to analyze
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Undo the last operation
    Undo,

    /// Show operation history
    History,

    /// Watch a directory and auto-organize new files
    Watch {
        /// Target directory to watch
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Organize files by their type (default)
        #[arg(long, group = "organize_mode")]
        by_type: bool,

        /// Organize files by date
        #[arg(long, group = "organize_mode")]
        by_date: bool,

        /// Organize files by extension
        #[arg(long, group = "organize_mode")]
        by_extension: bool,

        /// Use custom rules from config file
        #[arg(long, short)]
        config: Option<PathBuf>,

        /// Automatically move files (otherwise just preview)
        #[arg(long, short)]
        auto: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Launch interactive TUI mode
    Tui {
        /// Target directory to browse
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Create a sample configuration file
    Init {
        /// Path to create config file (default: ~/.neat/config.toml)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// Show current configuration
    Show {
        /// Path to config file
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}
