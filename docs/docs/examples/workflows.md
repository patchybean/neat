# Common Workflows

Real-world examples of using NeatCLI for everyday tasks.

## Weekly Downloads Cleanup

Clean up your Downloads folder every week:

```bash
# 1. Preview what will be organized
neatcli organize ~/Downloads

# 2. Organize by type
neatcli organize ~/Downloads --execute

# 3. Remove old files (older than 30 days)
neatcli clean ~/Downloads --older-than 30d --trash --execute

# 4. Find and remove duplicates
neatcli duplicates ~/Downloads --delete --trash --execute
```

## Photo Import Workflow

When importing photos from a camera:

```bash
# 1. Copy (not move) from SD card, organize by date taken
neatcli organize /Volumes/SD_CARD/DCIM \
  --by-date-taken \
  --copy \
  --execute

# 2. Find duplicate photos
neatcli duplicates ~/Photos --delete --trash --execute

# 3. Find similar photos (duplicates with edits)
neatcli similar ~/Photos --threshold 5

# 4. Organize by camera model
neatcli organize ~/Photos --by-camera --execute
```

## Music Library Organization

Organize a messy music collection:

```bash
# 1. Organize by artist
neatcli organize ~/Music --by-artist --execute

# 2. Or organize by album (Artist/Album structure)
neatcli organize ~/Music --by-album --execute

# 3. Find duplicate songs
neatcli duplicates ~/Music --delete --trash --execute
```

## Project Archival

Archive old project files:

```bash
# 1. Find old project files (> 1 year)
neatcli stats ~/Projects

# 2. Preview old files
neatcli clean ~/Projects --older-than 1y

# 3. Move to archive (using organize by date)
neatcli organize ~/Projects --by-date --after 2023-01-01 --before 2024-01-01 --execute
```

## Automated Downloads Organization

Set up auto-organization with watch mode:

```bash
# Start watching Downloads folder
neatcli watch ~/Downloads --auto
```

Or create a shell alias:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias clean-downloads='neatcli organize ~/Downloads --execute && neatcli clean ~/Downloads --older-than 30d --trash --execute'
```

## Disk Space Recovery

Find and remove space-wasting files:

```bash
# 1. Get statistics
neatcli stats ~/

# 2. Find large duplicates (> 100MB)
neatcli duplicates ~/ --min-size 100MB

# 3. Find similar large images
neatcli similar ~/Pictures --min-size 5MB

# 4. Clean old files
neatcli clean ~/Downloads --older-than 90d --trash --execute
```

## Backup Cleanup

Clean up backup folder duplicates:

```bash
# Find duplicates between original and backup
neatcli duplicates ~/Backup --delete --trash --execute

# Or find duplicates in specific folders
neatcli duplicates ~/Pictures ~/Backup/Pictures --delete --trash --execute
```
