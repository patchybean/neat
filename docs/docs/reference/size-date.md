# Size & Date Filters

Filter files by size and modification date.

## Size Filters

### --min-size

Only include files larger than the specified size:

```bash
neatcli organize ~/Downloads --min-size 10MB --execute
```

### --max-size

Only include files smaller than the specified size:

```bash
neatcli organize ~/Downloads --max-size 100MB --execute
```

### Size Format

| Format | Example | Bytes |
|--------|---------|-------|
| Bytes | `1000` or `1000B` | 1,000 |
| Kilobytes | `100KB` | 100,000 |
| Megabytes | `50MB` | 50,000,000 |
| Gigabytes | `1GB` | 1,000,000,000 |

!!! note "Base 10"
    Sizes use base 10 (1 KB = 1,000 bytes), not base 2 (1 KiB = 1,024 bytes).

### Examples

```bash
# Files between 1MB and 100MB
neatcli organize ~/Downloads --min-size 1MB --max-size 100MB --execute

# Large images only
neatcli organize ~/Photos --mime "image/*" --min-size 5MB --execute

# Small documents
neatcli organize ~/Documents --max-size 500KB --execute
```

## Date Filters

### --after

Only include files modified **after** the specified date:

```bash
neatcli organize ~/Downloads --after 2024-01-01 --execute
```

### --before

Only include files modified **before** the specified date:

```bash
neatcli organize ~/Downloads --before 2024-06-01 --execute
```

### Date Formats

Supported formats:

| Format | Example |
|--------|---------|
| ISO 8601 | `2024-01-15` |
| Slash | `2024/01/15` |

### Examples

```bash
# Files from 2024
neatcli organize ~/Downloads \
  --after 2024-01-01 \
  --before 2025-01-01 \
  --execute

# Files from last quarter
neatcli organize ~/Documents \
  --after 2024-10-01 \
  --execute

# Old files (before 2020)
neatcli clean ~/Archive --before 2020-01-01 --trash --execute
```

## Combining Size and Date

All filters work together:

```bash
# Large recent files
neatcli organize ~/Downloads \
  --min-size 50MB \
  --after 2024-06-01 \
  --execute

# Small old files (candidates for cleanup)
neatcli clean ~/Downloads \
  --max-size 1MB \
  --before 2024-01-01 \
  --trash \
  --execute
```

## See Also

- [Filters Reference](filters.md) - Name and MIME filters
- [clean command](../commands/clean.md) - Remove old files
