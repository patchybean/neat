//! Neat - A smart CLI tool to organize and clean up messy directories

mod cli;
mod commands;
mod config;
mod core;
mod tui;
mod utils;
mod watcher;

// Re-exports for convenience
pub use core::*;
pub use utils::*;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

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
            by_artist,
            by_album,
            dry_run,
            execute,
            ignore,
            min_size,
            max_size,
            after,
            before,
        } => {
            commands::organize::run(
                &path,
                by_type,
                by_date,
                by_extension,
                by_camera,
                by_date_taken,
                by_artist,
                by_album,
                dry_run,
                execute,
                cli.verbose,
                ignore,
                min_size,
                max_size,
                after,
                before,
            )?;
        }

        Commands::Clean {
            path,
            older_than,
            empty_folders,
            dry_run,
            execute,
            trash,
            min_size,
            max_size,
            after,
            before,
        } => {
            commands::clean::run(
                &path,
                older_than,
                empty_folders,
                dry_run,
                execute,
                trash,
                min_size,
                max_size,
                after,
                before,
            )?;
        }

        Commands::Duplicates {
            path,
            delete,
            dry_run,
            execute,
            trash,
            min_size,
            max_size,
            after,
            before,
            json,
            csv,
        } => {
            commands::duplicates::run(
                &path, delete, dry_run, execute, trash, min_size, max_size, after, before, json,
                csv,
            )?;
        }

        Commands::Similar {
            path,
            threshold,
            delete,
            dry_run,
            execute,
            trash,
        } => {
            commands::similar::run(&path, threshold, delete, dry_run, execute, trash)?;
        }

        Commands::Stats { path, json } => {
            commands::stats::run(&path, json)?;
        }

        Commands::Undo => {
            commands::undo::run()?;
        }

        Commands::History => {
            commands::history::run()?;
        }

        Commands::Watch {
            path,
            by_type,
            by_date,
            by_extension,
            config,
            auto,
        } => {
            commands::watch::run(&path, by_type, by_date, by_extension, config, auto)?;
        }

        Commands::Config { action } => {
            commands::config::run(action)?;
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
