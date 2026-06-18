# S005 — Add Decision Deadlines to Open Questions

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | medium                                                                   |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | OpenCode, deepseek-v4-pro, medium                                        |
| Reviewer                 |                                                                          |

## Issue

The two open-question blocks ("Product matching" and "Supporting multiple
currencies") list decisions that affect downstream stages but have no deadlines.
Specifically:
- "Product matching" decisions affect Stage G.
- "Multi-currency" decisions affect Stages H, J, K.

Without a decision deadline, these can remain open indefinitely, causing rework
or blocking progress when the dependent stage is reached.

## Suggestion

Add a "Decide by" field to each open-question block. Example:

> **Product matching (Stage G)** — Decide by: before Stage G implementation starts.

Or tie the deadline to a calendar milestone if the project has one. This creates
a forcing function so that design questions are resolved before they block
implementation.

## Feedback

(None yet — pending review)
