---
kind: observation
---
# A dispatched agent can hand back while agents it dispatched in the background still run

## Summary

A dispatched agent that dispatches agents of its own in the background can end its turn, and so
hand back to its dispatcher, while those agents still run. Two retrospectives of this repository
report five such hand-backs; after one of them the main session took two running auditors for
stopped and removed their worktrees. Where a background child's result goes once its dispatcher has
handed back is reported, not measured. The owner judges it a defect of the harness, and ruled out
the fix of dispatching such children in the foreground.

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
- What the files report of the mechanism, not measured: a child dispatched in the background keeps
  running after its dispatcher's turn ends, and its result reaches the main session, not the
  dispatcher. The session's reading, an assumption: an agent's turn ends when it replies without a
  tool call, and an agent with background children and no tool that blocks until they return has
  nothing to do but end it.
- `primer@dispatching-work` already has an agent report "what it acted on and what it did not, with
  why", and the implementer of 2026-11-22 reported its two auditors as still running; the loss there
  was the main session's reading of that report.
- Ruled out, by the owner's argument at the intake of twenty files: an instruction that an agent
  dispatches in the foreground the agents whose results it needs, since a foreground call blocks the
  dispatcher and so prevents a side investigation while it works. The analysis of the
  record-search workflow's file, section child-agents-outlive-parent-hand-back, under
  `path@knowledge-architect@docs/retrospective-reports/`, held that rival.

### Why it matters

A hand-back with children still running gives the main session a report that is not final, and a
session that then removes their worktrees, or dispatches the same work again, loses or repeats it;
an instruction about how an agent behaves is added only on evidence from real use, per
`design@agent-skills@behaviour-additions-need-real-use`, and the owner reports that no session
misbehaved on it.

### What would close it

A measurement of the harness's mechanics, one dispatched agent that sends a child to the background
and hands back, showing where the child's result goes; then either a report to the harness's
maintainers, if it is the harness's defect, or an instruction that real use shows a session lacked,
admitted under that head.
