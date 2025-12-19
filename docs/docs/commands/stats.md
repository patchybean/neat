# stats

Show statistics about a directory.

## Usage

```bash
neatcli stats [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--json` | Export as JSON |

## Examples

### Basic Statistics

```bash
neatcli stats ~/Documents
```

Output:
```
→ Analyzing /Users/you/Documents...

Files by Type:
──────────────────────────────────────────────────
  Documents      1,234 files    450 MB  ████████████████████
  Images           567 files    1.2 GB  ██████████
  Archives         123 files    800 MB  ████████
  Code             456 files     50 MB  ██
  Other            234 files     25 MB  █

Largest Files:
──────────────────────────────────────────────────
    150 MB  backup.zip
     80 MB  video_project.mp4
     45 MB  database.sqlite
     30 MB  photo_album.pdf
     20 MB  presentation.pptx

Oldest Files:
──────────────────────────────────────────────────
     3y ago  old_report.doc
     2y ago  archive_2022.zip
     1y ago  notes_2023.txt
    6mo ago  draft.pdf
     3mo ago  recent.xlsx

──────────────────────────────────────────────────
Total: 2,614 files, 2.5 GB
```

### JSON Export

```bash
neatcli stats ~/Documents --json
```

Output:
```json
{
  "total_files": 2614,
  "total_size": 2684354560,
  "categories": [
    {"name": "Documents", "count": 1234, "size": 471859200},
    {"name": "Images", "count": 567, "size": 1288490188},
    {"name": "Archives", "count": 123, "size": 838860800},
    {"name": "Code", "count": 456, "size": 52428800},
    {"name": "Other", "count": 234, "size": 26214400}
  ]
}
```

## Use Cases

- **Disk usage analysis** - See what's taking up space
- **Directory audits** - Understand contents before organizing
- **Reporting** - Export JSON for further processing

## See Also

- [organize](organize.md) - Organize files by type
- [duplicates](duplicates.md) - Find duplicates to reclaim space
