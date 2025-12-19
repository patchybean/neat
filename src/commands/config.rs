//! Config command handler

use anyhow::Result;
use colored::*;

use crate::cli::ConfigAction;
use crate::config::Config as NeatConfig;

/// Manage configuration
pub fn run(action: ConfigAction) -> Result<()> {
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
