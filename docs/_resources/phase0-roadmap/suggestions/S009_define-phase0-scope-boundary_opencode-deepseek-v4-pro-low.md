# S009 — Define Phase 0 Scope Boundary

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | low                                                                      |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | OpenCode, deepseek-v4-pro, low                                           |
| Reviewer                 |                                                                          |

## Issue

The Overview says "products that Minisforum sells worldwide" and the
Architecture says "targeting minimum viable product," but there is no explicit
list of what is intentionally *out* of scope for Phase 0. Several items are
marked "TBD" or "deferred" inline, but a single boundary statement would help
stakeholders understand what Phase 0 will *not* deliver, avoiding scope creep.

## Suggestion

Add an "Out of scope" section after the Architecture that lists what is
explicitly deferred past Phase 0:

- Multi-brand support (Minisforum only)
- Automated scheduler (manual trigger only)
- Catalog/collection processing
- Anti-scraping infrastructure beyond a basic HTTP fetch
- Model training for cheaper classification (high-capability LLM only)
- Admin UI or retailer management UI
- Multi-language content
- Price alerts or notification system

This makes the MVP boundary explicit and gives the TBD/deferred items a
canonical home (in combination with S001's deferred-decisions table).

## Feedback

(None yet — pending review)
