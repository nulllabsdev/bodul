# S006 - Mark Volatile Model Names as a Snapshot

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

Stage F (lines 91–94) names specific LLMs inline: "Using high capability LLM
tools for deep reasoning (Claude Opus 4.8 and GPT 5.5 ATM)". The "ATM" (at the
moment) hints these are current picks, but the specific model names are volatile
and will date quickly, while the roadmap is a longer-lived document. As written,
the sentence reads as if the named models are part of the design rather than an
illustrative present-day choice.

## Suggestion

Reframe the model names as an explicit, replaceable snapshot rather than baking
them into the stage description. For example: "Use a high-capability reasoning
LLM (currently Claude Opus 4.8 / GPT-5.5 — illustrative, expected to change) to
classify…". The intent of Stage F (high-capability model now, train a cheaper/
faster/local model later — lines 92–94) stays the same; only the dated specifics
are clearly marked as non-binding. (Note "GPT 5.5" also reads more
conventionally as "GPT-5.5".)

## Feedback
