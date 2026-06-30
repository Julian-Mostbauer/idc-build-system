## 1. Project Scaffolding & Setup

- [x] 1.1 Scaffold the new binary Cargo project for `idc` in the workspace root
- [x] 1.2 Add dependencies (`clap`, `serde`, `serde_yaml`, `dialoguer`, `tokio`, `anyhow`, `which`) to `Cargo.toml`
- [x] 1.3 Set up main module hierarchy (`src/main.rs`, `src/config.rs`, `src/detector.rs`, `src/adapters.rs`, `src/installer.rs`)

## 2. CLI Parsing & Configuration Persistence

- [x] 2.1 Implement CLI parser using `clap` supporting `build`, `run`, and `test` verbs with trailing argument capture
- [x] 2.2 Implement the local configuration parser and serializer for `idc.yaml` using `serde`
- [x] 2.3 Implement the configuration writer to save selected defaults to the project root `idc.yaml`

## 3. Context Detection & Traversal

- [x] 3.1 Implement parent directory climbing algorithm starting from the current working directory to locate the project root
- [x] 3.2 Implement file signature matchers for standard ecosystems (Cargo, Go mod, package.json, uv/pip, Maven/Gradle, dotnet)
- [x] 3.3 Implement the interactive `dialoguer` selection menu when multiple contexts are detected, integrating configuration save options

## 4. Run/Build/Test Adapters

- [x] 4.1 Implement Rust (`cargo`) adapter to invoke `cargo build`, `cargo run`, or `cargo test` with pass-through arguments
- [x] 4.2 Implement Go (`go`) adapter to invoke `go build`, `go run .`, or `go test ./...` with pass-through arguments
- [x] 4.3 Implement JavaScript/Node.js adapter to check lockfiles (`package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb`) and execute mapped scripts
- [x] 4.4 Implement Python, Java, and .NET command-mapping execution adapters

## 5. Toolchain Verification & Interactive Installation

- [x] 5.1 Implement PATH verification using the `which` crate to ensure target binaries exist before execution
- [x] 5.2 Implement interactive prompt indicating the missing toolchain and asking for permission to install
- [x] 5.3 Implement bootstrapper execution to download and execute standard installer scripts (e.g., rustup, uv, fnm) under Linux/macOS environments

