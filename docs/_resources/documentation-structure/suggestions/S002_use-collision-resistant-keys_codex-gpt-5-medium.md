# S002 - Use collision resistant keys

| Field                    | Value                             |
| ------------------------ | --------------------------------- |
| Priority                 | high                              |
| File                     | `docs/documentation-structure.md` |
| Decision                 | completed                         |
| Implementation reference | `docs/documentation-structure.md` |
| Created at               | 2026-06-18                        |
| Author                   | Codex, gpt-5, medium              |
| Reviewer                 | Miro                              |

## Issue

Lines 46-51 say each deliverable namespace is named after the target filename
without its extension and that this avoids collisions when several documents
share one `_resources/` parent. Basename-only namespaces do not avoid
collisions: `docs/api/contracts.md` and `docs/legal/contracts.md` would both map
to `contracts`. The current suggestion-driven workflow instead derives
`{document-name}` from the target path relative to `docs/`.

## Suggestion

Replace the basename-only rule with the collision-resistant key rule from
`docs/suggestion-driven-documentation.md`: drop the extension from the path
relative to `docs/`, replace `/` with `-`, and slug the result. Include examples
for both `docs/contracts.md` and a nested file such as `docs/api/contracts.md`.


## Feedback

I think we solved that in docs/suggestion-driven-documentation.md, look there
