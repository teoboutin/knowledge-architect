---
kind: design
---
# The claim check of a commit message is read at the session's start and needed at each commit

## Summary

`primer@verify-before-relying` asks that every factual claim of a commit message be listed and
checked before the commit. The primer is read at the session's start; the act happens at each
commit, and only the development skill's commit step restates it, for a commit of Rust code: a
commit made outside that skill meets no text that restates it. Eleven of the twelve workflow files of one intake
report the rule acted on as if absent, in three projects, and twenty-one of the 23 of a later one,
nine of this repository's ten begun after the restatement among them.

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
  `primer@verify-before-relying`. The file places the line after the verdict;
  `path@check@README.md` keeps the verdict as the last line a person reads, so such a line goes
  above it.
- Further instances, in the intake of forty retrospective files of 2026-10-31 to 2026-11-07:
  twenty-one of its 23 workflow files report the lapse, 15 of this repository's 17 and all 6 of
  thaum's, at least 54 instances, each caught before the merge. By where the false claim was
  written: commit messages in most files; answers to the owner, 4 instances in 4 files; heads, in
  3 files; issue entries, plan documents, a spec, code comments and a report file in the others.
  Four of the instances are attributions of the owner, counted again in
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`.
  The two files with none are those of the 0.9.0 release and of the slice that put `cli` on top.
  The session of the intake of seventeen files made four of them: a count over the history in a
  shallow clone said to find nothing, written into two agents, an issue and an analysis; the
  analyses' commit saying the unread entries were stated as each analysis's head says, where one
  head said so; and two messages counting as 0 instructions an edit that added one. The analysis
  is the section "The standing answers, with the claim-check lapses of twenty-one files" of
  2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in the commit "[intake]
  The analyses of forty received retrospective files, three outcomes for the owner".
- The count after the restatement in the development skill's commit step: of this repository's
  ten files whose session began after that restatement shipped, nine report the lapse; the one
  without is that of the slice that put `cli` on top. At least one lapse was in a message about
  Rust work, where the restatement is delivered: the slice that built the `model` crate, whose
  message said each public item was found by the compiler's errors. Whether that session loaded
  the development skill at the commit is an assumption: its file does not say. The six thaum files
  used 0.9.0, which ships no development skill.

The suspected mechanism, in one sentence: an instruction read once at the start of a long session
is not recalled at the moment it binds, which recurs at every commit.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, raised when work next
changes what a session reads or runs at a commit, or earlier on the owner's word.

That re-entry point was met once: slice 2 of the milestone on the development skill and the
Components discussed it in its thread on the commit's claim check, chose the restatement in the
development skill's commit step, in the commit "[agent-skills] The installed development skill,
its review agents, and the local skill cut to an overlay", and kept this entry open for the other
candidates below. No later change to the installed text's source or its fragments touches what a
session reads at a commit: `git log -G` over them, from the commit "[review] Repairs of the review
before the merge of slice 2, and D3 ruled" to the commit "[intake] The owner's rulings on the three
pending outcomes, and the thaum rule widened to every document", for "before each commit", "after
each commit" and "--staged", returns no commit. The next such change meets the
re-entry point again.

The same mechanism, for the rule against a commit or a push chained after a failing check, is
`issue@agent-skills@a-command-chained-after-a-failing-check-recurs`, whose re-entry point is this
discussion, since a delivery at the commit, a hook among them, may serve both.

### Why it matters

Each lapse costs an amend or a history edit, and a claim that survives misleads every later reader
of the history, against `goal@knowledge-architect@agents-work-without-drift`. Under the lapse rule
of `skill@knowledge-architect-agent-configuration@admission-standard`, a lapse that recurs at one
point of delivery is repaired by moving where the instruction is delivered.

### What would close it

The discussion picks a delivery at the commit, or rules that none is worth its cost: a line printed
by `check --staged` on its verdict, which changes `design@check@phases-gate-the-report`'s summary; a
sentence of the primer beside the staged check, read at the start as now; a line printed by
`commits` above its verdict, after the commit, where an amend still repairs the message, which
changes `design@check@a-commit-message-is-a-document`'s summary; a project's commit hook,
which the installed text cannot rely on. Then the retrospectives of a release show whether the
count falls.
