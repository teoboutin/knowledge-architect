# 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.9.0-387-gbd37bb85` when the work began, branch `finding-repairs`, tree clean."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@a-past-sentence-is-rewritten`: constrains, respect.
  - `design@check@message-references-resolve-in-either-tree`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@documentation-has-a-structure-and-a-model`: constrains, respect.
  - `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`: touches, respect.
  - `design@git@an-operation-in-git-s-terms-is-the-library-s`: constrains, respect.
  - `design@agent-skills@transcript-review-repairs-are-additional-commits`: constrains, respect.
  - `design@agent-skills@capabilities-not-structure`: constrains, respect.
  - `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`: touches, respect.
  - `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`: touches, respect.
  - `design@knowledge-architect@git-flow`: constrains, respect.
  - `design@knowledge-architect@rebase-merge-alone`: constrains, respect.
  - `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: constrains, respect.
  - `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements`: trigger-met, rule-before-work.

## deleted-entry-bare-slug-easy-to-miss: the bare-slug rule for an entry deleted by the commit is exact and easy to misstate

Members: 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow W3.

- **a. What it says.** "The session's brief told an implementer to name a deleted issue by its bare
  slug in the commit that deletes it. `cargo klarch commits` refused it: the first parent still
  holds the entry. The primer's rule ('only where its tree does not hold the entry … nor, for a
  commit message, its first parent's') is exact. Reported only as evidence that the rule's case, the
  commit that deletes the entry, is the common one and is easy to get wrong from memory." The file
  labels it "no defect; a pointer", and proposes nothing.
- **b. Whether it still applies.** The rule stands in `primer@where-knowledge-goes` at
  `v0.9.0-387-gbd37bb85` and at `v0.10.0-59-g605d1538`: "Either takes that form only where its tree
  does not hold the entry or the skill, nor, for a commit message, its first parent's". The refusal
  the implementer met names its repair: the finding of a backticked bare id, in
  `path@check@src/check/references.rs`, says "`<id>` is written in backticks without its kind, and
  names `<ref>`" with the repair "write" that reference, so the right form is delivered at the
  moment of the commit. `design@check@message-references-resolve-in-either-tree` gives the reason the
  full reference resolves: "A commit that closes an issue deletes the entry and names it in the
  message, and against its own tree alone every such message would dangle."
- **d. Its kind.** A lapse, of the session's brief, which misstated an exact rule from memory; the
  file's own standing answer counts it among the primer rules acted on as absent. One instance, and
  the checker caught it with its repair.
- **f. A better fix.** A restatement of the rule where a deleting commit is written would be a second
  home for one sentence of the primer, and the lapse rule adds no text for a clear instruction not
  followed; the checker already delivers the repair at the commit. No change.

Default: no change.
Outcome: decided by the session: no change, a lapse. Ground: the file finds the primer's rule exact,
the finding that refused the brief's form names the right one at the commit, and
`skill@knowledge-architect-retrospective-intake@admission-rules` repairs no lapse by adding text.

## review-record-moves-each-round: the record of a review sits in the last repair commit, so each later round moves it

Members: 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow W1. One cause with `transcript-review-record-without-repairs` (2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow W1): both come from the rule that names one commit for the record of the whole review, and one repair below closes both.

- **a. What it says.** `skill@knowledge-architect-review@what-review-leaves` puts the record of the review in "the message of the last repair commit". A decision-record re-review of a repair that splits a head lands its repairs after that commit. In the session, the record was written once and moved four times, each by a `git filter-branch --msg-filter` script, with the empty-diff and range-diff checks and a re-run of the gates each time: after the first round's last outcomes, after the second decision-record review, after the third, and after the transcript review's repairs. The file names the cause "a structure the work did not need": the rebase merge keeps every message of a commit that changes a file, and what the rule buys is that a reader finds the record at the branch's tip. Proposed fix: the record goes in the commit that holds the last repair of the axes run together, and a later round records itself in its own last repair commit, naming the earlier record. Rival named: a command that moves the record.
- **b. Whether it still applies.** Yes. At the file's version, v0.9.0-387-gbd37bb85, and at origin/main, `path@agent-skills@content/skills/review/SKILL.md`, the bullet that opens "A paragraph in the commit message" (line 220 at origin/main) reads: "records the review: the message of the last repair commit, or, where no repair commit is left, the message of the branch's last commit, reworded with no uncommitted work in the tree. Where the last transcript review has repairs, the record of its items is the message of its last repair commit, and the record of the earlier axes stays in the commit that held it when the transcript reviewer read it." (`git show v0.9.0-387-gbd37bb85:crates/agent-skills/content/skills/review/SKILL.md`, line 218, same words.)
- **c. Whether it is already recorded.** No entry records where the review's record goes; the skill's sentence is its one statement (`grep -rn -i 'last repair commit' crates/*/docs docs --include=*.md`, outside docs/retrospective-reports/, returns nothing). The second sentence quoted in b came from an earlier intake: the analysis of 2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow, W2 cluster, deleted in 467438ce and carried out in 7e0bb594, "[agent-skills] The intake's outcomes on the installed text that reword an instruction or add a member its head covers". That analysis chose, for the transcript review's repairs, "the transcript review's outcomes recorded in its own repair commit, and the earlier axes' record left where the transcript reviewer read it, which keeps one record per review that produced it and edits no history" (`git show 467438ce^:docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md`, section "W2, with the findings on how the review before a merge ends", item f). The fix here extends the same argument to every round; it reopens nothing that lost.
  The verdicts, acted on:
  - `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file` and `design@knowledge-architect@rebase-merge-alone` (constrains, respect): each round's record sits on a repair commit, which changes a file, or on the branch's last commit; no record is put on an empty commit. The installed bullet keeps its scope, "Where the branch's commits reach the main branch as they are (a fast-forward, or a rebase merge)", under which every such message reaches main.
  - `design@agent-skills@transcript-review-repairs-are-additional-commits` (constrains, respect): the fix leaves those repairs additional commits.
  - `design@knowledge-architect@git-flow` (constrains, respect): its bullet "Where every repair was folded, the record of the review goes into the message of the branch's last commit" stays true under the fix, which keeps the branch's last commit for a round with no repair commit left.
  - `issue@knowledge-architect@the-git-flow-head-bundles-unargued-statements` (trigger-met, rule-before-work, per the searcher): its re-entry is "the next change that edits the head". The fix edits the installed skill, not that head, whose sentence stays true, so the re-entry is not met. This is the session's reading of the head's text quoted above; the issue's outcome does not wait on it.
  - `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history` and `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command` (touches, respect): the four moves were rewords by hand. Under the fix no record moves, so these moves leave the issues' instances; the issues stand for every other reword.
  - `design@agent-skills@capabilities-not-structure` (constrains, respect) and `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies` (touches, respect): the repair removes a structure rather than adds a rule, and it does not decide where that rule stops.
  - `design@git@an-operation-in-git-s-terms-is-the-library-s` (constrains, respect): bears only on the rival, a command.
- **d. Its kind.** A defect of the text: a structure the work did not need, one fixed commit for the record of every round, which a later round must move.
- **e. Its scope.** The installed review skill, the agent-skills Component. `goal@agents-get-a-complete-workflow`. No head states the rule, so the fix reverses none; it changes what agents are told, so it is a decision, which the argument of the earlier analysis covers.
- **f. A better fix.** The nearest rival is a command that moves the record (the file's own rival, and the two issues above). What decides: a record that never moves needs no history edit, so no command, and costs no reader anything, since the last round's record, at the branch's tip, names the commit of each earlier one. The second rival is the proposal of `transcript-review-record-without-repairs`, deleting the transcript-review clause alone: it answers the round with no repairs and leaves the four moves. One repair closes both, since both come from one commit named for the record of the whole review.
- **g. Its route.** A text edit of the installed text at its source, `path@agent-skills@content/skills/review/SKILL.md`, as `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, read whole.
- **h. The proposal.** Handle now. Replace, in the bullet that opens "A paragraph in the commit message", the text from "records the review:" to "when the transcript reviewer read it." with the text below. Instructions removed: 2 (the record goes in "the last repair commit" of the whole review; the transcript review's own sentence, whose two parts the new text keeps as the general case). Instructions added: 2 (each round records itself in its own last repair commit, or the branch's last commit reworded; a later round names the commits of the earlier records and moves none). Net 0. The session holds it: the earlier intake's argument, "one record per review that produced it and edits no history", covers the new member, and no head or goal is strained.

```
- **A paragraph in the commit message** records each round of the review, the axes dispatched
  together and the repairs of their findings: the message of that round's last repair commit, or,
  where the round leaves no repair commit, the message of the branch's last commit, reworded with
  no uncommitted work in the tree. A record stays in the commit that holds it: a later round's
  record names, by subject, each commit that holds an earlier round's record.
```

  The rest of the bullet, from "A record carried by a message rides on a commit that changes a file", is kept.

Default: handle now, as h says.
Outcome: decided by the session: handle now: the review skill's record bullet reworded so each round records itself in its own last repair commit and no record moves. Ground: the argument the analysis of 2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow gave for the transcript review's record, which covers every round; `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`, which the new text keeps; no head states the rule.

## standing-2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: the standing answers

Members: the section "Answers to the standing questions" of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow, lines 66 to 104. Four of its answers
take sections of their own below: the missed tripwire, the agents unfinished by file ownership, the
audit of a family of texts, and the owner item on generic tools, which every file of the batch that
reports a tool points to. Its review-cost counts are in the lines for
`issue@agent-skills@the-review-cost-counts-are-not-compared-yet`.

Carried by a subject, named only:

- "confirmed by reading the code" written in an issue entry before reading it, corrected before the
  commit (lines 71 and 72): `message-claims-written-unchecked`;
- the brief that misstated the bare-slug rule (line 73): `deleted-entry-bare-slug-easy-to-miss`
  (W3 of this file);
- the record of the review moved four times and the message rewords (lines 77 to 79):
  `review-record-moves-each-round` and `no-command-rewords-branch-messages`; the count of a split
  head's citing sites by `git grep` (lines 79 and 80): the cost `touched-head-owes-whole-audit`
  weighs (W2 of this file);
- the installed sentence judged still true after the reversal, which three reviewers found false
  (lines 89 and 90): one of the batch's three lapses of `skill@knowledge-architect-decision-recording@reversal-check`, counted in the standing section
  of 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow.

Analysed here:

- **Implementers wrote stray files outside their worktrees four times; each deleted its own** (lines
  73 and 74). Kind: a lapse against the briefs, with no content lost.
- **Wrapping commit message lines to the width the agents kept** (line 80), by script. Kind: a task
  with no judgement, one session; no command and no record, and too small for a command.
- **Work an agent could have carried** (lines 83 and 84): "The assembly of each wave (cherry-picks)
  and the three record moves, done in the main session, spent its context." Kind: an observation of
  the dispatch judgement of `primer@dispatching-work`; the record moves are the subject
  `review-record-moves-each-round`, and the waves' assembly is the partition the section below
  records.
- **Installed file changed** (lines 68 to 70): at its source, two sentences the reversal made false.
  **Code rules** (lines 81 and 82): unit tests asked for logic reached only through the binary,
  "that held". Observations; the second bears on `tripwire@agent-skills@logic-tests-relabelled` and
  does not fire it.

Default: no change for the answers analysed in this section.
Outcome: decided by the session: no change. Ground: a lapse against briefs with nothing lost, a
mechanical step too small for a command, and observations of rules working as written.

## standing-2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: the tripwire the search missed, `tripwire@model@candidate-rule-silence`

Members: the answer "A register entry the record search missed?", lines 92 to 94: "Yes:
`tripwire@model@candidate-rule-silence`, found by the standing-state and decision-record reviewers;
it fired on the reversal's own decision. Why the search missed it is not established from the
reports."

- **Whether it still applies.** The firing was handled on that branch: 6e14d889, "[model] The
  tripwire on the candidate rule's silence leaves out a slug with one hash sign", now on main,
  narrowed the tripwire, which stands at crates/model/docs/tripwires.md, line 6.
- **Whether it is recorded.** The miss is not stated as a part of the work added after the search,
  so `skill@klarch-retrospective-intake@received-analysis` does not add it to
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`. That issue's first instance is
  a miss of unestablished cause too, recorded at the tripwire's firing, whose response asked what
  produced the misses; the issue's own scope is the added part, by its title.
- **Rival: add it there as a miss of unestablished cause.** The fact that decides: an instance with
  no cause gives the issue's discussion no mechanism to weigh among the causes its closing condition
  names (an agent's load, a split of a search unit, the reading, the quotation, a part added after
  the search), and the repository's intake skill adds only the misses it names.

Default: no change; the miss stays in this analysis and in history.
Outcome: decided by the session: no change. Ground: the firing was handled on its branch
(6e14d889); the miss carries no established cause, and the intake skill's step adds to the issue
only a miss on a part added after the search.

## standing-2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: agents unfinished by file ownership

Members: the answer "An agent returned unfinished?", lines 85 to 88: "Several, by file ownership:
the briefs gave each implementer a disjoint set of files, and five items needed one file another
agent owned (the `--staged` repair of a generated index, a non-UTF-8 options file, a parse trouble
carrying its action, a deleted index's regression patch). Each came back with the evidence and was
carried by a later agent."

- **Whether it is recorded.** `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`, a
  `deferred` entry, holds "How a plan document partitions work among agents dispatched in parallel,
  in a shared tree or in a worktree each, who commits, and how the parts are merged", with "The
  facts a design starts from". Its trigger, a plan document whose work wants agents in parallel, is
  not met: this branch added no plan document of its own
  (`git log bd37bb85..8a109107 -- docs/plans/` lists three edits of one other spec only, each a modification). The answer is a fact that design starts from, measured
  in a real session: a partition by file sends back every item whose fix crosses the partition.
- **Kind.** An observation, from a real session, of a cost of one partition shape; no instruction is
  blamed, since the partition is decided by no text.
- **The dispatched-report trigger.** `issue@agent-skills@dispatched-agents-write-their-report-as-they-work`
  waits for "a dispatched agent that returned with a report the session could not act on without
  redoing the work"; here each "came back with the evidence", so its trigger is not met.

Default: add the fact to the issue's list "The facts a design starts from", as below; no other
change.

```
- A partition by file sends back every item whose fix crosses it: in the branch that repaired the
  checker's finding texts, three waves of implementers in worktrees each held a disjoint set of
  files, and five items needed a file another agent owned; each came back with its evidence and was
  carried by a later agent, and the main session assembled each wave by cherry-picks (the standing
  answers of 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow).
```

Outcome: decided by the session: handle now, the fact added to
`issue@agent-skills@how-parallel-work-is-partitioned-among-agents`. Ground: the entry collects the
facts its design starts from, and this one is measured in a real session, which
`design@agent-skills@behaviour-additions-need-real-use` asks before a design rests on it; the edit
changes no instruction.

## standing-2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: an audit of a family of texts against one head

Members: the answer "A tool or procedure set up?", lines 99 to 104: "An audit of a family of texts
against one design head: one inventory agent per group of files, each giving every finding its
causes and a verdict, then the implementers as the independent verification. Generic: auditing
every text of one kind against one rule. Particular: the finding texts and
`design@check@finding-names-the-repair`. The version used ships audit axes for design records,
tripwires and agent text, not for a family of code texts: in part."

- **Whether it is recorded.** `issue@agent-skills@audit-axes-beyond-the-design-record`, a `todo`
  entry, keeps the axes no audit has and "the lessons of the audits run by hand before the skill
  existed that no axis has yet adopted", with two worked instances. This is a third, run by hand
  after the skill existed, on the alignment of an implementation's texts with a head, an axis that
  entry names ("alignment of implementation with design and goals"). The searcher of that entry
  routed this answer to it.
- **Kind.** A procedure set up, reported per `design@agent-skills@tool-report-moment`; its generic
  part also goes to the owner item below.

Default: add a third worked instance to the issue, after its second, as below.

```
**A third worked instance.** The branch that repaired the checker's finding texts, widened on the
owner's word to an audit of every finding text against `design@check@finding-names-the-repair`, ran
one inventory agent per group of files, each giving every finding its causes and a verdict, and
used the implementers of the repairs as the independent verification. Its session judged the
generic part, auditing every text of one kind against one rule, shipped in part, since the audit
axes read design records, tripwires and agent text and no family of code texts (the standing
answers of 2026-11-12-knowledge-architect-finding-repairs-klarch-workflow). It is an instance of
the axis on the alignment of an implementation with the design.
```

Outcome: decided by the session: handle now, the instance added to
`issue@agent-skills@audit-axes-beyond-the-design-record`. Ground: the entry keeps the audits run
by hand as lessons for the axes it lists, and this one runs on an axis it names; the edit changes no
instruction.

## standing-2026-11-12-knowledge-architect-finding-repairs-klarch-workflow: the generic part of each tool or procedure the batch reports

Members: the answers on a tool or a procedure set up of every workflow file of the batch that
reports one, this file first by stem. The answers that set up none (release-0-10-0,
split-guidelines-plan-directories, conformance-before-harvest-slice) and the files that predate the
question take no item.

- **The goal and the head.** `goal@generic-tools-and-procedures-are-shipped`: "knowledge-architect
  ships the generic part of one once knowledge-architect's owner takes it in ... A part is taken in
  by that owner's choice, and not because it is generic." `design@agent-skills@tool-report-moment`:
  the session "reports and does not filter", since "the owner can take in only what reaches the
  owner". The searcher of the goals asked whether the owner takes in the generic part of each. So
  each report reaches the owner here, once, with the session's reading of what is generic and of
  any subject or entry that already carries it.
- **The reports**, each with its generic part as its file states it:
  1. this file: an audit of every text of one kind against one rule, by inventory agents per group
     and the implementers as verification; shipped in part. Also added above to
     `issue@agent-skills@audit-axes-beyond-the-design-record`.
  2. 2026-11-12-knowledge-architect-retrospective-standing-questions-klarch-workflow, lines 113 to
     117: two steps of this repository's intake skill (review counts to an issue; missed entries to
     an issue); "Generic part: none found", particular to the repository that receives workflow
     files.
  3. 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow, lines 100 to 103: two
     dynamic workflows, one running the 36 record searchers of a design audit, one the eleven axes
     of the review before a merge, each agent in its own worktree; "Generic to every project running
     those activities"; the version ships saved workflows for the audit axes only. The search's
     workflow is the rival W1 of 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow
     proposes, weighed under `searcher-dispatch-exceeds-harness-cap`; the review's is weighed with
     `review-in-workflow-loses-transcript` and
     `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`.
  4. 2026-11-13-thaum-long-draw-klarch-workflow, lines 139 to 144: a prior-art survey of two open
     engines from shallow clones, quoting code with line numbers, "generic to any project that
     implements a specification other engines implement", shipped in part ("the design skill
     describes when to propose a survey, not how to run it"); a measurement kept as a probe, shipped
     wholly, which needs nothing.
  5. 2026-11-13-thaum-pin-move-0-10-klarch-workflow, lines 196 to 201: the "Plausible misreading:"
     line on a goal review's `blocks` finding, generic, not shipped, weighed under
     `two-sense-term-blocks-stop`; installed agents dispatched as general-purpose agents told to
     read their definition from disk, generic, shipped in part, weighed under
     `stale-agent-definition-after-install`; a message replay, generic, not shipped, weighed under
     `no-command-rewords-branch-messages`.
  6. 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow, lines 140 to 145: a
     move check of a move-only commit of a design home ("checking a relocation, not shipped"), and a
     heads-coverage test of a class of heads for required statements (not shipped); and, of the
     same kind as the first, the line-multiset move check of
     2026-11-11-thaum-split-oversized-modules-klarch-workflow, lines 120 to 122, set up for that
     project's own instruction.
  7. 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow, lines 121 to 124: the
     code-split scout, shipped, as the slice's work; a harness limit measured by dispatching a fresh
     agent, "so the session's own context does not distort it", generic, not shipped.
  8. 2026-11-14-thaum-casting-process-design-klarch-workflow, lines 124 to 131: a scratch file of
     the cut into slices handed to an assembling agent as decided ("The version used does not say
     where the cut is written for an assembler"), and a repair brief of numbered rulings for one
     repairing agent ("the version used ships the outcomes, not a brief shape"); both generic.
- **What the session cannot decide.** Which part to take in is the owner's by the goal's words;
  generic is necessary, not sufficient. The session's reading, for the owner to weigh: the items a
  subject already weighs (3, 5) take that subject's outcome first, and the owner's word here only
  where it takes in more than that outcome does; the move checks of 6 check mechanically what the review's routing and fidelity-of-relocation axes judge
  by reading, per `primer@design-heads`; 4, 7 and 8, and the heads-coverage test of 6, are
  procedures no subject or entry of this intake carries. The session makes no claim that any of them
  is worth its text: that is the admission standard's question, asked when a part is taken in.

Default: none is taken in at this intake; the reports stay in this analysis and in history, and a
part the owner takes in opens an issue in the Component that would ship it, as
`issue@knowledge-architect@this-repository-s-own-tools-are-not-checked-for-a-generic-part` closes
its own check.
Outcome: for the owner (a choice `goal@generic-tools-and-procedures-are-shipped` reserves to the
owner): pending.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## review-cost-trigger-met: the trigger of the review-cost series, met by three proposals argued from review rounds

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage-klarch-workflow.md`, in its section on `review-cost-trigger-met`, which gives the outcome.

## touched-head-owes-whole-audit: a head that a change touches owes its whole conformance, older debts included, at any edit

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage.md`, in its section on `touched-head-owes-whole-audit`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
