# Changelog

All notable changes to NeatCLI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

---

## [0.5.0] - 2025-12-26

### Added
- **Shell Hooks**: `post_action` field in config rules to execute shell commands after file operations
  - Variable substitution: `{file}`, `{dest}`, `{name}`, `{ext}`, `{dir}`
- **Content Filtering**: `--content` flag to filter files by text content (TXT, MD, JSON, etc.)
- **TUI Conflict Resolver**: Interactive conflict resolution view with keyboard shortcuts
  - `s` skip, `o` overwrite, `r` rename, `k` keep both, `←/→` navigate

---

## [0.4.0] - 2025-12-23

### Added
- **Template Variables**: Custom destination paths with `--template` flag
  - Variables: `{year}`, `{month}`, `{day}`, `{category}`, `{filename}`, `{ext}`
  - EXIF variables: `{camera}`, `{taken.year}`, `{taken.month}`
  - Audio variables: `{artist}`, `{album}`
  - Preset templates: `photos`, `music`, `by-type`, `by-date`
- **Profile System**: Save and reuse organize command configurations
  - `profile save`, `profile list`, `profile run`, `profile delete`, `profile show`
- **Quick Actions**: Shortcut commands for common workflows
  - `quick downloads`, `quick desktop`, `quick photos`, `quick music`, `quick cleanup`

### Changed
- Improved documentation with MkDocs Material theme
- Added comprehensive filter options (startswith, endswith, contains, regex, mime)

---

## [0.3.0] - 2025-12-18

### Added
- **Interactive TUI**: Visual file browser with ratatui
- **Similar Images Detection**: Perceptual hashing to find visually similar images
- **Conflict Strategies**: skip, overwrite, rename, ask, deduplicate, backup
- **Watch Mode**: Auto-organize new files as they appear
- **Shell Completions**: Bash, Zsh, Fish, PowerShell support

---

## [0.2.0] - 2025-12-15

### Added
- **EXIF Metadata**: Organize photos by camera model or date taken
- **Audio Metadata**: Organize music by artist or album (ID3 tags)
- **Size Filters**: `--min-size`, `--max-size` options
- **Date Filters**: `--after`, `--before` options
- **Ignore Patterns**: `.neatignore` file and `-I` flag
- **Export Formats**: JSON and CSV export for duplicates
- **Trash Support**: `--trash` flag for safe deletion

---

## [0.1.0] - 2025-12-12

### Added
- Initial release
- **Organize by Type**: Sort files into Images, Documents, Videos, Audio, Archives, Code, Data
- **Organize by Date**: Create YYYY/MM folder structure
- **Organize by Extension**: Group files by extension
- **Duplicate Detection**: SHA256 content hashing
- **Clean Command**: Remove old files
- **Stats Command**: Show directory statistics
- **Undo/History**: Rollback operations
- **Safe by Default**: Dry-run mode for preview

[Unreleased]: https://github.com/patchybean/neatcli/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/patchybean/neatcli/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/patchybean/neatcli/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/patchybean/neatcli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/patchybean/neatcli/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/patchybean/neatcli/releases/tag/v0.1.0

