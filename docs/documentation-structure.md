# Documentation Structure Guide
## Keeping Documents Clean While Preserving the Research Trail

---

## Scope

This guide is the repository policy for documentation under `docs/`. Supporting
material for each document lives under `docs/_resources/`. The single-file
appendix described at the end is a lightweight alternative only for documents
that are **not** under the suggestion-driven workflow.

---

## The Core Problem

You want the **document itself to stay readable**, while keeping a full **audit
trail of resources** accessible but out of the way.

The risk: mixing resources into the document creates confusion for readers. But
discarding them loses the audit history — valuable for investigating and
improving workflows later.

**Solution: two-layer approach.**

---

## Recommended Folder Structure

```
docs/
│
├── document.md                  ← The actual deliverable (clean, readable)
│
└── _resources/                  ← Everything that fed into it
    └── document/                ← Document-scoped namespace (one per deliverable)
        ├── INDEX.md             ← Master log: what was used, when, why
        ├── suggestions/         ← Actionable review feedback
        │   ├── S001_....md
        │   └── S002_....md
        ├── proposals/           ← Working copies updated before the deliverable
        │   ├── document-proposal-v1.md
        │   └── document-proposal-v2.md
        ├── research/
        │   ├── source-1.md
        │   └── source-2.pdf
        ├── reviews/
        │   ├── review-round-1.md
        │   └── feedback-alice.md
        └── drafts/
            ├── draft-v1.md
            └── draft-v2.md
```

The resource root is `docs/_resources/`; all supporting material lives under it,
present but clearly separate from the deliverable. Within it, each deliverable
gets its own namespace folder at `docs/_resources/{document-name}/`.

`{document-name}` is the collision-resistant key from the suggestion-driven
workflow: take the target file's path relative to `docs/`, drop the extension,
replace each `/` with `-`, then apply the slug normalization pipeline. For
example:

- `docs/contracts.md` → `docs/_resources/contracts/`
- `docs/api/contracts.md` → `docs/_resources/api-contracts/`
- `docs/legal/contracts.md` → `docs/_resources/legal-contracts/`

The last two share the basename `contracts.md` but resolve to distinct keys, so
their namespaces never collide. For the complete normalization pipeline and the
canonical rule, see [`suggestion-driven-documentation.md`](suggestion-driven-documentation.md).

### Required and optional folders

`suggestions/` and `proposals/` are workflow folders: they are governed by the
suggestion-driven lifecycle and are created on demand for any document under
review. `research/`, `reviews/`, and `drafts/` are optional — add them when a
document accumulates source material, review notes, or working drafts worth
preserving. Any resource worth keeping should be linked from `INDEX.md`;
anything not linked is treated as scratch.

---

## The Key: `INDEX.md` as the Bridge

This file solves the confusion problem. It's a lightweight log that *links* the
document to its resources without cluttering the document itself. Use the
variant that matches how the document is maintained.

### Research-driven documents

For documents assembled from research sources, the INDEX records what fed in:

```markdown
# Resource Index — [Document Name]

## Sources Used
| Resource          | Type     | Used For         | Section in Doc |
|-------------------|----------|------------------|----------------|
| source-1.md       | Research | Background stats | Introduction   |
| feedback-alice.md | Review   | Restructured §3  | Methods        |

## Decision Log
- Dropped source-2 — outdated (2019), superseded by source-1
- Alice's review prompted merging sections 3 and 4

## Open Questions
- [ ] Verify stat in §2 against primary source
```

### Suggestion-driven documents

For documents under the suggestion-driven workflow, the INDEX links the
suggestion and proposal artifacts:

```markdown
# Resource Index — [Document Name]

## Suggestions
- [S001 - Improve heading clarity](suggestions/S001_improve-heading-clarity_claude-sonnet-4-6-high.md)
- [S002 - Clarify scope](suggestions/S002_clarify-scope_codex-gpt-5-medium.md)

## Proposals
- [document-proposal-v1.md](proposals/document-proposal-v1.md)

## Decision Log
- Promoted proposal-v1 covering S001-S002.
```

The `## Suggestions` and `## Proposals` sections link to the artifacts only. The
decision log may record document-level rationale, but per-suggestion status
fields (`Decision`, `Reviewer`, `Implementation reference`) must remain only in
the suggestion documents — never copied into `INDEX.md`. See
[`suggestion-driven-documentation.md`](suggestion-driven-documentation.md) for
the canonical suggestion-driven INDEX schema.

The decision log is especially valuable for workflow review — it captures *why*
things were included or dropped, not just *what*.

---

## Suggestion and Proposal Workflow

Review feedback never edits the deliverable directly. It flows through the
document-scoped namespace before reaching the reader-facing file:

- **Suggestions** — actionable review feedback goes into
  `docs/_resources/{document-name}/suggestions/`, one document per suggestion.
- **Proposals** — proposal copies of the deliverable go into
  `docs/_resources/{document-name}/proposals/`, where suggestions are applied
  and reviewed first.
- **Controlled updates** — the deliverable is updated only after pending
  suggestions are accepted, refused, or deferred; an accepted suggestion is
  applied to a proposal copy, then the proposal is copied back over the target
  file.
- **Traceability** — each suggestion document is the sole source of truth for
  its own `Decision`, `Reviewer`, and `Implementation reference`. `INDEX.md` may
  group or link suggestion files for navigation, but it must not restate or
  duplicate those fields; read them only from the suggestion documents.

This gives reviewers and LLM tools a clear path from feedback to controlled
document updates.
[`suggestion-driven-documentation.md`](suggestion-driven-documentation.md) is
the normative source for the full suggestion and proposal lifecycle and
file-naming rules; if this guide and that one ever disagree, that document
governs.

---

## Principles

| Principle              | How it's achieved                              |
| ---------------------- | ---------------------------------------------- |
| Document stays clean   | Readers never see the scaffolding              |
| Trail is explicit      | INDEX links decision artifacts, not just files |
| Investigation-friendly | Maintainers can reconstruct every choice       |
| Scalable               | Works for solo work and team collaboration     |

---

## Lighter Alternative (Single-File Documents)

This alternative applies only to documents that are **not** under the
suggestion-driven workflow. When a document is under that workflow, its
suggestion and proposal artifacts must still live in
`docs/_resources/{document-name}/...`, even if other research notes are kept in
a lightweight appendix.

If a document is maintained as a single file rather than within a resource
folder, add a collapsible appendix at the end using an HTML `<details>` block
(rendered as a collapsible section by GitHub and most Markdown viewers):

```markdown
# My Document

[...main content...]

---

<details>
<summary>Appendix: Research Trail (not part of the deliverable)</summary>

### Sources
- ...

### Review Notes
- ...

</details>
```

This keeps everything in one file while visually separating the deliverable from
the supporting material.

---

## Summary

> **The document answers *what*. The resource trail answers *how you got there*.**

Valuable for workflow improvement — but only surfaced when you need it.
