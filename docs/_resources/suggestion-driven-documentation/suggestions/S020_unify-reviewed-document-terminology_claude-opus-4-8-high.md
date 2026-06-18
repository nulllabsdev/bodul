# S020 - Unify the terminology for the reviewed document

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

The document being reviewed is named with at least four different nouns and no
defined relationship between them: "target file" (lines 26, 46, 48-49), "target
document" (line 115), "original file" / "original target file" (lines 85, 88),
and "original" (line 148), plus "deliverable" inherited from the companion
guide. Line 85 says `File` records "the original file being reviewed," while
line 115 introduces "target document" for the same input to the Creating step. A
reader cannot tell whether these terms denote the same artifact.

## Suggestion

Pick one canonical term (e.g. "target file"), define it once near the top, and
use it consistently. Reserve "proposal copy" for the intermediate artifact, and
avoid introducing "original," "target document," and "deliverable" as if they
were distinct things.
