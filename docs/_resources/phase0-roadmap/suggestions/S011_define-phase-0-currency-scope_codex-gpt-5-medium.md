# S011 - Define Phase 0 Currency Scope

| Field                    | Value                    |
| ------------------------ | ------------------------ |
| Priority                 | high                     |
| File                     | `docs/phase0-roadmap.md` |
| Decision                 | refused                  |
| Implementation reference |                          |
| Created at               | 2026-06-19               |
| Author                   | Codex, gpt-5, medium     |
| Reviewer                 |                          |

## Issue

The roadmap says Phase 0 targets products Minisforum sells "worldwide" and that
retailer offers are inherently multi-currency, but it does not define which
currencies or regions Phase 0 actually supports. The existing `lib/money`
specification only supports USD, EUR, CAD, and AUD, and explicitly leaves
broader FX, tax/VAT, and cross-system multi-currency strategy to this roadmap.
Without a roadmap-level currency scope, offer ingestion, parsing, display, and
retailer selection can interpret "worldwide" differently.

## Suggestion

Add a Phase 0 currency/region scope decision to the roadmap. At minimum, state
whether Phase 0 is limited to the currencies currently supported by `lib/money`
(USD, EUR, CAD, AUD), whether unsupported retailer currencies are excluded or
captured as raw unparsed data, and whether FX conversion/tax normalization
remain out of scope or become explicit Phase 0 deliverables.

## Feedback
