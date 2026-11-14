---
kind: question
---
# Does a command find each test's recorded mutation whose quoted code has left the source?

## Summary

A test of logic records the mutation that showed it discriminates, quoting the mutated lines. When
the code changes, the record goes stale and nothing says so. One reviewer found two stale records by
a script of its own. Whether a command takes the task in is the owner's word; this entry holds the
instance until then.

## Details

### What

- The development skill, in its source `path@agent-skills@content/skills/development/SKILL.md`:
  "**A recorded check quotes** the `-` and `+` lines of the mutation, the file and the function or
  type that holds them, and the verdict". The code-shape reviewer,
  `path@agent-skills@content/agents/code-shape-reviewer.md`, reports "for a test of logic, its
  recorded mutation" where it is missing; nothing asks whether the quoted code still exists.
- No verb of `klarch` and no `cargo x` command reads recorded mutations: `klarch --help` and
  `ls tools/xtask/src/` list none, at the intake of 2026-11-14.

Instance: the session on the record search's verdicts in this repository, 2026-11-10, C2 of its
workflow file. The code-shape reviewer wrote a script of about twenty lines that gathers each test's
doc comment recording a mutation, extracts each fragment quoted before "made", and reports those no
longer in the source; it found two records whose quoted code had changed since the mutation ran,
one stating an outcome the code no longer produced. The file proposes a command listing each
recorded mutation whose quoted fragment no longer appears in its file, which the code-shape
reviewer points to.

The rivals: a sentence in the code-shape reviewer's definition asking it to check each record's
quoted lines against the source, with no command; or no change, on one instance.

The placement is open: under `design@knowledge-architect@mutation-command-host`, a tool whose
result exists without running the tests is no command of the mutation crate, and the rule that
places every other tool in `klarch` is argued by no head, per
`issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`.

The question: does the owner take the task in? `goal@tasks-a-tool-does-better-are-the-tool-s`
reserves it: "A task is taken in by a decision of knowledge-architect's owner, never because the
workflow asks it." `design@mutate@mutation-command` takes in the run of a mutation, not a reading of
the records.

### Why it matters

A stale record states an evidence of a test that the code no longer gives, so the code-shape
reviewer, which `design@agent-skills@code-shape-axis` and `design@agent-skills@test-economy` place
over a test's recorded mutation, judges a test as shown that is not, against
`goal@the-workflow-leads-to-better-code`. The result follows from the tree with no judgement, which
`goal@tasks-a-tool-does-better-are-the-tool-s` owes a tool once taken in.

### What would close it

The owner's word. Taken in: this entry becomes a `todo`, its placement argued with the rule of one
program. Not taken in: it is deleted, with that word in the commit.
