## Context

This design outlines the architecture of `idc` (Instant Developer Companion / Instant Development CLI), a zero-friction CLI utility written in Rust. Currently, developers must manually verify and run distinct commands for building, running, and testing across different languages. `idc` aims to scan the directory structure, identify the build environment, manage missing toolchains interactively, and map standard verbs (`build`, `run`, `test`) to the actual underlying toolchains.

## Goals / Non-Goals

**Goals:**
- Implement a unified Rust CLI `idc` supporting three verbs: `build`, `run`, `test`.
- Automatically identify the build system context by traversing directories upwards from the user's current working directory.
- Support Rust (`cargo`), Go (`go`), Node.js (`npm`/`yarn`/`pnpm`/`bun`), Python (`uv`/`poetry`/`pip`), Java (`mvn`/`gradle`), and .NET (`dotnet`).
- Support argument pass-through (e.g. `idc build -p my-pkg --release`).
- Interactively prompt and guide installation of missing compilers/interpreters using standard bootstrapping tools (e.g. `rustup` for Rust, `uv` for Python, `fnm` for Node).
- Persist polyglot/monorepo build targets locally in `idc.yaml`.

**Non-Goals:**
- Implementing actual compilers, package managers, or bundlers from scratch.
- Supporting daemon/background execution of long-running build processes (beyond normal subprocess execution).
- Designing container-based sandbox builds (e.g., automatically launching Docker containers to compile code).

## Decisions

### 1. Programming Language: Rust
- **Decision:** Build the binary in Rust.
- **Rationale:** Rust produces a fast, statically linked single binary with no external runtime dependencies (such as Python or Node.js). This ensures `idc` starts instantly and can be installed easily.
- **Alternatives Considered:** Go (also offers single binaries, but Rust's ecosystem around CLI parsing via `clap` and type-safe command wrappers is highly robust).

### 2. Configuration Format: `idc.yaml`
- **Decision:** Use YAML format for root configuration overrides.
- **Rationale:** Clean, easy to read, and write for developers. Supported out-of-the-box in Rust using `serde_yaml`.
- **Alternatives Considered:** JSON (harder to write/comment manually), TOML (already used by Cargo and Poetry, might cause minor confusion, but is a viable fallback). YAML was chosen for structure clarity.

### 3. Upward Root Directory Discovery
- **Decision:** Walk upwards starting from the current working directory (`std::env::current_dir()`) to detect build configuration files.
- **Rationale:** Mirroring `git` behavior allows users to run commands anywhere inside a project's subdirectories while executing the build commands at the project root.
- **Alternatives Considered:** Simple current directory scanning (forces users to run commands only at the root, degrading user experience).

### 4. Interactive Dialogs
- **Decision:** Use the `dialoguer` crate for prompt-based installation and project selection.
- **Rationale:** Standard, well-tested Rust crate for terminal user interfaces that handles rendering of selection lists and Y/n choices cleanly.

## Risks / Trade-offs

- **[Risk] Security of Installer Bootstrapping:** Downloading and executing shell scripts (like `rustup.sh` or `fnm` install scripts) could be seen as insecure or surprising.
  - *Mitigation:* `idc` will never run an installation silently. It must print the exact URL and command it is going to run, and ask for explicit user consent (Y/n) before starting.
- **[Risk] Cross-Platform Compilation/Installation:** Windows uses PowerShell or MSI installers, while Linux/macOS use bash scripts.
  - *Mitigation:* Focus the initial implementation on Linux/macOS. For Windows, fallback to printing installation instructions or using native executables like `winget` where available.
