# Configuration File

NeatCLI can be configured with a TOML file.

## Location

Default: `~/.neat/config.toml`

## Create Config

```bash
neatcli config init
```

## Example Config

```toml
[[rules]]
name = "Invoices"
pattern = "*invoice*.pdf"
destination = "Documents/Invoices/{year}"
priority = 10

[[rules]]
name = "Screenshots"
pattern = "Screenshot*.png"
destination = "Images/Screenshots/{year}-{month}"
priority = 5

[settings]
include_hidden = false
follow_symlinks = false
default_organize_mode = "by-type"
```

## Variables

| Variable | Description |
|----------|-------------|
| `{year}` | Current year (YYYY) |
| `{month}` | Current month (MM) |
| `{day}` | Current day (DD) |
