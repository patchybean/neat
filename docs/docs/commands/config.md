# config

Manage NeatCLI configuration.

## Usage

```bash
neatcli config <ACTION>
```

## Actions

### init

Create a sample configuration file:

```bash
neatcli config init
```

Creates `~/.neat/config.toml` with example rules.

### show

Display current configuration:

```bash
neatcli config show
```

## Examples

### Initialize Config

```bash
neatcli config init
```

Output:
```
✓ Created config file: /Users/you/.neat/config.toml

Sample rules:
  • Invoices: *invoice*.pdf → Documents/Invoices/{year}
  • Screenshots: Screenshot*.png → Images/Screenshots/{year}-{month}
```

### Show Config

```bash
neatcli config show
```

Output:
```
→ Config: /Users/you/.neat/config.toml

Rules:
────────────────────────────────────────────────────────────
  • Invoices (priority: 10)
    Pattern: *invoice*.pdf
    Dest:    Documents/Invoices/{year}

  • Screenshots (priority: 5)
    Pattern: Screenshot*.png
    Dest:    Images/Screenshots/{year}-{month}

Settings:
────────────────────────────────────────────────────────────
  Include hidden: false
  Follow symlinks: false
  Default mode: by-type
```

## Config Location

| Platform | Path |
|----------|------|
| macOS | `~/.neat/config.toml` |
| Linux | `~/.neat/config.toml` |
| Windows | `%USERPROFILE%\.neat\config.toml` |

## See Also

- [Configuration Guide](../getting-started/configuration.md) - Full config documentation
