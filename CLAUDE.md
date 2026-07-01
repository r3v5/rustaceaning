# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust learning repository organized as a Cargo workspace. Each crate is a standalone exercise or project (e.g., `minigrep`, `binary-search`). Rust edition 2024.

## Build & Test Commands

```bash
# Build entire workspace
cargo build

# Build single crate
cargo build -p minigrep

# Test entire workspace
cargo test

# Test single crate
cargo test -p minigrep

# Run a single test by name
cargo test -p minigrep case_sensitive

# Run minigrep binary
cargo run -p minigrep -- <query> <file_path>

# Case-insensitive search (env var)
IGNORE_CASE=1 cargo run -p minigrep -- <query> <file_path>
```

## Architecture

- **Workspace root** (`Cargo.toml`): lists all member crates in `members`
- Each crate lives in its own directory with independent `Cargo.toml`
- **minigrep**: CLI grep tool. `main.rs` handles args/config/orchestration, `lib.rs` exports search functions (`search`, `search_case_insensitive`)
- **binary-search**: Library crate (scaffold)

## Conventions

- Library logic goes in `lib.rs`, binary entry point in `main.rs`
- Tests live in `#[cfg(test)] mod tests` within the same file as the code they test
- New exercises: add a new crate directory and register it in the workspace `members` array
