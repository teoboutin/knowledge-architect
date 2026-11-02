# The core is renamed `check`, and the installed-file check moves to phase 4

The third slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- crates/core renamed crates/check, the Component `core` renamed `check` in the manifest, every
  reference naming the anchor `core` re-pointed, the citations the checker does not read included,
  found with the search the milestone document names. The package keeps the name
  `knowledge-architect` until #cli-on-top gives it to `cli`.
- The goals of `register@core@goal`, each sent to the Component that holds its subject, and each
  rewording of a goal that names "the core" drafted under `skill@knowledge-architect-goal-setting`;
  the owner rules on them before the slice merges.
- The installed-file check moved out of the foundation into `check`, as a check family of the last
  phase, beside `generated`, `registers` and `references`. `model` no longer depends on
  `agent-skills`.
- `index` and `index --staged` no longer refused by a stale install.
- `design@knowledge-architect@no-component-directory-named-after-the-project` rewritten for its
  instance: crates/check for the package `knowledge-architect` until #cli-on-top.
- The root's layout head updated for the rename.

## Claims

- **No reference names `core`.** Test: `git grep -n '@core@'` lists none; `cargo klarch check`
  passes. Shown to fail by a recorded reference to `core` left in a document.
- **A stale install is a phase-4 finding**, reported beside every other phase-4 finding. Test: a
  binary test over a mock project with one installed file edited: the finding is listed in the last
  phase, and a reference finding of the same tree is listed with it; shown to discriminate by a
  recorded mutation that puts the family back in phase 2.
- **`index` and `index --staged` write over a stale install.** Test: two binary tests over that
  project, one per command, each exits 0 and writes the index, into the working tree or into git's
  index; shown to fail by the same mutation.
- **`check --fix` still repairs the installed files first.** Test: the existing test of
  `design@core@fix-before-the-checks` passes; shown to fail by a recorded mutation that runs the
  repair after the gate.
- **`model` no longer depends on `agent-skills`.** Test: `cargo tree -p knowledge-architect-model`
  lists no `knowledge-architect-agent-skills`; shown to fail before the move, when it lists it.

## Audit subjects

`design@core@phases-gate-the-report`, `design@core@owned-namespace-check`,
`design@core@fix-before-the-checks`, `design@core@staged-check-semantics`,
`design@core@installed-entities-from-the-tree`; the closed issue
`installed-file-findings-belong-in-phase-four`, read in history: the commit that deleted it is
`git log --diff-filter=D --format=%h -- crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`,
and `git show <that commit>^:crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`
prints it;
`issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`;
`issue@core@core-heads-bundle-decisions-with-many-citing-sites`, whose head
`design@core@owned-namespace-check` this slice splits; `register@core@goal`;
`goal@core@relocation-is-one-manifest-edit`.

## Fails alone on

A reference left naming `core`, or a stale install that still stops a run before phase 4.

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

**Phase 4.** The core's placement rule puts a check before the last phase only when another check
reads its findings, or when it reports what the model could not read. Neither holds for the
installed-file findings: the installed files are outside the walk, no check reads the findings, and
`check::agents` reads their bytes from the survey, not from the model, which holds the installed
copies for definitions only, per `design@core@installed-entities-from-the-tree` (the closed issue
`installed-file-findings-belong-in-phase-four`, #arg-81). A stale installed skill still defines its
entities, so the model built over it is complete (#arg-69). The owner had judged the move correct
and not worth doing at the time; the split moves the code anyway (#arg-81, #arg-82), and the owner
ruled again, R5: "installed-text-check-home: agreed on phase 4" (#arg-103).

What the move still changes in use: `check --fix` already repairs the installed files before the
gate, per `design@core@fix-before-the-checks`, so an ordinary upgrade needs no second run; an
upgrade that removes a shipped file leaves its deletion unstaged, which no fix stages, and phase 2
then stops the run before the generated files. In the last phase that run reaches them. The head's
sentence on the two runs is rewritten with the order.

The rivals, the check kept in the foundation (A) or moved to `check` and kept in phase 2 (B), are
losing alternatives of the milestone document.

## Implementation sequence

1. The phase-4 family, inside the core, with no rename: the mechanism alone.
2. The rename of the directory, the anchor and every reference; the goals sent and drafted.
3. The heads split and rewritten, `design@knowledge-architect@no-component-directory-named-after-the-project`
   among them; the layout head updated.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #installed-text-check-home | `design@core@phases-gate-the-report`'s table, `design@core@fix-before-the-checks`'s order and its sentence on the two runs, and `design@core@owned-namespace-check` split in two, the committed files and the byte check, each rewritten in `check`'s home; the issue `installed-file-findings-belong-in-phase-four` was closed by the commit that added the milestone |
| #core-renamed-check | no head; the commit records the session's weighing and the owner's approval |
| premortem C5 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |
