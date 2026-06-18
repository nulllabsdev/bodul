# S008 - Clarify Phase 0 vs MVP Terminology

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

The Architecture section uses "Phase 0" and "MVP" in close proximity without
stating their relationship (lines 124–129):

- "Phase 0 is targeting minimum viable product that includes simple customer
  facing web" (line 126), and
- "All of the backend tasks will be built in MVP git repository" (line 127).

It is left implicit whether "Phase 0" and "MVP" are synonyms, whether the "MVP
git repository" is simply the repo for Phase 0 work, or whether MVP is a
distinct milestone. The document title and most references use "Phase 0", but
the repo and the architecture goal are framed as "MVP", so a reader has to guess
the two terms mean the same thing.

## Suggestion

State the relationship once, then use the terms consistently. Either declare
that "Phase 0 = the MVP" explicitly (and keep one primary term thereafter), or,
if "MVP git repository" is just a proper noun for the repo, name it as such so
it does not read as a separate milestone. This removes the ambiguity between the
document's title term and the architecture's term.

## Feedback
