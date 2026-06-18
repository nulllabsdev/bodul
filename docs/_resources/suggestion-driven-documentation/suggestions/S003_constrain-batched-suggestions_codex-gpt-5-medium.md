# S003 - Constrain batched suggestions

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

Lines 14-16 allow multiple small editorial suggestions in one document, but
lines 64-76 define a single `Priority`, `Decision`, `Implementation reference`,
and `Reviewer` for the whole suggestion document. If one item in the batch is
accepted and another is refused or deferred, the current schema cannot record
that accurately.

## Suggestion

Either require batched suggestions to be indivisible, meaning every item in the
batch must share the same decision and implementation reference, or add
item-level tracking fields for batched suggestions. The simpler rule is to keep
batches atomic and split any mixed-decision batch into separate suggestion
documents during review.

## Feedback

keep batches atomic and split any mixed-decision batch into separate suggestion
documents during review
