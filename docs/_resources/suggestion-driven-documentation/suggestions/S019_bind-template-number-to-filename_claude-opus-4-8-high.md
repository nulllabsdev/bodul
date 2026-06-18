# S019 - Bind the template heading number and title to the filename

| Field                    | Value                                                                                                      |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Priority                 | low                                                                                                        |
| File                     | `docs/suggestion-driven-documentation.md`                                                                  |
| Decision                 | completed                                                                                                  |
| Implementation reference | `docs/_resources/suggestion-driven-documentation/proposals/suggestion-driven-documentation-proposal-v2.md` |
| Created at               | 2026-06-18                                                                                                 |
| Author                   | Claude Code, claude-opus-4-8, high                                                                         |
| Reviewer                 |                                                                                                            |

## Issue

The template heading reads `# S012 - Improve heading clarity` (line 61) while
real numbering starts at `S001` (lines 42, 44), which can mislead readers into
thinking the heading number is arbitrary. More substantively, the relationship
between the three places a number and title appear — the filename
`S{number}_{title}_...`, the `# S{number} - Title` heading, and the prose title
— is never stated. A reader cannot tell whether the heading number must equal
the filename number, or whether the heading title must be the de-kebab'd form of
the filename `{title}`.

## Suggestion

State the binding explicitly, e.g. "The heading must read `# S{number} -
{Title}`, where `{number}` matches the filename's zero-padded number and
`{Title}` is the human-readable form of the filename `{title}`." Change the
template example from `S012` to `S001`, or add a note that the number is
illustrative, to avoid the mismatch with the documented starting number.
