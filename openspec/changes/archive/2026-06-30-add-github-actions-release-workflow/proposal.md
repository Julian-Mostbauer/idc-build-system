## Why

We need to automatically compile and release compiled binaries for Linux, macOS, and Windows on push to the `main` branch. The build must only trigger when actual codebase changes occur (excluding changes to documentation, specifications, or metadata).

## What Changes

- Add a GitHub Actions workflow `.github/workflows/release.yml` to compile releases on push.
- Configure path filtering to skip builds when changes only occur on `README.md`, `openspec/`, `docs/`, or metadata files.
- Enable automatic generation and updating of rolling release assets (e.g. `latest`) on GitHub.

## Capabilities

### New Capabilities

*None.*

### Modified Capabilities

- `cli-core`: Introduces automated GitHub Actions release workflow specifications.

## Impact

- Adds `.github/workflows/release.yml`.
