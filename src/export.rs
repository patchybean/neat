//! Export functionality for reports (JSON, CSV)

use serde::Serialize;
use std::io::Write;

use crate::duplicates::DuplicateGroup;

/// Serializable duplicate file for export
#[derive(Serialize)]
struct ExportFile {
    path: String,
    size: u64,
    modified: String,
}

/// Serializable duplicate group for export
#[derive(Serialize)]
struct ExportDuplicateGroup {
    hash: String,
    count: usize,
    wasted_space: u64,
    files: Vec<ExportFile>,
}

/// Export duplicates as JSON
pub fn export_duplicates_json<W: Write>(
    duplicates: &[DuplicateGroup],
    writer: &mut W,
) -> std::io::Result<()> {
    let groups: Vec<ExportDuplicateGroup> = duplicates
        .iter()
        .map(|g| {
            let files: Vec<ExportFile> = g
                .files
                .iter()
                .map(|f| ExportFile {
                    path: f.path.display().to_string(),
                    size: f.size,
                    modified: format!("{:?}", f.modified),
                })
                .collect();

            ExportDuplicateGroup {
                hash: g.hash.clone(),
                count: g.files.len(),
                wasted_space: g.wasted_space(),
                files,
            }
        })
        .collect();

    let json = serde_json::to_string_pretty(&groups)?;
    writeln!(writer, "{}", json)
}

/// Export duplicates as CSV
pub fn export_duplicates_csv<W: Write>(
    duplicates: &[DuplicateGroup],
    writer: &mut W,
) -> std::io::Result<()> {
    writeln!(writer, "group,hash,path,size")?;

    for (group_idx, group) in duplicates.iter().enumerate() {
        for file in &group.files {
            writeln!(
                writer,
                "{},{},{},{}",
                group_idx + 1,
                group.hash,
                file.path.display(),
                file.size
            )?;
        }
    }

    Ok(())
}

/// Serializable stats for export
#[derive(Serialize)]
pub struct ExportStats {
    pub total_files: usize,
    pub total_size: u64,
    pub categories: Vec<CategoryStats>,
}

#[derive(Serialize)]
pub struct CategoryStats {
    pub name: String,
    pub count: usize,
    pub size: u64,
}

/// Export stats as JSON
pub fn export_stats_json<W: Write>(stats: &ExportStats, writer: &mut W) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(stats)?;
    writeln!(writer, "{}", json)
}
