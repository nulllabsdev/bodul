# S024 - Stop claiming key derivation is reversible

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v3.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Codex, gpt-5, medium                                                                                       |
| Reviewer                 |                                                                                                            |

## Issue

Applying suggestions step 6 says the applier copies the proposal over the target
file by "reversing the `{document-name}` key derivation" (lines 196-199). That
is not generally possible. The key derivation intentionally flattens `/` to `-`
and can transliterate or strip non-ASCII characters and collapse punctuation
(lines 36-40), which makes the mapping lossy. A key like `api-contracts` does
not uniquely encode the original path once slug normalization has been applied.

## Suggestion

Replace the reversibility claim with a concrete source of truth for the target
path. For example, say the applier must read the original path from each
suggestion document's `File` field, or from proposal metadata recorded when the
proposal copy is created. The workflow should not imply that the slug alone can
recover the original file path.

## Feedback
