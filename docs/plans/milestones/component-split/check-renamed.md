# The core is renamed `check`, and the installed-file findings move to phase 4

The third slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

As D3 of the milestone document rules it.

- crates/core renamed crates/check, the Component `core` renamed `check` in the manifest, every
  reference naming the anchor `core` re-pointed, the citations the checker does not read included,
  found with the search the milestone document names and with `git grep -n 'crates/core'` and
  `git grep -n '@core@'`, the bare paths of Markdown files included, and xtask's path dependency
  followed. The package keeps the name `knowledge-architect` until #cli-on-top gives it to `cli`.
- The prose that names the Component "the core" outside the plan documents and the goals homes,
  reworded to name `check`: it names a Component that no longer exists, and the search above does
  not find it.
- The goals of `register@check@goal`, each sent to the Component that holds its subject, and each
  rewording of a goal that names "the core" drafted under `skill@knowledge-architect-goal-setting`;
  the owner rules on them before the slice merges.
- The family `check` of `path@elsewhere@crates/model/src/shipped.rs`, the installed-file check, moved to `check`,
  with the rendering of the shipped text and the tests over the installed set, as a check family of
  the last phase, beside `generated`, `registers` and `references`: a shipped file missing from the tree, an installed
  file whose bytes differ from the shipped text, a file of the installer's namespace this version
  does not ship, a deletion of one awaiting staging, and a root CLAUDE.md that does not import the
  shipped primer. Kept in phase 2, in `model`'s foundation, from the manifest's harness alone: a root
  CLAUDE.md the walk keeps out, until `issue@model@the-walk-may-exclude-a-file-the-model-requires`
  replaces it. Read from the manifest's harness, it applies to a commit's tree under `commits` too,
  like every other finding of `tree`; a commit that keeps its root CLAUDE.md out is a tree `check`
  already failed. `model` depends on nothing of `agent-skills` for a consumer: the tests that read the entity table's private items stay in it, by a path-only dev-dependency, a narrowing of D3 put to the owner.
- `index`, `index --staged`, `id`, `worklist` and `new-component` no longer refused by a stale
  install: each asks the foundation, which no longer holds the family.
- `design@knowledge-architect@no-component-directory-named-after-the-project` rewritten for its
  instance: crates/check for the package `knowledge-architect` until #cli-on-top.

## Claims

- **No reference names `core`.** Test: `cargo klarch check` passes, and
  `git grep -n '@core@' -- ':!docs/plans'` lists none, a reference inside a span that holds a
  command included; the plan documents' commands and
  quotations of the record search keep their words. Shown to fail by a recorded reference to `core`
  left in a design home.
- **A differing installed file is a phase-4 finding**, reported beside every other phase-4 finding.
  Test: a binary test over a copy of a mock project with one installed file edited, built as
  `check_fix_installs_a_differing_installed_file` of `path@check@tests/binary.rs` builds its copy:
  the finding is listed in the last phase, and a reference finding of the same tree is listed with
  it; shown to discriminate by a recorded mutation of `foundation` of `path@check@src/check/mod.rs`
  that stops the run on the family's findings, which `cargo x mutate` runs in one file.
- **A missing shipped file is a phase-4 finding, and the references to its entities are reported
  beside it.** Test: a binary test over a copy with one installed skill deleted and the deletion
  staged, whose documents cite that skill: the install finding and the unresolved reference are
  both listed in the last phase; shown to discriminate by the same mutation.
- **A root CLAUDE.md the walk keeps out still stops the run in phase 2.** Test: the existing test
  of `path@elsewhere@crates/model/src/shipped.rs` on it, moved to `tree` with the finding; shown to fail by a
  recorded mutation that removes it from `tree`, since a move to the last phase spans two files,
  which `cargo x mutate` refuses, per `design@mutate@one-file-per-run`.
- **`index` and `index --staged` write over a stale install.** Test: two binary tests over the
  edited copy, one per command, each exits 0 and writes the index, into the working tree or into
  git's index; shown to fail by the first claim's mutation for `index --staged`, and for `index`, which asks `complete_working_tree`, by the same refusal there.
- **`check --fix` still repairs the installed files first.** Test: the existing test of
  `design@check@fix-before-the-checks` passes; shown to fail by a recorded mutation that runs the
  repair after the gate.
- **`commits` still compares no installed copy.** Test: the existing test that a commit holding a
  file of the namespace the version does not ship passes `commits`; shown to fail by a recorded
  mutation that hands the commit's tree the survey's installed files.
- **`model` no longer depends on `agent-skills`.** Test: `cargo tree -e normal -p knowledge-architect-model`
  lists no `knowledge-architect-agent-skills`; shown to fail before the move, when it lists it.

## Fixtures

Copies of an existing mock project, built by the helpers of `path@check@tests/binary.rs` that copy a
project and serve the `claude` harness, `Sandbox::serve_claude` among them; no new committed mock
project.

## Audit subjects

`design@check@phases-gate-the-report`, `design@check@owned-namespace-check`,
`design@check@fix-before-the-checks`, `design@check@staged-check-semantics`,
`design@model@installed-entities-from-the-tree`; `tripwire@check@phases-gate-the-report-two`, whose
premise, no writer over a model that could not read the tree, the move keeps, per D3;
`issue@model@the-walk-may-exclude-a-file-the-model-requires`;
the closed issue `installed-file-findings-belong-in-phase-four`, read in history: the commit that
deleted it is
`git log --diff-filter=D --format=%h -- crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`,
and `git show <that commit>^:crates/core/docs/open-issues/installed-file-findings-belong-in-phase-four.md`
prints it; `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`;
`design@check@owned-namespace-check`, which this slice splits, the split
`issue@check@core-heads-bundle-decisions-with-many-citing-sites` no longer lists; `register@check@goal`;
`goal@check@relocation-needs-only-goal-changes`; `spec@plans@path-quickfixes`, on hold, which plans a
rewrite of `design@check@fix-before-the-checks` too: whichever lands second reads the other's.

## Goals sent

Read at the audit against the Components that exist at this slice:

- `goal@check@records-reach-their-reader` stays in `check`: its subject, the listings, goes to
  `query`, which #consumers-out creates and whose goals that slice drafts;
- `goal@check@documents-render-as-a-linked-site`: no Component holds it until `render` exists, and
  the purpose of `check` does not cover it, per
  `issue@check@the-core-s-purpose-leaves-the-rendered-site-outside-it`. Drafted to the root's goals
  home, for the owner's ruling;
- the others stay in `check`, each holding its subject there.

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
rename's cost, the references outside the guarantee of `relocation-is-one-manifest-edit`, now
`goal@check@relocation-needs-only-goal-changes`, against a descriptive name, and proposed it; the
owner approved it, R5, with the reason "core would become misleading" (#arg-95, #arg-106).

**Phase 4, and what stays in phase 2.** The core's placement rule puts a finding before the last
phase only when another check reads it, or when it reports what the model could not read. No check
reads the installed-file findings. An installed file that differs, or one the version does not
ship, is outside the walk, and `check::agents` reads its bytes from the survey, not from the model,
which holds the installed copies for definitions only, per
`design@model@installed-entities-from-the-tree` (the closed issue
`installed-file-findings-belong-in-phase-four`, #arg-81); a stale installed skill still defines its
entities, so the model built over it is complete (#arg-69). The owner had judged the move correct
and not worth doing at the time; the split moves the code anyway (#arg-81, #arg-82), and the owner
ruled again, R5: "installed-text-check-home: agreed on phase 4" (#arg-103). Over a missing shipped
file the model is still built, of the tree as it is, and the references to its entities are
reported as unresolved in the last phase, beside the install finding, per D3. A root CLAUDE.md the
walk keeps out is what the model could not read, so it stays in phase 2 until the walk refuses the
row that keeps it out.

What the move still changes in use: `check --fix` already repairs the installed files before the
gate, per `design@check@fix-before-the-checks`, so an ordinary upgrade needs no second run; an
upgrade that removes a shipped file leaves the deletion of its installed copy awaiting staging,
which no fix stages, and phase 2 then stops the run before the generated files. In the last phase
that run reaches them. The head's sentence on the two runs is rewritten with the order.

The rivals, the check kept in the foundation (A) or moved to `check` and kept in phase 2 (B), and
a missing shipped file kept in phase 2, D3's default, are losing alternatives of the milestone
document.

## Implementation sequence

1. The rename of the directory, the anchor and every reference, in the branch's first commit, per
   `primer@branch-commits`: `commits` judges each tree with the checker directories the tip's build
   compiled in, so a tree holding crates/core under a tip at crates/check has its literals read as
   prose. The amended plan documents land in that commit.
2. The prose that names the core.
3. The phase-4 family: the mechanism alone.
4. The heads split and rewritten, `design@knowledge-architect@no-component-directory-named-after-the-project`
   among them; the goals drafted.
5. The harvest.

## Harvest

| what | where |
| --- | --- |
| #installed-text-check-home | `design@check@phases-gate-the-report`'s table, `design@check@fix-before-the-checks`'s order and its sentence on the two runs, and `design@check@owned-namespace-check` split in two, each rewritten in `check`'s home with the rival its argument covers: the installed files are committed, against installing them on demand; and they are checked byte for byte in a namespace the installer owns, ownership decided by a name, against a record of installed files and a digest; the issue `installed-file-findings-belong-in-phase-four` was closed by the commit that added the milestone |
| what the move makes false in the other heads | `design@model@no-check-below-the-model`, `design@model@tree-inputs-gathered-here`, `design@model@model-facade`, `design@knowledge-architect@model-crate`, `design@knowledge-architect@installed-text-crate`, `design@check@inputs-are-gathered`, `design@check@index-staged-write`, `design@check@fix-refusal-mixed-state`, `design@check@init-keeps-existing-files`; each citation of `design@check@owned-namespace-check` re-pointed to the half it means, the primer's import line kept in one of them, and `design@check@orphan-rejected-entry` kept by citing each of its two `rejected` entries from the head whose argument covers it |
| what the move makes false outside the record | `path@check@README.md` on the phases, `check --fix` and `install-agent-skills`; `path@check@CLAUDE.md` on the mock projects; the `unsound` mock's planted file of the namespace; the changelog |
| #core-renamed-check | no head; the commit records the session's weighing and the owner's approval |
| the milestone's table | `design@knowledge-architect@no-component-directory-named-after-the-project`, its instance rewritten |
| premortem C5 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |
