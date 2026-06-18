# S007 - Document the Feedback section in the template

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

The suggestion-document template on lines 58-78 defines only `## Issue` and `##
Suggestion`. In practice, every existing suggestion document (S001-S005) carries
an additional `## Feedback` section that records the reviewer's note or
rationale for the decision. The documented schema and the actual convention
disagree, and the template currently provides no defined place to record *why* a
suggestion was refused or deferred — information that the companion guide
expects to surface (see `documentation-structure.md`, "Decision Log").

## Suggestion

Add `## Feedback` to the template as an optional section, and document its
ownership and timing in the field list around lines 80-95: the reviewer writes
it when recording a decision, and it is especially expected for `refused` and
`deferred` suggestions where the rationale would otherwise be lost. Make clear
it is written by the reviewer (not the suggestion author) so the roles stay
distinct.
