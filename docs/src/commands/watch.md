# watch

Watch a directory and auto-organize new files.

## Usage

```bash
neatcli watch [OPTIONS] <PATH>
```

## Options

| Flag | Description |
|------|-------------|
| `--by-type` | Organize by type (default) |
| `--by-date` | Organize by date |
| `--by-extension` | Organize by extension |
| `--config` | Use custom config file |
| `--auto` | Skip confirmations |

## Examples

```bash
# Watch Downloads folder
neatcli watch ~/Downloads

# Auto-organize without confirmation
neatcli watch ~/Downloads --auto
```
