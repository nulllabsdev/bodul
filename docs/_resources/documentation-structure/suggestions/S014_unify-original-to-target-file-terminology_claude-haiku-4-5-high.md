# S014 - Unify original to target-file terminology

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | medium                                                                                     |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Claude Code, claude-haiku-4-5, high                                                        |
| Reviewer                 |                                                                                            |

## Issue

Line 106 states "then the proposal is copied back over the **original**." The
companion document `suggestion-driven-documentation.md` has been updated (via
S020) to standardize on "target file" as the canonical term for the file being
reviewed, throughout the entire document. This single use of "original" in
`documentation-structure.md` is now inconsistent with that vocabulary and will
confuse readers who cross-reference between the two guides.

## Suggestion

Replace "original" on line 106 with "target file" to match the standardized
terminology from `suggestion-driven-documentation.md`. This maintains
consistency across both documents and reduces confusion for readers moving
between them.
