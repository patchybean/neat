# history

View the history of operations performed by neatcli.

## Usage

```bash
neatcli history
```

## Description

The `history` command displays a log of all operations performed by neatcli, including:

- File moves from `organize` command
- File deletions from `clean` command
- Duplicate file removals from `duplicates` command
- Similar image removals from `similar` command

This information is stored in `~/.neat/history.json` and is used by the `undo` command to reverse operations.

## Output

```
Operation History
─────────────────────────────────────────────────────────────
[2024-12-20 10:30:15] organize
  Moved: report.pdf → Documents/report.pdf
  Moved: photo.jpg → Images/photo.jpg

[2024-12-20 11:15:00] clean --older-than 30d
  Deleted: old_log.txt
  Deleted: temp_file.tmp
```

## History Storage

History is stored at:

```
~/.neat/history.json
```

!!! info "Automatic Cleanup"
    History entries older than 30 days are automatically cleaned up to prevent the file from growing too large.

## Related Commands

| Command | Description |
|---------|-------------|
| [undo](undo.md) | Undo the last operation using history |
| [organize](organize.md) | Organize files (creates history entries) |
| [clean](clean.md) | Clean files (creates history entries) |

## See Also

- [undo](undo.md) - Undo the last operation
