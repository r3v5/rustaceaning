# rustaceaning

Rust learning projects organized as a Cargo workspace.

## Projects

| Crate | Description |
|-------|-------------|
| `minigrep` | CLI grep tool with case-sensitive and case-insensitive search |
| `binary-search` | Binary search algorithm implementation |

## Usage

```bash
# Build all
cargo build

# Test all
cargo test

# Run minigrep
cargo run -p minigrep -- <query> <file>

# Case-insensitive search
IGNORE_CASE=1 cargo run -p minigrep -- <query> <file>
```
