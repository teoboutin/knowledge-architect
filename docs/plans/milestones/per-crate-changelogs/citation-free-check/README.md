# Slice 1: files declared free of citations, at two levels, and the root head on history

## Builds

- The manifest table `[walk.citation-free]`, with two lists of globs, `all` and `entries`. `Walk`
  in `path@model@src/manifest.rs` carries `#[serde(deny_unknown_fields)]`, so it gains a field for
  the subtable, and `Manifest::parse` gains the call that dispatches its resolve, which goes in a
  module of its own beside manifest.rs. `[walk.citation-free]` becomes a built-in table, so its name
  is taken from every extension, under `design@check@an-extension-claims-its-manifest-tables`.
- The glob matcher of the table, hand-written in `model`, with its unit tests there; no dependency
  is added.
- The findings of a citation-free file, in `check`, in a module of its own,
  `planned@check@src/check/citation_free.rs`, and the repair each names.
- At the level `entries`, the read of the installed copies for references, and for nothing else,
  where a glob of a project's `entries` matches them, on the owner's ruling of D2 of the milestone
  document.
- The declaration in this repository's knowledge-architect.toml: `all` matching the 11
  CRATES-IO.md files of the crates, `entries` matching the installed copies: the primer, each
  installed skill and each installed agent, as the milestone document's Names define them.
- A reminder in the root CLAUDE.md, beside `instructions@repository-skills`: a finding of the level
  `entries` names an installed copy, whose repair is in its source under content/, then
  `cargo klarch install-agent-skills`; the finding's message is not adapted to this repository.
- The installed setup skill's proposal of a CRATES-IO.md, which also proposes the `all`
  declaration of the page, per `design@agent-skills@setup-default-crates-io-page`.
- `skill@klarch-release`'s step "The shipped text cites no entry": the check replaces the read by
  hand for the md files the check reads, the primer, the skills and the agents; the step's grep
  stays for the saved workflows, the `.js` files of `path@agent-config@workflows/` with the
  installer's prefix, which no walk reads.
- The root design head written from `thread@per-crate-changelogs@no-history-beyond-the-branch`, a
  decision no work implements, recorded when made.
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
3. **The level `entries` gives each reference kind the verdict of the table under "Decided
   design"**: in a matched file, a reference to a design entry, to an issue, a `planned` reference
   and `path@<component>@<path>` naming a declared Component are findings; `path@elsewhere@<path>`,
   a `path` and a `register` reference under the anchor `*` or `plans`, and `register@goal` are not,
   and each admitted span is still resolved, so one that dangles is reported as dangling. The cases
   of the harness kinds, `skill@<shipped skill>`, `agent@<shipped agent>`, `primer@<slug>` and
   `instructions@<slug>`, run in a sandbox copy that serves `claude`, as `Sandbox::serve_claude` in
   `path@testing@tests/binary.rs` does, since every mock project but `unsound` declares
   `harness = []`, under which those words name no kind.
4. **The level `entries` reads an installed copy, and only where a glob matches it**: in that
   sandbox, a reference to an entry planted in an installed skill's copy is a finding with the glob
   declared, and no finding without it; no other rule reports anything in the copy either way.
5. **The id lints keep reading a citation-free file**: a plain word and a backticked span that are
   exactly a defined id are reported in a matched file as in any other; in a file at the level
   `all`, the repair `design@check@backticked-bare-id` and `design@check@reference-shape-outside-span`
   name is a rewrite in words, not the reference form, which the file refuses.
6. **The unanchored-path lint keeps reading a citation-free file**, and in one its repair names a
   rewrite in words, not the `path` form.
7. **No repair a finding of this slice names is the span with its backticks removed**: each names a
   rewrite in words, or a placeholder in angle brackets. Tested on the repair text of each finding.
8. **The glob dialect**: unit tests beside the matcher in `model`, one per rule of the dialect under
   "Decided design", a glob that matches no walked file and a path matched at both levels each a
   finding of the manifest's resolve.
9. **The current install passes**, premortem cause 5: the gate `cargo klarch check` of
   `cargo x gates` on this repository, with the declaration, reports nothing. Shown to fail by
   `cargo x mutate run --file crates/check/src/check/citation_free.rs`, replacing the test that
   admits a harness kind at the level `entries` by `false`, with `cargo klarch check` as the
   command: every `skill@<name>` citation of the installed copies is then reported.
10. **The closing test of the issue this slice answers**: a test in a new file under
    `path@testing@tests/`, `installed_copy_citing_an_entry_is_a_finding`, plants in the sandbox of
    claim 3 a reference to an entry of the sandbox in an installed skill's copy, declares `entries`
    over the installed copies, and asserts that `klarch check` exits 1 with the finding; the `%%`
    comments of content/ that cite an entry, which 23 files of
    `path@agent-skills@content/` hold today (`grep -rlE` of a `%%` line holding a backticked
    reference to a design entry, an issue or a rejected entry), pass, as claim 9's run shows, since content/ is not
    matched and the install removes the comments. Shown to fail, as claim 4 is, by a mutation that
    skips the read of the installed copies at the level `entries`.

## Fixtures

- The `all` cases and the non-harness cases of claim 3: the table `[walk.citation-free]` declared in
  `planted`, the one mock project where a phase-4 defect is planted, per `path@testing@CLAUDE.md`;
  its matched files hold, each in a file of its own, the spans of claims 1, 2, 3, 5, 6 and 7. The
  table is the addition this slice makes to the manifest's vocabulary.
- The harness-kind cases of claim 3 and the cases of claims 4 and 10: a sandbox copy that serves
  `claude`, as `Sandbox::serve_claude` does, with the shipped set installed and the table declared
  in the copy's manifest.
- The glob dialect: inline in the matcher's unit tests.

## Audit subjects

- `path@model@src/manifest.rs`, `path@model@src/model.rs`, `path@model@src/scan.rs`,
  `path@model@src/shipped_set.rs`, `path@check@src/check/references.rs`.
- `design@check@the-regime-has-no-opt-out`, `rejected@check@exempt-directory-manifest-row`,
  `design@agent-skills@plain-text-is-no-repair`, `design@check@reference-shape-outside-span`,
  `design@check@plain-word-lint`, `design@check@backticked-bare-id`,
  `design@check@unanchored-path-first-segment-here`, `design@model@a-reference-is-live-in-every-prose`,
  `design@agent-skills@shipped-text-cites-no-entry`, `design@knowledge-architect@crates-io-page-file`,
  `design@check@installed-files-outside-the-walk`, `design@model@installed-entities-from-the-tree`,
  `design@check@an-extension-claims-its-manifest-tables`,
  `design@check@check-reads-no-range-of-history`, `issue@git@a-shallow-clone-shortens-the-range-commits-judges`.
- `issue@model@manifest-rs-is-past-one-read`, weighed below.

## Fails alone on

- A finding, or its absence, on a citation-free file of a mock project, of the sandbox or of this
  repository.

## Premises that expire

- The shared changelog stays a row of `[walk] skip-files` until slice 5, which removes the rows and
  adds the changelogs to `all`. Nothing of this slice reads it meanwhile; the guard is slice 5's
  claim that this repository's check passes with the changelogs matched.

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@model@manifest-rs-is-past-one-read`, weighed.** The slice changes
  `path@model@src/manifest.rs` by a field of `Walk` and one dispatch in `Manifest::parse`, the
  table's resolve going in a module of its own. The default: no split of manifest.rs is planned
  here. Its argument, from the report of the code-split scout dispatched at the grounding of the
  design discussion, a report kept in that session's scratch space and not committed: the file has
  one reason to change, the manifest's grammar, read through one serde parse, which is the signal
  against a split (manifest.rs line 1 opens "`knowledge-architect.toml` — what a project
  declares"); and the new table adds to its resolvers only the field and the dispatch. The slice's
  design audit dispatches the scout again over the file, and either closes the issue on that argued
  decision or plans the split as a step before the work.

## What is already decided

The design entries this section cites are the seeds of this slice's design audit, as
`skill@knowledge-architect-planning@working-a-slice` takes them: those listed below, and the
design entries of "Audit subjects" restated here so that the audit takes them as seeds,
`design@check@the-regime-has-no-opt-out`, `design@agent-skills@plain-text-is-no-repair`,
`design@check@reference-shape-outside-span`, `design@check@plain-word-lint`,
`design@check@backticked-bare-id`, `design@check@unanchored-path-first-segment-here`,
`design@model@a-reference-is-live-in-every-prose`, `design@agent-skills@shipped-text-cites-no-entry`,
`design@knowledge-architect@crates-io-page-file`, `design@check@installed-files-outside-the-walk`,
`design@model@installed-entities-from-the-tree`, `design@check@an-extension-claims-its-manifest-tables`
and `design@check@check-reads-no-range-of-history`.

Rewritten by this slice, as the milestone document's "Reversed or rewritten" lists them with the
texts each repairs: `design@knowledge-architect@crates-io-page-file`, its rule enforced;
`design@check@installed-files-outside-the-walk` and `design@model@installed-entities-from-the-tree`,
narrowed by the member of the level `entries`, on the owner's ruling of D2;
`design@model@a-reference-is-live-in-every-prose`, narrowed: in a file at the level `all` no span is
resolved, and at the level `entries` a span is resolved as anywhere else and refused where it names
an entry or an anchor of the project.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@check@the-regime-has-no-opt-out`: gains the declaration as a member that adds obligations
  and removes none.
- `design@check@unanchored-path-first-segment-here`: its repair in a citation-free file.
- `design@check@backticked-bare-id` and `design@check@reference-shape-outside-span`: their repair in
  a file at the level `all`, a rewrite in words.
- `design@agent-skills@shipped-text-cites-no-entry`: the rule is now checked, over the installed
  copies.
- `design@agent-skills@setup-default-crates-io-page`: the proposal includes the declaration.
- `design@check@an-extension-claims-its-manifest-tables`: the built-in table takes its name from
  the extensions.
- `design@check@check-reads-no-range-of-history`: cites the new root head, which widens it.

## Decided design

### A file the manifest declares citation-free holds no reference, at one of two levels

From `thread@per-crate-changelogs@citation-free-paths`.

- `[walk.citation-free]` holds two lists of globs, `all` and `entries`. `all` is matched against the
  paths the walk reads; `entries` against those and against the installed copies, which the walk
  leaves out, on the owner's ruling of D2. An illustration of the shape:

  ```toml
  [walk.citation-free]
  all = ["crates/*/CRATES-IO.md"]
  entries = [".claude/knowledge-architect/PRIMER.md", ".claude/skills/knowledge-architect-*/", ".claude/agents/knowledge-architect-*.md"]
  ```

- **The glob dialect**: `*` matches any run of characters within one path segment, never a `/`;
  `**` matches any number of whole segments, none included; a glob ending with `/` names every file
  under that directory. A glob that matches no file it is matched against is a finding of the
  manifest's resolve, as a `skip-files` row that names no file is; so is a path matched by a glob
  of `all` and by one of `entries`. The matcher is hand-written in `model`, with no dependency, and
  its tests are its unit tests there.
- **`all`**: a span that parses as a reference is a finding whether or not it resolves, and no span
  of the file is resolved.
- **`entries`**: each span is resolved as anywhere else, and its kind gives its verdict:

  | span | verdict |
  | --- | --- |
  | `skill@<name>`, `agent@<name>`, `primer@<slug>`, `instructions@<slug>`, the harness kinds | admitted |
  | `path@*@<path>`, `path@plans@<path>`, `register@*@<register>`, `register@plans@<register>`, `register@goal` | admitted |
  | `path@elsewhere@<path>` | admitted: it names no anchor of the project |
  | `planned@<anchor>@<path>` | a finding: the form is legal only under docs/plans/ |
  | a reference to an entry of any register, a plan item included | a finding |
  | a `path` or `register` reference naming an anchor of the project, a Component's or a location's | a finding |
  | a placeholder, a span holding an angle bracket | passes |

  This serves the installed copies, which may cite what every installing project holds, on the
  owner's word at R7. The read of an installed copy at this level raises no finding of any other
  rule, so the reason of `design@check@installed-files-outside-the-walk` keeps every other rule.
- At both levels, a placeholder passes, as `primer@where-knowledge-goes` says of an illustration.
- The id lints keep reading a matched file, and so does the unanchored-path lint. In a file at the
  level `all`, the repair each names is a rewrite in words, since the reference form and the `path`
  form are what the file refuses. The cost accepted: a citation-free file cannot print an existing
  path of the tree in backticks.
- A finding's repair names a rewrite in words or a placeholder, never the span with its backticks
  removed, per `design@agent-skills@plain-text-is-no-repair`.
- The declaration adds obligations and removes none, so it is a member of
  `design@check@the-regime-has-no-opt-out`.
- A generated file is not matched: its findings are its sources'.
- Argument: `argument@per-crate-changelogs@arg-29`, `argument@per-crate-changelogs@arg-32`,
  `argument@per-crate-changelogs@arg-33`, `argument@per-crate-changelogs@arg-44`,
  `argument@per-crate-changelogs@arg-45`, `argument@per-crate-changelogs@arg-46`,
  `argument@per-crate-changelogs@arg-56`, `argument@per-crate-changelogs@arg-58`,
  `argument@per-crate-changelogs@arg-60`. Nearest rival: one level refusing every reference, defeated by the installed
  copies' legitimate citations of what every project holds (R6, R7). Rival of the read, the level
  `entries` applied to content/ with its `%%` comments exempt, defeated by `argument@per-crate-changelogs@arg-60`: it must handle the fragments and an exemption that applies in this repository alone.

### No check reads the history before the base of the branch it judges

From `thread@per-crate-changelogs@no-history-beyond-the-branch`. A head of the root's design home,
with the thread's slug, written in this slice because no work implements it.

- Its scope: every check of the checker, `commits` included, whose range already starts at the
  base, and every gate of the gates library, `path@gates@src/gates.rs`. The gate `rebased`, which
  `--require-rebased` runs, asks git whether the base the project passes, `origin/main` here, is an
  ancestor of `HEAD`, by `git merge-base --is-ancestor`, and is within the rule, and so is any other
  question to git about the branch's base, which
  `issue@git@a-shallow-clone-shortens-the-range-commits-judges` needs.
- Its argument, the owner's at R3: the cost of reading past history grows with every commit the
  project makes, for every future run, with no bound the project controls; and consumers'
  histories, with rewritten tags, shallow clones and imported squashes, would each cost design and
  code. A premise, attributed as the owner's report, with its hedges kept: the owner judged the
  value of such a check relatively low, since in the owner's belief a review would flag a deviation
  easily, even without being asked to watch for it.
- Its rival, a check that a released section equals its content at the version's tag, is ruled
  out in its body with both defeating arguments: the comparison with the tag reads history before
  the branch, which this head refuses; and a rival that compares within the branch alone is
  defeated by the owner's argument that a released text may be reworded without a change of
  meaning (R2), which no comparison of bytes or of sets tells apart, as
  `design@knowledge-architect@changelog-section-fixed-at-release` allows. No `rejected` entry is
  written for it: it meets no recording test of
  `skill@knowledge-architect-decision-recording@losing-alternatives`, and both its refutations are
  arguments a reader rebuilds from this head and that one.
- It widens `design@check@check-reads-no-range-of-history`, which covers `check` alone, and that
  head cites it.
- Argument: `argument@per-crate-changelogs@arg-11`, `argument@per-crate-changelogs@arg-17`,
  `argument@per-crate-changelogs@arg-30`, `argument@per-crate-changelogs@arg-35`,
  `argument@per-crate-changelogs@arg-53`, `argument@per-crate-changelogs@arg-54`.

## Harvest

| what | thread | home |
| --- | --- | --- |
| the declaration, its two levels, its glob dialect, its findings and their repairs | `thread@per-crate-changelogs@citation-free-paths` | a head of `register@check@design` on the citation-free findings, and a head of `register@model@design` on the table, each by the tests of `skill@knowledge-architect-decision-recording`; the rewritten heads and the touched heads listed above |
| the read of the installed copies at the level `entries` | `thread@per-crate-changelogs@citation-free-paths` | the members of `design@check@installed-files-outside-the-walk` and `design@model@installed-entities-from-the-tree`, on the owner's ruling of D2 |
| the narrowing of the live references | `thread@per-crate-changelogs@citation-free-paths` | the rewritten `design@model@a-reference-is-live-in-every-prose` |
| the crates.io page's rule enforced | `thread@per-crate-changelogs@citation-free-paths` | the rewritten `design@knowledge-architect@crates-io-page-file` |
| the rival of one level, and the rival of the read over content/ | `thread@per-crate-changelogs@citation-free-paths` | `register@check@rejected`, if a recording test holds |
| the head on history, with the content check ruled out in its body | `thread@per-crate-changelogs@no-history-beyond-the-branch`, `thread@per-crate-changelogs@released-content-check` | `register@knowledge-architect@design`, in this slice's work |
| premortem cause 5 | `thread@per-crate-changelogs@citation-free-paths` | claim 9; no tripwire |
| the two candidates of the closed issue | | the milestone document's "Later consequences": each an issue entry, or argued in the landing commit to need none |
