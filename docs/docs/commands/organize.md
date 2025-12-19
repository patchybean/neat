# organize

Organize files by type, date, extension, or metadata.

## Usage

```bash
neatcli organize [OPTIONS] [PATH]
```

**Arguments:**

- `PATH` - Directory to organize (default: current directory)

## Organization Modes

Choose how files should be organized:

| Mode | Flag | Result |
|------|------|--------|
| By Type | `--by-type` | `Images/`, `Documents/`, `Videos/`, etc. |
| By Date | `--by-date` | `2024/01/`, `2024/02/`, etc. |
| By Extension | `--by-extension` | `PDF/`, `JPG/`, `MP4/`, etc. |
| By Camera | `--by-camera` | Camera model from EXIF |
| By Date Taken | `--by-date-taken` | Date from EXIF metadata |
| By Artist | `--by-artist` | Artist from audio tags |
| By Album | `--by-album` | `Artist/Album/` structure |

!!! info "Default Mode"
    If no mode is specified, `--by-type` is used.

## Options

### Execution Options

| Flag | Short | Description |
|------|-------|-------------|
| `--execute` | `-e` | Actually execute the changes |
| `--dry-run` | `-n` | Preview changes (default) |
| `--copy` | `-c` | Copy files instead of moving |
| `--recursive` | `-r` | Include subdirectories |

### Filter Options

| Flag | Description | Example |
|------|-------------|---------|
| `--min-size` | Minimum file size | `--min-size 1MB` |
| `--max-size` | Maximum file size | `--max-size 100MB` |
| `--after` | Modified after date | `--after 2024-01-01` |
| `--before` | Modified before date | `--before 2024-12-31` |
| `--startswith` | Filename starts with | `--startswith "IMG_"` |
| `--endswith` | Filename ends with | `--endswith "_backup"` |
| `--contains` | Filename contains | `--contains "2024"` |
| `--regex` | Match regex pattern | `--regex "^IMG_\d{4}"` |
| `--mime` | Filter by MIME type | `--mime "image/*"` |
| `--ignore` | Ignore pattern | `--ignore "*.tmp"` |

## Examples

### Basic Organization

```bash
# Preview organization by type
neatcli organize ~/Downloads

# Execute organization
neatcli organize ~/Downloads --execute
```

### Organize Photos

```bash
# By camera model
neatcli organize ~/Photos --by-camera --execute

# By date taken from EXIF
neatcli organize ~/Photos --by-date-taken --execute

# Only JPEG images larger than 5MB
neatcli organize ~/Photos --mime "image/jpeg" --min-size 5MB --execute
```

### Organize Music

```bash
# By artist
neatcli organize ~/Music --by-artist --execute

# By album (creates Artist/Album/ structure)
neatcli organize ~/Music --by-album --execute
```

### Selective Organization

```bash
# Only files from 2024
neatcli organize ~/Downloads --after 2024-01-01 --execute

# Only files matching regex pattern
neatcli organize ~/Photos --regex "^IMG_\d{4}" --execute

# Only screenshots
neatcli organize ~/Desktop --startswith "Screenshot" --execute
```

### Copy Instead of Move

```bash
# Create organized copies, keep originals
neatcli organize ~/Photos --copy --execute
```

### Recursive Organization

```bash
# Include all subdirectories
neatcli organize ~/Downloads --recursive --execute
```

## Output

### Preview Mode (Default)

```
→ Scanning /Users/you/Downloads (organizing by type)...

Preview:
────────────────────────────────────────────────────────────

  Images (15 files)
    → photo.jpg
    → screenshot.png
    → ... and 13 more

  Documents (8 files)
    → report.pdf
    → notes.txt
    → ... and 6 more

────────────────────────────────────────────────────────────

Summary: 23 files to move (450 MB)

ℹ Use --execute to execute these changes.
```

### Execute Mode

```
→ Scanning /Users/you/Downloads (organizing by type)...

Results:
────────────────────────────────────────
  ✓ 23 files moved (450 MB)
```

## See Also

- [Filters Reference](../reference/filters.md)
- [MIME Types Reference](../reference/mime-types.md)
- [Photo Organization Examples](../examples/photos.md)
