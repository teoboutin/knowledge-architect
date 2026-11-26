# 2026-11-15-knowledge-architect-intake-forty-seven-files

- **Received file**: 2026-11-15-knowledge-architect-intake-forty-seven-files.md, this repository's own retrospective project file.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-59-g605d1538` when the work began. This repository's main commit then was `605d1538`. The branch was `intake-forty-seven-files`, and the tree held no uncommitted change."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `issue@agent-skills@patching-an-installed-skill`: depends-on, rule-before-work.
  - `design@knowledge-architect@workflow-findings-stay-here`: conflicts, check-reversal.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `rejected@agent-skills@one-repair-per-finding`: constrains, respect.
  - `design@agent-skills@knowledge-table-home`: constrains, respect.
  - `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect.
  - `design@agent-skills@restatement-size-test`: constrains, respect.
  - `design@agent-skills@overlay-by-separate-skills`: constrains, respect.
  - `rejected@agent-skills@directive-restated-everywhere`: constrains, respect.
  - `rejected@agent-skills@finding-reported-not-recorded`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@agent-skills@installed-fix-goes-upstream`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `issue@agent-skills@a-retrospective-that-runs-may-miss-an-item-kept-for-it`: touches, none.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@the-workflow-improves-through-real-use`: constrains, respect.
  - `design@knowledge-architect@no-operation-loses-content`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `design@agent-skills@received-file-marked`: constrains, respect.
  - `design@agent-skills@expectation-set-bounds-scope`: constrains, respect.
  - `goal@the-owner-s-word-is-verified`: constrains, respect.

## workflow-item-routing-override: the root's override of the primer's route for an item on the installed text is missed where the session routes

Members: 2026-11-15-knowledge-architect-intake-forty-seven-files P1;
2026-11-16-knowledge-architect-committed-transcripts P1;
2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow W6 and its standing answer on
a rule of the primer acted on as absent (its third item);
2026-11-22-knowledge-architect-owner-rulings-and-title-rules P1;
2026-11-22-knowledge-architect-owner-rulings-and-title-rules-klarch-workflow, its standing answers
on a rule of the primer acted on as absent (its second item) and on a project skill's addition
missed; 2026-11-25-knowledge-architect-tmp-cleanup P1;
2026-11-25-knowledge-architect-tmp-cleanup-klarch-workflow, its standing answer on a project skill's
addition missed.

- **a. What it says.** The root `CLAUDE.md` sets aside, for this repository, the primer's route of
  an item met outside the task whose fix lies in the installed text or in the checker: here it
  "takes the rows of `primer@met-outside-the-task`, never the workflow's file". Three sessions
  routed such an item to the workflow file anyway, by the primer's own paragraph: the
  committed-transcripts session twice (a turn's closing section and a review record; the routing
  reviewer caught both), the owner-rulings session once ("I'll keep it for this session's
  retrospective", and an earlier spec's repair commit saying "kept for the retrospective's workflow
  file"; the last transcript review caught it), and the tmp-cleanup session once (the transcript
  review caught it). Each became an issue entry or an issue's instance before the merge. The
  forty-seven-file session's P1 is a neighbour, not the same miss: an agent reported the
  wrapped-`Outcome`-line gap, the session repaired the instance, and left the cause with no outcome
  at all until the transcript reviewer found it, at the cost of one commit after the last review
  round. Proposals: the committed-transcripts P1 leads with a row of the root's "Where knowledge
  goes" table, net 0, moved; the tmp-cleanup P1 with the sentence moved "to the section that names
  the moment, or the project's rows of the knowledge table", none added; the owner-rulings P1
  argues no change, since "the exception is one sentence beside its pointer, which
  `primer@where-knowledge-goes` admits, and the session had it in context"; the forty-seven-file P1
  proposes none, a lapse.
- **b. Whether it still applies.** Yes. The sentence is the same at the file's version and at the
  analysed commit: `git show 605d1538:CLAUDE.md` near line 172 and `instructions@repository-skills`
  on main near line 174 both read "**So is an item met outside the task whose fix lies in the
  installed text or in the checker**: here both are this project's own, so it takes the rows of
  `primer@met-outside-the-task`, never the workflow's file, per that head." It sits in the section
  "Skills", in the paragraph on the retrospective's and the audit's findings. The primer's
  paragraph it sets aside, in `path@agent-skills@content/PRIMER.md` near line 237, reads "**An item
  whose fix lies in the installed text or in the checker** [...] goes to the workflow's file of
  `skill@knowledge-architect-retrospective`", and the closing-section rule it routes by sits near
  line 245 of the same file. The root holds no section on something met outside the task (its
  level-two headings, `git show 605d1538:CLAUDE.md` and on main: language and style, mechanical
  validation, where knowledge goes, verify before relying, verify mechanically, skills, git,
  release status).
- **c. Whether it is already recorded.** Yes, as an open question with a reserved answer.
  `issue@agent-skills@patching-an-installed-skill` (`depends-on`, `rule-before-work`) lists this
  supersession among the three this repository holds, and its "What bears on it" holds the first
  instance, the 2026-11-14 slice on splits planned before the work: "The retrospective proposed
  lifting the sentence to the head of the section. One instance; the shape decided here settles
  where it goes." `design@knowledge-architect@workflow-findings-stay-here` (`conflicts`,
  `check-reversal`) states the rule and its place: "The root `CLAUDE.md` states this case beside the
  other, in `instructions@repository-skills`." A move of the sentence rewrites that clause of the
  head; the rule itself, every workflow finding handled here, stands in every reading and no file
  argues against it. `design@agent-skills@knowledge-table-home` (`constrains`) bounds the
  knowledge-table lead: a project row "may refine a primer row [...]; it never contradicts one".
  `rejected@agent-skills@finding-reported-not-recorded` (`constrains`) is the reason the
  forty-seven-file P1's unrouted item was caught: every item leaves a record or a listed line.
- **d. Its kind.** For the three routings to the workflow file, a lapse of the session that recurs
  at one point of delivery: the instruction exists and is clear, and each session met the moment of
  routing, a turn's closing section, through the primer's paragraph, while the override sits in
  another section of another text, read at the session's start. With the 2026-11-14 instance the
  issue records, four sessions in eleven days. For the forty-seven-file P1, a lapse of
  `primer@met-outside-the-task`'s closing-section rule, once, which no text repairs.
- **e. Its scope.** It threatens `goal@records-reach-their-reader`: an instruction the session holds
  in context does not reach the act it governs. The owner of the fix is this repository's root
  `CLAUDE.md`, a project text; the primer's paragraph is installed text, and stays as it is for
  every other project, per `design@agent-skills@installed-fix-goes-upstream` and
  `design@agent-skills@overlay-by-separate-skills`. A move of the sentence is no reversal of the
  rule of `design@knowledge-architect@workflow-findings-stay-here`, but rewrites that head's clause
  on where the root states it, through `skill@knowledge-architect-decision-recording`.
- **f. A better fix.** The lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules` admits a move here: "Where the
  lapse recurs at one point of delivery, because the instruction is read at one moment and needed at
  another, the fix moves where it is delivered, and the recurrence is its evidence." The rivals:
  (1) no change, the owner-rulings P1's argument; it leaves the cost of one review catch and one
  repair per branch that meets such an item, four in eleven days. (2) A row of the root's knowledge
  table; it refines the primer's knowledge table, while the act is routed by the primer's other
  table, of something met outside the task, so a row there is read only if the session reads that
  table at the moment, which no file shows. (3) A root section headed as the primer's section, "Something met outside the
  task", holding the one sentence, so that a session writing its closing section finds a section of
  the same title in the root; 0 instructions added, 0 removed, one moved. (4) The case placed by the
  design discussion of `issue@agent-skills@patching-an-installed-skill`, which decides where every
  supersession is documented, this one included. The deciding fact between (3) and (4) is the
  issue's sentence "the shape decided here settles where it goes": a move now is redone by that
  discussion when its shape differs, and the search's action `rule-before-work` has an outcome that
  depends on an issue wait for its answer. Which comes first, the discussion or the four misses'
  cost, is the order of known work, which no goal ranks and the primer's knowledge table gives to
  the owner, in `path@knowledge-architect@docs/roadmap.md`, where the issue holds no row.
- **g. Its route.** The instances to `issue@agent-skills@patching-an-installed-skill`, under
  `skill@knowledge-architect-issue-tracking`; a move of the sentence, if the owner takes the
  discussion now, under `skill@knowledge-architect-design` for the shape and
  `skill@knowledge-architect-agent-configuration` for the root's text, with the clause of
  `design@knowledge-architect@workflow-findings-stay-here` rewritten under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Add the four misses to the issue's "What bears on it", replacing "One
  instance;" since the count changes, by the text in
  the intake's draft instances-B (not committed; its text is in the issue), and leave the sentence where it
  is until the issue's discussion places it. The owner can instead take that discussion now: the
  question the search put, reading 1 (instances, the sentence in place) against reading 2 (the
  discussion now). No text for the forty-seven-file P1, a lapse met once.

Default: reading 1: the instances added to `issue@agent-skills@patching-an-installed-skill`, the sentence left in place until its discussion; no change for the unrouted item of P1.
Outcome: for the owner (the order of known work, which no goal ranks: whether the discussion of `issue@agent-skills@patching-an-installed-skill` runs now, against the cost of the misses meanwhile): reading 1, the default: the instances added to `issue@agent-skills@patching-an-installed-skill`, the sentence left in place until its discussion. The owner: "defaults for all other items" (the owner's numbers are those of the message that put the nine outcomes to the owner, items 1 to 9 being, in order, tools-built-in-sessions, intake-assembly-by-script, parallel-agents-combined-by-cherry-pick, workflow-item-routing-override, standing-misread-course-reentry, transcript-show-has-no-end, phrase-search-across-line-break, batch-worktrees-and-build-bounds and child-agents-outlive-parent-hand-back; this outcome was item 4)

## restatement-edited-without-conversion: a restatement edited without its conversion to a pointer

Members: 2026-11-15-knowledge-architect-intake-forty-seven-files P2.

- **a. What it says.** The branch edited one sentence of point 2 of the root `CLAUDE.md`'s Git
  section, which restated more than one sentence of
  `design@knowledge-architect@no-operation-loses-content`; `primer@where-knowledge-goes` owes the
  conversion "when a change edits what it says". The applier measured only the bullet it edited; the
  routing reviewer found the omission and a repair converted the point to a pointer. No proposal: a
  lapse.
- **b. Whether it still applies.** No: repaired on that branch. `instructions@git-workflow` point 2
  on main, near line 217, reads "**No operation that can lose content, committed or not.** Read its
  home whole before any operation of this point". The repair is 4fb9a9d1, "[review] The agent
  configuration's repairs: point 2 of the git workflow points to its home, a check named, and net
  effect judged over the whole range", on main, whose message says the point "restated four
  sentences" of the head. The primer's rule is unchanged, `path@agent-skills@content/PRIMER.md` near
  line 440: "An existing longer restatement is converted when a change edits what it says".
- **c. Whether it is already recorded.** `design@agent-skills@restatement-size-test` (`constrains`)
  is the rule's head; no issue.
- **d. Its kind.** A lapse of the session, once: the instruction exists and is clear, and the
  routing axis caught it, as it is built to.
- **h. The proposal.** No change: a lapse, already repaired.

Default: no change, a lapse already repaired.
Outcome: decided by the session: no change, a lapse already repaired. Ground: 4fb9a9d1, on main, converted point 2 to a pointer; one lapse is no recurrence, per the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

## owner-directory-name: the owner's request named a directory the user instructions spell otherwise

Members: 2026-11-15-knowledge-architect-intake-forty-seven-files P3.

- **a. What it says.** The owner's request named a "retrospective" pending directory, singular; the
  user-level instructions name `~/Documents/retrospectives/pending`. The session checked the machine
  and used the instructions' name. No change proposed.
- **b. Whether it still applies.** The user-level instructions, outside this repository, still
  name `~/Documents/retrospectives/pending` as the place of new reports, and the received files of
  this intake lie there.
- **c. Whether it is already recorded.** `goal@the-owner-s-word-is-verified` (`constrains`): the
  session checks what the owner's word assumes against what it can measure, which the session did;
  `design@agent-skills@received-file-marked` (`constrains`) moves a handled file where the owner's
  instructions name.
- **d. Its kind.** An observation: an instruction followed as written. It is about the owner's
  words, under no installed skill's expectation set, per `design@agent-skills@expectation-set-bounds-scope`.
- **h. The proposal.** No change.

Default: no change, an observation.
Outcome: decided by the session: no change, an observation. Ground: the session verified the owner's word against the machine, as `goal@the-owner-s-word-is-verified` asks, and no text was at fault.
