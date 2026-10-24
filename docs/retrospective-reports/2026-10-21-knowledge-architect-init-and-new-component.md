# 2026-10-21-knowledge-architect-init-and-new-component

- **Received file**: 2026-10-21-knowledge-architect-init-and-new-component.md, this repository's
  project file of the session that designed, specified, built and merged `klarch init` and
  `klarch new-component`.
- **Version used**: built from its own source, main at `v0.8.0`, commit `7da68c9`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`, which holds the session's merged branch.
- **Standing entries that bear on it**:
  - for P1: `issue@core@the-tests-read-the-machine-s-git-configuration`,
    `tripwire@core@walked-count-differs-between-machines`, not met: no walked-file count differs;
  - for P2 and W2 of the workflow file: `design@knowledge-architect@git-flow`,
    `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`,
    `design@agent-skills@standing-state-review-before-every-merge`,
    `design@xtask@gates-list-primary-home`, `design@xtask@ci-builds-the-tip-alone`,
    `goal@xtask@one-command-runs-every-gate`,
    `tripwire@agent-skills@record-answered-questions-reach-the-owner`.

## P1. A new test file's git invocations read the machine's git configuration

- **a. What it says.** The new test file of `init` used helpers of its own, not `Sandbox`. One case,
  which relies on git refusing a repository as another user's, passed on the workstation and failed
  in CI; it reproduced locally under a global configuration holding `safe.directory = *`. The fix
  isolated that one invocation. The fix proposed for the cause: no text; close the open issue with
  one shared helper that runs git and the binary with `GIT_CONFIG_NOSYSTEM=1` and an empty
  `GIT_CONFIG_GLOBAL`.
- **b. Whether it still applies.** Yes. `path@core@CLAUDE.md`, lines 40-43, the same at `7da68c9`:
  "A test that runs git through `Sandbox` never depends on the machine's git identity … Other
  machine configuration still reaches some helpers, per
  `issue@core@the-tests-read-the-machine-s-git-configuration`." The commit "The test of a repository
  git refuses as another user's keeps the machine's git configuration out" touches only
  `path@core@tests/init.rs`: the one run that sets the owner-test variable gets the two variables,
  and its comment cites the issue. The file's other helpers, its own `klarch`, `git` and
  `klarch_env`, run with no isolation, and the two variables appear nowhere else in the core's
  tests or source.
- **c. Whether it is already recorded.** Yes: `issue@core@the-tests-read-the-machine-s-git-configuration`,
  open, a defect. Its close is the proposal's helper, "or a `GIT_CONFIG_GLOBAL` pointing at an empty
  file with `GIT_CONFIG_NOSYSTEM=1`", and it also asks that `path@core@CLAUDE.md` state the contract
  for every helper. The issue predates the new test file and does not name it; this CI failure is
  its first failure in real use, where the issue's own counts came from probes.
- **d. Its kind.** Already recorded; this is a new instance and a new site.
- **e. Its scope.** The core's tests; no goal is threatened beyond the gate's trustworthiness the
  issue names.
- **f. A better fix.** Rivals: close the issue now, the file's proposal, which is Rust work under
  `skill@klarch-development` of a size its issue already scopes; add the instance and the site to
  the issue, and leave its closing to a branch of its own; a sentence in the scoped CLAUDE.md, which
  the issue's own close already includes after the helper.
- **g. Its route.** An edit of the issue entry, under `skill@knowledge-architect-issue-tracking`.
- **h. The proposal.** Handle now: the issue gains `path@core@tests/init.rs` as a site and this CI
  failure, with its local reproduction, as an instance.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## P2, with W2 of 2026-10-21-knowledge-architect-init-and-new-component-klarch-workflow. A commit that lands after the review before the merge, and CI that first runs after it

- **a. What it says.** Two commits landed after the transcript review that repaired no review's
  findings: one wrote two goals and opened an issue, on the owner's ruling of a question the review
  raised; the other fixed a test that failed in CI, which first ran after the pull request was
  marked ready. No instruction says what review either owes, and the session landed both with the
  gates alone. W2 proposes that a commit landing after the review before a merge be a repair for this
  purpose, reviewed by each axis whose condition it meets. P2 notes that running CI on a draft would
  remove the late discovery, at a run per push, and that the head stating that CI does not run on a
  draft gives no reason.
- **b. Whether it still applies.** Yes. `skill@knowledge-architect-review@review-axes`, the same at
  `v0.8.0` and on main: "The review before a merge is the last review of a branch, after which only
  its own repairs land". The two commits are on main after the last review commit: "A Rust project
  gets every feature, and a project in another language a reduced mode", and the test fix named
  under P1. `design@knowledge-architect@git-flow`: "CI does not run on a draft." The CI workflow,
  ci.yml under .github/workflows, triggers on `ready_for_review` among others, and its job carries the
  condition that the pull request is not a draft; its comment explains why a skipped job counts as
  passing, and no reason for skipping drafts. Both the condition and the head's sentence come from
  the commit that imported the checker into this workspace, whose message says nothing of drafts.
- **c. Whether it is already recorded.** No entry records either case. Related:
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, under which a repair that
  adds, removes or reverses no head "is reviewed by no axis again"; the merge predicate of
  `instructions@git-workflow`, which reviews a line the rebase changed "as a repair is", a
  precedent for a change after the review; `design@xtask@ci-builds-the-tip-alone`, which names the
  cost of a CI run, "minutes per run on the runner".
- **d. Its kind.** W2: unclear. "Only its own repairs land" states what happens, and a commit that
  is not a repair has no rule. P2: a fork the owner holds, between a CI run per push of a draft and
  a late discovery; and a head whose rule carries no recorded argument.
- **e. Its scope.** `goal@xtask@one-command-runs-every-gate` holds either way, since CI runs the
  same command. W2 is installed text, the review skill; P2 is this repository's root design home
  and its CI file.
- **f. A better fix.** For W2, rivals: W2's proposal, which reads as wider than the head on
  re-review where it says "each axis whose condition it meets"; the same commit made a repair of the
  review, so that the head on re-review applies to it as written; the whole review run again, which
  W2 rules out on cost. A commit that lands after the review on an owner's answer to an item of it,
  or to meet the gates before the merge, is the review's own outcome, so the second rival changes no
  decision: it names a member of "its own repairs" that the head's argument covers. For P2, rivals:
  CI on drafts, which changes `design@knowledge-architect@git-flow`, `design@xtask@gates-list-primary-home`,
  the CI file and points 1, 4 and 5 of `instructions@git-workflow`; no change, the gates run
  locally before the merge, and a failure that only CI's machine shows is found when the pull
  request is marked ready, as here; an issue that asks for the draft rule's argument, which lets the
  owner weigh the fork with the cost measured.
- **g. Its route.** W2: an installed text edit under `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent". P2: an issue only.
- **h. The proposal.** Handle now, for W2: `skill@knowledge-architect-review@review-axes` says that
  a commit landing after the review before a merge, on the owner's answer to an item of it or to
  meet a gate, is one of its repairs, and `skill@knowledge-architect-review@what-review-leaves`
  then governs it. Instructions added: 0, a clause that names what "its own repairs" covers. For P2:
  open an issue, a `question` in the root Component, on whether CI runs on a draft, with this case
  and the head's missing argument; the rule stays as it is meanwhile.
- **Default**: handle now for W2; open an issue for P2, as h says.
- **Outcome**: accepted.
