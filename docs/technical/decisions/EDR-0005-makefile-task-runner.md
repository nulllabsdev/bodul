# EDR-0005: Makefile as Task Runner

* **Status:** Proposed
* **Driver:** @miro
* **Approver:** @miro
* **Date:** 2026-01-13
* **Area:** Developer Experience

## Context and Problem Statement

We need a consistent way to define and run common development tasks (build, test, lint, deploy, etc.) across the project. Without a standard task runner, developers may use different commands or scripts, leading to inconsistency and onboarding friction.

## Decision Drivers

* Consistent tooling in each library and application, with support running them all from root folder too
* Low barrier to entry - no additional tooling required
* Self-documenting interface for common operations

## Decision Outcome

Chosen option: **Makefile**

**Note:** Windows is not supported. Development requires Linux or macOS.

### Makefile Structure

Each component has its own Makefile for independent builds:

```
/
├── Makefile              # Root: runs tasks across all libs, shared, and apps
├── lib/
│   ├── <lib>/Makefile    # Each generic library has its own Makefile
├── shared/
│   ├── <lib>/Makefile    # Each shared library has its own Makefile
└── <app>/
    └── Makefile          # Each application has its own Makefile
```

* **Library Makefiles** (`lib/*/Makefile`, `shared/*/Makefile`): Build, test, and lint that specific library independently
* **Application Makefiles** (`<app>/Makefile`): Build, test, and lint that specific application independently
* **Root Makefile** (`/Makefile`): Orchestrates running tasks across all libraries and applications

### Standard Targets

Each Makefile should include (where applicable):

```makefile
help           # List available targets
build          # Build the project
test           # Run tests
lint           # Run linters
fmt            # Format code
clean          # Clean build artifacts
dev            # Start development server (apps only)
```

### Rationale

* `make` is available on virtually all Unix-like systems
* Ability to run consecutive tasks via `make <task1> <task2> <task3>` or parallel tasks via `make -jN <task1> <task2> <task3>`
* Clearer naming of tasks
* Independent builds allow faster iteration on specific components

## Consequences

### Positive
* Single entry points for all development tasks
* Easy onboarding - developers only need to know `make help`
* Consistent interface across all libraries and applications
* CI/CD pipelines use the same commands as local development
* Can build/test individual components or everything at once

### Negative
* Makefile syntax can be unintuitive for some developers
* No Windows support - requires Linux or macOS

## Links

* [GNU Make Manual](https://www.gnu.org/software/make/manual/)
