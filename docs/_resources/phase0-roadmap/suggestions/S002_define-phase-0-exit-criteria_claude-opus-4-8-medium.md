# S002 - Define Phase 0 Exit Criteria

| Field                    | Value                                |
| ------------------------ | ------------------------------------ |
| Priority                 | medium                               |
| File                     | `docs/phase0-roadmap.md`             |
| Decision                 | refused                              |
| Implementation reference |                                      |
| Created at               | 2026-06-19                           |
| Author                   | Claude Code, claude-opus-4-8, medium |
| Reviewer                 |                                      |

## Issue

The document is titled "Phase 0 Roadmap" and the Overview (lines 3–6) states the
ambition, but the document never defines what *completing* Phase 0 means. There
is no definition of done, no statement of which stages are mandatory for Phase 0
versus which are explicitly out, and no success criteria a reader could use to
judge whether Phase 0 is finished.

This matters because the document is heavily qualified: many capabilities are
marked "(TBD)" or "deferred to a later phase" (e.g. scheduler in A, catalog/
collection processing in C, bot-detection handling in D, model training in F,
observability in line 130). Without an explicit boundary, it is unclear whether,
for example, Stage K (customer-facing API + SPA) is in Phase 0 scope or whether
a thinner slice is the actual Phase 0 target.

## Suggestion

Add a short "Phase 0 Scope" or "Definition of Done" subsection (near the
Overview or after Stages) that states:

- the minimum set of stages that constitute a complete Phase 0,
- the concrete outcome that marks Phase 0 done (e.g. "a visitor can view matched
  Minisforum products with at least one retailer offer through the web SPA"), and
- a clear in/out boundary so the scattered "(TBD)"/"deferred" markers resolve
  against a single stated scope.

This pairs with
[S007](S007_surface-deferred-and-tbd-decisions_claude-opus-4-8-medium.md), which
proposes collecting the deferred items themselves into one tracked place.

## Feedback
