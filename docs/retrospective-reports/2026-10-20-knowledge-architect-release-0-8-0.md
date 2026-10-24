# 2026-10-20-knowledge-architect-release-0-8-0

- **Received file**: 2026-10-20-knowledge-architect-release-0-8-0.md, this repository's project file
  of the session that released 0.8.0 under `skill@klarch-release`.
- **Version used**: built from its own source, main at `v0.7.0-84-g93558a0`.
- **Analysed against**: main at `v0.8.0-14-ge763a3d`. None of the texts this file blames changed
  between `93558a0` and that commit: `git diff --stat` over them is empty.
- **Standing entries that bear on it**:
  - for P1: `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only`,
    `issue@knowledge-architect@a-mechanical-changelog-check`,
    `design@knowledge-architect@changelog-reviewed-at-the-release`;
  - for P2: `design@knowledge-architect@package-include-whitelist`;
  - for P4: `design@knowledge-architect@changelog-section-fixed-at-release`,
    `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
    `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument`;
  - for every section: `tripwire@agent-skills@real-use-refusal-overruled`, met only if the owner
    overrules a refusal of an addition below.

## P1. No axis of the release review checks that a changelog entry's behavioural claim is true

- **a. What it says.** The 0.8.0 Migration entry on the `rejected` register said a declared
  register whose directory is `rejected` is refused. The claim was true for a register of
  `scope = "component"` only. The changelog reviewer moved the clause and kept it as written, citing
  the code and running nothing; no other axis caught it; the session found it by checking its own
  commit message. The fix proposed: a clause under "Is an entry wrong?" of
  `agent@klarch-changelog-reviewer`, that each statement of what the checker reports, refuses or
  accepts is true at the commit under review, read in the code and, where the code does not settle
  it, run on a copy of a mock project; and a scope sentence widened to allow it.
- **b. Whether it still applies.** Yes. The agent's file is unchanged since `93558a0`. Its scope,
  lines 12-14 of the agent's definition: "**Not** whether a change is right, nor anything the other
  axes of the review judge." Its predicate "Is an entry wrong?", lines 37-41, lists the subsection,
  the surface, the class, the migration's consumer change, the net effect and the order, and no
  truth of the entry's content. Lines 16-18 already say "reproduce anything you assert" and forbid
  any change to the tree, so a run on a mock copy happens outside the tree. The narrowed entry is on
  main, from the commit "[review] Repairs of the release review: a manifest refusal under its own
  surface, the primer's audit axis, the scope of `dir = "."`".
- **c. Whether it is already recorded.** In its general form:
  `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only` says "No axis of the review
  before a merge reads a change's statements about existing code against the tree", and its close
  names a wider condition on the code-claims row. `issue@knowledge-architect@a-mechanical-changelog-check`
  covers the changelog's shape only. `design@knowledge-architect@changelog-reviewed-at-the-release`
  has the changelog reviewed once, at the release, "against the tests of
  `design@knowledge-architect@changelog-entries`"; the clause adds a predicate outside those tests.
- **d. Its kind.** Missing: a check no axis performs, seen in real use, reaching the reviewed head
  of a release branch.
- **e. Its scope.** `goal@knowledge-architect@documentation-stays-consistent`; the text is this
  repository's own agent, owned by the agent-config location. The clause widens the head on the
  release review, which `skill@knowledge-architect-decision-recording` judges: a member its argument
  ("reviewed once, at the release", where every entry of the range is read together) covers, since
  a changelog entry's truth is judged where the entries are read.
- **f. A better fix.** Rivals: the clause in the changelog reviewer, the proposal; the code-claims
  axis widened to every change's statements about code, which the issue above designs for every
  installing project, and which reads a changelog entry on its feature branch, before the release,
  so it would have caught this one earlier; no change, since the session's own claim check found it.
  The deciding fact: the changelog entries of a release are read together only at the release, and
  the changelog reviewer is the one axis there that reads each entry against the range; the general
  issue remains open for the earlier moment. Instructions added: 1, a predicate of an agent.
- **g. Its route.** A text edit of this repository's own agent, under
  `skill@knowledge-architect-agent-configuration`, with the head on the release review read under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: the clause and the widened scope sentence as the file states
  them; `issue@agent-skills@the-code-claims-axis-reads-plan-documents-only` gains this case as a
  second instance.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## P2. Step 7 of the release does not name the three files Cargo adds to every package

- **a. What it says.** Every package list holds `.cargo_vcs_info.json`, `Cargo.lock` and
  `Cargo.toml.orig`, which Cargo generates, so step 7 makes every release report them to the owner,
  and a real stray file would be listed beside them. The fix proposed: name them in step 7.
- **b. Whether it still applies.** Yes. `skill@klarch-release`, step 7: "Each lists its sources, its
  Cargo.toml, its README, its CRATES-IO.md, its licence files and its CHANGELOG.md, and agent-skills
  also its build script, content/ and snippets/ … report to the owner every file the step does not
  name." Measured on origin/main with cargo 1.98.0, `cargo package --list -p <crate>` for each of
  the three published packages, sources filtered out: each lists `.cargo_vcs_info.json`,
  CHANGELOG.md, CRATES-IO.md, `Cargo.lock`, Cargo.toml, `Cargo.toml.orig`, the two licence files and
  README.md, and agent-skills also its build script. No other unnamed file appears. A search of
  origin/main for `cargo_vcs_info` or `Cargo.toml.orig` returns nothing.
- **c. Whether it is already recorded.** No. The head the step cites,
  `design@knowledge-architect@package-include-whitelist`, is silent on them too: "Cargo adds the file
  `readme` names, its CRATES-IO.md, whatever `include` says". Its sentence is a statement of what a
  package holds, and it is short of three files by the measurement above.
- **d. Its kind.** Wrong, provable by measurement: the step's list and the head's account omit
  three files the tool always adds.
- **e. Its scope.** `goal@knowledge-architect@adoption-is-easy`; this repository's own skill and its
  root design home. The head's correction states a fact of the tool and changes no decision.
- **f. A better fix.** Rivals: the step names the three files, the proposal; the step filters them
  out of the comparison by a command, which adds an instruction to a hand step for the same effect;
  no change, which leaves three expected files in every report. The step and the head are corrected
  together, since the step cites the head. Instructions added: 0.
- **g. Its route.** A text edit of `skill@klarch-release` under
  `skill@knowledge-architect-agent-configuration`, and a correction of the head's account under
  `skill@knowledge-architect-decision-recording`.
- **h. The proposal.** Handle now: step 7 names the three files as the ones Cargo generates, and
  the head's sentence on what Cargo adds names them too.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## P3. The search of step 1 finds mentions that are not sites, and the step names one kind only

- **a. What it says.** `git grep` on the previous version found the released changelog sections,
  their crate copies and four issue entries describing events at 0.7.0; the session and a reviewer
  each judged them not to be sites. The fix proposed: the exception sentence names a statement
  about a past version beside an example inside a skill.
- **b. Whether it still applies.** Yes. `skill@klarch-release`, step 1: "`git grep -n '<previous
  version>'` finds every site that still names the previous one. An example inside a skill, such as
  an xtask manifest's own version, is not a site." The count is confirmed at the tag `v0.8.0`: the
  search for `0.7.0` hits CHANGELOG.md, its three crate copies and four issue files. On origin/main,
  a search for `0.8.0` hits the version sites and the four changelogs, so the next release meets the
  changelog kind again.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Unclear: the step's one exception leaves a kind the session meets at every
  release to judgement, twice this time.
- **e. Its scope.** This repository's own skill. No head.
- **f. A better fix.** Rivals: the rewording, the proposal; a narrower search command that skips
  CHANGELOG.md and the issue directories, which would also skip a stale version in an issue's
  instructions. Instructions added: 0, a rewording that names what the exception already means.
- **g. Its route.** A text edit of `skill@klarch-release`.
- **h. The proposal.** Handle now: "A statement about a past version, such as a released changelog
  section or an issue entry's evidence, and an example inside a skill, are not sites."
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## P4. Three texts describe a `Next release` section that a release removes

- **a. What it says.** CHANGELOG.md's preamble, the head on released sections and the root
  CLAUDE.md's knowledge table describe a `Next release` section as always present; after a release
  renames it, none exists. The fix proposed, low priority: "and, while entries await a release, one
  working section above them".
- **b. Whether it still applies.** The three texts are unchanged. CHANGELOG.md, lines 3-4: "One
  section per released version, and one working section above them, `Next release`, which the
  release renames to its version." `design@knowledge-architect@changelog-section-fixed-at-release`
  says the same; the knowledge table of `path@knowledge-architect@CLAUDE.md` says "and the
  `Next release` section". A `Next release` section exists on main now, created by the commit "The
  documentation and the changelog of `klarch init` and `klarch new-component`". So the texts are
  false only between a release and the next branch that writes an entry.
- **c. Whether it is already recorded.** The absent case is stated where a session acts on it:
  `design@knowledge-architect@the-branch-writes-its-changelog-entries` says the branch "creates the
  section above the newest released one when it is absent, as it is after a release renames it",
  and `instructions@git-workflow` point 1 says the same.
  `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument` is on the same
  head, about released sections.
- **d. Its kind.** An observation: the act-bearing texts state the absent case, so no act differs
  under the need test of `skill@knowledge-architect-agent-configuration@admission-standard`.
- **f. A better fix.** Rivals: the rewording, the proposal; no change. The rewording edits a head
  whose argument is already questioned by an open issue, and changes no act.
- **h. The proposal.** No change: the texts a branch acts on state the case.
- **Default**: no change, as h says.
- **Outcome**: accepted.
