# Slice 1: files declared free of citations, at two levels, and the root head on history

## Builds

- The manifest table `[walk.citation-free]`, with two lists of globs, `all` and `entries`. `Walk`
  in `path@model@src/manifest.rs` carries `#[serde(deny_unknown_fields)]`, so it gains a field for
  the subtable, and `Manifest::parse` gains the call that dispatches its resolve, which goes in a
  module of its own beside manifest.rs. `[walk]` is already a built-in table, and an extension
  claims only top-level tables (`Declared.tables` flattens the top-level keys, which
  `extension_tables` reads), so the subtable takes no name from the extensions, and
  `design@check@an-extension-claims-its-manifest-tables` is not touched.
- The glob matcher of the table, hand-written in `model`, with its unit tests there; no dependency
  is added. The two findings of the dialect, a glob that matches no file and a path matched at both
  levels, are phase 2's, in `declarations` of `path@model@src/tree.rs`, beside the finding of a
  `skip-files` row that names no file: `Manifest::parse` reads no file system, and only phase 2
  holds the walked paths and the installed copies.
- The findings of a citation-free file, in `check`, in a module of its own,
  `path@check@src/check/citation_free.rs`, and the repair each names. The arms of `judge_part`,
  `id_lint`, `id_finding` and `link` in `path@check@src/check/references.rs` write the findings and
  the repairs of a span today, so the work writes into them too: each asks the level of its file, and
  in a file at the level `all` names a repair in words.
- Before the tests of the sandbox, a move-only commit takes `klarch`, `Sandbox` and the helpers the
  new test file needs out of `path@testing@tests/binary.rs` into a support module under
  `path@testing@tests/`, which binary.rs and the new file share, as
  `issue@testing@binary-rs-is-past-one-read` says of a support module.
- Before the harvest, the conforming step of the heads the harvest touches, as "What is already
  decided" lists their verdicts.
- At the level `entries`, the read of the installed copies for references, and for nothing else,
  where a glob of a project's `entries` matches them, on D2 of the milestone document, closed by the
  session on the owner's argument `argument@per-crate-changelogs@arg-60`.
- The declaration in this repository's knowledge-architect.toml: `all` matching the 11
  CRATES-IO.md files of the crates, `entries` matching the installed copies: the primer, each
  installed skill and each installed agent, as the milestone document's Names define them.
- A reminder in the root CLAUDE.md, beside `instructions@repository-skills`: a finding of the level
  `entries` names an installed copy, whose repair is in its source under content/, then
  `cargo klarch install-agent-skills`; the finding's message is not adapted to this repository.
- The installed setup skill's proposal of a CRATES-IO.md, which also proposes the `all`
  declaration of the page, in `skill@knowledge-architect-setup@rust-project`.
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
   of the harness kinds, `skill@<shipped skill>`, `agent@<shipped agent>`, `primer@<slug>`, admitted, and
   `instructions@<slug>`, a finding, run in a sandbox copy that serves `claude`, as `Sandbox::serve_claude` in
   `path@testing@tests/support/mod.rs` does, since every mock project but `unsound` declares
   `harness = []`, under which those words name no kind.
4. **The level `entries` reads an installed copy, and only where a glob matches it**: in that
   sandbox, a reference to an entry planted in an installed skill's copy is a finding with the glob
   declared, and no finding without it; no rule of the walk reports anything in the copy either way.
   The comparison of an installed copy's bytes with the shipped set,
   `design@check@installed-files-compared-by-bytes`, reports the planted edit in both runs, as it
   reports any edit of an installed copy.
5. **The id lints keep reading a citation-free file**: a plain word and a backticked span that are
   exactly a defined id are reported in a matched file as in any other; in a file at the level
   `all`, the repair `design@check@backticked-bare-id` and `design@check@reference-shape-outside-span`
   name is a rewrite in words, not the reference form, which the file refuses.
6. **The unanchored-path lint keeps reading a citation-free file**, and in one its repair names a
   rewrite in words, not the `path` form.
7. **No repair a finding of this slice names is the span with its backticks removed**: each names a
   rewrite in words, or a placeholder in angle brackets. Tested on the repair text of each finding.
   In a file at the level `all`, so do the findings of a malformed span, of an anchor in the kind's
   position and of a link, whose repairs name a reference form elsewhere.
8. **The glob dialect**: unit tests beside the matcher in `model`, one per rule of the dialect under
   "Decided design"; a glob that matches no file it is matched against and a path matched at both
   levels each a finding of phase 2, tested in `path@model@src/tree.rs` as the `skip-files` finding
   is.
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
  declares"); and the new table adds to its resolvers only the field and the dispatch. Decided at
  the slice's design audit: no split in this slice. The scout, dispatched again, found the size
  signal holding on the code above the tests alone, and a second part that changes apart, the plan
  layout's names; neither is pushed by this work, which adds a field and one call, while the table's
  resolve goes in a module of its own. The issue stays open, at the same re-entry point, for a work
  that adds more than a dispatch to the file.
- **`issue@testing@binary-rs-is-past-one-read`, weighed at the design audit.** The tests of claims
  3, 4 and 10 need `Sandbox::serve_claude` and `klarch`, private to
  `path@testing@tests/binary.rs`. Decided: they go in a new test file, and the shared helpers move,
  in a move-only commit before them, into a support module. The split by command the issue asks for
  is not taken: this work adds no test of a command to binary.rs, so the issue stays open.
- **`path@check@src/check/references.rs` and `path@model@src/tree.rs`, past one read**, as the
  scout measured at the design audit, with no issue entry: the work writes into both. No split in
  this slice: in references.rs, the citation-free rule changes the finding that one arm writes for
  one observation, as the goals-home rule does, so a review of either rule reads that arm, and the
  rule's own logic goes in `citation_free.rs`; in tree.rs, the glob findings are changes that
  `declarations`, one process over the declared paths, applies. Each gets an issue entry.

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
narrowed by the read of the installed copies at the level `entries`, on D2, closed on
`argument@per-crate-changelogs@arg-60`, which the harvest records as a head of its own,
`design@check@installed-copies-read-at-the-level-entries`, the two heads pointing to it;
`design@model@a-reference-is-live-in-every-prose`, narrowed: in a file at the level `all` no span is
resolved, and at the level `entries` a span is refused where it names an entry or an anchor of the
project and takes the level's finding alone, and a span the level admits is resolved as anywhere
else.

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
- `design@check@check-reads-no-range-of-history`: cites the new root head, which widens it.

Added by the slice's design audit, from the verdicts of its record search, each a head whose text
states a rule the slice narrows:

- `design@model@section-homes-carry-slugs`: its bullet "An installed copy defines its sections and
  raises no finding", narrowed by the level `entries`.
- `design@check@plain-word-lint`: its repair in a file at the level `all`, a rewrite in words.
- `design@model@every-path-names-its-anchor`: its paragraph on a fenced path reference, whose
  escape-anchor repair a file at the level `all` refuses.
- `design@model@an-unresolved-reference-is-reported` and
  `design@model@an-unresolved-reference-is-never-dropped`: in a file at the level `all`, a span
  that resolves to nothing takes the citation-free finding and its repair in words.
- `design@model@at-span-candidate-rule`: "a known kind is segmented and resolved", which a file at
  the level `all` narrows to segmented.
- `design@model@a-harvested-slice-is-frozen`: its analogy, a frozen file read for its definitions
  alone "as an installed file is", which a declaring project's level `entries` makes false.

Judged untouched at the design audit:

- `design@check@an-extension-claims-its-manifest-tables`: the subtable takes no name from the
  extensions, as "Builds" says.
- `design@check@phases-gate-the-report`: the sentence of `path@check@docs/design/run-model.md`
  that the milestone document lists, "An installed copy raises no finding of phase 3", stays true,
  since a finding of the level `entries` is phase 4's.

The verdicts of `agent@knowledge-architect-design-record-auditor` on these heads, at the design
audit, and the conforming step each owes before the harvest:

| head | verdict | the step |
| --- | --- | --- |
| `design@check@the-regime-has-no-opt-out` | conforms | none |
| `design@check@check-reads-no-range-of-history` | conforms | none |
| `design@check@unanchored-path-first-segment-here` | two decisions: the first-segment rule, and what a path-shaped span is | split: the definition moves, unchanged but for three rewordings, "so each is a finding" read "so each is path-shaped", "outside the lint on purpose" read "not path-shaped, on purpose" and "a census of what it reports" read "a census of what the widened shape would report", and its title and its four shapes reworded on the advice of the auditor that checked it, into a new head of `register@model@design`, `design@model@path-shaped-span`, since `path_shaped` in `path@model@src/scan.rs` is `model`'s code; the citing sites re-pointed |
| `design@check@backticked-bare-id` | two decisions: its last paragraph, that `check --fix` writes nothing for the three lints on ids | split: a new head of `register@check@design`, `design@check@id-lints-write-no-fix`, after `design@check@quotation-read-by-no-id-lint`; the citing sites re-pointed, those of `spec@plans@path-quickfixes` included |
| `design@check@reference-shape-outside-span` | its title leaves out its exception beside an issue reference; its census names no command that re-takes it; it holds deliberation | retitled, slug kept; the re-take command added; "which a retrospective proposed" deleted |
| `design@agent-skills@shipped-text-cites-no-entry` | names the owner as the source of an argument | the attribution removed, the argument kept |
| `setup-default-crates-io-page` | no entry test admits it, and it quotes the owner | removed from the design home, its citations re-pointed to `skill@knowledge-architect-setup@rust-project`: D3 of the milestone document |
| `design@model@section-homes-carry-slugs` | quotes the owner, and names the owner as the source of an argument | the quotations replaced by the arguments they made, unattributed |
| `design@check@plain-word-lint` | its census names no command that re-takes it | the re-take command added |
| `design@model@every-path-names-its-anchor` | two decisions: its paragraph that a plan document is cited by its kind, never by its path; and its code is `model`'s alone | split: a new head, `design@model@plan-document-path-citation-refused`, after it, its citing sites re-pointed; the Component of both heads is D6 of the milestone document |
| `design@model@an-unresolved-reference-is-reported` | conforms | none |
| `design@model@an-unresolved-reference-is-never-dropped` | conforms | none |
| `design@model@at-span-candidate-rule` | its title leaves out that a span holding whitespace or an angle bracket is no candidate | retitled, slug kept |
| `design@model@a-harvested-slice-is-frozen` | names the owner's approval of the session's reading of a goal, and tells how the decision came about | the history removed, the reading and its rival kept |

## Decided design

### A file the manifest declares citation-free holds no reference, at one of two levels

From `thread@per-crate-changelogs@citation-free-paths`.

- `[walk.citation-free]` holds two lists of globs, `all` and `entries`. `all` is matched against the
  paths the walk reads; `entries` against those and against the installed copies, which the walk
  leaves out, on D2, closed on `argument@per-crate-changelogs@arg-60`. An illustration of the shape:

  ```toml
  [walk.citation-free]
  all = ["crates/*/CRATES-IO.md"]
  entries = [".claude/knowledge-architect/PRIMER.md", ".claude/skills/knowledge-architect-*/", ".claude/agents/knowledge-architect-*.md"]
  ```

- **The glob dialect**: `*` matches any run of characters within one path segment, never a `/`;
  `**` matches any number of whole segments, none included; a glob ending with `/` names every file
  under that directory. The matcher is hand-written in `model`, with no dependency, and its tests
  are its unit tests there. A glob that matches no file it is matched against is a finding, and so
  is a path matched by a glob of `all` and by one of `entries`. The two findings are phase 2's, in
  `declarations` of `path@model@src/tree.rs`, beside the finding of a `skip-files` row that names
  no file, since the manifest's resolve reads no file.
- **`all`**: a span that parses as a reference is a finding whether or not it resolves, and no span
  of the file is resolved. Every finding a span or a link raises in such a file names a repair in
  words or a placeholder, a malformed span's, an anchor in the kind's position's and a link's
  included.
- **`entries`**: a span's kind and its anchor give its verdict. A span the level admits is resolved
  as anywhere else in a walked file; one it refuses takes the level's finding alone; in an
  installed copy no span is resolved:

  | span | verdict |
  | --- | --- |
  | `skill@<name>`, `agent@<name>`, `primer@<slug>`, the harness kinds every installing project holds | admitted |
  | `instructions@<slug>` | a finding: it names a section of this project's root CLAUDE.md, which an installing project does not hold, per `design@agent-skills@shipped-text-cites-no-entry` |
  | `path@*@<path>`, `path@plans@<path>`, `register@*@<register>`, `register@plans@<register>`, `register@goal` | admitted |
  | `path@elsewhere@<path>` | admitted: it names no anchor of the project |
  | `planned@<anchor>@<path>` | a finding: the form is legal only under docs/plans/ |
  | a reference to an entry of any register, a plan item included | a finding |
  | a `path` or `register` reference naming an anchor of the project, a Component's or a location's | a finding |
  | a placeholder, a span holding an angle bracket | passes |

  This serves the installed copies, which may cite what every installing project holds, on the
  owner's word at R7. No rule of the walk reads an installed copy, and the only other check that
  reads it is the comparison of its bytes, `design@check@installed-files-compared-by-bytes`, so the
  reason of `design@check@installed-files-outside-the-walk` keeps every other rule: an admitted span
  that dangles is reported in a walked file the level matches, as anywhere else, and not in an
  installed copy, whose resolution within the shipped set is the shipped-set test's.
  The level enforces `design@agent-skills@shipped-text-cites-no-entry` in part: it admits every
  `path@*@<path>`, where that head admits only a path every conforming project holds, which no
  checker can know of another project, so that part stays the release's review.
  Under `commits`, an earlier commit's installed copies are read at this level by the tip's checker
  where the manifest that commit holds declares the glob.
- At both levels, a placeholder passes, as `primer@where-knowledge-goes` says of an illustration.
- The id lints keep reading a matched file, and so does the unanchored-path lint. No repair names
  a form the file's level refuses: in a file a glob matches, at either level, the findings of the
  id lints, of the unanchored-path lint, of an anchor in the kind's position and of a link name a
  rewrite in words, the session's decision at the review before the merge. The cost accepted: a
  file at the level `all` cannot print an existing path of the tree in backticks, and one at the
  level `entries` only in a form the level admits.
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
  code. A premise of the rival below, attributed as the owner's report, with its hedges and its
  scope kept: the owner judged the value of the released-content check relatively low, since in the
  owner's belief any review would flag a deviation in that case easily, even without being asked
  to watch for it.
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
| the declaration, its two levels, its glob dialect, its findings and their repairs | `thread@per-crate-changelogs@citation-free-paths` | a head of `register@check@design` on the citation-free findings, and two heads of `register@model@design`, on the table and on its glob dialect, each by the tests of `skill@knowledge-architect-decision-recording`; the rewritten heads and the touched heads listed above |
| the read of the installed copies at the level `entries` | `thread@per-crate-changelogs@citation-free-paths` | a head of `register@check@design`, `design@check@installed-copies-read-at-the-level-entries`, to which `design@check@installed-files-outside-the-walk` and `design@model@installed-entities-from-the-tree` point, on D2, closed on `argument@per-crate-changelogs@arg-60`; planned as a member of both, and given a head at the review before the merge, since it loses to a rival of its own, `rejected@check@citation-free-entries-over-the-source` |
| the narrowing of the live references | `thread@per-crate-changelogs@citation-free-paths` | the rewritten `design@model@a-reference-is-live-in-every-prose` |
| the crates.io page's rule enforced | `thread@per-crate-changelogs@citation-free-paths` | the rewritten `design@knowledge-architect@crates-io-page-file` |
| the rival of one level, and the rival of the read over content/ | `thread@per-crate-changelogs@citation-free-paths` | `register@check@rejected`, if a recording test holds |
| the head on history, with the content check ruled out in its body | `thread@per-crate-changelogs@no-history-beyond-the-branch`, `thread@per-crate-changelogs@released-content-check` | `register@knowledge-architect@design`, in this slice's work |
| premortem cause 5 | `thread@per-crate-changelogs@citation-free-paths` | claim 9; no tripwire |
| the two candidates of the closed issue | | the milestone document's "Later consequences": each an issue entry, or argued in the landing commit to need none; the guard on placeholder spans weighed against `rejected@model@placeholder-literal-part-check`, which it brings back unless a new argument is found |
| the repairs in words of a file at the level `all`, which the respelling of `spec@plans@path-quickfixes` would refuse there | | a line in the What of `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` and of `issue@knowledge-architect@path-quickfixes-read-against-the-renamed-checker`, added at the landing |
