# S008 - Integrate INDEX.md into the lifecycle

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | medium                                                                                                     |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 | Miro                                                                                                       |

## Issue

The companion guide `docs/documentation-structure.md` (lines 87-98) makes
`INDEX.md` the bridge that links accepted, refused, and deferred suggestions to
the deliverable, and points readers to `suggestion-driven-documentation.md` for
"the full lifecycle." But this document never mentions `INDEX.md` and never
tells the reviewer or applier when to create or update it. The cross-reference
is one-directional: the structure guide depends on a lifecycle step that the
lifecycle document does not define.

## Suggestion

Add `INDEX.md` to the lifecycle. At minimum, add a step in the Reviewing
workflow (after a decision is recorded) and/or the Applying workflow (when a
proposal is promoted to the original) to update `INDEX.md` so its links stay
current, and add a back-reference to `documentation-structure.md`. Keep the
division of responsibility the structure guide already states: the suggestion
document is the source of truth for its own decision; `INDEX.md` links or
summarizes without restating status.
