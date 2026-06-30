## ADDED Requirements

### Requirement: Python Command Fallback
If the system falls back to a standard Python script execution (when `uv` or `poetry` are not used), it SHALL invoke the command using the `python3` binary if `python` is not available in the system PATH.

#### Scenario: Running main.py with python3
- **WHEN** the user runs `idc run` in a Python project where `python` is missing but `python3` exists in PATH
- **THEN** the system SHALL execute `python3 main.py`
