# Installation

NeatCLI can be installed in several ways depending on your platform and preferences.

## Cargo (Recommended)

If you have Rust installed, the easiest way is via cargo:

```bash
cargo install neatcli
```

This will download, compile, and install the latest version from [crates.io](https://crates.io/crates/neatcli).

!!! note "Requirements"
    Rust 1.70 or later is required.

## Homebrew (macOS/Linux)

For macOS and Linux users, NeatCLI is available via Homebrew:

```bash
brew tap patchybean/tap
brew install neatcli
```

To upgrade to the latest version:
```bash
brew upgrade neatcli
```

## Pre-built Binaries

Pre-built binaries are available for:

| Platform | Architecture | Download |
|----------|--------------|----------|
| macOS | Apple Silicon (M1/M2/M3) | [neatcli-aarch64-apple-darwin.tar.gz](https://github.com/patchybean/neatcli/releases/latest) |
| macOS | Intel x86_64 | [neatcli-x86_64-apple-darwin.tar.gz](https://github.com/patchybean/neatcli/releases/latest) |
| Linux | x86_64 | [neatcli-x86_64-unknown-linux-gnu.tar.gz](https://github.com/patchybean/neatcli/releases/latest) |

Download and extract:
```bash
tar -xzf neatcli-*.tar.gz
sudo mv neatcli /usr/local/bin/
```

## From Source

Clone and build from source:

```bash
git clone https://github.com/patchybean/neatcli.git
cd neatcli
cargo build --release
```

The binary will be at `target/release/neatcli`.

## Verify Installation

Check that NeatCLI is installed correctly:

```bash
neatcli --version
```

Expected output:
```
neatcli 0.3.0
```

## Shell Completions

Generate shell completions for your shell:

=== "Bash"
    ```bash
    neatcli completions bash > ~/.local/share/bash-completion/completions/neatcli
    ```

=== "Zsh"
    ```bash
    neatcli completions zsh > ~/.zfunc/_neatcli
    ```

=== "Fish"
    ```bash
    neatcli completions fish > ~/.config/fish/completions/neatcli.fish
    ```

## Next Steps

- [Quick Start Guide](quickstart.md) - Learn the basics
- [Configuration](configuration.md) - Customize NeatCLI
