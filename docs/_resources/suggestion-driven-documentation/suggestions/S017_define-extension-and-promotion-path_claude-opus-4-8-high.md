# S017 - Define the proposal extension and promotion target path

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

Suggestion filenames are hardcoded to `.md` (line 40), but the proposal name
uses a variable `{extension}` (line 137) that is never defined — and the slug
rule (line
27) has already dropped the target's extension by the time the proposal name is
built, so it cannot be recovered from `{document-name}`. The promotion step
(lines 147-148) says "copy the proposal over the original file" but never states
how the original *path* is recovered from the flattened key (e.g. for
`docs/api/contracts.md` → key `api-contracts`, promotion must write back to
`docs/api/contracts.md`, not a flattened path).

## Suggestion

State that `{extension}` is the target file's own extension, that suggestion
documents are always `.md` regardless of target type, and that promotion writes
back to the original target path (reversing the key derivation). If non-`.md`
targets are out of scope, say so explicitly.
