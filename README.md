# idc (Instant Development Companion)

idc is a zero-friction, unified developer command companion written in Rust. It eliminates the need to manually install toolchains, manage runtimes, or remember different build tool syntax when context-switching between projects.

Simply run idc commands in any project directory. The tool resolves your project root, detects your build environment, and executes the correct native command.

## Key Features

* Context-Aware: Walks up directories to identify the project root and build system (similar to git).
* Zero Config Detection: Supports Rust (cargo), Go (go), JavaScript/TypeScript (npm, yarn, pnpm, bun, deno), C/C++ (cmake, make), Python (uv, poetry, pip), Java (mvn, gradle), and .NET (dotnet) out of the box.
* Argument Pass-through: Appends all trailing flags and options directly to the underlying tool.
* Toolchain Bootstrapping: Prompts to install missing compilers or runtime managers automatically.
* Custom Tasks: Run project-specific aliases configured in your local idc.yaml file.
* Shell Completions: Generate completions dynamically for bash, zsh, and fish.

## Usage

Instead of switching syntax across projects, just run:

```bash
# Compile or package the codebase
idc build

# Run the main target or dev server
idc run

# Run unit and integration tests
idc test

# Format source code
idc fmt

# Clean build outputs and caches
idc clean
```

Extra arguments are forwarded automatically:
```bash
# In a Rust project, this translates to: cargo test -- --nocapture
idc test -- --nocapture
```

## Custom Scripting (idc.yaml)

Define project-specific tasks in an idc.yaml at your project root:

```yaml
default_context: rust
commands:
  check-version: "echo 'v0.1.0'"
  deploy: "fly deploy --remote-only"
  lint: "cargo clippy"
```

Running `idc deploy` will execute your configured shell script in the project root.

## Shell Autocompletions

To configure autocompletions for your shell:

### Fish
```fish
idc completion fish > ~/.config/fish/completions/idc.fish
```

### Zsh
```zsh
idc completion zsh > ~/.zsh/_idc
```

### Bash
```bash
source <(idc completion bash)
```
