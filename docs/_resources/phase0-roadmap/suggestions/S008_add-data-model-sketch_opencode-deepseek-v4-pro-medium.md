# S008 — Add Data Model Sketch

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

The roadmap refers to two core entities — "product inventory" and "retailer
offers" — across half the stages (G, H, I, J, K) but never defines what they
contain or how they relate. The reader must infer the data model from
descriptions like "product inventory should hold all kinds of content: from
descriptions, features, technical specifications" (Stage G) and "retailer offers
store retailer side of data like id's, skus, prices, availability" (Stage H).

Without even a minimal logical model, the read-side stages (I, J) and the API
(Stage K) are underspecified.

## Suggestion

Add a "Core entities" subsection to the Architecture section with a minimal
logical model:

```
Product {
  id, canonical_name, model_name, brand, category,
  description, specifications, images
}

Offer {
  id, product_id → Product.id,
  retailer, retailer_sku, url, price, currency (ISO 4217),
  availability, scraped_at
}
```

This grounds the stage descriptions and gives the read-side stages (I, J) a
concrete target without committing to the physical schema.

## Feedback

(None yet — pending review)
