---
kind: todo
---
# No command gathers the record searchers' closing sections, and a dispatcher missed sixteen

## Summary

A dispatcher of record searchers carries each report's `## Decided` and `## Met outside the task`
into its own sections. Those sections are held only in the reports' files; `worklist --verify`
checks that each report has them and writes none of their content. When the harness cut the
searchers' returns, one session read the verdict files and never read sixteen reports' closing
sections, two items of which bore on the work.

## Details

### What

- The record searcher's description, `path@agent-skills@content/agents/record-searcher.md`: "Read
  the verdict files yourself ... and carry each report's `## Decided` and `## Met outside the task`
  into your own sections", per `design@agent-skills@session-acts-on-the-verdicts`.
- `judge_report` in `path@query@src/worklist.rs` refuses a report missing either section; a passing
  `--verify` writes the verdict files and `generated-lines.md`, or at an audit `gaps.md` and
  `fit-lines.md`, per `design@query@occasion-shaped-output`, and no file of the closing sections.

Instances:

- the session on the searcher's check and the reviewers' reports, 2026-11-11, W3 and C1 of its
  workflow file: in its second search, the 18 searchers' returns reached the session cut short by
  the harness after the first two. The session ran `--verify`, read the generated lines and the
  verdict files, and never read the closing sections of reports 3 to 18. Two items in them bore on
  the work, a path question and a defect of the installer; the transcript reviewer found both. The
  sections were later gathered by a script;
- the intake of forty-seven files, 2026-11-14: its session gathered the eighteen searchers' closing
  sections by reading each report as a message, which missed none, at the cost of the reading by
  hand. Reported by that session, not checked.

Alone, the first is a lapse of a clear instruction. That the harness cuts returns whenever many
agents return together is an assumption on two instances. The review skill solved the same loss
for reviewers by a report file each writes, `skill@knowledge-architect-review@review-invariants`.

The shapes:

1. a passing `--verify` also writes one file listing each report's two closing sections, by agent,
   which the description then names, C1's proposal; it widens
   `design@query@occasion-shaped-output` and what `--verify` writes under
   `design@query@worklist-writes-and-verifies-the-reports`;
2. the closing sections appended to an existing output, such as the head of `generated-lines.md`;
3. no change, the dispatcher reading each `report.md`'s two sections.

What decides between shapes 1 and 3 is whether the lapse recurs at this point of delivery.

### Why it matters

An item a searcher met outside its task, or a decision it took, that the dispatcher does not carry
is lost to the turn's sections, which `design@agent-skills@session-acts-on-the-verdicts` asks it to
fill; in the first instance only a later review found the two items.

### What would close it

A shape argued under `skill@knowledge-architect-design`, then built under `skill@klarch-development`
with a test of `--verify` over reports holding closing sections, that puts every report's
`## Decided` and `## Met outside the task` before the dispatching session; or the owner's ruling
that the reading stays by hand, recorded in the commit that deletes this entry.
