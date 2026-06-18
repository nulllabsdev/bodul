# S010 - Name the applier role in the Applying workflow

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | high                                                                                                       |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

The "Creating suggestions" and "Reviewing suggestions" sections name their actor
explicitly ("an LLM tool", line 113; "a reviewer", line 121), but "Applying
suggestions" (lines 134-150) is written as bare imperatives with no named
subject. It never says who creates the proposal, applies changes, or promotes
the proposal over the original. Compounding this, step 6 (lines 147-150) sets
`Decision` to `completed` from inside the applying workflow, even though lines
97-99 and 124 establish `Decision` as reviewer-owned — so an unnamed actor
transitions a reviewer-owned field. The engineering director (lines 108-109,
142) appears only as a question-answering authority, not as the applier.

## Suggestion

Open the "Applying suggestions" section with an actor lead-in parallel to the
other two sections (e.g. "When applying accepted suggestions, the applier:"),
and state whether the applier is the reviewer, the LLM tool, or a distinct role.
Explicitly say who performs the `accepted → completed` transition and reconcile
it with the reviewer-ownership statement at lines 97-99.


## Feedback

Also LLM tool. However, accepting is done by human ATM
