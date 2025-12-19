# Filters

NeatCLI supports various filters to narrow down which files are processed.

## Size Filters

```bash
--min-size <SIZE>   # Minimum file size
--max-size <SIZE>   # Maximum file size
```

Size formats: `100B`, `10KB`, `5MB`, `1GB`

## Date Filters

```bash
--after <DATE>    # Files modified after date
--before <DATE>   # Files modified before date
```

Date format: `YYYY-MM-DD` or `YYYY/MM/DD`

## Name Filters

```bash
--startswith <STRING>   # Filename starts with
--endswith <STRING>     # Filename ends with (before extension)
--contains <STRING>     # Filename contains
```

## Regex Filter

```bash
--regex <PATTERN>   # Match filename with regex
```

Example: `--regex "^IMG_\d{4}"`

## MIME Type Filter

```bash
--mime <TYPE>   # Filter by MIME type
```

Examples:
- `--mime "image/*"` - All images
- `--mime "image/jpeg"` - Only JPEG
- `--mime "application/pdf"` - Only PDF

## Combining Filters

All filters can be combined:

```bash
neatcli organize ~/Downloads \
  --mime "image/*" \
  --min-size 1MB \
  --after 2024-01-01 \
  --execute
```
