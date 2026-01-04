# Agent Instructions for uuid-utils

This document provides guidance for AI agents (including Claude Code) working on the uuid-utils codebase. It outlines the project structure, design principles, and guidelines for making changes.

## Project Overview

**uuid-utils** is a command-line utility written in Rust for working with UUIDs. It provides functionality to:
- Generate UUIDs (v4, v7)
- Format UUIDs in various representations (canonical, compact, hex, uppercase)
- Display detailed information about UUIDs
- Normalize .NET System.Guid byte order to RFC4122 format

### Tech Stack
- **Language**: Rust (2024 edition)
- **CLI Framework**: clap v4.5.54 (with derive features)
- **UUID Library**: uuid v1.19.0 (with v4 and v7 features)

## Project Structure

```
uuid-utils/
├── src/
│   ├── main.rs              # CLI entry point and command routing
│   ├── formatters.rs        # UUID formatting functions
│   ├── generators.rs        # UUID generation functions
│   └── subcommands/
│       ├── mod.rs           # Subcommand module exports
│       ├── generate.rs      # `gen` subcommand implementation
│       ├── fmt.rs           # `fmt` subcommand implementation
│       ├── info.rs          # `info` subcommand implementation
│       └── normalize.rs     # `normalize` subcommand implementation
├── docs/
│   ├── usage.md             # User-facing usage examples
│   └── design.md            # Architecture and design decisions
├── Cargo.toml               # Project dependencies
└── AGENTS.md               # This file
```

## Design Principles

### 1. Separation of Concerns
- **Formatters** (`src/formatters.rs`): Each format type (canonical, compact, hex) has its own dedicated formatter function for expandability
- **Generators** (`src/generators.rs`): Each UUID version (v4, v7, etc.) has its own generator function
- **Subcommands** (`src/subcommands/`): Each CLI command has its own module with isolated implementation

### 2. Leverage Existing Libraries
- The project uses the battle-tested `uuid` crate rather than reimplementing UUID functionality from scratch
- This ensures correctness and RFC4122 compliance

### 3. CLI Design
- Uses clap's derive API for clean, declarative command definitions
- Supports both positional arguments and piped input (e.g., for `fmt` command)
- Follows Unix philosophy: do one thing well, compose with other tools

## Guidelines for Making Changes

### Adding a New UUID Version

1. **Update dependencies** in `Cargo.toml`:
   ```toml
   [dependencies.uuid]
   version = "1.19.0"
   features = ["v4", "v7", "v8"]  # Add new version feature
   ```

2. **Add generator** in `src/generators.rs`:
   ```rust
   pub fn generate_v8() -> Uuid {
       // Implementation
   }
   ```

3. **Update `gen` subcommand** in `src/subcommands/generate.rs`:
   - Add new version to match statement
   - Update help text if needed

4. **Update documentation** in `docs/usage.md` with examples

### Adding a New Format

1. **Add formatter** in `src/formatters.rs`:
   ```rust
   pub fn format_base64(uuid: &Uuid) -> String {
       // Implementation
   }
   ```

2. **Update `fmt` subcommand** in `src/subcommands/fmt.rs`:
   - Add new flag to `FmtArgs` struct
   - Update command enum in `main.rs` if needed
   - Update formatter logic

3. **Add tests** to verify formatting correctness

4. **Update documentation** in `docs/usage.md`

### Adding a New Subcommand

1. **Create new module** in `src/subcommands/<name>.rs`:
   ```rust
   pub struct <Name>Args {
       // Command arguments
   }

   pub fn run(args: <Name>Args) {
       // Implementation
   }
   ```

2. **Export module** in `src/subcommands/mod.rs`:
   ```rust
   pub mod <name>;
   ```

3. **Add command variant** in `src/main.rs`:
   - Add to `Commands` enum with clap annotations
   - Add match arm in main function to call `subcommands::<name>::run()`

4. **Document usage** in `docs/usage.md`

5. **Update design docs** in `docs/design.md` if architectural changes are needed

### Code Style and Quality

1. **Follow Rust conventions**:
   - Use `cargo fmt` for consistent formatting
   - Run `cargo clippy` to catch common mistakes
   - Write idiomatic Rust code

2. **Error handling**:
   - Use `Result` types for operations that can fail
   - Provide clear error messages to users
   - Handle invalid UUID input gracefully

3. **Testing**:
   - Add unit tests for new formatters and generators
   - Add integration tests for new subcommands
   - Test edge cases (invalid input, empty input, etc.)

4. **Documentation**:
   - Add doc comments (`///`) to public functions
   - Keep `docs/usage.md` up to date with examples
   - Update `docs/design.md` when making architectural changes

## Common Tasks

### Running the Project
```bash
cargo run -- gen v4
cargo run -- fmt 550e8400-e29b-41d4-a716-446655440000 --compact
cargo run -- info 550e8400-e29b-41d4-a716-446655440000
cargo run -- normalize D004A78FD44C1B4E8213324AE10814DC
```

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Linting
```bash
cargo clippy
cargo fmt --check
```

## Special Considerations

### .NET System.Guid Normalization

The `normalize` command handles the unique byte order used by .NET's System.Guid, which differs from RFC4122:
- First 4 bytes (Data1): Little-endian
- Next 2 bytes (Data2): Little-endian
- Next 2 bytes (Data3): Little-endian
- Last 8 bytes: Big-endian (unchanged)

When working on this feature, ensure you understand the byte-swapping logic and test with known .NET-generated GUIDs.

### UUID Versions

Currently supported:
- **v4**: Random UUIDs
- **v7**: Time-ordered UUIDs (recommended for new applications)

If adding new versions (v1, v3, v5, v6, v8), ensure the `uuid` crate feature is enabled and the generator is properly implemented.

## Getting Help

- **Usage examples**: See `docs/usage.md`
- **Design decisions**: See `docs/design.md`
- **Rust documentation**: Run `cargo doc --open`
- **UUID spec**: RFC4122 for standard UUID behavior

## Notes for AI Agents

- Always read the relevant source files before making changes
- Maintain consistency with existing code style and patterns
- When adding features, follow the established separation of concerns
- Update all relevant documentation when making changes
- Test changes thoroughly before considering them complete
- If uncertain about design decisions, check `docs/design.md` or ask the user
