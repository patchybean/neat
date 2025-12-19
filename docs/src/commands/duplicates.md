# duplicates

Find duplicate files by content hash.

## Usage

```bash
neatcli duplicates [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--delete` | Delete duplicates (keeps first file) |
| `--trash` | Move duplicates to trash |
| `--json` | Export results as JSON |
| `--csv` | Export results as CSV |
| `-e, --execute` | Execute deletion |

## Examples

```bash
# Find duplicates
neatcli duplicates ~/Pictures

# Delete duplicates (keeps first file)
neatcli duplicates ~/Pictures --delete --execute

# Export to JSON
neatcli duplicates ~/Pictures --json > duplicates.json
```
