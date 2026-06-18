# S005 - Link Open Questions to Stages

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

The Open Questions section (lines 8–53) appears before the Stages section (lines
55–123) and refers to stages by letter in its headings — "Product matching
(Stage G)" and "Supporting multiple currencies (Stages H, J, K)". A reader
encounters "Stage G" and "Stages H, J, K" before those stages are defined later
in the document, with no way to jump to them. The reference is one-directional
and plain text, so the connection between an open question and the stage(s) it
blocks is stated but not navigable.

## Suggestion

Make the cross-references navigable:

- turn the stage letters in the Open Questions headings into anchor links to the
  corresponding stage subsections (e.g. `(Stage [G](#g-addupdate-products-inventory))`), and
- optionally add a back-link from each affected stage to its open question, so a
  reader in the Stages section knows an unresolved decision gates that stage.

This is a small navigation improvement; it complements
[S001](S001_make-stage-dependencies-explicit_claude-opus-4-8-medium.md), which
addresses the deeper structural point of documenting inter-stage dependencies.

## Feedback
