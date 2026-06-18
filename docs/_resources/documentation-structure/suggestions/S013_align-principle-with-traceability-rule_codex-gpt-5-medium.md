# S013 - Align the principle table with the traceability rule

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | medium                                                                                     |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Codex, gpt-5, medium                                                                       |
| Reviewer                 |                                                                                            |

## Issue

The principle table says `INDEX` "links decisions, not just files" (line 126),
but the traceability rule on lines 107-110 says `INDEX.md` must not restate or
duplicate suggestion `Decision`, `Reviewer`, or `Implementation reference`
fields. "Links decisions" can be read as a request to summarize decision status
inside `INDEX.md`, which conflicts with the stricter traceability rule later in
the document.

## Suggestion

Revise the principle wording so it matches the operational rule. For example,
change it to "INDEX links decision artifacts, not just files" or "INDEX links
document-level rationale, not per-suggestion status fields."

## Feedback

INDEX links decision artifacts, not just files
