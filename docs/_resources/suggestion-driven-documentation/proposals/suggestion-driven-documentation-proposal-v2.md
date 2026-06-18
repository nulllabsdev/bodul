# Suggestion-Driven Documentation

| Field   | Value      |
| ------- | ---------- |
| Version | 0.2        |
| Date    | 2026-06-18 |

Instead of making documentation changes on the fly during a pull request review,
ask LLM reviewers to write each actionable suggestion down in a suggestion
document. Review those documents afterward, decide which suggestions to
implement, and keep them as the record of what was proposed, completed, refused,
or deferred.

The default is one suggestion per document. A single document may contain
multiple suggestions only for small editorial-only batches such as typos,
formatting fixes, or similar non-behavioral wording changes. Batches must be
atomic: every item in a batch shares one `Priority`, `Decision`, and
`Implementation reference`. If review would give items in a batch different
decisions, split the mixed-decision batch into separate suggestion documents so
each decision is recorded accurately.

## Overview

Suggestion-driven documentation works in three phases. First, an LLM tool
reviews the target file and writes each finding as a suggestion document
(Creating). Next, a human reviewer reads each suggestion and records a decision
(Reviewing). Finally, an LLM tool applies accepted suggestions to a proposal
copy, and a human reviewer promotes the proposal to the target file (Applying).

## Storage

Store suggestions in `docs/_resources/{document-name}/suggestions/`. The
resource root is the repository's `docs/_resources/` directory; all suggestion
and proposal folders live underneath it.

`{document-name}` is a collision-resistant key derived from the target file's
path relative to `docs/`: drop the file extension and replace each `/` with `-`,
then apply SEO-style slug rules. Normalize as follows: lowercase the string;
transliterate or strip non-ASCII characters; replace any run of non-alphanumeric
characters with a single hyphen; trim leading and trailing hyphens; preserve
digits as-is. For example:

- `docs/contracts.md` → `contracts`
- `docs/api/contracts.md` → `api-contracts`
- `docs/legal/contracts.md` → `legal-contracts`

The last two files share the basename `contracts.md` but resolve to distinct
keys, so their suggestion folders never collide.

## File naming

Use the following filename format:

`S{number}_{title}_{tool-model-effort}.md`

Example: `S001_improve-heading-clarity_claude-sonnet-4-6-high.md`

- `number` must be sequential and zero-padded to three digits: `001`, `002`,
  `003`, and so on.
- Suggestion numbers are tracked per target file suggestion folder. For example,
  suggestions for `docs/contracts.md` go into
  `docs/_resources/contracts/suggestions/`.
- Suggestion numbers are never reused within the same target file suggestion
  folder. Each new suggestion gets the next available number in that folder, and
  gaps from refused or removed suggestions are acceptable.
- `title` must be a short lowercase kebab-case summary, for example
  `improve-heading-clarity`.
- `tool-model-effort` must be a lowercase hyphenated identifier for the
  authoring tool, model, and effort level. `effort` must be one of `low`,
  `medium`, or `high`, using the same vocabulary as `Priority`. For tools that
  have no effort concept, omit the effort segment and end the identifier with
  the model name. For example: `claude-sonnet-4-6-high` or `codex-gpt-5-medium`.

## Suggestion document structure

Each suggestion document should use this structure:

```md
# S001 - Improve heading clarity

| Field                    | Value                                |
|--------------------------|--------------------------------------|
| Priority                 | medium                               |
| File                     | `docs/example.md`                    |
| Decision                 | pending                              |
| Implementation reference |                                      |
| Created at               | 2026-04-12                           |
| Author                   | Claude Code, claude-sonnet-4-6, high |
| Reviewer                 |                                      |

## Issue
Briefly describe the problem being pointed out.

## Suggestion
Describe the proposed change clearly and concretely.

## Feedback
Reviewer notes about the decision, especially for refused or deferred
suggestions.
```

The heading must read `# S{number} - {Title}`, where `{number}` matches the
filename's zero-padded number and `{Title}` is the human-readable form of the
filename `{title}` segment (de-kebab-cased, title-cased).

- `Priority` records importance, such as `low`, `medium`, or `high`.
- `File` records the target file being reviewed, not the proposal copy.
- `Decision` records the current state of the suggestion: `pending`, `accepted`,
  `completed`, `refused`, or `deferred`. `completed` means the change has been
  applied to the target file, not merely to a proposal copy. A suggestion that
  has been applied to a proposal but whose proposal has not yet been promoted to
  the target file stays `accepted`.

| State       | Set by   | When                             | May transition to                 |
| ----------- | -------- | -------------------------------- | --------------------------------- |
| `pending`   | Author   | Suggestion created               | `accepted`, `refused`, `deferred` |
| `accepted`  | Reviewer | Decision to implement            | `completed`                       |
| `completed` | Applier  | Proposal promoted to target file | —                                 |
| `refused`   | Reviewer | Decision not to implement        | `accepted` (edge case)            |
| `deferred`  | Reviewer | Decision postponed               | `accepted` (edge case)            |

- `Implementation reference` records the proposal artifact where the accepted
  suggestion was applied, such as
  `docs/_resources/{document-name}/proposals/{document-name}-proposal-v1.md`.
- `Created at` records when the suggestion was written.
- `Author` records who created the suggestion. For tools, include the tool name,
  model, and effort level.
- `Reviewer` records who reviewed the suggestion. Leave it blank while
  `Decision` is `pending`. Populate it when `Decision` becomes `accepted`,
  `completed`, `refused`, or `deferred`.
- `Feedback` is an optional reviewer-owned section. Use it to record decision
  rationale, especially when a suggestion is `refused` or `deferred`.
- `INDEX.md` links to suggestion documents for navigation. Suggestion documents
  remain the source of truth for their own `Decision`, `Reviewer`, and
  `Implementation reference`. See
  [`documentation-structure.md`](documentation-structure.md) for the resource
  index structure.

### INDEX.md contents

An INDEX.md for a suggestion-driven document must contain:

- `## Suggestions` — a link list of all suggestion documents in the folder.
- `## Proposals` — a link list of all proposal files.
- `## Decision Log` (optional) — narrative notes on accepted, refused, or
  deferred decisions, without duplicating `Decision`, `Reviewer`, or
  `Implementation reference` fields from the suggestion documents.

Note: the companion `documentation-structure.md` describes a different INDEX
schema ("Sources Used / Open Questions") for general research-driven documents.
Only the schema above applies to suggestion-driven workflows.

The engineering director is the human authority for questions that arise while
applying accepted suggestions.

## Creating suggestions

When an LLM tool is given a file to review:

1. Review the target file and write suggestions into
   `docs/_resources/{document-name}/suggestions/`.
2. Commit the suggestion documents.

## Reviewing suggestions

Before applying suggestions, a reviewer:

1. Reads each suggestion with `Decision` set to `pending`.
2. Sets `Decision` to `accepted`, `refused`, or `deferred`.
3. Populates `Reviewer`.
4. Writes `Feedback` when the decision needs rationale.
5. Updates `docs/_resources/{document-name}/INDEX.md` so it links to the
   suggestion document without duplicating its status fields.
6. Leaves `Implementation reference` blank until an accepted suggestion is
   applied to a proposal artifact.

Only accepted suggestions move into the applying workflow.

## Applying suggestions

When applying accepted suggestions, the applier (LLM tool):

1. Creates a copy in `docs/_resources/{document-name}/proposals/` named
   `{document-name}-proposal-v{number}.{extension}`, where `{extension}` is
   the extension of the target file (e.g. `.md`). Suggestion documents are
   always `.md` regardless of the target file's extension. Version numbers
   start at `1`. If proposal versions already exist for that file, use the
   next number after the highest existing version.
2. Commits the proposal copy as-is.
3. Reads all suggestion documents with `Decision` set to `accepted`.
4. If there are any questions for the engineering director, asks them.
5. For each accepted suggestion, applies the change to the proposal copy and
   sets `Implementation reference` to the proposal artifact. Leaves `Decision`
   as `accepted`; the suggestion is not `completed` until the change reaches
   the target file.
6. Once every accepted suggestion targeting a proposal has been resolved,
   copies the proposal over the target file at its original path (reversing
   the `{document-name}` key derivation), updates
   `docs/_resources/{document-name}/INDEX.md`, then sets `Decision` to
   `completed` for each suggestion that was applied. The `completed` transition
   is performed by the applier as part of this step.

## Subsequent rounds

Each new review round always reviews the current target file, not a proposal
copy. Prior `completed` and `refused` suggestions are out of scope for the new
round; re-raise them as new suggestion documents if they need to be revisited.
Suggestions currently `deferred` may be moved to `accepted` by the reviewer
before applying begins, or re-raised as new suggestions.
