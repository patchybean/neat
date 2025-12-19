# undo

Undo the last operation.

## Usage

```bash
neatcli undo
```

## How it Works

NeatCLI tracks all file movements and can restore them:
- Moves are reversed (files returned to original location)
- Deletes cannot be undone (use `--trash` flag instead)

## Examples

```bash
# Undo last operation
neatcli undo

# View operation history
neatcli history
```
