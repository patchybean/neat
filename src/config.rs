//! Configuration and custom rules handling

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use chrono::{Datelike, Utc};
use glob::Pattern;
use serde::{Deserialize, Serialize};

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Custom organization rules
    #[serde(default)]
    pub rules: Vec<Rule>,

    /// Default settings
    #[serde(default)]
    pub settings: Settings,
}

/// Default settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Include hidden files
    #[serde(default)]
    pub include_hidden: bool,

    /// Follow symlinks
    #[serde(default)]
    pub follow_symlinks: bool,

    /// Default organize mode
    #[serde(default = "default_organize_mode")]
    pub default_organize_mode: String,
}

fn default_organize_mode() -> String {
    "by-type".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            include_hidden: false,
            follow_symlinks: false,
            default_organize_mode: default_organize_mode(),
        }
    }
}

/// A custom organization rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Rule name for display
    pub name: String,

    /// Glob pattern to match files (e.g., "*invoice*.pdf")
    pub pattern: String,

    /// Destination folder template (supports {year}, {month}, {day}, {ext})
    pub destination: String,

    /// Priority (higher = processed first)
    #[serde(default)]
    pub priority: i32,
}

impl Rule {
    /// Check if a filename matches this rule's pattern
    pub fn matches(&self, filename: &str) -> bool {
        Pattern::new(&self.pattern)
            .map(|p| p.matches(filename))
            .unwrap_or(false)
    }

    /// Get the destination path for a file
    pub fn get_destination(&self, base_path: &Path, filename: &str, ext: Option<&str>) -> PathBuf {
        let now = Utc::now();

        let dest = self
            .destination
            .replace("{year}", &now.year().to_string())
            .replace("{month}", &format!("{:02}", now.month()))
            .replace("{day}", &format!("{:02}", now.day()))
            .replace("{ext}", ext.unwrap_or("unknown"));

        base_path.join(dest).join(filename)
    }
}

impl Config {
    /// Load config from a TOML file
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;

        Ok(config)
    }

    /// Load config from default location (~/.neat/config.toml)
    pub fn load_default() -> Result<Option<Self>> {
        if let Some(home) = dirs::home_dir() {
            let config_path = home.join(".neat").join("config.toml");
            if config_path.exists() {
                return Ok(Some(Self::load(&config_path)?));
            }
        }
        Ok(None)
    }

    /// Get rules sorted by priority (highest first)
    pub fn get_sorted_rules(&self) -> Vec<&Rule> {
        let mut rules: Vec<_> = self.rules.iter().collect();
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        rules
    }

    /// Find matching rule for a filename
    pub fn find_matching_rule(&self, filename: &str) -> Option<&Rule> {
        self.get_sorted_rules()
            .into_iter()
            .find(|rule| rule.matches(filename))
    }

    /// Create a sample config file
    pub fn create_sample(path: &Path) -> Result<()> {
        let sample = Config {
            rules: vec![
                Rule {
                    name: "Invoices".to_string(),
                    pattern: "*invoice*.pdf".to_string(),
                    destination: "Documents/Invoices/{year}".to_string(),
                    priority: 10,
                },
                Rule {
                    name: "Screenshots".to_string(),
                    pattern: "Screenshot*.png".to_string(),
                    destination: "Images/Screenshots/{year}-{month}".to_string(),
                    priority: 5,
                },
                Rule {
                    name: "Downloads by Month".to_string(),
                    pattern: "*".to_string(),
                    destination: "Archive/{year}/{month}".to_string(),
                    priority: 0,
                },
            ],
            settings: Settings::default(),
        };

        let content =
            toml::to_string_pretty(&sample).context("Failed to serialize sample config")?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_matches() {
        let rule = Rule {
            name: "Test".to_string(),
            pattern: "*.pdf".to_string(),
            destination: "Documents".to_string(),
            priority: 0,
        };

        assert!(rule.matches("document.pdf"));
        assert!(rule.matches("invoice.pdf"));
        assert!(!rule.matches("image.png"));
    }

    #[test]
    fn test_rule_matches_complex() {
        let rule = Rule {
            name: "Invoices".to_string(),
            pattern: "*invoice*.pdf".to_string(),
            destination: "Documents/Invoices".to_string(),
            priority: 0,
        };

        assert!(rule.matches("invoice_2024.pdf"));
        assert!(rule.matches("my_invoice.pdf"));
        assert!(!rule.matches("invoice.txt"));
    }
}
