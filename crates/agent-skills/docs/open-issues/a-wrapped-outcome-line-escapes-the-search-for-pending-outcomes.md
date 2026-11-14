---
kind: defect
---
# A wrapped `Outcome` line escapes the search for pending outcomes before an intake's merge

## Summary

Before the branch that holds an intake's analyses merges, the installed intake lists the outcomes
still pending with one `grep` that reads a single line. An analysis wraps its prose, and nothing
says the `Outcome` line stays on one line, so a pending outcome whose line breaks before
"pending" is not listed, and the branch could merge with it unruled.

## Details

### What

- `skill@knowledge-architect-retrospective-intake@after-the-ruling`: "list every `Outcome` line
  still pending, with `grep -rn -E 'Outcome.*for the owner \(.*\): pending' docs/retrospective-reports/`".
- `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` gives the line's two forms
  and no rule on its length.
- The instance: the intake of forty-seven files, 2026-11-14. Of its five pending outcomes the
  search listed four; the line of the fifth, in the standing section of the finding-repairs workflow
  file's analysis, broke before "pending". The agent that wrote the analyses commit counted the
  `Outcome` lines by another command and found the gap; the line was rejoined by "[intake] An
  Outcome line pending for the owner is rejoined on one line, so a search for pending outcomes finds
  it".

The shapes:

1. `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` says the `Outcome` line is
   written on one line, for the search;
2. the search reads a section's whole `Outcome` paragraph, or matches `for the owner` alone and
   reads each hit;
3. the checker, or a command of `query`, lists the pending outcomes of the analyses.

### Why it matters

An owner item missed by the search is merged unruled, against the intake's own rule that the
branch merges once none is pending.

### What would close it

A shape chosen under `skill@knowledge-architect-design` and written in the installed text, or
built, with the instance above as its case.
