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

