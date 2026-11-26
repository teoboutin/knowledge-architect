---
kind: observation
---
# A dispatched agent can hand back while agents it dispatched in the background still run

## Summary

A dispatched agent that dispatches agents of its own in the background can end its turn, and so
hand back to its dispatcher, while those agents still run. Two retrospectives of this repository
report five such hand-backs; after one of them the main session took two running auditors for
stopped and removed their worktrees. Where a background child's result goes once its dispatcher has
handed back is reproduced in the harness's own issue tracker: to the main session. It reads as a
defect of the harness rather than of the workflow's text, and dispatching such children in the
foreground is ruled out.

## Details

### What

- The record-search workflow's retrospective, 2026-11-16, W4: an implementer, the harvest agent and
  a repairer each dispatched auditors in the background, under briefs that said to wait for every
  agent dispatched before reporting, and each handed back before its auditors returned, saying the
  harness "made me hand back": four early hand-backs, each resumed by a message from the main
  session.
- The owner-rulings retrospective, 2026-11-22, W6: an implementer handed back after three of five
  steps, reporting two auditors "still running"; the main session found no result files, took them
  for stopped and removed both worktrees; both were still running, one finished in the main
  checkout, and their reports reached the main session directly.
- What the files report of the mechanism: a child dispatched in the background keeps running after
  its dispatcher's turn ends, and its result reaches the main session, not the dispatcher.
- The harness's issue tracker holds the mechanism, reproduced: anthropics/claude-code#108513, a
  background subagent whose own background subagent waits on background work is told by the
  harness's `[handback-send-enforce]` prompt to hand back, and the child's report then reaches the
  main session; and anthropics/claude-code#103605, a hand-back accepted while the subagent's own
  background children still run, whose comments name the trigger, the child's interim notice. The
  two occurrences above were added to the first as a comment on the owner's word at the intake of
  twenty files, with the removed worktrees as the cost the report did not hold.
- `primer@dispatching-work` already has an agent report "what it acted on and what it did not, with
  why", and the implementer of 2026-11-22 reported its two auditors as still running; the loss there
  was the main session's reading of that report.
- Ruled out at the intake of twenty files: an instruction that an agent dispatches in the
  foreground the agents whose results it needs. A foreground call blocks the dispatcher until the
  child returns, so the dispatcher cannot run a side investigation, or go on working, meanwhile;
  the instruction seemed to bring more problems than it solves. Several foreground calls sent in
  one message do run at once, as the Agent tool's own description states (not measured), so it does
  not prevent a parallel batch. An instruction aimed at dispatched agents alone also meets that an
  agent has no explicit signal of whether it was itself dispatched: it usually starts from a brief
  another agent wrote, and nothing more is known to tell it. The analysis of the record-search
  workflow's file, section child-agents-outlive-parent-hand-back, under
  `path@knowledge-architect@docs/retrospective-reports/`, held that rival.

### Why it matters

A hand-back with children still running gives the main session a report that is not final, and a
session that then removes their worktrees, or dispatches the same work again, loses or repeats it;
an instruction about how an agent behaves is added only on evidence from real use, per
`design@agent-skills@behaviour-additions-need-real-use`, and the owner reported that, as far as
the owner knows, no session misbehaved on it.

### What would close it

The harness's fix of anthropics/claude-code#108513, which this entry then closes with; or an
instruction that real use shows a session lacked meanwhile, admitted under that head.
