# S014 - Define Event Sourcing Boundaries

| Field                    | Value                    |
| ------------------------ | ------------------------ |
| Priority                 | medium                   |
| File                     | `docs/phase0-roadmap.md` |
| Decision                 | deferred                 |
| Implementation reference |                          |
| Created at               | 2026-06-19               |
| Author                   | Codex, gpt-5, medium     |
| Reviewer                 |                          |

## Issue

The roadmap says Stage A needs commands and events support, and Stages G/H need
event sourcing, while earlier stages store sitemaps, page HTML, and extracted
data in the database. It does not define which data is event-sourced and which
is ordinary scrape artifact storage. This leaves unclear boundaries for command
handlers, event streams, replay, projections, and read-side ownership.

## Suggestion

Add a concise event-sourcing boundary section. Specify whether Phase 0
event-sources only durable business state such as canonical products and
retailer offers, while sitemap/page fetch artifacts are stored as operational
records, or whether every pipeline stage emits events. Also name the minimum
event streams or aggregate boundaries required before Stages I/J can build
projections.

## Feedback
