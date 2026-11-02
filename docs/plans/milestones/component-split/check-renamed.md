# The core is renamed `check`, and the installed-file findings that judge no part of the model move to phase 4

The third slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

As D3 of the milestone document's default states it; the owner's ruling on D3 may move the two
findings it keeps in phase 2 to the last phase too, and the slice is built on the ruling it has.

- crates/core renamed crates/check, the Component `core` renamed `check` in the manifest, every
  reference naming the anchor `core` re-pointed, the citations the checker does not read included,
  found with the search the milestone document names, and xtask's path dependency followed. The
  package keeps the name `knowledge-architect` until #cli-on-top gives it to `cli`.
- The goals of `register@core@goal`, each sent to the Component that holds its subject, and each
  rewording of a goal that names "the core" drafted under `skill@knowledge-architect-goal-setting`;
  the owner rules on them before the slice merges.
- The family of `check::agents` split by what each finding reports. To `check`, as a check family
  of the last phase, beside `generated`, `registers` and `references`: an installed file whose bytes
  differ from the shipped text, a file of the installer's namespace this version does not ship, a
  deletion of one awaiting staging, and a root CLAUDE.md that does not import the shipped primer.
  Kept in phase 2, in `model`'s foundation: a shipped file missing from the tree, whose entities
  the model then lacks, per `design@core@installed-entities-from-the-tree`, and a root CLAUDE.md
  the walk keeps out, which the model could not read. `model` no longer depends on `agent-skills`
  for the comparison: the shipped list it needs, the install paths, it takes as data.
- `index` and `index --staged` no longer refused by an installed file that differs, or by an
  unshipped one.
- `design@knowledge-architect@no-component-directory-named-after-the-project` rewritten for its
  instance: crates/check for the package `knowledge-architect` until #cli-on-top.

## Claims

- **No reference names `core`.** Test: `cargo klarch check` passes, and
  ``git grep -n '`[a-z]*@core@' -- ':!docs/plans'`` lists none; the plan documents' commands and
  quotations of the record search keep their words. Shown to fail by a recorded reference to `core`
  left in a design home.
- **A differing installed file is a phase-4 finding**, reported beside every other phase-4 finding.
  Test: a binary test over a copy of a mock project with one installed file edited, built as
  `check_fix_installs_a_differing_installed_file` of `path@core@tests/binary.rs` builds its copy:
  the finding is listed in the last phase, and a reference finding of the same tree is listed with
  it; shown to discriminate by a recorded mutation that puts the family back in phase 2.
- **A missing shipped file still stops the run in phase 2.** Test: a binary test over a copy with
  one installed file deleted and the deletion staged: the run stops in phase 2 and names it; shown
  to discriminate by a recorded mutation that moves that finding to the last phase.
- **`index` and `index --staged` write over a differing installed file.** Test: two binary tests
  over the edited copy, one per command, each exits 0 and writes the index, into the working tree
  or into git's index; shown to fail by the first claim's mutation.
- **`check --fix` still repairs the installed files first.** Test: the existing test of
  `design@core@fix-before-the-checks` passes; shown to fail by a recorded mutation that runs the
  repair after the gate.
- **`model` no longer depends on `agent-skills`.** Test: `cargo tree -e normal -p knowledge-architect-model`
  lists no `knowledge-architect-agent-skills`; shown to fail before the move, when it lists it.

## Fixtures

Copies of an existing mock project, built by the helpers of `path@core@tests/binary.rs` that copy a
project and serve the `claude` harness, `Sandbox::serve_claude` among them; no new committed mock
project.

## Audit subjects

`design@core@phases-gate-the-report`, `design@core@owned-namespace-check`,
`design@core@fix-before-the-checks`, `design@core@staged-check-semantics`,
`design@core@installed-entities-from-the-tree`; `tripwire@core@phases-gate-the-report-two`, whose
premise, no writer over an incomplete model, the finding kept in phase 2 for a missing file keeps;
the closed issue `installed-file-findings-belong-in-phase-four`, read in history: the commit that
deleted it is
`git log --diff-filter=D --format=%h -- crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`,
and `git show <that commit>^:crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`
prints it; `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`;
`design@core@owned-namespace-check`, which this slice splits, the split
`issue@core@core-heads-bundle-decisions-with-many-citing-sites` no longer lists; `register@core@goal`;
`goal@core@relocation-is-one-manifest-edit`; `spec@plans@path-quickfixes`, on hold, which plans a
rewrite of `design@core@fix-before-the-checks` too: whichever lands second reads the other's.

## Fails alone on

A reference left naming `core`, or a differing installed file that still stops a run before phase
4.

## Premises that expire

The package name `knowledge-architect` stays on `check` until #cli-on-top, so the crate's directory
and its package name differ for two slices, as
`design@knowledge-architect@no-component-directory-named-after-the-project` allows; #cli-on-top's
rewrite of that head ends it.

## Decided design

**The rename.** With `model` out, "core" no longer says which crate it is. The session weighed the
rename's cost, the references outside `goal@core@relocation-is-one-manifest-edit`'s guarantee,
against a descriptive name, and proposed it; the owner approved it, R5, with the reason "core would
become misleading" (#arg-95, #arg-106).

**Phase 4, and what stays in phase 2.** The core's placement rule puts a finding before the last
phase only when another check reads it, or when it reports what the model could not read. No check
reads the installed-file findings. An installed file that differs, or one the version does not
ship, is outside the walk, and `check::agents` reads its bytes from the survey, not from the model,
which holds the installed copies for definitions only, per
`design@core@installed-entities-from-the-tree` (the closed issue
`installed-file-findings-belong-in-phase-four`, #arg-81); a stale installed skill still defines its
entities, so the model built over it is complete (#arg-69). The owner had judged the move correct
and not worth doing at the time; the split moves the code anyway (#arg-81, #arg-82), and the owner
ruled again, R5: "installed-text-check-home: agreed on phase 4" (#arg-103). The plan's second review
found that #arg-69 does not hold of a shipped file that is missing, whose entities the model then
lacks, nor of a root CLAUDE.md the walk keeps out: each reports what the model could not read, so
the placement rule keeps them in phase 2. That narrowing is D3's, awaiting the owner.

What the move still changes in use: `check --fix` already repairs the installed files before the
gate, per `design@core@fix-before-the-checks`, so an ordinary upgrade needs no second run; an
upgrade that removes a shipped file leaves the deletion of its installed copy awaiting staging,
which no fix stages, and phase 2 then stops the run before the generated files. In the last phase
that run reaches them. The head's sentence on the two runs is rewritten with the order.

The rivals, the check kept in the foundation (A) or moved to `check` and kept in phase 2 (B), are
losing alternatives of the milestone document, and so is the move whole, if D3 is ruled as its
default.

## Implementation sequence

1. The phase-4 family, inside the core, with no rename: the mechanism alone.
2. The rename of the directory, the anchor and every reference; the goals sent and drafted.
3. The heads split and rewritten, `design@knowledge-architect@no-component-directory-named-after-the-project`
   among them.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #installed-text-check-home | `design@core@phases-gate-the-report`'s table, `design@core@fix-before-the-checks`'s order and its sentence on the two runs, and `design@core@owned-namespace-check` split in two, the committed files and the byte check, each rewritten in `check`'s home; the issue `installed-file-findings-belong-in-phase-four` was closed by the commit that added the milestone |
| #core-renamed-check | no head; the commit records the session's weighing and the owner's approval |
| the milestone's table | `design@knowledge-architect@no-component-directory-named-after-the-project`, its instance rewritten |
| premortem C5 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |
