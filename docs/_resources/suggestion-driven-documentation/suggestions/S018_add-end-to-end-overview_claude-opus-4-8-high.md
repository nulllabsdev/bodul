# S018 - Add an end-to-end flow overview

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | medium                                                                                                     |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

After a two-paragraph rationale (lines 8-20), the document dives straight into
storage paths, the collision-key algorithm, and filename minutiae (lines 22-56)
before a first-time reader ever learns the Create → Review → Apply lifecycle
that those names serve. The lifecycle only appears at lines 111-150, so a
newcomer must hold a lot of reference detail with no mental model to hang it on.
The companion `documentation-structure.md` leads with "The Core Problem" and a
solution shape before details.

## Suggestion

Add a short "Overview" / "How it works" section right after the intro (before
line
22) with three or four sentences naming the phases — create suggestions, review and
decide, apply via a proposal then promote to the original — and who performs
each. This gives the reader the spine before the bones.
