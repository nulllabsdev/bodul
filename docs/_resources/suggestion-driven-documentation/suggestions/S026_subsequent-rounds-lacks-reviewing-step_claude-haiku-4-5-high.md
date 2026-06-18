# S026 - Subsequent rounds lacks reviewing step for deferred

| Field                    | Value                                     |
| ------------------------ | ----------------------------------------- |
| Priority                 | low                                       |
| File                     | `docs/suggestion-driven-documentation.md` |
| Decision                 | pending                                   |
| Implementation reference |                                           |
| Created at               | 2026-06-18                                |
| Author                   | Claude Code, claude-haiku-4-5, high       |
| Reviewer                 |                                           |

## Issue

`## Subsequent rounds` (lines 203-209) states that deferred suggestions "may be
moved to `accepted` by the reviewer before applying begins, or re-raised as new
suggestions." But `## Reviewing suggestions` (lines 164-175) lists the
reviewer's checklist with step 1 as "Reads each suggestion with `Decision` set
to `pending`." This step excludes prior `deferred` suggestions, so a reviewer
following the checklist mechanically would only process new `pending`
suggestions and would never be prompted to revisit deferred items from earlier
rounds.

## Suggestion

Add an explicit step to the Reviewing section checklist (e.g., before current
step 1) instructing the reviewer to identify and evaluate `deferred` suggestions
from prior rounds: "Checks for suggestions with `Decision` set to `deferred`,
decides whether to move them to `accepted`, or leaves them deferred for a future
round." This makes the deferred-suggestion workflow part of the official
Reviewing process.


## Feedback

Lets uncomplicate, deferred and skipped are final steps
