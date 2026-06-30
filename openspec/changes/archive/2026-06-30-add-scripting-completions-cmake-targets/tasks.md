## 1. Setup & Dependencies

- [x] 1.1 Add `clap_complete` to `Cargo.toml` dependencies

## 2. Custom Scripting in `idc.yaml`

- [x] 2.1 Update `IdcConfig` struct in `src/config.rs` to include a deserializable `commands` dictionary
- [x] 2.2 Intercept the command-line subcommand argument early in `src/main.rs` and run the matching shell command if it exists in the `idc.yaml` scripting registry

## 3. Shell Completions Command

- [x] 3.1 Add `completion` subcommand to CLI parser in `src/main.rs` accepting Zsh, Bash, and Fish shells
- [x] 3.2 Implement completion script generator using `clap_complete` writing directly to stdout


## 4. CMake Target Cache Resolution

- [x] 4.1 Implement a parser for `build/CMakeCache.txt` in `src/adapters.rs` to locate the main target compilation project name
- [x] 4.2 Update `scan_for_executables` fallback logic in `src/adapters.rs` to prioritize execution of the binary matching the parsed project name
