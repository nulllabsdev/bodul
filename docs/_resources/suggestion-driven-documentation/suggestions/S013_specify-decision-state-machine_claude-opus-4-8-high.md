# S013 - Specify and summarize the Decision state machine

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

`Decision` has five states (line 86), but their transitions are defined only in
scattered prose and two of them are dead ends. The Reviewing and Applying
sections only ever move `pending → accepted → completed` (lines 121-150). There
is no defined transition out of `deferred` (can it become `accepted` later?) and
no statement that `refused` is terminal — so a `deferred` suggestion like S005
sits in limbo. The non-obvious "stays `accepted` until the original is updated"
rule (lines 88-90, 143-146) is also easy to miss because it is spread across
sections.

## Suggestion

Add a compact state-transition summary near lines 86-90 — a small table (`State
| Set by | When | Next states`) or an inline flow such as `pending → accepted →
(applied to proposal: still accepted) → completed`, with `refused` as terminal
and `deferred` returning to a future review round. This both fills the missing
transitions and consolidates rules now spread across three sections.


## Feedback

deffered and refused can be at some point moved to accepted but this would be a
tiny edgecase
