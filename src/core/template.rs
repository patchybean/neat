//! Template engine for flexible file/folder naming
//!
//! Supports variables like: `{year}/{month}/{category}/{filename}`

use std::collections::HashMap;
use std::path::Path;

use chrono::{Datelike, Local};

use crate::classifier::Classifier;
use crate::scanner::FileInfo;
use crate::utils::metadata::{is_audio_supported, is_exif_supported, AudioMetadata, ImageMetadata};

/// Template engine for rendering destination paths
pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    /// Create a new template engine with variables from a file
    pub fn from_file(file: &FileInfo, classifier: &Classifier) -> Self {
        let mut variables = HashMap::new();

        // Basic file info
        let filename = Path::new(&file.name)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| file.name.clone());

        variables.insert("filename".to_string(), filename);
        variables.insert("name".to_string(), file.name.clone());
        variables.insert(
            "extension".to_string(),
            file.extension
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        );
        variables.insert(
            "ext".to_string(),
            file.extension
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        );

        // File size
        variables.insert("size".to_string(), file.size.to_string());
        variables.insert("size_kb".to_string(), (file.size / 1024).to_string());
        variables.insert(
            "size_mb".to_string(),
            (file.size / (1024 * 1024)).to_string(),
        );

        // Modified date
        if let Ok(duration) = file.modified.duration_since(std::time::UNIX_EPOCH) {
            let secs = duration.as_secs() as i64;
            if let Some(dt) = chrono::DateTime::from_timestamp(secs, 0) {
                let local: chrono::DateTime<Local> = dt.with_timezone(&Local);
                variables.insert("year".to_string(), local.year().to_string());
                variables.insert("month".to_string(), format!("{:02}", local.month()));
                variables.insert("day".to_string(), format!("{:02}", local.day()));
                variables.insert("date".to_string(), local.format("%Y-%m-%d").to_string());
            }
        }

        // Current date/time
        let now = Local::now();
        variables.insert("now.year".to_string(), now.year().to_string());
        variables.insert("now.month".to_string(), format!("{:02}", now.month()));
        variables.insert("now.day".to_string(), format!("{:02}", now.day()));
        variables.insert("now.date".to_string(), now.format("%Y-%m-%d").to_string());

        // Category
        let category = classifier.classify(file.extension.as_deref());
        variables.insert("category".to_string(), category.folder_name().to_string());
        variables.insert("type".to_string(), category.folder_name().to_string());

        // Try to get EXIF metadata for images
        if is_exif_supported(&file.path) {
            if let Some(meta) = ImageMetadata::from_path(&file.path) {
                if let Some(camera) = meta.camera_folder_name() {
                    variables.insert("camera".to_string(), camera);
                }
                if let Some(date_taken) = meta.date_taken_folder() {
                    variables.insert("date_taken".to_string(), date_taken.clone());
                    // Parse date_taken for year/month (format: YYYY/MM)
                    let parts: Vec<&str> = date_taken.split('/').collect();
                    if parts.len() >= 2 {
                        variables.insert("taken.year".to_string(), parts[0].to_string());
                        variables.insert("taken.month".to_string(), parts[1].to_string());
                    }
                }
            }
        }

        // Try to get audio metadata
        if is_audio_supported(&file.path) {
            if let Some(meta) = AudioMetadata::from_path(&file.path) {
                if let Some(artist) = meta.artist_folder_name() {
                    variables.insert("artist".to_string(), artist);
                }
                if let Some(album) = meta.album_folder_name() {
                    variables.insert("album".to_string(), album);
                }
            }
        }

        TemplateEngine { variables }
    }

    /// Create from a HashMap of variables
    pub fn new(variables: HashMap<String, String>) -> Self {
        TemplateEngine { variables }
    }

    /// Render a template string, replacing {variable} with values
    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();

        for (key, value) in &self.variables {
            let pattern = format!("{{{}}}", key);
            result = result.replace(&pattern, value);
        }

        // Remove any remaining unresolved variables (set to "Unknown")
        let re = regex::Regex::new(r"\{[^}]+\}").unwrap();
        result = re.replace_all(&result, "Unknown").to_string();

        // Clean up path: remove double slashes, trim
        result = result.replace("//", "/");
        result.trim_matches('/').to_string()
    }

    /// Get a variable value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Set a variable
    pub fn set(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    /// List all available variables
    pub fn list_variables(&self) -> Vec<(&String, &String)> {
        self.variables.iter().collect()
    }
}

/// Built-in template presets
pub fn get_preset_template(preset: &str) -> Option<&'static str> {
    match preset.to_lowercase().as_str() {
        "by-type" | "type" => Some("{category}/{filename}"),
        "by-date" | "date" => Some("{year}/{month}/{filename}"),
        "by-extension" | "extension" | "ext" => Some("{extension}/{filename}"),
        "by-camera" | "camera" => Some("{camera}/{filename}"),
        "by-date-taken" | "date-taken" => Some("{taken.year}/{taken.month}/{filename}"),
        "by-artist" | "artist" => Some("{artist}/{filename}"),
        "by-album" | "album" => Some("{artist}/{album}/{filename}"),
        "photos" => Some("{taken.year}/{taken.month}/{filename}"),
        "music" => Some("{artist}/{album}/{filename}"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic() {
        let mut vars = HashMap::new();
        vars.insert("year".to_string(), "2024".to_string());
        vars.insert("month".to_string(), "12".to_string());
        vars.insert("filename".to_string(), "photo".to_string());

        let engine = TemplateEngine::new(vars);
        let result = engine.render("{year}/{month}/{filename}");

        assert_eq!(result, "2024/12/photo");
    }

    #[test]
    fn test_render_missing_variable() {
        let vars = HashMap::new();
        let engine = TemplateEngine::new(vars);
        let result = engine.render("{missing}/test");

        assert_eq!(result, "Unknown/test");
    }

    #[test]
    fn test_render_cleans_path() {
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "".to_string());
        vars.insert("b".to_string(), "folder".to_string());

        let engine = TemplateEngine::new(vars);
        let result = engine.render("/{a}/{b}/");

        // Empty variable becomes Unknown, path cleaned
        assert!(result.contains("folder"));
    }

    #[test]
    fn test_preset_templates() {
        assert_eq!(
            get_preset_template("by-type"),
            Some("{category}/{filename}")
        );
        assert_eq!(
            get_preset_template("by-date"),
            Some("{year}/{month}/{filename}")
        );
        assert_eq!(get_preset_template("unknown"), None);
    }
}
