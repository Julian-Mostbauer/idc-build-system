## Context

We are implementing `fmt` and `clean` subcommands for `idc`. This requires updating the main subcommand enum and mapping target commands across all supported adapters.

## Goals / Non-Goals

**Goals:**
- Implement `fmt` and `clean` subcommands.
- Map formatting and cleanups accurately across all 9 environments.
- Support standard fallbacks (Prettier, Ruff, Black, directory nuking).

**Non-Goals:**
- Creating custom formatting configs or code parsers.

## Decisions

### 1. Intercepting and mapping verbs
- **Decision:** Add `Fmt` and `Clean` commands to the `Commands` enum in `src/main.rs`. Set `verb` to `"fmt"` or `"clean"` and pass to `adapters::run_context_command`.
- **Rationale:** Keeps command resolution completely consistent with how `build`, `run`, and `test` work.

### 2. Context-Specific Command Mappings
- **Rust**: `fmt` -> `("cargo".into(), vec!["fmt".into()])`, `clean` -> `("cargo".into(), vec!["clean".into()])`
- **Go**: `fmt` -> `("go".into(), vec!["fmt".into(), "./...".into()])`, `clean` -> `("go".into(), vec!["clean".into(), "-i".into(), "-cache".into()])`
- **Node.js**:
  - `fmt`: check custom scripts. Fallback: `("npx".into(), vec!["prettier".into(), "--write".into(), ".".into()])`.
  - `clean`: check custom scripts. Fallback: Delete folders (`dist/`, `build/`, `out/`, `.next/`) using standard `std::fs::remove_dir_all`.
- **Python**:
  - `fmt`: run `ruff format .` if `ruff` exists. Otherwise, fall back to `black .`. If using `uv`/`poetry`, prepend `uv run` / `poetry run`.
  - `clean`: delete Python cache dirs/files.
- **Java**:
  - `fmt`: gradle format or maven spotless tasks.
  - `clean`: `mvn clean` or `gradlew clean`.
- **.NET**:
  - `fmt` -> `("dotnet".into(), vec!["format".into()])`
  - `clean` -> `("dotnet".into(), vec!["clean".into()])`
- **CMake**:
  - `fmt` -> If `clang-format` is in path, run on all source files.
  - `clean` -> `("cmake".into(), vec!["--build".into(), "build".into(), "--target".into(), "clean".into()])`
- **Makefile**:
  - `fmt` -> `("make".into(), vec!["fmt".into()])` (or `format`)
  - `clean` -> `("make".into(), vec!["clean".into()])`
- **Deno**:
  - `fmt` -> `("deno".into(), vec!["fmt".into()])`
  - `clean` -> `("deno".into(), vec!["task".into(), "clean".into()])` if task exists, else do nothing.

## Risks / Trade-offs

- **[Risk] Prettier fallback failure:** Node.js projects without `prettier` in dependencies or globally might fail when running `npx prettier`.
  - *Mitigation:* `npx prettier` dynamically downloads prettier if it is not present locally, but standard shell execution will succeed.
