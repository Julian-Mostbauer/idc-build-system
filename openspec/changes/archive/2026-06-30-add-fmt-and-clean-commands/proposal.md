## Why

Developers want to format their codebase and clean intermediate build files without remembering ecosystem-specific commands (like Spotless, cargo clean, go clean, or black/ruff formatting). Adding standard `fmt` and `clean` subcommands to `idc` provides a unified interface for code formatting and cleanup.

## What Changes

- Add standard subcommands `fmt` and `clean` to the CLI argument parser.
- Map the `fmt` and `clean` verbs to native executors for Rust, Go, Node.js, Python, Java, .NET, CMake, Makefile, and Deno contexts.
- Provide custom formatting fallbacks (such as Prettier for JS, Ruff/Black for Python, and clang-format for CMake).
- Provide custom cleanup fallbacks (such as deleting common folders for Node.js).

## Capabilities

### New Capabilities

*None.*

### Modified Capabilities

- `cli-core`: Introduces `fmt` and `clean` commands and implements support mappings across all 9 build system contexts.

## Impact

- Modifies `src/main.rs` to support `fmt` and `clean` subcommands.
- Modifies `src/adapters.rs` to implement resolving execution commands for `fmt` and `clean` across all supported contexts.
