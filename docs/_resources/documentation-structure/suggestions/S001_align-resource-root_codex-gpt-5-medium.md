# S001 - Align resource root

| Field                    | Value                             |
| ------------------------ | --------------------------------- |
| Priority                 | high                              |
| File                     | `docs/documentation-structure.md` |
| Decision                 | completed                         |
| Implementation reference | `docs/documentation-structure.md` |
| Created at               | 2026-06-18                        |
| Author                   | Codex, gpt-5, medium              |
| Reviewer                 | Miro                              |

## Issue

The folder tree and explanatory text on lines 21-51 describe `_resources/` as a
directory beside the deliverable inside an arbitrary project folder. The current
suggestion-driven workflow defines the resource root as `docs/_resources/`, with
all suggestion and proposal folders under that root. Readers following this
guide could create suggestion folders in a different place than the canonical
workflow expects.

## Suggestion

Update the recommended structure to match the canonical `docs/_resources/` root,
or explicitly separate the general documentation-resource pattern from the
repository-specific suggestion workflow. If this guide is meant to govern this
repository, show examples rooted at `docs/_resources/{document-name}/...`.

## Feedback

Yes, path is `docs/_resources/`
