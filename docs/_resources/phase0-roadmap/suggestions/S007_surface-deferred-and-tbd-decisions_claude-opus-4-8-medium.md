# S007 - Surface Deferred and TBD Decisions

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | low                                                                      |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Claude Code, claude-opus-4-8, medium                                     |
| Reviewer                 |                                                                          |

## Issue

Deferred work and undecided choices are scattered inline across the document as
"(TBD)" and "deferred" notes, with no single place that collects them. Examples:

- Stage A: scheduler deferred; catalog/scraping processing "(TBD)" (lines 60, 63).
- Stage C: processing collections and catalog "(TBD)" (line 77).
- Stage D: multiple IPs / browser agents "(TBD)" (line 83).
- Stage F: train cheaper/local model "(TBD)" (line 94).
- Stage K: "TBD on REST vs GraphQL" (line 121).
- Architecture: docker build "how+where (TBD)" (line 129); observability deferred
  (line 130).

Two distinct problems: (1) genuinely *open decisions* (e.g. REST vs GraphQL,
docker build location) are buried inside individual stages where they are easy
to miss, even though the document already has an "Open Questions" section for
exactly this purpose; and (2) *deferred capabilities* are listed but not tracked
anywhere collectively, so there is no at-a-glance view of what Phase 0 is
consciously leaving out.

## Suggestion

Consolidate for visibility, without removing the inline context:

- promote buried open *decisions* (REST vs GraphQL, docker build how/where, and
  similar) into the existing "Open Questions" section, or at least link them
  there, so all undecided choices live in one place; and
- add a short "Deferred to later phases" list (or table) gathering the "(TBD)"/
  deferred *capabilities*, each tagged with the stage it belongs to.

This pairs with
[S002](S002_define-phase-0-exit-criteria_claude-opus-4-8-medium.md): a clear
scope boundary plus a single deferred-items list together make Phase 0's in/out
line legible.

## Feedback
