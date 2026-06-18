# S003 - Cross-Reference the Money Library

| Field                    | Value                                |
| ------------------------ | ------------------------------------ |
| Priority                 | medium                               |
| File                     | `docs/phase0-roadmap.md`             |
| Decision                 | deferred                             |
| Implementation reference |                                      |
| Created at               | 2026-06-19                           |
| Author                   | Claude Code, claude-opus-4-8, medium |
| Reviewer                 |                                      |

## Issue

The "Supporting multiple currencies (Stages H, J, K)" open question (lines
38–53) covers storing source prices, presentation currency, FX, and tax/VAT.
Since this roadmap was written, a dedicated money library has been specified at
`lib/money/docs/` (`BR001_initial-business-requirements.md` and
`TS001_money-type.md`). TS001 explicitly states it *complements the
multi-currency open question in this roadmap* and that FX conversion is out of
its scope.

The relationship is now one-directional: the money spec points at this roadmap,
but the roadmap does not point back. A reader of the roadmap has no idea the
money type exists or that part of this open question is already being resolved:

- "Always store the source price and its ISO 4217 currency code on the offer"
  (lines 43–44) is exactly what `Money` + the canonical serialization in TS001
  §2.10 now provide.
- FX provider/refresh and VAT normalisation (lines 45–53) remain genuinely open
  here, and TS001 deliberately leaves them out — so the boundary is now defined,
  not just open.

## Suggestion

Add a cross-reference from the multi-currency section to the money library:

- link to `lib/money/docs/TS001_money-type.md` (and `BR001`), and
- note which parts of this open question are now committed (source-price + ISO
  currency storage, exact integer minor units) versus which remain roadmap-level
  open decisions (presentation-currency strategy, FX provider + refresh cadence,
  VAT normalisation).

This makes the two documents mutually discoverable and prevents duplicated or
conflicting currency decisions.

## Feedback
