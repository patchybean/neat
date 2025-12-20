# quick

Quick actions for common workflows - presets that run with a single command.

## Usage

```bash
neatcli quick <ACTION> [OPTIONS]
```

## Actions

### downloads

Organize `~/Downloads` by file type.

```bash
neatcli quick downloads        # Execute
neatcli quick downloads -n     # Preview only
```

### desktop

Clean up `~/Desktop` by file type.

```bash
neatcli quick desktop          # Execute
neatcli quick desktop -n       # Preview only
```

### photos

Organize photos by EXIF date taken.

```bash
neatcli quick photos                    # Default: ~/Pictures
neatcli quick photos ~/Photos           # Custom path
neatcli quick photos ~/Photos -n        # Preview only
```

### music

Organize music by artist/album structure.

```bash
neatcli quick music                     # Default: ~/Music
neatcli quick music ~/Music -n          # Preview only
```

### cleanup

Find old files in Downloads (older than 30 days by default).

```bash
neatcli quick cleanup                   # Default: 30 days
neatcli quick cleanup --days 7          # Custom threshold
neatcli quick cleanup --trash           # Move to trash
```

## Examples

```bash
# Quick organization workflow
neatcli quick downloads
neatcli quick desktop

# Organize photos from a specific folder
neatcli quick photos ~/DCIM

# Find files older than 2 weeks
neatcli quick cleanup --days 14
```

## See Also

- [organize](organize.md) - Full organize command with all options
- [profile](profile.md) - Save and reuse custom configurations
