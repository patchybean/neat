# watch

Watch a directory and auto-organize new files as they appear.

## Usage

```bash
neatcli watch [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--by-type` | Organize by file type (default) |
| `--by-date` | Organize by date |
| `--by-extension` | Organize by extension |
| `--config` | Use custom config file |
| `--auto` | Auto-confirm without prompts |

## Examples

### Watch Downloads Folder

```bash
neatcli watch ~/Downloads
```

Output:
```
→ Watching /Users/you/Downloads (press Ctrl+C to stop)...

[14:32:15] New file: report.pdf
           → Moving to Documents/report.pdf
           Confirm? [y/N]:
```

### Auto Mode

Skip confirmations:

```bash
neatcli watch ~/Downloads --auto
```

### With Custom Config

```bash
neatcli watch ~/Downloads --config ~/.neat/custom.toml
```

## How It Works

1. NeatCLI monitors the directory for new files
2. When a file is created or modified, it's detected
3. The file is organized according to the selected mode
4. With `--auto`, this happens immediately; otherwise, you're prompted

!!! tip "Background Service"
    For permanent background watching, consider setting up a launchd (macOS) or systemd (Linux) service.

## Use Cases

- **Downloads folder** - Auto-organize downloads as they complete
- **Sync folders** - Organize files from cloud sync services
- **Import folders** - Sort files from camera imports

## See Also

- [organize](organize.md) - One-time organization
- [Configuration](../getting-started/configuration.md) - Custom rules
