# tui

Launch the interactive TUI (Text User Interface) mode.

## Usage

```bash
neatcli tui [PATH]
```

## Features

The TUI provides a visual interface for:

- Browsing files in the directory
- Selecting files for organization
- Previewing moves before executing
- Choosing organization mode interactively

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Space` | Select/deselect file |
| `a` | Select all |
| `A` | Deselect all |
| `1-7` | Choose organize mode |
| `Enter` | Preview moves |
| `e` | Execute moves |
| `q` | Quit |

## Organization Modes

| Key | Mode |
|-----|------|
| `1` | By Type |
| `2` | By Date |
| `3` | By Extension |
| `4` | By Camera (EXIF) |
| `5` | By Date Taken (EXIF) |
| `6` | By Artist (Audio) |
| `7` | By Album (Audio) |

## Examples

### Open TUI

```bash
neatcli tui ~/Downloads
```

### Current Directory

```bash
neatcli tui
```

## Screenshots

```
┌─────────────────────────────────────────────────────────────┐
│ NeatCLI - /Users/you/Downloads                              │
├─────────────────────────────────────────────────────────────┤
│ [ ] report.pdf                               2.3 MB         │
│ [x] photo.jpg                                5.1 MB         │
│ [x] screenshot.png                           1.2 MB         │
│ [ ] video.mp4                               150.0 MB        │
│ [ ] document.docx                            500 KB         │
├─────────────────────────────────────────────────────────────┤
│ Mode: By Type  │  Selected: 2 files (6.3 MB)                │
│ [1] Type [2] Date [3] Ext [4] Camera [5] Date [6] Artist   │
├─────────────────────────────────────────────────────────────┤
│ ↑↓: Navigate  Space: Select  Enter: Preview  e: Execute    │
└─────────────────────────────────────────────────────────────┘
```

## See Also

- [organize](organize.md) - Command-line organization
- [Quick Start](../getting-started/quickstart.md) - Basic usage
