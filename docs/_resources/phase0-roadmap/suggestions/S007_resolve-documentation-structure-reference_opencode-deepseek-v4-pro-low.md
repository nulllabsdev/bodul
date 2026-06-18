# S007 — Resolve documentation-structure Reference

| Field                    | Value                          |
| ------------------------ | ------------------------------ |
| Priority                 | low                            |
| File                     | `docs/phase0-roadmap.md`       |
| Decision                 | refused                        |
| Implementation reference |                                |
| Created at               | 2026-06-19                     |
| Author                   | OpenCode, deepseek-v4-pro, low |
| Reviewer                 |                                |

## Issue

The suggestion-driven-documentation guidelines reference
`documentation-structure.md` at line 139:

> See `documentation-structure.md` for the resource index structure.

This file does not exist in the repository. Additionally, the roadmap document's
existing "Open Questions" section resembles the "Sources Used / Open Questions"
schema that the guidelines attribute to `documentation-structure.md`, but the
roadmap has no "Sources Used" counterpart.

## Suggestion

Either:
1. Create `docs/documentation-structure.md` with the schema described (Sources
   Used / Open Questions), or
2. Remove the orphan reference from `suggestion-driven-documentation.md` and
   define the schema for non-suggestion-driven INDEX files inline.

## Feedback

(None yet — pending review)
