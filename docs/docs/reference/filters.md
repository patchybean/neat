# Filters Reference

NeatCLI provides powerful filtering options to select exactly which files to process.

## Name Filters

Filter files based on their filename.

### --startswith

Match files whose name starts with a string:

```bash
neatcli organize ~/Photos --startswith "IMG_" --execute
```

Matches: `IMG_0001.jpg`, `IMG_vacation.png`  
Doesn't match: `photo.jpg`, `my_IMG_file.jpg`

### --endswith

Match files whose name ends with a string (before extension):

```bash
neatcli organize ~/Downloads --endswith "_backup" --execute
```

Matches: `file_backup.zip`, `data_backup.sql`  
Doesn't match: `backup.zip`, `backup_file.zip`

### --contains

Match files whose name contains a string:

```bash
neatcli organize ~/Documents --contains "2024" --execute
```

Matches: `report_2024.pdf`, `2024_budget.xlsx`, `tax_2024_final.doc`

## Regex Filter

### --regex

Use regular expressions for advanced matching:

```bash
# Files starting with IMG_ followed by 4 digits
neatcli organize ~/Photos --regex "^IMG_\d{4}" --execute

# Files with dates in format YYYY-MM-DD
neatcli organize ~/Downloads --regex "\d{4}-\d{2}-\d{2}" --execute
```

!!! tip "Regex Syntax"
    NeatCLI uses Rust regex syntax, which is similar to Perl/PCRE.

Common patterns:

| Pattern | Matches |
|---------|---------|
| `^IMG_` | Starts with "IMG_" |
| `\.pdf$` | Ends with ".pdf" |
| `\d{4}` | 4 digits |
| `[A-Z]+` | One or more uppercase letters |
| `.*backup.*` | Contains "backup" anywhere |

## MIME Type Filter

### --mime

Filter by file type using MIME types:

```bash
# All images
neatcli organize ~/Downloads --mime "image/*" --execute

# Only JPEG images
neatcli organize ~/Downloads --mime "image/jpeg" --execute

# Only PDFs
neatcli organize ~/Downloads --mime "application/pdf" --execute
```

Common MIME types:

| MIME Type | Files |
|-----------|-------|
| `image/*` | All images |
| `image/jpeg` | JPEG only |
| `image/png` | PNG only |
| `video/*` | All videos |
| `audio/*` | All audio |
| `application/pdf` | PDF documents |
| `application/zip` | ZIP archives |
| `text/*` | All text files |
| `text/plain` | Plain text |

## Combining Filters

All filters can be combined. Files must match **all** specified filters:

```bash
# JPEG images larger than 5MB from 2024
neatcli organize ~/Photos \
  --mime "image/jpeg" \
  --min-size 5MB \
  --after 2024-01-01 \
  --execute

# Files starting with "IMG_" that contain "vacation"
neatcli organize ~/Photos \
  --startswith "IMG_" \
  --contains "vacation" \
  --execute
```

## See Also

- [Size & Date Filters](size-date.md)
- [MIME Types Reference](mime-types.md)
