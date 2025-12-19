# NeatCLI Development Roadmap

Track progress of neatcli development.

## ✅ Completed Features (v0.2.0)

### Core Features
- [x] Organize by Type (Images, Documents, Videos, Audio, Archives, Code, Data)
- [x] Organize by Date (YYYY/MM structure)
- [x] Organize by Extension
- [x] Find Duplicates (SHA256 content hashing)
- [x] Clean Old Files (--older-than)
- [x] Remove Empty Folders
- [x] Directory Statistics

### Safety & UX
- [x] Dry-run by Default
- [x] Undo Operations
- [x] Operation History Logging
- [x] Confirmation Prompts

### Advanced
- [x] Watch Mode (auto-organize new files)
- [x] Custom Rules via TOML config
- [x] Interactive TUI
- [x] Trash Support (--trash flag)
- [x] Shell Completions (bash, zsh, fish, powershell)
- [x] Ignore Patterns (.neatignore, -I flag)

---

## 🚧 In Progress

_No features currently in progress_

---

## 📋 Planned Features

### Phase 1: Metadata Support
- [ ] **EXIF Metadata for Images**
  - Organize by camera model (Canon, Sony, iPhone)
  - Organize by date taken (more accurate than file modified date)
  - Extract GPS location
  - Crate: `kamadak-exif`

- [ ] **Audio Metadata (ID3/MP3 Tags)**
  - Organize by artist
  - Organize by album
  - Organize by genre
  - Crate: `lofty`

- [ ] **PDF Metadata**
  - Extract author, title
  - Organize by author
  - Crate: `lopdf`

### Phase 2: Advanced Duplicate Detection
- [ ] **Perceptual Hash for Images**
  - Find visually similar images (not just byte-identical)
  - Detect resized/recompressed versions
  - Crate: `image`, `img_hash`

- [ ] **Fuzzy Content Matching**
  - Find files with 95%+ similarity
  - Useful for finding near-duplicates

- [ ] **SSD Optimization**
  - Parallel file reading
  - Batched I/O operations
  - Progress estimation

### Phase 3: Quality of Life
- [ ] **Size Filters**
  - `--min-size 10MB`
  - `--max-size 1GB`

- [ ] **Date Range Filters**
  - `--after 2024-01-01`
  - `--before 2024-12-31`

- [ ] **Export Reports**
  - JSON output for scripting
  - CSV export
  - HTML report generation

- [ ] **Interactive Mode Improvements**
  - Multi-select in TUI
  - Keyboard shortcuts help
  - Search/filter files

### Phase 4: Integration
- [ ] **Cloud Storage Detection**
  - Detect Dropbox, iCloud, Google Drive folders
  - Warn about sync conflicts

- [ ] **Daemon Mode**
  - Background service
  - System tray integration (optional)

- [ ] **Homebrew Core Submission**
  - Requires 75+ GitHub stars
  - Submit PR to homebrew-core

---

## 🐛 Known Issues

_No known issues_

---

## 📝 Notes

### Version History
- **v0.2.0** - Current stable release
  - Full feature set as listed above
  - Published on crates.io
  - Available via Homebrew tap

### Contribution Ideas
1. Add more file categories
2. Improve error messages
3. Add more tests
4. Documentation improvements

---

_Last updated: 2024-12-19_
