# Resource Index - Suggestion-Driven Documentation

## Suggestions

- [S001 - Define resource root](suggestions/S001_define-resource-root_codex-gpt-5-medium.md)
- [S002 - Avoid target name collisions](suggestions/S002_avoid-target-name-collisions_codex-gpt-5-medium.md)
- [S003 - Constrain batched suggestions](suggestions/S003_constrain-batched-suggestions_codex-gpt-5-medium.md)
- [S004 - Define completion semantics](suggestions/S004_define-completion-semantics_codex-gpt-5-medium.md)
- [S005 - Close application commit flow](suggestions/S005_close-application-commit-flow_codex-gpt-5-medium.md)
- [S006 - Normalize resource path prefixes](suggestions/S006_normalize-resource-path-prefixes_claude-opus-4-8-high.md)
- [S007 - Document the Feedback section in the template](suggestions/S007_document-feedback-section_claude-opus-4-8-high.md)
- [S008 - Integrate INDEX.md into the lifecycle](suggestions/S008_integrate-index-into-lifecycle_claude-opus-4-8-high.md)
- [S009 - Define or replace the engineering director role](suggestions/S009_define-engineering-director-role_claude-opus-4-8-high.md)
- [S010 - Name the applier role in the Applying workflow](suggestions/S010_name-applier-role_claude-opus-4-8-high.md)
- [S011 - Define suggestion numbering under concurrent authorship](suggestions/S011_concurrent-suggestion-numbering_claude-opus-4-8-high.md)
- [S012 - Govern subsequent review rounds on a revised document](suggestions/S012_govern-multi-round-reviews_claude-opus-4-8-high.md)
- [S013 - Specify and summarize the Decision state machine](suggestions/S013_specify-decision-state-machine_claude-opus-4-8-high.md)
- [S014 - Define the INDEX.md schema and reconcile it with the companion guide](suggestions/S014_define-index-schema_claude-opus-4-8-high.md)
- [S015 - Enumerate allowed effort levels](suggestions/S015_enumerate-effort-levels_claude-opus-4-8-high.md)
- [S016 - Complete the slug normalization rules](suggestions/S016_complete-slug-normalization-rules_claude-opus-4-8-high.md)
- [S017 - Define the proposal extension and promotion target path](suggestions/S017_define-extension-and-promotion-path_claude-opus-4-8-high.md)
- [S018 - Add an end-to-end flow overview](suggestions/S018_add-end-to-end-overview_claude-opus-4-8-high.md)
- [S019 - Bind the template heading number and title to the filename](suggestions/S019_bind-template-number-to-filename_claude-opus-4-8-high.md)
- [S020 - Unify the terminology for the reviewed document](suggestions/S020_unify-reviewed-document-terminology_claude-opus-4-8-high.md)
- [S021 - Refresh the version and date header](suggestions/S021_refresh-version-and-date-header_claude-opus-4-8-high.md)
- [S022 - Add headings to the reference block](suggestions/S022_add-reference-section-headings_claude-opus-4-8-high.md)
- [S023 - Bootstrap resource artifacts explicitly](suggestions/S023_bootstrap-resource-artifacts_codex-gpt-5-medium.md)
- [S024 - Stop claiming key derivation is reversible](suggestions/S024_stop-claiming-key-derivation-is-reversible_codex-gpt-5-medium.md)
- [S025 - Align promotion ownership](suggestions/S025_align-promotion-ownership_codex-gpt-5-medium.md)
- [S026 - Subsequent rounds lacks reviewing step for deferred](suggestions/S026_subsequent-rounds-lacks-reviewing-step_claude-haiku-4-5-high.md)

## Proposals

- [suggestion-driven-documentation-proposal-v1.md](proposals/suggestion-driven-documentation-proposal-v1.md)
- [suggestion-driven-documentation-proposal-v2.md](proposals/suggestion-driven-documentation-proposal-v2.md)
- [suggestion-driven-documentation-proposal-v3.md](proposals/suggestion-driven-documentation-proposal-v3.md)

## Decision Log

- Created `suggestion-driven-documentation-proposal-v1.md` as the first proposal
  copy for accepted suggestions S006-S009.
- Promoted the proposal back to `docs/suggestion-driven-documentation.md`.
- Created `suggestion-driven-documentation-proposal-v2.md` for accepted
  suggestions S010, S012-S022 (S011 deferred). Applied: Overview section (S018),
  Storage/File naming/Suggestion document structure headings (S022), slug
  normalization pipeline (S016), effort-level enumeration (S015), template heading
  binding (S019), Decision state-machine table (S013), INDEX.md schema subsection
  (S014), unified "target file" terminology (S020), named applier role (S010),
  extension and promotion-path definition (S017), version bump to 0.2 (S021),
  Subsequent rounds section (S012).
- Promoted proposal-v2 to `docs/suggestion-driven-documentation.md`.
- Created `suggestion-driven-documentation-proposal-v3.md` for accepted
  suggestions S023, S024, S025. Applied: bootstrap rule for resource artifacts
  in Creating suggestions (S023), removed the reversible-key-derivation claim in
  favor of reading the target path from each suggestion's `File` field (S024),
  and made promotion LLM-owned in the Overview (S025). S026 remains pending.
- Promoted proposal-v3 to `docs/suggestion-driven-documentation.md`.
