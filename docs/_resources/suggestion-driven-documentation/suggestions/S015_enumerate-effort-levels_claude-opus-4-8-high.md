# S015 - Enumerate allowed effort levels

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

The `tool-model-effort` filename segment (lines 54-56) and the `Author` field
(lines 70, 95-96) both require an "effort level," but the allowed values are
never enumerated — they appear only by example (`-high`, `-medium`). By
contrast, `Priority` (line 84) explicitly lists `low`, `medium`, `high`. A
reader cannot tell whether effort shares that exact set, allows others (e.g.
`max`, `none`), or is free-form, and whether tools that have no effort concept
omit the segment.

## Suggestion

Add an explicit enumeration for effort, e.g. "`effort` must be one of `low`,
`medium`, `high`," and state whether it is intentionally the same set as
`Priority` or independent. Define the fallback for tools without an effort
concept (omit the segment, or use a sentinel such as `default`).
