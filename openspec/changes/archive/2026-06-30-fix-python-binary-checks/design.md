## Context

We are modifying the toolchain verification checking and command execution resolution for Python files.

## Goals / Non-Goals

**Goals:**
- Recognize `python3` or `uv` in PATH as valid Python toolchain components.
- Execute commands using `python3` if `python` is not available.

**Non-Goals:**
- Managing virtual environments automatically in this specific binary verification fix.

## Decisions

### 1. Alternative Verification in `check_toolchain`
- **Decision:** Check if context is Python. If so, return `true` if `python`, `python3`, or `uv` is found in the path.
- **Rationale:** Prevents false alarms on modern Linux distros.

### 2. Execution Fallback in `adapters.rs`
- **Decision:** Inspect path to select between `"python"` and `"python3"`.
- **Rationale:** Ensures command execution succeeds on systems without the `python` alias.
