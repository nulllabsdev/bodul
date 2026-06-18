# S001 - Make Stage Dependencies Explicit

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | medium                                                                   |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Claude Code, claude-opus-4-8, medium                                     |
| Reviewer                 |                                                                          |

## Issue

The Stages section (lines 55–123) lists work as a flat, alphabetically lettered
sequence A–K. The prose makes clear these stages are not a simple linear chain —
there are real data dependencies between them:

- G (add/update products inventory) consumes the classification output of F.
- H (add/update retailer offers) matches against the inventory built in G.
- I (read side for product inventory) projects from G's events.
- J (read side for retailer offers) projects from H's events.
- K (customer-facing API + SPA) needs both read sides, I and J.

None of this is stated. A reader cannot tell which stages are strictly ordered,
which can proceed in parallel (e.g. I and J are independent once G and H exist),
or where the critical path runs. The Open Questions section reinforces that the
ordering matters — it pins questions to "Stage G" and "Stages H, J, K" — yet the
dependency structure those labels imply is never drawn.

## Suggestion

Make the dependency structure explicit. Either:

- add a short "depends on:" note to each stage that has upstream prerequisites
  (e.g. "**H. Add/update retailer offers** — depends on G"), or
- add a small dependency diagram / DAG near the top of the Stages section showing
  the edges (F→G, G→H, G→I, H→J, I→K, J→K) and highlighting independent stages
  that can run concurrently.

This clarifies the critical path and what can be parallelized during Phase 0
planning, and complements
[S005](S005_link-open-questions-to-stages_claude-opus-4-8-medium.md), which
links the Open Questions to these stages.

## Feedback
