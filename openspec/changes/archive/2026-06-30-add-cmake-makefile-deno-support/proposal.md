## Why

Developers working on C/C++ projects (utilizing CMake and Makefiles) and modern JavaScript/TypeScript runtimes (utilizing Deno) currently lack unified CLI runner mappings and auto-installation wrappers within `idc`. Adding support for these build systems enables `idc` to serve a much broader set of developer environments.

## What Changes

- Extend the build context detector to recognize `CMakeLists.txt` (CMake), `Makefile`/`makefile`/`GNUmakefile` (Makefile), and `deno.json`/`deno.jsonc` (Deno) projects.
- Map the standard verbs (`build`, `run`, `test`) to these new contexts:
  - **CMake**: Auto-configure `cmake -B build` if missing, then build with `cmake --build build`; run tests using `ctest`; scan `build/` to run the compiled binary.
  - **Makefile**: Invoke standard targets (`make`, `make run`, `make test`).
  - **Deno**: Run `deno test` for tests, `deno task start` or resolved default files (`main.ts`, etc.) for run.
- Add interactive bootstrapping wrappers for `cmake`, `make`, and `deno` when missing from PATH.

## Capabilities

### New Capabilities

*None (extending existing systems).*

### Modified Capabilities

- `cli-core`: Scans for CMake, Makefile, and Deno signatures and maps execution adapters.
- `toolchain-auto-install`: Implements auto-installation bootstrapping for `cmake`, `make`, and `deno`.

## Impact

- Modifies `src/detector.rs` to add variants to `BuildContext` enum and detection rules.
- Modifies `src/adapters.rs` to implement command builders for CMake, Makefile, and Deno.
- Modifies `src/installer.rs` to implement bootstrapping script commands for `cmake`, `make`, and `deno`.
