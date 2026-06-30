## ADDED Requirements

### Requirement: Unified Formatting Command Execution
The system SHALL support the subcommand verb `fmt` and map it to native formatting tools in each ecosystem (e.g. `cargo fmt` for Rust, `go fmt ./...` for Go, `dotnet format` for .NET, and `deno fmt` for Deno).

#### Scenario: Running fmt in a Go project
- **WHEN** the user executes `idc fmt` in a Go project
- **THEN** the system SHALL execute `go fmt ./...` in the project root

### Requirement: Unified Clean Command Execution
The system SHALL support the subcommand verb `clean` and map it to clean target/artifact commands in each ecosystem (e.g. `cargo clean` for Rust, `go clean -i -cache` for Go, `dotnet clean` for .NET, and `cmake --build build --target clean` for CMake).

#### Scenario: Running clean in a CMake project
- **WHEN** the user executes `idc clean` in a CMake project
- **THEN** the system SHALL execute `cmake --build build --target clean` in the project root

### Requirement: Node.js Formatting and Cleaning Fallbacks
For Node.js projects, the system SHALL check `package.json` for custom scripts. If no custom `format`/`fmt` script is defined, it SHALL run `npx prettier --write .`. If no custom `clean` script is defined, it SHALL recursively delete standard build output directories (`dist/`, `build/`, `out/`, `.next/`).

#### Scenario: Cleaning Node.js project without clean script
- **WHEN** the user executes `idc clean` in a Node project containing no custom `clean` script in `package.json`
- **THEN** the system SHALL delete `dist/`, `build/`, `out/`, and `.next/` folders if they exist

### Requirement: Python Formatting Fallback
For Python projects, the system SHALL prioritize formatting code using `ruff format .` (running via `uv` or `poetry` if applicable). If `ruff` is not configured, it SHALL fall back to `black .`.

#### Scenario: Formatting Python codebase with Ruff
- **WHEN** the user runs `idc fmt` in a Python project where `ruff` is configured
- **THEN** the system SHALL execute `ruff format .` (or its `uv`/`poetry` mapped command)
