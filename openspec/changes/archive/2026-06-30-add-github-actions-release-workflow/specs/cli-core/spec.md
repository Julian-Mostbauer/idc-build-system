## ADDED Requirements

### Requirement: Automated Github Actions Release Generation
On push events to the `main` branch, the system SHALL automatically build release binaries for Linux, macOS, and Windows and upload them to a rolling pre-release tag `latest` on GitHub.

#### Scenario: Running the release workflow
- **WHEN** a push occurs on branch `main` containing codebase modifications
- **THEN** the system SHALL invoke the release job, compiling release targets and attaching them to the GitHub release page

### Requirement: Documentation and Specification Paths Exclusions
The release workflow SHALL NOT trigger when changes are restricted purely to `README.md`, `openspec/`, `docs/`, `.gitignore`, or `LICENSE` files.

#### Scenario: Code docs change push
- **WHEN** a push to `main` only contains modifications inside the `openspec/` planning folders or `README.md`
- **THEN** the release workflow SHALL NOT run
