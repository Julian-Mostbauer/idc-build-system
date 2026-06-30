# idc (Instant Development Companion)

`idc` is a zero-friction, unified developer command companion written in Rust. It eliminates the need to manually install toolchains, manage runtimes, or remember different build tool syntax when context-switching between projects. 

Simply run `idc build`, `idc run`, or `idc test` in any directory, and `idc` will trace the path upwards, locate your project root, detect your build environment, and execute the correct commands.

## 🚀 Key Features

* **Context-Aware Directory Climbing**: Walks up parent directories to identify the project root and build system (similar to `git`).
* **Zero Configuration Detection**: Supports major ecosystems out of the box:
  * 🦀 **Rust**: `cargo`
  * 🐹 **Go**: `go`
  * 🟢 **JavaScript / TypeScript**: Node (`npm`, `yarn`, `pnpm`, `bun`) and **Deno** (`deno.json`)
  * 🛠️ **C/C++**: CMake (`CMakeLists.txt`) and Makefiles (`Makefile`)
  * 🐍 **Python**: `uv`, `poetry`, `pip`
  * ☕ **Java**: Maven (`mvn`), Gradle (`gradle` / `./gradlew`)
  * 🎯 **.NET**: `dotnet`
* **Direct Argument Pass-through**: Appends all trailing flags and options directly to the underlying tool (e.g., `idc build --release --verbose` -> `cargo build --release --verbose`).
* **On-Demand Toolchain Bootstrapping**: If a required compiler or manager is missing, `idc` interactively prompts you to automatically install it using official installation scripts (e.g., `rustup`, `fnm`, `uv`).
* **Polyglot & Monorepo Support**: Shows a clean interactive menu when multiple language files are present in the same root, and allows you to persist your preference in a local `idc.yaml` file.
* **Custom Tasks**: Run custom tasks and local alias commands mapped inside your project configuration.
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

---

## ⚙️ Custom Scripting (`idc.yaml`)

Define local tasks and shortcuts directly in an `idc.yaml` at your project root:

```yaml
default_context: rust
commands:
  check-version: "echo 'v0.1.0'"
  deploy: "fly deploy --remote-only"
  lint: "cargo clippy"
```

Running `idc deploy` or `idc check-version` will automatically execute your configured shell command.

---

## 🐚 Shell Autocompletions

To enable command and argument completions in your active shell:

### Fish
Add the completions to your fish directory:
```fish
idc completion fish > ~/.config/fish/completions/idc.fish
```

### Zsh
Write the completion file to a directory in your `$fpath`:
```zsh
idc completion zsh > ~/.zsh/_idc
```

### Bash
Source the completions in your `.bashrc`:
```bash
source <(idc completion bash)
```
