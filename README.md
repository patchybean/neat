# 🧹 neatcli

[![CI](https://github.com/patchybean/neatcli/actions/workflows/ci.yml/badge.svg)](https://github.com/patchybean/neatcli/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/patchybean/neatcli)](https://github.com/patchybean/neatcli/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A smart CLI tool to organize and clean up messy directories, built in Rust.

## ✨ Features

- **Organize by Type** - Automatically sort files into folders: Images, Documents, Videos, Audio, Archives, Code, Data
- **Organize by Date** - Create YYYY/MM folder structure based on file dates
- **Organize by Extension** - Group files by their extensions
- **EXIF Metadata** - Organize photos by camera model or date taken (from EXIF data)
- **Audio Metadata** - Organize music by artist or album (from ID3/audio tags)
- **Find Duplicates** - Detect duplicate files using SHA256 content hashing
- **Find Similar Images** - Detect visually similar images using perceptual hashing
- **Clean Old Files** - Remove files older than a specified duration
- **Watch Mode** - Auto-organize new files as they appear
- **Custom Rules** - Define your own organization rules via TOML config
- **Undo Operations** - Rollback your last operation
- **Interactive TUI** - Visual file browser with keyboard navigation
- **Trash Support** - Move files to system trash instead of permanent deletion
- **Shell Completions** - Tab completion for Bash, Zsh, Fish, PowerShell
- **Ignore Patterns** - Skip files via `.neatignore` or `-I` flag
- **Safe by Default** - Dry-run mode lets you preview changes before executing

## 📦 Installation

### Using Homebrew

```bash
brew tap patchybean/tap
brew install neatcli
```

### Using Cargo

```bash
cargo install neatcli
```

### From source

```bash
git clone https://github.com/patchybean/neatcli
cd neatcli
cargo build --release
sudo cp target/release/neatcli /usr/local/bin/
```

## 🚀 Usage

### Organize Files

```bash
# Organize by type (preview only - safe)
neatcli organize ~/Downloads --by-type

# Actually execute the organization
neatcli organize ~/Downloads --by-type --execute

# Organize by date (YYYY/MM structure)
neatcli organize ~/Downloads --by-date --execute

# Organize by extension
neatcli organize ~/Downloads --by-extension --execute

# Organize photos by camera model (from EXIF data)
neatcli organize ~/Photos --by-camera --execute
# Creates: Canon EOS 5D/, iPhone 15 Pro/, etc.

# Organize photos by date taken (from EXIF, more accurate)
neatcli organize ~/Photos --by-date-taken --execute

# Organize music by artist
neatcli organize ~/Music --by-artist --execute
# Creates: Taylor Swift/, Ed Sheeran/, etc.

# Organize music by album (Artist/Album structure)
neatcli organize ~/Music --by-album --execute
# Creates: Taylor Swift/1989/, Ed Sheeran/Divide/, etc.

# Ignore specific patterns
neatcli organize ~/Downloads --by-type -I "*.log" -I "temp_*" --execute
```

### Find Duplicates

```bash
# Find duplicate files
neatcli duplicates ~/Downloads

# Find and delete duplicates
neatcli duplicates ~/Downloads --delete --execute

# Move duplicates to trash instead
neatcli duplicates ~/Downloads --delete --trash --execute
```

### Find Similar Images

```bash
# Find visually similar images (perceptual hashing)
neatcli similar ~/Photos

# Use stricter matching (lower threshold = more strict)
neatcli similar ~/Photos --threshold 5

# Delete similar images (keeps first in each group)
neatcli similar ~/Photos --delete --execute

# Move similar images to trash
neatcli similar ~/Photos --delete --trash --execute
```

### Filter by Size

You can filter files by size in `organize`, `clean`, and `duplicates` commands using `--min-size` and `--max-size`.

Supported units: B, KB, MB, GB, TB

```bash
# Organize only large files (> 1GB)
neatcli organize ~/Downloads --min-size 1GB

# Clean small log files (< 10KB)
neatcli clean ~/Logs --max-size 10KB --exec

# Find duplicates among large videos only
neatcli duplicates ~/Movies --min-size 500MB
```

### Filter by Date

Filter files by modification date using `--after` and `--before` flags (format: YYYY-MM-DD).

```bash
# Organize only files modified after 2024-01-01
neatcli organize ~/Downloads --after 2024-01-01

# Clean files modified before 2024-06-01
neatcli clean ~/Logs --before 2024-06-01 --execute

# Find duplicates in a specific date range
neatcli duplicates ~/Photos --after 2024-01-01 --before 2024-12-31
```

### Clean Old Files

```bash
# Preview files older than 30 days
neatcli clean ~/Downloads --older-than 30d

# Delete files older than 7 days
neatcli clean ~/Downloads --older-than 7d --execute

# Move old files to trash instead of permanent deletion
neatcli clean ~/Downloads --older-than 7d --trash --execute

# Remove empty folders
neatcli clean ~/Downloads --empty-folders --execute
```

### Statistics

```bash
# Show directory statistics
neatcli stats ~/Downloads

# Export stats as JSON
neatcli stats ~/Downloads --json
```

### Export Formats

```bash
# Export duplicates as JSON
neatcli duplicates ~/Downloads --json

# Export duplicates as CSV
neatcli duplicates ~/Downloads --csv
```

### Undo

```bash
# Undo the last operation
neatcli undo

# View operation history
neatcli history
```

### Watch Mode (Auto-organize)

```bash
# Watch directory for new files (preview mode)
neatcli watch ~/Downloads

# Auto-organize new files
neatcli watch ~/Downloads --auto

# Use custom rules
neatcli watch ~/Downloads --config ~/.neat/config.toml --auto
```

### Custom Rules

```bash
# Create a sample config file
neatcli config init

# Show current configuration
neatcli config show
```

Sample `~/.neat/config.toml`:
```toml
[[rules]]
name = "Invoices"
pattern = "*invoice*.pdf"
destination = "Documents/Invoices/{year}"
priority = 10

[[rules]]
name = "Screenshots"
pattern = "Screenshot*.png"
destination = "Images/Screenshots/{year}-{month}"
priority = 5
```

### Shell Completions

```bash
# Generate completions for your shell
neatcli completions bash > ~/.local/share/bash-completion/completions/neatcli
neatcli completions zsh > ~/.zfunc/_neatcli
neatcli completions fish > ~/.config/fish/completions/neatcli.fish

# PowerShell
neatcli completions powershell > _neatcli.ps1
```

### Ignore Patterns

Create a `.neatignore` file in any directory to skip files during scanning:

```bash
# .neatignore example
*.log
*.tmp
temp_*
node_modules
.git
```

Or use the `-I` flag on command line:
```bash
neatcli organize . -I "*.log" -I "backup_*" --execute
```

## 🛡️ Safety Features

1. **Dry-run by default** - All operations preview changes first
2. **Confirmation prompts** - Destructive operations require confirmation
3. **Undo capability** - Rollback file moves with `neatcli undo`
4. **Operation logging** - All operations are logged to `~/.neat/history.json`

## 📁 File Categories

| Category   | Extensions |
|------------|------------|
| Images     | jpg, jpeg, png, gif, bmp, svg, webp, ico, tiff, heic, raw |
| Documents  | pdf, doc, docx, txt, rtf, odt, xls, xlsx, ppt, pptx, csv, md, epub |
| Videos     | mp4, avi, mov, mkv, wmv, flv, webm, m4v, mpeg, mpg |
| Audio      | mp3, wav, flac, aac, ogg, wma, m4a, opus |
| Archives   | zip, tar, gz, rar, 7z, bz2, xz, tgz, dmg, iso |
| Code       | rs, py, js, ts, go, java, c, cpp, h, hpp, cs, rb, php, swift, kt, scala, html, css, scss, vue, jsx, tsx, sh, bash, zsh, fish |
| Data       | json, xml, yaml, yml, toml, sql, db, sqlite |

## 📋 Command Reference

```
neatcli [OPTIONS] <COMMAND>

Commands:
  organize    Organize files by type or date
  clean       Clean old files from a directory
  duplicates  Find duplicate files by content
  stats       Show statistics about a directory
  undo        Undo the last operation
  history     Show operation history
  watch       Watch directory and auto-organize new files
  config      Manage configuration (init, show)
  tui         Interactive TUI file browser
  completions Generate shell completions
  help        Print help

Options:
  -v, --verbose  Enable verbose output
  -q, --quiet    Suppress all output except errors
  -h, --help     Print help
  -V, --version  Print version
```

## 🔧 Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_BACKTRACE=1 cargo run -- organize . --by-type
```

## 📄 License

MIT © Patchy Bean
