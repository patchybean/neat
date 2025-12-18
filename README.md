# 🧹 Neat

A smart CLI tool to organize and clean up messy directories, built in Rust.

## ✨ Features

- **Organize by Type** - Automatically sort files into folders: Images, Documents, Videos, Audio, Archives, Code, Data
- **Organize by Date** - Create YYYY/MM folder structure based on file dates
- **Organize by Extension** - Group files by their extensions
- **Find Duplicates** - Detect duplicate files using SHA256 content hashing
- **Clean Old Files** - Remove files older than a specified duration
- **Undo Operations** - Rollback your last operation
- **Safe by Default** - Dry-run mode lets you preview changes before executing

## 📦 Installation

### From source

```bash
git clone https://github.com/yourname/neat
cd neat
cargo build --release
sudo cp target/release/neat /usr/local/bin/
```

### Using Cargo

```bash
cargo install neat
```

## 🚀 Usage

### Organize Files

```bash
# Organize by type (preview only - safe)
neat organize ~/Downloads --by-type

# Actually execute the organization
neat organize ~/Downloads --by-type --execute

# Organize by date (YYYY/MM structure)
neat organize ~/Downloads --by-date --execute

# Organize by extension
neat organize ~/Downloads --by-extension --execute
```

### Find Duplicates

```bash
# Find duplicate files
neat duplicates ~/Downloads

# Find and delete duplicates
neat duplicates ~/Downloads --delete --execute
```

### Clean Old Files

```bash
# Preview files older than 30 days
neat clean ~/Downloads --older-than 30d

# Delete files older than 7 days
neat clean ~/Downloads --older-than 7d --execute

# Remove empty folders
neat clean ~/Downloads --empty-folders --execute
```

### Statistics

```bash
# Show directory statistics
neat stats ~/Downloads
```

### Undo

```bash
# Undo the last operation
neat undo

# View operation history
neat history
```

## 🛡️ Safety Features

1. **Dry-run by default** - All operations preview changes first
2. **Confirmation prompts** - Destructive operations require confirmation
3. **Undo capability** - Rollback file moves with `neat undo`
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
neat [OPTIONS] <COMMAND>

Commands:
  organize    Organize files by type or date
  clean       Clean old files from a directory
  duplicates  Find duplicate files by content
  stats       Show statistics about a directory
  undo        Undo the last operation
  history     Show operation history
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

MIT License
