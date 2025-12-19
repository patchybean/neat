# organize

Organize files by type, date, extension, or metadata.

## Usage

```bash
neatcli organize [OPTIONS] [PATH]
```

## Options

| Flag | Description |
|------|-------------|
| `--by-type` | Organize by file type (default) |
| `--by-date` | Organize by date (YYYY/MM structure) |
| `--by-extension` | Organize by file extension |
| `--by-camera` | Organize images by camera model (EXIF) |
| `--by-date-taken` | Organize by date taken (EXIF) |
| `--by-artist` | Organize audio by artist |
| `--by-album` | Organize audio by album |
| `-c, --copy` | Copy files instead of moving |
| `-r, --recursive` | Scan subdirectories |
| `-e, --execute` | Actually execute changes |
| `-n, --dry-run` | Preview changes (default) |

## Filters

| Flag | Description |
|------|-------------|
| `--min-size` | Minimum file size (e.g., `1MB`) |
| `--max-size` | Maximum file size (e.g., `100MB`) |
| `--after` | Files modified after date (YYYY-MM-DD) |
| `--before` | Files modified before date |
| `--startswith` | Filename starts with string |
| `--endswith` | Filename ends with string |
| `--contains` | Filename contains string |
| `--regex` | Match filename with regex |
| `--mime` | Filter by MIME type (e.g., `image/*`) |

## Examples

```bash
# Organize by type
neatcli organize ~/Downloads --execute

# Copy instead of move
neatcli organize ~/Downloads --copy --execute

# Organize images by camera model
neatcli organize ~/Photos --by-camera --execute

# Only JPEG images larger than 1MB
neatcli organize . --mime "image/jpeg" --min-size 1MB --execute

# Files starting with "IMG_"
neatcli organize . --startswith "IMG_" --execute
```
