# Slice 3: the changelog's model, the `[changelog]` table and the checks, opt-in

## Builds

- In `model`:
  - the resolve of the manifest table `[changelog]`, in a module of its own beside
    `path@model@src/manifest.rs`. `Declared` there flattens every top-level table it does not
    name into `tables`, for an extension to claim, and phase 1 refuses one no extension claims, so
    `Declared` gains a field for `[changelog]`, and `Manifest::parse` the call that dispatches its
    resolve. `[changelog]` becomes a built-in table, which takes the name from every extension,
    under `design@model@an-extension-claims-its-manifest-tables`;
  - the read of the `Cargo.toml` of each Component, where `[changelog]` is declared, through
    `cargo-reader`, which `model` now depends on. The survey of `path@model@src/survey.rs` already
    holds each `Cargo.toml` with its text in `outside`, on the working tree and in the snapshots of
    `--staged` and `commits`, since `read_tree` of `path@check@src/commands/history.rs` reads every
    file no walk row keeps out, so the read takes its text from there and no gathering changes.
    A package that inherits a field reads the workspace's `[workspace.package]` from the nearest
    `Cargo.toml` at or above the Component's directory, within the project, that holds a
    `[workspace]` table, per `design@cargo-reader@the-caller-finds-the-workspace-manifest`. A
    `Cargo.toml` that `cargo-reader` refuses is a finding of the last phase carrying its reason, never
    a failure of the model's build, so a parent tree's assembly does not fail on it;
  - the parse of a crate's changelog, which records its preamble, its version sections, its entries
    and its unread lines, and raises no finding;
  - the rendering of the assembled layout from the parsed changelogs, a pure function the `query`
    command of slice 4 also calls, in a module of its own, not in `path@model@src/index.rs`, whose
    reference is the register indexes' heads;
  - the assembled file among the generated paths, `generated_paths` of `path@model@src/index.rs`,
    written by `cargo klarch index` and `cargo klarch check --fix`, each refusing under the same
    conditions.
- In `check`, checks of the last phase: every finding below. The bump check, which reads Cargo's
  semver, and the `include` check, which reads Cargo's `include` patterns, are each a module of
  its own, since each brings an external reference.
- In `cargo-reader`, the match of an `include` pattern against a package-relative path, as Cargo
  reads the pattern, which the `include` check calls, per
  `design@knowledge-architect@tools-read-cargo-through-the-crate`; its reading of Cargo in the
  crate's grounding file.
- No change to this repository's manifest: it declares no `[changelog]` until slice 5, so nothing of
  this slice runs over this repository. Every test is over mock projects.
- The readings of Cargo's semver compatibility the bump check makes, in a grounding file of `check`.
- The new tests of the `klarch` binary in a test file of their own in `testing`, never in
  `path@testing@tests/binary.rs`, which `issue@testing@binary-rs-is-past-one-read` holds past one
  read.
- The entries of this slice in the shared changelog's `Next release` section, under its current
  grammar.

## Claims

Each test is shown to fail against a wrong implementation by a mutation of the code it guards, run
with `cargo x mutate`.

1. **Opt-in**: a mock project with published crates, a `CHANGELOG.md` of another shape and no
   `[changelog]` table yields no changelog finding, and no file is assembled.
2. **The changelog crates**: with `components` listed, exactly those Components are read; with
   none listed, exactly those whose `Cargo.toml` has a `[package]` table that Cargo would publish,
   by the rule of "The changelog crates, the opt-in and the table"; a changelog crate with no
   `CHANGELOG.md` is a finding.
3. **The parse raises no phase finding**: a mock project whose changelog holds a misspelt category
   and a line of no grammar passes phases 1 to 3, so `klarch id` and `klarch worklist` answer over
   it, and the last phase reports each line.
4. **The grammar**: an entry of an undeclared category, of a category used by a Component outside
   its `[changelog.category-components]` list, and of a class other than patch, minor and major are
   each a finding; a version heading out of order, and a version heading with no entry, are each a
   finding; a version that `unclassed` lists admits entries with no tag, and no other version does.
5. **The bump check under the lockstep**: in a mock project whose changelog crates all set
   `version.workspace = true`, a released version that follows its predecessor over every crate by
   less or more than its highest class is a finding, and one that follows it exactly is not, with
   0.x read as Cargo reads it; a crate's file that skips a version is no finding.
6. **The bump check per crate**: where a changelog crate does not inherit the workspace version,
   each crate's file is judged alone.
7. **The `include` check**: a changelog crate whose `include` matches no `CHANGELOG.md` is a
   finding; one with no `include` is not.
8. **The preamble**: a preamble that differs from the one the manifest determines is a finding
   whose message gives the expected text whole, and `--fix` writes no byte of the crate's
   changelog; the expected preamble lists the categories in their declared order and the
   `unclassed` versions, names the configured assembled path in words, and gives `assembled-url`
   where set.
9. **The assembled file**: with two changelog crates or more, a stale or missing assembled file is
   a finding and `--fix` writes it in the layout below; with one, nothing is assembled.
10. **The writers' refusals**: `cargo klarch check --fix`, `cargo klarch index` and
    `cargo klarch index --staged` each refuse to write the assembled file over a file at its path,
    in the tree each writes, that lacks the generated header, and refuse to write it while any
    changelog holds an unread line or an entry whose category the manifest does not declare for its
    crate, naming the line; no refusal writes any byte.
11. **The rendering round-trips**: the assembled text of a mock project, parsed back per crate,
    yields every entry of every crate with its category, class and text unchanged.
12. **The changelog crates by Cargo's rule**: with `components` absent, a mock Component with
    `publish = []`, and one with no `version`, are no changelog crate; one with
    `publish = ["<registry>"]` is one.
13. **What a changelog crate's files give**: a changelog crate whose `CHANGELOG.md` a `[walk]` row
    keeps out is a finding naming the row, never one that reports the file missing; a changelog
    crate whose `Cargo.toml` `cargo-reader` refuses is a finding carrying its reason, and `klarch id`
    answers over it.

## Fixtures

The `[changelog]` table declared in `planted`, the one mock project where a phase-4 defect is
planted, per `path@testing@CLAUDE.md`, which says the mock projects declare no table `check` does
not own; `[changelog]` is a built-in table, so `planted` may declare it. Three of its Components
hold a `Cargo.toml` and a `CHANGELOG.md`: one with `publish = false`, two published, one of which
restricts nothing and one of which carries the restricted category. Mock projects hold no
`Cargo.toml` today: the addition this slice makes is that file in a mock Component's directory,
read only where `[changelog]` is declared. The defects of claims 4, 5, 7 and 8 are planted in
`planted`, each in a file of its own. The states no committed fixture holds, claim 3's phase-1 to
phase-3 pass over a malformed entry, claim 9's stale and missing assembled file, claim 10's file
without the header, and claim 12's Cargo forms, are built in copies by the tests, as
`path@testing@CLAUDE.md` says of a tree's state such as an absent or edited generated file.

## Audit subjects

- `path@model@src/manifest.rs`, `path@model@src/walk.rs`, `path@model@src/index.rs`,
  `path@model@src/phases.rs`, `path@check@src/commands/history.rs`, `path@check@src/check/generated.rs`.
- `design@knowledge-architect@model-crate`, `design@check@phases-gate-the-report`,
  `design@check@nothing-of-a-project-is-compiled-in`, `design@check@the-regime-has-no-opt-out`,
  `design@model@components-carry-the-same-documents`, `design@model@registers-are-declared`,
  `design@model@git-supplies-the-walk`, `design@model@one-list-of-generated-files`,
  `design@check@fix-before-the-checks`, `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`,
  `design@create@init-asks-no-confirmation`, `design@check@staged-tree-source`,
  `design@check@model-then-checks`, `design@knowledge-architect@versioning-policy`,
  `design@check@safe-fix-definition`, `design@check@fix-scope`,
  `design@model@an-extension-claims-its-manifest-tables`.
- `issue@model@the-structural-findings-might-be-checked-at-scan-time`,
  `issue@model@the-walk-may-exclude-a-file-the-model-requires`,
  `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`,
  `issue@knowledge-architect@a-conflict-in-a-generated-file-is-resolved-by-hand`.
- `issue@model@manifest-rs-is-past-one-read` and `issue@check@history-rs-is-past-one-read`, weighed
  below, and `issue@check@commands-mod-rs-holds-two-parts-that-change-apart`, weighed below.

## Fails alone on

- A changelog finding, or its absence, over a mock project; or the bytes of a mock project's
  assembled file.

## Premises that expire

- This repository declares no `[changelog]`, so its root CHANGELOG.md, a row of `skip-files`, is
  not the assembled file until slice 5. The guard: claim 10's refusals, which keep the first
  `--fix` or `index` of slice 5 from writing over the shared changelog unless slice 5 first
  replaces it.

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@model@manifest-rs-is-past-one-read`, weighed.** The slice adds one more dispatch to
  `path@model@src/manifest.rs`, the table's resolve going in a module of its own. Where slice 1
  closed the issue on its argued decision, this slice weighs the file again only if its scout
  reports a new signal; otherwise its design audit weighs it as slice 1's spec says.
- **`issue@check@history-rs-is-past-one-read`, weighed.** The snapshots must gather each
  Component's `Cargo.toml`. The code-split scout found that `read_tree` of
  `path@check@src/commands/history.rs` already reads every file the walk does not skip, so the
  gathering does not change. The module's code changes at its two callers of `generated_list`,
  which wrap its error with `EXTENSION_REPAIR`: each takes the repair the refusal carries. That
  adds no part to the module and no second reference, so no split is planned, and the issue stays
  open at its re-entry point, its size signal standing, as the design audit found.
- **`issue@check@commands-mod-rs-holds-two-parts-that-change-apart`, weighed.** Its re-entry point
  is met: `generated_list` in `path@check@src/commands/mod.rs` gains the assembled file. The
  refusals over the assembled path and over an unread line are computed by a function of `model`
  over the parsed changelogs and the generated texts `Inputs` already holds, `committed` and
  `generated_unreadable`, so the change to the runs part is one call and an error that carries its
  own repair. It adds no part, and the milestone document keeps the split for slice 4, so the
  issue stays open.
- **`issue@model@manifest-rs-is-past-one-read`, weighed.** The scout reports no new signal: the
  table's resolve goes in a module of its own; `path@model@src/manifest.rs` gains a field of
  `Declared`, one call in `Manifest::parse` after `resolve_components`, since `components` and
  `category-components` name Components, and an accessor. The issue stays open.

## What is already decided

The seeds of this slice's design audit are the design entries this section cites, as
`skill@knowledge-architect-planning@working-a-slice` takes them; those of "Audit subjects" it does
not cite below are restated here: `design@check@phases-gate-the-report`,
`design@check@the-regime-has-no-opt-out`, `design@model@components-carry-the-same-documents`,
`design@model@one-list-of-generated-files`, `design@check@fix-before-the-checks`,
`design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`,
`design@create@init-asks-no-confirmation`, `design@check@staged-tree-source`,
`design@knowledge-architect@versioning-policy`, `design@check@safe-fix-definition`,
`design@check@fix-scope` and `design@model@an-extension-claims-its-manifest-tables`.

Rewritten by this slice, as the milestone document's "Reversed or rewritten" lists them with the
texts each repairs:

- `design@model@git-supplies-the-walk`: "No generated path is declared" admits the assembled path,
  the one generated path the manifest declares. `tree` does not assert that it exists: the head's
  premise, that a generated path cannot be asserted to exist, since a fresh clone may have none,
  holds for it too. The generated-files check reports it missing with its own finding, "the
  generated file is missing" (`MISSING` in `path@check@src/check/generated.rs`), or out of date.
- `design@model@registers-are-declared` and `design@check@nothing-of-a-project-is-compiled-in`:
  the set of changelog crates is declared in the manifest by `[changelog]`, opt-in, as "the
  published crates", whose list each `Cargo.toml` already declares; a second list by hand drifts,
  and the drift it would hide, a new published crate with no changelog, is what the check exists to
  report. The second head also gains its sixth compiled-in member, the changelog's grammar and
  classes.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@knowledge-architect@partial-publish-rerun`: its publish order gains `cargo-reader` before
  `model`, which this slice makes depend on it.
- `design@knowledge-architect@model-crate`: "it depends on `git` alone" becomes `git` and `cargo-reader`;
  and it gains the member that divides the changelog's work, every parse and rendering in `model`,
  raising no finding, every finding in `check`'s last phase, as the line between phases 1 to 3 and
  the checks it holds applies.
- `design@check@the-regime-has-no-opt-out`: gains `[changelog]` as a declaration that adds
  obligations and removes none.
- `design@model@one-list-of-generated-files`: its list gains the assembled file.
- `design@check@fix-before-the-checks`: gains the refusal while a line is unread.
- `design@model@an-extension-claims-its-manifest-tables`: the built-in table takes the name
  `changelog` from the extensions.
- `design@check@model-then-checks`: its example "such as a changelog … the extension hands it the
  text" becomes false, since the changelog is part of the model.
- `design@model@components-carry-the-same-documents`: its "Nothing is discovered by filename"
  needs no reading of `publish`, since a changelog crate is picked among the declared Components by
  what its `Cargo.toml` holds, as the design audit found; its sentence on what a project adds to the
  set cites the head the harvest writes on the opt-in.
- `design@check@index-takes-no-dry-run`: its refusals gain the two over the assembled file.
- `design@knowledge-architect@cargo-reader-crate`: its dependents gain `model`, a member its
  argument covers, "The other tools read Cargo through it".
- `design@check@checker-source-literals-are-data`: its list of the libraries `klarch` links omits
  `cargo-reader`, which `path@cli@src/main.rs` passes as a checker directory since slice 2; repaired
  in a commit of its own, as wording now false.

The design audit judged the heads above, by `agent@knowledge-architect-design-record-auditor` over
the first eight, and a conforming step before the harvest brings these to the rules:

- `design@check@the-regime-has-no-opt-out`: "checked exactly as it was" becomes "checked exactly as
  under the default", twice; its Component is D9 of the milestone document.
- `design@model@one-list-of-generated-files`: the paragraph "An instance with no directory
  contributes no index", which passes no entry test and which the comment at
  `file_register_indexes` holds, leaves the head.
- `design@check@fix-before-the-checks`: retitled, since its title lists the steps of the run; its
  paragraph on the exit code after a write becomes a head of its own, and its paragraph on the
  deletion it leaves unstaged becomes another, whose attribution is D11.
- `design@model@an-extension-claims-its-manifest-tables`: its Component is D10; its opening
  sentence names the built-in tables, which `model`'s `Declared` holds.
- `design@check@model-then-checks`: its figures over thaum, which no command takes again, leave the
  head, and its "Four things follow" becomes three and one exception.
- `design@model@components-carry-the-same-documents`: retitled, slug kept, since its body admits
  documents only some Components carry.

`design@knowledge-architect@partial-publish-rerun` and `design@knowledge-architect@model-crate`
conform. The second holds one decision, the crate and the line that bounds it, so its touch owes
no split under `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it`;
its list of the crates that depend on `model` omits `testing`, which the harvest completes.

## Decided design

### The changelog crates, the opt-in and the table

From `thread@per-crate-changelogs@changelog-model-home` and
`thread@per-crate-changelogs@changelog-opt-in`.

- The changelog checks run only in a project whose manifest declares `[changelog]`. A project that
  declares none is asked nothing, so the work is no new ask of an adopting project.
- The changelog crates are those `components` lists, in the manifest's order of the Components; or,
  where it lists none, every Component whose `Cargo.toml` has a `[package]` table that Cargo would
  publish. Read from the Cargo reference, manifest chapter, "The version field" and "The publish
  field", as the stable toolchain's documentation holds it: `version` is optional and required for
  publishing, and omitting it keeps a package unpublished; `publish = false` disables publishing;
  `publish` as a list names the registries a package may be published to. So a package is a
  changelog crate where it sets a `version`, its own or inherited, and `publish` is absent, `true`,
  or a list naming a registry; an empty list names none, and is read as `false`, a reading of the
  list form the reference does not state in words, recorded in the grounding file of `cargo-reader`.
  Reading `publish` is no discovery by filename, and it reports a new published crate with no
  changelog, which a hand list would miss.
- A changelog crate with no `CHANGELOG.md` is a finding. One whose `CHANGELOG.md` a `[walk]` row
  keeps out is a finding naming the row, with its removal as the repair, never one that reports the
  file missing, in the direction `issue@model@the-walk-may-exclude-a-file-the-model-requires`
  records, which stays open for the files it lists.
- `categories` is required in the table: the categories are the project's, so none is compiled in,
  per `design@check@nothing-of-a-project-is-compiled-in`.
- `[changelog.category-components]` restricts a category to the Components it lists; a category it
  does not name is open to every changelog crate.
- `unclassed` lists the versions whose entries carry no category and no class, such as this
  repository's 0.1.0.
- An illustration of the table, as slice 5 declares it here:

  ```toml
  [changelog]
  assembled = "CHANGELOG.md"
  categories = ["Migration", "New features", "Workflow"]
  unclassed = ["0.1.0"]

  [changelog.category-components]
  Workflow = ["agent-skills"]
  ```

- Argument: `argument@per-crate-changelogs@arg-16`, `argument@per-crate-changelogs@arg-42`.
  Nearest rival: the checks on in every project with published crates, defeated because every
  adopter with its own convention would fail its next pin move.

### The grammar and the classes are compiled in

From `thread@per-crate-changelogs@changelog-grammar-compiled-in` and
`thread@per-crate-changelogs@changelog-entry-categories`.

- A crate's changelog: the preamble, then `## Next release`, then one `## <version>` per released
  version that has an entry in the crate, in descending order. A version heading with no entry is a
  finding, per `thread@per-crate-changelogs@absent-version-section`. A version heading is a line
  markdown reads as a level-two heading, per `design@model@headings-open-with-hash-marks`, so a
  `## ` line inside a fenced block of an entry's text is text.
- An entry is one list item, `- <category>, <class>: <text>`, its text continuing on the lines
  indented under it; in a version `unclassed` lists, `- <text>`. No subsection heading.
- The classes are patch, minor and major: Cargo's semver, which `goal@serves-one-kind-of-project`
  fixes. The grammar is the tool's convention for a kind of document, as the plans layout is. Both
  are the sixth member of `design@check@nothing-of-a-project-is-compiled-in`.
- The test each category applies, such as what earns a Migration entry, stays in the workflow's
  text, since no mechanical check can apply it.
- The declaration of the categories adds obligations and removes none, a member of
  `design@check@the-regime-has-no-opt-out`.
- Argument: `argument@per-crate-changelogs@arg-15`, `argument@per-crate-changelogs@arg-22`,
  `argument@per-crate-changelogs@arg-43`, `argument@per-crate-changelogs@arg-44`. Nearest rival:
  a category heading in each crate's file, defeated because a crate with three entries would carry
  three headings and Workflow would exist in one file.

### Every finding is the last phase's; the parse records what it could not read

From `thread@per-crate-changelogs@changelog-findings-in-check`.

- The parse in `model` records every line it reads and every line it cannot, and raises no finding
  of phases 1 to 3, since nothing in the model depends on a changelog's content and
  `design@knowledge-architect@model-crate` gives those phases whether the model can be trusted.
- Every finding of this slice is a check of `check`'s last phase. So `id` and `worklist` of `query`,
  which call `complete_working_tree`, answer over a malformed entry.
- A missing `categories` key, or a value of the table of the wrong type, is a finding of the
  manifest's resolve, as for any table: it is about the manifest, not about a changelog's content.
- Argument: `argument@per-crate-changelogs@arg-37`.

### The bump check reads the tree alone

From `thread@per-crate-changelogs@changelog-bump-check` and
`thread@per-crate-changelogs@absent-version-section`.

- Under the lockstep, where every changelog crate sets `version.workspace = true`: the versions are
  those every crate's file holds together; each released version's class is the highest among its
  entries over every crate, and it must follow the next lower version by that class. A crate's file
  may skip a version.
- Otherwise, each crate's file is judged alone, the same way.
- Following by a class: at 1.0 and above, major bumps the first number, minor the second, patch the
  third, each resetting those after it; under 0.x, major and minor both bump the second number and
  patch the third, as Cargo treats them.
- A version `unclassed` lists, and the oldest version, are not judged; `Next release` carries no
  version and is not judged.
- No history is read, per `thread@per-crate-changelogs@no-history-beyond-the-branch`.
- Argument: `argument@per-crate-changelogs@arg-16`, `argument@per-crate-changelogs@arg-24`.

### A crate that sets `include` lists its changelog

From `thread@per-crate-changelogs@changelog-include-check`. A changelog crate whose `Cargo.toml`
sets `include`, with no pattern matching its `CHANGELOG.md` as Cargo reads the pattern, is a
finding. It replaces the assertion of the test in `path@xtask@src/changelog.rs`, which slice 5
removes. Argument: `argument@per-crate-changelogs@arg-40`.

### The preamble is the checker's

From `thread@per-crate-changelogs@category-order-in-preamble` and
`thread@per-crate-changelogs@changelog-the-pin-reads`.

- A crate's preamble, its lines above the first version heading, is determined by the manifest: it
  names the crate's package; says the file holds that crate's entries only; where a file is
  assembled, names it in words by the path `assembled` configures, "the changelog of the whole
  project is <assembled path> in its repository", with `assembled-url` as a link where set; lists
  the categories in their order; and lists the `unclassed` versions, where any.
- A consumer's `klarch changelog` holds no manifest of the publisher, since no crate ships
  knowledge-architect.toml, so it parses and renders by the preamble: each crate's file is
  self-contained.
- A preamble that differs, by more than its line endings, as
  `design@check@generated-text-compared-without-line-endings` reads a generated text, is a finding
  whose message gives the expected text whole; the writer repairs it. `--fix` writes no byte of a crate's changelog, which is hand-written: per
  `design@check@safe-fix-definition`, a fix that touches a hand-written file would rewrite what a
  writer meant, and `design@check@fix-scope` and
  `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` admit only installed
  and generated files. The assembled file stays the one generated file of this work.
- No URL is built from `repository`, and `repository` is required nowhere (R5, R6).
- An illustration of a preamble:

  ```markdown
  # Changelog of knowledge-architect-check

  This file holds the entries of this crate only. The changelog of the whole project is
  CHANGELOG.md in its repository.

  Categories, in order: Migration, New features, Workflow. Versions with no category and no class:
  0.1.0.
  ```

- Argument: `argument@per-crate-changelogs@arg-41`, `argument@per-crate-changelogs@arg-49`.

### The assembled file: where, in which layout, and written how

From `thread@per-crate-changelogs@assembled-changelog-location`,
`thread@per-crate-changelogs@assembled-changelog-layout` and
`thread@per-crate-changelogs@assembled-file-not-overwritten`.

- **Where**: at the path `assembled` names, `CHANGELOG.md` at the root where the key is absent,
  spelled as git lists it and read by `normalise_one` of `path@model@src/manifest.rs`, as a `[walk]`
  row is, per `design@model@git-supplies-the-walk`.
  With one changelog crate, nothing is assembled, whatever the key.
- **Layout**: a generated header, as an `index.md` opens with one; then each version, `Next
  release` first, as `## <version>`; under it each category in its declared order, as `###
  <category>`; under it each changelog crate in the manifest's order of the Components, as `####
  <package name>`, the published name with its prefix; under it the crate's entries of that category,
  each `- <class>: <text>`, in the order of the crate's file. A version `unclassed` lists has no
  category level: `#### <package name>`, then `- <text>`. An empty level is omitted.
- **Written**: by `cargo klarch index` and `cargo klarch check --fix`, from the parsed changelogs;
  checked current by the check; outside the walk by construction, as an `index.md` is; listed in
  `generated_paths`. A missing assembled file is reported by the generated-files check with its own
  finding, "the generated file is missing", as a missing `index.md` is, and a stale one as out of
  date; `tree` does not assert that the declared path exists, per the rewritten
  `design@model@git-supplies-the-walk` above. Each finding names the repair the register indexes'
  name for the tree judged, `index_action` of `path@check@src/check/generated.rs`.
- **Refusals**: `cargo klarch check --fix` and `cargo klarch index` each refuse to write over a file
  at the path that lacks the generated header,
  per `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`, and refuses to
  write while any changelog holds an unread line, or an entry whose category the manifest does not
  declare for its crate, per `design@check@fix-before-the-checks`: the assembled file would omit
  either. Each refusal names its repair. `cargo klarch index --staged` applies both to the staged
  tree, its file at the path read from the staged blob. Both are computed in `model` from the
  parsed changelogs and the generated texts the gathering already reads, so the three writers
  refuse by one function.
- **Committed**: no crate ships it, so `rejected@knowledge-architect@changelog-copies-written-at-packaging`
  does not bear on it; it is committed for the readers of the repository.
- An illustration of the layout:

  ```markdown
  ## 0.11.0

  ### Migration

  #### knowledge-architect-check

  - minor: …

  #### knowledge-architect-agent-skills

  - patch: …

  ### New features

  …
  ```

- Argument: `argument@per-crate-changelogs@arg-8`, `argument@per-crate-changelogs@arg-14`,
  `argument@per-crate-changelogs@arg-19`, `argument@per-crate-changelogs@arg-21`,
  `argument@per-crate-changelogs@arg-27`, `argument@per-crate-changelogs@arg-48`. Nearest rivals:
  version › crate › category, defeated because a version's Migration entries would scatter; a
  location by Component, defeated because the root is no published crate here.

## Harvest

| what | thread | home |
| --- | --- | --- |
| the division of the changelog's work between `model` and `check` | `thread@per-crate-changelogs@changelog-model-home` | a member of `design@knowledge-architect@model-crate`, whose line between phases 1 to 3 and the checks it applies |
| the opt-in, the crates read from Cargo's rule | `thread@per-crate-changelogs@changelog-model-home`, `thread@per-crate-changelogs@changelog-opt-in` | the members of the rewritten `design@model@registers-are-declared` and `design@check@nothing-of-a-project-is-compiled-in`; a head of `register@check@design` on the opt-in, by the tests of `skill@knowledge-architect-decision-recording`; the touched heads listed above; the reading of Cargo's `publish` and `version` in the grounding file of `cargo-reader` |
| the grammar and the classes compiled in, the categories declared | `thread@per-crate-changelogs@changelog-grammar-compiled-in`, `thread@per-crate-changelogs@changelog-entry-categories` | the member of `design@check@nothing-of-a-project-is-compiled-in`; a head on the grammar in `register@model@design` |
| every finding in the last phase | `thread@per-crate-changelogs@changelog-findings-in-check` | the member of `design@knowledge-architect@model-crate` above; its rival, phase findings, in `register@knowledge-architect@rejected` if a recording test admits it, and otherwise in that head's body |
| the bump check, the omitted versions, the `include` check | `thread@per-crate-changelogs@changelog-bump-check`, `thread@per-crate-changelogs@absent-version-section`, `thread@per-crate-changelogs@changelog-include-check` | heads of `register@check@design`; the reading of semver in a grounding file of `check` |
| the preamble, a finding with no fix | `thread@per-crate-changelogs@category-order-in-preamble` | a head of `register@check@design` |
| the assembled file | `thread@per-crate-changelogs@assembled-changelog-location`, `thread@per-crate-changelogs@assembled-changelog-layout`, `thread@per-crate-changelogs@assembled-file-not-overwritten` | heads of `register@model@design`, the member of the rewritten `design@model@git-supplies-the-walk`, the members of `design@model@one-list-of-generated-files` and `design@check@fix-before-the-checks`; the rival layouts in `register@model@rejected` if a recording test holds |
