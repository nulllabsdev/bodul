# S012 - Make the collapsible appendix example actually collapsible

| Field                    | Value                                                                                      |
| ------------------------ | ------------------------------------------------------------------------------------------ |
| Priority                 | medium                                                                                     |
| File                     | `docs/documentation-structure.md`                                                          |
| Decision                 | completed                                                                                  |
| Implementation reference | `docs/_resources/documentation-structure/proposals/documentation-structure-proposal-v1.md` |
| Created at               | 2026-06-18                                                                                 |
| Author                   | Codex, gpt-5, medium                                                                       |
| Reviewer                 |                                                                                            |

## Issue

Lines 140-158 tell the reader to add a "collapsible appendix", but the example
is just a normal Markdown heading and body. There is no collapsible mechanism in
the snippet, so the example does not demonstrate the behavior the prose asks
for.

## Suggestion

Either change the wording to "appendix" if collapsibility is only aspirational,
or show a concrete collapsible pattern such as an HTML `<details>` block and
note any renderer assumptions needed for it to work.

## Feedback
