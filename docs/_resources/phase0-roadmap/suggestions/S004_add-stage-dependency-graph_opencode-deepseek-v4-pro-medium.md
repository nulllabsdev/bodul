# S004 — Add Stage Dependency Graph

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

Stages A through K are listed sequentially, but the actual dependency
relationships are more nuanced than a flat list suggests:
- B depends on A (needs a trigger + retailer data).
- C depends on B (needs sitemap files stored).
- D depends on C (needs URLs extracted).
- E depends on D (needs raw pages stored).
- G depends on E (needs parsed data) and possibly F (classification).
- I depends on G (needs product events).
- J depends on H (needs offer events), and H depends on G.
- K depends on I and J (needs read-side projections).
- F (LLM classification) can run independently of E if manual, or after E if
  automated.

Without an explicit dependency view, the reader cannot tell which stages can be
parallelized (e.g., I could start once G has produced initial events, without
waiting for H/J).

## Suggestion

Add a mermaid flowchart or an explicit "Dependencies" list to the top of the
Stages section showing predecessor/successor relationships. This surfaces the
critical path and makes parallel work opportunities visible.

## Feedback

(None yet — pending review)
