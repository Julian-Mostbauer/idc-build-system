## Context

We are adding custom scripting support, shell autocompletions, and robust CMake target detection to `idc`. This requires updating the CLI argument parser, the project configuration struct, and the CMake adapter.

## Goals / Non-Goals

**Goals:**
- Intercept custom subcommands defined under a `commands` key in `idc.yaml` and execute them in the shell.
- Provide a `completion` command that outputs Shell completion scripts for Bash, Zsh, and Fish.
- Enhance CMake execution to read metadata from the build directory (like `CMakeCache.txt`) to locate target binaries.

**Non-Goals:**
- Implementing a full-blown shell inside `idc` (we delegate commands to the system shell `/bin/sh` or `cmd.exe`).
- Rewriting CMake's dependency generation mechanisms.

## Decisions

### 1. Intercepting Custom Subcommands
- **Decision:** Check if `idc.yaml` contains the entered subcommand first. If it does, run the command in the shell (`sh -c` or `cmd /C`) and forward standard inputs/outputs.
- **Rationale:** This prioritizes local scripting overrides and project scripts over default compiler mappings, letting developers override standard verbs if desired.
- **Implementation:** Parse `std::env::args()` to check the first argument. If it matches a key in `commands` map in `idc.yaml`, run it immediately and bypass Clap.

### 2. Generating Completions via `clap_complete`
- **Decision:** Add `clap_complete` crate to `Cargo.toml`. Create a `completion` subcommand in `src/main.rs` that takes `Shell` and calls `clap_complete::generate` writing to `std::io::stdout()`.
- **Rationale:** Leverages the existing Clap parser structure to ensure autocompletion output is always accurate and updated automatically when commands change.

### 3. CMake Target Detection via Project Name
- **Decision:** Parse `build/CMakeCache.txt` to extract the project name variable (e.g. `CMAKE_PROJECT_NAME:STATIC=my_project`) or executable targets. If found, prioritize running `build/my_project` (or similar resolved path) first.
- **Rationale:** Standard CMake projects define a main project name which matches the default build target. Matching this name avoids directory scanning ambiguity.

## Risks / Trade-offs

- **[Risk] Shell Portability of Custom Commands:** Shell commands in `idc.yaml` might use bash-specific features that fail on Windows cmd.exe.
  - *Mitigation:* Document that custom scripts should be written portably, or invoke shell-agnostic scripts/binaries. Run via `/bin/sh` on Unix and `cmd.exe` on Windows.
