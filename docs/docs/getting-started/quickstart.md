# Quick Start

This guide will walk you through the basics of using NeatCLI to organize your files.

## Your First Organization

Let's organize a messy Downloads folder:

```bash
neatcli organize ~/Downloads
```

This will **preview** what will happen without making any changes:

```
→ Scanning /Users/you/Downloads (organizing by type)...

Preview:
────────────────────────────────────────────────────────────

  Images (15 files)
    → photo.jpg
    → screenshot.png
    → diagram.svg
    → ... and 12 more

  Documents (8 files)
    → report.pdf
    → notes.txt
    → invoice.docx
    → ... and 5 more

  Videos (3 files)
    → recording.mp4
    → tutorial.mov
    → clip.avi

────────────────────────────────────────────────────────────

Summary: 26 files to move (1.2 GB)

ℹ Use --execute to execute these changes.
```

!!! tip "Dry-run by Default"
    NeatCLI always shows a preview first. Nothing is changed until you add `--execute`.

## Execute the Organization

When you're happy with the preview, run with `--execute`:

```bash
neatcli organize ~/Downloads --execute
```

Your files will be moved into organized folders:

```
Downloads/
├── Images/
│   ├── photo.jpg
│   ├── screenshot.png
│   └── ...
├── Documents/
│   ├── report.pdf
│   └── ...
└── Videos/
    └── recording.mp4
```

## Organization Modes

NeatCLI supports multiple organization modes:

### By Type (Default)
```bash
neatcli organize ~/Downloads --by-type --execute
```
Creates folders: `Images/`, `Documents/`, `Videos/`, `Audio/`, `Archives/`, `Code/`

### By Date
```bash
neatcli organize ~/Photos --by-date --execute
```
Creates folders: `2024/01/`, `2024/02/`, etc.

### By Extension
```bash
neatcli organize ~/Downloads --by-extension --execute
```
Creates folders: `PDF/`, `JPG/`, `MP4/`, etc.

### By Camera (EXIF)
```bash
neatcli organize ~/Photos --by-camera --execute
```
Creates folders: `iPhone 15 Pro/`, `Canon EOS R5/`, etc.

### By Date Taken (EXIF)
```bash
neatcli organize ~/Photos --by-date-taken --execute
```
Uses the actual photo date from EXIF metadata.

### By Artist (Audio)
```bash
neatcli organize ~/Music --by-artist --execute
```
Creates folders: `The Beatles/`, `Taylor Swift/`, etc.

### By Album
```bash
neatcli organize ~/Music --by-album --execute
```
Creates folders: `Artist/Album/`

## Filtering Files

You can filter which files to organize:

### By Size
```bash
# Only files larger than 10MB
neatcli organize ~/Downloads --min-size 10MB --execute

# Only files smaller than 100MB
neatcli organize ~/Downloads --max-size 100MB --execute
```

### By Date
```bash
# Only files modified after January 2024
neatcli organize ~/Downloads --after 2024-01-01 --execute
```

### By Name
```bash
# Only files starting with "IMG_"
neatcli organize ~/Photos --startswith "IMG_" --execute

# Only files containing "2024"
neatcli organize ~/Documents --contains "2024" --execute
```

### By MIME Type
```bash
# Only images
neatcli organize ~/Downloads --mime "image/*" --execute

# Only PDFs
neatcli organize ~/Downloads --mime "application/pdf" --execute
```

## Copy Instead of Move

Use `--copy` to copy files instead of moving them:

```bash
neatcli organize ~/Downloads --copy --execute
```

Original files remain in place; organized copies are created.

## Undo Changes

Made a mistake? Undo the last operation:

```bash
neatcli undo
```

View your operation history:
```bash
neatcli history
```

## Next Steps

- [Commands Reference](../commands/organize.md) - Full command documentation
- [Filters](../reference/filters.md) - All available filters
- [Configuration](configuration.md) - Customize with config files
