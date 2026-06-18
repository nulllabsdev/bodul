# S006 - Normalize resource path prefixes

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 | Miro                                                                                                       |

## Issue

Path references disagree about the `docs/` prefix. The resource root is defined
with the full prefix on line 22 (`docs/_resources/{document-name}/suggestions/`)
and lines 47-48 (`docs/_resources/contracts/suggestions/`), but later references
drop it:

- Line 89 (`Implementation reference` example):
  `_resources/{document-name}/proposals/{document-name}-proposal-v1.md`
- Line 102 (Creating suggestions): `_resources/{document-name}/suggestions/`
- Line 119 (Applying suggestions): `_resources/{document-name}/proposals/`

S001 fixed the root *definition* to live under `docs/`, but the workflow steps
and the template example were left half-converted. A reader cannot tell whether
these paths are repository-relative or `docs/`-relative, which is exactly the
ambiguity S001 set out to remove.

## Suggestion

Adopt a single convention and apply it everywhere. Recommended: use the explicit
`docs/_resources/...` root established by S001 in every path reference,
including the `Implementation reference` example in the template and steps 1 of
both the Creating and Applying workflows. If repo-relative `_resources/...` is
preferred instead, change lines 22 and 47-48 to match — but do not leave both
forms in the document.

## Feedback

for now it will be explicit `docs/_resources/...`
