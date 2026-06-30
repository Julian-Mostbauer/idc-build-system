## Context

We are introducing an automated CI/CD pipeline on GitHub to build cross-platform binaries and host them as downloadable release artifacts.

## Goals / Non-Goals

**Goals:**
- Trigger builds automatically on push to `main`.
- Skip builds if only documentation or specs changed.
- Compile for Linux, macOS, and Windows.
- Update release assets under a rolling `latest` release tag.

**Non-Goals:**
- Code signing macOS or Windows binaries in this initial workflow setup.

## Decisions

### 1. Workflow Triggers
- **Decision:** Trigger on `push` to `main` with `paths-ignore` set to ignore non-code files.
- **Rationale:** Ensures that pushes containing doc changes do not consume runner minutes unnecessarily.

### 2. Multi-Platform Matrix
- **Decision:** Use a matrix strategy compiling:
  * `ubuntu-latest` -> `x86_64-unknown-linux-gnu`
  * `macos-latest` -> `x86_64-apple-darwin`
  * `windows-latest` -> `x86_64-pc-windows-msvc`
- **Rationale:** Targets the major architectures.

### 3. Release Upload
- **Decision:** Use `softprops/action-gh-release@v1` targeting tag `latest` with `prerelease: true` and `overwrite: true`.
- **Rationale:** Automatically updates the rollouts under the same rolling tag.

## Risks / Trade-offs

- **[Risk] GitHub Token Permission Errors:** Release publishing requires `contents: write` scope.
  - *Mitigation:* Explicitly declare `permissions: contents: write` in the workflow YAML.
