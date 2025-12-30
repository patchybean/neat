//! Duplicate detection using xxHash3 content hashing

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use xxhash_rust::xxh3::xxh3_64;

use crate::scanner::{format_size, FileInfo};

/// A group of duplicate files
#[derive(Debug)]
pub struct DuplicateGroup {
    #[allow(dead_code)]
    pub hash: String,
    pub files: Vec<FileInfo>,
    pub size: u64,
}

impl DuplicateGroup {
    /// Get the wasted space (all but one file)
    pub fn wasted_space(&self) -> u64 {
        if self.files.len() > 1 {
            self.size * (self.files.len() as u64 - 1)
        } else {
            0
        }
    }
}

/// Threshold for using partial hashing (files larger than this use partial hash first)
const PARTIAL_HASH_THRESHOLD: u64 = 1024 * 1024; // 1MB
/// Size of chunks to read from start and end for partial hashing
const PARTIAL_HASH_CHUNK_SIZE: usize = 512 * 1024; // 512KB

/// Find duplicate files by content
pub fn find_duplicates(files: &[FileInfo]) -> Result<Vec<DuplicateGroup>> {
    if files.is_empty() {
        return Ok(Vec::new());
    }

    // Step 1: Group by size (files with different sizes can't be duplicates)
    let mut by_size: HashMap<u64, Vec<&FileInfo>> = HashMap::new();
    for file in files {
        if file.size > 0 {
            // Skip empty files
            by_size.entry(file.size).or_default().push(file);
        }
    }

    // Filter to only groups with potential duplicates
    let potential_dups: Vec<_> = by_size
        .into_values()
        .filter(|group| group.len() > 1)
        .collect();

    if potential_dups.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2: For large files, use partial hashing first (first + last 512KB)
    let total_files: usize = potential_dups.iter().map(|g| g.len()).sum();

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Hashing files [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Flatten all files to hash
    let files_to_hash: Vec<&FileInfo> = potential_dups.into_iter().flatten().collect();

    // First pass: partial hash for large files, full hash for small files
    let by_partial_hash: Mutex<HashMap<String, Vec<&FileInfo>>> = Mutex::new(HashMap::new());

    files_to_hash.par_iter().for_each(|file| {
        let hash = if file.size > PARTIAL_HASH_THRESHOLD {
            hash_file_partial(&file.path)
        } else {
            hash_file_full(&file.path)
        };
        
        if let Ok(h) = hash {
            let mut map = by_partial_hash.lock().unwrap();
            map.entry(h).or_default().push(*file);
        }
        pb.inc(1);
    });

    pb.finish_and_clear();

    // Step 3: For large files with matching partial hashes, do full hash
    let partial_results = by_partial_hash.into_inner().unwrap();
    let by_hash: Mutex<HashMap<String, Vec<FileInfo>>> = Mutex::new(HashMap::new());

    // Files that need full hashing (large files with matching partial hashes)
    let needs_full_hash: Vec<_> = partial_results
        .iter()
        .filter(|(_, files)| files.len() > 1 && files[0].size > PARTIAL_HASH_THRESHOLD)
        .flat_map(|(_, files)| files.iter())
        .collect();

    // Small files already have full hashes, add them directly
    for (hash, files) in &partial_results {
        if files.len() > 1 && files[0].size <= PARTIAL_HASH_THRESHOLD {
            let mut map = by_hash.lock().unwrap();
            for file in files {
                map.entry(hash.clone()).or_default().push((*file).clone());
            }
        }
    }

    // Full hash the large files that had matching partial hashes
    if !needs_full_hash.is_empty() {
        let pb2 = ProgressBar::new(needs_full_hash.len() as u64);
        pb2.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} Verifying [{bar:40.cyan/blue}] {pos}/{len}")
                .unwrap()
                .progress_chars("█▓░"),
        );

        needs_full_hash.par_iter().for_each(|file| {
            if let Ok(hash) = hash_file_full(&file.path) {
                let mut map = by_hash.lock().unwrap();
                map.entry(hash).or_default().push((**file).clone());
            }
            pb2.inc(1);
        });

        pb2.finish_and_clear();
    }

    // Step 4: Build duplicate groups
    let by_hash = by_hash.into_inner().unwrap();
    let duplicates: Vec<DuplicateGroup> = by_hash
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .map(|(hash, files)| {
            let size = files.first().map(|f| f.size).unwrap_or(0);
            DuplicateGroup { hash, files, size }
        })
        .collect();

    Ok(duplicates)
}

/// Hash a file fully using xxHash3
fn hash_file_full(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let hash = xxh3_64(&buffer);
    Ok(format!("{:016x}", hash))
}

/// Hash only first and last 512KB of a file (for quick duplicate detection)
fn hash_file_partial(path: &Path) -> Result<String> {
    use std::io::Seek;
    use std::io::SeekFrom;

    let file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();

    // Read first chunk
    let first_chunk_size = std::cmp::min(PARTIAL_HASH_CHUNK_SIZE as u64, file_size) as usize;
    let mut first_chunk = vec![0u8; first_chunk_size];
    reader.read_exact(&mut first_chunk)?;
    buffer.extend_from_slice(&first_chunk);

    // Read last chunk (if file is large enough and not overlapping with first chunk)
    if file_size > PARTIAL_HASH_CHUNK_SIZE as u64 * 2 {
        let last_chunk_start = file_size - PARTIAL_HASH_CHUNK_SIZE as u64;
        reader.seek(SeekFrom::Start(last_chunk_start))?;
        let mut last_chunk = vec![0u8; PARTIAL_HASH_CHUNK_SIZE];
        reader.read_exact(&mut last_chunk)?;
        buffer.extend_from_slice(&last_chunk);
    }

    let hash = xxh3_64(&buffer);
    Ok(format!("p{:016x}", hash)) // prefix with 'p' to distinguish partial hashes
}

/// Display duplicate groups
pub fn display_duplicates(groups: &[DuplicateGroup]) {
    if groups.is_empty() {
        println!("{}", "No duplicate files found.".green());
        return;
    }

    let total_wasted: u64 = groups.iter().map(|g| g.wasted_space()).sum();
    let total_count: usize = groups.iter().map(|g| g.files.len() - 1).sum();

    println!("\n{}", "Duplicate Files Found:".bold().yellow());
    println!("{}", "─".repeat(60));

    for (i, group) in groups.iter().enumerate() {
        if i >= 10 {
            println!("\n... and {} more duplicate groups", groups.len() - 10);
            break;
        }

        println!(
            "\n  {} ({}) - {} copies:",
            format!("Group {}", i + 1).cyan().bold(),
            format_size(group.size).dimmed(),
            group.files.len()
        );

        for (j, file) in group.files.iter().enumerate() {
            let marker = if j == 0 {
                "●".green()
            } else {
                "○".yellow()
            };
            println!("    {} {}", marker, file.path.display());
        }
    }

    println!("\n{}", "─".repeat(60));
    println!(
        "\n{}: {} duplicate files in {} groups",
        "Summary".bold(),
        total_count.to_string().yellow(),
        groups.len().to_string().cyan()
    );
    println!(
        "{}: {} could be recovered by removing duplicates",
        "Wasted space".bold(),
        format_size(total_wasted).red()
    );
    println!(
        "\n{} Use {} to remove duplicates (keeps first file in each group).",
        "ℹ".blue(),
        "--delete --execute".yellow()
    );
}

/// A group of visually similar images
#[derive(Debug)]
pub struct SimilarGroup {
    /// Representative file (first in group)
    pub representative: FileInfo,
    /// Similar files
    pub similar: Vec<(FileInfo, u32)>, // (file, hamming distance)
}

impl SimilarGroup {
    /// Get total space used by similar files
    pub fn similar_space(&self) -> u64 {
        self.similar.iter().map(|(f, _)| f.size).sum()
    }
}

/// Check if a file is a supported image format for perceptual hashing
fn is_image_supported(path: &std::path::Path) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    matches!(
        ext.as_deref(),
        Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") | Some("webp")
    )
}

/// Find visually similar images using perceptual hashing
#[allow(clippy::needless_range_loop)]
pub fn find_similar_images(files: &[FileInfo], threshold: u32) -> Result<Vec<SimilarGroup>> {
    use image_hasher::{HashAlg, HasherConfig};

    // Filter to only image files
    let images: Vec<&FileInfo> = files
        .iter()
        .filter(|f| is_image_supported(&f.path))
        .collect();

    if images.len() < 2 {
        return Ok(Vec::new());
    }

    println!(
        "  {} Calculating perceptual hashes for {} images (parallel)...",
        "→".cyan(),
        images.len()
    );

    let pb = ProgressBar::new(images.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} Hashing images [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})",
            )
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Configure hasher with DCT algorithm (good for finding similar images)
    let hasher = HasherConfig::new()
        .hash_alg(HashAlg::DoubleGradient)
        .hash_size(16, 16)
        .to_hasher();

    // Calculate hashes for all images in parallel
    let hashes: Vec<(&FileInfo, Option<image_hasher::ImageHash>)> = images
        .par_iter()
        .map(|file| {
            pb.inc(1);
            let hash = image::open(&file.path)
                .ok()
                .map(|img| hasher.hash_image(&img));
            (*file, hash)
        })
        .collect();

    pb.finish_and_clear();

    // Find similar images
    let mut groups: Vec<SimilarGroup> = Vec::new();
    let mut used: std::collections::HashSet<usize> = std::collections::HashSet::new();

    println!(
        "  {} Comparing {} image pairs...",
        "→".cyan(),
        images.len() * (images.len() - 1) / 2
    );

    for i in 0..hashes.len() {
        if used.contains(&i) {
            continue;
        }

        let (file_i, hash_i) = &hashes[i];
        let hash_i = match hash_i {
            Some(h) => h,
            None => continue,
        };

        let mut similar: Vec<(FileInfo, u32)> = Vec::new();

        for j in (i + 1)..hashes.len() {
            if used.contains(&j) {
                continue;
            }

            let (file_j, hash_j) = &hashes[j];
            let hash_j = match hash_j {
                Some(h) => h,
                None => continue,
            };

            let distance = hash_i.dist(hash_j);

            if distance <= threshold {
                similar.push(((*file_j).clone(), distance));
                used.insert(j);
            }
        }

        if !similar.is_empty() {
            used.insert(i);
            groups.push(SimilarGroup {
                representative: (*file_i).clone(),
                similar,
            });
        }
    }

    Ok(groups)
}

/// Display similar image groups
pub fn display_similar_images(groups: &[SimilarGroup]) {
    if groups.is_empty() {
        println!("{}", "No similar images found.".green());
        return;
    }

    let total_similar: usize = groups.iter().map(|g| g.similar.len()).sum();
    let total_space: u64 = groups.iter().map(|g| g.similar_space()).sum();

    println!("\n{}", "Similar Images Found:".bold().yellow());
    println!("{}", "─".repeat(60));

    for (i, group) in groups.iter().enumerate() {
        if i >= 10 {
            println!("\n... and {} more similar image groups", groups.len() - 10);
            break;
        }

        println!(
            "\n  {} ({} similar):",
            format!("Group {}", i + 1).cyan().bold(),
            group.similar.len()
        );

        // Show representative (keep this one)
        println!(
            "    {} {} ({})",
            "●".green(),
            group.representative.path.display(),
            format_size(group.representative.size).dimmed()
        );

        // Show similar files
        for (file, distance) in &group.similar {
            println!(
                "    {} {} ({}, {}% similar)",
                "○".yellow(),
                file.path.display(),
                format_size(file.size).dimmed(),
                100 - (distance * 100 / 256).min(100)
            );
        }
    }

    println!("\n{}", "─".repeat(60));
    println!(
        "\n{}: {} similar images in {} groups",
        "Summary".bold(),
        total_similar.to_string().yellow(),
        groups.len().to_string().cyan()
    );
    println!(
        "{}: {} used by similar images",
        "Space".bold(),
        format_size(total_space).yellow()
    );
    println!(
        "\n{} Lower threshold = more strict matching (default: 10)",
        "ℹ".blue()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use tempfile::tempdir;

    fn make_file_info(path: PathBuf, size: u64) -> FileInfo {
        FileInfo {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            extension: path.extension().map(|e| e.to_string_lossy().to_string()),
            path,
            size,
            modified: SystemTime::now(),
            created: None,
        }
    }

    #[test]
    fn test_wasted_space_single_file() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![make_file_info(PathBuf::from("/a.txt"), 100)],
            size: 100,
        };
        assert_eq!(group.wasted_space(), 0);
    }

    #[test]
    fn test_wasted_space_two_files() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![
                make_file_info(PathBuf::from("/a.txt"), 100),
                make_file_info(PathBuf::from("/b.txt"), 100),
            ],
            size: 100,
        };
        assert_eq!(group.wasted_space(), 100); // 1 duplicate
    }

    #[test]
    fn test_wasted_space_three_files() {
        let group = DuplicateGroup {
            hash: "abc".to_string(),
            files: vec![
                make_file_info(PathBuf::from("/a.txt"), 500),
                make_file_info(PathBuf::from("/b.txt"), 500),
                make_file_info(PathBuf::from("/c.txt"), 500),
            ],
            size: 500,
        };
        assert_eq!(group.wasted_space(), 1000); // 2 duplicates * 500
    }

    #[test]
    fn test_find_duplicates_empty() {
        let files: Vec<FileInfo> = vec![];
        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_duplicates_no_duplicates() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");

        let mut f1 = File::create(&file1).unwrap();
        write!(f1, "content A").unwrap();

        let mut f2 = File::create(&file2).unwrap();
        write!(f2, "content B").unwrap();

        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];

        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty()); // Different content, no duplicates
    }

    #[test]
    fn test_find_duplicates_with_duplicates() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");

        let mut f1 = File::create(&file1).unwrap();
        write!(f1, "same content").unwrap();

        let mut f2 = File::create(&file2).unwrap();
        write!(f2, "same content").unwrap();

        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];

        let result = find_duplicates(&files).unwrap();
        assert_eq!(result.len(), 1); // One duplicate group
        assert_eq!(result[0].files.len(), 2);
    }

    #[test]
    fn test_find_duplicates_empty_files_skipped() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("empty1.txt");
        let file2 = dir.path().join("empty2.txt");

        File::create(&file1).unwrap(); // Empty file
        File::create(&file2).unwrap(); // Empty file

        let files = vec![
            FileInfo::from_path(&file1).unwrap(),
            FileInfo::from_path(&file2).unwrap(),
        ];

        let result = find_duplicates(&files).unwrap();
        assert!(result.is_empty()); // Empty files are skipped
    }

    #[test]
    fn test_hash_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        write!(file, "hello world").unwrap();

        let hash = hash_file_full(&file_path).unwrap();

        // xxHash3 of "hello world" should be consistent
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16); // xxHash3 64-bit hex is 16 chars
    }
}
