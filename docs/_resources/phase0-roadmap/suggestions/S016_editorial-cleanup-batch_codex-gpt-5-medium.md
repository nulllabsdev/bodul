# S016 - Editorial Cleanup Batch

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | low                                                                      |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Codex, gpt-5, medium                                                     |
| Reviewer                 |                                                                          |

## Issue

The roadmap has a few small editorial issues that do not change behavior but
make the document read less cleanly:

- "than later" should be "then later".
- "id's" and "skus" should be normalized to "IDs" and "SKUs".
- "eventsourcing" should be "event sourcing".
- "ATM" should be expanded or replaced with clearer wording such as "for now".
- Several long bullets would be easier to review if wrapped consistently.

## Suggestion

Apply these editorial fixes as a single atomic cleanup batch. Do not change
roadmap scope or decisions in this batch; keep it limited to spelling,
capitalization, terminology, and line-wrapping cleanup.

## Feedback
