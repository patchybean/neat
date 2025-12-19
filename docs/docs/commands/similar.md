# similar

Find visually similar images using perceptual hashing.

## Usage

```bash
neatcli similar [OPTIONS] <PATH>
```

## How It Works

Unlike `duplicates` which finds exact copies, `similar` finds images that **look alike**:

1. **Image loading** - Each image is loaded and processed
2. **Perceptual hashing** - A "fingerprint" is created based on visual content
3. **Comparison** - Images with similar fingerprints are grouped together

!!! info "Perceptual Hashing"
    Perceptual hashing creates a hash based on what the image looks like, not its exact bytes.
    This means it can detect similar images even if they have different resolutions, formats, or minor edits.

## Options

| Flag | Description | Default |
|------|-------------|---------|
| `--threshold` | Similarity threshold (0-64) | 5 |
| `--delete` | Delete similar images |  |
| `--trash` | Move to trash | |
| `--execute` `-e` | Execute deletion | |

### Threshold

The threshold controls how similar images must be to match:

| Threshold | Meaning |
|-----------|---------|
| 0-2 | Nearly identical (same image, different format) |
| 3-5 | Very similar (minor crops, resizes) |
| 6-10 | Somewhat similar (different angles, edits) |
| 11+ | Loosely similar (may include false positives) |

!!! tip "Start Low"
    Start with a low threshold (2-3) and increase if you want to find more matches.

## Examples

### Find Similar Images

```bash
neatcli similar ~/Photos
```

Output:
```
→ Scanning /Users/you/Photos for similar images (threshold: 5)...
  Found 500 images to analyze

Similar Images Found:
────────────────────────────────────────────────────────────

  Group 1 (Original: photo.jpg):
    ● /Users/you/Photos/photo.jpg (5.2 MB)
    ○ /Users/you/Photos/photo_edited.jpg (4.8 MB) - distance: 2
    ○ /Users/you/Photos/photo_small.jpg (1.1 MB) - distance: 3

  Group 2 (Original: sunset.png):
    ● /Users/you/Photos/sunset.png (8.0 MB)
    ○ /Users/you/Photos/sunse_crop.jpg (2.5 MB) - distance: 4

────────────────────────────────────────────────────────────

Summary: 3 similar images in 2 groups
Potential space savings: 8.4 MB
```

### Stricter Matching

```bash
# Only find nearly identical images
neatcli similar ~/Photos --threshold 2
```

### Delete Similar Images

```bash
# Move similar images to trash
neatcli similar ~/Photos --delete --trash --execute
```

### Use Cases

- **Photo cleanup** - Find duplicate exports or edits
- **Screenshot cleanup** - Find multiple screenshots of the same thing
- **Backup deduplication** - Find images that exist in multiple folders

## Supported Formats

- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)
- BMP (.bmp)
- TIFF (.tiff, .tif)

## See Also

- [duplicates](duplicates.md) - Find exact duplicate files
- [organize](organize.md) - Organize images by metadata
