## ADDED Requirements

### Requirement: CMake Toolchain Bootstrapping
When the user consents to auto-installing CMake, the system SHALL execute `brew install cmake` on macOS, and `sudo apt-get update && sudo apt-get install -y cmake` on Linux systems with `apt`.

#### Scenario: Installing CMake on Linux
- **WHEN** the user runs `idc build` in a CMake project, CMake is missing, and the user approves the installation on Linux (apt-based)
- **THEN** the system SHALL run `sudo apt-get update && sudo apt-get install -y cmake`

### Requirement: Make Toolchain Bootstrapping
When the user consents to auto-installing `make`, the system SHALL execute `xcode-select --install` or `brew install make` on macOS, and `sudo apt-get update && sudo apt-get install -y build-essential` on Linux systems with `apt`.

#### Scenario: Installing make on macOS
- **WHEN** the user runs `idc build` in a Makefile project, make is missing, and the user approves the installation on macOS
- **THEN** the system SHALL run `brew install make` or prompt for Xcode command line tools

### Requirement: Deno Toolchain Bootstrapping
When the user consents to auto-installing Deno, the system SHALL execute the official installer command `curl -fsSL https://deno.land/x/install/install.sh | sh` on Linux/macOS.

#### Scenario: Installing Deno on Linux
- **WHEN** the user runs `idc run` in a Deno project, Deno is missing, and the user approves the installation on Linux
- **THEN** the system SHALL run `curl -fsSL https://deno.land/x/install/install.sh | sh`
