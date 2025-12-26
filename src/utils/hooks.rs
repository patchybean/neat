//! Shell hook executor for post-action commands

use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// Execute a shell command with variable substitution
///
/// Supported variables:
/// - {file} - Full path to the source file
/// - {dest} - Full path to the destination file
/// - {name} - Filename without extension
/// - {ext} - File extension
/// - {dir} - Destination directory
pub fn execute_hook(command: &str, source: &Path, dest: &Path) -> Result<()> {
    let expanded = substitute_vars(command, source, dest);

    #[cfg(unix)]
    {
        Command::new("sh")
            .arg("-c")
            .arg(&expanded)
            .status()
            .with_context(|| format!("Failed to execute hook: {}", expanded))?;
    }

    #[cfg(windows)]
    {
        Command::new("cmd")
            .arg("/C")
            .arg(&expanded)
            .status()
            .with_context(|| format!("Failed to execute hook: {}", expanded))?;
    }

    Ok(())
}

/// Substitute variables in command string
fn substitute_vars(command: &str, source: &Path, dest: &Path) -> String {
    let file = source.to_string_lossy();
    let dest_str = dest.to_string_lossy();
    let name = source
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = source
        .extension()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let dir = dest
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    command
        .replace("{file}", &file)
        .replace("{dest}", &dest_str)
        .replace("{name}", &name)
        .replace("{ext}", &ext)
        .replace("{dir}", &dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_substitute_vars_basic() {
        let source = PathBuf::from("/home/user/document.pdf");
        let dest = PathBuf::from("/home/user/Documents/document.pdf");

        let cmd = "echo {file} -> {dest}";
        let result = substitute_vars(cmd, &source, &dest);

        assert!(result.contains("/home/user/document.pdf"));
        assert!(result.contains("/home/user/Documents/document.pdf"));
    }

    #[test]
    fn test_substitute_vars_name_ext() {
        let source = PathBuf::from("/tmp/report.pdf");
        let dest = PathBuf::from("/docs/report.pdf");

        let cmd = "process {name}.{ext}";
        let result = substitute_vars(cmd, &source, &dest);

        assert_eq!(result, "process report.pdf");
    }

    #[test]
    fn test_substitute_vars_dir() {
        let source = PathBuf::from("/tmp/file.txt");
        let dest = PathBuf::from("/archive/2024/file.txt");

        let cmd = "notify {dir}";
        let result = substitute_vars(cmd, &source, &dest);

        assert!(result.contains("/archive/2024"));
    }

    #[test]
    fn test_substitute_vars_no_extension() {
        let source = PathBuf::from("/tmp/Makefile");
        let dest = PathBuf::from("/docs/Makefile");

        let cmd = "moved {name}";
        let result = substitute_vars(cmd, &source, &dest);

        assert_eq!(result, "moved Makefile");
    }
}
