# S007 - Make structure example portable

| Field                    | Value                                          |
| ------------------------ | ---------------------------------------------- |
| Priority                 | low                                            |
| File                     | `docs/documentation-structure.md`              |
| Decision                 | refused                                        |
| Implementation reference |                                                |
| Created at               | 2026-06-18                                     |
| Author                   | Codex documentation review team, gpt-5, medium |
| Reviewer                 |                                                |

## Issue

The folder structure example on lines 21-44 uses box-drawing characters and
inline arrow annotations. This is readable in rendered Markdown, but it is less
portable for plain-text tools, screen readers, and copy-paste reuse. It also
mixes structural paths with explanatory comments in the same block.

## Suggestion

Replace the tree with an ASCII-only code block and move the explanations into
bullets immediately below it. For example, show just the paths in the code
block, then explain `document.md`, `INDEX.md`, `suggestions/`, `proposals/`, and
the optional folders in prose.

## Feedback
