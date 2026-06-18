# S003 - Clarify index status ownership

| Field                    | Value                             |
| ------------------------ | --------------------------------- |
| Priority                 | medium                            |
| File                     | `docs/documentation-structure.md` |
| Decision                 | completed                         |
| Implementation reference | `docs/documentation-structure.md` |
| Created at               | 2026-06-18                        |
| Author                   | Codex, gpt-5, medium              |
| Reviewer                 | Miro                              |

## Issue

Lines 96-98 say each suggestion document is the source of truth for its decision
and that `INDEX.md` links to or summarizes accepted, refused, and deferred
suggestions without restating their status. This is easy to misapply: a summary
of accepted, refused, and deferred suggestions will often become another status
record unless the guide defines what `INDEX.md` may and may not duplicate.

## Suggestion

Define the boundary between `INDEX.md` and suggestion documents. For example,
state that `INDEX.md` may group or link suggestion files for navigation, but the
`Decision`, `Reviewer`, and `Implementation reference` fields must only be read
from the suggestion documents.
