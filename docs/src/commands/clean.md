# clean

Remove old files and empty folders.

## Usage

```bash
neatcli clean [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--older-than` | Delete files older than duration (e.g., `30d`, `1w`) |
| `--empty-folders` | Remove empty folders |
| `--trash` | Move to trash instead of permanent delete |
| `-e, --execute` | Execute the cleanup |
| `-n, --dry-run` | Preview what will be deleted (default) |

## Examples

```bash
# Preview files older than 30 days
neatcli clean ~/Downloads --older-than 30d

# Remove old files (move to trash)
neatcli clean ~/Downloads --older-than 30d --trash --execute

# Remove empty folders
neatcli clean ~/Projects --empty-folders --execute
```
