---
kind: todo
---
# The agent-configuration skill names no place for an instruction local to one machine and never committed

## Summary

`skill@knowledge-architect-agent-configuration@where-text-goes` sorts agent-facing text between the
root `CLAUDE.md`, a scoped `CLAUDE.md`, a skill and a subagent definition, every one committed. It
names no place for an instruction that holds on one machine only and must not be committed. The
harness offers one, a `CLAUDE.local.md` at the project's root, which it loads at launch beside
`CLAUDE.md`, and which the project keeps out of git.

## Details

### What

The case met: on the owner's machine, Remote Control sessions receive a system-prompt block
written for cloud sessions, which says the `gh` CLI is unavailable; a session followed it, skipped
the draft pull request the root `CLAUDE.md` asks for at a branch's first push, and told the owner it
could not open one, while `gh` was installed and logged in. The cause is a harness defect,
reported upstream as issue 100759 of the anthropics/claude-code repository. The owner asked for
a counter-instruction local to the machine, and it was written as a gitignored `CLAUDE.local.md`
at the root of this repository.

Read in the Claude Code documentation, page "How Claude remembers your project": "For private
per-project preferences that shouldn't be checked into version control, create a `CLAUDE.local.md`
at the project root. It loads alongside `CLAUDE.md` and is treated the same way. Add
`CLAUDE.local.md` to your `.gitignore` so it isn't committed." The same page says a gitignored
`CLAUDE.local.md` "only exists in the worktree where you created it", so a session run in another
worktree does not load it.

Not established: whether a session or a subagent that the harness starts in a worktree of its own
loads the file of the main checkout, and whether the checker's walk skips it in every project, as
it does here through `.gitignore`.

### Why it matters

An adopting project's owner who needs an instruction for one machine, such as a correction of the
harness's prompt, a local path or a tool that only that machine has, finds no row of the skill to
place it, and the primer's last row then sends the session to ask the owner each time, or the
session commits it into the root `CLAUDE.md`, where it reaches every other machine.
Maintaining the agent configuration is an activity
`goal@agents-get-a-complete-workflow` lists, which an agent carries out with
the workflow and what its project adds: for this case of it, the skill gives no place.

### What would close it

A design discussion under `skill@knowledge-architect-design` on whether the skill's tables name
the local file: what belongs there (an instruction true of one machine, never a decision or a rule
of the project), that it is gitignored, how the checker's walk treats it, and what a worktree
loses. The evidence of real use is the case above, the owner's request. Then the edit, or the
owner's ruling that the case stays to judgement, recorded in the commit that deletes this entry.
