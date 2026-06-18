# S006 - Define scope and audience

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | medium                                                                                     |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Codex documentation review team, gpt-5, medium                                             |
| Reviewer                 |                                                                                            |

## Issue

The guide alternates between repository-specific guidance and general document
organization guidance without defining its scope. Lines 21-62 make `docs/` and
`docs/_resources/` normative for this repository, while lines 132-161 introduce
a single-file alternative that is not under the suggestion-driven workflow. A
new writer cannot tell whether the document is a repository policy, a portable
pattern, or both.

## Suggestion

Add a short "Scope" or "Audience" section near the top. State that the primary
policy applies to documentation under `docs/` in this repository, and that the
single-file appendix is only a lightweight alternative for documents outside the
suggestion-driven workflow.

## Feedback

use only docs/_resources/
