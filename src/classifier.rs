//! File classifier - categorize files by extension

use std::collections::HashMap;

/// File category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Images,
    Documents,
    Videos,
    Audio,
    Archives,
    Code,
    Data,
    Other,
}

impl Category {
    /// Get the folder name for this category
    pub fn folder_name(&self) -> &'static str {
        match self {
            Category::Images => "Images",
            Category::Documents => "Documents",
            Category::Videos => "Videos",
            Category::Audio => "Audio",
            Category::Archives => "Archives",
            Category::Code => "Code",
            Category::Data => "Data",
            Category::Other => "Other",
        }
    }

    /// Get all categories
    pub fn all() -> &'static [Category] {
        &[
            Category::Images,
            Category::Documents,
            Category::Videos,
            Category::Audio,
            Category::Archives,
            Category::Code,
            Category::Data,
            Category::Other,
        ]
    }
}

/// Classifier for file extensions
pub struct Classifier {
    extension_map: HashMap<String, Category>,
}

impl Default for Classifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Classifier {
    /// Create a new classifier with default mappings
    pub fn new() -> Self {
        let mut map = HashMap::new();

        // Images
        for ext in ["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff", "heic", "raw"] {
            map.insert(ext.to_string(), Category::Images);
        }

        // Documents
        for ext in ["pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx", "csv", "md", "epub"] {
            map.insert(ext.to_string(), Category::Documents);
        }

        // Videos
        for ext in ["mp4", "avi", "mov", "mkv", "wmv", "flv", "webm", "m4v", "mpeg", "mpg"] {
            map.insert(ext.to_string(), Category::Videos);
        }

        // Audio
        for ext in ["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus"] {
            map.insert(ext.to_string(), Category::Audio);
        }

        // Archives
        for ext in ["zip", "tar", "gz", "rar", "7z", "bz2", "xz", "tgz", "dmg", "iso"] {
            map.insert(ext.to_string(), Category::Archives);
        }

        // Code
        for ext in ["rs", "py", "js", "ts", "go", "java", "c", "cpp", "h", "hpp", "cs", "rb", "php", "swift", "kt", "scala", "html", "css", "scss", "vue", "jsx", "tsx", "sh", "bash", "zsh", "fish"] {
            map.insert(ext.to_string(), Category::Code);
        }

        // Data
        for ext in ["json", "xml", "yaml", "yml", "toml", "sql", "db", "sqlite"] {
            map.insert(ext.to_string(), Category::Data);
        }

        Classifier { extension_map: map }
    }

    /// Classify a file by its extension
    pub fn classify(&self, extension: Option<&str>) -> Category {
        match extension {
            Some(ext) => self
                .extension_map
                .get(&ext.to_lowercase())
                .copied()
                .unwrap_or(Category::Other),
            None => Category::Other,
        }
    }

    /// Get the category for a file extension
    pub fn get_category(&self, extension: &str) -> Category {
        self.classify(Some(extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_images() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("jpg")), Category::Images);
        assert_eq!(classifier.classify(Some("PNG")), Category::Images);
        assert_eq!(classifier.classify(Some("gif")), Category::Images);
    }

    #[test]
    fn test_classify_documents() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("pdf")), Category::Documents);
        assert_eq!(classifier.classify(Some("docx")), Category::Documents);
    }

    #[test]
    fn test_classify_unknown() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("xyz")), Category::Other);
        assert_eq!(classifier.classify(None), Category::Other);
    }
}
