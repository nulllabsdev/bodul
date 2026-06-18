# S004 - Clarify the Overview Business Model

| Field                    | Value                                                                    |
| ------------------------ | ------------------------------------------------------------------------ |
| Priority                 | low                                                                      |
| File                     | `docs/phase0-roadmap.md`                                                 |
| Decision                 | completed                                                                |
| Implementation reference | `docs/_resources/phase0-roadmap/proposals/phase0-roadmap-proposal-v1.md` |
| Created at               | 2026-06-19                                                               |
| Author                   | Claude Code, claude-opus-4-8, medium                                     |
| Reviewer                 |                                                                          |

## Issue

The Overview (lines 3–6) says: "We will be scraping, processing and displaying
products that Minisforum sells worldwide." This wording implies Minisforum is
the seller. The rest of the document describes a different model:

- We scrape **retailers** (Stages A–E talk about retailers, sitemaps, product
  pages), and
- Stage G (lines 96–101) and the Product matching open question (lines 13–36)
  make clear Minisforum is the single **manufacturer** whose products are sold by
  many different retailers, each of which we scrape.

So the pipeline tracks one manufacturer's products as offered by multiple
retailers — not products that the manufacturer itself sells worldwide. The
Overview's one-sentence framing undersells (and slightly misstates) the core
domain model, which is the manufacturer/retailer/offer distinction that the
whole pipeline is built around.

## Suggestion

Reword the Overview to state the actual model concisely, e.g.: "We scrape,
match, and display offers for Minisforum products (a single manufacturer) as
sold by many retailers worldwide." Optionally add one line naming the three core
entities the pipeline produces — product (manufacturer-side), retailer, and
offer (retailer-side) — since Stages G and H already split inventory data from
offer data along exactly that line.

## Feedback

for mvp, we will focus on minisforum only
