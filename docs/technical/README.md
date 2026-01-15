# Technical Documentation

## Overview

Technical specifications, architectural decisions, and engineering requirements for the Bodul platform.

## Quick Links

- [Engineering Decisions Registry](engineering-decisions.md) - All EDRs with status tracking

## Engineering Decisions (EDR)

| ID                                                                | Title                     | Status   | One-line outcome                               |
|-------------------------------------------------------------------|---------------------------|----------|------------------------------------------------|
| [EDR-0001](decisions/EDR-0001-using-linear-for-issue-tracking.md) | Linear for issue tracking | Accepted | Public Linear.app for issue tracking           |
| [EDR-0002](decisions/EDR-0002-rust-programming-language.md)       | Programming languages     | Accepted | Rust primary, Go secondary                     |
| [EDR-0003](decisions/EDR-0003-postgresql-data-storage.md)         | PostgreSQL database       | Accepted | PostgreSQL for ACID, PostGIS, full-text search |
| [EDR-0004](decisions/EDR-0004-monorepo-architecture.md)           | Monorepo architecture     | Accepted | Monorepo with lib/, shared/, layered apps      |
| [EDR-0005](decisions/EDR-0005-makefile-task-runner.md)            | Makefile task runner      | Accepted | All tasks defined via Makefile                 |
| [EDR-0006](decisions/EDR-0006-internal-service-security.md)       | Internal service security | Accepted | Auth client with hardcoded credentials         |

## Technical Specifications (TS)

| ID                                                      | Title              | Status | Description                                 |
|---------------------------------------------------------|--------------------|--------|---------------------------------------------|
| [TS-0001](specifications/TS-0001-makefile-structure.md) | Makefile structure | Active | Target definitions and workspace iteration  |
| [TS-0002](specifications/TS-0002-ci-pipeline.md)        | CI pipeline        | Active | GitHub Actions for fmt, clippy, test, build |

## Engineering Requirements (ER)

| ID                                                       | Title                 | Status   | Description                            |
|----------------------------------------------------------|-----------------------|----------|----------------------------------------|
| [ER-0001](requirements/ER-0001-retailer-code-library.md) | Retailer code library | Accepted | elemental crate with RetailerCode enum |
