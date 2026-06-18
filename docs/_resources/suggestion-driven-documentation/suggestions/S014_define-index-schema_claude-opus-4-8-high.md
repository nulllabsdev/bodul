# S014 - Define the INDEX.md schema and reconcile it with the companion guide

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

The document tells the reader to "update" `INDEX.md` (lines 127, 149) and says
it "links to suggestion documents for navigation" (lines 102-106), but never
states what an INDEX.md must *contain*. It defers entirely to
`documentation-structure.md` — but that companion prescribes a different INDEX
schema ("Sources Used / Decision Log / Open Questions", its lines 71-86) than
the one actually in use. The real `INDEX.md` for this very document uses `##
Suggestions` / `## Proposals` / `## Decision Log`, matching neither doc's stated
schema. A reader following the normative lifecycle doc cannot produce the INDEX
the workflow actually uses.

## Suggestion

Add a short "INDEX.md contents" subsection to
`suggestion-driven-documentation.md` defining the required sections for a
suggestion-driven INDEX (at minimum a `## Suggestions` link list and a `##
Proposals` link list, optionally `## Decision Log`), honoring the existing
no-duplicate-status-fields rule. Clarify that the companion's "Sources Used /
Open Questions" schema applies to research-driven docs, and update whichever
document is authoritative so its example matches sanctioned practice.
