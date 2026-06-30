## Context

This change extends `idc` to support CMake, Makefile, and Deno contexts. The initial implementation supported Cargo, Go mod, package.json, Python, Java, and .NET. We must incorporate CMake, Makefile, and Deno without breaking any of the existing build systems, and maintain clean detection, execution, and installation logic.

## Goals / Non-Goals

**Goals:**
- Detect C++ projects using `CMakeLists.txt` or `Makefile`/`makefile`/`GNUmakefile`.
- Detect TypeScript/JavaScript Deno projects using `deno.json`/`deno.jsonc`.
- Support `build`, `run`, and `test` commands for CMake, Makefile, and Deno.
- Provide auto-installation prompts and runners for `cmake`, `make`, and `deno`.
- Integrate CMake auto-configuration when building without a configured build directory.

**Non-Goals:**
- Implementing custom C++ compiler flags or custom makefile generators.
- Resolving complex C++ library dependencies automatically.

## Decisions

### 1. CMake Build and Run Flow
- **Decision:** For `build`, if `build/` folder doesn't exist, execute `cmake -B build` followed by `cmake --build build`. For `run`, scan `build/` for executable files. If there is exactly one, execute it. If multiple are found, match against the project root directory name. If it's still ambiguous, prompt the user.
- **Rationale:** CMake projects compile targets into a build directory (often `build/` or `out/`). Finding and configuring this dynamically makes running C++ applications zero-friction.

### 2. Makefile Target Mapping
- **Decision:** Execute make command directly as `make <target>` (e.g. `make test` or `make run`), letting the native shell process exit status propagate without parsing the Makefile.
- **Rationale:** Makefile targets are arbitrary. Trying to parse Makefile syntax to confirm targets is complex and error-prone due to includes, wildcard definitions, and variables. Simple shell propagation is robust and standard.

### 3. Deno Entrypoint Priority
- **Decision:** For running without a `start` task in `deno.json`, search for entrypoint files in the priority order: `main.ts` ➔ `main.js` ➔ `index.ts` ➔ `index.js` ➔ `mod.ts`.
- **Rationale:** Matches Deno standard project structures and typical user naming conventions.

### 4. Interactive Installers for C++ Toolchains
- **Decision:** Trigger native package managers (`brew` on macOS, `apt` on Linux) to install `cmake` and `make`/`build-essential`. For Deno, use Deno's official curl shell installer script.
- **Rationale:** Ensures installation is tailored to the developer's platform rather than compiling compiler tools from source.

## Risks / Trade-offs

- **[Risk] Executable Detection in CMake Build Folder:** Scanning the `build/` directory might pick up internal CMake executables or test runner executables.
  - *Mitigation:* Filter out known internal CMake directories (like `CMakeFiles/` or `_deps/`) and only scan the output level of the build folder. If multiple executables are found, prioritizing the one matching the project directory name resolves the most common setup.
