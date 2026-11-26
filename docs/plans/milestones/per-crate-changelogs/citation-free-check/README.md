# Slice 1: files declared free of citations, at two levels, and the root head on history

## Builds

- The manifest table `[walk.citation-free]`, with two lists of globs, `all` and `entries`, resolved
  in a module of its own beside `path@model@src/manifest.rs`, which gains only the call that
  dispatches the table.
- The findings of a citation-free file, in `check`, and the repair each names.
- The declaration in this repository's knowledge-architect.toml: `all` matching the 11
  CRATES-IO.md files of the crates, `entries` matching the installed copies the walk reads: the
  primer, each installed skill and each installed agent, as the milestone document's Names define
  them.
- The installed setup skill's proposal of a CRATES-IO.md, which also proposes the `all`
  declaration of the page, per `design@agent-skills@setup-default-crates-io-page`.
- `skill@klarch-release`'s step "The shipped text cites no entry", which lists by hand every
  backticked span with an `@` in the installed copies, replaced by the check.
- The root design head written from `thread@per-crate-changelogs@no-history-beyond-the-branch`, and
  the root `rejected` entry written from `thread@per-crate-changelogs@released-content-check`: both
  decisions no work implements, recorded when made.
- The closure of `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`.
- The entries of this slice in the shared changelog's `Next release` section, under its current
  grammar.

## Claims

Each test is shown to fail against a wrong implementation by a mutation of the code it guards, run
with `cargo x mutate`, as `skill@knowledge-architect-development` says.

1. **A reference in an `all` file is a finding whether or not it resolves.** A mock project's
   matched file holding a reference that resolves, and another holding one that dangles, each yield
   the finding; the same spans in an unmatched file yield none and a dangling-reference finding
   respectively. Shown to fail by a mutation that resolves the span before the level is applied.
2. **A placeholder passes at both levels.** A span with an angle bracket, such as
   `design@<anchor>@<id>`, in a matched file yields no finding.
3. **The level `entries` refuses an entry or an anchor of the project, and admits the rest.** In a
   matched file: a reference to a design entry, to an issue, and `path@<component>@<path>` naming a
   declared Component are findings; `skill@<shipped skill>`, `agent@<shipped agent>`,
   `primer@<slug>`, a `path` and a `register` reference under the anchor `*` or `plans`, and
   `register@goal` are not, and each of these is still resolved, so one that dangles is reported as
   dangling.
4. **The id lints keep reading a citation-free file**: a plain word and a backticked span that are
   exactly a defined id are reported in a matched file as in any other.
5. **The unanchored-path lint keeps reading a citation-free file**, and in one its repair names a
   rewrite in words, not the `path` form.
6. **No repair a finding of this slice names is the span with its backticks removed**: each names a
   rewrite in words, or a placeholder in angle brackets. Tested on the repair text of each finding.
7. **The current install passes**, premortem cause 5: `cargo klarch check` on this repository,
   with the declaration, reports nothing.
8. **The issue's closing test**: `cargo x gates` fails on a reference to an entry planted in a
   scratch copy of an installed copy, and passes on a `%%` comment of content/ that cites an entry,
   since content/ is not matched.

## Fixtures

A mock project under `path@testing@tests/projects/`, or a table added to an existing one, that
declares `[walk.citation-free]`: the table is the addition this slice makes to the manifest's
vocabulary. Its matched files hold, each in a file of its own, the spans of claims 1 to 6.

## Audit subjects

- `path@model@src/manifest.rs`, `path@model@src/scan.rs`, `path@check@src/check/references.rs`.
- `design@check@the-regime-has-no-opt-out`, `rejected@check@exempt-directory-manifest-row`,
  `design@agent-skills@plain-text-is-no-repair`, `design@check@reference-shape-outside-span`,
  `design@check@plain-word-lint`, `design@check@backticked-bare-id`,
  `design@check@unanchored-path-first-segment-here`, `design@model@a-reference-is-live-in-every-prose`,
  `design@agent-skills@shipped-text-cites-no-entry`, `design@knowledge-architect@crates-io-page-file`,
  `design@check@check-reads-no-range-of-history`, `issue@git@a-shallow-clone-shortens-the-range-commits-judges`.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`, which this slice closes, and
  `issue@model@manifest-rs-is-past-one-read`, weighed below.

## Fails alone on

- A finding, or its absence, on a citation-free file of a mock project or of this repository.

## Premises that expire

- The shared changelog stays a row of `[walk] skip-files` until slice 5, which removes the rows and
  adds the changelogs to `all`. Nothing of this slice reads it meanwhile; the guard is slice 5's
  claim that this repository's check passes with the changelogs matched.

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@model@manifest-rs-is-past-one-read`, weighed.** The slice changes
  `path@model@src/manifest.rs` by one dispatch of the new table, whose resolve goes in a module of
  its own. The default: no split of manifest.rs is planned here. Its argument, from the code-split
  scout's report: the file has one reference, the manifest's grammar, read through one parse, the
  signal against a split; and the new table adds no line to its resolvers but the dispatch. The
  slice's design audit, with the scout's report over the file, either closes the issue on that
  argued decision or plans the split as a step before the work.

## What is already decided

The entries this slice's audit takes as seeds are the audit subjects' design entries above.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@knowledge-architect@crates-io-page-file`: its rule becomes enforced (listed there as
  rewritten).
- `design@check@the-regime-has-no-opt-out`: gains the declaration as a member that adds obligations
  and removes none.
- `design@model@a-reference-is-live-in-every-prose`: gains the citation-free files, where a
  reference is read and refused.
- `design@check@unanchored-path-first-segment-here`: its repair in a citation-free file.
- `design@agent-skills@shipped-text-cites-no-entry`: the rule is now checked.
- `design@agent-skills@setup-default-crates-io-page`: the proposal includes the declaration.
- `design@check@check-reads-no-range-of-history`: cites the new root head, which widens it.

## Decided design

### A file the manifest declares citation-free holds no reference, at one of two levels

From `thread@per-crate-changelogs@citation-free-paths`.

- `[walk.citation-free]` holds two lists of globs, `all` and `entries`, matched against the paths
  the walk reads. An illustration of the shape:

  ```toml
  [walk.citation-free]
  all = ["crates/*/CRATES-IO.md"]
  entries = [".claude/knowledge-architect/PRIMER.md", ".claude/skills/knowledge-architect-*/", ".claude/agents/knowledge-architect-*.md"]
  ```

- **`all`**: a span that parses as a reference is a finding whether or not it resolves, and no span
  of the file is resolved.
- **`entries`**: a reference to an entry of a register, or a `path` or `register` reference naming
  an anchor of the project, is a finding; a reference of a harness kind (`skill`, `agent`,
  `primer`), a `path` or a `register` reference under the anchor `*` or `plans`, and
  `register@goal` are admitted and resolved as anywhere else. This serves the installed copies,
  which may cite what every installing project holds, on the owner's word at R7.
- At both levels, a placeholder, a span holding an angle bracket, passes, as
  `primer@where-knowledge-goes` says of an illustration.
- The id lints keep reading a matched file, and so does the unanchored-path lint, whose repair there
  is a rewrite in words, since its usual repair, the `path` form, is what the file refuses. The
  cost accepted: a citation-free file cannot print an existing path of the tree in backticks.
- A finding's repair names a rewrite in words or a placeholder, never the span with its backticks
  removed, per `design@agent-skills@plain-text-is-no-repair`.
- The declaration adds obligations and removes none, so it is a member of
  `design@check@the-regime-has-no-opt-out`.
- A generated file is not matched: its findings are its sources'.
- Argument: `argument@per-crate-changelogs@arg-29`, `argument@per-crate-changelogs@arg-32`,
  `argument@per-crate-changelogs@arg-33`, `argument@per-crate-changelogs@arg-44`,
  `argument@per-crate-changelogs@arg-45`, `argument@per-crate-changelogs@arg-46`,
  `argument@per-crate-changelogs@arg-56`, `argument@per-crate-changelogs@arg-58`. Nearest rival: one
  level refusing every reference, defeated by the installed copies' legitimate citations of what
  every project holds (R6, R7).

### No check reads the history before the base of the branch it judges

From `thread@per-crate-changelogs@no-history-beyond-the-branch`. A head of the root's design home,
with the thread's slug, written in this slice because no work implements it.

- Its scope: every check of the checker, `commits` included, whose range already starts at the
  base. `--require-rebased`, which asks git whether `origin/main` is an ancestor of `HEAD`, is
  within it, and so is any other question to git about the branch's base, which
  `issue@git@a-shallow-clone-shortens-the-range-commits-judges` needs.
- Its argument, the owner's at R3: the cost of reading past history grows with every commit the
  project makes, for every future run, with no bound the project controls; consumers' histories,
  with rewritten tags, shallow clones and imported squashes, would each cost design and code; and
  the value is low, since a review flags a deviation without being asked to look for it.
- It widens `design@check@check-reads-no-range-of-history`, which covers `check` alone, and that
  head cites it.
- It cites the `rejected` entry below.
- Argument: `argument@per-crate-changelogs@arg-17`, `argument@per-crate-changelogs@arg-30`,
  `argument@per-crate-changelogs@arg-35`, `argument@per-crate-changelogs@arg-53`.

### A check of a released section's content is rejected

From `thread@per-crate-changelogs@released-content-check`, ruled out. A `rejected` entry of the
root, with the thread's slug, cited by the head above.

- It records both arguments: the comparison with the tree at a version's tag reads history before
  the branch, against the head above; a rival that compares within the branch alone is defeated by
  the owner's argument that a released text may be reworded without a change of meaning (R2), which
  no comparison of bytes or of sets tells apart.
- It is recorded because `issue@knowledge-architect@a-mechanical-changelog-check` lists the
  assertion as a thing to build, so it could plausibly be proposed again.
- Argument: `argument@per-crate-changelogs@arg-11`, `argument@per-crate-changelogs@arg-17`,
  `argument@per-crate-changelogs@arg-54`.

## Harvest

| what | thread | home |
| --- | --- | --- |
| the declaration, its two levels, its findings and their repairs | `thread@per-crate-changelogs@citation-free-paths` | a head of `register@check@design` on the citation-free findings, and a head of `register@model@design` on the table, each by the tests of `skill@knowledge-architect-decision-recording`; the touched heads listed above |
| the rival of one level | `thread@per-crate-changelogs@citation-free-paths` | `register@check@rejected`, if a recording test holds |
| the head on history | `thread@per-crate-changelogs@no-history-beyond-the-branch` | `register@knowledge-architect@design`, in this slice's work |
| the content check | `thread@per-crate-changelogs@released-content-check` | `register@knowledge-architect@rejected`, in this slice's work |
| premortem cause 5 | `thread@per-crate-changelogs@citation-free-paths` | claim 7; no tripwire |
| the issue closed | | `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` deleted; its two candidates outside this slice carried into issue entries or argued to need none, in the closing commit |
