# S003 — Move Architecture Before Stages

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

The Architecture section currently appears after all stages (line 124). However,
every stage is constrained by the architecture decisions listed there:
- Rust + PostgreSQL (affects all stages)
- Event sourcing (affects G, H, I, J)
- Dockerless local dev / dockerized deployment (affects all stages)

Because architecture is at the bottom, a reader moving linearly through the
document encounters stage descriptions without the design constraints that
govern them.

## Suggestion

Move the Architecture section to immediately follow the Overview (before
Stages), so it reads:

1. Overview
2. Architecture
3. Open Questions
4. Stages

Alternatively, keep Architecture at the bottom but add a prominent note at the
top of the Stages section: "All stages assume the Architecture described below."

## Feedback

(None yet — pending review)
