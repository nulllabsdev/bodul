# S025 - Align promotion ownership

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

The overview and applying workflow disagree about who promotes the proposal to
the target file. Lines 24-28 say an LLM tool applies accepted suggestions to a
proposal copy, and a human reviewer promotes the proposal to the target file.
But the Applying suggestions workflow says the applier is the LLM tool (line
181), then has that applier copy the proposal over the target file and set
suggestions to `completed` (lines 196-201). This makes promotion ownership
ambiguous at the highest-risk step in the workflow.

## Suggestion

Choose one promotion owner and make the overview, Applying suggestions section,
and `Decision` state table agree. If promotion is human-owned, split Applying
into LLM application and human promotion steps. If promotion is LLM-owned,
update the overview to remove the human reviewer promotion claim and clarify the
human reviewer's authority before promotion.

## Feedback

LLM should do it
