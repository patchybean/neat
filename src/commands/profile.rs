//! Profile system for saving and running CLI presets

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};

use crate::cli::ProfileAction;

/// A saved profile with organize command options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
    pub paths: Vec<PathBuf>,
    pub options: ProfileOptions,
}

/// Options that can be saved in a profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileOptions {
    pub by_type: bool,
    pub by_date: bool,
    pub by_extension: bool,
    pub by_camera: bool,
    pub by_date_taken: bool,
    pub by_artist: bool,
    pub by_album: bool,
    pub recursive: bool,
    pub copy: bool,
    pub on_conflict: String,
    pub min_size: Option<String>,
    pub max_size: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub startswith: Option<String>,
    pub endswith: Option<String>,
    pub contains: Option<String>,
    pub regex: Option<String>,
    pub mime: Option<String>,
    pub ignore: Vec<String>,
}

impl Profile {
    /// Get profiles directory
    fn profiles_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not find home directory")?;
        let dir = home.join(".neat").join("profiles");
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    /// Save profile to file
    pub fn save(&self) -> Result<()> {
        let dir = Self::profiles_dir()?;
        let path = dir.join(format!("{}.toml", self.name));

        let content = toml::to_string_pretty(self).context("Failed to serialize profile")?;

        let mut file = File::create(&path).context("Failed to create profile file")?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Load profile by name
    pub fn load(name: &str) -> Result<Self> {
        let dir = Self::profiles_dir()?;
        let path = dir.join(format!("{}.toml", name));

        if !path.exists() {
            anyhow::bail!("Profile '{}' not found", name);
        }

        let content = fs::read_to_string(&path).context("Failed to read profile file")?;

        let profile: Profile = toml::from_str(&content).context("Failed to parse profile")?;

        Ok(profile)
    }

    /// List all profiles
    pub fn list_all() -> Result<Vec<String>> {
        let dir = Self::profiles_dir()?;
        let mut profiles = Vec::new();

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "toml") {
                if let Some(name) = path.file_stem() {
                    profiles.push(name.to_string_lossy().to_string());
                }
            }
        }

        profiles.sort();
        Ok(profiles)
    }

    /// Delete profile
    pub fn delete(name: &str) -> Result<()> {
        let dir = Self::profiles_dir()?;
        let path = dir.join(format!("{}.toml", name));

        if !path.exists() {
            anyhow::bail!("Profile '{}' not found", name);
        }

        fs::remove_file(&path)?;
        Ok(())
    }
}

/// Run profile command
pub fn run(action: ProfileAction) -> Result<()> {
    match action {
        ProfileAction::Save {
            name,
            description,
            paths,
            by_type,
            by_date,
            by_extension,
            by_camera,
            by_date_taken,
            by_artist,
            by_album,
            recursive,
            copy,
            on_conflict,
            min_size,
            max_size,
            after,
            before,
            startswith,
            endswith,
            contains,
            regex,
            mime,
            ignore,
        } => {
            let profile = Profile {
                name: name.clone(),
                description,
                command: "organize".to_string(),
                paths,
                options: ProfileOptions {
                    by_type,
                    by_date,
                    by_extension,
                    by_camera,
                    by_date_taken,
                    by_artist,
                    by_album,
                    recursive,
                    copy,
                    on_conflict: on_conflict.unwrap_or_else(|| "rename".to_string()),
                    min_size,
                    max_size,
                    after,
                    before,
                    startswith,
                    endswith,
                    contains,
                    regex,
                    mime,
                    ignore,
                },
            };

            profile.save()?;
            println!("{} Saved profile '{}'", "✓".green(), name.bold());

            Ok(())
        }

        ProfileAction::List => {
            let profiles = Profile::list_all()?;

            if profiles.is_empty() {
                println!("{}", "No profiles saved yet.".yellow());
                println!(
                    "  Use {} to create one.",
                    "neatcli profile save <name> ...".cyan()
                );
            } else {
                println!("{}", "Saved profiles:".bold());
                for name in profiles {
                    if let Ok(profile) = Profile::load(&name) {
                        let desc = profile.description.unwrap_or_default();
                        let mode = get_mode_name(&profile.options);
                        println!(
                            "  {} {} {}",
                            "●".green(),
                            name.bold(),
                            format!("({})", mode).dimmed()
                        );
                        if !desc.is_empty() {
                            println!("    {}", desc.dimmed());
                        }
                    } else {
                        println!("  {} {}", "●".yellow(), name);
                    }
                }
            }

            Ok(())
        }

        ProfileAction::Run { name, dry_run } => {
            let profile = Profile::load(&name)?;

            println!("{} Running profile '{}'...", "→".cyan(), name.bold());

            run_profile(&profile, !dry_run)?;

            Ok(())
        }

        ProfileAction::Delete { name } => {
            Profile::delete(&name)?;
            println!("{} Deleted profile '{}'", "✓".green(), name.bold());
            Ok(())
        }

        ProfileAction::Show { name } => {
            let profile = Profile::load(&name)?;

            println!("{} {}", "Profile:".bold(), profile.name.cyan());
            if let Some(desc) = &profile.description {
                println!("  Description: {}", desc);
            }
            println!("  Command: {}", profile.command);
            println!("  Paths: {:?}", profile.paths);
            println!("  Mode: {}", get_mode_name(&profile.options));

            if profile.options.recursive {
                println!("  Recursive: yes");
            }
            if profile.options.copy {
                println!("  Copy mode: yes");
            }

            Ok(())
        }
    }
}

fn get_mode_name(options: &ProfileOptions) -> &str {
    if options.by_date {
        "by-date"
    } else if options.by_extension {
        "by-extension"
    } else if options.by_camera {
        "by-camera"
    } else if options.by_date_taken {
        "by-date-taken"
    } else if options.by_artist {
        "by-artist"
    } else if options.by_album {
        "by-album"
    } else {
        "by-type"
    }
}

fn run_profile(profile: &Profile, execute: bool) -> Result<()> {
    use crate::organizer::{
        execute_moves, plan_moves, preview_moves, print_results, ConflictStrategy, OrganizeMode,
    };
    use crate::scanner::{parse_date, parse_size, scan_directory, ScanOptions};

    let mode = if profile.options.by_date {
        OrganizeMode::ByDate
    } else if profile.options.by_extension {
        OrganizeMode::ByExtension
    } else if profile.options.by_camera {
        OrganizeMode::ByCamera
    } else if profile.options.by_date_taken {
        OrganizeMode::ByDateTaken
    } else if profile.options.by_artist {
        OrganizeMode::ByArtist
    } else if profile.options.by_album {
        OrganizeMode::ByAlbum
    } else {
        OrganizeMode::ByType
    };

    let conflict_strategy = match profile.options.on_conflict.as_str() {
        "skip" => ConflictStrategy::Skip,
        "overwrite" => ConflictStrategy::Overwrite,
        "ask" => ConflictStrategy::Ask,
        _ => ConflictStrategy::Rename,
    };

    let min_size = profile
        .options
        .min_size
        .as_ref()
        .map(|s| parse_size(s))
        .transpose()
        .map_err(|e| anyhow::anyhow!(e))?;

    let max_size = profile
        .options
        .max_size
        .as_ref()
        .map(|s| parse_size(s))
        .transpose()
        .map_err(|e| anyhow::anyhow!(e))?;

    let after_date = profile
        .options
        .after
        .as_ref()
        .map(|s| parse_date(s))
        .transpose()
        .map_err(|e| anyhow::anyhow!(e))?;

    let before_date = profile
        .options
        .before
        .as_ref()
        .map(|s| parse_date(s))
        .transpose()
        .map_err(|e| anyhow::anyhow!(e))?;

    for path in &profile.paths {
        let canonical = path
            .canonicalize()
            .with_context(|| format!("Path does not exist: {:?}", path))?;

        println!("  {} {}", "Scanning".dimmed(), canonical.display());

        let options = ScanOptions {
            include_hidden: false,
            max_depth: if profile.options.recursive {
                None
            } else {
                Some(1)
            },
            follow_symlinks: false,
            ignore_patterns: profile.options.ignore.clone(),
            min_size,
            max_size,
            after_date,
            before_date,
            name_startswith: profile.options.startswith.clone(),
            name_endswith: profile.options.endswith.clone(),
            name_contains: profile.options.contains.clone(),
            regex_pattern: profile.options.regex.clone(),
            mime_filter: profile.options.mime.clone(),
        };

        let files = scan_directory(&canonical, &options)?;
        let moves = plan_moves(&files, &canonical, mode);

        if moves.is_empty() {
            println!("  {}", "All files organized.".green());
            continue;
        }

        if execute {
            let cmd_name = format!("profile {}", profile.name);
            let result = execute_moves(&moves, &cmd_name, conflict_strategy)?;
            print_results(&result);
        } else {
            preview_moves(&moves, &canonical);
        }
    }

    Ok(())
}
