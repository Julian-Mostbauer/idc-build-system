## Why

Developers using `idc` want to run custom scripts and aliases defined locally, output shell autocompletions for their environments, and have a more robust mechanism for discovering compiled executable targets in CMake projects rather than relying on directory scanning.

## What Changes

- Add support for executing custom scripting tasks defined inside a `commands` mapping in `idc.yaml`.
- Introduce a shell completion generator subcommand `idc completion <shell>` using `clap_complete`.
- Upgrade the CMake runner's target detection to inspect the generated build directory configuration files (such as `CMakeCache.txt` or target lists) to resolve executable targets.

## Capabilities

### New Capabilities

*None.*

### Modified Capabilities

- `cli-core`: Resolves and runs custom commands from `idc.yaml`, generates shell autocompletions, and parses CMake metadata files for target resolution.

## Impact

- Adds `clap_complete` to `Cargo.toml`.
- Updates `src/config.rs` to parse the `commands` dictionary.
- Updates `src/main.rs` to add the `completion` command and intercept custom scripts.
- Modifies `src/adapters.rs` to parse CMake build configurations.
