# S002 — Add Stage Completion Criteria

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | high                                                                     |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | OpenCode, deepseek-v4-pro, high                                          |
| Reviewer                 |                                                                          |

## Issue

Each stage (A through K) describes activities but does not define what "done"
means for that stage. Without acceptance criteria, it is not clear when a stage
is complete, and progress tracking across the roadmap is ambiguous.

For example, Stage A says "Trigger sitemap product sourcing for all active
retailers," but does a single successful manual trigger count as done, or does
the mechanism need to be reliable enough to hand off to automation?

## Suggestion

Add a "Done when" bullet list to each stage. Examples:

- **Stage A**: "Done when a manual trigger produces side effects in the event
  store for every hardcoded retailer, and each side effect is replayable."
- **Stage D**: "Done when product pages for all active retailers can be fetched
  and stored with HTTP 200 success rate ≥ 90% across a full run."
- **Stage K**: "Done when the SPA can display an inventory page and an offer
  page for a canonical product, populated from the read-side projections."

This makes each stage independently verifiable and gives a clear handoff signal
to the next stage.

## Feedback

(None yet — pending review)
