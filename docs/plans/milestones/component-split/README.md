# This repository is split into Components of one purpose each: `git`, `model`, `check`, `query`, `transcript`, `create`, `cli` and `testing`, with `bundle`, `render` and `rewrite` placed for later work

## Status and audience

This milestone document is the plan of the split of the core into eight Components, each a crate
of its own, and of the placement of three more that later work creates. It holds what crosses the
slices; each slice's spec holds what only that slice builds. It leaves when its last slice has
landed and every default it lists is ruled.

It comes from one design discussion, held in one session, which began at the owner's message "New
rules relative to how components and code in projects following knowledge-architect should be split
have just been added." The discussion applied `skill@knowledge-architect-component-creation` to this
repository, compared the result with the owner's own plan, and revised the skill;
`spec@plans@component-rules-revision` holds that revision, which the roadmap orders before this
milestone, on the owner's word.

- Written for a session that did not witness the discussion.
- Where this document or a slice's spec and a design home disagree on a point "What is already
  decided" does not list as reversed or rewritten, the design home wins, and the disagreement is a
  defect of the plan document.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits: D1 below.

## How the work is done

Read `skill@knowledge-architect-planning@working-a-slice` whole before each slice, then follow it.
This repository adds:

- `skill@klarch-development`, with `skill@knowledge-architect-development`, before each slice's code;
- **each new published crate owes, in the slice that creates it**: an `include` whitelist and the
  license copies, per `design@knowledge-architect@package-include-whitelist`; a CHANGELOG.md copy,
  its `skip-files` row and its place in `cargo x changelog`, per
  `design@knowledge-architect@the-changelog-ships-in-every-crate`; a CRATES-IO.md, per
  `design@knowledge-architect@crates-io-page-file`; the checkout tie in its library root and build
  script, per `design@knowledge-architect@a-build-is-tied-to-its-checkout`, which
  `path@xtask@tests/checkout.rs` asserts; the version of the workspace, per
  `design@knowledge-architect@version-lockstep`; its directory and package name in the lists of
  `design@core@a-foreign-build-is-refused` and `design@core@checker-source-literals-are-data`; and
  the Component's documents and register homes, declared in the manifest with
  `cargo klarch new-component`, its goals stated with the owner under
  `skill@knowledge-architect-goal-setting`;
- each slice moves the design heads, `rejected` entries, issues and tripwires whose subject its new
  Component holds, per `skill@knowledge-architect-component-creation@creation-procedure` point 4,
  and re-points every reference `cargo klarch check` then reports.

## Names

- **the core**: the Component `core`, crates/core, package `knowledge-architect`, today's checker
  library and binary. The slice #check-renamed renames it `check`.
- **the foundation**: `check::foundation` in `path@core@src/check/mod.rs`, which runs phases 1 to 3
  of a run: the manifest's complaints, `check::tree` and `check::agents` (phase 2), and the entity
  table's definition findings (phase 3). `cli::complete_working_tree` in `path@core@src/cli/mod.rs`
  refuses over a model the foundation stops.
- **the installed-file check**: `check::agents` in `path@core@src/check/agents.rs`, which compares the
  installed agent files with the shipped text, byte for byte.
- **the resolver and the lints**: in `path@core@src/check/references.rs`, the functions that resolve
  a reference against the entity table (`table`, `harness`, `register`, `path`, `anchored_target`,
  `planned_target`) and report what fails, one function each today.
- **the mock tests**: the 36 `#[test]` functions of `path@core@src/mock_projects.rs`, a
  `#[cfg(test)]` module that runs the core over the mock projects of `path@core@tests/projects/`.
- **an integrated test**: a test that drives the project through its public API, as a consumer
  would, as `spec@plans@component-rules-revision` defines it.
- **P1, P2**: the two shapes of #command-parsing-home. P1: the `cli` Component parses every klarch
  command, the domain crates expose a typed API with no clap in it. P2: each domain crate derives
  clap's types for its own commands, and `cli` flattens them.
- **the phase-1 contribution**: what an extension gives phase 1 today through `Extension::resolve`
  of `path@core@src/extension.rs`: the tables it claims, its complaints, the paths it declares and
  the files it generates.

## What the work is

What exists today, read at the branch that adds this document:

- **One crate holds the checker.** crates/core holds the library and the binary `klarch`, per
  `design@core@single-crate`; its `src/` holds 473 `#[test]` functions, counted with
  `grep -rc '#\[test\]' crates/core/src`. Its modules import each other as the mapping table below
  lists; `graph.rs` is imported by `cli/worklist.rs` alone, and `transcript.rs` imports nothing of
  the crate.
- **The commands are one enum**, `cli::Command` in `path@core@src/cli/mod.rs`, which a binary that
  registers extensions flattens, per `design@core@the-core-cli-is-a-library-module`.
- **Every git invocation of the core is in `path@core@src/git.rs`**, per
  `design@core@git-is-asked-through-one-module`; `gates` and `mutate` run git themselves.
- **`show`, `id`, `worklist` and the writers run the foundation**, and none of them runs phase 4.
- **The installed-file check is phase 2**, which the issue `installed-file-findings-belong-in-phase-four` recorded as
  misplaced.
- **The core's design home** holds 156 heads, counted with `grep -c '^### ' crates/core/docs/design/*.md`.
- **References naming the anchor `core`**: 1,475 backticked spans in 153 files, counted with
  ``git grep -o '`[a-z]*@core@[^`]*`' | wc -l`` and ``git grep -l '`[a-z]*@core@' | wc -l``.

Outside this work, each with what owns it:

- `bundle`, `render` and `rewrite`, placed here and created by their own work:
  `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`,
  `issue@core@the-documents-do-not-render-as-a-site` and
  `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`, each with the leads
  of this discussion, in the roadmap's order;
- `gates`, `mutate` and `xtask`, unchanged: moving `gates` and `mutate` onto the `git` crate is
  possible and not required (#arg-19);
- the revision of the component-creation skill and the code-shape fragment:
  `spec@plans@component-rules-revision`;
- the first publication of each new crate, which the release workflow cannot do, per
  `design@knowledge-architect@publish-credential`: the release after the milestone, on the owner's
  word.

## What is already decided

**Decisions the split rests on, not argued again:**

- `goal@knowledge-architect@projects-are-split-into-components`,
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
  `goal@core@documents-render-as-a-linked-site`, `goal@core@projects-add-their-own-checks`,
  `goal@core@relocation-is-one-manifest-edit`, `goal@core@records-reach-their-reader`,
  `goal@core@document-rules-are-checked`, `goal@core@documents-stay-current`,
  `goal@core@checks-a-commit-from-its-hook`: the criteria below;
- `design@knowledge-architect@version-lockstep`: every crate keeps one version;
- `design@knowledge-architect@no-component-directory-named-after-the-project`, which already names
  "crates/core and crates/cli" for a future split, and its rival
  `rejected@knowledge-architect@crate-directory-named-after-project`;
- `design@core@model-then-checks`, `design@core@phases-gate-the-report` (its rule that a finding is
  classed by the phase that produces it), `design@core@one-entity-table`,
  `design@core@one-list-of-generated-files`, `design@core@anchors-listing-command`,
  `design@core@id-lookup-command`, `design@core@worklist-writes-and-verifies-the-reports`: each holds
  across the new boundaries;
- the git heads that move whole to `git`: `design@core@per-user-ignore-file-pinned`,
  `design@core@repository-variables-dropped`, `design@core@index-file-variable-kept`,
  `design@core@a-path-is-bytes`, `design@core@git-floor-for-commit-trees`,
  `design@core@failed-git-is-exit-two`;
- `design@gates@the-library-owns-the-flags` and `rejected@gates@gates-command-of-the-checker`: `cli`
  flattens no command of `gates`, and `gates` keeps its flags;
- `design@core@exit-code-ladder`, `design@core@binary-name`, `design@core@one-question-one-command`,
  `design@core@ne-minimal`;
- `rejected@core@module-tree-as-public-api`, `rejected@core@doc-hidden-test-items`,
  `rejected@core@testing-feature-for-test-items`, `rejected@core@inputs-struct-literal`,
  `rejected@core@unit-tests-moved-under-tests`: none comes back; each new crate has a facade.

**Decisions the work reverses or rewrites**, each with the slice that does it. Every text that
`cargo klarch show` lists as citing it is re-pointed or rewritten by that slice, and the slice
#check-renamed re-points what is left naming `core`:

| head | done by | citing files today |
| --- | --- | --- |
| `design@core@single-crate` | #cli-on-top, reversed | `path@core@docs/design/transcripts.md`, `spec@plans@component-rules-revision` |
| `design@knowledge-architect@two-crates` | #check-renamed | none |
| `design@core@the-core-cli-is-a-library-module` | #cli-on-top | `path@agent-skills@docs/design/project-configuration.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/run-model.md`, `path@core@docs/design/transcripts.md`, `path@core@README.md`, `path@core@src/cli/create.rs`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `path@core@tests/binary.rs` |
| `design@core@arguments-parse-through-clap` | #cli-on-top | `path@core@docs/design/creating.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/manifest-table-exposes-the-toml-crate.md`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `spec@plans@component-rules-revision`, `register@xtask@design` |
| `design@core@transcript-command-in-core` | #consumers-out | none |
| `design@core@api-facade` | #integrated-tests; each slice writes its crate's facade | `path@core@CLAUDE.md`, `path@core@docs/design/transcripts.md`, `path@core@docs/open-issues/an-extension-cannot-see-which-register-a-document-belongs-to.md`, `path@core@docs/open-issues/manifest-table-exposes-the-toml-crate.md`, `register@core@tripwire`, `path@core@src/lib.rs`, `spec@plans@component-rules-revision` |
| `design@core@phases-gate-the-report` | #model-out (the foundation), #check-renamed (the phase table) | `path@agent-skills@docs/open-issues/audit-axes-beyond-the-design-record.md`, `path@agent-skills@docs/open-issues/the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit.md`, `path@core@CLAUDE.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/entities-and-headings.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/registers.md`, `path@core@docs/design/run-model.md`, `path@core@docs/design/worklist.md`, `register@core@tripwire`, `path@core@README.md`, `path@core@src/cli/create.rs`, `path@core@src/cli/mod.rs`, `path@core@src/cli/worklist.rs`, `spec@plans@path-quickfixes` |
| `design@core@owned-namespace-check` | #check-renamed, split as its issue says | `path@knowledge-architect@CLAUDE.md`, `path@agent-skills@docs/design/project-configuration.md`, `register@agent-skills@design`, `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/design/shipped-set.md`, `path@agent-skills@docs/open-issues/patching-an-installed-skill.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/a-project-workflow-can-declare-an-installed-workflow-s-name.md`, `path@core@docs/open-issues/core-heads-bundle-decisions-with-many-citing-sites.md`, `register@core@rejected`, `path@core@README.md`, `path@core@src/agents.rs`, `path@core@src/check/agents.rs`, `path@core@src/check/mod.rs`, `path@core@src/cli/history.rs`, `path@core@src/cli/mod.rs`, `path@core@src/manifest.rs`, `path@core@src/mock_projects.rs`, `path@core@src/survey.rs`, `path@core@src/walk.rs`, `path@core@tests/binary.rs`, `register@knowledge-architect@design`, `path@knowledge-architect@docs/open-issues/the-skill-bundling-passes-are-tied-to-this-project-s-skills.md`, `spec@plans@path-quickfixes` |
| `design@core@fix-before-the-checks` | #check-renamed | `path@core@docs/design/run-model.md`, `path@core@README.md`, `path@core@src/cli/mod.rs`, `path@core@tests/binary.rs`, `spec@plans@path-quickfixes` |
| `design@core@model-then-checks` | #model-out | `path@core@docs/design/extensions-and-api.md`, `path@core@src/check/references.rs`, `path@core@src/cli/gathered.rs`, `path@core@src/extension.rs`, `spec@plans@component-rules-revision`, `spec@plans@path-quickfixes` |
| `design@core@git-is-asked-through-one-module` | #git-out | `path@core@CLAUDE.md`, `path@core@docs/design/creating.md` |
| `design@core@inputs-are-gathered` | #model-out | `register@core@rejected` |
| `design@core@checker-source-literals-are-data` | each slice that adds a crate; #cli-on-top for the list the binary hands over | `path@agent-config@skills/klarch-development/SKILL.md`, `path@core@CLAUDE.md`, `path@core@docs/design/prose.md`, `path@core@docs/design/run-model.md`, `register@core@rejected`, `register@core@tripwire`, `path@core@README.md`, `path@core@src/check/references.rs`, `path@core@src/check/registers.rs`, `path@core@src/cli/history.rs`, `path@core@src/cli/mod.rs`, `path@core@src/entity.rs`, `path@core@src/graph.rs`, `path@core@src/index.rs`, `path@core@src/lib.rs`, `path@core@src/main.rs`, `path@core@src/model.rs`, `path@core@src/records.rs`, `path@core@src/scan.rs`, `path@core@src/source/mod.rs`, `path@core@src/source/rs.rs`, `path@core@tests/binary.rs` |
| `design@core@a-foreign-build-is-refused` | each slice that adds a crate; #cli-on-top | `path@agent-skills@src/lib.rs`, `path@core@CLAUDE.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/prose.md`, `path@core@docs/design/run-model.md`, `path@core@src/build_origin.rs`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `register@knowledge-architect@design` |
| `design@core@installed-binary-version-check` | #cli-on-top | `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@docs/open-issues/co-development-through-a-cargo-patch.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/commit-messages.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/transcripts.md`, `path@core@src/cli/mod.rs`, `path@core@src/lib.rs`, `path@core@src/main.rs`, `path@core@src/manifest.rs`, `path@core@tests/binary.rs`, `path@gates@docs/open-issues/the-gate-list-head-bundles-the-library-s-independence.md`, `register@knowledge-architect@design` |
| `design@knowledge-architect@partial-publish-rerun` | the milestone's harvest, for the publish order | `path@agent-config@skills/klarch-release/SKILL.md`, `register@knowledge-architect@design` |
| `design@knowledge-architect@mutation-command-host` | the milestone's harvest, for the publish order | `path@agent-skills@docs/design/project-configuration.md`, `register@mutate@design`, `register@xtask@design` |
| `design@core@nothing-of-a-project-is-compiled-in` | #cli-on-top | `path@agent-skills@docs/open-issues/pointers-written-bare-to-clear-a-finding-reach-no-check.md`, `path@core@CLAUDE.md`, `path@core@docs/design/anchors.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/plan-documents.md`, `path@core@docs/design/prose.md`, `path@core@README.md`, `path@core@src/cli/create.rs` |
| `design@core@an-extension-plugs-in-through-phased-hooks` | #model-out | `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/plan-documents.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/citation-regime-over-any-pinned-text.md`, `path@core@README.md`, `path@core@src/check/mod.rs`, `path@core@src/check/tree.rs`, `path@core@src/cli/mod.rs`, `path@core@src/extension.rs`, `spec@plans@path-quickfixes` |

`design@knowledge-architect@two-crates` is rewritten at the slice that the bundling work owns, not
here; this milestone changes its sentence "The checker depends on it by path and by exact version"
only in which crate depends on `agent-skills`, at #check-renamed. `design@core@transcript-command-in-core`
is rewritten at #consumers-out: its argument from `design@core@single-crate` falls with that head.

**The discussion's record search** graded 193 entries `bears` and 110 `may-bear`, for this milestone
and `spec@plans@component-rules-revision` together. Those that bear on the split, with their outcome:

- the heads above, rested on or rewritten;
- the publication heads, `design@knowledge-architect@publish-credential`,
  `design@knowledge-architect@owner-word-mechanism`, `design@knowledge-architect@versioning-policy`,
  `design@knowledge-architect@changelog-entries`, `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
  `design@knowledge-architect@stays-at-zero-x`, `design@knowledge-architect@capability-cost-before-migration`,
  `design@knowledge-architect@no-future-breaking-cost-argument`: each slice writes its changelog
  entries; the library break is allowed under 0.x; each new crate's first publish is D1's subject;
- `design@agent-skills@xtask-pins-checker`, `design@agent-skills@pinned-binary-runs-inside-the-project`,
  `design@core@declared-command`: `cli` keeps the package name `knowledge-architect`, so the setup
  snippets' dependency line is unchanged;
- `design@core@at-span-candidate-rule`: an anchor `git` makes a backticked `git@host:...` span a
  finding; the one such span, in `path@core@src/check/references.rs`, is a string literal of the
  checker's own source, which is data; the name `git` stays;
- `design@core@kind-names-refused-as-anchors`, `design@core@anchor-names-unique`,
  `design@core@components-carry-the-same-documents`, `design@agent-skills@goals-required`: every new
  name is free, and each Component carries its documents and a goal;
- `design@core@an-entity-belongs-to-its-anchor`, `design@core@every-path-names-its-anchor`,
  `design@agent-skills@a-reference-claims-a-revisit`: every moved entry's references are re-pointed,
  and heads that depend across the new Components cite each other;
- `issue@core@core-heads-bundle-decisions-with-many-citing-sites`: a move of a head is no touch of it,
  per `design@agent-skills@existing-heads-on-touch`; #check-renamed rewrites
  `design@core@owned-namespace-check` and splits it in two as the issue says, and the issue's other
  heads are split by the slice that rewrites one, if any does;
- the issue `installed-file-findings-belong-in-phase-four`: closed by the commit that adds this document, since #check-renamed schedules its work, which that slice's spec carries;
- `issue@core@manifest-table-exposes-the-toml-crate`, `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`,
  `tripwire@core@private-item-needed`: the facades of `model` and `check` meet them; nothing is
  decided here on them, and the tripwire's response routes a need an integrated test shows;
- the issues whose code moves, re-anchored by the slice that moves their files:
  `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`,
  `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`,
  `issue@core@finding-texts-are-not-audited-for-a-needless-cause`,
  `issue@core@a-span-whose-head-shortens-a-kind-passes-the-check-silent`,
  `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head`,
  `issue@core@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`,
  `issue@core@index-staged-clears-an-entrys-flags`,
  `issue@core@the-missing-directory-refusal-does-not-hold-the-no-loss-claim`,
  `issue@core@show-lists-no-subdocument-of-a-directory-home`,
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
  `issue@core@a-forked-subagent-s-first-line-is-an-unrecognised-type`,
  `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`,
  `issue@core@check-under-a-sparse-checkout-reports-every-file-outside-the-cone`,
  `issue@core@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`,
  `issue@core@the-tests-read-the-machine-s-git-configuration`,
  `issue@core@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
  `issue@core@a-component-states-at-least-one-goal`;
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: P1's typed API offers data
  by a library function, which its open policy names; the policy stays open;
- `issue@knowledge-architect@this-repository-warns-on-none-of-the-documentation-lints`,
  `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`,
  `issue@agent-skills@the-upstream-set-omits-the-published-libraries`,
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: each new crate adds one more site
  these issues count; none is closed or decided here;
- `issue@agent-skills@the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface`:
  the successor of `design@core@single-crate` is a layout decision, recorded in the root design home
  as the component-creation skill says, so the case it asks about does not arise;
- `design@xtask@one-module-per-subcommand`, `design@xtask@view-included-not-exported`,
  `rejected@xtask@source-view-exported-from-the-library`, `rejected@xtask@source-view-in-the-checker`:
  `MANIFEST_NAME` moves to `model`; the source view is the bundling work's, unchanged here.

Every other entry the search graded bears on `spec@plans@component-rules-revision`, which lists it,
or on nothing in this milestone. The `may-bear` entries not read in full, each with the searcher's
reason: the publication and versioning heads `design@agent-skills@exact-pin` and
`design@knowledge-architect@publish-after-merge` (the split changes no procedure); the audit-axis
issues (no axis is touched); the reference-lint heads on spans such as `git#<id>` (no instance
shown); `design@core@transcript-directory-resolution` (unchanged; it confirms `transcript` depends on
`git` and takes the project's root from its caller).

## Criteria

### Each Component that holds Rust code serves one purpose, with an interface that changes for a reason of its own `##one-purpose-per-component`

Binding, from `goal@knowledge-architect@projects-are-split-into-components`. Met by the eight
Component threads and #no-intermediate-component.

### The mechanical tasks the workflow asks over the scanned record are commands of a published crate that needs no check `##tools-need-no-check`

Binding, from `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. Met by #model-crate and
#query-crate; judged by #a1-no-check-below-the-tools.

### The documents can render as one linked site from the model alone `##render-from-the-model`

Binding, from `goal@core@documents-render-as-a-linked-site`. Met by #model-crate, which separates the
resolver from the lints, and #render-crate.

### An extension needs no change to the checker and uses only its public, documented API `##extension-api-stays-public`

Binding, from `goal@core@projects-add-their-own-checks`. Met by #cli-crate, which keeps the package
name and re-exports the extension API, #model-crate's phase-1 contribution, and #testing-crate.

### No rejected design of the facade comes back `##no-rejected-facade`

Binding as a presumption, from `rejected@core@module-tree-as-public-api`,
`rejected@core@doc-hidden-test-items`, `rejected@core@testing-feature-for-test-items` and
`rejected@core@inputs-struct-literal`. Met by each new crate's facade, and by #testing-crate's
integrated tests through the public API.

### The test fixtures never reach a package `##fixtures-unshipped`

Binding as a presumption, from `design@knowledge-architect@package-include-whitelist`. Met by
#testing-crate.

### The fixed cost of each new crate `##cost-per-crate`

Weighed. Each new crate name owes the obligations listed under "How the work is done", and a first
publish by hand. Ten new names: `git`, `model`, `check`, `query`, `transcript`, `create`, `testing`,
and later `bundle`, `render`, `rewrite`. Unmet-and-accepted needs the owner's word naming it: D1.

### Re-pointing the references to the renamed anchor `##rename-cost`

Weighed. 1,475 backticked references in 153 files name `core`. Accepted by the owner's ruling on
#core-renamed-check, R5.

## Threads

### `git` is a Component of its own: every git invocation and the readers of git's objects, later the write side of the history rewriting `##git-crate`

Proposed by the session, R1 (#arg-5 to #arg-7), from the owner's plan, R1 (#arg-2). Final state:
approved, by the owner's word against the table, R3: "For the split: I agree with your table now."
and R6: "Agreed on the table." Arguments: #arg-2, #arg-5, #arg-6, #arg-7, #arg-19, #arg-97, #arg-114.
Shape: `spec@component-split@git-out`. Harvest: the `git` Component's design home.

### `bundle` is placed as a Component of its own, the generic build and install machinery of shipped agent text, created by its own work `##bundle-crate`

Proposed by the owner, R1 (#arg-4); placed by the session, R1 (#arg-8), and R4 (#arg-89). Final
state: approved. The owner, R5: "bundle-crate: goal can be written when it is created. For now, we
are only planning for its placement in the split, not even how it will be implemented." Arguments:
#arg-4, #arg-8, #arg-89, #arg-104. Shape: "Decided design" below. Harvest: the leads of
`issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`, written in
the commit that adds this document.

### `model` holds the manifest, the walk, the scan, the entity table, phases 1 to 3, the resolver separated from the lints, and the phase-1 contribution an extension makes `##model-crate`

Proposed by the owner, R2 (#arg-26 to #arg-30), against the session's first cut (#arg-16); the
session's R2 measurement (#arg-42 to #arg-47); the foundation placed by the owner, R4 (#arg-68);
the phase-1 contribution found by the record search (#arg-93). Final state: approved, by the owner's
word against the checkpoint, R6: "Agreed on the table." The owner, R4: "check::foundation phases 1
to 3 are basically checking that the model is right and can be properly scanned, not checking the
content. It probably belongs with the model and scanner, not the checks." Arguments: #arg-16,
#arg-26 to #arg-30, #arg-42 to #arg-47, #arg-68, #arg-93, #arg-110, #arg-111, #arg-112. Shape:
`spec@component-split@model-out`. Harvest: the `model` Component's design home.

### `query` holds the commands that read the model and run no check: show, issues, tripwires, anchors, id and worklist, and later the keyword search and the audit's work lists `##query-crate`

Proposed by the owner, R2 (#arg-31), from the session's `worklist` cut (#arg-9, #arg-10) and its
`explore` (#arg-48). Final state: approved. The owner, R3: "For the naming, I like query more than
explore." Arguments: #arg-9, #arg-10, #arg-29, #arg-30, #arg-31, #arg-48, #arg-64, #arg-112. Shape:
`spec@component-split@consumers-out`. Harvest: the `query` Component's design home. Absorbs
#worklist-crate.

### `render` is placed as a Component of its own, depending on `model` alone, created by its own work `##render-crate`

Proposed by the session, R1 (#arg-11, #arg-12), from the owner's plan, R1 (#arg-3). Final state:
approved. The owner, R3: "And I agree render is out of it. It has a different purposes, and takes
decisions about a different subject". Arguments: #arg-3, #arg-11, #arg-12, #arg-49, #arg-65. Shape:
"Decided design" below. Harvest: the leads of `issue@core@the-documents-do-not-render-as-a-site`.

### `transcript` is a Component of its own, the reader of the agent harness's transcripts, depending on `git` `##transcript-crate`

Proposed by the session, R1 (#arg-9, #arg-10). Final state: approved, by the owner's word against
the table, R3 and R6. Arguments: #arg-9, #arg-10. Shape: `spec@component-split@consumers-out`.
Harvest: the `transcript` Component's design home.

### `rewrite` is placed as a Component of its own, depending on `git` alone, created by the port `##rewrite-crate`

Proposed by the owner, R1 (#arg-1, #arg-2); placed by the session, R1 (#arg-13), and R4 (#arg-90,
#arg-91). Final state: approved, by the owner's word against the table, R6. Arguments: #arg-1,
#arg-2, #arg-13, #arg-34, #arg-90, #arg-91. Shape: "Decided design" below. Harvest: the leads of
`issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`.

### `create` is a Component of its own: `init`, `new-component` and `init --rust` `##create-crate`

Proposed by the session, R1 (#arg-14), as the cut weakest by the signals. Final state: approved, by
the owner's word against the table, R3 and R6. Arguments: #arg-14. Shape:
`spec@component-split@consumers-out`. Harvest: the `create` Component's design home.

### `cli` is the top-level crate: the package `knowledge-architect`, the binary `klarch`, the parsing of every klarch command, and the re-export of the extension API; the library and the binary are no longer one crate `##cli-crate`

Proposed by the session, R1 (#arg-15, #arg-20, #arg-21); the owner's argument, R2 (#arg-32,
#arg-33). Final state: approved. The owner, R2: "I think it is better to have the binary as a "top
level" crate, such as the cli crate you proposed, which pulls the logic from the others and
implements the interface, potentially pulling rendering logic from another crate if that applies
(such as a project with a gui). Though the difficulty here mostly came from our own recorded
decision, which is quite old, made for reasons that no longer apply identically, and can be
reversed." Arguments: #arg-15, #arg-20, #arg-21, #arg-32, #arg-33, #arg-96, #arg-98. Shape:
`spec@component-split@cli-on-top`. Harvest: the root design home, for the layout; the `cli`
Component's design home.

### `testing` is a shared test Component: it publishes the harness an extension's tests need, and holds this repository's fixtures and integrated tests in its unshipped tests `##testing-crate`

Proposed by the owner, R2 (#arg-37 to #arg-41), against the session's R1 cut (#arg-18); shaped by
the session, R2 (#arg-55 to #arg-57), and R4 (#arg-83, #arg-84, #arg-87, #arg-88). Final state:
approved, by the owner's word against the checkpoint, R6: "Agreed on the table." The owner, R2:
"our mocking tools need to be published, because we support writing extensions, which needs the
mocking tools publsished. It could be published as its own crate too, though. Also, centralizing
mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in
multiple crates that need them, or crates reaching into mock projects of others." Arguments:
#arg-18, #arg-37 to #arg-41, #arg-55 to #arg-57, #arg-62, #arg-83, #arg-84, #arg-87, #arg-88,
#arg-101, #arg-102, #arg-108, #arg-110, #arg-118. Shape: `spec@component-split@integrated-tests`.
Harvest: the `testing` Component's design home; `design@core@api-facade`, rewritten.

### No intermediate Component groups the new Components `##no-intermediate-component`

Proposed by the session, R1 (#arg-17). Final state: approved, by the owner's word against the
table, R3 and R6. Arguments: #arg-17. Shape: "Decided design" below. Harvest: the root design
home's layout head states the absence.

### The core is renamed `check` `##core-renamed-check`

Proposed by the session, R1 and R2 (#arg-21, #arg-51), and R4 (#arg-95). Final state: approved. The
owner, R5: "core-renamed-check agreed to rename, core would become misleading". Arguments: #arg-21,
#arg-51, #arg-95, #arg-106. Shape: `spec@component-split@check-renamed`. Harvest: no head; the
commit records it.

### The installed-file check is a check family of the last phase, in `check` `##installed-text-check-home`

Proposed by the session, R4 (#arg-69, #arg-70), as shape B; replaced by B′ after the record search
(#arg-81, #arg-82). Final state: approved. The owner, R5: "installed-text-check-home: agreed on phase
4". Arguments: #arg-50, #arg-68 to #arg-70, #arg-81, #arg-82, #arg-103, #arg-115. Shape:
`spec@component-split@check-renamed`. Harvest: `design@core@phases-gate-the-report`,
`design@core@owned-namespace-check` and `design@core@fix-before-the-checks` rewritten; the issue
closed.

### The arguments of a command are parsed in the lowest crate that every binary offering the command depends on: `cli` for klarch, the crate itself for `gates` and `mutate` `##command-parsing-home`

Proposed by the owner, R2 (#arg-35); challenged at the owner's request, R3 (#arg-61), by the
session, R4 (#arg-71 to #arg-78). Final state: approved. The owner, R5: "command-parsing-home I
think your synthesis is right." Arguments: #arg-35, #arg-53, #arg-61, #arg-71 to #arg-78, #arg-100,
#arg-113. Shape: `spec@component-split@cli-on-top`. Harvest: the `cli` Component's design home;
`design@core@arguments-parse-through-clap` rewritten.

### The record worklist is a Component of its own `##worklist-crate`

Proposed by the session, R1 (#arg-9, #arg-10). Final state: superseded by #query-crate, which holds
`worklist` with the other commands that read the model (#arg-31, #arg-48).

## Arguments

The arguments of the discussion that bear on this milestone's threads, numbered as the
discussion's extraction numbered them, so this document and `spec@plans@component-rules-revision`
name one argument by one number. Each quotes its key words verbatim.

### History rewriting is quite common action that agents often struggle with because git is not very (cut) `##arg-1`

R1, the owner. Threads: [history rewriting] #rewrite-crate.

"history rewriting is quite common action that agents often struggle with because git is not very easy/convenient to use non-interactively. the most common is correcting a commit message, but other actions happen too, reordering or fixing a previous commit of the branch"

### It would need both a model of git command, and a way to read tracked file and commit content, as a (cut) `##arg-2`

R1, the owner. Threads: #rewrite-crate, #git-crate.

"It would need both a model of git command, and a way to read tracked file and commit content, as a foundation for the actual commands, IIRC." / "I do not want to handle the port now, but this plan of mine has to be factored in when discussing the split."

### Rendering the project documentation into html, via a preprocessing of the walked markdown documents, then (cut) `##arg-3`

R1, the owner. Threads: [html rendering] #render-crate.

"rendering the project documentation into html, via a preprocessing of the walked markdown documents, then through mdbook (though this is more of a technical design lead)"

### The plan to split the agent-skill component into a separate utility skill bundling crate, plus our shipped (cut) `##arg-4`

R1, the owner. Threads: [skill bundling] #bundle-crate.

"the plan to split the agent-skill component into a separate utility skill bundling crate, plus our shipped skils relying on it."

### Its own reason to change: a git query or a minimum git version that the rewriter needs and the checker (cut) `##arg-5`

R1, the session. Threads: #git-crate.

"Its own reason to change: a git query or a minimum git version that the rewriter needs and the checker doesn't."

### Its own decisions: the per-user ignore file pointed at the null device, repository resolution, honouring (cut) `##arg-6`

R1, the session. Threads: #git-crate.

"Its own decisions: the per-user ignore file pointed at the null device, repository resolution, honouring `GIT_INDEX_FILE`, paths kept as bytes, git 2.36, and "a failure is exit 2 with git's reason". These heads move. The checker's meaning of the walk stays in core"

### The deciding argument: without this crate, 'rewrite' would depend on the whole checker (tree-sitter, (cut) `##arg-7`

R1, the session. Threads: #git-crate, #rewrite-crate.

"The deciding argument: without this crate, `rewrite` would depend on the whole checker (tree-sitter, pulldown-cmark, toml_edit) just to reach a module that is `pub(crate)` today."

### The generic forms of 5 agent-skills heads move `##arg-8`

R1, the session. Threads: #bundle-crate.

"The generic forms of 5 agent-skills heads move" / "The heads about this workflow's own content stay, for example "content/ is in the walk" and the setup snippets."

### Each meets all four criteria on code that exists today `##arg-9`

R1, the session. Threads: #transcript-crate, #worklist-crate.

"each meets all four criteria on code that exists today. `transcript.rs` imports nothing from the crate, and `cli/transcript.rs` uses only `output`, `transcript` and `Manifest`."

### Each changes for a reason the checker doesn't share: Claude Code's transcript format for one, the (cut) `##arg-10`

R1, the session. Threads: #transcript-crate, #worklist-crate.

"Each changes for a reason the checker doesn't share: Claude Code's transcript format for one, the record-search procedure for the other."

### The pure/output-layer rule keeps an output layer with its data unless it "decides a layout `##arg-11`

R1, the session. Threads: #render-crate.

"the pure/output-layer rule keeps an output layer with its data unless it "decides a layout". A site does decide one: page tree, link targets, SUMMARY."

### The mdbook dependency tree also stays out of every extension's build `##arg-12`

R1, the session. Threads: #render-crate.

"The mdbook dependency tree also stays out of every extension's build."

### Depends on 'git' only `##arg-13`

R1, the session. Threads: #rewrite-crate.

"depends on `git` only. Composing it with the checker (reword, then `commits`) happens in `cli` or in a skill's procedure, so the crate needs no edge to `core`."

### Passes the criteria (own purpose, own tests, 13 heads) `##arg-14`

R1, the session. Threads: #create-crate.

"passes the criteria (own purpose, own tests, 13 heads). It is the weakest cut by the signals: it needs `fix_then_check`, `locate`, `complete_working_tree` and the foreign-build refusal to become public API of `core`."

### The commands module has to sit above every crate that contributes a subcommand `##arg-15`

R1, the session. Threads: #cli-crate.

"the commands module has to sit above every crate that contributes a subcommand. If it stays in core, `render` and `worklist` (both depend on core) would form a cycle with it."

### A new register kind or a new reference form touches both, so almost every expected change would cross the (cut) `##arg-16`

R1, the session. Threads: [model apart from checks] #model-crate.

"A new register kind or a new reference form touches both, so almost every expected change would cross the boundary. That is the third signal against a split." (retracted in #arg-45)

### Every candidate grouping (core + worklist + render + create, or bundle + agent-skills) shares design that (cut) `##arg-17`

R1, the session. Threads: #no-intermediate-component.

"Every candidate grouping (core + worklist + render + create, or bundle + agent-skills) shares design that already lives in one member (`core`) or shares none. With no shared design, it has nothing to hold."

### Core's own unit tests use it, so core would have to dev-depend on a crate that depends on core `##arg-18`

R1, the session. Threads: [shared test Component] #testing-crate.

"Core's own unit tests use it, so core would have to dev-depend on a crate that depends on core. The skill itself warns against that double build."

### This is possible, and it would show the boundary is useful `##arg-19`

R1, the session. Threads: [gates/mutate onto git].

"This is possible, and it would show the boundary is useful. The split doesn't require it."

### The entry argued that a separate binary crate "would save its users no dependency `##arg-20`

R1, the session. Threads: #cli-crate (reversal).

"`design@core@single-crate` argued that a separate binary crate "would save its users no dependency". That premise fails once the command set spans crates."

### The core's package is renamed `##arg-21`

R1, the session. Threads: #cli-crate, #core-renamed-check (publishing).

"The core's package is renamed. Extension writers see no break, because `cli` keeps the name `knowledge-architect` and re-exports `extension`."

### It applies "where a crate would force a cycle `##arg-22`

R1, the session. Threads: [hard point 1] #cycle-from-a-reversible-decision.

"It applies "where a crate would force a cycle". Here the cycle comes only from a recorded decision (binary in core). The rules don't say whether to move the binary or use a module directory."

### My argument was that render would only need the model (and the scanner which builds it from disk) `##arg-26`

R2, the owner. Threads: [model/checker split] #model-crate.

"my argument was that render would only need the model (and the scanner which builds it from disk)."

### However, the model and scanner might have limited testability as a standalone crate if split from checker `##arg-27`

R2, the owner. Threads: #model-crate.

"However, the model and scanner might have limited testability as a standalone crate if split from checker."

### And your argument that they basically always impact each other `##arg-28`

R2, the owner. Threads: #model-crate.

"And your argument that they basically always impact each other. However, this would change if we start adding more cli tools to explore or postprocess the scanned model."

### For now, there is mostly worklist and show, as well as the planned render, which do not need the checker `##arg-29`

R2, the owner. Threads: #model-crate, #query-crate.

"For now, there is mostly worklist and show, as well as the planned render, which do not need the checker. This is already non negligible."

### Leads are a command to perform searches in existing entries by list of keywords and report closest matches (cut) `##arg-30`

R2, the owner. Threads: #query-crate, #model-crate.

"Leads are a command to perform searches in existing entries by list of keywords and report closest matches (more efficient than grepping the project, a true search algorithm), mechanical worklists for the audit methodology" / "This is bound to increase, because our goals say that mechanical tasks agent perform due to the workflow have to be provided by klarch"

### I also had in mind one crate for tools that use the scanned models to provide useful functionalities (cut) `##arg-31`

R2, the owner. Threads: #query-crate.

"I also had in mind one crate for tools that use the scanned models to provide useful functionalities (includes the worklist)."

### I think it is better to have the binary as a "top level" crate, such as the cli crate you proposed, which (cut) `##arg-32`

R2, the owner. Threads: #cli-crate, #cycle-from-a-reversible-decision.

"I think it is better to have the binary as a "top level" crate, such as the cli crate you proposed, which pulls the logic from the others and implements the interface, potentially pulling rendering logic from another crate if that applies (such as a project with a gui)."

### The difficulty here mostly came from our own recorded decision, which is quite old, made for reasons that (cut) `##arg-33`

R2, the owner. Threads: #cycle-from-a-reversible-decision, #cli-crate.

"the difficulty here mostly came from our own recorded decision, which is quite old, made for reasons that no longer apply identically, and can be reversed."

### Rewrite is better shaped as one crate with modules rather than one per functionality, because the (cut) `##arg-34`

R2, the owner. Threads: #commands-sharing-a-purpose, #rewrite-crate.

"rewrite is better shaped as one crate with modules rather than one per functionality, because the subcommands it provides share most of the design, ideas and have one centralized purpose (rewrite history), and only differ on subject and application."

### Grouping the parsing logic, and any meaningfull verification that can be applied at parsing, is more (cut) `##arg-35`

R2, the owner. Threads: #command-parsing-home, #cli-crate.

"grouping the parsing logic, and any meaningfull verification that can be applied at parsing, is more convenient, while the application logic should be mostly pulled from other crates (or assembled from high level logic that is provided by other crates)"

### In practice, the condition on unit tests using fixtures you mention would always apply, no ? `##arg-37`

R2, the owner. Threads: #test-component-public-api, #test-through-public-api.

"in practice, the condition on unit tests using fixtures you mention would always apply, no ? My point on that is that there is value in moving integrated testing with mocks or similar out of unit tests, and sometimes out of the crate themselves, so that they focus on testing the actual public API, and not internals"

### (thaum had a different argument for the split which was a dev dependency cycle, but that does not apply (cut) `##arg-38`

R2, the owner. Threads: #testing-crate.

"(thaum had a different argument for the split which was a dev dependency cycle, but that does not apply here I think)"

### I believe that here, unit tests using mock projects is already quite a design smell on the testing (cut) `##arg-39`

R2, the owner. Threads: #test-through-public-api.

"I believe that here, unit tests using mock projects is already quite a design smell on the testing methodology. Though it can be argued that it would be to hard and inefficient to build scanned models without mock projects."

### Our mocking tools need to be published, because we support writing extensions, which needs the mocking (cut) `##arg-40`

R2, the owner. Threads: #testing-crate.

"our mocking tools need to be published, because we support writing extensions, which needs the mocking tools publsished. It could be published as its own crate too, though."

### Centralizing mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in (cut) `##arg-41`

R2, the owner. Threads: #testing-crate.

"centralizing mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in multiple crates that need them, or crates reaching into mock projects of others. This problem did not exists with the project as a single core crate, but it will surface under the split."

### Both 'show' and 'worklist' go through 'complete_working_tree', which runs 'check::foundation', phases 1 to 3 `##arg-42`

R2, the session. Threads: #model-crate.

"Both `show` and `worklist` go through `complete_working_tree`, which runs `check::foundation`, phases 1 to 3" / "Neither of them runs phase 4, the content checks. So the line runs between phases 1–3 and phase 4, not between "model" and `check/`."

### The the entry run-model head explains why tools need phases 1–3: "a finding computed over an incomplete (cut) `##arg-43`

R2, the session. Threads: #model-crate.

"The run-model head explains why tools need phases 1–3: "a finding computed over an incomplete model is unreliable"."

### Resolving a reference and reporting a broken one are one function today `##arg-44`

R2, the session. Threads: #model-crate, #render-crate.

"Resolving a reference and reporting a broken one are one function today." / "Render needs the resolving part with no reporting attached, so this function has to be pulled apart whether or not the crates are split."

### My claim that "every change crosses the boundary" was overstated `##arg-45`

R2, the session. Threads: #model-crate.

"My claim that "every change crosses the boundary" was overstated." / "So 44 of the 99 commits that touch either side touch both. More than half stay on one side. Some of the "both" commits are explained by fact 2, since resolution currently sits in `check/`."

### More consumers of the model, none of them needing a check, are expected `##arg-46`

R2, the session. Threads: #model-crate.

"more consumers of the model, none of them needing a check, are expected. With that, the model has its own reason to change (a new consumer needs a new query) and its own interface."

### On testability: the model's outputs are the findings of phases 1–3, the entity table and reference resolution `##arg-47`

R2, the session. Threads: #model-crate.

"On testability: the model's outputs are the findings of phases 1–3, the entity table and reference resolution. All three can be tested through its public API over mock projects."

### 'explore' is one crate, by your point 2 `##arg-48`

R2, the session. Threads: #query-crate.

"`explore` is one crate, by your point 2. Its commands share one purpose (mechanical tasks over the scanned records) and one design (the graph, the packing, the `--out`/`--verify` exchange with agents)."

### I keep 'render' out of 'explore' `##arg-49`

R2, the session. Threads: #render-crate.

"I keep `render` out of `explore`. Its purpose is publishing a site, it decides a layout, and mdbook's dependency tree would land on every user of `explore`."

### If the byte comparison moves to 'check', the tools would accept that tree `##arg-50`

R2, the session. Threads: #installed-text-check-home.

"If the byte comparison moves to `check`, the tools would accept that tree. I think that is correct: a mismatched install doesn't make the model incomplete. But it changes current behaviour."

### Renaming 'core' to 'check' moves every head and reference that names the anchor 'core' `##arg-51`

R2, the session. Threads: #core-renamed-check.

"Renaming `core` to `check` moves every head and reference that names the anchor `core`. Per `goal@core@relocation-is-one-manifest-edit` that should cost one manifest edit, but the old slugs in commit messages stay as they are."

### Its "two concrete changes" test, applied to one subcommand at a time, passes every time `##arg-53`

R2, the session. Threads: #commands-sharing-a-purpose, #command-parsing-home.

"Its "two concrete changes" test, applied to one subcommand at a time, passes every time. The skill should say that commands sharing one purpose and one design are one Component, with one module each. A top-level binary crate holds the parsing, the checks that can be done at parse time, and the interface, and pulls its logic from below."

### 'mock_projects.rs' is a '[cfg(test)]' module that tests internals `##arg-55`

R2, the session. Threads: #test-component-public-api, #testing-crate.

"`mock_projects.rs` is a `#[cfg(test)]` module that tests internals" / "`path@core@tests/binary.rs` (7,026 lines) already tests only through the binary, so after the split it goes naturally to `cli`'s tests." / "So the shared test Component works here if one rule holds: no crate's unit tests use a mock project."

### The mock-based tests of 'model' and 'check' `##arg-56`

R2, the session. Threads: #testing-crate.

"the mock-based tests of `model` and `check`. Those two can't dev-depend on it without building themselves twice." / "`explore`, `render`, `create` and `cli` sit above `testing`, so they dev-depend on it with no cycle." / "The fixtures then exist once, and no crate reaches into another's."

### The cost: the mock tests that now read internals must be rewritten against the public API `##arg-57`

R2, the session. Threads: #testing-crate, #test-through-public-api.

"The cost: the mock tests that now read internals must be rewritten against the public API. That either shows the API exposes enough, or forces an addition to it."

### Try to challenge my statement about "A top-level binary crate holds the parsing, the checks that can be (cut) `##arg-61`

R3, the owner. Threads: #command-parsing-home.

"try to challenge my statement about "A top-level binary crate holds the parsing, the checks that can be done at parse time, and the interface, and pulls its logic from below". Does it fit the shape you proposed, were each crate implements its own command parsing for what it provides, and the top level cli flattens it. I think this is quite a good shape too."

### I think that the cost of rewriting the mock tests to use the public API is a good thing `##arg-62`

R3, the owner. Threads: #test-through-public-api.

"I think that the cost of rewriting the mock tests to use the public API is a good thing. If the API does not exposes enough, we will see it clearly. This will drive us to improve this public API, and thus also benefit consumenrs who write extensions."

### For the naming, I like query more than explore `##arg-64`

R3, the owner. Threads: #query-crate.

choice, no argument: "For the naming, I like query more than explore." / "I agree with your table now."

### I agree render is out of it `##arg-65`

R3, the owner. Threads: #render-crate.

"I agree render is out of it. It has a different purposes, and takes decisions about a different subject"

### Check::foundation phases 1 to 3 are basically checking that the model is right and can be properly (cut) `##arg-68`

R4, the owner. Threads: #model-crate, #installed-text-check-home.

"check::foundation phases 1 to 3 are basically checking that the model is right and can be properly scanned, not checking the content. It probably belongs with the model and scanner, not the checks."

### 'check/agents.rs' is phase 2, but it checks that the installed files match the shipped text byte for byte `##arg-69`

R4, the session. Threads: #installed-text-check-home.

"`check/agents.rs` is phase 2, but it checks that the installed files match the shipped text byte for byte. That says whether the workflow's copy is current, not whether the tree can be read." / "A stale installed skill still defines its entities, so the model built over it is complete."

### I found nothing that needs a tool to refuse a stale install `##arg-70`

R4, the session. Threads: #installed-text-check-home.

"I found nothing that needs a tool to refuse a stale install. An agent reads the installed files from disk, not through `query`. That is why B is my default."

### The binary that offers 'gates' and 'mutate' is each project's own xtask, written from a snippet `##arg-71`

R4, the session. Threads: #command-parsing-home.

"The binary that offers `gates` and `mutate` is each project's own xtask, written from a snippet. No shared interface crate sits above it. Under P1, every project's xtask would declare the flags again, and the copies would drift. So P1 cannot apply to `gates` and `mutate`."

### Two kinds of binary offer them: 'klarch' itself, and each project's extension binary `##arg-72`

R4, the session. Threads: #command-parsing-home.

"Two kinds of binary offer them: `klarch` itself, and each project's extension binary" / "Under P1 both depend on `cli`, so each command still has one spelling."

### A frontend that isn't a CLI gets a usable API `##arg-73`

R4, the session. Threads: #command-parsing-home.

"A frontend that isn't a CLI gets a usable API. Your GUI case, or `render` driven from another tool, wants `render::build(&model, &RenderOptions)`, not a clap struct. That typed API is also what #test-through-public-api would test against."

### Decisions about the whole interface get one home `##arg-74`

R4, the session. Threads: #command-parsing-home.

"Decisions about the whole interface get one home. `design@core@exit-code-ladder`, flags shared across commands (`--staged`, `--out`), and output conventions apply to every command. Under P2 they are restated in each domain crate."

### Clap leaves the domain crates' interface `##arg-75`

R4, the session. Threads: #command-parsing-home.

"clap leaves the domain crates' interface. `design@core@arguments-parse-through-clap` says "clap is part of the core's public interface". Under P1 only `cli` has that cost."

### Under P1, the domain crate states it as a type, for example 'enum CheckMode { Working, Staged, Fix }', so (cut) `##arg-76`

R4, the session. Threads: #command-parsing-home.

"Under P1, the domain crate states it as a type, for example `enum CheckMode { Working, Staged, Fix }`, so the invalid combination cannot be built. `cli`'s `conflicts_with` only refuses it early. The type is the rule, and the declaration spells it."

### Each new command touches two crates `##arg-77`

R4, the session. Threads: #command-parsing-home.

"Each new command touches two crates. The interface and the logic are then two subjects, which is the point of the split."

### Argument parsing lives in the lowest crate that every binary offering the command depends on `##arg-78`

R4, the session. Threads: #command-parsing-home.

"argument parsing lives in the lowest crate that every binary offering the command depends on. For klarch's commands that is `cli`, which is your shape. For `gates` and `mutate` it is the crate itself, since no shared crate exists above them."

### The entry says these findings belong in phase 4 under the core's own placement rule: the files are outside (cut) `##arg-81`

R4 (L756), the session. Threads: #installed-text-check-home.

finding 1: "[the issue `installed-file-findings-belong-in-phase-four`] says these findings belong in phase 4 under the core's own placement rule: the files are outside the walk, and no check reads the findings." / "The split moves this code anyway, so the reason for deferring mostly goes away."

### 'model' then depends on neither 'agent-skills' nor 'bundle' `##arg-82`

R4 (L756), the session. Threads: #installed-text-check-home.

"`model` then depends on neither `agent-skills` nor `bundle`." / "`index` and `index --staged` are no longer refused by a stale install." / "This is a timing fork you already ruled the other way. It's yours"

### The testing crate moves the 36 mock-based tests `##arg-83`

R4 (L756), the session. Threads: #test-through-public-api, #testing-crate.

finding 2: "#testing-crate moves the 36 mock-based tests. That doesn't bring back the rejected entry, which was about moving all unit tests." / "The guideline must be an order of preference" / "As a rule to move every test, it is the rejected design."

### 5 of the 36 build 'Inputs' by hand `##arg-84`

R4 (L756), the session. Threads: #testing-crate, #test-through-public-api.

"5 of the 36 build `Inputs` by hand." / "Through the public API, each needs a mock project whose tree really has that state. Making `Inputs` constructible would bring back `rejected@core@inputs-struct-literal`. Default: rewrite the 5 over mock projects."

### The entry says "the test fixtures and the design documents never reach a package `##arg-87`

R4 (L756), the session. Threads: #testing-crate.

finding 3: "`design@knowledge-architect@package-include-whitelist` says "the test fixtures and the design documents never reach a package"." / "Extension writers get the tools, not our fixtures."

### Constraint: the entry accepts '"fixture"' only inside a directory the binary names as its own source, so (cut) `##arg-88`

R4 (L756), the session. Threads: #testing-crate.

"Constraint: `design@core@checked-sentinel-values` accepts `"fixture"` only inside a directory the binary names as its own source, so `testing`'s directory must be on that list."

### 'bundle' needs a goal first `##arg-89`

R4 (L756), the session. Threads: #bundle-crate.

finding 4: "`bundle` needs a goal first" / "It also leaves open which crate holds the install." / "`bundle` holds the generic install" / "`check` holds the comparison family and the command."

### The entry counts three history tasks as taken in: the rebase, the reword and the fold `##arg-90`

R4 (L756), the session. Threads: #rewrite-crate.

finding 5: "`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` counts three history tasks as taken in: the rebase, the reword and the fold." / "The port's other commands (`prune-dead-paths`, `split-by-path`, `list-large-paths`) are a generic part taken in on your word" / "That word belongs at the port, not now."

### The entry binds every command 'rewrite' ships `##arg-91`

R4 (L756), the session. Threads: #rewrite-crate.

"`design@knowledge-architect@no-operation-loses-content` binds every command `rewrite` ships."

### 'resolve' runs in phase 1, which now sits in 'model' `##arg-93`

R4 (L756), the session. Threads: #model-crate.

finding 7: "`resolve` runs in phase 1, which now sits in `model`." / "So `model` must define a phase-1 contribution interface (claimed tables, declared paths, generated files), and `check`'s `Extension` trait builds on it. One trait becomes two. That is a design point for the slice, not a reason against the split."

### A crate's first version can't go through the release workflow: it needs an API token, by hand `##arg-94`

R4 (L756), the session. Threads: (every new crate).

finding 8: "a crate's first version can't go through the release workflow: it needs an API token, by hand." / "Default: a crate is created when its slice builds it, so `render` and `rewrite` wait for their work. Each first publish is your word at the release."

### Renaming a directory "renames its anchor `##arg-95`

R4 (L756), the session. Threads: #core-renamed-check.

finding 9: "renaming a directory "renames its anchor ... and is no move", so this is outside its guarantee. Most of those references get re-pointed by the move to `model`, `query`, `git` and the rest anyway" / "with `model` split out, "core" no longer says which crate it is."

### The entry already anticipates this split `##arg-96`

R4 (L756), the session. Threads: #cli-crate.

finding 10: "`design@knowledge-architect@no-component-directory-named-after-the-project` already anticipates this split" / "Keeping the package name `knowledge-architect` on `cli` leaves the setup snippet's dependency line ... unchanged." / "`rejected@gates@gates-command-of-the-checker` binds `cli`: it flattens no `gates` command."

### The entry makes a backticked the entry span a finding once an anchor 'git' exists `##arg-97`

R4 (L756), the session. Threads: #git-crate (Decided).

"`design@core@at-span-candidate-rule` makes a backticked `git@host:…` span a finding once an anchor `git` exists. The census found one such span ... inside a string literal of the checker's own source, which the checker reads as data. No prose is affected today."

### The successor of the entry is recorded in the root design home `##arg-98`

R4 (L756), the session. Threads: #cli-crate (Decided).

"The successor of `design@core@single-crate` is recorded in the root design home. The component-creation skill puts layout decisions there. That also sidesteps the case `issue@agent-skills@the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface` asks about."

### Command-parsing-home I think your synthesis is right `##arg-100`

R5, the owner. Threads: #command-parsing-home.

ruling, no argument: "command-parsing-home I think your synthesis is right."

### What I meant is an integrated tests, that mocks real world application `##arg-101`

R5, the owner. Threads: #test-through-public-api.

"What I meant is an integrated tests, that mocks real world application. Such a test uses the public API just like a consumer would. If it cannot examine the returned objects fromthe public API in sufficient detail, this means that things are being hidden from public consumers, which is a problem."

### If the test is trying to reach into an internal that would have no observable effect to a public consumer, (cut) `##arg-102`

R5, the owner. Threads: #test-through-public-api.

"If the test is trying to reach into an internal that would have no observable effect to a public consumer, then the test is in the wrong place and should not be an integrated test using mock proejcts."

### Installed-text-check-home: agreed on phase 4 `##arg-103`

R5, the owner. Threads: #installed-text-check-home.

ruling, no argument: "installed-text-check-home: agreed on phase 4"

### Bundle-crate: goal can be written when it is created `##arg-104`

R5, the owner. Threads: #bundle-crate.

"bundle-crate: goal can be written when it is created. For now, we are only planning for its placement in the split, not even how it will be implemented."

### Core-renamed-check agreed to rename, core would become misleading `##arg-106`

R5, the owner. Threads: #core-renamed-check.

"core-renamed-check agreed to rename, core would become misleading"

### Your clarification of the thread on testing through the public API settles the conflict with the entry `##arg-107`

R5, the session. Threads: #test-through-public-api.

"Your clarification of #test-through-public-api settles the conflict with `rejected@core@unit-tests-moved-under-tests`. The rule applies to integrated tests over mock projects, not to unit tests." (restated guideline quoted in section 2, L771)

### This doesn't touch the 437 unit tests `##arg-108`

R5, the session. Threads: #test-through-public-api, #testing-crate.

"This doesn't touch the 437 unit tests. The 5 mock tests that build `Inputs` by hand each go one of two ways: the state they inject becomes a real mock tree, such as one with no committed file, or the claim becomes a unit test over an in-memory model, which `design@core@model-then-checks` already provides for."

### When an integrated test is blocked by a missing observation, the entry fires, and its response already (cut) `##arg-109`

R5, the session. Threads: #test-through-public-api.

"When an integrated test is blocked by a missing observation, `tripwire@core@private-item-needed` fires, and its response already routes the fix."

### You didn't address the last three findings directly `##arg-110`

R5, the session. Threads: #model-crate, #testing-crate, #test-component-public-api.

"You didn't address the last three findings directly. Your next word against this table settles them."

### 'check', 'query' and 'render' need so many 'model' internals that 'model''s public API becomes its module (cut) `##arg-111`

R5, the session. Threads: #model-crate (premortem C1).

"`check`, `query` and `render` need so many `model` internals that `model`'s public API becomes its module tree, which brings back `rejected@core@module-tree-as-public-api`" / caught by "`model`'s lib.rs is a facade of role modules under `unreachable_pub`"

### A 'query' command needs phase-4 output, so 'query' depends on 'check' `##arg-112`

R5, the session. Threads: #query-crate, #model-crate (C2, A1).

"a `query` command needs phase-4 output, so `query` depends on `check`" / "A1: after the split, `cargo tree` for `model`, `query` and `render` shows no dependency on `check`." / "A dependency added this way passes every gate, so the failure would otherwise be silent."

### A rule stated in a domain type and in the clap declaration drift apart, so an invalid combination passes (cut) `##arg-113`

R5, the session. Threads: #command-parsing-home (C3).

"a rule stated in a domain type and in the clap declaration drift apart, so an invalid combination passes clap and is refused later, or with another exit code" / "`cli` maps every refusal by a domain's options to exit 2 before any work, one test per refusal"

### 'rewrite''s write side needs an environment that the git heads say every invocation drops or keeps, such (cut) `##arg-114`

R5, the session. Threads: #git-crate (C4).

"`rewrite`'s write side needs an environment that the git heads say every invocation drops or keeps, such as a temporary `GIT_INDEX_FILE`" / "the port's design meets it as a request; no tripwire"

### In phase 4, a stale install shows reference findings beside the install finding, and a session repairs the (cut) `##arg-115`

R5, the session. Threads: #installed-text-check-home (C5).

"in phase 4, a stale install shows reference findings beside the install finding, and a session repairs the references instead of running the install" / "`--fix` repairs the installed files first; judged at the end"

### An extension writer needs a helper that lives in 'testing''s unshipped 'tests/' `##arg-118`

R5, the session. Threads: #testing-crate (C8).

"an extension writer needs a helper that lives in `testing`'s unshipped `tests/`" / "`tripwire@core@private-item-needed` already covers a consumer needing an item it can't reach"

## New names, in one place

```text
crates/git          package knowledge-architect-git          Component git
crates/model        package knowledge-architect-model        Component model
crates/check        package knowledge-architect-check        Component check (today's crates/core)
crates/query        package knowledge-architect-query        Component query
crates/transcript   package knowledge-architect-transcript   Component transcript
crates/create       package knowledge-architect-create       Component create
crates/cli          package knowledge-architect              Component cli; the binary klarch
crates/testing      package knowledge-architect-testing      Component testing
crates/bundle, crates/render, crates/rewrite   placed; created by their own work
```

The names of the types each slice adds, the phase-1 contribution's trait among them, are that
slice's spec's.

## Decided design

The shape of each Component the milestone builds is in the spec of the slice that builds it. The
threads that place work and build nothing:

### `bundle`, `render` and `rewrite` are placed, and created by their own work

Each is a Component of its own, a published crate under crates/, created when its work starts, in
the roadmap's order after this milestone: `rewrite`, then `bundle`, then `render`. `rewrite` depends
on `git` alone (#arg-13); `render` on `model` alone (#arg-11); `bundle` on nothing of the checker,
and `agent-skills` depends on it as a build-dependency (#arg-8, #arg-89). Each is created by the
work that builds it, so no crate name is published before its code exists (#arg-94). The nearest
rival, creating them empty in this milestone, reserves names for code no one has designed.

### No intermediate Component groups the new Components

The eight Components sit beside one another under crates/. An intermediate Component holds the
design several nested Components share, and every candidate grouping shares design that lives in
one member, the checker's or the model's, or shares none (#arg-17). The root design home's head on
the layout states the absence.

## Mapping tables

**Every file of crates/core today, and where it goes.** The slice named moves it; a file split
between two crates is cut at that slice's audit along the line the table gives.

| today | goes to | slice |
| --- | --- | --- |
| `path@core@src/git.rs` | `git` | #git-out |
| `path@core@src/manifest.rs`, `path@core@src/walk.rs`, `path@core@src/survey.rs`, `path@core@src/scan.rs`, `path@core@src/source/`, `path@core@src/document.rs`, `path@core@src/entity.rs`, `path@core@src/records.rs`, `path@core@src/model.rs`, `path@core@src/index.rs`, `path@core@src/finding.rs` | `model` | #model-out |
| `path@core@src/check/tree.rs` | `model`: phase 2 | #model-out |
| `path@core@src/check/mod.rs` | `model`: `foundation`; `check`: phase 4, `Report`, `run_with` | #model-out |
| `path@core@src/check/references.rs` | `model`: the resolver; `check`: the lints | #model-out |
| `path@core@src/agents.rs` | `model`: the installed copies as the entity table reads them; `check`: the install and its repairs | #model-out |
| `path@core@src/extension.rs` | `model`: the phase-1 contribution; `check`: the rest of `Extension` and `Prepared` | #model-out |
| `path@core@src/check/agents.rs` | `check`: a family of phase 4 | #check-renamed |
| `path@core@src/check/generated.rs`, `path@core@src/check/registers.rs` | `check` | #check-renamed |
| `path@core@src/cli/history.rs` | `check` | #check-renamed |
| `path@core@src/cli/gathered.rs` | `model`: what phases 1 to 3 read; `check`: what phase 4 reads | #model-out |
| `path@core@src/cli/mod.rs` | `cli`: the command enum, parsing, dispatch, `locate`; each domain crate: its commands' logic, behind a typed API | #cli-on-top |
| `path@core@src/cli/output.rs` | `cli`, or the crate whose data it prints, per the pure and output layers of `skill@knowledge-architect-component-creation@component-shapes`, decided at the audit | #cli-on-top |
| `path@core@src/build_origin.rs`, `path@core@src/main.rs` | `cli` | #cli-on-top |
| `path@core@src/cli/worklist.rs`, `path@core@src/graph.rs` | `query` | #consumers-out |
| `path@core@src/cli/transcript.rs`, `path@core@src/transcript.rs` | `transcript` | #consumers-out |
| `path@core@src/cli/create.rs` | `create` | #consumers-out |
| `path@core@src/lib.rs` | each crate's own facade | each slice |
| `path@core@src/mock_projects.rs`, `path@core@src/testing.rs` | `testing` | #integrated-tests |
| `path@core@tests/binary.rs` | `cli` | #cli-on-top |
| `path@core@tests/init.rs` | `create` | #consumers-out |
| `path@core@tests/extension_api.rs`, `path@core@tests/projects/` | `testing` | #integrated-tests |
| `path@core@tests/transcripts/` | `transcript` | #consumers-out |
| `path@core@tests/worklist-entries-before-the-snapshot.txt` | `query` | #consumers-out |

**Every command, and the crate that holds its logic.** `cli` parses all of them, per
#command-parsing-home.

| command | crate |
| --- | --- |
| `check`, `index`, `commits`, `install-agent-skills` | `check` |
| `model`: it calls each extension's dump hook, so it needs the extensions | `check` |
| `show`, `issues`, `tripwires`, `anchors`, `id`, `worklist` | `query` |
| `transcript` | `transcript` |
| `init`, `new-component` | `create` |

## Losing alternatives

- **The model left inside the checker** (#arg-16), lost to #model-crate: 35 of the 99 commits that
  touched either side touched the model alone, 20 the checks alone, so the boundary is crossed by 44,
  not by every change (#arg-45), and the goals predict more consumers of the model that need no
  check (#arg-30).
- **The library and the binary in one crate**, `design@core@single-crate`, lost to #cli-crate: its
  premise, that a separate binary crate saves its users no dependency, fails once the command set
  spans crates, and a commands module in the checker would form a cycle with `query` and `create`
  (#arg-15, #arg-20).
- **The `render` and `worklist` code as module directories inside the checker**, the
  module-directory exception, lost to #cli-crate: the cycle came from a decision that can be
  reversed (#arg-22, #arg-33).
- **P2, each domain crate parsing its own commands**, lost to #command-parsing-home for klarch's
  commands (#arg-73 to #arg-75); it stays the shape of `gates` and `mutate` (#arg-71).
- **Shape A, the installed-file check kept in the foundation**, and **shape B, moved to `check` and
  added to phase 2**, lost to B′ (#arg-81): the findings belong in phase 4 by the core's own rule.
- **`render` inside `query`**, lost to #render-crate: it decides a layout, and mdbook's dependency
  tree would reach every user of `query` (#arg-49, #arg-65).
- **One crate per worklist**, superseded by #query-crate.
- **`explore` as the query crate's name**, lost on the owner's choice, R3.
- **The core's name kept**, lost to #core-renamed-check: with `model` split out, "core" no longer
  says which crate it is (#arg-95).
- **No shared test Component, the mock tests kept as a unit-test module**, lost to #testing-crate
  (#arg-37, #arg-41, #arg-55).

## Readings

Empty: the work reads no external specification. The two facts of Cargo the split rests on are read
in `path@agent-skills@docs/grounding/crate-boundaries.md`.

## Premortem

Assume the split shipped and failed:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| C1: `check`, `query` and `render` need so many `model` internals that `model`'s public API becomes its module tree, `rejected@core@module-tree-as-public-api` back | #model-crate | a claim of #model-out: `model`'s lib.rs is a facade of role modules under `unreachable_pub` |
| C2: a `query` command needs phase-4 output, so `query` depends on `check` | #query-crate, #model-crate | an acceptance criterion, #a1-no-check-below-the-tools; the record search found one such case before it was built, the `model` command, which the mapping table keeps in `check` |
| C3: a rule stated in a domain type and in the clap declaration drift apart, so an invalid combination passes clap and is refused later, or with another exit code | #command-parsing-home | a claim of #cli-on-top: `cli` maps every refusal by a domain's options to exit 2 before any work, one test per refusal |
| C4: `rewrite`'s write side needs an environment the git heads say every invocation drops or keeps, such as a temporary `GIT_INDEX_FILE` | #git-crate | named as a cause; the port meets it as a request of its own design, so no tripwire |
| C5: in phase 4, a stale install shows reference findings beside the install finding, and a session repairs the references instead of running the install | #installed-text-check-home | named as a cause for the harvest of #check-renamed to judge; `--fix` repairs the installed files first |
| C8: an extension writer needs a helper that lives in `testing`'s unshipped tests | #testing-crate | named as a cause; `tripwire@core@private-item-needed` already watches a consumer needing an item it cannot reach |

C6 and C7 of the discussion stress the threads of `spec@plans@component-rules-revision`, which holds
them.

## Acceptance criteria

### No crate below the tools depends on the checks: `cargo tree` for `model`, `query` and, once it exists, `render` shows no `knowledge-architect-check` `##a1-no-check-below-the-tools`

Guards #model-crate, whose premise is that the tools over the record need the model and no check.
Judged at #consumers-out, for `model` and `query`, and at the milestone's last landing. Fires when
`cargo tree -p knowledge-architect-query` or `cargo tree -p knowledge-architect-model` lists
`knowledge-architect-check`. A dependency added this way passes every gate, so the failure is
otherwise silent.

## Implementation sequence

1. [The git invocation layer and the readers of git's objects become the `git` crate](git-out.md),
   `spec@component-split@git-out`.
2. [The model, the scan, phases 1 to 3 and the resolver become the `model` crate](model-out.md),
   `spec@component-split@model-out`.
3. [The core is renamed `check`, and the installed-file check moves to phase 4](check-renamed.md),
   `spec@component-split@check-renamed`.
4. [The binary, the parsing and the interface become the top-level `cli` crate](cli-on-top.md),
   `spec@component-split@cli-on-top`.
5. [The commands over the model, the transcript reader and the project creator leave the checker](consumers-out.md),
   `spec@component-split@consumers-out`.
6. [The shared test Component holds the fixtures and the integrated tests](integrated-tests.md),
   `spec@component-split@integrated-tests`.

## Order rationale

- #git-out before #model-out: the model's walk and snapshot read git through the crate the first
  slice lands, and the first slice moves the module that has no caller of its own in the crate.
- #model-out before #check-renamed: the rename then renames what is left of the checker, not the
  model.
- #check-renamed before #cli-on-top: `cli` depends on the renamed crate under its final package
  name, which the rename frees.
- #cli-on-top before #consumers-out: `create` depends on `check`, so its command can leave the
  checker only once the binary sits above both.
- #consumers-out before #integrated-tests: the integrated tests drive every crate through its public
  API, which each crate has only once it is out.

## Defaults awaiting the owner

- **D1, on #cost-per-crate**: the fixed cost of each new crate is accepted, and each crate is
  created by the slice that builds it, `bundle`, `render` and `rewrite` by their own work. The
  owner's word "Agreed on the table", R6, was given against the checkpoint of threads; the criteria
  table, where this cost stood as unmet-and-accepted awaiting a word naming it, was a separate
  table. Built as written until the owner rules.

## Harvest

| what | where | when |
| --- | --- | --- |
| the layout: which Components exist, their dependencies, no intermediate Component, the successor of `design@core@single-crate` | a head of the root design home, `register@knowledge-architect@design`, with `design@knowledge-architect@two-crates` and `design@knowledge-architect@mutation-command-host`'s publish order judged | at the landing of #integrated-tests, the last slice |
| each slice's threads | its spec's harvest row | at that slice's landing |
| the reversed decisions and the losing alternatives above | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | at the landing of the slice that decides each |
| C4, C5, C8 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | at the milestone's last landing |
| #a1-no-check-below-the-tools | reported at #consumers-out and at the last landing | as `skill@knowledge-architect-planning@acceptance-criteria` says |

## Later consequences

- `rewrite`, `bundle` and `render` are each created by their own work, in the roadmap's order; their
  issues hold the leads.
- The release after the milestone publishes each new crate's first version by hand, on the owner's
  word, per `design@knowledge-architect@publish-credential`; the release workflow's crate list and
  order, and the package lists of `skill@klarch-release`, take the new crates.
- `gates` and `mutate` may later move onto the `git` crate; nothing schedules it.
