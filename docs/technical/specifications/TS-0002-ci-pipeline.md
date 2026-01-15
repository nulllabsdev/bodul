# TS-0002: CI Pipeline (GitHub Actions)

| Field         | Value      |
|---------------|------------|
| Status        | Proposed   |
| Owner         | @miro      |
| Created       | 2026-01-15 |
| Last updated  | 2026-01-15 |
| Related issue | -          |

## Summary

Specification for continuous integration using GitHub Actions. Covers code style enforcement, testing, and release build verification on every push and pull request.

## Goals

1. **Automated quality gates** - Prevent merging code that fails checks
2. **Fast feedback** - Developers know within minutes if their code passes
3. **Consistent standards** - Same checks run locally (`make fmt clippy test build`) and in CI
4. **Low maintenance** - Leverage existing Makefile targets where possible

## Non-goals

- Deployment automation
- Security scanning
- Binary artifact publishing

## Pipeline overview

### Trigger events

| Event          | Branches | Behavior                  |
|----------------|----------|---------------------------|
| `push`         | `main`   | Run on every push to main |
| `pull_request` | `main`   | Run on PRs targeting main |

### Jobs

| Job      | Purpose         | Checks                                               |
|----------|-----------------|------------------------------------------------------|
| `fmt`    | Code formatting | `cargo fmt --all --check`                            |
| `clippy` | Linting         | `cargo clippy --workspace --all-targets -D warnings` |
| `test`   | Unit tests      | `cargo test --workspace`                             |
| `build`  | Compilation     | `cargo build --workspace --release`                  |

All jobs run in parallel for fastest feedback.

## Technical specification

### Rust toolchain

| Setting    | Value               | Source                           |
|------------|---------------------|----------------------------------|
| Version    | `1.85`              | `shared/Cargo.toml` rust-version |
| Edition    | `2024`              | `shared/Cargo.toml` edition      |
| Components | `rustfmt`, `clippy` | Required for style checks        |

### Runner environment

| Setting     | Value                          | Rationale                   |
|-------------|--------------------------------|-----------------------------|
| OS          | `ubuntu-latest`                | Standard, fast provisioning |
| Concurrency | Cancel in-progress on new push | Saves CI minutes            |

### Caching strategy

| Cache            | Key                                   | Path                |
|------------------|---------------------------------------|---------------------|
| Cargo registry   | `cargo-registry-{hash of Cargo.lock}` | `~/.cargo/registry` |
| Cargo index      | `cargo-index-{hash of Cargo.lock}`    | `~/.cargo/git`      |
| Target directory | `cargo-target-{hash of Cargo.lock}`   | `target/`           |

Cache invalidation occurs when `Cargo.lock` changes.

## Workflow file

**Location**: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

# Cancel in-progress runs on new pushes to the same branch
concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  fmt:
    name: Format
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "1.85"
          components: rustfmt

      - name: Check formatting
        run: cargo fmt --all --check
        working-directory: shared

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "1.85"
          components: clippy

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            shared/target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('shared/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Run clippy
        run: cargo clippy --workspace --all-targets -- -D warnings
        working-directory: shared

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "1.85"

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            shared/target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('shared/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Run tests
        run: cargo test --workspace
        working-directory: shared

  build:
    name: Build
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@master
        with:
          toolchain: "1.85"

      - name: Cache cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            shared/target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('shared/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      - name: Build release
        run: cargo build --workspace --release
        working-directory: shared
```

## Job details

### `fmt` job

**Purpose**: Verify all code is formatted according to `rustfmt` defaults.

**Command**: `cargo fmt --all --check`

**Behavior**:
- `--all` checks all workspace members
- `--check` exits with error if formatting differs (does not modify files)
- No caching needed (fast execution)

**Failure resolution**: Run `make fmt` locally before pushing.

### `clippy` job

**Purpose**: Catch common mistakes, enforce idioms, deny warnings.

**Command**: `cargo clippy --workspace --all-targets -- -D warnings`

**Flags**:
- `--workspace` checks all crates in the workspace
- `--all-targets` includes bins, tests, examples, benches
- `-D warnings` treats warnings as errors

**Caching**: Enabled to speed up incremental analysis.

**Failure resolution**: Run `make clippy` or `make cfix` locally.

### `test` job

**Purpose**: Run all unit tests to catch regressions.

**Command**: `cargo test --workspace`

**Flags**:
- `--workspace` runs tests for all crates

**Caching**: Enabled to speed up compilation.

**Failure resolution**: Run `make test` locally to reproduce and fix.

### `build` job

**Purpose**: Verify release build compiles successfully.

**Command**: `cargo build --workspace --release`

**Flags**:
- `--workspace` builds all crates
- `--release` uses optimized build profile

**Caching**: Enabled to speed up compilation.

**Failure resolution**: Run `make buildr` locally to reproduce.

## Local/CI parity

| Check  | Local command | CI equivalent                                           |
|--------|---------------|---------------------------------------------------------|
| Format | `make fmt`    | `cargo fmt --all --check`                               |
| Lint   | `make clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| Test   | `make test`   | `cargo test --workspace`                                |
| Build  | `make buildr` | `cargo build --workspace --release`                     |

Developers can verify CI will pass by running:

```bash
make fmt
make clippy
make test
make buildr
```

If all succeed locally, CI will pass.

## Branch protection (recommended)

Configure GitHub branch protection for `main`:

| Setting                           | Value                            |
|-----------------------------------|----------------------------------|
| Require status checks             | Yes                              |
| Required checks                   | `fmt`, `clippy`, `test`, `build` |
| Require branches to be up to date | Yes                              |
| Include administrators            | Yes                              |

## Future enhancements

### Matrix builds (if needed)

```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    rust: ["1.85", "stable"]
```

### Code coverage

Add coverage reporting with `cargo-tarpaulin` or `cargo-llvm-cov`.

### Security scanning

Add `cargo-audit` for dependency vulnerability scanning.

## Metrics

### Target execution times

| Job               | Expected duration |
|-------------------|-------------------|
| `fmt`             | < 30 seconds      |
| `clippy` (cold)   | 2-3 minutes       |
| `clippy` (cached) | < 1 minute        |
| `test` (cold)     | 2-3 minutes       |
| `test` (cached)   | < 1 minute        |
| `build` (cold)    | 3-4 minutes       |
| `build` (cached)  | < 2 minutes       |

### CI minutes budget

With current scope (4 jobs running in parallel, ~4 minutes wall time):
- ~8 minutes per PR (2 runs: open + final push)
- ~4 minutes per merge to main

## Implementation checklist

- [ ] Create `.github/workflows/ci.yml` with workflow above
- [ ] Verify workflow runs on test PR
- [ ] Enable branch protection on `main`
- [ ] Update this specification status to "Active"

## References

- [TS-0001: Makefile Structure](./TS-0001-makefile-structure.md) - Local development targets
- [GitHub Actions documentation](https://docs.github.com/en/actions)
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) - Rust toolchain installer
