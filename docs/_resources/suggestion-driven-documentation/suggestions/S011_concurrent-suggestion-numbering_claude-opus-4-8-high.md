# S011 - Define suggestion numbering under concurrent authorship

| Field                    | Value                                     |
| ------------------------ | ----------------------------------------- |
| Priority                 | high                                      |
| File                     | `docs/suggestion-driven-documentation.md` |
| Decision                 | deferred                                  |
| Implementation reference |                                           |
| Created at               | 2026-06-18                                |
| Author                   | Claude Code, claude-opus-4-8, high        |
| Reviewer                 |                                           |

## Issue

Lines 44-51 require suggestion numbers to be "sequential" and "the next
available number in that folder," but the document assumes a single author. When
multiple reviewers create suggestions in parallel — for example a team of
reviewers in one round, which is exactly this review's situation — each
independently computes "the next available number" from the same folder state
and they collide (two authors both write `S010`). The "Creating suggestions"
workflow (lines 113-117) reinforces the single-actor assumption: review, then
commit, with no coordination step.

## Suggestion

Add a concurrency rule for number assignment. Options: reserve disjoint number
ranges per author, have a coordinator assign numbers before parallel work
begins, or define a collision-resolution rule (renumber on commit conflict) so
parallel authors never clash on `S{number}`. State the chosen rule in the
"Creating suggestions" section.
