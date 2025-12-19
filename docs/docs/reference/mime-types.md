# MIME Types Reference

Complete reference of MIME types for filtering files.

## Wildcard Matching

Use `*` to match all subtypes:

```bash
neatcli organize ~/Downloads --mime "image/*"   # All images
neatcli organize ~/Downloads --mime "video/*"   # All videos
neatcli organize ~/Downloads --mime "audio/*"   # All audio
```

## Images

| MIME Type | Extensions |
|-----------|------------|
| `image/jpeg` | .jpg, .jpeg |
| `image/png` | .png |
| `image/gif` | .gif |
| `image/webp` | .webp |
| `image/svg+xml` | .svg |
| `image/bmp` | .bmp |
| `image/tiff` | .tiff, .tif |
| `image/heic` | .heic |
| `image/heif` | .heif |
| `image/*` | All of the above |

## Videos

| MIME Type | Extensions |
|-----------|------------|
| `video/mp4` | .mp4 |
| `video/quicktime` | .mov |
| `video/x-msvideo` | .avi |
| `video/x-matroska` | .mkv |
| `video/webm` | .webm |
| `video/x-flv` | .flv |
| `video/mpeg` | .mpeg, .mpg |
| `video/*` | All of the above |

## Audio

| MIME Type | Extensions |
|-----------|------------|
| `audio/mpeg` | .mp3 |
| `audio/wav` | .wav |
| `audio/flac` | .flac |
| `audio/aac` | .aac |
| `audio/ogg` | .ogg |
| `audio/x-m4a` | .m4a |
| `audio/midi` | .mid, .midi |
| `audio/*` | All of the above |

## Documents

| MIME Type | Extensions |
|-----------|------------|
| `application/pdf` | .pdf |
| `application/msword` | .doc |
| `application/vnd.openxmlformats-officedocument.wordprocessingml.document` | .docx |
| `application/vnd.ms-excel` | .xls |
| `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet` | .xlsx |
| `application/vnd.ms-powerpoint` | .ppt |
| `application/vnd.openxmlformats-officedocument.presentationml.presentation` | .pptx |
| `text/plain` | .txt |
| `text/markdown` | .md |
| `text/html` | .html |
| `text/csv` | .csv |

## Archives

| MIME Type | Extensions |
|-----------|------------|
| `application/zip` | .zip |
| `application/gzip` | .gz |
| `application/x-tar` | .tar |
| `application/x-rar-compressed` | .rar |
| `application/x-7z-compressed` | .7z |
| `application/x-bzip2` | .bz2 |

## Code

| MIME Type | Extensions |
|-----------|------------|
| `text/javascript` | .js |
| `text/x-python` | .py |
| `text/x-rust` | .rs |
| `text/x-c` | .c, .h |
| `text/x-java` | .java |
| `application/json` | .json |
| `application/xml` | .xml |
| `text/x-yaml` | .yaml, .yml |

## Examples

```bash
# Only PDF documents
neatcli organize ~/Documents --mime "application/pdf" --execute

# All images except GIFs
neatcli organize ~/Photos --mime "image/*" --regex ".*\.gif$" --execute

# Video files
neatcli organize ~/Downloads --mime "video/*" --execute
```

## See Also

- [Filters Reference](filters.md)
- [organize command](../commands/organize.md)
