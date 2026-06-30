## Why

On systems where `python` is not aliased in the PATH (but `python3` or `uv` are available), `idc` falsely reports that Python is not installed. We need to check for alternative binary names in the path and dynamically resolve the correct Python executable to run.

## What Changes

- Modify toolchain checking for Python to search for `python`, `python3`, or `uv`.
- Update the Python adapter to execute `python3` if `python` is not available in the path.

## Capabilities

### New Capabilities

*None.*

### Modified Capabilities

- `toolchain-auto-install`: Searches for alternative binaries for Python context verification.
- `cli-core`: Dynamically maps command execution to `python3` if `python` is missing.

## Impact

- Modifies `src/installer.rs` to allow alternative check rules.
- Modifies `src/adapters.rs` to resolve the active python command dynamically.
