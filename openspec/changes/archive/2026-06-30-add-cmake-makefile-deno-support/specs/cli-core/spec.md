## ADDED Requirements

### Requirement: CMake Context Resolution
The system SHALL detect CMake projects by looking for `CMakeLists.txt` files. For `build` commands, it SHALL configure the project via `cmake -B build` if the `build/` directory is missing, then compile using `cmake --build build`. For `test` commands, it SHALL run `ctest --test-dir build`. For `run` commands, it SHALL search the `build/` directory for executable targets, resolving ambiguities by checking folder name or prompting the user.

#### Scenario: Running CMake build without build directory
- **WHEN** the user executes `idc build` in a project containing a `CMakeLists.txt` but no `build/` folder
- **THEN** the system SHALL run `cmake -B build` followed by `cmake --build build`

### Requirement: Makefile Context Resolution
The system SHALL detect Makefile projects by looking for `Makefile`, `makefile`, or `GNUmakefile` files. It SHALL execute the verbs `build`, `run`, and `test` by calling `make`, `make run`, and `make test` respectively, without checking if targets exist beforehand.

#### Scenario: Running tests in a Makefile project
- **WHEN** the user executes `idc test` in a project containing a `Makefile`
- **THEN** the system SHALL run `make test` and let the terminal process output and exit status bubble up naturally

### Requirement: Deno Context Resolution
The system SHALL detect Deno projects by looking for `deno.json` or `deno.jsonc` files. It SHALL run `deno task build` if configured (otherwise treat as a no-op) for `build`, `deno test --allow-all` for `test`, and `deno task start` or a fallback script run (searching `main.ts`, `main.js`, `index.ts`, `index.js`, `mod.ts` in order) for `run`.

#### Scenario: Running default entrypoint in a Deno project
- **WHEN** the user runs `idc run` in a Deno project containing `main.ts` but no `start` task in `deno.json`
- **THEN** the system SHALL run `deno run --allow-all main.ts`
