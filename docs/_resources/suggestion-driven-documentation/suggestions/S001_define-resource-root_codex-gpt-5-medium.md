# S001 - Define resource root

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

The storage path is ambiguous. Lines 18-20 say to store suggestions in
`_resources/{document-name}/suggestions/`, but lines 30-32 show suggestions for
`docs/contracts.md` going into `docs/_resources/contracts/suggestions/`. Those
two statements imply different roots unless the reader infers that `_resources`
is relative to the reviewed file's parent directory or to `docs/`.

## Suggestion

State the resource root explicitly. For example, define whether suggestion and
proposal folders live under `docs/_resources/`, beside each reviewed document,
or under a repository-level `_resources/` directory. Then update every path in
the document to use the same convention.

## Feedback

folder _resources should be for now in docs/ folder. Update all
