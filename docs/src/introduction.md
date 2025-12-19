# NeatCLI

A smart CLI tool to organize and clean up messy directories.

## Features

- 🗂️ **Organize files** by type, date, extension, or metadata (EXIF, ID3)
- 🧹 **Clean up** old files and empty folders
- 🔍 **Find duplicates** by content hash
- 🖼️ **Find similar images** using perceptual hashing
- 📊 **Statistics** about directory contents
- ⏪ **Undo** operations with full history
- 👁️ **Watch mode** for auto-organizing
- 🖥️ **Interactive TUI** for visual file management

## Version 0.3.0 - What's New

- `--copy` flag: Copy files instead of moving
- `--recursive` flag: Scan subdirectories
- Name filters: `--startswith`, `--endswith`, `--contains`
- `--regex`: Filter by regex pattern
- `--mime`: Filter by MIME type (e.g., `image/*`)
