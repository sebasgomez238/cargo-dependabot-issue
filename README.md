# cargo-dependabot-issue

Reproduction repository for https://github.com/dependabot/dependabot-core/issues/13523

## Purpose

This repository demonstrates the issue where Dependabot fails to detect `.cargo/config.toml` files at the root of a repository. The issue is that when Dependabot tries to fetch the `.cargo/config.toml` file, it incorrectly prepends the job's starting directory instead of walking from the repository root up to find the configuration file.

## Repository Structure

```
.
├── .cargo/
│   └── config.toml          # Cargo configuration file that Dependabot should find
├── Cargo.toml               # Main project manifest with outdated dependencies
├── src/
│   └── main.rs              # Simple Rust application
└── README.md
```

## Dependencies

The `Cargo.toml` includes intentionally outdated versions of popular Rust crates:
- `serde` v1.0.130 (newer versions available)
- `tokio` v1.15.0 (newer versions available)
- `anyhow` v1.0.50 (newer versions available)

This allows Dependabot to detect available updates and test whether it can properly handle the `.cargo/config.toml` file during the update process.

## Building

```bash
cargo check
cargo build
cargo run
```

## Expected Behavior

Dependabot should:
1. Find and read the `Cargo.toml` file
2. Locate and read the `.cargo/config.toml` file at the repository root
3. Detect outdated dependencies
4. Propose updates for the dependencies

## Actual Issue

Dependabot fails to find `.cargo/config.toml` because it incorrectly prepends the job's directory path when trying to fetch the file, instead of searching from the repository root upward as Cargo does natively.

