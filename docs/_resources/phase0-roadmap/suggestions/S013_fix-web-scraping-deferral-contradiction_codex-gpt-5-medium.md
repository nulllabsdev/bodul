# S013 - Fix Web Scraping Deferral Contradiction

| Field                    | Value                    |
| ------------------------ | ------------------------ |
| Priority                 | medium                   |
| File                     | `docs/phase0-roadmap.md` |
| Decision                 | refused                  |
| Implementation reference |                          |
| Created at               | 2026-06-19               |
| Author                   | Codex, gpt-5, medium     |
| Reviewer                 |                          |

## Issue

Stage A says "Processing catalogs and scraping web is deferred", but Stages B-D
then describe Phase 0 sitemap fetching, sitemap processing, product URL
extraction, and product page fetching. Those are web-scraping activities. The
current wording makes it unclear whether Phase 0 includes only sourcing URLs,
fetching sitemap files, fetching product pages, or fully processing product
pages.

## Suggestion

Rewrite the Stage A deferral bullet to distinguish the specific work that is
deferred from the scraping work that remains in Phase 0. For example, state that
Phase 0 includes sitemap fetch/process and product page fetch, while collection
and catalog page processing are deferred. Keep Stage C/D wording consistent with
that boundary.

## Feedback
