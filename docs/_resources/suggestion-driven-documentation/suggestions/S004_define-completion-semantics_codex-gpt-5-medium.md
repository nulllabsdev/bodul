# S004 - Define completion semantics

| Field                    | Value                                     |
| ------------------------ | ----------------------------------------- |
| Priority                 | medium                                    |
| File                     | `docs/suggestion-driven-documentation.md` |
| Decision                 | completed                                 |
| Implementation reference | `docs/suggestion-driven-documentation.md` |
| Created at               | 2026-06-18                                |
| Author                   | Codex, gpt-5, medium                      |
| Reviewer                 | Miro                                      |

## Issue

Lines 105-109 say to set an accepted suggestion to `completed` after applying it
to the proposal copy. Line 110 then says to copy the proposal over the original
file after every accepted suggestion targeting a proposal has been resolved.
This makes `completed` mean "applied to a proposal" before the original file has
actually been updated, which can be misleading during interrupted or partially
finished workflows.

## Suggestion

Define exactly what `completed` means. If completion means applied to the final
target document, keep suggestions in an intermediate state until the proposal is
copied over the original. If completion means applied to a proposal, add a
separate field or workflow state that records whether the proposal has been
promoted to the original file.

## Feedback

You are right, completed needs to mean after applying on original file
