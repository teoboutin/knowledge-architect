# The model, the scan, phases 1 to 3 and the resolver become the `model` crate

The second slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `model`, crates/model, package `knowledge-architect-model`, depending on `git` and,
  until #check-renamed, on `agent-skills`, which the core then depends on by path and by exact
  version; its goals drafted and put to the owner as the milestone document says; everything "How
  the work is done" says a new published crate owes, its README stating the four conditions of a
  target project, the asks and the limit of the guarantee, per
  `goal@knowledge-architect@serves-one-kind-of-project`.
- The files the mapping table of the milestone document sends to `model`, and its share of the
  files it splits: the foundation, phases 1 to 3, the installed-file check still in phase 2 until
  #check-renamed moves it; `complete_working_tree`, the gate the writers and `worklist` call; the
  resolver, cut from the lints of `path@core@src/check/references.rs`, returning a
  `model::resolve::Outcome`, a `Target` or the `Cause` of its failure, which the lints turn into
  findings; the phase-1 contribution as data, the struct `model::extension::Contribution`, which
  `check` computes from each extension's `Extension::tables` and `Extension::resolve`, whose
  signatures do not change, so no extension changes. The mapping table below takes each failure arm
  of the resolver's functions to a `Cause`.
- `model`'s facade, one file, with `#![warn(unreachable_pub)]`, whose role modules are listed under
  "New names".
- The core keeps every public path it has: `cli::complete_working_tree`, `cli::Gathered`,
  `extension::Inputs`, `testing::foundation`, `MANIFEST_NAME`, `Manifest`, `Model` and `document`
  among them, each a re-export or a thin function over `model`.

## Claims

- **The foundation gives the same verdicts.** Test: the binary's suite,
  `path@core@tests/binary.rs`, passes with two assertions changed and no other, each by the list of
  crates a binary links: the summary's checker-source line, in
  `the_core_binary_names_the_checker_s_directories_and_counts_the_files_under_them`, and the
  `cargo clean` line of `a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused`; shown
  to discriminate by a recorded mutation of a phase-2 assertion moved to `model`.
- **A tree holding the model crate at another path than the binary's is refused.** Test: a new
  binary test, the sibling of `a_tree_holding_the_git_crate_at_another_path_than_the_binarys_is_refused`;
  shown to discriminate by a recorded mutation of the model crate's entry of `libraries` in
  `path@core@src/main.rs`.
- **The gate is the same.** `complete_working_tree` gathers in `model` what it gathers today but the
  committed generated files and the `register.toml` files, whose reads cannot fail, in the order git
  is asked today. Test: the binary's tests of the writers pass.
- **Every reference finding is unchanged in text and in count.** Test: the existing reference tests
  pass; a recorded mutation of the resolver's handling of one failing segment fails one.
- **`show` prints what it printed.** It calls `check::references::register` and
  `check::references::planned_target` today, for a target and for a refusal; the resolver keeps
  both. Test: the binary's tests of `show` pass; a recorded mutation of the resolver's refusal of a
  `planned` path fails one.
- **`model`'s public API is a facade of role modules**: premortem C1. Two tests, since
  `unreachable_pub` is silent on `pub mod scan;`, which publishes the module tree that
  `rejected@core@module-tree-as-public-api` refuses, and on an inherent method of a reachable type.
  A unit test reads `model`'s lib.rs and refuses a `pub mod <name>;` with no inline body and a glob
  `pub use`, shown to fail by a recorded `pub mod scan;`. `unreachable_pub` warns on any other `pub`
  item no role module names, and the clippy gate denies warnings, shown to fail by a recorded `pub`
  item outside the facade. The inherent methods are read in the review, per D4 of the milestone
  document.
- **An extension's tables, declared paths and generated files reach phase 1 unchanged.** Test:
  `path@core@tests/extension_api.rs` passes with no change, and the tests of claimed tables do:
  `a_table_no_extension_claims_is_a_complaint` and `a_claimed_table_the_manifest_leaves_out_is_a_complaint`
  in `path@core@src/extension.rs`, and `the_core_binary_refuses_a_table_no_extension_of_it_claims`
  in `path@core@tests/binary.rs`; shown to discriminate by a recorded mutation that drops the claimed
  tables from the contribution, which the first two fail on: the binary registers no extension, so
  it claims nothing with or without the mutation.
- **Every git invocation of the library code of the core and of `model` goes through `git`.** Test:
  `git grep -n 'Command::new("git")' crates/core/src crates/model/src` lists none outside a
  `#[cfg(test)]` module, the one hit being `every_tracked_rust_file_yields_prose` in the parser's
  tests, per `issue@git@tools-run-git-past-the-wrapper`; shown to discriminate by a recorded mutation
  that adds such a call to a library module of `model`.

## Fixtures

The mock projects of `path@core@tests/projects/`, unchanged.

## Audit subjects

`path@core@src/check/mod.rs`, `path@core@src/check/references.rs`, `path@core@src/extension.rs`
(`Extension::tables`, `Extension::resolve`, `Resolution`, `extension::configure`),
`path@core@src/cli/gathered.rs`, `path@core@src/cli/history.rs`, `path@core@src/agents.rs`, and `show`
in `path@core@src/cli/mod.rs`; `design@core@model-then-checks`, `design@core@phases-gate-the-report`,
`design@core@an-extension-plugs-in-through-phased-hooks`,
`design@core@an-extension-claims-its-manifest-tables`, `design@model@a-wrong-declaration-is-a-finding`,
`design@core@inputs-are-gathered`, `design@model@one-entity-table`,
`design@model@one-list-of-generated-files`, `design@core@orphan-rejected-entry` and
`design@core@heading-register-two-shapes`, whose text names files this slice cuts; the heads and
issues the Harvest's rows name;
`rejected@core@module-tree-as-public-api`.

## Fails alone on

A finding of phases 1 to 3, or a reference finding, that differs from the tree's before the move.

## Premises that expire

- The mock tests of `path@core@src/mock_projects.rs` read internals this slice moves into `model`.
  Until #integrated-tests, each runs where its internals are: those over the model as a unit-test
  module of `model`, those over the checks as one of the core. Those in `model` reach the mock
  projects of the core's directory by a path relative to `model`'s, the one crate reaching another's
  fixtures in the milestone, for four slices. #integrated-tests ends it; the claim of that slice that
  no unit test loads a mock project guards it.
- The installed-file check stays in phase 2 until #check-renamed, so for this slice it stays in the
  foundation, in `model`, and `model` depends on `agent-skills` for the shipped text it compares.
  The claim of #check-renamed that `model` no longer depends on `agent-skills` guards it. The tests
  of the shipped text, in the test module of `path@core@src/agents.rs`, go with `shipped` to `model`,
  and go on to `check` at that slice. The entries about the check itself,
  `design@core@owned-namespace-check` and
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, stay in the core,
  since their subject returns there at that slice.

## Decided design

The tools over the record need the model and at most phases 1 to 3, never phase 4: `id`,
`worklist` and the writers run the foundation and no check, and `show` runs no phase but reads the
resolver (#arg-42, corrected at the plan's review); the foundation says whether the model can be
trusted, not whether the content conforms (#arg-68). The resolver is cut from the lints because the
site renderer needs the resolution with no finding attached (#arg-44). The boundary is crossed by 44
of the 99 commits that touched either side, not by every change (#arg-45), and the goals predict
more consumers (#arg-30, #arg-46). An extension's phase-1 contribution sits in `model` because phase
1 does (#arg-93); it is data rather than a trait because phase 1 is then a pure function that a test
drives with a value and no extension, which is the default shape of the code, and because `query`
and `render` run it with no extension, so `model` has no reason to name one.
`complete_working_tree` sits in `model` because `worklist` calls it, and `query` must reach it with
no check (the plan's review, premortem C2).

**The words of a `Cause` sit in `model`.** `show` prints a refusal's two texts, and #consumers-out
moves `show` to `query`, which #a1-no-check-below-the-tools keeps off the checks, so the texts cannot
sit in the lints. The lints write a finding from them at their site; the site renderer reads the
`Cause` and needs no finding.

**`model` gathers what the resolver reads.** `Inputs` stays the one public struct an extension
reads, in the core. `model::foundation::TreeInputs` holds what phases 1 to 3 and the resolver read:
`present`, `directories`, `outside`, `ignored`, `tracked_and_ignored`, `refused`, `links`, `installed`
and `shipped`; `Inputs::tree` lends it. `model`'s `Gathered` reads them, the check-ignore batch
included, since a consumer of the resolver with no check needs its answer, or a target the rules
cover reads as absent, against `design@model@ignored-targets-are-not-asserted`; the core's `Gathered`
adds the committed generated files and the `register.toml` files, which phase 4 alone reads.

**A head that a slice's cut divides stays in the core**, with the code in `model` citing it: a move is
no touch of a head, per `design@agent-skills@existing-heads-on-touch`, and splitting one is the work of
`issue@core@core-heads-bundle-decisions-with-many-citing-sites`. `design@core@every-path-names-its-anchor`,
whose grammar goes to the resolver and whose unanchored lint stays, is the case in point.

The nearest rival, the model kept inside the checker (#arg-16), is a losing alternative of the
milestone document.

## New names, in one place

Beside `model::resolve::Outcome` and `model::extension::Contribution`, which the milestone document
lists:

```text
crates/model/src/lib.rs        the facade: the root, and the role modules below, each with an inline body
crates/model/src/phases.rs     foundation, Phase, Stop, TreeInputs, Gathered, complete_working_tree
crates/model/src/tree.rs       today's check::tree, phase 2
crates/model/src/shipped.rs    today's check::agents, with the part of agents.rs that it and the gathering read
crates/model/src/resolver.rs   Outcome, Target, Cause, and the resolver's functions
crates/model/src/contribution.rs  Contribution, contribute
the root                       Manifest, MANIFEST_NAME, Model, Document, Finding, escaped, CRATE_DIR, PACKAGE, component_dir
model::document                a document's parse, as the core's role module re-exports it
model::declared                what a manifest declares: Scope, Shape, Pin, Walk, the register names and the document names
model::table                   the entity table and the anchors: Anchors, Anchor, Entities, Kind, Candidate, candidate, Site, and the records over them
model::listing                 the listing: Survey, survey, Outside, the walk's filters
model::generated               the generated files: generated_paths, file_register_indexes, label, rel_from
model::installed               the shipped text as phase 2 reads it: render, lf, shipped, imports_primer, PRIMER, IMPORT_LINE
model::foundation, model::resolve, model::extension   the role modules over phases.rs, resolver.rs and contribution.rs
Inputs::tree                   crates/core/src/check/mod.rs
```

`model` holds a private constant of the checker's version for the installed-file check's words, equal
to the core's `CHECKER_VERSION` by `design@knowledge-architect@version-lockstep`; the public constant
stays the core's.

## Mapping tables

**Each failure arm of the resolver's functions, and its cause of `Outcome`.** The six functions
"Names" lists, with `assert_target`, which `path`'s named arm and the lint `link` call, and `planned`,
which calls `planned_target`, so one kind has one rule. Read at this slice's audit from `path@core@src/check/references.rs`, where each arm pushes one finding today. Every cause
names what the arm's two texts need beyond the span, and `Cause::texts` writes the same two texts.

| function | the arm | `Cause` |
| --- | --- | --- |
| `table` | `Resolution::UnknownAnchor` | `TableUnknownAnchor { anchor, listed }` |
| `table` | `Resolution::AnchorLacksRegister` | `AnchorLacksKind { anchor, kind, carriers }` |
| `table` | `Resolution::OutsidePlan` | `OutsidePlan { plan, form, id }` |
| `table` | `Resolution::Undefined`, an item of a section | `UndefinedItem { anchor, kind, id, section }` |
| `table` | `Resolution::Undefined`, any other | `Undefined { anchor, kind, id, home }` |
| `harness` | the owner is named and defined by nothing | `NoOwner { kind, owner }` |
| `harness` | the owner is defined, its section is not | `NoOwnerSection { kind, owner, id }` |
| `harness` | a named kind whose name is defined by nothing | `NoNamed { kind, id }` |
| `harness` | the primer or the root instructions without the section | `NoSection { document, id }` |
| `register` | the id is no register | `NoRegister { id, homed }` |
| `register` | the register is an item register | `ItemRegister { id }` |
| `register` | only a Directory register's entries carry it | `CarriedByEntries { id, by }` |
| `register` | the escape anchor | `RegisterAtEscape { carriers, every }` |
| `register` | `*`, a register attached to some Components | `RegisterEveryAttached { id, components, carriers }` |
| `register` | `*`, a register not of every Component | `RegisterEveryNotAll { id, carriers }` |
| `register` | an unknown anchor | `RegisterUnknownAnchor { name, carriers, every }` |
| `register` | a plan anchor | `RegisterAtPlan { name, form }` |
| `register` | an entry anchor | `RegisterAtEntry { name, form }` |
| `register` | an anchor that does not carry it | `RegisterNotCarried { name, id, carriers, registers }` |
| `path`, the escape | the target is an entry | `EntryByPath { form }` |
| `path`, the escape | the target is a register's home | `HomeByPath { anchor, register, suffix }` |
| `path`, the escape | the target resolves in this tree | `ResolvesHere { form }` |
| `path`, `*` | a refused shape | `EveryRefused { why }` |
| `path`, `*` | the homes of a register attached to some Components | `AttachedHomesByPath { register, components }` |
| `path`, `*` | a register's home | `HomeByPath { anchor: "*", register, suffix }` |
| `path`, `*` | a required document with a trailing slash | `RequiredKind` |
| `path`, `*` | no Component carries it | `InNoComponent` |
| `path`, `*` | it claims a file and names a directory | `ClaimsFile` |
| `path`, `*` | it claims a directory and names a file | `ClaimsDirectory` |
| `anchored_target`, for a `path` | an unknown anchor | `PathUnknownAnchor { name, names }` |
| `anchored_target`, for a `planned` | an unknown anchor | `PlannedUnknownAnchor { name, names }` |
| `anchored_target` | a milestone anchor | `AtMilestone { name }` |
| `anchored_target` | a spec anchor | `AtSpec { name }` |
| `anchored_target` | a refused shape | `Refused { why }` |
| `anchored_target` | a plan document by its path | `PlanDocumentByPath { form }` |
| `anchored_target` | an entry | `EntryByPath { form }` |
| `anchored_target`, for a `path` | a register's home | `HomeByPath { anchor, register, suffix }` |
| `anchored_target` | inside a deeper anchor | `InsideDeeperAnchor { owner }` |
| `planned_target` | the two reserved words | `PlannedAtReserved { anchor }` |
| `planned`, which `judge_part` calls | outside the plans directory | `PlannedOutsidePlans` |
| `planned` | it points into a file that exists | `PointsIntoExisting { file, form }` |
| `planned` | the target exists | `NowExists { target, form }` |
| `assert_target`, which `path`'s named arm calls | the target does not exist | `Missing { target }` |
| `assert_target` | it claims a directory and names a file | `ClaimsDirectory` |
| `assert_target` | it claims a file and names a directory | `ClaimsFile` |

**What each success is.** `Ok(Target)`: `Entity` for `table` and `harness`, `Home` for
`register`, `Path(target)` for a named anchor, a target the ignore rules cover included,
`Carried(paths)` for `*`, `Required` for a required document under `*`, `Elsewhere` for the escape,
and `Planned(target)` for a `planned` path.

**Each helper of `path@core@src/check/references.rs`, and its side.** To `model::resolve`: `refused`,
`resolved_here`, `without_suffix`, `converted`, `in_plans_directory`, `entry_at`, `plan_document`,
`is_navigation`, `has_scheme`, `ignore_queries`, and the dispatch that takes a candidate to the
function of its kind, so no consumer repeats it. Staying as lints: `judge`, `judge_part`,
`check_under`, `orphans`, `link`, which reports a link's own refusal and calls `assert_target`, the
goals-home citation, the three id lints, `names_this_tree` and `names_this_project`.

**Each unit test that reads what moves.** A test that stays in the core reads public items alone,
since `cfg(test)` is set only for the crate under test:

| test | where it goes |
| --- | --- |
| the tests of `check::tree`, `check::agents`, `foundation` | with the code, to `model` |
| the test module of `path@core@src/agents.rs` over `shipped` and the entity constants | with `shipped`, to `model`; the test of the owned namespace stays |
| `an_anchor_carrying_no_such_register_is_reported_with_the_anchors_that_do`, which needs `Anchors::from_list` | to `model`, as a test of `Cause::AnchorLacksKind` |
| the other tests of `path@core@src/check/references.rs` | stay; `Anchors::declared` is `Anchors::of(manifest, &[])` |
| `check::testing` | a copy in each crate: three lines over a flat listing, shared by no public item |

## Losing alternatives

- **The phase-1 contribution as a second trait**, #arg-93's first wording, lost to the struct: under
  the trait, phase 1 calls an extension's code, so a test of it needs an extension, and `model` names
  a trait that `query` and `render` have no implementation of. It is an interface others consume, so
  the harvest judges it by the tests for a losing alternative.
- **The words of a cause in the lints**, lost to `Cause::texts`: `show` could not then print a
  refusal from `query` with no check.
- **The check-ignore batch left in the core's gathering**, lost to `model`'s: a consumer of the
  resolver with no check would read every ignored target as absent.

## Implementation sequence

**The first change sets nothing stricter for an earlier tree.** Adding crates/model to the compiled
lists exempts or compares a directory no earlier tree holds; `snapshot_checker_dirs` in
`path@elsewhere@crates/core/src/model.rs` matches nothing there. Taken again by `cargo klarch commits` over the branch.
Step 2 lands as one commit, with the crate's entry in those lists, since its intermediate trees do
not compile and a later entry would read `model`'s literals as prose.

1. Cut the resolver from the lints inside the core, with no crate boundary: the mechanism alone.
2. Create `model` and move the files and the foundation, behind the facade.
3. Make the phase-1 contribution data, with `Extension`'s signatures unchanged.
4. Move the heads, the `rejected` entries, the issues and the tripwires whose subject is the model, and
   the part of the core's CLAUDE.md and README that describes what moved. Rewrite "No entry yet" in
   each home that takes an entry, per
   `issue@core@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`. Search for the names
   no check reads, `check::tree`, `check::agents`, `check::foundation`, "the core's test" and "a test of
   the core", in every Markdown file and comment, and rewrite each that the move turns false.
5. The harvest.

## Harvest

| what | where |
| --- | --- |
| #model-crate | `model`'s design home: the moved heads; `design@core@model-then-checks` and `design@core@an-extension-plugs-in-through-phased-hooks` rewritten for the two crates, the part saying that `check` depends on `model` in the root's design home, per the milestone document's harvest; `design@core@phases-gate-the-report` rewritten for the crate that runs phases 1 to 3; `design@core@inputs-are-gathered` rewritten for the two gatherings, one in each crate |
| premortem C1 | its claim above |
| the heads whose decision the code in `model` alone enforces | moved whole to `model`'s design home: `design@model@reserved-anchors`, `design@model@register-home-reference`, `design@model@path-to-a-home-is-reported`, `design@model@plans-is-a-reserved-anchor`, `design@model@planned-path-form`, `design@model@trailing-slash-claims-directory`, `design@model@ignored-targets-are-not-asserted`; every head of `path@model@docs/design/anchors.md`; `design@model@git-supplies-the-walk`, `design@model@links-are-no-documents`, `design@model@unportable-names-refused`, `design@model@tracked-and-ignored-is-a-finding`; `design@model@one-entity-table`, `design@model@an-entry-is-a-heading-at-the-register-level`, `design@model@two-segment-ids`, `design@model@statement-precedes-the-slug`, `design@model@a-slug-is-read-anywhere-in-its-heading`, `design@model@a-slug-elsewhere-defines-nothing`, `design@model@headings-open-with-hash-marks`, `design@model@an-entity-belongs-to-its-anchor`, `design@model@an-unresolved-reference-is-reported`; `design@model@a-failed-parse-is-loud`; `design@model@grammars-not-prefixes`, `design@model@rust-parser-keeps-every-comment`, `design@model@names-collected-where-declared`; `design@model@registers-are-declared`, `design@model@home-at-the-carrier-path`, `design@model@a-wrong-declaration-is-a-finding`, `design@model@frontmatter-subset`, `design@model@frontmatter-is-prose`, `design@model@declarable-directory-register`, `design@model@entry-anchor-carries`, `design@model@entry-anchor-path-kind`; every head of `path@core@docs/design/plan-documents.md` but `design@core@plan-document-sections-by-kind`; `design@model@harness-kinds`, `design@model@root-instructions-alone-modelled`, `design@model@harness-entity-names`, `design@model@unusable-harness-name-is-a-finding`, `design@model@installed-entities-from-the-tree`; `design@model@at-span-candidate-rule`, `design@model@a-span-is-one-line`, `design@model@a-reference-is-live-in-every-prose`, `design@model@harness-kinds-cited-without-anchor`, `design@model@section-homes-carry-slugs`; `design@model@a-file-register-index-is-rows`, `design@model@one-list-of-generated-files`; each with the `rejected` entries only it cites, and every issue and tripwire whose subject its code is |
| the heads a slice owes for a new crate, per the milestone's table | `design@core@checker-source-literals-are-data`, the last sentence of `design@core@single-crate` and `design@knowledge-architect@partial-publish-rerun` name `model`; `design@core@a-foreign-build-is-refused` and `design@knowledge-architect@mutation-command-host` hold as they stand; `design@core@api-facade`: what the core's facade names of `model`'s, with `model`'s own facade a head of `model` |
| sentences the move turns false | `design@model@plan-items-by-section`, `design@core@component-manifest-edit`, `design@agent-skills@checker-model-in-primer`, `design@agent-skills@shipped-text-cites-no-entry`, `issue@model@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`, `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`, `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to` and `tripwire@core@private-item-needed`, per D4; root's Component table, and its sentence on a figure in the core's documents |
| `design@knowledge-architect@checker-asks-git-through-the-crate` | the root's design home, since it binds the code of two Components, per `design@agent-skills@intermediate-component-occasion`; `design@knowledge-architect@git-crate` names `model` beside the checker |
| the resolver's interface and the contribution as data, which others consume, and the check-ignore batch gathered with them | new heads of `model`, judged by `primer@design-heads`; `model` holds "`check` depends on `model`" in the root's, per the milestone's harvest |
| the losing alternatives above and the model left inside the checker, #arg-16 | judged by `skill@knowledge-architect-decision-recording@losing-alternatives`, from the head each loses to |
| `model`'s own CLAUDE.md | the contract on the walk and the ignore batch, and the rule on where a phase-2 finding goes, from the core's |
| the crate's place in `path@agent-config@skills/klarch-release/SKILL.md` | its lists of crates, pins and order |
| `model`'s goals | drafted under `skill@knowledge-architect-goal-setting`, and put to the owner before the merge |
