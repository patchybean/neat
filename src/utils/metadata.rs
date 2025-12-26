//! Metadata extraction for images and other files

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::{Exif, In, Reader, Tag, Value};

/// Extract GPS coordinate from EXIF data (latitude or longitude)
/// Converts from DMS (degrees/minutes/seconds) to decimal degrees
fn extract_gps_coordinate(exif: &Exif, coord_tag: Tag, ref_tag: Tag) -> Option<f64> {
    let coord_field = exif.get_field(coord_tag, In::PRIMARY)?;

    // GPS coordinates are stored as 3 rationals: [degrees, minutes, seconds]
    let rationals = match &coord_field.value {
        Value::Rational(v) if v.len() >= 3 => v,
        _ => return None,
    };

    let degrees = rationals[0].to_f64();
    let minutes = rationals[1].to_f64();
    let seconds = rationals[2].to_f64();

    // Convert DMS to decimal degrees
    let mut decimal = degrees + (minutes / 60.0) + (seconds / 3600.0);

    // Check reference (N/S for latitude, E/W for longitude)
    // South and West are negative
    if let Some(ref_field) = exif.get_field(ref_tag, In::PRIMARY) {
        let ref_value = ref_field.display_value().to_string();
        let ref_char = ref_value.trim().trim_matches('"').chars().next();
        if matches!(ref_char, Some('S') | Some('W')) {
            decimal = -decimal;
        }
    }

    Some(decimal)
}

/// EXIF metadata extracted from an image
#[derive(Debug, Clone, Default)]
pub struct ImageMetadata {
    /// Camera make (e.g., "Canon", "Sony", "Apple")
    pub camera_make: Option<String>,
    /// Camera model (e.g., "Canon EOS 5D", "iPhone 15 Pro")
    pub camera_model: Option<String>,
    /// Date/time when the photo was taken
    pub date_taken: Option<String>,
    /// GPS latitude (decimal degrees, negative for South)
    pub gps_latitude: Option<f64>,
    /// GPS longitude (decimal degrees, negative for West)
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

        // Extract GPS coordinates
        let gps_latitude = extract_gps_coordinate(&exif, Tag::GPSLatitude, Tag::GPSLatitudeRef);
        let gps_longitude = extract_gps_coordinate(&exif, Tag::GPSLongitude, Tag::GPSLongitudeRef);

        Some(ImageMetadata {
            camera_make,
            camera_model,
            date_taken,
            gps_latitude,
            gps_longitude,
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
        assert!(is_exif_supported(Path::new("photo.heic")));
        assert!(!is_exif_supported(Path::new("photo.png")));
        assert!(!is_exif_supported(Path::new("document.pdf")));
    }

    #[test]
    fn test_is_audio_supported() {
        assert!(is_audio_supported(Path::new("song.mp3")));
        assert!(is_audio_supported(Path::new("song.MP3")));
        assert!(is_audio_supported(Path::new("song.flac")));
        assert!(is_audio_supported(Path::new("song.m4a")));
        assert!(is_audio_supported(Path::new("song.opus")));
        assert!(is_audio_supported(Path::new("song.wav")));
        assert!(!is_audio_supported(Path::new("video.mp4")));
        assert!(!is_audio_supported(Path::new("document.pdf")));
    }

    #[test]
    fn test_date_taken_folder_parsing() {
        let meta1 = ImageMetadata {
            date_taken: Some("\"2024:06:15 10:30:00\"".to_string()),
            ..Default::default()
        };
        assert_eq!(meta1.date_taken_folder(), Some("2024/06".to_string()));

        let meta2 = ImageMetadata {
            date_taken: Some("2023:12:25 08:00:00".to_string()),
            ..Default::default()
        };
        assert_eq!(meta2.date_taken_folder(), Some("2023/12".to_string()));
    }

    #[test]
    fn test_date_taken_folder_none() {
        let meta = ImageMetadata::default();
        assert_eq!(meta.date_taken_folder(), None);
    }

    #[test]
    fn test_camera_folder_name_basic() {
        let meta = ImageMetadata {
            camera_model: Some("Canon EOS 5D".to_string()),
            ..Default::default()
        };
        assert_eq!(meta.camera_folder_name(), Some("Canon EOS 5D".to_string()));
    }

    #[test]
    fn test_camera_folder_name_with_quotes() {
        let meta = ImageMetadata {
            camera_model: Some("\"iPhone 15 Pro\"".to_string()),
            ..Default::default()
        };
        assert_eq!(meta.camera_folder_name(), Some("iPhone 15 Pro".to_string()));
    }

    #[test]
    fn test_camera_folder_name_sanitizes_special_chars() {
        let meta = ImageMetadata {
            camera_model: Some("Model/With:Special*Chars".to_string()),
            ..Default::default()
        };
        let result = meta.camera_folder_name().unwrap();
        assert!(!result.contains('/'));
        assert!(!result.contains(':'));
        assert!(!result.contains('*'));
    }

    #[test]
    fn test_camera_folder_name_falls_back_to_make() {
        let meta = ImageMetadata {
            camera_make: Some("Sony".to_string()),
            camera_model: None,
            ..Default::default()
        };
        assert_eq!(meta.camera_folder_name(), Some("Sony".to_string()));
    }

    #[test]
    fn test_camera_folder_name_none() {
        let meta = ImageMetadata::default();
        assert_eq!(meta.camera_folder_name(), None);
    }

    #[test]
    fn test_audio_artist_folder_name() {
        let meta = AudioMetadata {
            artist: Some("Taylor Swift".to_string()),
            ..Default::default()
        };
        assert_eq!(meta.artist_folder_name(), Some("Taylor Swift".to_string()));
    }

    #[test]
    fn test_audio_artist_folder_name_sanitizes() {
        let meta = AudioMetadata {
            artist: Some("AC/DC".to_string()),
            ..Default::default()
        };
        let result = meta.artist_folder_name().unwrap();
        assert!(!result.contains('/'));
    }

    #[test]
    fn test_audio_album_folder_name() {
        let meta = AudioMetadata {
            album: Some("1989".to_string()),
            ..Default::default()
        };
        assert_eq!(meta.album_folder_name(), Some("1989".to_string()));
    }

    #[test]
    fn test_audio_folder_name_empty_string() {
        let meta = AudioMetadata {
            artist: Some("   ".to_string()),
            album: Some("".to_string()),
            ..Default::default()
        };
        // Empty or whitespace-only should return None
        assert_eq!(meta.artist_folder_name(), None);
        assert_eq!(meta.album_folder_name(), None);
    }

    #[test]
    fn test_image_metadata_default() {
        let meta = ImageMetadata::default();
        assert!(meta.camera_make.is_none());
        assert!(meta.camera_model.is_none());
        assert!(meta.date_taken.is_none());
        assert!(meta.gps_latitude.is_none());
        assert!(meta.gps_longitude.is_none());
    }

    #[test]
    fn test_audio_metadata_default() {
        let meta = AudioMetadata::default();
        assert!(meta.artist.is_none());
        assert!(meta.album.is_none());
        assert!(meta.title.is_none());
        assert!(meta.genre.is_none());
        assert!(meta.year.is_none());
    }
}
