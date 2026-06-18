# S016 - Complete the slug normalization rules

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

The `{document-name}` slug rules (lines 26-29) say "lowercase, ASCII, words
separated by single hyphens" but leave key cases unspecified: how non-ASCII
characters are transliterated vs. dropped, what counts as a word boundary
(spaces, underscores, dots, digits adjacent to letters), and how
leading/trailing/ consecutive separators collapse. The same gap applies to
`title` (lines 52-53). Because `{document-name}` is the collision-resistance key
(the point of S002), two different paths could slug to the same key under
divergent interpretations (e.g. `api_v2/contracts.md` vs `api-v2/contracts.md`),
reintroducing the collisions S002 set out to prevent.

## Suggestion

Specify a concrete normalization pipeline: lowercase; transliterate or strip
non-ASCII (state which); replace any run of non-alphanumeric characters with a
single hyphen; trim leading/trailing hyphens; preserve digits as-is. Apply the
same rule to `title`, and state a maximum length and truncation behavior for
both.

## Feedback

normalization pipeline: lowercase; transliterate or strip non-ASCII (state
which); replace any run of non-alphanumeric characters with a single hyphen;
trim leading/trailing hyphens; preserve digits as-is.
