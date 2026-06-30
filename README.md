# idc (Instant Development Companion)

`idc` is a zero-friction, unified developer command companion written in Rust. It eliminates the need to manually install toolchains, manage runtimes, or remember different build tool syntax when context-switching between projects. 

Simply run `idc build`, `idc run`, or `idc test` in any directory, and `idc` will trace the path upwards, locate your project root, detect your build environment, and execute the correct commands.

## 🚀 Key Features

* **Context-Aware Directory Climbing**: Walks up parent directories to identify the project root and build system (similar to `git`).
* **Zero Configuration Detection**: Supports major ecosystems out of the box:
  * 🦀 **Rust**: `cargo`
  * 🐹 **Go**: `go`
  * 🟢 **JavaScript / TypeScript**: `npm`, `yarn`, `pnpm`, `bun` (automatically detected via lockfiles)
  * 🐍 **Python**: `uv`, `poetry`, `pip`
  * ☕ **Java**: Maven (`mvn`), Gradle (`gradle` / `./gradlew`)
  * 🎯 **.NET**: `dotnet`
* **Direct Argument Pass-through**: Appends all trailing flags and options directly to the underlying tool (e.g., `idc build --release --verbose` -> `cargo build --release --verbose`).
* **On-Demand Toolchain Bootstrapping**: If a required compiler or manager is missing, `idc` interactively prompts you to automatically install it using official installation scripts (e.g., `rustup`, `fnm`, `uv`).
* **Polyglot & Monorepo Support**: Shows a clean interactive menu when multiple language files are present in the same root, and allows you to persist your preference in a local `idc.yaml` file.
* **Lightweight & Fast**: Built in Rust as a statically linked binary with zero external runtime dependencies and sub-millisecond startup times.

## 🛠️ Usage

Instead of switching syntax across projects, just run:

```bash
# Compiles or packages the codebase
idc build

# Executes the main target or development server
idc run

# Runs unit and integration test suites
idc test
```

Any extra parameters are automatically passed through:
```bash
# In a Rust project, this translates to: cargo test -- --nocapture
idc test -- --nocapture
```
