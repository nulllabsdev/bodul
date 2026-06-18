# S006 — Expand Stage G Entity Resolution

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | medium                                                                   |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | OpenCode, deepseek-v4-pro, medium                                        |
| Reviewer                 |                                                                          |

## Issue

The Open Questions section calls product matching "the hardest part of the
pipeline" and dedicates 14 lines of detailed analysis to matching signals,
variants, and stability. However, the Stage G description itself (lines 96–100)
is only 4 lines and generic:

> Match products with existing ones, otherwise add new product
> Product inventory should hold all kinds of content…
> Needs eventsourcing

The stage body does not summarize the approach, reference the open questions, or
indicate how the open decisions affect the stage's design. A reader scanning
stage descriptions alone would miss the complexity.

## Suggestion

Expand the Stage G body with:
1. A one-sentence summary of the matching approach (signal precedence, store
   mapping for stability).
2. An explicit cross-reference to the Product Matching open questions block.
3. A note on what the "minimal viable" matching looks like for Phase 0 (e.g.,
   exact model-name match only, deferring fuzzy/LLM-assisted matching).

## Feedback

(None yet — pending review)
