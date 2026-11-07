# 2026-11-01-knowledge-architect-release-0-9-0

- **Received file**: 2026-11-01-knowledge-architect-release-0-9-0.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.8.0-252-gc4b0a117`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@knowledge-architect@a-mechanical-changelog-check`.

## P1 and P2, with P1 of 2026-11-02-knowledge-architect-component-split-git-out, P2 of 2026-11-06-knowledge-architect-consumers-out and P1 of 2026-11-07-knowledge-architect-integrated-tests: the release's lists of crates, directories and test files are restatements that drift

- **a.** What it says.
  - P1 here: step 7 of `skill@klarch-release` named agent-skills' directories, "build script,
    content/ and snippets/", and the package also held `path@agent-skills@fragments/`; repaired on the branch. The
    routing reviewer judged that the list is the comparison the step makes, so its drift is the
    signal; the defect is that nobody updates it when `include` changes.
  - P2 here: step 8 named the expected dry-run warnings, "ignoring test `binary`" and "ignoring test
    `extension_api`", and a third appeared; repaired to one warning per test file.
  - P1 of component-split-git-out: the skill counted the crates ("the four published crates", "its
    four sites", "all four"); repaired to names.
  - P2 of consumers-out and P1 of integrated-tests: the milestone's list of what a new crate owes
    missed the `skill@klarch-release@failed-publish` paragraph of the skill and the step-1 pins of `check`; the crate list
    and order are spelled in at least five places. Proposal of integrated-tests: one home, a list
    `cargo x` prints in publish order, which the workflow and the skill read; rival, a test that
    compares the lists.
- **b.** Whether it still applies. The named instances are repaired: step 7 names the fragments directory
  (commit "[knowledge-architect] The release skill's package list names agent-skills' fragments/",
  `8c306e92`); step 8 expects one warning per test file (commit "[knowledge-architect] The release
  skill's dry run expects one warning per test file of the core", `5dff7fda`); the counts left at the
  commit "[knowledge-architect] The git crate in the publish order, the head on the installed text's
  crate rewritten, and the texts that listed the crates" (`f8ce602d`). The cause holds at HEAD. The
  crate set or its order is spelled by hand at seven places:
  `path@agent-config@skills/klarch-release/SKILL.md`, lines 8 to 10 (the scope), line 32 (the exact
  pins, per crate), lines 100 to 102 (the dry run's order) and line 141 (the failed publish's upload
  order); `path@knowledge-architect@.github/workflows/release.yml`, lines 39 and 70; and
  `design@knowledge-architect@partial-publish-rerun`, whose body spells the order. Step 8 at lines 102
  to 104 lists the test directories, "of `path@cli@tests/`, of `path@testing@tests/` and of
  `path@mutate@tests/`", a further list, which a crate gaining tests makes stale. Measured at HEAD: the
  step-1 pins match the `=0.9.0` pins of the crates' Cargo.toml files, and the crates' test directories
  that hold a Rust file are those three (`path@query@tests/` holds a text file only), so no list is
  false today. Two facts make most lists derivable: the step itself says "`git grep -n '<previous
  version>'` finds every site that still names the previous one", which finds every pin; and
  `path@xtask@src/changelog.rs`, lines 17 to 19, derives the published set: one copy per directory under crates/ that holds a Cargo manifest,
  "since every crate there is published and nothing else is".
- **c.** Not recorded as a defect. `design@knowledge-architect@partial-publish-rerun` and
  `design@knowledge-architect@the-changelog-ships-in-every-crate` constrain any change: the publish
  job publishes "each before the crates that depend on it" and skips a version crates.io holds; every
  published crate carries the changelog copy, which the test over every crate directory enforces.
- **d.** Its kind: wrong, a restatement that drifted from its home, recurring: four sessions met it
  (release 0.9.0 twice, the first split slice, the last two slices).
- **e.** Its scope: `goal@knowledge-architect@agents-work-without-drift`; owners: the release skill,
  this repository's own text; the release workflow and the head, the root Component. No fix reverses
  a decision: the head's decision is the re-run, and its order is a restatement of the job's list.
- **f.** A better fix, one for the cluster. The rival to one generated list for every site: the
  derivations the skill already holds or can state in words, which remove four of the seven
  enumerations, and the list of test directories, with no code; the two lists of the release workflow need a source the workflow runs,
  such as a `cargo x` command printing the crates under crates/ in dependency order, which is code
  and a branch of its own. A test that compares the lists keeps every copy. Step 7's list stays: the
  routing reviewer's argument holds, since comparing the package with `include` would compare cargo
  with itself.
- **g.** Its route: the skill edit, `skill@knowledge-architect-agent-configuration`, handled now;
  the release workflow's lists and the head's order, an issue.
- **h.** The proposal.
  - Handle now, in `path@agent-config@skills/klarch-release/SKILL.md`: the scope names the published
    crates as every crate under crates/, with xtask never published; step 1 names "the exact pins
    `version = "=<version>"` between the workspace crates", found by the grep the step already gives,
    in place of the per-crate list; step 8 expects the order "each crate before the crates that
    depend on it", and one warning per Rust file directly under the tests directory of a published crate;
    the `skill@klarch-release@failed-publish` paragraph says the job uploads the crates in the order of its list in
    `path@knowledge-architect@.github/workflows/release.yml`. Instructions: +0, −0; five enumerations
    replaced.
  - Open an issue in `knowledge-architect`, kind `todo`, proposed id
    `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`: the two lists of the release workflow and the order in
    the head's body, with as closing condition one source of the order that the workflow runs and
    the head points to.
- **Default**: handle now (the skill's five enumerations replaced by derivations), and open an issue
  for the release workflow's lists, `knowledge-architect`, kind `todo`,
  `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`.
- **Outcome**: decided by the session: handle now and open an issue. Ground: the cause is a
  restatement whose home is derivable (the grep of step 1; the published set of
  `path@xtask@src/changelog.rs`); a pointer or a derivation removes the drift at 0 instructions,
  per `primer@where-knowledge-goes`; the workflow's lists need code.

## P4: the highest bump class of the changelog section is counted by hand

- **a.** What it says: step 2 of `skill@klarch-release`, "The version chosen at step 1 follows the
  highest bump class among its entries", was carried out by a grep, re-taken by three reviewers.
  Proposal: `cargo x changelog` prints the highest class and the version it implies.
- **c.** Already recorded: `issue@knowledge-architect@a-mechanical-changelog-check`, whose assertions
  include "that the version a release names follows the highest class among its entries, a patch at
  least". It does not hold this instance. The sentence stands at HEAD,
  `path@agent-config@skills/klarch-release/SKILL.md`, lines 46 and 47.
- **Default**: no change, with the reason: recorded; the issue gains the instance.
- **Outcome**: decided by the session: no change; the issue gains the instance, a count by grep
  re-taken by three reviewers. Ground: the entry holds the assertion and the discussion that decides
  where the check lives; a printing command is one of its shapes.

## P3: the parse check of a saved workflow names only `node`

Analysed with P1 of 2026-11-01-knowledge-architect-pre-release-fixes, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-pre-release-fixes.md`.

## Read-back: the standing-state report first saved as a summary

Analysed with W1 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## Read-back: the transcript reviewer dispatched while one item awaited the owner

Analysed with W2 of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.md`.
