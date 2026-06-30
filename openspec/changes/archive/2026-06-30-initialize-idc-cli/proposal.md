## Why

Developers frequently switch between projects written in different languages and using different build systems (e.g., Cargo, Go modules, npm, pip/uv, Maven, Gradle, dotnet). Learning and running specific commands (`cargo build`, `go test ./...`, `npm run build`) and manually managing toolchains adds unnecessary overhead. The `idc` tool simplifies this by providing a unified CLI interface (`idc build`, `idc run`, `idc test`) that automatically detects the project context, configures itself, and runs the correct commands, installing toolchains on-demand if they are missing.

## What Changes

- Create a single, self-contained CLI tool named `idc` written in Rust.
- Implement the core CLI commands: `build`, `run`, and `test`.
- Add a directory scanner that traverses upwards from the current directory to find project roots.
- Implement language-specific adapters for Rust, Go, Node.js, Python, Java, and .NET.
- Introduce interactive selection and persistence using a local `idc.yaml` configuration file for polyglot or monorepo projects.
- Support pass-through of command-line flags directly to the underlying build tool.
- Integrate interactive auto-installation for missing toolchains using standard bootstrap scripts (e.g., `rustup`, `fnm`, `uv`).

## Capabilities

### New Capabilities

- `cli-core`: Core CLI harness that handles upward directory scanning, project context detection, interactive menu selections, argument pass-through, and configuration persistence in `idc.yaml`.
- `toolchain-auto-install`: Feature that detects missing build commands (e.g., `cargo`, `go`, `node`) and prompts the user to automatically install them via standard toolchain bootstrap scripts.

### Modified Capabilities

*None (new project).*

## Impact

- Initializes the project repository as a Rust binary project.
- Relies on crates like `clap` for CLI parsing, `dialoguer` for interactive menus, `serde` and `serde_yaml` for configuration persistence.
