# clean

Remove old files and empty folders from a directory.

## Usage

```bash
neatcli clean [OPTIONS] <PATH>
```

**Arguments:**

- `PATH` - Directory to clean (required)

## Options

| Flag | Description | Example |
|------|-------------|---------|
| `--older-than` | Delete files older than duration | `--older-than 30d` |
| `--empty-folders` | Remove empty folders | |
| `--trash` | Move to trash instead of deleting | |
| `--execute` `-e` | Execute the cleanup | |
| `--dry-run` `-n` | Preview what will be deleted | |
| `--min-size` | Minimum file size | `--min-size 1KB` |
| `--max-size` | Maximum file size | `--max-size 10MB` |

## Duration Format

The `--older-than` flag accepts various duration formats:

| Format | Example | Meaning |
|--------|---------|---------|
| Days | `30d` | 30 days |
| Weeks | `2w` | 2 weeks |
| Hours | `24h` | 24 hours |

## Examples

### Remove Old Files

```bash
# Preview files older than 30 days
neatcli clean ~/Downloads --older-than 30d

# Delete files older than 30 days (permanent)
neatcli clean ~/Downloads --older-than 30d --execute

# Move old files to trash (safer)
neatcli clean ~/Downloads --older-than 30d --trash --execute
```

### Remove Empty Folders

```bash
# Preview empty folders
neatcli clean ~/Projects --empty-folders

# Remove empty folders
neatcli clean ~/Projects --empty-folders --execute
```

### Combined Cleanup

```bash
# Remove old files AND empty folders
neatcli clean ~/Downloads --older-than 7d --empty-folders --trash --execute
```

### With Size Filters

```bash
# Only delete small old files (< 1MB)
neatcli clean ~/Downloads --older-than 30d --max-size 1MB --execute
```

## Output

### Preview Mode

```
→ Scanning /Users/you/Downloads for files older than 30d...

Old files to clean:
────────────────────────────────────────────────────────────
  ○ old_report.pdf (45d old, 2.3 MB)
  ○ backup_2024.zip (60d old, 150 MB)
  ○ temp_file.tmp (90d old, 12 KB)

Summary: 3 files (152.3 MB) older than 30d

ℹ Use --execute to delete these files.
```

### Execute Mode

```
→ Scanning /Users/you/Downloads for files older than 30d...

✓ Deleted 3 files (152.3 MB)
```

!!! warning "Permanent Deletion"
    Without `--trash`, files are permanently deleted and cannot be recovered.
    Use `--trash` for safer cleanup.

## See Also

- [undo](undo.md) - Undo operations (only works for moves, not deletes)
- [duplicates](duplicates.md) - Find and remove duplicate files
