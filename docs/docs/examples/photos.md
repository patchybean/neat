# Photo Organization

Complete guide to organizing your photo library with NeatCLI.

## Basic Organization

### By Date Taken (Recommended)

Uses EXIF metadata for accurate dates:

```bash
neatcli organize ~/Photos --by-date-taken --execute
```

Result:
```
Photos/
├── 2024/
│   ├── 01/
│   │   ├── IMG_0001.jpg
│   │   └── IMG_0002.jpg
│   ├── 02/
│   │   └── vacation_001.jpg
│   └── 12/
│       └── christmas.jpg
└── 2023/
    └── ...
```

### By Camera Model

Separate photos by device:

```bash
neatcli organize ~/Photos --by-camera --execute
```

Result:
```
Photos/
├── iPhone 15 Pro/
│   ├── IMG_0001.jpg
│   └── IMG_0002.jpg
├── Canon EOS R5/
│   └── _DSC0001.jpg
└── Sony A7 III/
    └── DSC00001.jpg
```

### By File Date

Fallback for photos without EXIF:

```bash
neatcli organize ~/Photos --by-date --execute
```

Uses file modification date instead of EXIF date taken.

## Filtering Photos

### Only Large Photos

```bash
# Photos larger than 5MB (skip thumbnails)
neatcli organize ~/Photos --min-size 5MB --execute
```

### Only JPEGs

```bash
neatcli organize ~/Photos --mime "image/jpeg" --execute
```

### iPhone Photos Only

```bash
# Filter by filename pattern
neatcli organize ~/Photos --startswith "IMG_" --execute
```

### Photos from 2024

```bash
neatcli organize ~/Photos --after 2024-01-01 --before 2025-01-01 --execute
```

## Cleanup

### Find Duplicate Photos

```bash
# Find exact duplicates
neatcli duplicates ~/Photos

# Delete duplicates (keeps first copy)
neatcli duplicates ~/Photos --delete --trash --execute
```

### Find Similar Photos

Find near-duplicates (different sizes, edits, exports):

```bash
# Default threshold
neatcli similar ~/Photos

# Stricter matching
neatcli similar ~/Photos --threshold 3

# Delete similar photos
neatcli similar ~/Photos --delete --trash --execute
```

## Importing from Camera

### Copy from SD Card

```bash
# Copy (not move) and organize by date taken
neatcli organize /Volumes/SD_CARD/DCIM \
  --by-date-taken \
  --copy \
  --execute
```

### Verify Import

```bash
# Check for duplicates after import
neatcli duplicates ~/Photos

# Get statistics
neatcli stats ~/Photos
```

## Full Workflow Example

Complete photo organization workflow:

```bash
#!/bin/bash

PHOTOS_DIR=~/Photos

# 1. Import from SD card
if [ -d "/Volumes/SD_CARD/DCIM" ]; then
  neatcli organize /Volumes/SD_CARD/DCIM \
    --by-date-taken \
    --copy \
    --execute
fi

# 2. Remove duplicates
neatcli duplicates $PHOTOS_DIR --delete --trash --execute

# 3. Find similar images for review
neatcli similar $PHOTOS_DIR --threshold 3

# 4. Show statistics
neatcli stats $PHOTOS_DIR
```

## Tips

!!! tip "EXIF Data"
    `--by-date-taken` uses EXIF DateTimeOriginal, which is the actual capture time.
    `--by-date` uses file modification date, which may change when copying files.

!!! tip "RAW + JPEG"
    If you shoot RAW+JPEG, both files will have the same name but different extensions.
    Use `--by-extension` to separate them, or keep them together with `--by-date-taken`.

!!! tip "Screenshots"
    To exclude screenshots, use:
    ```bash
    neatcli organize ~/Photos --regex "^(?!Screenshot)" --execute
    ```
