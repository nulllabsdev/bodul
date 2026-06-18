# S001 — Add TBD Tracking Mechanism

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | high                                                                     |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | OpenCode, deepseek-v4-pro, high                                          |
| Reviewer                 |                                                                          |

## Issue

The roadmap contains multiple items marked "TBD" or "deferred" with no tracking
mechanism:

- Catalog/collection processing (Stages A, C)
- Bot detection / anti-scraping infrastructure (Stage D)
- Model training for cheaper classification (Stage F)
- Docker build infrastructure (Architecture section)
- Error handling and observability (Architecture section)
- REST vs GraphQL decision (Stage K)

Each of these represents a scope decision that could block Phase 0 or silently
slip out of scope. Currently the reader must scan the entire document to find
them all, and no single view shows what has been decided, deferred, or
forgotten.

## Suggestion

Add a "Deferred Decisions" table near the end of the document (after Stages,
before Architecture) that collects every TBD/deferred item in one place, with
columns: `Item`, `Affects`, `Status` (TBD / deferred-to-phase-1 / decided),
`Decision`, and `Decided on`. When an item is decided, update the table and
remove the inline "TBD" from the stage description or replace it with a
cross-reference to the table entry.

## Feedback

(None yet — pending review)
