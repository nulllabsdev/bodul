# S023 - Bootstrap resource artifacts explicitly

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v3.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Codex, gpt-5, medium                                                                                       |
| Reviewer                 |                                                                                                            |

## Issue

The lifecycle assumes the resource tree already exists. Creating suggestions
(lines 156-162) tells the author to write into
`docs/_resources/{document-name}/suggestions/`, and Reviewing suggestions (lines
164-175) says to update `docs/_resources/{document-name}/INDEX.md`. The document
never states who creates `docs/_resources/{document-name}/`, who creates
`suggestions/`, or what to do when `INDEX.md` does not exist yet on the first
review round.

## Suggestion

Add an explicit bootstrap rule in Creating suggestions: if
`docs/_resources/{document-name}/` or its `suggestions/` folder does not exist,
create it; if `INDEX.md` does not exist, create it using the documented
suggestion-driven schema before review begins.

## Feedback
