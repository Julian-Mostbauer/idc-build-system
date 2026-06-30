## ADDED Requirements

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
