# 2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow

- **Received file**: 2026-10-19-knowledge-architect-intake-and-evaluation-klarch-workflow.md, the
  workflow file of this repository's session that ran an intake of thirteen received files, carried
  out its outcomes and evaluated the intake.
- **Version used**: built from its own source, main at `v0.7.0-62-g6e2b300`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`.
- **Standing entries that bear on it**:
  - for W1: `a-retrospective-version-line-names-a-commit-the-repository-does-not-hold`,
    `design@agent-skills@version-line-names-the-main-commit`, `design@agent-skills@retro-trigger`;
  - for the cluster under W2: `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`,
    `design@agent-skills@transcript-review-last-before-merge`,
    `design@agent-skills@transcript-review-repairs-are-additional-commits`,
    `design@agent-skills@default-ruled-before-merge`,
    `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`,
    `issue@agent-skills@the-retrospective-counts-no-review-cost`;
  - for the standing answers: `tripwire@agent-skills@attribution-loses-the-scope`,
    `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`,
    `design@agent-skills@verification-in-primer`, `tripwire@agent-skills@rarer-rule-in-the-model`.

## W1. The version line's command names a different commit once the branch has merged

- **a. What it says.** The retrospective is offered when a branch has merged, and runs on the main
  branch then; `git describe --tags $(git merge-base HEAD <main branch>)` printed the merged result
  where the session's tree had contained the commit the branch started from. The session named the
  second by judgement. The fix proposed: the line names the main commit the session's tree contained
  when its work began, run before the merge or taken at the session's start.
- **b. Whether it still applies.** Yes, unchanged since `6e2b300`.
  `skill@knowledge-architect-retrospective@what-it-examines`: "the main commit the session's tree
  contains, described with `git describe --tags $(git merge-base HEAD <main branch>)`", and the same
  command with `git rev-parse --short` for the project's file. The merge predicate of
  `instructions@git-workflow` ends with `git checkout main && git pull --ff-only`, so a retrospective
  run after it has HEAD on main, which holds the branch. Confirmed: `git describe --tags` on the
  reported branch's last commit gives the line the command printed.
- **c. Whether it is already recorded.** In part.
  `a-retrospective-version-line-names-a-commit-the-repository-does-not-hold`
  records version lines naming SHAs the repository does not hold, and its suspected mechanism is "the
  session described a local main branch that held the branch's own commits, as a merge done locally
  before GitHub's rebase merge would, whose SHAs the rebase merge then replaced. Not reproduced." This
  finding reproduces its first half by reading: run after a merge, the command names the merged
  result, a SHA a later rebase merge replaces. The issue's close: "the retrospective computes the
  line in a way that survives the merge".
- **d. Its kind.** Wrong: the command's result depends on when it runs, and the trigger of
  `design@agent-skills@retro-trigger` runs it after the merge.
- **e. Its scope.** `goal@knowledge-architect@the-workflow-improves-through-real-use`; installed
  text, the retrospective skill. `design@agent-skills@version-line-names-the-main-commit` names the
  commit "so that an intake reads the blamed text as the session read it"; a line naming the merged
  result makes the intake read text that may hold the session's own repairs, so a finding looks
  repaired before the session met it.
- **f. A better fix.** Rivals: the commit the branch started from, the proposal, which is on the
  main branch before the merge and so survives a rebase merge; no change, the session's judgement,
  which held here and loses where the session does not notice that its main holds its own work; the
  intake's count fallback, which handles a SHA the repository lacks but not a SHA that resolves to
  the wrong commit. The first also meets the open issue's close. Instructions added: 0, a rewording
  of when the command runs.
- **g. Its route.** An installed text edit under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", and the issue closed under `skill@knowledge-architect-issue-tracking`.
- **h. The proposal.** Handle now: the version line names the main commit the session's tree held
  when its work began, the command run at the session's start or on the commit the branch started
  from; and the open issue closes on this repair, with the reading above as its mechanism.
- **Default**: handle now, as h says.
- **Outcome**: now.

## W2, with the findings on how the review before a merge ends

This section analyses W2 of this file, W1 of
2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow, W5 of
2026-10-22-rootpoint-live-control-slice-klarch-workflow and W2 of
2026-10-24-thaum-driven-testing-driven-tests-klarch-workflow: four sessions, four questions on the
end of one review.

- **a. What they say.**
  - W2 here, an observation: each repair that added a head brought the decision-record axis back,
    three passes in all, each finding real defects.
  - W1 of the init file: the owner's rulings on the transcript review's items led to a repair that
    added a head; the decision-record review of it led to repairs that split two heads, so two new
    heads. Whether "that review" in "the review ends with that review's repairs" means the
    transcript review or the follow-up review, the text does not say, and the second reading has no
    bound. The fix proposed: the review ends with the repairs of the axes that review the last
    transcript review's repairs.
  - W5 of the live-control file: an owner's ruling arrived while the last transcript review ran,
    which reported it missing; its repair commit recorded it. Whether that repair needs the
    transcript review again, the skill leaves open; the session judged not.
  - W2 of the driven-tests file: the record of the review was written in the last repair commit
    before the transcript review, since the transcript reviewer reads the outcomes there; the
    transcript review's repair then became the last commit, and the record had to move, by a history
    edit, since one of its sentences was false.
- **b. Whether they still apply.** The texts are unchanged at `v0.8.0` and on main.
  `skill@knowledge-architect-review@what-review-leaves`: "a repair made before the last transcript
  review is reviewed before it … a repair of the last transcript review is reviewed after it, and the
  review ends with that review's repairs. Any other repair … is reviewed by no axis again"; and
  "**A paragraph in the commit message** records the review: the message of the last repair commit,
  or, where no repair commit is left, the message of the branch's last commit".
  `skill@knowledge-architect-review@review-axes`: the transcript reviewer runs "after every other axis
  has run, its repairs are committed and every item of their reports has its outcome".
- **c. Whether they are already recorded.**
  `design@agent-skills@transcript-review-last-before-merge` settles the reading W1 asks about: "An
  additional commit that adds, removes or reverses a design head is reviewed by the decision-record
  axis at least, and the review ends with that review's repairs", where "that review" is the
  decision-record review. `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`
  accepts W2's cost: "Each round of repairs then adds at most as many re-reviews as it adds, removes
  or reverses heads", before the last transcript review. Neither records W5's case or where the
  record goes in W2 of the driven-tests file.
  `issue@agent-skills@the-retrospective-counts-no-review-cost` asks for the review rounds, the repair
  commits and the owner's corrections, and says nothing records them; three of this repository's
  received files carry them, this one, the defined-ids workflow file and, in part, the init workflow
  file.
- **d. Their kind.** W1: unclear, in the skill only; the head states the meaning. W2 here: an
  observation within the head's accepted cost. W5: an observation; the text supports the session's
  reading, and what stays unchecked is the fidelity of a ruling first recorded after the last
  transcript review, one instance. W2 of the driven-tests file: missing, the record's place when the
  transcript review has repairs.
- **e. Their scope.** `goal@knowledge-architect@agents-get-a-complete-workflow`, for fewer review
  rounds; installed text, the review skill.
- **f. A better fix.**
  - For W1, rivals: the skill reworded to the head's reading, the proposal; a fixed number of
    rounds, which the head's trigger already makes one after the transcript review. Instructions
    added: 0.
  - For the record's place, rivals: the record moves into the transcript review's last repair
    commit, the literal reading, which costs a history edit whenever an earlier message held the
    record; the transcript review's outcomes recorded in its own repair commit, and the earlier
    axes' record left where the transcript reviewer read it, which keeps one record per review that
    produced it and edits no history. The second fits
    `design@agent-skills@transcript-review-repairs-are-additional-commits`, under which those repairs
    are additional commits. Instructions added: 0, a rewording of which commit holds which part.
  - For W5: no change, one instance, with the session's reading supported by the text.
  - For the counts: the issue gains them.
- **g. Their route.** An installed text edit of the review skill, and an edit of the issue entry.
- **h. The proposal.** Handle now: `skill@knowledge-architect-review@what-review-leaves` says the
  review ends with the repairs of the decision-record review of the last transcript review's
  repairs; and that where the last transcript review has repairs, the record of its items is the
  message of its last repair commit, the record of the earlier axes staying in the commit that held
  it. `issue@agent-skills@the-retrospective-counts-no-review-cost` gains the counts of the three
  files. No change for W2 here and for W5.
- **Default**: handle now, as h says.
- **Outcome**: now.

## W3. An analysis kept check-clean through a branch that renames and closes what it cites

- **a.** An observation: three rewrites of an analysis's citations as outcomes closed and renamed
  entries; the rule is clear and the cost small.
- **b.** The rule is unchanged: `primer@where-knowledge-goes`, a record of a deliberation names a
  removed or renamed entry by its bare slug, "A renamed entry's old slug carries beside it the
  reference". It is `design@agent-skills@a-past-sentence-is-rewritten`.
- **h.** No change.
- **Default**: no change, an observation.
- **Outcome**: none.

## C1. No checked form names a path at every installing project's root

This finding is analysed with P1 of 2026-10-19-knowledge-architect-intake-and-evaluation, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-19-knowledge-architect-intake-and-evaluation.md`.

## The standing answers: claims in commit messages and in answers to the owner left unchecked, a recurrence

- **a. What it says.** Here, six claims in five commit messages, searches or counts that did not
  reproduce, and a count given to the owner as 9 where 8 held. The same rule, "A commit message is a
  document for this purpose", or its sentence on answers to the owner, is reported acted on as if
  absent in eleven of the twelve workflow files of this intake: this one, the release and the init
  files of this repository, and all eight files from the other two projects; only the defined-ids
  workflow file reports none. Each was caught before the merge, by the session or by a reviewer, and
  repaired by an amend or a history edit, but one claim of the short-deck-soak file, which reached
  the main branch and became true after its commit.
- **b. Whether it still applies.** The rule is unchanged, `primer@verify-before-relying`, "Verify a
  claim before writing it": "Before each commit, list every factual claim its message makes … Writing
  the message from memory of the work is where the false claims come from." It is in the primer,
  read at the session's start; the act it asks for happens at each commit, and no text the session
  reads at the commit restates it. `primer@checker-model` asks for `check --staged` "after staging
  and before each commit", which judges the tree and not the claims.
- **c. Whether it is already recorded.** No entry records the recurrence.
  `design@agent-skills@verification-in-primer` keeps the verification rules in the primer.
  `tripwire@agent-skills@rarer-rule-in-the-model` watches what enters the primer's checker section.
- **d. Its kind.** A lapse that recurs at one point of delivery, the commit, in eleven sessions of
  three projects. Under the lapse rule of
  `skill@knowledge-architect-agent-configuration@admission-standard`, the fix moves where the
  instruction is delivered, and the recurrence is its evidence.
- **e. Its scope.** `goal@knowledge-architect@agents-work-without-drift`; installed text or the
  checker, depending on where the delivery lands.
- **f. A better fix.** Rivals: a line printed by `check --staged` on its verdict, which every session
  runs at that moment, and which changes a command's output and `design@core@phases-gate-the-report`'s
  summary; a sentence of the primer's checker section beside the staged check, which the session
  reads at its start, the same moment as now; a project's commit hook, which the installed text
  cannot rely on; no change, which leaves the cost of an amend or a history edit at most commits of
  every session. The shape is not settled, and each candidate meets a head, so it is a design
  question.
- **g. Its route.** An issue only: the design is not settled, and the work is not this session's.
- **h. The proposal.** Open an issue: a `design` entry in agent-skills, the claim check of a commit
  message delivered at the session's start and needed at each commit, with this recurrence's count
  as its evidence and the candidates above.
- **Default**: open an issue, as h says.
- **Outcome**: issue.

## The standing answers: attributions of the owner

- **a. What it says.** A quotation of the owner in a review commit's message, and a batch-approved
  default attributed to the owner in a head, both caught by reviewers. The same kind of slip, a
  hedge of the owner's dropped or a remark recorded as a ruling, is reported in five other files of
  this intake, the defined-ids, thaum retrospective-intake, reader-refusals, live-control and
  editor-conveniences workflow files, each caught before the merge.
- **b.** The rules stand, `primer@design-heads`: "A durable record attributes a decision to the
  owner; it does not quote the owner", and "An approval is not a ground".
- **c.** `tripwire@agent-skills@attribution-loses-the-scope` fires when "the transcript reviewer
  reports a misstated ruling that traces to an attribution in a durable record, where the owner's
  words would have shown the scope". `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`
  records the owner's ruling that such a slip caught before the merge did not fire it.
- **d.** Lapses, each caught before the merge, by the reviewer whose expected rate
  `skill@knowledge-architect-retrospective@what-it-examines` names. On the owner's earlier ruling,
  the tripwire does not fire on them.
- **h. The proposal.** No change: the tripwire not fired, on the ruling the issue records.
- **Default**: no change, as h says.
- **Outcome**: this probably happens because the planning skill asks for exact quotes, which is probably getting agents confused. this deserves a design issue IMO.
