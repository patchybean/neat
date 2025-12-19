---
description: Coding rules and guidelines for neatcli project
---

# NeatCLI Coding Guidelines

## Project Structure
```
src/
├── main.rs          # Entry point only (~50 lines)
├── cli.rs           # CLI definitions (clap)
├── commands/        # Command handlers
├── core/            # Core logic (scanner, organizer, etc.)
└── utils/           # Utilities (logger, export, etc.)
```

## Code Rules

### 1. Module Organization
- Each command goes in `src/commands/<command>.rs`
- Core logic goes in `src/core/`
- Utilities go in `src/utils/`
- Keep `main.rs` minimal - only argument parsing and dispatch

### 2. Naming Conventions
- Functions: `snake_case`
- Structs/Enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Files: `snake_case.rs`

### 3. Error Handling
- Use `anyhow::Result` for functions that can fail
- Use `thiserror` for custom error types
- Prefer `?` operator over `.unwrap()`

### 4. Dependencies
- Add new dependencies to `Cargo.toml` with version pinning
- Prefer well-maintained crates with active development

### 5. Testing
- Add unit tests in the same file using `#[cfg(test)]` module
- Integration tests go in `tests/` directory
- Run `cargo test` before committing

### 6. Code Quality
// turbo-all
```bash
cargo fmt          # Format code
cargo clippy -- -D warnings  # Lint with warnings as errors
cargo test         # Run all tests
```

### 7. Commit Messages
- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks
- `test:` - Adding tests

### 8. Version Bumping
1. Update `Cargo.toml` version
2. Commit with `chore: bump version to X.Y.Z`
3. Tag with `git tag vX.Y.Z`
4. Push with tags: `git push && git push --tags`

### 9. Release Process
1. GitHub Actions auto-builds binaries on tag push
2. Update Homebrew formula with new SHA256
3. Run `cargo publish` for crates.io

### 10. Documentation
- **Every new feature MUST have documentation**
- Add docs in `docs/docs/` following MkDocs structure
- Update `docs/mkdocs.yml` navigation if adding new pages
- For new commands: `docs/docs/commands/<command>.md`
- For new filters/options: update `docs/docs/reference/`
- Commit docs with `docs:` prefix

