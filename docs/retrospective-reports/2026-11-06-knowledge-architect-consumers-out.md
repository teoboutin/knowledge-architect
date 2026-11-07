# 2026-11-06-knowledge-architect-consumers-out

- **Received file**: 2026-11-06-knowledge-architect-consumers-out.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-160-gcd748d2b`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes`, `issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`.

## P1: `gh pr ready` right after a push lost the real CI run

- **a.** What it says: the session checked that GitHub held the push, then ran `gh pr ready`; the
  ready event started a run, and a second run for the same head, which skipped, cancelled it within
  two seconds through the workflow's `cancel-in-progress`: "Canceling since a higher priority waiting
  request for ci-49 exists". The merge predicate caught it; the session re-ran the cancelled run.
  Proposal: name the case at the predicate, or keep the skipped job out of the concurrency group,
  which removes the cause (adds 0); rival, leave it.
- **b.** Whether it still applies. Yes. `path@knowledge-architect@.github/workflows/ci.yml`, lines 13
  to 34 at `cd748d2b` and at HEAD (no commit since has touched it): the run triggers on `opened`,
  `synchronize`, `reopened` and `ready_for_review`; the group is
  "ci-${{ github.event.pull_request.number }}" with "cancel-in-progress: true"; and the job runs only
  "if: ${{ !github.event.pull_request.draft }}". So a run of the push's event, whose payload says
  draft, joins the same group as the ready run, cancels it if it starts later, and then skips.
  Assumption, not measured here: the later start of the push's run is an order of event delivery
  that GitHub does not guarantee, which the run's message shows happened once. The comment at the
  block gives its reason: "A later push to one pull request cancels the run on the earlier head".
  The group and its setting date from the commit "Restructure the imported checker into the
  knowledge-architect workspace" (`831519e8`).
- **c.** Not recorded. `design@knowledge-architect@git-flow` states the draft rule and the merge's
  wait for a successful run, and names no concurrency; no head or comment argues that a draft's run
  must share the group. `issue@knowledge-architect@ci-does-not-run-on-a-draft-and-no-record-says-why`
  bears on it: if CI ran on drafts, the push's run would test the same head instead of skipping.
- **d.** Its kind: wrong, a defect of the CI configuration; one instance.
- **e.** Its scope: `design@knowledge-architect@git-flow`'s merge on a successful run; owner: the root
  Component's CI workflow. The fix keeps the comment's reason, since a push to a pull request that is
  ready still cancels the earlier head's run, and reverses no decision.
- **f.** A better fix. The rivals: naming the case at the predicate adds an instruction for a race
  the configuration causes; leaving it costs a re-run each time it recurs, and a session may wait on
  a cancelled run. The fix at the cause: the draft flag in the group's key,
  "ci-${{ github.event.pull_request.number }}-${{ github.event.pull_request.draft }}", so that a run
  whose payload says draft, which skips, never cancels a run of a ready pull request. A setting of
  `cancel-in-progress` by the draft flag is the nearest rival, and it loses: the draft run would
  still join the group and queue behind the ready run. Assumption: the effect is shown only by a run
  on GitHub, at the next pull request made ready right after a push.
- **g.** Its route: a small edit of this repository's own CI file, with its comment.
- **h.** The proposal: handle now; the comment says why the key holds the draft flag.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: the cause is in
  `path@knowledge-architect@.github/workflows/ci.yml`, and the fix removes it at 0 instructions, the
  comment's reason kept; the merge predicate of `design@knowledge-architect@git-flow` is unchanged.

## P2: the milestone's list of what a new crate owes misses two lists

Analysed with P1 of 2026-11-01-knowledge-architect-release-0-9-0, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-release-0-9-0.md`.

## P4: the root's thaum rule lists Components, and drifted

- **a.** What it says: `instructions@verify-before-relying` said "A figure in the documents of
  `check` or of `model` measured in another project"; an audit reader found the figures had moved to
  `query`; repaired by adding a third name, which drifts the same way at the next move. Proposal:
  "in the documents of any Component" (adds 0). Rival: the list, which no reason favours.
- **b.** Whether it still applies. Yes. `path@knowledge-architect@CLAUDE.md`, line 82 at HEAD: "A
  figure in the documents of `check`, of `model` or of `query` measured in another project, thaum,
  names thaum as its source." Its list grew with each move of the core's text: the commits "[model]
  The model, the scan, phases 1 to 3 and the resolver become the crate knowledge-architect-model"
  (`d2793f99`), "[check] The prose that names the core names `check`" (`37cf06bc`) and "[query] The
  commands over the record, the transcript reader and the project creator leave the checker, with
  their record" (`3c8c6125`). Its scope came from the sentence the owner worded at the commit "The
  root CLAUDE.md drops two mentions of thaum and introduces the one that stays" (`58e0f8a8`): "the
  core's documents were first written in another project, thaum; a figure measured there names thaum
  as its source". The premise of that scope, where the text first written in thaum sits, moved with
  the split; and a head outside the list cites thaum's figures, as
  `design@agent-skills@application-judged-by-diff` does, naming thaum.
  `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes` quotes the sentence
  verbatim.
- **c.** Not recorded as a defect; the issue above quotes it.
- **d.** Its kind: wrong, a list that drifts with each move of the record.
- **e.** Its scope: `goal@knowledge-architect@agents-work-without-drift`; owner: the root's own text.
  It reverses no head. The owner worded the sentence at `58e0f8a8`; its scope named where the text of
  thaum's origin was, a premise the split made false, per `primer@owner-word-premise`, and the three
  later commits widened it without the owner.
- **f.** A better fix. The rival, the list kept, drifts at every move. The fix: "A figure measured in
  another project, thaum, names thaum as its source, in any document of this repository". Instructions:
  +0, −0.
- **g.** Its route: a text edit of the root, `skill@knowledge-architect-agent-configuration`, with the
  issue's quotation updated in the same commit.
- **h.** The proposal: the owner worded the sentence on the premise that the core's documents were
  first written in thaum, and the split into Components made it false: the text of that origin now
  sits in several Components, and three later commits widened the list without the owner. A ruling
  of the owner whose premise proves false goes back to the owner, per `primer@owner-word-premise`.
  The corrected premise: a figure measured in thaum can sit in any document of this repository. The
  default: the sentence reads "A figure measured in another project, thaum, names thaum as its
  source, in any document of this repository", with the issue's quotation updated in the same
  commit. Nothing of it is built before the owner's word.
- **Default**: the sentence widened to any document of this repository, once the owner rules.
- **Outcome**: for the owner (a contradiction: a ruling of the owner whose premise proved false):
  handle now, the sentence widened to any document of this repository. The owner: "Q3 agreed"
  Then, at the review before the merge, the owner asked whether the line needs to name thaum, and
  whether the installed text already covers it. The session first read the primer as covering it,
  which the owner refuted: no installed rule names the project a figure was taken in. The owner:
  "Delete it. But the primer does not hold the exact instruction of naming which project it came
  from. I think it would be better if it did." Carried out: the root line is deleted, and the
  primer's rule on a number asks for the project it was taken in where that is another project.

## P5: a push of a stale ref

Analysed with P1 of 2026-11-01-knowledge-architect-development-milestone-plan, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan.md`.

## P3: the milestone has the session draft the goals

Analysed with W1 of 2026-11-06-knowledge-architect-cli-on-top-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-06-knowledge-architect-cli-on-top-klarch-workflow.md`.

## P6: what the owner holds about the session's reach

Analysed with W1 of 2026-11-06-knowledge-architect-consumers-out-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-06-knowledge-architect-consumers-out-klarch-workflow.md`.
