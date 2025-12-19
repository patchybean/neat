//! File filters - name, regex, MIME type filtering

use regex::Regex;
use std::path::Path;

/// Name-based file filters
#[derive(Debug, Default, Clone)]
pub struct NameFilter {
    /// File name must start with this string
    pub startswith: Option<String>,
    /// File name must end with this string (before extension)
    pub endswith: Option<String>,
    /// File name must contain this string
    pub contains: Option<String>,
    /// Case insensitive matching
    pub case_insensitive: bool,
}

impl NameFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a filename matches the filter
    pub fn matches(&self, filename: &str) -> bool {
        let name = if self.case_insensitive {
            filename.to_lowercase()
        } else {
            filename.to_string()
        };

        // Remove extension for matching
        let name_without_ext = Path::new(&name)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or(name.clone());

        if let Some(ref prefix) = self.startswith {
            let prefix = if self.case_insensitive {
                prefix.to_lowercase()
            } else {
                prefix.clone()
            };
            if !name_without_ext.starts_with(&prefix) {
                return false;
            }
        }

        if let Some(ref suffix) = self.endswith {
            let suffix = if self.case_insensitive {
                suffix.to_lowercase()
            } else {
                suffix.clone()
            };
            if !name_without_ext.ends_with(&suffix) {
                return false;
            }
        }

        if let Some(ref substr) = self.contains {
            let substr = if self.case_insensitive {
                substr.to_lowercase()
            } else {
                substr.clone()
            };
            if !name.contains(&substr) {
                return false;
            }
        }

        true
    }

    /// Check if filter is empty (no constraints)
    pub fn is_empty(&self) -> bool {
        self.startswith.is_none() && self.endswith.is_none() && self.contains.is_none()
    }
}

/// Check if a filename matches a regex pattern
pub fn matches_regex(filename: &str, pattern: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.is_match(filename))
}

/// Check if a file matches a MIME type filter
/// Supports wildcards like "image/*", "application/pdf"
pub fn matches_mime(path: &Path, mime_filter: &str) -> bool {
    let guess = mime_guess::from_path(path);

    if let Some(mime) = guess.first() {
        let mime_str = mime.to_string();

        // Handle wildcard patterns like "image/*"
        if let Some(prefix) = mime_filter.strip_suffix("/*") {
            return mime_str.starts_with(prefix);
        }

        // Exact match
        mime_str == mime_filter
    } else {
        false
    }
}

/// Get MIME type for a file path
pub fn get_mime_type(path: &Path) -> Option<String> {
    mime_guess::from_path(path).first().map(|m| m.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_filter_startswith() {
        let filter = NameFilter {
            startswith: Some("IMG_".to_string()),
            ..Default::default()
        };
        assert!(filter.matches("IMG_0001.jpg"));
        assert!(!filter.matches("photo.jpg"));
    }

    #[test]
    fn test_name_filter_endswith() {
        let filter = NameFilter {
            endswith: Some("_backup".to_string()),
            ..Default::default()
        };
        assert!(filter.matches("file_backup.txt"));
        assert!(!filter.matches("file.txt"));
    }

    #[test]
    fn test_name_filter_contains() {
        let filter = NameFilter {
            contains: Some("2024".to_string()),
            ..Default::default()
        };
        assert!(filter.matches("report_2024_01.pdf"));
        assert!(!filter.matches("report_2023.pdf"));
    }

    #[test]
    fn test_name_filter_case_insensitive() {
        let filter = NameFilter {
            startswith: Some("img_".to_string()),
            case_insensitive: true,
            ..Default::default()
        };
        assert!(filter.matches("IMG_0001.jpg"));
        assert!(filter.matches("img_0001.jpg"));
    }

    #[test]
    fn test_name_filter_combined() {
        let filter = NameFilter {
            startswith: Some("IMG_".to_string()),
            contains: Some("2024".to_string()),
            ..Default::default()
        };
        assert!(filter.matches("IMG_2024_0001.jpg"));
        assert!(!filter.matches("IMG_2023_0001.jpg"));
        assert!(!filter.matches("photo_2024.jpg"));
    }

    #[test]
    fn test_regex_matches() {
        assert!(matches_regex("IMG_0001.jpg", r"^IMG_\d{4}").unwrap());
        assert!(!matches_regex("photo.jpg", r"^IMG_\d{4}").unwrap());
    }

    #[test]
    fn test_mime_type_detection() {
        let path = Path::new("test.jpg");
        assert_eq!(get_mime_type(path), Some("image/jpeg".to_string()));

        let path = Path::new("test.pdf");
        assert_eq!(get_mime_type(path), Some("application/pdf".to_string()));
    }

    #[test]
    fn test_mime_filter_exact() {
        let path = Path::new("test.pdf");
        assert!(matches_mime(path, "application/pdf"));
        assert!(!matches_mime(path, "image/jpeg"));
    }

    #[test]
    fn test_mime_filter_wildcard() {
        let path = Path::new("test.jpg");
        assert!(matches_mime(path, "image/*"));
        assert!(!matches_mime(path, "application/*"));

        let path = Path::new("test.png");
        assert!(matches_mime(path, "image/*"));
    }
}
