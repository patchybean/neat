//! Content-aware filtering for PDF and text files

use std::fs;
use std::path::Path;

use anyhow::Result;

/// Supported file types for content extraction
pub fn is_content_extractable(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    match ext.as_deref() {
        Some("txt") | Some("md") | Some("log") | Some("csv") | Some("json") | Some("xml") => true,
        #[cfg(feature = "pdf")]
        Some("pdf") => true,
        _ => false,
    }
}

/// Extract text content from a file
/// Supports plain text files and PDF (when pdf feature is enabled).
pub fn extract_text(path: &Path) -> Result<String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    // For plain text files, just read the content
    if is_plain_text(path) {
        let content = fs::read_to_string(path)?;
        return Ok(content);
    }

    // PDF extraction (when feature is enabled)
    #[cfg(feature = "pdf")]
    if ext.as_deref() == Some("pdf") {
        return extract_pdf_text(path);
    }

    // For unsupported types, return empty
    Ok(String::new())
}

/// Extract text from PDF file
#[cfg(feature = "pdf")]
fn extract_pdf_text(path: &Path) -> Result<String> {
    use anyhow::Context;

    let bytes = fs::read(path).with_context(|| format!("Failed to read PDF: {:?}", path))?;

    pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| anyhow::anyhow!("PDF extraction failed: {}", e))
}

/// Check if file content contains a pattern (case-insensitive)
pub fn matches_content(path: &Path, pattern: &str) -> bool {
    if !is_content_extractable(path) {
        return false;
    }

    match extract_text(path) {
        Ok(content) => content.to_lowercase().contains(&pattern.to_lowercase()),
        Err(_) => false,
    }
}

/// Check if file is plain text
fn is_plain_text(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    matches!(
        ext.as_deref(),
        Some("txt")
            | Some("md")
            | Some("log")
            | Some("csv")
            | Some("json")
            | Some("xml")
            | Some("yaml")
            | Some("yml")
            | Some("toml")
            | Some("ini")
            | Some("cfg")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_is_content_extractable() {
        assert!(is_content_extractable(Path::new("file.txt")));
        assert!(is_content_extractable(Path::new("readme.md")));
        assert!(is_content_extractable(Path::new("data.json")));
        assert!(!is_content_extractable(Path::new("image.png")));
        assert!(!is_content_extractable(Path::new("video.mp4")));

        // PDF should be extractable when pdf feature is enabled
        #[cfg(feature = "pdf")]
        assert!(is_content_extractable(Path::new("document.pdf")));
    }

    #[test]
    fn test_extract_text_from_txt() {
        let mut file = NamedTempFile::with_suffix(".txt").unwrap();
        writeln!(file, "Hello World").unwrap();
        writeln!(file, "This is a test file").unwrap();

        let content = extract_text(file.path()).unwrap();
        assert!(content.contains("Hello World"));
        assert!(content.contains("test file"));
    }

    #[test]
    fn test_matches_content() {
        let mut file = NamedTempFile::with_suffix(".txt").unwrap();
        writeln!(file, "Invoice #12345").unwrap();
        writeln!(file, "Amount: $100.00").unwrap();

        assert!(matches_content(file.path(), "invoice"));
        assert!(matches_content(file.path(), "INVOICE")); // case insensitive
        assert!(matches_content(file.path(), "12345"));
        assert!(!matches_content(file.path(), "receipt"));
    }

    #[test]
    fn test_matches_content_non_text_file() {
        // Non-extractable files should return false
        assert!(!matches_content(Path::new("image.png"), "test"));
    }
}
