## 1. CLI Commands Setup

- [x] 1.1 Add `fmt` and `clean` subcommands to `Cli` parser and `Commands` enum in `src/main.rs`
- [x] 1.2 Map the subcommands to target actions in `main.rs` to route to `adapters::run_context_command`

## 2. Implement Format & Clean Adapters

- [x] 2.1 Implement `fmt` and `clean` command execution for Rust, Go, .NET, and Deno in `src/adapters.rs`
- [x] 2.2 Implement `fmt` and `clean` command execution for Node.js (with custom script verification and fallbacks) in `src/adapters.rs`
- [x] 2.3 Implement `fmt` and `clean` command execution for Python (Ruff vs Black formatting and directory/cache cleanups) in `src/adapters.rs`
- [x] 2.4 Implement `fmt` and `clean` command execution for CMake, Makefile, and Java in `src/adapters.rs`

## 3. Verification & Documentation

- [x] 3.1 Verify and build release binary using `cargo build --release`
- [x] 3.2 Update `README.md` to document the new `fmt` and `clean` commands
