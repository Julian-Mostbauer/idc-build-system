## 1. Build Context Detection Update

- [x] 1.1 Add enum variants `CMake`, `Makefile`, and `Deno` to `BuildContext` in `src/detector.rs`
- [x] 1.2 Implement mapping in `BuildContext::from_name` and `BuildContext::required_binary` for the new variants
- [x] 1.3 Update `scan_directory` in `src/detector.rs` to detect files: `CMakeLists.txt`, `Makefile`/`makefile`/`GNUmakefile`, and `deno.json`/`deno.jsonc`

## 2. Command Execution Adapters

- [x] 2.1 Implement Deno execution adapter in `src/adapters.rs` (supporting default tasks and fallback entrypoints search)
- [x] 2.2 Implement Makefile execution adapter in `src/adapters.rs` (routing command verbs to standard make targets directly)
- [x] 2.3 Implement CMake build/test execution adapter in `src/adapters.rs` (triggering `cmake -B build` configuration dynamically and running ctest)
- [x] 2.4 Implement target binary scanner for CMake run mapping in `src/adapters.rs`

## 3. Installer Bootstrapping Update

- [x] 3.1 Update `get_installer_details` in `src/installer.rs` to include bootstrap scripts and package installs for `cmake`, `make`, and `deno`
- [x] 3.2 Verify project compiles cleanly with warning-free `cargo check`
