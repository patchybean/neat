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
| `d` | Deselect all |
| `m` | Change organize mode |
| `b` | Open batch action menu |
| `Enter` / `p` | Preview moves |
| `?` | Show help |
| `q` | Quit |

## Batch Operations

Press `b` with selected files to open the batch action menu:

| Key | Action |
|-----|--------|
| `t` | Move to trash |
| `d` | Delete permanently |
| `o` | Organize with current mode |
| `Esc` | Cancel |

## Organization Modes

Press `m` to cycle through modes:

| Mode | Description |
|------|-------------|
| By Type | Images/, Documents/, Videos/, etc. |
| By Date | YYYY/MM/ structure |
| By Extension | PDF/, JPG/, MP4/, etc. |
| By Camera | Camera model from EXIF |
| By Date Taken | Date from EXIF metadata |
| By Artist | Artist from audio tags |
| By Album | Artist/Album/ structure |

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
