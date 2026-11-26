# 2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow

- **Received file**: 2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: main at `v0.10.0-126-g8c994ace` when the work began, on the branch `mutation-survivors-three-suites`, with a clean tree."
- **Analysed against**: main at `v0.10.0-302-g31ea157f`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `design@agent-skills@shipped-text-cites-no-entry`: constrains, respect.
  - `issue@agent-skills@the-material-finding-duty-has-no-head`: touches, none.
  - `design@agent-skills@search-record-kept-per-occasion`: constrains, respect.
  - `design@agent-skills@no-thread-closes-before-the-search`: constrains, respect.
  - `design@agent-skills@outcome-over-display`: constrains, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `design@agent-skills@local-intent-binds`: constrains, respect.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `issue@check@no-checked-form-cites-a-point-inside-a-section`: touches, respect.
  - `design@knowledge-architect@changelog-entries`: constrains, respect.
  - `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@query@record-is-every-register-entry`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@one-skill-per-activity`: constrains, respect.
  - `design@agent-skills@code-change-reads-its-record`: constrains, respect.
  - `issue@agent-skills@a-record-search-misses-what-is-added-after-it`: touches, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `goal@design-with-agents-as-counterparts`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: touches, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`: constrains, respect.
  - `design@agent-skills@xtask-pins-checker`: constrains, respect.
  - `design@agent-skills@one-run-pass`: constrains, respect.
  - `issue@agent-skills@a-saved-workflow-runs-the-review-before-a-merge`: touches, respect.
  - `design@agent-skills@a-scratch-directory-per-subagent`: constrains, respect.
  - `design@check@a-foreign-build-is-refused`: constrains, respect.
  - `issue@git@tools-run-git-past-the-wrapper`: touches, respect.
  - `design@git@an-operation-in-git-s-terms-is-the-library-s`: constrains, respect.
  - `design@knowledge-architect@a-build-is-tied-to-its-checkout`: constrains, respect.
  - `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@agent-skills@settled-means-recorded`: conflicts, check-reversal.
  - `design@agent-skills@primer-limit`: constrains, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@agent-skills@context-cost-of-agent-text`: constrains, respect.
  - `rejected@agent-skills@design-load-unconditioned`: constrains, respect.
  - `rejected@agent-skills@trigger-judges-settled-design`: constrains, respect.
  - `issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`: trigger-met, rule-before-work.
  - `design@agent-skills@development-skill`: constrains, respect.
  - `design@agent-skills@no-routing-table`: constrains, respect.
  - `design@agent-skills@installed-skill-takes-precedence`: constrains, respect.
  - `rejected@agent-skills@root-routing-table`: constrains, respect.
  - `rejected@agent-skills@admission-standard-in-primer`: constrains, respect.
  - `rejected@agent-skills@verdicts-verified-in-the-workflow`: constrains, respect.
  - `design@agent-skills@premortem-as-watch-points`: constrains, respect.
  - `design@agent-skills@synthetic-evidence-not-built`: constrains, respect.
  - `design@query@light-line-keeps-its-reason`: constrains, respect.

## reopened-thread-search: whether a thread reopened on the subject the search read owes a search of its own

Members: W1 of 2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow.

- **a. What it says.** A review found a premise of a closed thread false, and the thread reopened as
  a second question to the owner, on the subject the search had read. The decision-record reviewer
  of the repair read the in-change path, whose commit carries "the lines of `generated-lines.md`
  that the grounding's record search wrote", as owing a second search; the session started one,
  about 24 agents, and stopped it when the owner asked whether it was needed. The file blames loop
  step 1, "When a thread opens, after the search, on a subject its work did not cover, run a search
  of its own", and the in-change path, and proposes that loop step 1 state its case whole: a thread
  that opens or reopens after the search runs a search of its own only where it brings in a subject
  the search did not read (rewords one, adds 0).
- **b. Whether it still applies.** The text is the same at the file's version and on main. In
  `path@agent-skills@content/skills/design/SKILL.md`, loop step 1, near line 585 on main (572 at
  8c994ace): "When a thread opens, after the search, on a subject its work did not cover, run a
  search of its own, with the draft extended by that thread as the work and the thread as its
  subject"; near line 513 on main (511 at 8c994ace): "It is the discussion's one search of the whole
  record, never a pass over what changed since the first rounds." The in-change path, near line 164
  on both, names the lines "that the grounding's record search wrote", one search.
- **c. Whether it is already recorded.** `design@agent-skills@record-searched-before-the-work`
  (read with `cargo klarch show`) has the search run "again for a thread that opens on a subject the
  first work did not cover", for the stated reason that the first search "judged it against the
  first draft". The search graded `design@agent-skills@no-thread-closes-before-the-search`
  `constrains, respect`, which routes an entry returned on a held thread, and
  `rejected@agent-skills@two-readings-left-to-judgement` `constrains, respect`: a confirmed
  two-readings instruction is reworded. No issue.
- **d. Its kind.** A lapse of a reviewer, not two readings. The condition of the rule is the
  subject, "on a subject its work did not cover", which decides a reopening as it decides an
  opening, and the step says the search is "the discussion's one search of the whole record, never
  a pass over what changed". No text gives a second search for a subject the first one read: the
  in-change path names the lines of the one search the grounding ran. The reading the session took
  is the text's.
- **e. Its scope.** The installed design skill; no decision reversed.
- **f. A better fix.** The file's rewording adds "or reopens" to a condition that already decides
  the case, so it changes no act, and one reviewer's reading, once, is no two readings that a
  reading confirms, the case `design@agent-skills@no-two-installed-instructions-contradict`
  repairs: "an instruction that a reading confirms is read two ways is repaired". Rival: the rewording, which costs nothing in use and would end a reading
  the condition already excludes; it loses on the lapse rule of
  `skill@knowledge-architect-retrospective-intake@admission-rules`, since one lapse of a clear
  instruction is not repaired by adding text, and on the fact that the reviewer's reading came from
  the in-change path, which the rewording does not touch. The file also notes that the finding lay in
  code comments, which the search reads by no reading, per `design@query@record-is-every-register-entry`:
  a second search could not have found it, which `grounding-skips-code-comments` below takes.
- **g. Its route.** Nothing.
- **h. The proposal.** No change: a reviewer's lapse of a clear condition.

Default: no change, a lapse.
Outcome: decided by the session: no change, a lapse of a clear instruction. Ground: loop step 1 of `path@agent-skills@content/skills/design/SKILL.md` asks a search of its own only "on a subject its work did not cover" and states the search is "the discussion's one search of the whole record", the same at 8c994ace and on main; the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

## grounding-skips-code-comments: the design grounding did not read the reason a test comment records

Members: W2 and the standing answer on primer rules, its second item, of
2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow.

- **a. What it says.** The decision was how the tests check a set of texts. Two older tests carried
  a comment saying why they compare a text whole, citing a head. The record search reads no
  comment, and its searcher graded that head `unrelated`; the session's grep covered the design
  homes, the `rejected` homes and the scoped `CLAUDE.md` files, not the comments. Q1 went to the
  owner on a premise the comment defeats; five reviewers found it, and Q2 reopened it. The standing
  answer adds that the comment grep of `skill@knowledge-architect-decision-recording@reversal-check`
  was left out of the decision's commit, which the decision-record reviewer reported. Proposal: move
  that grep into loop step 1 of the design skill, beside the grep of the record, the reversal check
  pointing to it (one home, adds 0).
- **b. Whether it still applies.** The instructions that cover the case were all in the text at the
  file's version, and still are:
  - the in-change path of `path@agent-skills@content/skills/design/SKILL.md`, near line 170 at
    8c994ace and on main: the reversal check and the head tests decide, "Apply both before
    proposing to the owner where a decision lands";
  - the reversal check, `path@agent-skills@fragments/reversal-check.md`, line 11 at 8c994ace and on
    main: "Grep the subject's terms across every document that carries decisions (the design homes,
    the `rejected` homes, the scoped `CLAUDE.md` files) and the comments of the code the subject
    touches";
  - the primer, "Read the record of code before changing it": "read what the project decided about
    that code: each entry its comments cite".
- **c. Whether it is already recorded.** `design@agent-skills@local-intent-binds` (`constrains,
  respect`) states the rank of a reason at the code; `design@agent-skills@code-change-reads-its-record`
  (`constrains, respect`) sends a change to code to the entries its comments cite;
  `design@query@record-is-every-register-entry` (`constrains, respect`) keeps comments out of the
  search's record. The head the comment cites missed by its searcher is in
  `searcher-grade-refuted-later` below. No issue.
- **d. Its kind.** A lapse of the session. On the in-change path the reversal check runs before the
  proposal, and its grep names the comments of the code the subject touches; the session ran the
  grep without its last member. The primer's reading of the record of code, which names the entries
  a comment cites, covered the same comment. One instance.
- **e. Its scope.** The installed design and decision-recording skills. The file's move would make
  the reversal check, which runs at every write into a design home and not only in a discussion,
  read its comment grep from a step of another skill.
- **f. A better fix.** The file's rival, extending the search to comments, loses on
  `design@query@record-is-every-register-entry`, as the file says. The file's move loses to no
  change: it removes the comment grep from a check that also runs outside any design discussion,
  where loop step 1 is never run, or leaves there a pointer to another skill's step, which
  `design@agent-skills@one-skill-per-activity` admits only at a step that needs it (graded
  `constrains, respect`). The case a move would cover beyond this one, a full-path discussion that
  puts a fork before any reversal check, no session has shown; it predicts a behaviour, which the
  evidence rule of `skill@knowledge-architect-retrospective-intake@admission-rules` parks rather
  than writes, and the file's own instance ran on the in-change path. The deciding fact: on that
  path the instruction was delivered at the moment it was needed, and not followed.
- **g. Its route.** Nothing.
- **h. The proposal.** No change: a lapse of an instruction delivered at its moment.

Default: no change, a lapse.
Outcome: decided by the session: no change, a lapse. Ground: the in-change path of `path@agent-skills@content/skills/design/SKILL.md` applies the reversal check "before proposing to the owner", and `path@agent-skills@fragments/reversal-check.md` greps "the comments of the code the subject touches", both at 8c994ace; one lapse is not repaired by text, per the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

## batch-worktrees-and-build-bounds: the worktrees, scratch directories and build bounds of a batch of agents are made by hand

Members: W3, C1, the standing answers on scripted tasks (worktree part), on unfinished agents (the
nine reviewers lost to the restart) and on tools (the build-job cap), of
2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow; the standing answer on tasks
carried by hand of 2026-11-25-knowledge-architect-tmp-cleanup-klarch-workflow (its worktree part);
P4 of 2026-11-25-knowledge-architect-windows-and-macos-gates. Three retrospectives.

- **a. What it says.** Eleven reviewers dispatched together, each building at cargo's default job
  count on a 32-core machine with 30 GB of memory, ran it out of memory; the restart stopped all
  eleven, nine were dispatched again, and `CARGO_BUILD_JOBS=3` let the batch finish. The worktrees
  and scratch directories of every batch were made and removed by shell loops (4, 11, 3, 1 and 1).
  W3 blames `skill@knowledge-architect-review@review-invariants`, which gives each building subagent
  its own worktree and target directory and says nothing of how many build at once. Proposal (W3
  with C1): the command the project's issue names, bounding the build jobs as it makes the
  worktrees, the invariant pointing to it (+1). The tmp-cleanup file made its reviewers' worktrees by
  shell loops; the gates file's project side made them by hand and left 32 GB of builds in its
  scratch directory, which needs the owner's deletion, and proposes an instance.
- **b. Whether it still applies.** Yes. The worktree bullet of
  `path@agent-skills@content/skills/review/SKILL.md`, near line 123 on main and 124 at 8c994ace,
  is unchanged, and a grep of that file for `CARGO_BUILD_JOBS` and for "build jobs" returns nothing
  on main. No command makes the worktrees: the issue's "What" says neither `klarch --help` nor this
  repository's tool holds one.
- **c. Whether it is already recorded.** Yes:
  `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`
  (`touches`, `respect`, with an owner question), a `todo` on the roadmap, whose "What would close
  it" opens "The owner takes the task in or declines it" and names a command that can "report the
  disk and bound the build jobs a batch of builds needs". This file's instance, the memory and the
  build bound, is in it since bb93531b "[review] The memory a batch of reviewers' builds took is an
  instance of the issue on the review's worktrees", on main. `goal@tasks-a-tool-does-better-are-the-tool-s`
  and `goal@generic-tools-and-procedures-are-shipped` (both `constrains`, `respect`, with owner
  questions): "A task is taken in by a decision of knowledge-architect's owner, never because the
  workflow asks it."
- **d. Its kind.** A defect, missing: a task whose result follows from the axes, the commit and the
  machine with no judgement, carried by hand at every batch, and one run lost to it. The command is
  the recorded issue's.
- **e. Its scope.** The tool's side, a command of this repository or of the published crates, once
  taken in; the review skill's invariant would point to it. Whether the task is taken in is reserved
  to the owner by `goal@the-owner-decides`, "whether and when it takes in the generic part of a tool
  or a procedure".
- **f. A better fix.** The file's rival, a sentence in the invariant asking the dispatcher to bound
  the jobs (+1), leaves the bound to be computed by hand at every dispatch, the task the issue
  gives a command; it loses as the file says. No change to the text now: the invariant cannot point
  to a command that does not exist, and the only text that would serve before it is that rival. One
  fix for the cluster: the issue's command, which makes the worktrees and scratch directories, bounds
  the jobs and reports and removes the disk, covers all four members. Two instance lines, the
  tmp-cleanup branch and the gates branch with its 32 GB, are in
  the intake's draft instances-D (not committed; its text is in the issue). A shared build directory, which
  would cut both the memory and the disk, is no rival: here every target is tied to its checkout,
  and `design@knowledge-architect@a-build-is-tied-to-its-checkout` keeps the separate directory of
  the review invariant "for the rebuilds it saves and for the build lock". The issue's own
  sentence ruling it out cites `design@check@a-foreign-build-is-refused` as well, a reason that does
  not hold in this repository, where every build is tied; its repair is a wording fix of its own,
  row 2 of `primer@met-outside-the-task`, written in the same instance file.
- **g. Its route.** An issue only, the work needing a branch of its own once the owner takes it in.
- **h. The proposal.** The instances added to the recorded issue. Whether the task is taken in now
  is the owner's: taken in, the issue's command becomes work to schedule; declined, the issue is
  deleted with that reason, per its close.

Default: the two instances added to `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`, which stays open on the roadmap; no text added now.
Outcome: for the owner (whether the owner takes in the task of a command that makes a batch's worktrees and scratch directories, bounds its build jobs and removes them, which `goal@the-owner-decides` reserves to the owner): the task taken in, the issue staying open on the roadmap as the tool's work, with the two instances. The owner: "6,7,8 should be taken in."

## issue-entry-exempts-design-grounding: whether an issue entry's closing work exempts an owner's request from the design skill

Members: W4 of 2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow.

- **a. What it says.** The owner asked to solve an issue entry whose "What would close it" stated
  the work, a test per recorded mutation. The session read it as stating the request and did not
  ground the whole task in the design skill; it met the work's one open decision separately, on the
  in-change path. The file blames the primer's entry for the design skill, "before starting a
  change the owner asks for or proposes, unless a plan document, an outcome the owner ruled in an
  audit, an intake or a review, or a design head or a failing test already states what the owner
  asks for or proposes", and proposes either adding "an issue entry's closing work" to the list or
  stating that an issue entry does not exempt; "Which one is a ruling on scope for the owner."
- **b. Whether it still applies.** The text is the same: `path@agent-skills@content/PRIMER.md`,
  lines 573 to 578 on main and at 8c994ace, with "or the change decides nothing (a typo, a stale
  pointer, a rewording)" after the list.
- **c. Whether it is already recorded.** Yes, as a decision: `design@agent-skills@settled-means-recorded`,
  graded `conflicts`, `check-reversal`. The head states the same list and its argument: "Each
  exception is a record the session can look at, or a change it can see decides nothing. A record
  settles what the owner asks for or proposes, not merely the result a change must produce: a
  failing test states a result, and a shape the owner proposes to reach it is still challenged." Its
  rejected rivals, `rejected@agent-skills@trigger-judges-settled-design` (`constrains, respect`),
  a trigger asking the session to judge before the load whether a design is settled, and
  `rejected@agent-skills@design-load-unconditioned` (`constrains, respect`).
- **d. Its kind.** A lapse of the session, and no question for the owner. The list is closed, each
  member a kind of record, and an issue entry is not among them, so the text refuses the skip rather
  than leaving it open. The head argues why: an issue entry's "What would close it" is written by a
  session, mostly states a result, and for a `design` entry names rivals unranked, so it settles no
  shape the owner asks for, as a failing test does not. The session's reading, that an entry whose
  closing work decides nothing is exempt, is the second exception already, "the change decides
  nothing", which the work did not meet, since it held one open decision.
- **e. Its scope.** The installed primer and the head's decision. The widening would reverse a
  statement of the head; the record holds the argument against it.
- **f. A better fix.** The file's first fix, adding an issue entry to the list, contradicts the
  head's argument and goes through `skill@knowledge-architect-decision-recording@reversal-check`
  only with a new argument, which the file does not bring: its instance is the case the head
  describes, a result stated and a decision still met. Its second fix, a sentence saying an issue
  entry does not exempt, restates what a closed list says and is a rule written for one
  interaction. The premise that the choice is "a ruling on scope for the owner" is false: no goal is
  to set, and the record ranks the two by the head's argument, so `primer@question-authority` gives
  it to the session. No change.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change, a lapse; the record decides the question.
Outcome: decided by the session: no change; the file's two fixes are ranked by the record, not the owner's. Ground: `design@agent-skills@settled-means-recorded` keeps the closed list, "A record settles what the owner asks for or proposes, not merely the result a change must produce"; an entry whose work decides nothing is already exempt as a change that "decides nothing"; `primer@question-authority` gives what the record settles to the session.

## implementer-skips-project-skill: an implementer or a brief misses what the development skills add

Members: the standing answer on a project skill's addition of
2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow; the same answer of
2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow; the same answer of
2026-11-25-knowledge-architect-intake-items-divided-klarch-workflow. Three retrospectives.

- **a. What it says.** Three misses at the moment an agent's work meets a skill that adds to
  another: in the mutation survivors, one implementing agent's grounding under the development
  skills did not read the entries citing the file it added tests to, among them an issue whose
  closing occasion the work met, found by three reviewers; in the record search branch, one
  implementer did not load `skill@klarch-development` for two comment lines in Rust, with nothing
  following from it; in the intake of items divided, the first brief the session wrote, the text of
  the intake skill's subagent section, named sections of the installed skill and not the project
  skill that adds to it, found by reviewers and repaired. None proposes a fix.
- **b. Whether it still applies.** The first: `path@agent-skills@content/skills/development/SKILL.md`,
  "Ground before editing", on main and at 8c994ace: "read what the project decided about it, as the
  primer's "Read the record of code before changing it" says: the entries its comments cite, every
  entry that cites its file". The second: the root `CLAUDE.md`, `instructions@verify-before-relying`,
  and the description of `skill@klarch-development`, both unchanged. The third is repaired: at
  d729c1c7, the branch's commit that wrote it, the subagent section of
  `path@agent-skills@content/skills/retrospective-intake/SKILL.md` briefed each agent with "the
  findings, the files to read," and named sections of the skill; at 31ea157f, on main, "this skill
  and each skill of the project that adds to it, to load whole" (`git show` of the section at both).
- **c. Whether it is already recorded.** The second is an instance of
  `issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`,
  graded `trigger-met`, `rule-before-work`: its re-entry point is "the next instance, or the
  owner's word", which this instance meets, taken in the next section. The search graded
  `design@agent-skills@no-routing-table` and `rejected@agent-skills@root-routing-table`
  `constrains, respect`: no outcome adds a table mapping project skills to installed ones.
- **d. Its kind.** The first: a lapse of the installed development skill's grounding, which names
  the entries citing the file; once. The second: a lapse of a clear instruction, the recorded
  issue's kind of instance. The third: a defect of the text the branch wrote, repaired in it.
- **e. Its scope.** This repository's agent configuration for the second; the installed text for the
  first and the third.
- **f. A better fix.** One fix for the cluster would be a delivery at the moment of the edit, which
  the recorded issue weighs, three rivals unranked; it is no fix of the first miss, which the skill
  loaded did deliver, nor of the third, repaired. Rival: no change for all three; it holds for the
  first and the third, and the second is the issue's.
- **g. Its route.** Nothing for the first and the third; the second goes to the issue's section
  below.
- **h. The proposal.** No change for the first (a lapse) and the third (repaired on main by
  31ea157f); the second as the section `rust-edit-issue-reentry-met` says.

Default: no change for the first and the third; the second is the instance of the next section.
Outcome: decided by the session: no change for the mutation survivors' miss, a lapse of the grounding of `skill@knowledge-architect-development`, and for the intake's brief, repaired by 31ea157f on main; the record search's miss goes to the section on the issue it is an instance of. Ground: the development skill's grounding names "every entry that cites its file"; the subagent section of `path@agent-skills@content/skills/retrospective-intake/SKILL.md` names "each skill of the project that adds to it" on main.

## rust-edit-issue-reentry-met: the issue's re-entry point is met

Members: the standing answer on a project skill's addition of
2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow, against
`issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`.

- **a. What it says.** One implementer of the record search branch did not load
  `skill@klarch-development` for two comment lines in Rust; nothing followed from it.
- **b. Whether it still applies.** The two deliveries the issue names, both read at a session's
  start, are unchanged on main: the root `CLAUDE.md`, `instructions@verify-before-relying`, and the
  skill's description.
- **c. Whether it is already recorded.** The issue, a `design` entry, holds three instances, the
  suspected mechanism "an instruction read at the session's start is not recalled at an edit of
  Rust made inside a task that looks like text work", three rivals "none ranked by the record" (a
  pointer in each crate's scoped `CLAUDE.md`, a sentence where the three tasks are delivered, no
  change), and "Re-entry point: the next instance, or the owner's word." The search graded it
  `trigger-met`, `rule-before-work`, with the owner question of which delivery the project takes.
- **d. Its kind.** A standing entry whose re-entry point the file meets: a fourth instance, by a
  dispatched implementer rather than the session.
- **e. Its scope.** This repository's agent configuration; the issue names
  `goal@the-workflow-leads-to-better-code` as what an edit made without the grounding threatens.
- **f. A better fix.** The re-entry raises the issue's discussion; it does not settle it. This
  instance adds a fact to it: the edit was made by an agent under a brief, which is one of the three
  homes the issue's second rival names. The issue lists its rivals as "none ranked by the record",
  and its first rival rests on "an assumption to measure", whether the harness loads a scoped
  `CLAUDE.md` at such an edit, which a measurement answers, per `primer@question-authority`. So no
  fork stands yet to put to the owner: the discussion that the issue's "What would close it" names
  argues the rivals, takes that measurement, and puts to the owner only what survives it. It needs a
  branch of its own, since it changes this repository's agent configuration under
  `skill@knowledge-architect-design`. When it is held is the order of known work, which the roadmap
  holds and the owner sets; the intake schedules nothing.
- **g. Its route.** An issue only: the instance line, written in
  the intake's draft instances-D (not committed; its text is in the issue); the discussion on a branch of its
  own.
- **h. The proposal.** The instance added, with the re-entry point recorded as met; the discussion
  is held in a branch of its own, its fork, if one survives, put to the owner there.

Default: the instance added to the issue, its re-entry point met; the discussion held in a branch of its own under `skill@knowledge-architect-design`, the issue open until it lands.
Outcome: decided by the session: the instance added to `issue@agent-config@a-rust-edit-inside-another-task-is-made-without-the-development-skills`, its re-entry point met, and the issue open for the discussion on a branch of its own. Ground: the issue's "What would close it" is "A design discussion under `skill@knowledge-architect-design` that picks a delivery"; its first rival rests on an unmeasured assumption, so no fork is yet left for the owner, per `primer@question-authority`; the order of the work is the roadmap's.

## searcher-grade-refuted-later: a searcher's grade refuted later by reviewers or by cross-checkers

Members: the standing answer on missed entries, its first item, of
2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow; the same answer, its first
item, of 2026-11-16-knowledge-architect-record-search-workflow-klarch-workflow. Two
retrospectives.

- **a. What it says.** In the mutation survivors, the head a test comment cited, which defeated a
  premise of Q1, was graded `unrelated` against a draft whose rule it contradicts, and five
  reviewers found it. In the record search branch, `design@agent-skills@work-divided-by-judgement`,
  `design@agent-skills@design-audit-checks-fit` and `goal@serves-one-kind-of-project` were graded
  `constrains, respect` and refuted to `conflicts` by the experiment's cross-checkers; the session
  missed them because its compact lines left out the searchers' reasons. No proposal.
- **b. Whether it still applies.** The record search branch's cause of the session's miss is
  repaired on that branch: `design@query@light-line-keeps-its-reason` keeps "its quotation and its
  reason" in each light line, harvested in 07e5db31, on main. What remains, in both files, is the
  searcher's grade itself.
- **c. Whether it is already recorded.** `design@agent-skills@premortem-as-watch-points`
  (`constrains, respect`) says the retrospective's question on a missed entry "tells a searcher's
  judgement from an entry or a part of the work that came after the search, the last of which
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it` records instances of": the
  first cause has no entry. `design@query@worklist-bound-is-40000-words` (read with `cargo klarch
  show`) says "No measurement of recall stands behind any value of the bound; real sessions measure
  it". `rejected@agent-skills@verdicts-verified-in-the-workflow` (`constrains, respect`) rejects a
  refuting stage the workflow runs; `design@agent-skills@synthetic-evidence-not-built`
  (`constrains, respect`) has the experiment's refutations count for the part a reading confirms.
- **d. Its kind.** An observation that is evidence for an unmeasured property, the search's recall:
  two retrospectives, four entries. Neither file shows an instruction the searcher did not follow.
- **e. Its scope.** The installed searcher and the record search's design; `goal@records-reach-their-reader`.
- **f. A better fix.** No change to the searcher's text: no instance shows a cause a text holds, and
  the refuting stage that would catch such grades is a rejected design. Rival: no record at all;
  it loses on the bound's head, which asks real sessions to measure recall, while a miss by
  judgement reaches no entry and is read again only in the archived files. An issue that collects
  these misses gives the measurement a place. Related, for the session that assembles the
  analyses: the tripwire graded `unrelated` in the cluster `tripwire-reentry-graded-unrelated` is a
  miss of the same cause.
- **g. Its route.** An issue only.
- **h. The proposal.** Open an issue of kind `design` in agent-skills, `issue@agent-skills@a-searcher-misjudges-an-entry-the-work-contradicts`,
  with both instances, the mechanism as an assumption, and a re-entry point at the next discussion
  of the searcher's grading or of the bound, or a third instance.

Default: open the issue `issue@agent-skills@a-searcher-misjudges-an-entry-the-work-contradicts`, kind `design`, in agent-skills.
Outcome: decided by the session: open an issue, `issue@agent-skills@a-searcher-misjudges-an-entry-the-work-contradicts`, kind `design`, in agent-skills. Ground: `design@agent-skills@premortem-as-watch-points` names a searcher's judgement as a cause of a missed entry that no entry collects, and `design@query@worklist-bound-is-40000-words` asks real sessions to measure the recall no measurement stands behind; the record search branch's own cause is repaired by `design@query@light-line-keeps-its-reason`.

## reword-and-fold-have-no-command: rewording an older message is a script each time

Members: C2 and the standing answer on scripted tasks (its reword part), of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `reword-and-fold-have-no-command`, which gives the outcome.

## verify-before-relying-lapses: an unchecked premise told to the owner

Members: the standing answer on primer rules, its first item, of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `verify-before-relying-lapses`, which gives the outcome.

## past-one-read-trigger-met-by-any-edit: the split rule applied through an issue whose occasion is any test added

Members: the standing answer on rules about code, tests or splitting, of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-16-knowledge-architect-committed-transcripts-klarch-workflow.md`, in its section on `past-one-read-trigger-met-by-any-edit`, which gives the outcome.

## session-carried-agent-work: work the session carried itself

Members: the standing answer on work an agent could have carried, of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `session-carried-agent-work`, which gives the outcome.

## search-misses-work-added-after-it: the issue on the binary's test file bore on parts the search's work did not hold

Members: the standing answer on missed entries, its second item, of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `search-misses-work-added-after-it`, which gives the outcome.

## review-cost-answers: the review rounds, repair commits and owner corrections

Members: the standing answer on review rounds, of this file.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-thaum-announced-store-slice-klarch-workflow.md`, in its section on `review-cost-answers`, which gives the outcome.

## parallel-agents-combined-by-cherry-pick: repairs split over parallel agents by area and cherry-picked

Members: the standing answer on tools, its first item, of this file. Its second item, the build-job
cap, is in `batch-worktrees-and-build-bounds` above.

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-15-knowledge-architect-intake-forty-seven-files-klarch-workflow.md`, in its section on `parallel-agents-combined-by-cherry-pick`, which gives the outcome.

## standing-2026-11-16-knowledge-architect-mutation-survivors-klarch-workflow: the standing answers that report nothing

Members: the standing answers of this file on an installed file changed ("No"), on a reference or a
path without backticks ("No", scratch paths written as the commands that take them), on an agent
that returned unfinished ("None", the nine reviewers lost to the restart being in
`batch-worktrees-and-build-bounds`), and on a conclusion that proved wrong or was verified twice
("No", each report verified once, by re-running a sample of its mutations or reading its logs).

- **a. What they say.** Each answers "No" or "None", with the reason quoted above.
- **d. Their kind.** Observations: nothing to act on. The answer on paths written as commands names
  no checked form missing, so it is no finding on the checker.
- **h. The proposal.** No change.

Default: no change, observations.
Outcome: decided by the session: no change, observations. Ground: each answer reports nothing to act on, and none names a missing checked form, per the question on references of `skill@knowledge-architect-retrospective@what-it-examines`.
