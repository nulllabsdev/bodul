# S012 - Make Product Variant Boundary A Stage G Gate

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | high                                                                     |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Codex, gpt-5, medium                                                     |
| Reviewer                 |                                                                          |

## Issue

The roadmap correctly identifies the product-vs-variant boundary as an open
question, but later stages depend on that decision. Stage G creates or updates
canonical products, Stage H attaches retailer offers to inventory, and Stages
I-K project and expose those records. If Phase 0 does not decide whether a
product is a model with variants or a specific configuration, implementations
may create incompatible canonical IDs, offer links, event payloads, projections,
and API responses.

## Suggestion

Promote the product-vs-variant question to an explicit Stage G entry gate.
Record the Phase 0 default, for example: canonical product represents the
Minisforum model, configurations are variants, and retailer offers point to a
variant when configuration details are known. If the opposite default is
preferred, state that each configuration is its own canonical product and
describe how model-level grouping is deferred.

## Feedback
