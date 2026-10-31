---
kind: design
---
# The claim check of a commit message is read at the session's start and needed at each commit

## Summary

`primer@verify-before-relying` asks that every factual claim of a commit message be listed and
checked before the commit. The primer is read at the session's start; the act happens at each
commit, and no text read at a commit restates it. Eleven of the twelve workflow files of one intake
report the rule acted on as if absent, in three projects.

## Details

### What

- The rule, "Verify a claim before writing it": "Before each commit, list every factual claim its
  message makes … run or open each against the tree the commit holds … Writing the message from
  memory of the work is where the false claims come from." Its sentence on answers to the owner is
  reported lapsed the same way.
- The instances: the retrospective intake of 2026-10-24 received twelve workflow files, four of
  this repository and eight of two projects that install the workflow. Eleven report claims in
  commit messages, or in answers to the owner, that were false when written: counts, search
  results, statements that a text says something, attributions of a delivery to the owner. Each was
  caught before the merge, by the session or a reviewer, and repaired by an amend or a history
  edit, but one, which reached a main branch and became true after its commit. The analyses are in
  the commit "[intake] The analyses of sixteen received retrospective files, with the owner's
  ruling on each", under `path@knowledge-architect@docs/retrospective-reports/`.
- `primer@checker-model` asks for `cargo klarch check --staged` "after staging and before each
  commit", the one command every session runs at that moment; it judges the tree, not the claims.
- Further instances, in the intake of seventeen retrospective files of 2026-10-25 to 2026-10-31: six of this repository's eight workflow files of sessions of
  2026-10-25 to 2026-10-31 report the lapse, those of the design-home split, the graded entries and
  `klarch id`, `klarch worklist`, `klarch transcript`, the tripwire standard and the primer's terms
  section; three of them include a false answer to the owner. The analyses are in the commit
  "[intake] The analyses of seventeen received retrospective files, every outcome decided by the
  session".
- A candidate the retrospective of the session that built `klarch transcript` proposes: `cargo
  klarch commits`, which the primer asks for after each commit, prints one line naming what it did
  not judge, a count, a search, a quotation or an attribution in a message, with a pointer to
  `primer@verify-before-relying`. `path@core@README.md` keeps the verdict as the last line a
  person reads, so such a line goes above it.

The suspected mechanism, in one sentence: an instruction read once at the start of a long session
is not recalled at the moment it binds, which recurs at every commit.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, raised when work next
changes what a session reads or runs at a commit, or earlier on the owner's word.

### Why it matters

Each lapse costs an amend or a history edit, and a claim that survives misleads every later reader
of the history, against `goal@knowledge-architect@agents-work-without-drift`. Under the lapse rule
of `skill@knowledge-architect-agent-configuration@admission-standard`, a lapse that recurs at one
point of delivery is repaired by moving where the instruction is delivered.

### What would close it

The discussion picks a delivery at the commit, or rules that none is worth its cost: a line printed
by `check --staged` on its verdict, which changes `design@core@phases-gate-the-report`'s summary; a
sentence of the primer beside the staged check, read at the start as now; a line printed by
`commits` above its verdict, after the commit, where an amend still repairs the message, which
changes `design@core@a-commit-message-is-a-document`'s summary; a project's commit hook,
which the installed text cannot rely on. Then the retrospectives of a release show whether the
count falls.
