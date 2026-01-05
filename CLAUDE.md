# Claude Code Instructions for uuid-utils

Welcome to the **uuid-utils** project! This is a Rust-based command-line utility for working with UUIDs.

## Quick Start

For detailed instructions on working with this codebase, including project structure, design principles, and guidelines for making changes, please refer to:

**[AGENTS.md](./AGENTS.md)** - Complete guide for AI agents working on this project

## Project Quick Reference

- **Language**: Rust (2024 edition)
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run**: `cargo run -- <command>`

### Available Commands
- `gen` - Generate new UUIDs (v4, v7)
- `fmt` - Format existing UUIDs
- `info` - Display UUID information
- `nil` - Return the nil UUID (all zeros)
- `normalize` - Convert .NET System.Guid to RFC4122 format

## Documentation

- [AGENTS.md](./AGENTS.md) - Detailed agent instructions and contribution guidelines
- [docs/usage.md](./docs/usage.md) - User-facing usage examples
- [docs/design.md](./docs/design.md) - Architecture and design decisions

## Before Making Changes

1. Read the relevant sections in [AGENTS.md](./AGENTS.md)
2. Review [docs/design.md](./docs/design.md) for architectural context
3. Check existing implementations in `src/` for patterns to follow
4. Run `cargo fmt` and `cargo clippy` before committing

## Notes

- This project follows a modular architecture with clear separation between formatters, generators, and subcommands
- All UUID operations leverage the `uuid` crate - avoid reimplementing core functionality
- Keep documentation in sync with code changes
