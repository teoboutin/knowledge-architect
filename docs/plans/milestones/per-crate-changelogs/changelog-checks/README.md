# Slice 3: the changelog's model, the `[changelog]` table and the checks, opt-in

## Builds

- In `model`:
  - the resolve of the manifest table `[changelog]`, in a module of its own beside
    `path@model@src/manifest.rs`, which gains only the call that dispatches the table;
  - the walk's gathering of the `Cargo.toml` of each Component, where `[changelog]` is declared, on
    the working tree and in the snapshots of `--staged` and `commits`, read through `cargo`, which
    `model` now depends on;
  - the parse of a crate's changelog, which records its preamble, its version sections, its entries
    and its unread lines, and raises no finding;
  - the rendering of the assembled layout from the parsed changelogs, a pure function the `query`
    command of slice 4 also calls;
  - the assembled file among the generated paths, `generated_paths` of `path@model@src/index.rs`,
    written by `cargo klarch index` and `cargo klarch check --fix`.
- In `check`, checks of the last phase: every finding below.
- No change to this repository's manifest: it declares no `[changelog]` until slice 5, so nothing of
  this slice runs over this repository. Every test is over mock projects.
- The readings of Cargo's semver compatibility the bump check makes, in a grounding file of `check`.
- The entries of this slice in the shared changelog's `Next release` section, under its current
  grammar.

## Claims

Each test is shown to fail against a wrong implementation by a mutation of the code it guards, run
with `cargo x mutate`.

1. **Opt-in**: a mock project with published crates, a `CHANGELOG.md` of another shape and no
   `[changelog]` table yields no changelog finding, and no file is assembled.
2. **The changelog crates**: with `components` listed, exactly those Components are read; with
   none listed, exactly those whose `Cargo.toml` has a `[package]` table not setting
   `publish = false`; a changelog crate with no `CHANGELOG.md` is a finding.
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
8. **The preamble**: a preamble that differs from the one the manifest determines is a finding,
   and `--fix` rewrites it; it lists the categories in their declared order and the `unclassed`
   versions, names the assembled file in words, and gives `assembled-url` where set.
9. **The assembled file**: with two changelog crates or more, a stale or missing assembled file is
   a finding and `--fix` writes it in the layout below; with one, nothing is assembled.
10. **The writer's refusals**: `--fix` refuses to write the assembled file over a file at its path
    that lacks the generated header, and refuses to write it while any changelog holds an unread
    line, naming the line; neither refusal writes any byte.
11. **The rendering round-trips**: the assembled text of a mock project, parsed back per crate,
    yields every entry of every crate with its category, class and text unchanged.

## Fixtures

A mock project under `path@testing@tests/projects/` with a `[changelog]` table and three
Components, each holding a `Cargo.toml` and a `CHANGELOG.md`: one with `publish = false`, two
published, one of which restricts nothing and one of which carries the restricted category. Mock
projects hold no `Cargo.toml` today: the addition this slice makes is that file in a mock
Component's directory, read only where `[changelog]` is declared. The defects of claims 3, 4, 5, 7,
8 and 10 are planted in copies, each in a test of its own, as `path@testing@CLAUDE.md` says.

## Audit subjects

- `path@model@src/manifest.rs`, `path@model@src/walk.rs`, `path@model@src/index.rs`,
  `path@model@src/phases.rs`, `path@check@src/commands/history.rs`, `path@check@src/check/generated.rs`.
- `design@knowledge-architect@model-crate`, `design@check@phases-gate-the-report`,
  `design@check@nothing-of-a-project-is-compiled-in`, `design@check@the-regime-has-no-opt-out`,
  `design@model@components-carry-the-same-documents`, `design@model@registers-are-declared`,
  `design@model@git-supplies-the-walk`, `design@model@one-list-of-generated-files`,
  `design@check@fix-before-the-checks`, `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`,
  `design@create@init-asks-no-confirmation`, `design@check@staged-tree-source`,
  `design@check@model-then-checks`, `design@knowledge-architect@versioning-policy`.
- `issue@model@the-structural-findings-might-be-checked-at-scan-time`,
  `issue@model@the-walk-may-exclude-a-file-the-model-requires`,
  `issue@check@index-staged-leaves-the-working-tree-s-generated-file-stale`,
  `issue@knowledge-architect@a-conflict-in-a-generated-file-is-resolved-by-hand`.
- `issue@model@manifest-rs-is-past-one-read` and `issue@check@history-rs-is-past-one-read`, weighed
  below.

## Fails alone on

- A changelog finding, or its absence, over a mock project; or the bytes of a mock project's
  assembled file.

## Premises that expire

- This repository declares no `[changelog]`, so its root CHANGELOG.md, a row of `skip-files`, is
  not the assembled file until slice 5. The guard: claim 10's refusal, which keeps the first
  `--fix` of slice 5 from writing over the shared changelog unless slice 5 first replaces it.

## What the work is

What this slice alone changes, beside what the milestone document's "What the work is" says:

- **`issue@model@manifest-rs-is-past-one-read`, weighed.** The slice adds one more dispatch to
  `path@model@src/manifest.rs`, the table's resolve going in a module of its own. Where slice 1
  closed the issue on its argued decision, this slice weighs the file again only if its scout
  reports a new signal; otherwise its design audit weighs it as slice 1's spec says.
- **`issue@check@history-rs-is-past-one-read`, weighed.** The snapshots must gather each
  Component's `Cargo.toml`. The code-split scout found that `read_tree` of
  `path@check@src/commands/history.rs` already reads every file the walk does not skip, so the
  module may not change. The default: no split is planned; if the slice changes the module's code,
  its design audit closes the issue with an argued decision or plans the split before the work.

## What is already decided

The entries this slice's audit takes as seeds are the audit subjects' design entries above.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@knowledge-architect@model-crate`: "it depends on `git` alone" becomes `git` and `cargo`.
- `design@check@nothing-of-a-project-is-compiled-in`: gains a sixth member, the changelog's
  grammar and classes.
- `design@check@the-regime-has-no-opt-out`: gains `[changelog]` as a declaration that adds
  obligations and removes none.
- `design@model@git-supplies-the-walk`: "No generated path is declared" gains the assembled file's
  declared path, outside the walk by construction.
- `design@model@one-list-of-generated-files`: its list gains the assembled file.
- `design@check@fix-before-the-checks`: gains the refusal while a line is unread.
- `design@check@model-then-checks`: its example "such as a changelog … the extension hands it the
  text" becomes false, since the changelog is part of the model.
- `design@model@components-carry-the-same-documents`: judged at the audit, whether its "Nothing is
  discovered by filename" needs the reading of `publish` stated.

## Decided design

### The changelog crates, the opt-in and the table

From `thread@per-crate-changelogs@changelog-model-home` and
`thread@per-crate-changelogs@changelog-opt-in`.

- The changelog checks run only in a project whose manifest declares `[changelog]`. A project that
  declares none is asked nothing, so the work is no new ask of an adopting project.
- The changelog crates are those `components` lists, in the manifest's order of the Components; or,
  where it lists none, every Component whose `Cargo.toml` has a `[package]` table not setting
  `publish = false`. Reading `publish` is no discovery by filename, and it reports a new published
  crate with no changelog, which a hand list would miss.
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
  finding, per `thread@per-crate-changelogs@absent-version-section`.
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
  assembled, names it in words, "the changelog of the whole project is CHANGELOG.md at the root of
  its repository", with `assembled-url` as a link where set; lists the categories in their order;
  and lists the `unclassed` versions, where any.
- A consumer's `klarch changelog` holds no manifest of the publisher, since no crate ships
  knowledge-architect.toml, so it parses and renders by the preamble: each crate's file is
  self-contained.
- A preamble that differs is a finding, and `--fix` writes it. Where the lines above the first
  version heading are not a preamble of the checker's form, `--fix` refuses and names them, so it
  loses nothing the project wrote, per `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`.
- No URL is built from `repository`, and `repository` is required nowhere (R5, R6).
- An illustration of a preamble:

  ```markdown
  # Changelog of knowledge-architect-check

  This file holds the entries of this crate only. The changelog of the whole project is
  CHANGELOG.md at the root of its repository.

  Categories, in order: Migration, New features, Workflow. Versions with no category and no class:
  0.1.0.
  ```

- Argument: `argument@per-crate-changelogs@arg-41`, `argument@per-crate-changelogs@arg-49`.

### The assembled file: where, in which layout, and written how

From `thread@per-crate-changelogs@assembled-changelog-location`,
`thread@per-crate-changelogs@assembled-changelog-layout` and
`thread@per-crate-changelogs@assembled-file-not-overwritten`.

- **Where**: at the path `assembled` names, `CHANGELOG.md` at the root where the key is absent.
  With one changelog crate, nothing is assembled, whatever the key.
- **Layout**: a generated header, as an `index.md` opens with one; then each version, `Next
  release` first, as `## <version>`; under it each category in its declared order, as `###
  <category>`; under it each changelog crate in the manifest's order of the Components, as `####
  <package name>`, the published name with its prefix; under it the crate's entries of that category,
  each `- <class>: <text>`, in the order of the crate's file. A version `unclassed` lists has no
  category level: `#### <package name>`, then `- <text>`. An empty level is omitted.
- **Written**: by `cargo klarch index` and `cargo klarch check --fix`, from the parsed changelogs;
  checked current by the check; outside the walk by construction, as an `index.md` is; listed in
  `generated_paths`. A missing assembled file is reported as not current, as an `index.md` is; the
  audit checks how `tree` treats a declared generated path that does not exist yet.
- **Refusals**: `--fix` refuses to write over a file at the path that lacks the generated header,
  per `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`, and refuses to
  write while any changelog holds an unread line, per `design@check@fix-before-the-checks`. Each
  refusal names its repair.
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
| where the model and the checks live, the opt-in, the crates read from `publish` | `thread@per-crate-changelogs@changelog-model-home`, `thread@per-crate-changelogs@changelog-opt-in` | heads of `register@model@design` and `register@check@design`, by the tests of `skill@knowledge-architect-decision-recording`; the touched heads listed above |
| the grammar and the classes compiled in, the categories declared | `thread@per-crate-changelogs@changelog-grammar-compiled-in`, `thread@per-crate-changelogs@changelog-entry-categories` | the member of `design@check@nothing-of-a-project-is-compiled-in`; a head on the grammar in `register@model@design` |
| every finding in the last phase | `thread@per-crate-changelogs@changelog-findings-in-check` | a head of `register@check@design`, and the rival, phase findings, in `register@check@rejected` if a recording test holds |
| the bump check, the omitted versions, the `include` check | `thread@per-crate-changelogs@changelog-bump-check`, `thread@per-crate-changelogs@absent-version-section`, `thread@per-crate-changelogs@changelog-include-check` | heads of `register@check@design`; the reading of semver in a grounding file of `check` |
| the preamble | `thread@per-crate-changelogs@category-order-in-preamble` | a head of `register@check@design` |
| the assembled file | `thread@per-crate-changelogs@assembled-changelog-location`, `thread@per-crate-changelogs@assembled-changelog-layout`, `thread@per-crate-changelogs@assembled-file-not-overwritten` | heads of `register@model@design`, the members of `design@model@git-supplies-the-walk`, `design@model@one-list-of-generated-files` and `design@check@fix-before-the-checks`; the rival layouts in `register@model@rejected` if a recording test holds |
