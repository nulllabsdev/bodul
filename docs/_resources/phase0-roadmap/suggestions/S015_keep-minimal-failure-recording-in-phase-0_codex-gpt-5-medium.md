# S015 - Keep Minimal Failure Recording In Phase 0

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | medium                                                                   |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Codex, gpt-5, medium                                                     |
| Reviewer                 |                                                                          |

## Issue

The architecture section defers error handling and observability to a later
phase, including retries, failure storage, and alerting on parser breakage.
Phase 0 still depends on HTTP fetching, sitemap parsing, retailer-specific HTML
parsers, and manual or LLM-assisted classification. Without at least minimal
failure records, manual Phase 0 runs will be hard to resume, inspect, or rerun
when a fetch, parse, or classification step fails.

## Suggestion

Keep full observability and alerting deferred, but add minimal Phase 0 failure
recording as in scope. Record the failed stage, retailer, source URL or artifact
ID, failure kind/message, and timestamp so manual reruns can target failed work.
Clarify that automated retry policy, alerting, dashboards, and parser breakage
monitoring remain later-phase work.

## Feedback
