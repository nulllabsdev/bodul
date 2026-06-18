# S015 - Defer slug spec to normative source

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | low                                                                                        |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Claude Code, claude-haiku-4-5, high                                                        |
| Reviewer                 |                                                                                            |

## Issue

Lines 51-53 state the slug normalization rule as "lowercase, ASCII, single
hyphens." The companion `suggestion-driven-documentation.md` now defines a
complete normalization pipeline (via S016): lowercase; transliterate or strip
non-ASCII; replace any run of non-alphanumeric characters with a single hyphen;
trim leading and trailing hyphens; preserve digits as-is. The partial rule in
`documentation-structure.md` omits critical steps (collapsing separators,
trimming), so readers who rely only on this document's statement will not have
the full spec. Although lines 61-62 point to the other document as "the
canonical rule," the side-by-side partial restatement risks drift if the
documents are not kept in sync.

## Suggestion

Remove the partial slug rule from lines 51-53 and replace it with a forward
reference to the normative source: "For the complete slug normalization
pipeline, see
[`suggestion-driven-documentation.md`](suggestion-driven-documentation.md)."
This eliminates duplication and ensures all writers consult the one
authoritative source for the exact normalization rules.
