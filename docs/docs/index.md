# Welcome to NeatCLI

<div align="center" markdown>

**A smart CLI tool to organize and clean up messy directories**

[![Crates.io](https://img.shields.io/crates/v/neatcli.svg)](https://crates.io/crates/neatcli)
[![GitHub release](https://img.shields.io/github/v/release/patchybean/neatcli)](https://github.com/patchybean/neatcli/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

## What is NeatCLI?

NeatCLI is a powerful command-line tool written in Rust that helps you automatically organize files, find duplicates, clean up old files, and maintain tidy directories. It's designed to be fast, safe, and flexible.

!!! tip "Perfect for"
    - 📂 Organizing your Downloads folder
    - 📸 Sorting photos by date or camera
    - 🎵 Organizing music by artist/album
    - 🗑️ Cleaning up old files and duplicates
    - 👀 Watching folders for automatic organization

## Features

- **🗂️ Smart Organization** - Organize files by type, date, extension, or metadata (EXIF/ID3)
- **🔍 Duplicate Detection** - Find duplicate files using SHA256 content hashing
- **🖼️ Similar Images** - Detect visually similar images using perceptual hashing
- **🧹 Cleanup Tools** - Remove old files and empty folders
- **⏪ Undo Support** - Full operation history with undo capability
- **👁️ Watch Mode** - Auto-organize new files as they appear
- **🖥️ Interactive TUI** - Visual file management interface
- **⚡ Fast & Safe** - Written in Rust, with dry-run by default

## Quick Example

```bash
# Preview what will happen (dry-run is default)
neatcli organize ~/Downloads

# Actually organize the files
neatcli organize ~/Downloads --execute

# Find duplicate files
neatcli duplicates ~/Pictures

# Show directory statistics
neatcli stats ~/Documents
```

## Getting Started

Ready to get started? Check out the [Installation](getting-started/installation.md) guide or jump straight to the [Quick Start](getting-started/quickstart.md) tutorial.

## Installation

=== "Cargo"
    ```bash
    cargo install neatcli
    ```

=== "Homebrew"
    ```bash
    brew tap patchybean/tap
    brew install neatcli
    ```

=== "From Source"
    ```bash
    git clone https://github.com/patchybean/neatcli.git
    cd neatcli && cargo build --release
    ```
