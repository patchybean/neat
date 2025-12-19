# similar

Find visually similar images using perceptual hashing.

## Usage

```bash
neatcli similar [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--threshold` | Similarity threshold (default: 5, lower = more similar) |
| `--delete` | Delete similar images |
| `--trash` | Move to trash |
| `-e, --execute` | Execute deletion |

## Examples

```bash
# Find similar images
neatcli similar ~/Photos

# Stricter threshold
neatcli similar ~/Photos --threshold 3

# Delete similar images
neatcli similar ~/Photos --delete --trash --execute
```
