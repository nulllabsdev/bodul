# S002 - Avoid target name collisions

| Field                    | Value                                     |
| ------------------------ | ----------------------------------------- |
| Priority                 | high                                      |
| File                     | `docs/suggestion-driven-documentation.md` |
| Decision                 | completed                                 |
| Implementation reference | `docs/suggestion-driven-documentation.md` |
| Created at               | 2026-06-18                                |
| Author                   | Codex, gpt-5, medium                      |
| Reviewer                 | Miro                                      |

## Issue

Lines 18-20 derive `{document-name}` from the target file's name without its
extension or directory path. This can collide for files that share a basename,
such as `docs/api/contracts.md` and `docs/legal/contracts.md`. Lines 30-34 then
say numbering is tracked per target file suggestion folder, but the folder name
is not guaranteed to identify a unique target file.

## Suggestion

Define a collision-resistant target key. Options include preserving the relative
path under `_resources`, using a slug derived from the relative path, or adding
a required target path field to the folder naming scheme. Include an example
with two files that have the same basename in different directories.

## Feedback

Prepend folder paths to document-name, force SEO rules onto it while replacing
slash with `-`
