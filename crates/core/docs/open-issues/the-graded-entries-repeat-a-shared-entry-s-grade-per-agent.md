---
kind: todo
---
# The graded entries carry a shared entry's grade once per agent

## Summary

`worklist --verify` assembles `bears.md` and `may-bear.md` with every agent's grade line of each
entry, identical lines merged. An entry many clusters share, a goal first, carries one line per
agent that graded it, most of them saying the same thing in other words, and the file a session
reads whole grows with the number of agents.

## Details

### What

`assemble` in `path@core@src/cli/worklist.rs` writes the lines, per
`design@core@graded-entries-assembled`.

Instances:

- W4 and C2 of the retrospective of the intake deciding by authority, 2026-10-31: `bears.md` of 80
  entries, about 35,000 words, the session's largest read.
- the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31: `bears.md` of 200 entries held 83,788 words by `wc -w`; its 365 grade lines held 14,933
  words, the 25 goals 156 of the lines, and 6 entries 11 lines each.

### Why it matters

The design skill reads `bears.md` whole, and the intake reads the entries in it, so each repeated
line costs every grounding, against `design@agent-skills@context-cost-of-agent-text`'s measure of
what a session loads and does not apply.

### What would close it

The assembly writes one line per distinct reason, or per agent only where the reasons differ,
`design@core@graded-entries-assembled` rewritten through `skill@knowledge-architect-design`; the
counts above taken again on a search of the same size.
