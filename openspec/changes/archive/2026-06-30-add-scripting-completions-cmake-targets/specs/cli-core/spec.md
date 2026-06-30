## ADDED Requirements

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
