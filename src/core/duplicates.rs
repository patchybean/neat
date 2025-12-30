//! Duplicate detection using direct byte comparison (faster than hashing)

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::Mutex;

use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use memmap2::Mmap;
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

/// Chunk size for comparing large files (64KB)
const COMPARE_CHUNK_SIZE: usize = 64 * 1024;
/// Threshold for using memory-mapped files (files larger than this use mmap)
const MMAP_THRESHOLD: u64 = 64 * 1024; // 64KB

/// Find duplicate files by content using hybrid hash + direct compare
pub fn find_duplicates(files: &[FileInfo]) -> Result<Vec<DuplicateGroup>> {
    if files.is_empty() {
        return Ok(Vec::new());
    }

    // Step 1: Group by size (files with different sizes can't be duplicates)
    let mut by_size: HashMap<u64, Vec<&FileInfo>> = HashMap::new();
    for file in files {
        if file.size > 0 {
            by_size.entry(file.size).or_default().push(file);
        }
    }

    // Filter to only groups with potential duplicates (same size)
    let potential_dups: Vec<Vec<&FileInfo>> = by_size
        .into_values()
        .filter(|group| group.len() > 1)
        .collect();

    if potential_dups.is_empty() {
        return Ok(Vec::new());
    }

    let total_files: usize = potential_dups.iter().map(|g| g.len()).sum();
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Hashing files [{bar:40.cyan/blue}] {pos}/{len} ({per_sec})")
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Step 2: Quick hash first 4KB to group files (O(n) instead of O(n²))
    let files_flat: Vec<&FileInfo> = potential_dups.into_iter().flatten().collect();
    let by_quick_hash: Mutex<HashMap<String, Vec<&FileInfo>>> = Mutex::new(HashMap::new());

    files_flat.par_iter().for_each(|file| {
        if let Ok(hash) = quick_hash_4kb(&file.path) {
            let mut map = by_quick_hash.lock().unwrap();
            map.entry(hash).or_default().push(*file);
        }
        pb.inc(1);
    });

    pb.finish_and_clear();

    // Step 3: For groups with matching quick hash, verify with full comparison
    let quick_hash_groups = by_quick_hash.into_inner().unwrap();
    let candidates: Vec<Vec<&FileInfo>> = quick_hash_groups
        .into_values()
        .filter(|group| group.len() > 1)
        .collect();

    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    // Step 4: Direct compare within each candidate group (small groups, fast)
    let duplicates: Mutex<Vec<DuplicateGroup>> = Mutex::new(Vec::new());

    candidates.par_iter().for_each(|group| {
        if let Ok(groups) = find_duplicates_in_group(group) {
            let mut dups = duplicates.lock().unwrap();
            dups.extend(groups);
        }
    });

    Ok(duplicates.into_inner().unwrap())
}

/// Quick hash of first 4KB for fast grouping
fn quick_hash_4kb(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();
    
    // Include size in hash to differentiate same-prefix files
    let chunk_size = std::cmp::min(4096, size as usize);
    
    if size > MMAP_THRESHOLD {
        let mmap = unsafe { Mmap::map(&file)? };
        let hash = xxh3_64(&mmap[..chunk_size]);
        return Ok(format!("{:016x}_{}", hash, size));
    }
    
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0u8; chunk_size];
    reader.read_exact(&mut buffer)?;
    let hash = xxh3_64(&buffer);
    Ok(format!("{:016x}_{}", hash, size))
}

/// Find duplicates within a group of files with matching quick hash
fn find_duplicates_in_group(files: &[&FileInfo]) -> Result<Vec<DuplicateGroup>> {
    if files.len() < 2 {
        return Ok(Vec::new());
    }

    // For small groups (most common case), direct compare all pairs
    if files.len() == 2 {
        if files_are_equal(&files[0].path, &files[1].path).unwrap_or(false) {
            let hash = quick_hash(&files[0].path).unwrap_or_else(|_| "unknown".to_string());
            return Ok(vec![DuplicateGroup {
                hash,
                files: vec![files[0].clone(), files[1].clone()],
                size: files[0].size,
            }]);
        }
        return Ok(Vec::new());
    }

    // For larger groups, compare pairs
    let mut groups: Vec<Vec<FileInfo>> = Vec::new();
    let mut processed: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for i in 0..files.len() {
        if processed.contains(&i) {
            continue;
        }

        let mut current_group = vec![files[i].clone()];
        processed.insert(i);

        for j in (i + 1)..files.len() {
            if processed.contains(&j) {
                continue;
            }

            if files_are_equal(&files[i].path, &files[j].path).unwrap_or(false) {
                current_group.push(files[j].clone());
                processed.insert(j);
            }
        }

        if current_group.len() > 1 {
            groups.push(current_group);
        }
    }

    let result: Vec<DuplicateGroup> = groups
        .into_iter()
        .map(|files| {
            let size = files.first().map(|f| f.size).unwrap_or(0);
            let hash = quick_hash(&files[0].path).unwrap_or_else(|_| "unknown".to_string());
            DuplicateGroup { hash, files, size }
        })
        .collect();

    Ok(result)
}

/// Compare two files for equality using memory-mapped access (very fast)
fn files_are_equal(path1: &Path, path2: &Path) -> Result<bool> {
    let file1 = File::open(path1)?;
    let file2 = File::open(path2)?;

    let size1 = file1.metadata()?.len();
    let size2 = file2.metadata()?.len();

    // Different sizes = not equal
    if size1 != size2 {
        return Ok(false);
    }

    // Empty files are equal
    if size1 == 0 {
        return Ok(true);
    }

    // Use memory-mapped files for large files (much faster)
    if size1 > MMAP_THRESHOLD {
        // Safety: we're only reading, files are opened read-only
        let mmap1 = unsafe { Mmap::map(&file1)? };
        let mmap2 = unsafe { Mmap::map(&file2)? };
        
        // Direct byte comparison - very fast due to mmap
        return Ok(mmap1[..] == mmap2[..]);
    }

    // For small files, use buffered reading
    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);
    let mut buf1 = [0u8; COMPARE_CHUNK_SIZE];
    let mut buf2 = [0u8; COMPARE_CHUNK_SIZE];

    loop {
        let n1 = reader1.read(&mut buf1)?;
        let n2 = reader2.read(&mut buf2)?;

        if n1 != n2 {
            return Ok(false);
        }

        if n1 == 0 {
            return Ok(true);
        }

        if buf1[..n1] != buf2[..n2] {
            return Ok(false);
        }
    }
}

/// Quick hash for display purposes (not for comparison)
fn quick_hash(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();
    
    // For small files, hash entire content
    if size <= MMAP_THRESHOLD {
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        return Ok(format!("{:016x}", xxh3_64(&buffer)));
    }
    
    // For large files, hash first 64KB only (for display)
    let mmap = unsafe { Mmap::map(&file)? };
    let chunk_size = std::cmp::min(COMPARE_CHUNK_SIZE, mmap.len());
    let hash = xxh3_64(&mmap[..chunk_size]);
    Ok(format!("{:016x}", hash))
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

        let hash = quick_hash(&file_path).unwrap();

        // xxHash3 of "hello world" should be consistent
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16); // xxHash3 64-bit hex is 16 chars
    }
}
