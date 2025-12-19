# Configuration

NeatCLI can be customized using a configuration file and ignore files.

## Configuration File

Create a configuration file at `~/.neat/config.toml`:

```bash
neatcli config init
```

### Example Configuration

```toml
# Custom rules for organizing files
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

[[rules]]
name = "Downloads Archive"
pattern = "*.zip"
destination = "Archives/{year}"
priority = 3

# Global settings
[settings]
include_hidden = false
follow_symlinks = false
default_organize_mode = "by-type"
```

### Rule Fields

| Field | Description | Required |
|-------|-------------|----------|
| `name` | Display name for the rule | Yes |
| `pattern` | Glob pattern to match files | Yes |
| `destination` | Target folder (supports variables) | Yes |
| `priority` | Higher priority rules match first | No |

### Variables

Use these variables in `destination`:

| Variable | Example | Description |
|----------|---------|-------------|
| `{year}` | `2024` | Current year |
| `{month}` | `01` | Current month (01-12) |
| `{day}` | `15` | Current day (01-31) |

### Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `include_hidden` | `false` | Include hidden files (starting with `.`) |
| `follow_symlinks` | `false` | Follow symbolic links |
| `default_organize_mode` | `by-type` | Default organization mode |

## Ignore File

Create a `.neatignore` file in any directory to exclude files:

```
# Comments start with #
*.tmp
*.bak
node_modules
.git
.DS_Store
Thumbs.db
```

Patterns follow glob syntax, similar to `.gitignore`.

## CLI Options

Many settings can also be passed via command line:

```bash
# Ignore patterns
neatcli organize ~/Downloads --ignore "*.tmp" --ignore "*.bak"

# Include hidden files
# (not available via CLI, use config file)
```

## View Current Config

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
