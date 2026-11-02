# The core is renamed `check`, and the installed-file check moves to phase 4

The third slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- crates/core renamed crates/check, the Component `core` renamed `check` in the manifest, every
  reference naming the anchor `core` re-pointed. The package keeps the name `knowledge-architect`
  until #cli-on-top gives it to `cli`.
- The installed-file check moved out of the foundation into `check`, as a check family of the last
  phase, beside `generated`, `registers` and `references`. `model` no longer depends on
  `agent-skills`.
- `index` and `index --staged` no longer refused by a stale install.

## Claims

- **No reference names `core`.** Test: `git grep -n '@core@'` lists none; `cargo klarch check`
  passes.
- **A stale install is a phase-4 finding**, reported beside every other phase-4 finding. Test: a
  binary test over a mock project with one installed file edited: the finding is listed in the last
  phase, and a reference finding of the same tree is listed with it; shown to discriminate by a
  recorded mutation that puts the family back in phase 2.
- **`index` writes over a stale install.** Test: a binary test running `index` over that project
  exits 0 and writes the index.
- **`check --fix` still repairs the installed files first.** Test: the existing test of
  `design@core@fix-before-the-checks` passes.
- **`cargo tree -p knowledge-architect-model` lists no `knowledge-architect-agent-skills`.**

## Audit subjects

`design@core@phases-gate-the-report`, `design@core@owned-namespace-check`,
`design@core@fix-before-the-checks`, `design@core@staged-check-semantics`,
`design@core@installed-entities-from-the-tree`; the issue `installed-file-findings-belong-in-phase-four`;
`issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`;
`issue@core@core-heads-bundle-decisions-with-many-citing-sites`, whose head
`design@core@owned-namespace-check` this slice splits; `goal@core@relocation-is-one-manifest-edit`.

## Fails alone on

A reference left naming `core`, or a stale install that still stops a run before phase 4.

## Premises that expire

The package name `knowledge-architect` stays on `check` until #cli-on-top, so the check's
crate and its package name differ for two slices, as
`design@knowledge-architect@no-component-directory-named-after-the-project` allows.

## Decided design

**The rename.** With `model` out, "core" no longer says which crate it is; the owner: "core would
become misleading" (#arg-95, #arg-106). Its cost, the references outside
`goal@core@relocation-is-one-manifest-edit`'s guarantee, was weighed and accepted.

**Phase 4.** The installed-file findings belong in the last phase by the core's own placement rule:
the installed files are outside the walk, and no check reads the findings
(the issue `installed-file-findings-belong-in-phase-four`, #arg-81). A stale installed skill still
defines its entities, so the model built over it is complete (#arg-69). The issue recorded the move
as correct and not worth doing then; the split moves the code anyway (#arg-81, #arg-82). The rivals,
the check kept in the foundation (A) or moved to `check` and added to phase 2 (B), are losing
alternatives of the milestone document.

## Implementation sequence

1. The phase-4 family, inside the core, with no rename: the mechanism alone.
2. The rename of the directory, the anchor and every reference.
3. The heads split and rewritten; the issue closed.
4. The harvest.

## Harvest

| what | where |
| --- | --- |
| #installed-text-check-home | `design@core@phases-gate-the-report`'s table, `design@core@fix-before-the-checks`'s order, and `design@core@owned-namespace-check` split in two, the committed files and the byte check, each rewritten in `check`'s home; the issue `installed-file-findings-belong-in-phase-four` was closed by the commit that added the milestone |
| #core-renamed-check | no head; the commit records it |
| premortem C5 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` |
