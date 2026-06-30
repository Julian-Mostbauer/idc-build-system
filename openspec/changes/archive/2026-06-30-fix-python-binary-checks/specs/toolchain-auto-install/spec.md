## ADDED Requirements

### Requirement: Python Toolchain Alternative Checking
The system SHALL verify the presence of Python toolchains by checking for `python`, `python3`, or `uv` executables in the system PATH.

#### Scenario: Python3 is available but python is missing
- **WHEN** the user runs `idc build` in a Python project, `python3` is installed in PATH but `python` is not
- **THEN** the system SHALL consider the Python toolchain to be installed and proceed without prompting
