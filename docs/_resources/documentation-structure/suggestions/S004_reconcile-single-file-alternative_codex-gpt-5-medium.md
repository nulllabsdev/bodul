# S004 - Reconcile single file alternative

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

Lines 117-140 recommend a single-file appendix alternative for supporting
material. That advice conflicts with the suggestion-driven workflow when the
document is under review, because review feedback, decisions, and proposal
copies are supposed to live in `docs/_resources/{document-name}/...`, not inside
the deliverable itself.

## Suggestion

Constrain the single-file appendix alternative to documents that are not using
the suggestion-driven workflow, or state that suggestion and proposal artifacts
must still use `docs/_resources/{document-name}/...` even when other research
notes are kept in a lightweight appendix.
