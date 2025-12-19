# duplicates

Find duplicate files by content hash.

## Usage

```bash
neatcli duplicates [OPTIONS] <PATH>
```

## How It Works

NeatCLI uses SHA256 hashing to find files with identical content:

1. **Size grouping** - Files are grouped by size (different sizes can't be duplicates)
2. **Hash calculation** - SHA256 hash is computed for files with matching sizes
3. **Duplicate detection** - Files with identical hashes are duplicates

!!! tip "Performance"
    Only files with matching sizes are hashed, making the process very fast even for large directories.

## Options

| Flag | Description |
|------|-------------|
| `--delete` | Delete duplicates (keeps first file) |
| `--trash` | Move to trash instead of permanent delete |
| `--json` | Export results as JSON |
| `--csv` | Export results as CSV |
| `--execute` `-e` | Execute deletion |
| `--min-size` | Minimum file size |
| `--max-size` | Maximum file size |
| `--after` | Files modified after date |
| `--before` | Files modified before date |

## Examples

### Find Duplicates

```bash
# Find duplicates in Pictures folder
neatcli duplicates ~/Pictures
```

Output:
```
→ Scanning /Users/you/Pictures for duplicate files...
  Found 1234 files to analyze

Duplicate Files Found:
────────────────────────────────────────────────────────────

  Group 1 (5.2 MB) - 3 copies:
    ● /Users/you/Pictures/photo.jpg
    ○ /Users/you/Pictures/backup/photo.jpg
    ○ /Users/you/Pictures/old/photo_copy.jpg

  Group 2 (2.1 MB) - 2 copies:
    ● /Users/you/Pictures/vacation.png
    ○ /Users/you/Pictures/exports/vacation.png

────────────────────────────────────────────────────────────

Summary: 3 duplicate files in 2 groups
Wasted space: 9.4 MB could be recovered by removing duplicates

ℹ Use --delete --execute to remove duplicates (keeps first file in each group).
```

### Delete Duplicates

```bash
# Move duplicates to trash (keeps first file)
neatcli duplicates ~/Pictures --delete --trash --execute
```

!!! warning "Which File is Kept?"
    The first file in each group (marked with ●) is always kept.
    Duplicates (marked with ○) are deleted.

### Export Results

```bash
# Export to JSON
neatcli duplicates ~/Pictures --json > duplicates.json

# Export to CSV
neatcli duplicates ~/Pictures --csv > duplicates.csv
```

### Filter by Size

```bash
# Only find duplicates larger than 10MB
neatcli duplicates ~/Downloads --min-size 10MB
```

## JSON Output Format

```json
{
  "groups": [
    {
      "hash": "a1b2c3...",
      "size": 5242880,
      "files": [
        "/path/to/original.jpg",
        "/path/to/duplicate.jpg"
      ]
    }
  ],
  "total_duplicates": 3,
  "wasted_space": 15728640
}
```

## See Also

- [similar](similar.md) - Find visually similar images
- [clean](clean.md) - Remove old files
