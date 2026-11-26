---
kind: todo
---
# No command searches a phrase across a line break and inline markup

## Summary

`primer@verify-before-relying` has a claim that a search returns nothing, or that a hit is the only
one in the tree, checked by a search that matches across a line break and across emphasis marks.
No command of knowledge-architect runs such a search, so each session writes its own, and a plain
`grep`, which matches within one line, passes vacuously on a wrapped phrase.

## Details

### What

Instances:

- C1 of the retrospective file `2026-11-16-knowledge-architect-doc-gate-and-lints-klarch-workflow`:
  the spec's claim that a rewritten rule left no "never an issue" behind was first checked by a
  plain `grep`, which three reviewers showed passed vacuously, both target sentences being wrapped;
  the session then wrote a `perl -0ne` search with `\s+` between the words. At the commit the
  session began from, `bb93531b`, the planning skill's source held the phrase as "never an" at the
  end of one line and "issue." at the start of the next, which a `perl -0` search over
  `git show bb93531b:crates/agent-skills/content/skills/planning/SKILL.md` finds and a `grep` does
  not.

The task follows from the tree with no judgement: a phrase, a set of paths or a revision, a match
with any run of whitespace, a line break included, between two words, and emphasis marks and
backticks ignored. Its output is each hit with its file and line.

Which program holds the command, and whether it is a command of its own or an option of an
existing one, is open: `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`
holds the placement rule unargued. It reads files and judges nothing, so placed in `query` it names
and runs no check, per `design@query@query-depends-on-no-check`. It is no member of
`issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work`, a ranked search of the
record's entries by keywords: this one matches a given phrase exactly over given paths.

### Why it matters

A claim of absence written on a vacuous search is a false claim the primer's rule exists to
prevent, and nothing but a reviewer who runs the search again finds it. The task recurs in every
session that writes such a claim, and a command gives each the same result, per
`goal@tasks-a-tool-does-better-are-the-tool-s`; the owner took the task in at the intake of
twenty files.

### What would close it

The command, or the option, built with a test over a fixture holding a phrase wrapped over two lines
and one split by emphasis marks, and the primer's sentence naming it.
