# cli-core Specification

## Purpose
TBD - created by archiving change initialize-idc-cli. Update Purpose after archive.
## Requirements
### Requirement: Upward Directory Traversal
The system SHALL scan the current working directory and walk up parent directories to search for known build configuration files (e.g., `Cargo.toml`, `go.mod`, `package.json`, `pyproject.toml`, `requirements.txt`, `pom.xml`, `build.gradle`, `*.csproj`, `*.sln`) or a local `idc.yaml` configuration file to determine the project root.

#### Scenario: Find Cargo.toml in parent directory
- **WHEN** the user executes `idc build` in a subdirectory `/src` under `/my-project` which contains a `Cargo.toml` at its root
- **THEN** the system SHALL successfully detect the project root as `/my-project` and determine the context to be Rust

### Requirement: Single-Context Auto-Execution
When exactly one build context is detected in the project root and no `idc.yaml` override exists, the system SHALL automatically map the user's command (`build`, `run`, `test`) to the corresponding build tool executable.

#### Scenario: Running test in a Go project
- **WHEN** the user executes `idc test` in a project containing only a `go.mod` file at the root
- **THEN** the system SHALL invoke `go test ./...` in the project root directory

### Requirement: Polyglot Interactive Selection
When multiple build contexts are detected in the project root and no default is configured, the system SHALL display an interactive terminal menu listing all detected contexts, allowing the user to select one, run all, or configure a default.

#### Scenario: Selecting Node build in a polyglot repository
- **WHEN** the user executes `idc build` in a directory containing both a `package.json` and a `go.mod`, with no default configured
- **THEN** the system SHALL display an interactive prompt listing both options and wait for user selection

### Requirement: Configuration Persistence
If the user selects to save their choice during interactive polyglot selection, the system SHALL create or update an `idc.yaml` file in the project root.

#### Scenario: Saving default configuration
- **WHEN** the user selects Node as the default project from the interactive menu
- **THEN** the system SHALL write a local `idc.yaml` file indicating Node is the default build target

### Requirement: Argument Pass-Through
The system SHALL append all command-line arguments and flags passed after the command verb directly to the native build tool execution.

#### Scenario: Passing flags to Cargo
- **WHEN** the user runs `idc build --release --verbose` in a Rust project
- **THEN** the system SHALL execute `cargo build --release --verbose` in the target project root

### Requirement: CMake Context Resolution
The system SHALL detect CMake projects by looking for `CMakeLists.txt` files. For `build` commands, it SHALL configure the project via `cmake -B build` if the `build/` directory is missing, then compile using `cmake --build build`. For `test` commands, it SHALL run `ctest --test-dir build`. For `run` commands, it SHALL search the `build/` directory for executable targets, resolving ambiguities by checking folder name or prompting the user.

#### Scenario: Running CMake build without build directory
- **WHEN** the user executes `idc build` in a project containing a `CMakeLists.txt` but no `build/` folder
- **THEN** the system SHALL run `cmake -B build` followed by `cmake --build build`

### Requirement: Makefile Context Resolution
The system SHALL detect Makefile projects by looking for `Makefile`, `makefile`, or `GNUmakefile` files. It SHALL execute the verbs `build`, `run`, and `test` by calling `make`, `make run`, and `make test` respectively, without checking if targets exist beforehand.

#### Scenario: Running tests in a Makefile project
- **WHEN** the user executes `idc test` in a project containing a `Makefile`
- **THEN** the system SHALL run `make test` and let the terminal process output and exit status bubble up naturally

### Requirement: Deno Context Resolution
The system SHALL detect Deno projects by looking for `deno.json` or `deno.jsonc` files. It SHALL run `deno task build` if configured (otherwise treat as a no-op) for `build`, `deno test --allow-all` for `test`, and `deno task start` or a fallback script run (searching `main.ts`, `main.js`, `index.ts`, `index.js`, `mod.ts` in order) for `run`.

#### Scenario: Running default entrypoint in a Deno project
- **WHEN** the user runs `idc run` in a Deno project containing `main.ts` but no `start` task in `deno.json`
- **THEN** the system SHALL run `deno run --allow-all main.ts`

### Requirement: Custom Script Execution
If a command matching the requested subcommand verb exists in the `commands` section of `idc.yaml`, the system SHALL execute the configured shell command value directly instead of checking language adapters.

#### Scenario: Running custom deploy task
- **WHEN** the user runs `idc deploy` in a project where `idc.yaml` contains `commands: { deploy: "fly deploy" }`
- **THEN** the system SHALL execute the shell command `fly deploy` in the project root

### Requirement: Shell Completions Generation
The system SHALL support a subcommand `completion <shell>` (where shell is `bash`, `zsh`, or `fish`) that prints the shell completion code to stdout.

#### Scenario: Generating Bash completions
- **WHEN** the user executes `idc completion bash`
- **THEN** the system SHALL print the Bash shell autocompletions script to stdout

### Requirement: CMake Target Graph Verification
The system SHALL resolve executable target binaries in CMake projects by reading build artifacts or cache files (such as `build/CMakeCache.txt` or compiler configurations) if present in the build directory, falling back to directory scanning.

#### Scenario: Target resolution via CMake cache
- **WHEN** the user executes `idc run` in a CMake project with multiple compiled binaries in `build/`
- **THEN** the system SHALL check CMake build files to identify the executable target and run it

### Requirement: Python Command Fallback
If the system falls back to a standard Python script execution (when `uv` or `poetry` are not used), it SHALL invoke the command using the `python3` binary if `python` is not available in the system PATH.

#### Scenario: Running main.py with python3
- **WHEN** the user runs `idc run` in a Python project where `python` is missing but `python3` exists in PATH
- **THEN** the system SHALL execute `python3 main.py`

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

