# toolchain-auto-install Specification

## Purpose
TBD - created by archiving change initialize-idc-cli. Update Purpose after archive.
## Requirements
### Requirement: Missing Toolchain Detection
The system SHALL check the environment `PATH` to verify if the required compiler, interpreter, or package manager executable for the detected build context is installed.

#### Scenario: Rustup or cargo executable is missing
- **WHEN** the user runs `idc build` in a Rust project but `cargo` is not found in the environment `PATH`
- **THEN** the system SHALL recognize that the Rust toolchain is missing and trigger the installation prompt

### Requirement: Interactive Installation Prompt
When a toolchain is missing, the system SHALL display an interactive prompt to the user explaining which toolchain is missing and requesting permission to download and install it.

#### Scenario: User declines installation
- **WHEN** the system prompts the user to install the missing Go toolchain, and the user selects "No"
- **THEN** the system SHALL print a message detailing how to manually install Go, and exit with an error code

### Requirement: Installer Bootstrapping
When the user consents to automated installation, the system SHALL execute the appropriate platform bootstrap script or package manager command to install the toolchain.

#### Scenario: Bootstrap installation of Rustup
- **WHEN** the user selects "Yes" to install the Rust toolchain on a Linux system
- **THEN** the system SHALL run the standard rustup installer shell script: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

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

