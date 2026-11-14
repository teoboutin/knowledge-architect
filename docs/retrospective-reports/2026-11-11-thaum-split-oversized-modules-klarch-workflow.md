# 2026-11-11-thaum-split-oversized-modules-klarch-workflow

- **Received file**: 2026-11-11-thaum-split-oversized-modules-klarch-workflow.md, the workflow file of a retrospective of a project that installs the workflow.
- **Version used**: "knowledge-architect pinned at `=0.9.0`."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@upstream-tool-loop`: constrains, respect.
  - `design@agent-skills@move-lands-in-move-only-commits`: constrains, respect.
  - `design@check@owned-namespace-check`: conflicts, check-reversal.
  - `design@check@generated-files-are-pure`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `issue@knowledge-architect@no-head-argues-what-becomes-an-ask`: touches, respect.
  - `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`: constrains, respect.
  - `design@model@one-list-of-generated-files`: constrains, respect.
  - `goal@every-target-project-can-adopt-it`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `design@git@index-file-variable-kept`: constrains, respect.
  - `design@agent-skills@checker-model-in-primer`: constrains, respect.
  - `design@agent-skills@staged-forms-for-a-partial-commit`: constrains, respect.
  - `design@check@finding-names-the-repair`: constrains, respect.
  - `design@check@one-question-one-command`: conflicts, check-reversal.
  - `design@check@index-staged-write`: conflicts, check-reversal.
  - `design@check@fix-refusal-mixed-state`: constrains, respect.
  - `rejected@check@index-diff-with-write-flag`: constrains, respect.
  - `rejected@check@index-staged-writes-working-file`: brings-back, new-argument-needed.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `design@check@an-extension-plugs-in-through-phased-hooks`: constrains, respect.
  - `issue@check@an-extension-cannot-see-which-register-a-document-belongs-to`: touches, respect.
  - `design@model@one-entity-table`: conflicts, check-reversal.
  - `issue@model@a-rust-item-named-in-prose-has-no-checked-form`: touches, respect.
  - `goal@projects-add-their-own-checks`: constrains, respect.
  - `goal@projects-are-split-into-modular-units`: constrains, respect.
  - `rejected@agent-skills@may-bear-on-a-doubt-naming-nothing`: constrains, respect.
  - `design@agent-skills@synthetic-evidence-not-built`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.

## searcher-role-relations-unstated: the report skeleton named no relation each role admits

Members: 2026-11-11-thaum-split-oversized-modules-klarch-workflow W1; 2026-11-12-thaum-triggers-design-klarch-workflow standing answer ("A primer rule acted on as if absent": four of 14 searchers gave a relation the role does not admit).

- **a. What it says.** In a project that pins the checker at `=0.9.0`, 7 of 14 searchers graded a tripwire or a `rejected` entry `bears` with a relation its role does not admit, and `--verify` refused 12 lines ("`may change` is no relation for the role `tripwire`: give one of fires"). It blames the skeleton's header, "Grade each entry: `bears` with a relation, `may-bear`, or `unrelated`." The fix: each skeleton line names the relations its role admits (0 instructions added). The second file of the same project and pin reports four of 14 searchers refused the same way.
- **b. Whether it still applies.** No. At the tag v0.9.0 the header is the blamed one: `git grep -n "Grade each entry" v0.9.0 -- crates` returns line 235 of `path@elsewhere@crates/core/src/cli/worklist.rs`, "Grade each entry: `bears` with a relation, `may-bear`, or `unrelated`". At v0.10.0 and at origin/main the same search returns nothing. The skeleton was rewritten by d0ba6a45 (`v0.9.0-241`), "[harvest] The record search packed by search unit, its verdicts, the session acting on them, the draft searched and the audit's fit recorded", which also deleted the issue that recorded this defect, `the-report-skeleton-names-no-relation-its-entry-s-role-admits`, under the `query` anchor. Since then each skeleton line's `verdict:` field lists the verdicts the entry's register admits: the doc comment of `render` in `path@query@src/worklist.rs`, near line 731, "whose `verdict:` field lists the verdicts the entry's register admits", and `agent@knowledge-architect-record-searcher@fill-the-report`, "the skeleton lists the verdicts the entry's register admits; replace them by one". `design@query@worklist-writes-and-verifies-the-reports` records the reason and the measurement ("13 of 45 searchers wrote a relation their entry's role did not admit"). The tag v0.10.0 contains d0ba6a45; both files ran 0.9.0, the release before it.

Default: no change.
Outcome: decided by the session: no change, already repaired. Ground: d0ba6a45, released in 0.10.0, makes every skeleton line list the verdicts its register admits, which is the fix the file proposes.

## generated-file-cherry-pick-conflicts: every cherry-pick of a commit that regenerated a generated file conflicts in it

Members: 2026-11-11-thaum-split-oversized-modules-klarch-workflow C5.

- **a. What it says.** In thaum, on 0.9.0, "The fifteen commits, made on separate branches, each
  regenerated the rule index; applying them in sequence conflicted in that file at nearly every
  commit. The session wrote a script: take either side, run `cargo klarch index`, stage, continue."
  Proposal: "document, or ship through `install-agent-skills`, a `.gitattributes` entry and a merge
  driver that regenerates the generated files, since their content follows from the tree with no
  judgement".
- **b. Whether it still applies.** Yes. At `v0.10.0-59-g605d1538` no installed text says how a
  conflict in a generated file is resolved: a search of `path@agent-skills@content/` for "conflict"
  beside "index" or "generated" returns nothing. This repository's root `CLAUDE.md` states it for
  itself alone, in `instructions@git-workflow` point 3: "A rebase that stops on a conflict in a
  generated `index.md` is resolved by regenerating it: `cargo klarch index`, then `git add` on the
  file."
- **c. Whether it is already recorded.** No issue holds it. The verdicts bound a fix:
  - `design@check@owned-namespace-check` (`conflicts`): "The install edits nothing outside the
    namespace", so a `.gitattributes` entry shipped by the install is refused;
  - `design@check@generated-files-are-pure` and `design@model@one-list-of-generated-files`: a
    regeneration is sound because a generated file is a function of the tree, and any list of the
    files a driver or a command touches is the one list;
  - `design@agent-skills@move-lands-in-move-only-commits`: each move commit passes its checks, so
    each regenerates the index, which is why the conflicts are expected;
  - `goal@tasks-a-tool-does-better-are-the-tool-s`, with the searcher's owner question, whether the
    owner takes the task in; the record's practice for such a task is an issue whose closing
    condition is that decision, as the entries on the review's worktrees and on renames hold.
- **d. Its kind.** Missing: a task with no judgement, carried by a script, which recurs wherever a
  branch's commits are replayed (cherry-pick or rebase), as the root's own rule for a rebase shows.
- **e. Its scope.** The checker's `index` command and the installed text;
  `goal@tasks-a-tool-does-better-are-the-tool-s`.
- **f. A better fix.** A merge driver is given the three versions of one file; whether the rest of
  the tree the replayed commit will hold is in place when git runs it is not established, so a
  driver that runs the generator rests on an assumption. It is also per-clone configuration, the reason `rejected@check@commit-msg-hook` records against a hook. The nearer
  rivals: a command that resolves a stopped replay whose only conflicts are in generated files
  (regenerate from the tree, stage), or one sentence of installed text where the checker's model is
  taught, as this repository's root states for a rebase. Which, and whether at all, is the owner's
  taking in of the task, then a design discussion.
- **g. Its route.** An issue only; the work needs a branch of its own. Its anchor is the root's:
  the move may land in the checker or in the installed text, which the discussion decides, as for
  the issues on renames and on the review's worktrees, both in the root's issue directory.
- **h. The proposal.** Open an issue in the root, `knowledge-architect`, kind `todo`, slug
  `issue@knowledge-architect@a-conflict-in-a-generated-file-is-resolved-by-hand`: the instance; the root's rule as the same
  task in this repository; the bounds above; "What would close it": the owner takes the task in or
  declines it; if taken in, a design discussion places the move, with a test that replays a commit
  regenerating an index over a mock project; if declined, the entry is deleted with that reason.

Default: open the issue.
Outcome: decided by the session: open an issue, `knowledge-architect`, kind `todo`, slug
`issue@knowledge-architect@a-conflict-in-a-generated-file-is-resolved-by-hand`. Ground: the task follows from the tree with no
judgement and recurs, which `goal@tasks-a-tool-does-better-are-the-tool-s` gives a tool only once the
owner takes it in, so the decision stays the issue's closing condition; the proposed driver shipped
by the install is refused by `design@check@owned-namespace-check`.

## index-staged-leaves-worktree-stale: after `index --staged` and the commit, the working tree's generated file is stale

Members: 2026-11-11-thaum-split-oversized-modules-klarch-workflow C4,
2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow C3.

- **a. What it says.** In thaum, on 0.9.0, a session retitled an entry, ran `index --staged`,
  committed, "and left the working tree's index holding the old title; a reviewer found the stale
  file". It proposes that `index --staged` print each working-tree generated file that now differs
  from the staged one, with "run `cargo klarch index` to bring the working tree level". In this
  repository, a spec was deleted with `git rm`, `index --staged` and `check --staged` passed, the
  commit was made; the working tree's copy of `path@plans@specs/index.md` still held the deleted
  spec's row, so `cargo klarch check` "failed with 'the generated file is out of date' until a plain
  `cargo klarch index` ran". It proposes that `index --staged` "also writes the working tree's copy
  where that copy equals HEAD's" (0 instructions added).
- **b. Whether it still applies.** Yes. The blamed instruction, at `v0.9.0`, at
  `v0.9.0-371-g7173deed` and at `v0.10.0-59-g605d1538` alike, is the primer's paragraph on
  `check --fix` in `path@agent-skills@content/PRIMER.md`: "Take one of the two repairs it names:
  `{{command}} index --staged` then `{{command}} check --staged`, to commit the staged part, or
  `{{command}} index`, to take the working tree whole, staged after it." The phrase thaum quotes,
  "writes the generated files the staged tree needs into git's index", is the changelog's entry for
  the command (CHANGELOG.md, the `cli` line of the release that added it), not the primer's. The
  behaviour is the head's: `design@check@index-staged-write` "sets the staged entry of each whose
  staged bytes differ, or that the index lacks, leaving the working-tree file as it is", and "plain
  `check` judges the working tree as before".
- **c. Whether it is already recorded.** No issue holds it. Two entries bear:
  - `rejected@check@index-staged-writes-working-file`, `brings-back` against dispatch-judgement's
    proposal, action `new-argument-needed`. Its reason: "After it, plain `check` fails, since the
    working-tree file no longer matches the working tree, and `check --staged` fails until the file
    is staged." The proposal's condition, the working-tree copy equal to HEAD's, answers that reason
    only in part. Where the working tree's rows differ from HEAD's, the copy equal to HEAD's already
    fails plain `check`, so writing the staged rows there loses nothing. But where an unstaged
    change undoes the staged one for that register, the copy equal to HEAD's is current for the
    working tree, and the staged rows written there make plain `check` fail: the case the reason
    names. So it is no new argument against the reason as written.
  - `design@check@one-question-one-command`, `conflicts` against thaum's notice: "A second command
    answering the same question in its own format is what drifts". Whether a generated file is
    current is `check`'s question, and `check` already answers it, with the repair `index`.
  - `design@check@fix-refusal-mixed-state` and `design@model@one-list-of-generated-files` constrain
    any change: both repairs the refusal names stay correct, and any write takes the generated files
    from the one list.
- **d. Its kind.** Missing, at the checker: no move of the two named repairs leaves the working tree
  current after a partial commit, and the next `check` meets it. Both sessions recovered with one
  `cargo klarch index`, which the finding of `check` names.
- **e. Its scope.** The checker, `check`; `goal@documentation-stays-consistent`. Each proposal
  changes `design@check@index-staged-write`, whose title says "and no working-tree file": a reversal,
  per `skill@knowledge-architect-decision-recording@reversal-check`.
- **f. A better fix.** A rival neither file names: `index --staged` also writes each working-tree
  generated file from the working tree, as plain `index` does, beside the staged entry from the
  staged tree. The rejected entry's reason does not cover it: that reason is against writing the
  *staged* rows into the working-tree file, and a working-tree file generated from the working tree
  passes plain `check` by construction, while the staged entry passes `check --staged`. It reverses
  the head's title all the same, so it is a design question. The other rival is no change: the next
  `check` names `index`, at the cost of one run, and thaum's reviewer found what that run would have
  found. A line in the primer, after the commit, loses to that rival: the finding already delivers
  the repair at the moment it is needed, per `design@check@finding-names-the-repair`.
- **g. Its route.** A decision of the checker: `skill@knowledge-architect-design`, then
  `skill@klarch-development`. It needs a branch of its own.
- **h. The proposal.** Open an issue in `check`, kind `design`, slug
  `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`: the two instances; the mechanism,
  that the head leaves the working-tree file as it was, so after the commit it holds the old rows;
  the three shapes (thaum's notice, which `design@check@one-question-one-command` defeats;
  dispatch-judgement's write of the staged rows, which the rejected entry's reason still covers;
  the write of the working tree's own generation, which no recorded reason covers), and no change;
  its re-entry point, a design discussion when a change next touches `index --staged`, or earlier on
  the owner's word. Its `Why it matters` names `design@check@index-staged-write`, which it strains,
  and `goal@documentation-stays-consistent`.

Default: open the issue.
Outcome: decided by the session: open an issue, `check`, kind `design`, slug
`issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`. Ground: each proposal changes
`design@check@index-staged-write`; one brings back `rejected@check@index-staged-writes-working-file`
without an argument that answers its reason, the other is defeated by
`design@check@one-question-one-command`, and the shape no record defeats still reverses the head's
title, so the work is a design discussion on a branch of its own.

## item-beside-path-unchecked: no check sees that an item named beside a `path` reference still lives in that file

Members: 2026-11-11-thaum-split-oversized-modules-klarch-workflow C2.

- **a. What it says.** In thaum, on 0.9.0, after a module was split and its file kept as the
  parent, a sentence naming an item `X` beside a `path` reference to the old file "still resolves
  while `X` now lives in a child".
  The session re-pointed 84 rows of one record, and most of the 100 entries the search graded as
  bearing on the work, by a script that listed every line naming an old file within two lines of a
  moved item's name, then read each candidate. Proposal: "for a fixed shape a project declares (a
  table column holding a `path` reference followed by item names), a check that each name occurs in
  the target; prose is out of reach". That project opened an issue for its own record.
- **b. Whether it still applies.** Yes. At `v0.10.0-59-g605d1538`, `primer@checker-model` lists no
  reference kind that names a Rust item, and a `path` reference resolves to its file alone.
- **c. Whether it is already recorded.** Yes, as its class:
  `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, kind `design`: "a move or a rename of
  the item leaves the name stale with a passing check, where a `path` reference to the item's file
  would dangle". Its one instance is a crate split of this repository. Thaum's case is the other
  half: the file stays, so the `path` reference does not dangle either. The proposal's shape is a
  candidate for that issue's re-entry discussion. Its limits, from the verdicts:
  `design@model@one-entity-table` (`conflicts`): "A check that resolves a name of its own is the
  shape this refuses", so a name check resolves through the one table, as the issue's own candidate,
  a reference kind resolved against the Rust parse, does; and `design@check@an-extension-plugs-in-through-phased-hooks`
  with `goal@projects-add-their-own-checks`: a table shape one project declares is that project's
  check, which its binary registers.
- **d. Its kind.** Missing, at the checker; recorded as a class; one further instance.
- **h. The proposal.** Add the instance to the issue, with the shape it proposes and its limits.

Text to add to `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, as a paragraph at the
end of `### What`:

```markdown
A further instance, from thaum's retrospective of its split of oversized modules, on 0.9.0,
2026-11-11: a module was split and its file kept as the parent, so a sentence naming an item beside
a `path` reference to that file still resolved while the item lived in a child. The session
re-pointed 84 rows of one record, and most of the entries its search graded as bearing on the
work, by a script that listed each line naming an old file within two lines of a moved item's
name. A `path` reference does not dangle in this case, since the file stays. Its proposal, a check
that each item named in a table column a project declares, beside a `path` reference, occurs in
the target, is a shape for this discussion: under `design@model@one-entity-table` the names resolve
through the one table, and a shape one project declares is that project's own check, per
`design@check@an-extension-plugs-in-through-phased-hooks`.
```

Default: the instance is added to the recorded issue.
Outcome: decided by the session: open an issue, as an instance added to
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`. Ground: the issue records the missing
checked form for a Rust item named in prose, and this case is the same form missing where the file
survives the move; the proposal is a shape for its discussion, limited by
`design@model@one-entity-table`.

## standing-2026-11-11-thaum-split-oversized-modules-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-11-thaum-split-oversized-modules-klarch-workflow, lines 102 to 122, of a project that
installs the workflow, pinned at 0.9.0. It predates the questions on agents, misses, review cost and
tools.

Carried by a subject, named only:

- four claims of the closing message written from memory and found false by reviewers, a fifth
  caught before the push (lines 105 to 110): `message-claims-written-unchecked`;
- the repoint of every text naming a moved item by its file (C2), the list of unread `may-bear`
  entries for the message (C3), and the conflicts in generated files across cherry-picks (C5) (lines
  116 to 120): `item-beside-path-unchecked`, `unread-may-bear-list-fails-commits` and
  `generated-file-cherry-pick-conflicts`.

Analysed here:

- **A project skill's addition departed from on purpose** (lines 111 to 113): "the project's review
  skill names a worktree directory the owner's global instructions contradict", recorded by the
  project as an issue. Kind: the project's own text against its owner's instructions; no installed
  text is blamed, and the project carries it.
- **A move check by script** (lines 120 to 122): each of fifteen commits checked to move lines only,
  "a script comparing the line multisets of the changed files with a pass for rustfmt's rewraps,
  for an instruction of the project's own, so no finding here". Kind: a tool set up for a project's
  own procedure. A move check of the same kind was set up in this repository, in
  2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow, whose answer reports its
  generic part; this one is counted with it in the owner item on generic tools, in the section of
  2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.
- **Installed file changed**: no. **Backticks**: no. Observations.

Default: no change for the answers analysed here; the move check counted in the owner item.
Outcome: decided by the session: no change here. Ground: the departure is the project's own,
outside the installed text; the move check reaches the owner through the owner item on generic
tools, per `design@agent-skills@tool-report-moment`.

## bears-file-too-large-to-read-whole: the grounding's "read whole `bears.md`" cannot be followed when the graded set exceeds one session's reading

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `bears-file-too-large-to-read-whole`, which gives the outcome.

## unread-may-bear-list-fails-commits: the list of verdicts with the searchers' reasons is a hand-built join that fails `commits`

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow.md`, in its section on `unread-may-bear-list-fails-commits`, which gives the outcome.

## session-owner-line-unrecognised: the transcript reader reports the harness's `session-owner` line type as unrecognised

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow.md`, in its section on `session-owner-line-unrecognised`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
