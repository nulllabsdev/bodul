# S005 - Close application commit flow

| Field                    | Value                                     |
| ------------------------ | ----------------------------------------- |
| Priority                 | medium                                    |
| File                     | `docs/suggestion-driven-documentation.md` |
| Decision                 | deferred                                  |
| Implementation reference |                                           |
| Created at               | 2026-06-18                                |
| Author                   | Codex, gpt-5, medium                      |
| Reviewer                 | Miro                                      |

## Issue

The creation workflow on lines 82-84 explicitly commits suggestion documents,
and the application workflow on lines 100-104 commits the proposal copy as-is.
After that, lines 105-110 mutate the proposal, update suggestion metadata, and
copy the proposal over the original file, but the document does not say when to
commit those final changes.

## Suggestion

Add explicit commit points for the applied proposal, updated suggestion
documents, and final original-file update. If those changes should be committed
together, say so and name the expected commit contents. If they should be split,
define the order and purpose of each commit.
