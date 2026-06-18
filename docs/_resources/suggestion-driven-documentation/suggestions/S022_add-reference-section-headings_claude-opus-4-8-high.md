# S022 - Add headings to the reference block

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | low                                                                                                        |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

The first ~110 lines interleave reference material (storage path, collision key,
filename format, field definitions) with no heading structure at all, then the
document abruptly switches to `##`-headed procedural sections at line 111. The
unheaded reference block is hard to navigate and visually indistinguishable from
the rationale prose, so a reader cannot tell where "rules to look up" end and
"steps to follow" begin, and a rendered table of contents shows only the
workflow half.

## Suggestion

Give the reference block explicit headings (e.g. `## Where suggestions live`,
`## File naming`, `## Suggestion document structure`) so the document has a
consistent heading hierarchy throughout and the reference-vs-procedure boundary
is visible in any table of contents.
