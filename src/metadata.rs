//! Metadata extraction for images and other files

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::{In, Reader, Tag};

/// EXIF metadata extracted from an image
#[derive(Debug, Clone, Default)]
pub struct ImageMetadata {
    /// Camera make (e.g., "Canon", "Sony", "Apple")
    pub camera_make: Option<String>,
    /// Camera model (e.g., "Canon EOS 5D", "iPhone 15 Pro")
    pub camera_model: Option<String>,
    /// Date/time when the photo was taken
    pub date_taken: Option<String>,
    /// GPS latitude
    #[allow(dead_code)]
    pub gps_latitude: Option<f64>,
    /// GPS longitude
    #[allow(dead_code)]
    pub gps_longitude: Option<f64>,
}

impl ImageMetadata {
    /// Extract EXIF metadata from an image file
    pub fn from_path(path: &Path) -> Option<Self> {
        let file = File::open(path).ok()?;
        let mut bufreader = BufReader::new(file);
        let exif = Reader::new().read_from_container(&mut bufreader).ok()?;

        let camera_make = exif
            .get_field(Tag::Make, In::PRIMARY)
            .map(|f| f.display_value().to_string().trim().to_string());

        let camera_model = exif
            .get_field(Tag::Model, In::PRIMARY)
            .map(|f| f.display_value().to_string().trim().to_string());

        let date_taken = exif
            .get_field(Tag::DateTimeOriginal, In::PRIMARY)
            .or_else(|| exif.get_field(Tag::DateTime, In::PRIMARY))
            .map(|f| f.display_value().to_string().trim().to_string());

        Some(ImageMetadata {
            camera_make,
            camera_model,
            date_taken,
            gps_latitude: None, // TODO: implement GPS extraction
            gps_longitude: None,
        })
    }

    /// Get a clean camera name for folder organization
    pub fn camera_folder_name(&self) -> Option<String> {
        // Try model first, then make
        if let Some(ref model) = self.camera_model {
            // Clean up the model name for folder use
            let clean = model
                .trim_matches('"')
                .replace(['/', '\\', ':', '*', '?', '<', '>', '|'], "_")
                .trim()
                .to_string();
            if !clean.is_empty() {
                return Some(clean);
            }
        }

        if let Some(ref make) = self.camera_make {
            let clean = make
                .trim_matches('"')
                .replace(['/', '\\', ':', '*', '?', '<', '>', '|'], "_")
                .trim()
                .to_string();
            if !clean.is_empty() {
                return Some(clean);
            }
        }

        None
    }

    /// Get date taken as YYYY/MM format for folder organization
    pub fn date_taken_folder(&self) -> Option<String> {
        let date_str = self.date_taken.as_ref()?;
        // EXIF date format is typically "YYYY:MM:DD HH:MM:SS"
        let clean = date_str.trim_matches('"');

        if clean.len() >= 10 {
            let parts: Vec<&str> = clean.split([':', ' ', '-']).collect();
            if parts.len() >= 2 {
                let year = parts[0];
                let month = parts[1];
                if year.len() == 4 && month.len() == 2 {
                    return Some(format!("{}/{}", year, month));
                }
            }
        }
        None
    }
}

/// Check if a file is a supported image format for EXIF extraction
pub fn is_exif_supported(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    matches!(
        ext.as_deref(),
        Some("jpg") | Some("jpeg") | Some("tiff") | Some("tif") | Some("heic") | Some("heif")
    )
}

/// Audio metadata extracted from music files
#[derive(Debug, Clone, Default)]
pub struct AudioMetadata {
    /// Artist name
    pub artist: Option<String>,
    /// Album name
    pub album: Option<String>,
    /// Track title
    #[allow(dead_code)]
    pub title: Option<String>,
    /// Genre
    #[allow(dead_code)]
    pub genre: Option<String>,
    /// Year
    #[allow(dead_code)]
    pub year: Option<u32>,
}

impl AudioMetadata {
    /// Extract audio metadata from a music file
    pub fn from_path(path: &Path) -> Option<Self> {
        use lofty::file::TaggedFileExt;
        use lofty::probe::Probe;
        use lofty::tag::Accessor;

        let tagged_file = Probe::open(path).ok()?.read().ok()?;
        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())?;

        Some(AudioMetadata {
            artist: tag.artist().map(|s| s.to_string()),
            album: tag.album().map(|s| s.to_string()),
            title: tag.title().map(|s| s.to_string()),
            genre: tag.genre().map(|s| s.to_string()),
            year: tag.year(),
        })
    }

    /// Get artist folder name for organization
    pub fn artist_folder_name(&self) -> Option<String> {
        self.artist
            .as_ref()
            .map(|a| {
                a.replace(['/', '\\', ':', '*', '?', '<', '>', '|'], "_")
                    .trim()
                    .to_string()
            })
            .filter(|s| !s.is_empty())
    }

    /// Get album folder name for organization
    pub fn album_folder_name(&self) -> Option<String> {
        self.album
            .as_ref()
            .map(|a| {
                a.replace(['/', '\\', ':', '*', '?', '<', '>', '|'], "_")
                    .trim()
                    .to_string()
            })
            .filter(|s| !s.is_empty())
    }
}

/// Check if a file is a supported audio format
pub fn is_audio_supported(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    matches!(
        ext.as_deref(),
        Some("mp3")
            | Some("flac")
            | Some("m4a")
            | Some("aac")
            | Some("ogg")
            | Some("wav")
            | Some("wma")
            | Some("opus")
            | Some("aiff")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_exif_supported() {
        assert!(is_exif_supported(Path::new("photo.jpg")));
        assert!(is_exif_supported(Path::new("photo.JPEG")));
        assert!(is_exif_supported(Path::new("photo.tiff")));
        assert!(!is_exif_supported(Path::new("photo.png")));
        assert!(!is_exif_supported(Path::new("document.pdf")));
    }

    #[test]
    fn test_date_taken_folder_parsing() {
        let mut meta = ImageMetadata::default();

        meta.date_taken = Some("\"2024:06:15 10:30:00\"".to_string());
        assert_eq!(meta.date_taken_folder(), Some("2024/06".to_string()));

        meta.date_taken = Some("2023:12:25 08:00:00".to_string());
        assert_eq!(meta.date_taken_folder(), Some("2023/12".to_string()));
    }
}
