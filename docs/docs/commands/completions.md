# completions

Generate shell completion scripts for command auto-completion.

## Usage

```bash
neatcli completions <SHELL>
```

## Supported Shells

| Shell | Description |
|-------|-------------|
| `bash` | Bash completion |
| `zsh` | Zsh completion |
| `fish` | Fish shell completion |
| `powershell` | PowerShell completion |
| `elvish` | Elvish shell completion |

## Installation

### Bash

```bash
# Add to your .bashrc
neatcli completions bash > ~/.local/share/bash-completion/completions/neatcli

# Or for system-wide installation
sudo neatcli completions bash > /etc/bash_completion.d/neatcli
```

### Zsh

```bash
# Add to your .zshrc, make sure fpath includes this directory
neatcli completions zsh > ~/.zfunc/_neatcli

# Then add to .zshrc:
# fpath+=~/.zfunc
# autoload -Uz compinit && compinit
```

### Fish

```bash
neatcli completions fish > ~/.config/fish/completions/neatcli.fish
```

### PowerShell

```powershell
# Add to your PowerShell profile
neatcli completions powershell > _neatcli.ps1
. ./_neatcli.ps1
```

## Example

After installation, you can use Tab to auto-complete commands and options:

```bash
neatcli org<TAB>       # Completes to "organize"
neatcli organize --by<TAB>  # Shows: --by-type --by-date --by-extension ...
```

## See Also

- [Quick Start](../getting-started/quickstart.md) - Get started with neatcli
- [Configuration](../getting-started/configuration.md) - Configure neatcli
