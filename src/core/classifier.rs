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
    #[allow(dead_code)]
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
        for ext in [
            "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff", "heic", "raw",
        ] {
            map.insert(ext.to_string(), Category::Images);
        }

        // Documents
        for ext in [
            "pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx", "csv", "md",
            "epub",
        ] {
            map.insert(ext.to_string(), Category::Documents);
        }

        // Videos
        for ext in [
            "mp4", "avi", "mov", "mkv", "wmv", "flv", "webm", "m4v", "mpeg", "mpg",
        ] {
            map.insert(ext.to_string(), Category::Videos);
        }

        // Audio
        for ext in ["mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus"] {
            map.insert(ext.to_string(), Category::Audio);
        }

        // Archives
        for ext in [
            "zip", "tar", "gz", "rar", "7z", "bz2", "xz", "tgz", "dmg", "iso",
        ] {
            map.insert(ext.to_string(), Category::Archives);
        }

        // Code
        for ext in [
            "rs", "py", "js", "ts", "go", "java", "c", "cpp", "h", "hpp", "cs", "rb", "php",
            "swift", "kt", "scala", "html", "css", "scss", "vue", "jsx", "tsx", "sh", "bash",
            "zsh", "fish",
        ] {
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
    #[allow(dead_code)]
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
        assert_eq!(classifier.classify(Some("webp")), Category::Images);
        assert_eq!(classifier.classify(Some("heic")), Category::Images);
    }

    #[test]
    fn test_classify_documents() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("pdf")), Category::Documents);
        assert_eq!(classifier.classify(Some("docx")), Category::Documents);
        assert_eq!(classifier.classify(Some("txt")), Category::Documents);
        assert_eq!(classifier.classify(Some("md")), Category::Documents);
        assert_eq!(classifier.classify(Some("epub")), Category::Documents);
    }

    #[test]
    fn test_classify_videos() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("mp4")), Category::Videos);
        assert_eq!(classifier.classify(Some("avi")), Category::Videos);
        assert_eq!(classifier.classify(Some("mkv")), Category::Videos);
        assert_eq!(classifier.classify(Some("mov")), Category::Videos);
        assert_eq!(classifier.classify(Some("webm")), Category::Videos);
    }

    #[test]
    fn test_classify_audio() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("mp3")), Category::Audio);
        assert_eq!(classifier.classify(Some("wav")), Category::Audio);
        assert_eq!(classifier.classify(Some("flac")), Category::Audio);
        assert_eq!(classifier.classify(Some("m4a")), Category::Audio);
        assert_eq!(classifier.classify(Some("opus")), Category::Audio);
    }

    #[test]
    fn test_classify_archives() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("zip")), Category::Archives);
        assert_eq!(classifier.classify(Some("tar")), Category::Archives);
        assert_eq!(classifier.classify(Some("rar")), Category::Archives);
        assert_eq!(classifier.classify(Some("7z")), Category::Archives);
        assert_eq!(classifier.classify(Some("dmg")), Category::Archives);
    }

    #[test]
    fn test_classify_code() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("rs")), Category::Code);
        assert_eq!(classifier.classify(Some("py")), Category::Code);
        assert_eq!(classifier.classify(Some("js")), Category::Code);
        assert_eq!(classifier.classify(Some("ts")), Category::Code);
        assert_eq!(classifier.classify(Some("go")), Category::Code);
        assert_eq!(classifier.classify(Some("html")), Category::Code);
    }

    #[test]
    fn test_classify_data() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("json")), Category::Data);
        assert_eq!(classifier.classify(Some("xml")), Category::Data);
        assert_eq!(classifier.classify(Some("yaml")), Category::Data);
        assert_eq!(classifier.classify(Some("toml")), Category::Data);
        assert_eq!(classifier.classify(Some("sql")), Category::Data);
    }

    #[test]
    fn test_classify_unknown() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("xyz")), Category::Other);
        assert_eq!(classifier.classify(Some("unknown")), Category::Other);
        assert_eq!(classifier.classify(None), Category::Other);
    }

    #[test]
    fn test_classify_case_insensitive() {
        let classifier = Classifier::new();
        assert_eq!(classifier.classify(Some("PDF")), Category::Documents);
        assert_eq!(classifier.classify(Some("Mp3")), Category::Audio);
        assert_eq!(classifier.classify(Some("JSON")), Category::Data);
    }

    #[test]
    fn test_category_folder_names() {
        assert_eq!(Category::Images.folder_name(), "Images");
        assert_eq!(Category::Documents.folder_name(), "Documents");
        assert_eq!(Category::Videos.folder_name(), "Videos");
        assert_eq!(Category::Audio.folder_name(), "Audio");
        assert_eq!(Category::Archives.folder_name(), "Archives");
        assert_eq!(Category::Code.folder_name(), "Code");
        assert_eq!(Category::Data.folder_name(), "Data");
        assert_eq!(Category::Other.folder_name(), "Other");
    }

    #[test]
    fn test_category_all() {
        let all = Category::all();
        assert_eq!(all.len(), 8);
        assert!(all.contains(&Category::Images));
        assert!(all.contains(&Category::Other));
    }
}
