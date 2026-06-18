# S005 - Add normative reference link

| Field                    | Value                             |
| ------------------------ | --------------------------------- |
| Priority                 | low                               |
| File                     | `docs/documentation-structure.md` |
| Decision                 | completed                         |
| Implementation reference | `docs/documentation-structure.md` |
| Created at               | 2026-06-18                        |
| Author                   | Codex, gpt-5, medium              |
| Reviewer                 | Miro                              |

## Issue

Lines 100-102 point readers to `suggestion-driven-documentation.md` for the full
lifecycle and naming rules, but the reference is plain text and does not say
which document is normative if the two guides disagree. This matters because the
two documents currently describe different resource-root and namespace rules.

## Suggestion

Make the reference an explicit markdown link to
`docs/suggestion-driven-documentation.md` and state that it is the normative
source for suggestion and proposal lifecycle rules.
