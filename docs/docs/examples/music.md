# Music Library Organization

Organize your music collection using audio metadata.

## Basic Organization

### By Artist

```bash
neatcli organize ~/Music --by-artist --execute
```

Result:
```
Music/
├── The Beatles/
│   ├── Hey Jude.mp3
│   └── Yesterday.mp3
├── Taylor Swift/
│   ├── Shake It Off.mp3
│   └── Blank Space.mp3
└── Unknown Artist/
    └── untitled_track.mp3
```

### By Album

Creates Artist/Album hierarchy:

```bash
neatcli organize ~/Music --by-album --execute
```

Result:
```
Music/
├── The Beatles/
│   ├── Abbey Road/
│   │   └── Come Together.mp3
│   └── Let It Be/
│       └── Let It Be.mp3
└── Taylor Swift/
    └── 1989/
        └── Shake It Off.mp3
```

## Filtering

### Only MP3 Files

```bash
neatcli organize ~/Music --mime "audio/mpeg" --execute
```

### Only FLAC (High Quality)

```bash
neatcli organize ~/Music --mime "audio/flac" --execute
```

### Large Audio Files

```bash
# FLAC and WAV files (larger than 30MB)
neatcli organize ~/Music --min-size 30MB --execute
```

## Cleanup

### Find Duplicate Songs

```bash
# Find exact duplicates
neatcli duplicates ~/Music

# Remove duplicates
neatcli duplicates ~/Music --delete --trash --execute
```

### Find Songs with Missing Metadata

Songs without artist tags go to "Unknown Artist":

```bash
# After organizing by artist
ls ~/Music/Unknown\ Artist/
```

## Converting Between Formats

While NeatCLI doesn't convert formats, you can organize by extension to separate them:

```bash
neatcli organize ~/Music --by-extension --execute
```

Result:
```
Music/
├── MP3/
├── FLAC/
├── M4A/
└── WAV/
```

## Full Workflow

```bash
#!/bin/bash

MUSIC_DIR=~/Music

# 1. Remove duplicates first
neatcli duplicates $MUSIC_DIR --delete --trash --execute

# 2. Organize by album
neatcli organize $MUSIC_DIR --by-album --execute

# 3. Show statistics
neatcli stats $MUSIC_DIR
```

## Supported Formats

NeatCLI can read metadata from:

| Format | Extensions | Metadata |
|--------|------------|----------|
| MP3 | .mp3 | ID3v1, ID3v2 |
| FLAC | .flac | Vorbis comments |
| M4A/AAC | .m4a, .aac | iTunes tags |
| OGG | .ogg | Vorbis comments |
| WAV | .wav | Limited |

## Tips

!!! tip "Tagging First"
    For best results, ensure your music files have proper metadata tags.
    Tools like MusicBrainz Picard or Kid3 can help auto-tag files.

!!! tip "Compilations"
    Compilation albums may have different artists per track.
    `--by-album` keeps compilation albums together.
