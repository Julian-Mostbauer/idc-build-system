## 1. Implement Python Checks

- [x] 1.1 Update `installer::check_toolchain` in `src/installer.rs` to verify `python`, `python3`, or `uv` for Python context
- [x] 1.2 Update `adapters::resolve_command` in `src/adapters.rs` to use `python3` dynamically if `python` is not in PATH
- [x] 1.3 Compile and verify `cargo check` runs warning-free
