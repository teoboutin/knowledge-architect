---
kind: todo
---
# A retrospective's version line names no harness and no version of it, which a finding on the harness's behaviour needs

## Summary

The version line of a retrospective's two files names the version of knowledge-architect and, in
the project's file, the project's main commit, per
`design@agent-skills@version-line-names-the-main-commit`; it names neither the harness the session
ran in nor that harness's version. A finding that turns on how the harness behaves, such as when a
dispatched agent hands back, can then not be matched to the harness's own reports, whose behaviour
changes between versions. The owner proposed at the intake of twenty files that the line name the
exact harness and its version.

## Details

### What

- The instruction: `path@agent-skills@content/skills/retrospective/SKILL.md`, "Then **the version
  the session used**, in one line", whose two bullets name knowledge-architect's version and the
  project's main commit.
- The measurement: of the 164 files under the owner's pending and archived retrospective
  directories on 2026-11-26, 3 name the harness's version, by
  `grep -l -i -E 'Claude Code [0-9]|harness ran'` over them.
- The case that showed the need: the two retrospectives behind
  `issue@agent-skills@a-dispatched-agent-can-hand-back-while-its-own-agents-run` gave no harness
  version on their version lines, so the comment added to the harness's issue
  anthropics/claude-code#108513 named the version one of them mentions in another finding, "2.1.321
  and later"; that issue ties the behaviour to a change of the harness's version 2.1.318.

### Why it matters

An intake checks a finding against the text at the version its file names; a finding on the
harness's behaviour has the harness's version as that text, and without it the intake cannot tell
a repaired behaviour from a standing one, nor report the finding where the harness's maintainers can
match it.

### What would close it

The retrospective skill's version line naming the harness and its exact version, as the session
can read them (for Claude Code, the `version` field its transcript records on each line), with the
intake reading it where a finding turns on the harness's behaviour; shaped under
`skill@knowledge-architect-design` on a branch of its own.
