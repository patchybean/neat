# organize

Organize files by type, date, extension, or metadata.

## Usage

```bash
neatcli organize [OPTIONS] [PATHS]...
```

**Arguments:**

- `PATHS` - One or more directories to organize (default: current directory)

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

## Template Variables

Use the `--template` flag for custom destination paths with flexible variables.

### Usage

```bash
neatcli organize ~/Photos --template "{year}/{month}/{category}/{filename}" --execute
```

### Available Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{filename}` | File name without extension | `photo` |
| `{name}` | Full file name with extension | `photo.jpg` |
| `{ext}` / `{extension}` | File extension | `jpg` |
| `{category}` / `{type}` | File category (Images, Documents, etc.) | `Images` |
| `{year}` | Year from modified date | `2024` |
| `{month}` | Month from modified date (zero-padded) | `12` |
| `{day}` | Day from modified date (zero-padded) | `25` |
| `{date}` | Full date (YYYY-MM-DD) | `2024-12-25` |
| `{size}` | File size in bytes | `1048576` |
| `{size_kb}` | File size in KB | `1024` |
| `{size_mb}` | File size in MB | `1` |

#### Current Date/Time Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{now.year}` | Current year | `2024` |
| `{now.month}` | Current month | `12` |
| `{now.day}` | Current day | `22` |
| `{now.date}` | Current date | `2024-12-22` |

#### Image Metadata (EXIF)

| Variable | Description | Example |
|----------|-------------|---------|
| `{camera}` | Camera model from EXIF | `Canon EOS 5D` |
| `{date_taken}` | Date taken from EXIF | `2024/12` |
| `{taken.year}` | Year from EXIF date | `2024` |
| `{taken.month}` | Month from EXIF date | `12` |

#### Audio Metadata

| Variable | Description | Example |
|----------|-------------|---------|
| `{artist}` | Artist from audio tags | `Taylor Swift` |
| `{album}` | Album from audio tags | `1989` |

### Preset Templates

Use these preset names instead of writing a full template:

| Preset | Template |
|--------|----------|
| `by-type` / `type` | `{category}/{filename}` |
| `by-date` / `date` | `{year}/{month}/{filename}` |
| `by-extension` / `ext` | `{extension}/{filename}` |
| `by-camera` / `camera` | `{camera}/{filename}` |
| `by-date-taken` / `date-taken` | `{taken.year}/{taken.month}/{filename}` |
| `by-artist` / `artist` | `{artist}/{filename}` |
| `by-album` / `album` | `{artist}/{album}/{filename}` |
| `photos` | `{taken.year}/{taken.month}/{filename}` |
| `music` | `{artist}/{album}/{filename}` |

### Template Examples

```bash
# Organize by year and category
neatcli organize ~/Downloads --template "{year}/{category}/{filename}" --execute

# Photos with camera info
neatcli organize ~/Photos --template "{taken.year}/{taken.month}/{camera}/{filename}" --execute

# Music with artist/album structure
neatcli organize ~/Music --template "{artist}/{album}/{filename}" --execute

# Use presets
neatcli organize ~/Photos --template "photos" --execute
neatcli organize ~/Music --template "music" --execute

# Organize by current date (for backup purposes)
neatcli organize ~/Downloads --template "backup/{now.date}/{category}/{filename}" --execute
```

!!! tip "Missing Variables"
    If a variable is not available (e.g., no EXIF data), it will be replaced with `Unknown`.

## Options

### Execution Options

| Flag | Short | Description |
|------|-------|-------------|
| `--execute` | `-e` | Actually execute the changes |
| `--dry-run` | `-n` | Preview changes (default) |
| `--copy` | `-c` | Copy files instead of moving |
| `--recursive` | `-r` | Include subdirectories |

### Conflict Resolution

| Flag | Value | Description |
|------|-------|-------------|
| `--on-conflict` | `skip` | Skip files that already exist at destination |
| `--on-conflict` | `overwrite` | Overwrite existing files |
| `--on-conflict` | `rename` | Rename with suffix `_1`, `_2`, etc. (default) |
| `--on-conflict` | `ask` | Ask interactively for each conflict |

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

### Multiple Locations

```bash
# Organize multiple directories at once
neatcli organize ~/Downloads ~/Desktop --by-type --execute

# Different paths with same options
neatcli organize /path/to/photos /path/to/backups --by-date-taken --execute
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
