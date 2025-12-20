# profile

Save and manage reusable organize command configurations.

## Usage

```bash
neatcli profile <ACTION> [OPTIONS]
```

## Actions

### save

Save organize options as a named profile.

```bash
neatcli profile save <NAME> [OPTIONS]
```

**Options:**

| Flag | Description |
|------|-------------|
| `-d, --description` | Profile description |
| `-p, --paths` | Target directories |
| `--by-type` | Organize by file type |
| `--by-date` | Organize by date |
| `--by-extension` | Organize by extension |
| `--by-camera` | Organize by camera (EXIF) |
| `--by-date-taken` | Organize by date taken (EXIF) |
| `--by-artist` | Organize by artist |
| `--by-album` | Organize by album |
| `-r, --recursive` | Include subdirectories |
| `-c, --copy` | Copy instead of move |
| `--on-conflict` | Conflict strategy |

**Example:**

```bash
neatcli profile save my-photos \
  --paths ~/Photos \
  --by-date-taken \
  --recursive \
  --description "Organize all photos by date"
```

### list

List all saved profiles.

```bash
neatcli profile list
```

Output:
```
Saved profiles:
  ● my-photos (by-date-taken)
      Organize all photos by date
  ● downloads (by-type)
```

### run

Execute a saved profile.

```bash
neatcli profile run <NAME>           # Execute
neatcli profile run <NAME> -n        # Preview only
```

### show

Display profile details.

```bash
neatcli profile show <NAME>
```

### delete

Remove a saved profile.

```bash
neatcli profile delete <NAME>
```

## Storage

Profiles are stored as TOML files in `~/.neat/profiles/`.

## Examples

```bash
# Create profiles for common tasks
neatcli profile save work-docs \
  --paths ~/Documents/Work \
  --by-type \
  --description "Organize work documents"

neatcli profile save camera-roll \
  --paths ~/DCIM \
  --by-date-taken \
  --recursive

# Run a profile
neatcli profile run camera-roll

# Preview before running
neatcli profile run work-docs -n
```

## See Also

- [organize](organize.md) - Full organize command reference
- [quick](quick.md) - Built-in preset actions
