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

    /// Shell command to execute after moving file (optional)
    /// Supports {file}, {dest}, {name}, {ext}, {dir} placeholders
    #[serde(default)]
    pub post_action: Option<String>,
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
                    post_action: Some("echo 'Organized invoice: {name}'".to_string()),
                },
                Rule {
                    name: "Screenshots".to_string(),
                    pattern: "Screenshot*.png".to_string(),
                    destination: "Images/Screenshots/{year}-{month}".to_string(),
                    priority: 5,
                    post_action: None,
                },
                Rule {
                    name: "Downloads by Month".to_string(),
                    pattern: "*".to_string(),
                    destination: "Archive/{year}/{month}".to_string(),
                    priority: 0,
                    post_action: None,
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
            post_action: None,
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
            post_action: None,
        };

        assert!(rule.matches("invoice_2024.pdf"));
        assert!(rule.matches("my_invoice.pdf"));
        assert!(!rule.matches("invoice.txt"));
    }

    #[test]
    fn test_rule_get_destination_simple() {
        let rule = Rule {
            name: "Test".to_string(),
            pattern: "*.pdf".to_string(),
            destination: "Documents".to_string(),
            priority: 0,
            post_action: None,
        };

        let base = PathBuf::from("/home/user");
        let result = rule.get_destination(&base, "file.pdf", Some("pdf"));
        assert_eq!(result, PathBuf::from("/home/user/Documents/file.pdf"));
    }

    #[test]
    fn test_rule_get_destination_with_ext_template() {
        let rule = Rule {
            name: "Test".to_string(),
            pattern: "*".to_string(),
            destination: "Files/{ext}".to_string(),
            priority: 0,
            post_action: None,
        };

        let base = PathBuf::from("/base");
        let result = rule.get_destination(&base, "doc.pdf", Some("pdf"));
        assert_eq!(result, PathBuf::from("/base/Files/pdf/doc.pdf"));
    }

    #[test]
    fn test_rule_get_destination_no_extension() {
        let rule = Rule {
            name: "Test".to_string(),
            pattern: "*".to_string(),
            destination: "Files/{ext}".to_string(),
            priority: 0,
            post_action: None,
        };

        let base = PathBuf::from("/base");
        let result = rule.get_destination(&base, "Makefile", None);
        assert_eq!(result, PathBuf::from("/base/Files/unknown/Makefile"));
    }

    #[test]
    fn test_get_sorted_rules_by_priority() {
        let config = Config {
            rules: vec![
                Rule {
                    name: "Low".to_string(),
                    pattern: "*".to_string(),
                    destination: "low".to_string(),
                    priority: 0,
                    post_action: None,
                },
                Rule {
                    name: "High".to_string(),
                    pattern: "*".to_string(),
                    destination: "high".to_string(),
                    priority: 10,
                    post_action: None,
                },
                Rule {
                    name: "Medium".to_string(),
                    pattern: "*".to_string(),
                    destination: "medium".to_string(),
                    priority: 5,
                    post_action: None,
                },
            ],
            settings: Settings::default(),
        };

        let sorted = config.get_sorted_rules();
        assert_eq!(sorted[0].name, "High");
        assert_eq!(sorted[1].name, "Medium");
        assert_eq!(sorted[2].name, "Low");
    }

    #[test]
    fn test_find_matching_rule() {
        let config = Config {
            rules: vec![
                Rule {
                    name: "PDFs".to_string(),
                    pattern: "*.pdf".to_string(),
                    destination: "Documents".to_string(),
                    priority: 5,
                    post_action: None,
                },
                Rule {
                    name: "Everything".to_string(),
                    pattern: "*".to_string(),
                    destination: "Other".to_string(),
                    priority: 0,
                    post_action: None,
                },
            ],
            settings: Settings::default(),
        };

        // PDF should match the PDF rule (higher priority)
        let pdf_rule = config.find_matching_rule("report.pdf");
        assert!(pdf_rule.is_some());
        assert_eq!(pdf_rule.unwrap().name, "PDFs");

        // Non-PDF should match the catchall
        let other_rule = config.find_matching_rule("image.png");
        assert!(other_rule.is_some());
        assert_eq!(other_rule.unwrap().name, "Everything");
    }

    #[test]
    fn test_find_matching_rule_none() {
        let config = Config {
            rules: vec![Rule {
                name: "PDFs only".to_string(),
                pattern: "*.pdf".to_string(),
                destination: "Documents".to_string(),
                priority: 0,
                post_action: None,
            }],
            settings: Settings::default(),
        };

        let result = config.find_matching_rule("image.png");
        assert!(result.is_none());
    }

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert!(!settings.include_hidden);
        assert!(!settings.follow_symlinks);
        assert_eq!(settings.default_organize_mode, "by-type");
    }
}
