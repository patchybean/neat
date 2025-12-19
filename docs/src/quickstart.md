# Quick Start

## Organize Files by Type

```bash
# Preview what will happen
neatcli organize ~/Downloads --dry-run

# Execute the organization
neatcli organize ~/Downloads --execute
```

Files will be moved to folders like:
- `Images/` (jpg, png, gif, etc.)
- `Documents/` (pdf, doc, txt, etc.)
- `Videos/` (mp4, mov, avi, etc.)
- `Audio/` (mp3, wav, flac, etc.)

## Find Duplicates

```bash
neatcli duplicates ~/Pictures
```

## Get Statistics

```bash
neatcli stats ~/Documents
```

## Interactive Mode (TUI)

```bash
neatcli tui ~/Downloads
```
