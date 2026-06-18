# S008 - Make index example match ownership rules

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | high                                                                                       |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Codex documentation review team, gpt-5, medium                                             |
| Reviewer                 |                                                                                            |

## Issue

Lines 107-110 say `INDEX.md` must not duplicate `Decision`, `Reviewer`, or
`Implementation reference` because suggestion documents are the source of truth
for those fields. The `INDEX.md` example on lines 71-86 does not include a
suggestions section, so readers do not see how to link suggestions without
restating their status. The example also includes a broad "Decision Log" that
could be mistaken for a place to copy suggestion status.

## Suggestion

Update the `INDEX.md` example to include a `## Suggestions` section with links
to suggestion documents only. Add a short note under the example that the
decision log can record document-level rationale, but suggestion status fields
must remain only in the suggestion documents.

## Feedback
