# S012 - Govern subsequent review rounds on a revised document

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

The workflow describes a single pass: create suggestions, review, build proposal
v1, promote it (lines 113-150). It handles proposal *versioning* ("if proposal
versions already exist, use the next number," lines 138-139) but never governs a
second *review round* on a document that has already been revised — which is
exactly the current situation (proposal-v1 exists and S001-S009 are already
decided). It is unspecified whether new reviewers review the just-promoted
original or the latest proposal, whether `completed`/`refused` suggestions are
in scope, and how `deferred` items (such as the still-open S005) re-enter
review.

## Suggestion

Add a "Subsequent rounds" note stating that each new round reviews the current
original file; that `completed` and `refused` suggestions are out of scope
unless re-raised as new suggestions; and that `deferred` suggestions are
explicitly reconsidered in the next round. Tie this to the `Decision` state
rules.

## Feedback

Only accepted suggestions can be worked on
