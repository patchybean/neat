# undo

Undo the last file operation.

## Usage

```bash
neatcli undo
```

## How It Works

NeatCLI keeps a history of all file operations:

- **Moves** can be undone (files are moved back)
- **Deletes** cannot be undone (use `--trash` instead)

## Examples

### Undo Last Operation

```bash
neatcli undo
```

Output:
```
→ Undoing 'organize --by-type' (23 operations)...

✓ Restored 23 files
```

### View History

```bash
neatcli history
```

Output:
```
Operation History:
────────────────────────────────────────────────────────────
  2024-12-19 15:30:00  organize --by-type (23 files)
  2024-12-19 14:15:00  organize --by-date (45 files)
  2024-12-18 10:00:00  clean --older-than 30d (12 files)
  ... and 5 more operations
```

## Limitations

!!! warning "Deleted Files"
    Files deleted with `clean --execute` (without `--trash`) cannot be recovered.
    Always use `--trash` for safer cleanup operations.

!!! info "History Size"
    By default, the last 100 operations are kept. Older operations are automatically removed.

## See Also

- [history](undo.md) - View operation history
- [organize](organize.md) - Organize files
