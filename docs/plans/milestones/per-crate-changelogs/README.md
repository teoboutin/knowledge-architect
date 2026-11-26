# Per-crate changelogs, a model of them with its checks, a command that assembles them from the registry, and files declared free of citations

## Status and audience

This milestone is the plan of the work that replaces the one changelog every crate ships with one
changelog per published crate, reads those files into a model with checks over it, assembles the
project's changelog from them, gives a consuming project a command that assembles the changelogs of
its dependencies from the sources cargo fetched, and lets a manifest declare files that must hold
no citation. It is written for a session that did not witness the design discussion that produced
it. It leaves the repository, its directory whole, in a commit of its own, once its last slice has
landed and every default it lists is ruled, per
`skill@knowledge-architect-planning@plan-document-leaves`.

- Where this document, or the spec of one of its slices, and a design home disagree on a point
  that "What is already decided" does not list as reversed or rewritten, the design home wins, and
  the disagreement is a defect of this document.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits.
- Its line numbers name the tree at the commit that adds this document; each slice's design audit
  takes them again.

It was assembled from one session's transcript. The discussion begins at the owner's message that
opens "I want to discuss changelog shape, and also shipping a structural model with scan and
lint/checks for changelog". Find the session by that message, never by a session identifier. Its
rounds, each an owner message and the replies to it, named by the message's opening words:

| round | the owner's message opens |
| --- | --- |
| R1 | "I want to discuss changelog shape" |
| R2 | "Agreed on changelog-per-component" |
| R3 | "About changelog-the-pin-reads: this is not a question" |
| R4 | "changelog-the-pin-reads agreed. If crates here" |
| R5 | "About Requiring repository, why was" |
| R6 | "agreed on dropping the ask" |
| R7 | "Q1: if keeping the same trigger" |
| R8 | "table confirmed", the owner's word on the last checkpoint table |
| R9 | "agreed on a milestone with that order" |

Q1 and Q2 are the labels of the two questions the session put to the owner in its reply to R6:
Q1, whether the `cargo-reader` crate stays in lockstep with the other crates, and Q2, whether
`[walk.citation-free]` also covers the installed copies under .claude. The owner ruled both at R7,
whose message opens with Q1. The code-split scout was dispatched at the grounding of the
discussion; its report stayed in that session's scratch space, as
`skill@knowledge-architect-planning@working-a-slice` says of a scout's report, and every claim of it
the design rests on is restated, with its evidence in the tree, in the item or the section that
uses it: `argument@per-crate-changelogs@arg-37`, `argument@per-crate-changelogs@arg-38`,
`argument@per-crate-changelogs@arg-39`, `argument@per-crate-changelogs@arg-40`, and the weighed
issues of slices 1, 3 and 4.

The committed transcript ends at R8's message, as
`skill@knowledge-architect-planning@spec-contents` says. R9, the owner's approval of a milestone of
five slices in the order of "Implementation sequence", came after the last checkpoint, so its
attribution in this document's items is its whole record. The round's arguments are items of
[deliberation](deliberation.md); the record search's generated lines are
[the discussion's search](search/discussion-1.txt).

## How the work is done

Each slice is worked as `skill@knowledge-architect-planning@working-a-slice` says: read it whole
before each slice. Additions this milestone's work needs:

- This document's own branch merges before slice 1's branch starts, so every slice branches from a
  main branch that holds the milestone.
- Each slice is one branch, one pull request and one merge, in the order of "Implementation
  sequence", each merged before the next begins.
- Each slice's design audit dispatches `agent@knowledge-architect-design-record-auditor` over the
  heads its spec lists under "Heads the harvest touches", as
  `skill@knowledge-architect-planning@cutting-steps-and-slices` says. That audit was not run at
  assembly: the lists below are drawn from the search that section names, and the judgement of
  each listed head is owed at the slice's design audit.
- A slice that changes the installed text writes it under
  `skill@knowledge-architect-agent-configuration`, as `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", says, read whole.
- Until slice 5 merges, this repository's changelog stays the one root CHANGELOG.md and its copies:
  slices 1 to 4 write their entries there, under its current grammar, per
  `design@knowledge-architect@changelog-entries`.

## Names

- **the shared changelog**: today's root CHANGELOG.md, 1892 lines and 155049 bytes (`wc -l -c
  CHANGELOG.md`), copied byte for byte into each of the 11 directories under crates/ by `cargo x
  changelog` (`grep -l '"/CHANGELOG.md"' crates/*/Cargo.toml | wc -l`, and `copies` in
  `path@xtask@src/changelog.rs`, which takes every directory under crates/ holding a Cargo.toml),
  per `design@knowledge-architect@the-changelog-ships-in-every-crate`.
- **`cargo x changelog`**: the subcommand of `path@xtask@src/changelog.rs`, whose functions
  `copies` and `sync` write the copies, and whose test
  `every_published_crate_carries_the_root_changelog` fails while one differs.
- **a crate's changelog**: the `CHANGELOG.md` in the directory of a published crate, holding only
  that crate's entries, per `thread@per-crate-changelogs@changelog-per-component`.
- **a changelog crate**: a Component whose crate holds a changelog under
  `thread@per-crate-changelogs@changelog-model-home`: one the `[changelog]` table lists, or, where
  it lists none, one whose `Cargo.toml` has a `[package]` table that Cargo would publish, as slice
  3's spec defines it from the Cargo reference: a `version` set, and `publish` neither `false` nor
  an empty list.
- **the assembled file**: the one changelog of the whole project, generated from every crate's
  changelog at the path `[changelog] assembled` names, per
  `thread@per-crate-changelogs@assembled-changelog-location`.
- **a category**: what an entry's first tag names, one of the list the manifest declares, such as
  Migration, New features and Workflow here; it replaces today's subsection, per
  `thread@per-crate-changelogs@changelog-entry-categories`.
- **a class**: patch, minor or major, the bump class of
  `design@knowledge-architect@versioning-policy`.
- **a surface**: today's first tag of an entry, one of checks, cli, manifest, library,
  agent-skills, gates and mutate, per `design@knowledge-architect@changelog-entries`. The work
  removes it.
- **the preamble**: the lines of a crate's changelog above its first version heading, which the
  checker determines, per `thread@per-crate-changelogs@category-order-in-preamble`.
- **an unread line**: a line of a crate's changelog that the model's parse records as one it could
  not read, per `thread@per-crate-changelogs@changelog-findings-in-check`.
- **the pin move**: `skill@knowledge-architect-setup@moving-the-pin`, whose step 2 tells a
  consuming project which changelog to read, at line 329 of its installed copy under .claude, from
  its source `path@agent-skills@content/skills/setup/SKILL.md`, line 343.
- **the registry sources**: the unpacked source of each package a consuming project depends on, as
  `cargo metadata` reports its manifest path, in the cargo registry or at a path dependency.
- **the resolved graph**: the packages of `cargo metadata`'s resolved dependency graph in the
  consuming project.
- **the installed copies**: the files `cargo klarch install-agent-skills` writes under .claude:
  `path@agent-config@knowledge-architect/PRIMER.md`; the `SKILL.md` of each directory of
  `path@agent-config@skills/` whose name begins with the installer's prefix, knowledge-architect-;
  each file of `path@agent-config@agents/` with that prefix; and each saved workflow of
  `path@agent-config@workflows/` with that prefix. None of them is in the walk: `Model::build` in
  `path@model@src/model.rs` adds every path of the installer's namespace, as `owned_path` in
  `path@model@src/manifest.rs` matches it, to the paths the walk leaves out, per
  `design@check@installed-files-outside-the-walk`; their markdown files are parsed for the entities
  they define, per `design@model@installed-entities-from-the-tree`, and their references are read
  only at the level `entries`, where a glob matches them, per
  `design@check@installed-copies-read-at-the-level-entries`. The saved workflows are `.js` files besides, which no walk would read, since the walk reads
  `md` and `rs` files alone (`LIVE_SUFFIXES` in `path@model@src/walk.rs`).
- **a citation-free file**: a file a `[walk.citation-free]` glob matches, at the level `all` or
  `entries`, per `thread@per-crate-changelogs@citation-free-paths`.

## What the work is

What exists today at each site the work touches:

- **The changelog.** One root CHANGELOG.md. Its preamble says entries inside a subsection are
  sorted by surface. It has one section per version, `## Next release` first, then `## 0.10.0` down
  to `## 0.1.0`; the subsections are `### Migration`, `### New features` and `### Workflow`, an
  empty one omitted (0.5.0 has no New features). An entry is ``- `<surface>`, <class>: <text>``,
  except in 0.1.0, whose 24 entries carry a surface and no class and no subsection
  (`sed -n '/^## 0.1.0/,$p' CHANGELOG.md | grep -c '^- '`). Counted with
  `grep -oE '^- \`[a-z-]+\`, (patch|minor|major)' CHANGELOG.md`: agent-skills 263, checks 54, cli
  35, library 34, manifest 13, gates 3, mutate 1, so 101 classed entries carry a surface that is no
  one crate (checks, manifest, library).
- **The copies.** Every crate under crates/ lists `/CHANGELOG.md` in its `include`, and `cargo x
  changelog` writes the copies. Each copy and the root file is a row of `[walk] skip-files` in
  knowledge-architect.toml, 13 rows since slice 2 (`sed -n '/^skip-files/,/^]/p' knowledge-architect.toml | grep -c
  CHANGELOG.md`), per `design@knowledge-architect@the-changelog-is-outside-the-walk`: no check reads
  any of them. The list is written by hand, so a new crate's copy needs a row of its own.
- **Citations in the changelog.** 85 lines of CHANGELOG.md hold a backticked span shaped as a
  reference with no angle bracket (`grep -E '\`[a-z]+@[^\`<]*\`' CHANGELOG.md | wc -l`), and line
  101 holds a backticked span of the path `path@knowledge-architect@.cargo/config.toml`, which
  `git ls-files .cargo` lists.
- **The pin move.** Step 2 reads
  `$CARGO_HOME/registry/src/<index>/knowledge-architect-<version>/CHANGELOG.md`, the binary
  crate's copy, and lists each Migration entry that cites an axis's section of
  `skill@knowledge-architect-project-audit` or
  `skill@knowledge-architect-goal-setting@whole-set-review`, per `design@agent-skills@audit-triggers`.
  The refusal of a binary at another version than the pin, in the function
  `refuse_another_version_built_at` of `path@check@src/commands/mod.rs`, line 304, tells the
  reader to "read the changelog of each version from {pinned} to {version}".
- **The crates.io pages.** Each of the 12 crates under crates/ since slice 2 holds a CRATES-IO.md
  (`ls crates/*/CRATES-IO.md | wc -l`), which `design@knowledge-architect@crates-io-page-file` says
  holds no reference, and which this repository declares at the level `all` since slice 1.
- **The installed copies.** They are outside the walk, as Names says. Their source, content/ and
  the snippets the build inlines, is in the walk, per
  `design@agent-skills@shipped-text-cites-no-entry`, so every reference there is checked against
  this repository: one to an entry of this repository resolves here and would dangle in an
  installing project. Since slice 1, the level `entries` judges that on the installed copies of the
  primer, the skills and the agents, which this repository declares at that level; the reviews and
  the hand check of `skill@klarch-release`, its step "The shipped text cites no entry", judge the
  saved workflows and each `path@*@<path>` the level admits.
- **Cargo.** Since slice 2, `cargo-reader` reads a `Cargo.toml` and runs and reads `cargo metadata`
  for the tools, per `design@knowledge-architect@tools-read-cargo-through-the-crate`. Its one caller
  is `declares` in `path@check@src/build_origin.rs`, which `foreign_copy` calls on each candidate
  directory: it reads the `Cargo.toml` with `std::fs::read_to_string` and compares the package name
  `read_manifest` reads with the linked library's package name, a compiled constant
  (`Library.package`). No crate calls `run_metadata` yet. The walk reads `md` and `rs` files alone.
- **Every crate under crates/ sets `repository.workspace = true`** (`grep -n repository
  crates/*/Cargo.toml`, 12 files since slice 2), and the workspace's `Cargo.toml` sets `repository`;
  tools/xtask, never published, sets none. `path@cli@Cargo.toml`, the package
  `knowledge-architect`, depends on neither `knowledge-architect-gates` nor
  `knowledge-architect-mutate`.
- **`query`.** Its lib.rs opens "Tools over a project's scanned record", and its `id` and
  `worklist` call `complete_working_tree` (`path@query@src/id.rs`, `path@query@src/worklist.rs`),
  so a finding of phases 1 to 3 stops both.
- **The release.** `agent@klarch-changelog-reviewer` judges the changelog at each release, per
  `design@knowledge-architect@changelog-reviewed-at-the-release`; `skill@klarch-release` runs `cargo
  x changelog` and checks the shipped text for references by hand.

Outside the work, each with what owns it:

- **The release procedure of a consuming project**, which the owner intends to ship later, R2: no
  plan document yet. This work ships none of it; the model and the `cargo-reader` crate are what it would
  build on.
- **How the crates are versioned**: `issue@knowledge-architect@every-crate-takes-a-version-at-each-release`,
  row 1 of docs/roadmap.md, on the owner's word at R7. This work keeps
  `design@knowledge-architect@version-lockstep`, and the `cargo-reader` crate takes the workspace version.
- **References in a rendered changelog**: `issue@knowledge-architect@the-documents-do-not-render-as-a-site`.
  The owner, R3, said a render site could make clickable references worth allowing; that changes
  one line of the manifest, not this design.
- **A check that reads a released section's content at its tag**: ruled out,
  `thread@per-crate-changelogs@released-content-check`.
- **A citation of one of this repository's own skills, `skill@<project skill>`, from the installed
  copies**, and **a guard on the placeholder spans of the shipped text**: the two parts of the
  closed issue `shipped-text-citing-an-entry-is-unchecked` that the `entries` level does not
  cover. Each is a later consequence of slice 1, under "Later consequences", which slice 1's
  landing turns into an issue entry or argues needs none.

## What is already decided

Kept, and not argued again:

- `design@knowledge-architect@version-lockstep`.
- `design@knowledge-architect@versioning-policy`: the classes and their meaning.
- `design@knowledge-architect@changelog-section-fixed-at-release`: a released section's content
  never changes, its structure may. The head leaves open what counts as structure; the owner chose
  among the readings it permits, R1 and R3: moving an entry into another file under another
  heading, and rewriting a span in words, are structure.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`.
- `design@knowledge-architect@changelog-reviewed-at-the-release`, for what no check asserts:
  whether a change owes an entry, its class, its category, its crate and the truth of its text.
- `design@check@check-reads-no-range-of-history`, widened by
  `thread@per-crate-changelogs@no-history-beyond-the-branch`.
- `design@knowledge-architect@model-crate`: phases 1 to 3 say whether the model can be trusted,
  which places every changelog finding in `check`; slice 3 adds to it the member that divides the
  changelog's work between `model` and `check`, as its spec says.
- `design@knowledge-architect@git-crate`: the precedent of the `cargo-reader` crate.
- `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` and
  `design@check@fix-before-the-checks`: the refusals of the assembled file's writer.
- `design@check@safe-fix-definition` and `design@check@fix-scope`: `--fix` writes the assembled
  file, a generated file, and nothing of a crate's changelog, which is hand-written: a preamble that
  differs is a finding the writer repairs, as slice 3's spec says.
- `design@agent-skills@plain-text-is-no-repair`: what "rewritten in words" means.
- The goals: `goal@serves-one-kind-of-project` (Rust: Cargo's manifest and semver),
  `goal@generic-tools-and-procedures-are-shipped`, `goal@documentation-has-a-structure-and-a-model`,
  `goal@documentation-stays-consistent`, `goal@every-target-project-can-adopt-it`.

Rejected entries the work changes, each at slice 5's harvest:

- `rejected@knowledge-architect@changelog-copies-written-at-packaging`: a rival of git-ignored
  copies, written at packaging, of the shared changelog. Its one citing head,
  `design@knowledge-architect@the-changelog-ships-in-every-crate`, is reversed, and no copy exists
  after slice 5. The harvest moves its citation to the successor head where the successor's
  argument covers the rival, or plans its leaving, as
  `skill@knowledge-architect-decision-recording` says of a reversal. It does not bear on the
  assembled file, which no `include` lists.
- `rejected@knowledge-architect@changelog-read-by-every-check`: narrowed to its first reason, that
  the changelog faces outward and holds no reference to real content, which the new checks keep by
  refusing every reference and resolving none. Its second reason, that a later change of the
  grammar would ask a released section for a repair that changes its content, is defeated by the
  owner's choice at R3, among the readings the head permits, that a rewrite of a span in words is a
  change of structure. It stays
  `live`.

Reversed or rewritten, with every text the change repairs and the slice that repairs it. The texts
are those `cargo klarch show` lists as referencing each decision at the commit that adds this
document, and those the record search's triage found:

| decision | change | texts repaired | slice |
| --- | --- | --- | --- |
| `design@knowledge-architect@the-changelog-ships-in-every-crate` | reversed: each crate ships its own changelog | `skill@klarch-release` (its `cargo x changelog` step), the root CLAUDE.md (`## Git` point 1, the CHANGELOG.md row of the knowledge table, the `xtask` row of the Component table), `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`, the preamble of `register@xtask@design`, `path@xtask@src/changelog.rs` (removed), the heads `design@knowledge-architect@changelog-section-fixed-at-release`, `design@knowledge-architect@package-include-whitelist` and `design@knowledge-architect@the-changelog-is-outside-the-walk` | 5 |
| `design@knowledge-architect@the-changelog-is-outside-the-walk` | rewritten under a new slug, `changelogs-hold-no-citation`, since the old one would name the rival: the changelog files are in the walk, read for structure, at the citation-free level `all`; the assembled file is outside the walk by construction. The owner, R3: the head was about its consequences, which this shape keeps | `agent@klarch-changelog-reviewer`, the preamble of `path@check@docs/design/reference-lints.md` ("CHANGELOG.md is read by none of the lints"), the `skip-files` rows and their comment in knowledge-architect.toml, `design@knowledge-architect@changelog-section-fixed-at-release`, each citation of the old slug re-pointed | 5 |
| `design@knowledge-architect@changelog-entries` | rewritten: the surface is replaced by the crate's file, the subsection by the category tag, and a Migration entry names an audit axis by its section title | `agent@klarch-changelog-reviewer`, `skill@klarch-release`, the root CLAUDE.md `## Git` point 1, `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@changelog-reviewed-at-the-release`, `design@agent-skills@audit-triggers`, `issue@agent-skills@a-one-time-technical-setup-has-no-place-a-project-gets-it-from` | 5 |
| `design@agent-skills@audit-triggers` | rewritten: the trigger is a Migration entry that names an axis by its section title, not one that cites it | the installed pin move's step 2 and step 5, `skill@knowledge-architect-project-audit` (its description and its occasions list), the primer's line on the project audit in `primer@installed-skills` and its `primer@coined-terms` ("Migration entry: an entry of the checker's changelog"), `design@agent-skills@tripwire-audit-axis` (judged: its wording "names this axis in its Migration entry" may already hold) | 5 |
| `design@knowledge-architect@crates-io-page-file` | rewritten: its rule is enforced by the citation-free level `all` | the setup skill's CRATES-IO.md proposal, in `skill@knowledge-architect-setup@rust-project`, which proposes the declaration | 1 |
| `design@check@installed-files-outside-the-walk`, `design@model@installed-entities-from-the-tree` | rewritten, narrowed, on D2, closed by the session on the owner's argument `argument@per-crate-changelogs@arg-60`, by the read that slice 1 records as a head of its own, `design@check@installed-copies-read-at-the-level-entries`, since it loses to a rival of its own: the installed copies stay outside the walk and raise no finding, except that the checker reads them for references at the level `entries` where a project declares a glob that matches them; the reason "reports those paths in a project that can repair none of them" keeps every other rule, since only a declaring project is read so | the doc comment of `installed` in `path@model@src/model.rs`, the module documentation of `path@model@src/shipped_set.rs`, `path@check@docs/design/run-model.md` ("An installed copy raises no finding of phase 3", judged at slice 1's design audit to stay true, since a finding of the level `entries` is phase 4's), `path@model@docs/design/agent-configuration.md`, `design@agent-skills@shipped-text-cites-no-entry`, `path@agent-skills@CLAUDE.md` ("content/ is in the walk"), `skill@klarch-release` step 4 | 1 |
| `design@model@a-reference-is-live-in-every-prose` | rewritten, narrowed: in a file at the level `all`, no span is resolved; at the level `entries`, a span is refused where it names an entry or an anchor of the project and takes the level's finding alone, and a span the level admits is resolved as anywhere else in a walked file | the citers `cargo klarch show` lists, read at the slice's design audit for the sentence "In every file the walk reads" | 1 |
| `design@model@git-supplies-the-walk` | rewritten, a member added: "No generated path is declared" admits the assembled path, the one generated path the manifest declares; `tree` does not assert it exists, since the head's premise, that a generated path cannot be asserted to exist, holds for it too, and the generated-files check reports it missing with its own finding, "the generated file is missing" (`MISSING` in `path@check@src/check/generated.rs`), or out of date | the citers `cargo klarch show` lists, read at the slice's design audit for the changed sentence, which a grep of "generated path is declared" finds in the head alone | 3 |
| `design@model@registers-are-declared`, `design@check@nothing-of-a-project-is-compiled-in` | rewritten, a member added: the set of changelog crates is declared in the manifest by `[changelog]`, opt-in, as "the published crates", whose list each `Cargo.toml` already declares; a second list by hand drifts, and the drift it would hide, a new published crate with no changelog, is what the check exists to report. The grammar and the classes are the sixth compiled-in member | the citers `cargo klarch show` lists, read at the slice's design audit for "the manifest declares every list a check reads" | 3 |
| `design@knowledge-architect@query-crate` | rewritten: its purpose widens to the record of the project or of one of its dependencies, its dependency line gains `cargo-reader`, and its reasoning over the default criteria gains the second reason to change, a dependency's changelog read through `cargo-reader`, argued at the slice's design audit; the placement of `klarch changelog` in `klarch` lands here or in a root head beside it | the `query` row of the root CLAUDE.md's Component table, `path@query@CLAUDE.md` line 3, `path@query@CRATES-IO.md`, the `description` of `path@query@Cargo.toml`, the crate documentation of `path@query@src/lib.rs` lines 1-2, the command list of `path@cli@README.md`, `path@query@README.md`'s first sentence | 4 |
| `design@agent-skills@a-reference-claims-a-revisit` | rewritten, a member added: a citation-free file owes no reference, by the argument of `thread@per-crate-changelogs@axis-named-in-words`: it is read in a consumer's registry where no reference resolves, a released section cannot follow a rename, and the release's review reads `Next release` | the citers `cargo klarch show` lists, read at the slice's design audit | 5 |

The heads each slice's harvest touches without reversing or rewriting them are listed in that
slice's spec. The lists are drawn from this search, run at the commit that adds this document:
`cargo klarch show` on each decision above and on each head a slice's harvest adds a member to,
keeping each citing head of a design home whose text the harvest changes; and a grep of every
design home for the changed wording, `grep -n -i "changelog\|CRATES-IO\|range of history\|history
before"` over docs/design.md, crates/\*/docs/design\* and tools/\*/docs/design\*. The search found
these citing or matching heads whose text no harvest changes, each judged so:
`design@knowledge-architect@capability-cost-before-migration`,
`design@knowledge-architect@owner-pushes-the-release-tag`,
`design@agent-skills@no-progress-record`, `design@agent-skills@knowledge-table-home`,
`design@agent-skills@code-claims-scope`, `design@model@section-homes-carry-slugs`,
`design@model@plan-document-directory`, `design@agent-skills@audit-repairs-the-whole-record`,
`design@agent-skills@scoped-auditor-on-touched-heads`, `design@agent-skills@existing-heads-on-touch`
and `design@agent-skills@tripwire-horizon`.

Entries the record search gave a verdict whose outcome changed the design, a thread or the work,
each with its outcome. Every other verdict is in [the discussion's search](search/discussion-1.txt).

| entry | outcome |
| --- | --- |
| `design@check@nothing-of-a-project-is-compiled-in` | the grammar and the classes are a sixth compiled-in member, `thread@per-crate-changelogs@changelog-grammar-compiled-in`; slice 3 |
| `design@model@components-carry-the-same-documents` | the checks run only where `[changelog]` is declared, `thread@per-crate-changelogs@changelog-opt-in`; reading `publish` is no discovery by filename |
| `design@model@registers-are-declared`, `design@check@nothing-of-a-project-is-compiled-in` | the set of changelog crates read from `publish` is a member of both, argued in "Reversed or rewritten"; slice 3 |
| `design@check@an-extension-claims-its-manifest-tables` | `[changelog]` becomes a built-in table, so the name `changelog` is taken from every extension; slice 3 lists it among the heads it touches. `[walk.citation-free]` is a subtable of the built-in `[walk]`, and an extension claims only top-level tables, so slice 1 does not touch the head, as its design audit found |
| `design@check@safe-fix-definition`, `design@check@fix-scope` | `--fix` writes no byte of a crate's changelog: a preamble that differs is a finding whose message gives the expected text; slice 3 |
| `design@model@a-reference-is-live-in-every-prose` | narrowed for the citation-free files, listed in "Reversed or rewritten"; slice 1 |
| `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` | answered by the opt-in: the work asks nothing of a project that declares no `[changelog]` |
| `design@check@the-regime-has-no-opt-out`, `rejected@check@exempt-directory-manifest-row` | both new declarations add obligations and remove none; the id lints keep reading citation-free files; slices 1 and 3 add the members |
| `design@agent-skills@plain-text-is-no-repair`, `design@check@reference-shape-outside-span`, `design@check@plain-word-lint`, `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` | a reference is rewritten in words or as an angle-bracket placeholder, never with its backticks removed; slices 1 and 5 |
| `design@check@unanchored-path-first-segment-here` | the lint stays in force in a citation-free file, where its repair is a rewrite in words; slice 1 |
| `design@agent-skills@a-reference-claims-a-revisit` | a member added on the argument of `thread@per-crate-changelogs@axis-named-in-words`, listed in "Reversed or rewritten"; slice 5 |
| `design@knowledge-architect@model-crate` | every changelog finding is in `check`, `thread@per-crate-changelogs@changelog-findings-in-check`; its "depends on `git` alone" is repaired, slice 3 |
| `design@check@fix-before-the-checks`, `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`, `design@create@init-asks-no-confirmation` | the assembled file's writer refuses, `thread@per-crate-changelogs@assembled-file-not-overwritten`; slice 3 |
| `design@model@git-supplies-the-walk`, `design@model@one-list-of-generated-files` | the assembled file is a declared generated path outside the walk by construction; the first head is rewritten with that member, as "Reversed or rewritten" argues, and the second gains it; slice 3 |
| `design@knowledge-architect@xtask-crate` | `cargo x changelog` is removed, slice 5 |
| `design@query@a-function-takes-a-configured-manifest`, `design@knowledge-architect@package-include-whitelist` | no crate ships knowledge-architect.toml, so the category order is written into each preamble, `thread@per-crate-changelogs@category-order-in-preamble`; slice 3 |
| `design@agent-skills@xtask-pins-checker` | the closure of `knowledge-architect` misses `gates` and `mutate`, so the command selects over the resolved graph; slice 4 |
| `issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head` | the command is placed in `klarch` on the argument of `thread@per-crate-changelogs@registry-changelog-command`; the general rule stays that issue's question, and slice 4's commit names the argument for that issue |
| `issue@git@a-shallow-clone-shortens-the-range-commits-judges` | the head of `thread@per-crate-changelogs@no-history-beyond-the-branch` admits asking git about the base; slice 1 |
| `design@knowledge-architect@query-crate` | rewritten, listed in "Reversed or rewritten"; slice 4 |
| `design@knowledge-architect@partial-publish-rerun` | the publish order it spells gains `cargo-reader` before each crate that depends on it: before `check` at slice 2, and before `model` and `query` at slices 3 and 4, which make them depend on it |
| `issue@knowledge-architect@every-crate-takes-a-version-at-each-release` | its trigger met by the `cargo-reader` crate; the owner turned it into a `todo` at row 1 of the roadmap, R7, in the commit that adds this document |
| the issue `shipped-text-citing-an-entry-is-unchecked` | scheduled by slice 1, whose `entries` level answers it on the owner's word at R7; closed by the review of this document, which moved its two other candidates to "Later consequences" and re-pointed its citations to this milestone |
| the issues `a-mechanical-changelog-check` and `the-changelog-holds-references-no-check-reads` | scheduled by slices 3 and 5; closed by the review of this document, their instances moved into `criterion@per-crate-changelogs@shape-checked-mechanically`, and their citations re-pointed to this milestone |
| `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment` | cited the two changelog issues; its sentence now names this milestone, re-pointed by the review of this document |
| `issue@knowledge-architect@the-publish-order-is-spelled-by-hand` | it points at `path@xtask@src/changelog.rs`, which slice 5 removes; slice 5 re-points it |
| `issue@model@manifest-rs-is-past-one-read`, `issue@check@history-rs-is-past-one-read`, `issue@check@commands-mod-rs-holds-two-parts-that-change-apart`, `issue@testing@binary-rs-is-past-one-read` | weighed in the spec of the slice that changes each file: slices 1 and 3, 3, 4 and 4 |
| `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`, `issue@agent-skills@a-touched-head-owes-its-older-bundles-in-the-change-that-touches-it` | obligations of this plan: each slice's list of touched heads, and the audit of each at the slice's design audit |

## Criteria

### A project moving its pin finds every entry it must act on, from the sources cargo fetched `##pin-move-finds-every-entry`

Binding. From `goal@every-target-project-can-adopt-it` and the head the work reverses,
`design@knowledge-architect@the-changelog-ships-in-every-crate`, R1. Met by
`thread@per-crate-changelogs@changelog-the-pin-reads`,
`thread@per-crate-changelogs@registry-changelog-command` and
`thread@per-crate-changelogs@category-order-in-preamble`.

### No released entry's content changes `##released-entries-keep-their-content`

Binding. From `design@knowledge-architect@changelog-section-fixed-at-release` and the owner's
framing at R1, that the conversion is a change of structure only. Met by
`thread@per-crate-changelogs@changelog-holds-no-citation`, the released spans rewritten in words or
as placeholders.

### A lower crate's consumer reads only that crate's entries `##lower-crate-reads-its-own-entries`

Weighed. The owner's proposal, R1. Met by `thread@per-crate-changelogs@changelog-per-component` and
the withdrawal of `thread@per-crate-changelogs@cross-crate-entries`.

### A branch writes its entries where it changes code, with no copy step to forget `##entries-written-where-the-code-changes`

Weighed. From `design@knowledge-architect@the-branch-writes-its-changelog-entries`, R1. Met by
`thread@per-crate-changelogs@changelog-per-component`; `cargo x changelog` is removed.

### The shape is checkable mechanically `##shape-checked-mechanically`

Binding once decided. From the owner's intent of a mechanical check of the changelog with a model
of its sections and entries, recorded until this milestone in the issue
`a-mechanical-changelog-check`, R1. Met by `thread@per-crate-changelogs@changelog-model-home`,
`thread@per-crate-changelogs@changelog-findings-in-check` and
`thread@per-crate-changelogs@changelog-bump-check`. The instances that issue recorded, which the
checks of slice 3 would have reported:

- a `checks` entry appended after a `manifest` entry in a Migration subsection, on the branch that
  fixed four issues before the 0.3.0 release, which every gate passed and two reviewers found
  (findings P2 and C2 of the retrospective of 2026-10-04); the order by surface leaves with the
  surface, and the grammar check of slice 3 is the guard of the shape that replaces it;
- the version of a release chosen against the highest class of its entries by a grep over the
  `Next release` section, counted again by three reviewers (P4 of the retrospective of the release
  of 0.9.0, 2026-11-01), which the bump check of slice 3 asserts; that retrospective's proposal,
  `cargo x changelog` printing the highest class and the version it implies, is answered by the
  bump check, a finding on every commit.

## New names, in one place

Every name below is new unless marked existing. A key's or a flag's spelling is the illustration's,
and the slice that builds it fixes it in its commit; the names of findings are the slices'.

```text
crates/cargo-reader/                  the Component `cargo-reader`, package knowledge-architect-cargo-reader   slice 2
[walk.citation-free] all = [<glob>]   knowledge-architect.toml; resolved in a module of its own      slice 1
                                      under crates/model/src/manifest/ or beside manifest.rs
[walk.citation-free] entries = [<glob>]   the same table                                             slice 1
crates/check/src/check/citation_free.rs   the findings of a citation-free file                     slice 1
installed_copy_citing_an_entry_is_a_finding   a test in a new file under crates/testing/tests/      slice 1
[changelog]                           knowledge-architect.toml; resolved in a module of its own      slice 3
  components = [<Component>]          the changelog crates, in the manifest's Component order
  assembled = "<path>"                the assembled file; default "CHANGELOG.md"
  assembled-url = "<url>"             optional; a link the preamble prints
  categories = [<category>]           the categories, in their assembled order
  unclassed = [<version>]             versions whose entries carry no category and no class
[changelog.category-components]       <category> = [<Component>]; a category absent: every Component
klarch changelog <package> --from <version> [--to <version>] [--package <name>]...   crates/cli, logic in crates/query   slice 4
crates/<dir>/CHANGELOG.md             existing paths, each a crate's changelog from slice 5 on
CHANGELOG.md                          existing path, the assembled file from slice 5 on
```

## Decided design

The threads whose shape more than one slice builds are below. Every other thread closed `approved`
or `argued` has its decided design in the spec of the slice that builds it, as "Implementation
sequence" maps them.

### One changelog per published crate

Decided by `thread@per-crate-changelogs@changelog-per-component`, approved.

- Each changelog crate holds a `CHANGELOG.md` in its directory, listed in its `include`, holding
  only that crate's entries: one section per version that has an entry in that crate, `## Next
  release` first, then each released version in descending order.
- An entry that concerns several crates is written in each crate whose consumers meet it, since
  every crate can be consumed on its own (the owner, R3). No source file holds entries of no crate,
  per `thread@per-crate-changelogs@cross-crate-entries`, withdrawn.
- The project's changelog is the assembled file, one file at one path, generated; no crate ships
  it, and no second copy of it exists. The owner, R2, objected to a second copy of a large file that
  grows without bound.
- Argument: `argument@per-crate-changelogs@arg-1`, `argument@per-crate-changelogs@arg-12`,
  `argument@per-crate-changelogs@arg-6`, `argument@per-crate-changelogs@arg-26`. Nearest rival: today's
  shared changelog, defeated by the owner's argument that a consuming project that keeps and
  publishes changelogs needs one per crate, R2, and by its size, 155049 bytes in 12 files.

### What a project moving its pin reads

Decided by `thread@per-crate-changelogs@changelog-the-pin-reads`, approved.

- The pin move runs `klarch changelog` over the registry sources, which cargo unpacked when it
  fetched the new version, and reads its output: the assembled layout of every crate of the
  project, for the versions crossed. Slice 4 builds the command and rewrites step 2.
- People read the assembled file at its path in the repository. No crate ships it, so the pin move
  never reads a network host other than the one cargo fetched from.
- Each crate's preamble says the file holds that crate's entries only, and names the assembled file
  in words, by the path `[changelog] assembled` configures, as "the changelog of the whole project
  is <assembled path> in its repository", with the link `assembled-url` gives where the manifest
  sets one. No URL is built from
  `repository`, and nothing requires a crate to set `repository` (R5, R6).
- In the release that converts this repository, `cli`'s changelog carries a Migration entry
  saying that the file now holds the binary's entries only, and to run `cargo klarch changelog
  knowledge-architect --from <old>`; from premortem cause 1. Slice 5 writes it.
- Argument: `argument@per-crate-changelogs@arg-3`, `argument@per-crate-changelogs@arg-13`,
  `argument@per-crate-changelogs@arg-18`, `argument@per-crate-changelogs@arg-25`. Nearest rival: the pin move reading the root
  file on GitHub, the owner's inclination at R2; defeated by a sandbox that reaches the registry
  and no other host, and by the URL that a GitHub address needs and a registry path does not (R2,
  R3). Rival shapes A and B, a crate shipping the assembled file, lost on the second copy.

### Where the changelog's model and its checks live

Decided by `thread@per-crate-changelogs@changelog-model-home`, approved, with
`thread@per-crate-changelogs@changelog-opt-in`, `thread@per-crate-changelogs@changelog-grammar-compiled-in`
and `thread@per-crate-changelogs@changelog-findings-in-check`, each built in slice 3.

- The parse of a crate's changelog and the rendering of the assembled layout live in `model`; the
  checks live in `check`; the facts read from Cargo live in `cargo-reader` (slice 2). `query`'s command
  calls the parse and the rendering of `model` (slice 4).
- The changelog crates are those `[changelog] components` lists, or, where it lists none, every
  Component whose `Cargo.toml` has a `[package]` table that Cargo would publish, as slice 3's spec
  defines it.
- The division of this work between `model`, every parse and rendering, raising no finding, and
  `check`, every finding, lands as a member of the root head `design@knowledge-architect@model-crate`,
  whose line between phases 1 to 3 and the checks it applies; its rival, structural findings raised
  in phases 1 to 3, lands in `register@knowledge-architect@rejected` only where a recording test
  admits it, and otherwise in that head's body. What is one Component's own stays in its home: the
  grammar in `model`'s, the bump, `include` and preamble checks in `check`'s.
- Consuming projects are expected to use it, R2. The owner, R2, expects consuming projects that
  maintain and publish such changelogs within weeks to months: a report of the owner.

## Mapping tables

The conversion's mapping, of every entry of the shared changelog to its place in a crate's
changelog, is the one total function the work needs, and lives in the spec of slice 5,
[the conversion](repository-conversion/README.md).

## Losing alternatives

Each with the thread it lost to and the fact that decided it.

- **Today's shared changelog**: lost to `thread@per-crate-changelogs@changelog-per-component`. A
  consuming project that keeps and publishes changelogs needs one per crate (the owner, R2); the
  shared file is 155049 bytes in 12 files and gives a lower crate's reader every crate's entries.
- **A root source file for entries of no single crate**, `thread@per-crate-changelogs@cross-crate-entries`:
  withdrawn by the session at R2, superseded by one entry in each crate concerned, under
  `thread@per-crate-changelogs@changelog-per-component`. A reader of one lower crate alone would
  never see an entry kept there; and the root's name, `knowledge-architect`, is the binary's
  package's, so its heading would collide with the binary's.
- **The binary crate shipping the assembled file as its `CHANGELOG.md` (A), or as a second file
  (B)**: lost to `thread@per-crate-changelogs@changelog-the-pin-reads`. Each is a second copy of a
  file that grows without bound (the owner, R2).
- **The pin move reading the assembled file on GitHub**: lost to
  `thread@per-crate-changelogs@changelog-the-pin-reads`. A sandbox may reach the registry and no
  other host; a GitHub address needs `repository` and a tag scheme, a registry path neither. The
  owner, R3, rejected network independence as the argument, since fetching the crate and running
  the model both need the network, and kept the sandbox argument.
- **The assembled file located by Component**: lost to
  `thread@per-crate-changelogs@assembled-changelog-location`. The root is no published crate here,
  and a project with one crate has its crate's file only; a path covers both, a Component neither.
- **Version › crate › category layout** (the owner's first proposal, R1): lost to
  `thread@per-crate-changelogs@assembled-changelog-layout`. Its Migration entries of one version
  scatter over every crate, where the pin move acts on them as one block.
- **Crates ordered alphabetically by package name**: lost to
  `thread@per-crate-changelogs@assembled-changelog-layout`. The manifest's order is a choice the
  project makes with no new option (the owner, R3).
- **An entry placed in the crate whose code changes**: lost to
  `thread@per-crate-changelogs@entry-owning-component`. It shows the reader the checker's internal
  layering, `model` against `check`.
- **Category headings in each crate's file**: lost to
  `thread@per-crate-changelogs@changelog-entry-categories`. A crate with three entries would carry
  three headings, and Workflow would exist in one file only.
- **Structural findings raised in phases 1 to 3**: lost to
  `thread@per-crate-changelogs@changelog-findings-in-check`. `design@knowledge-architect@model-crate`
  gives those phases whether the model can be trusted; and `id` and `worklist` of `query` call
  `complete_working_tree`, so one misspelt category would stop them.
- **The changelog checks on in every project with published crates**: lost to
  `thread@per-crate-changelogs@changelog-opt-in`. Every adopter with its own changelog convention
  would fail at its next pin move, against `goal@every-target-project-can-adopt-it`.
- **The classes declared in the manifest**: lost to
  `thread@per-crate-changelogs@changelog-grammar-compiled-in`. They are Cargo's semver, which
  `goal@serves-one-kind-of-project` fixes.
- **Reading Cargo in a module of `model`**: lost to `thread@per-crate-changelogs@cargo-crate`.
  `model` would gain a second reason to change, Cargo's formats, and `query` would read
  `cargo metadata` in a second Component.
- **Opening per-crate versioning in this work**: lost to `thread@per-crate-changelogs@cargo-crate`
  on the owner's word, R7: the question goes to its own discussion before the next release.
- **The registry command's selection by the dependency closure of the named package**: lost to
  `thread@per-crate-changelogs@registry-changelog-command`. Measured: `path@cli@Cargo.toml`
  depends on neither `gates` nor `mutate`, so the closure of `knowledge-architect` misses both.
- **A preamble URL built from `repository`, and `repository` required in a changelog crate**:
  dropped at R5 and R6, under `thread@per-crate-changelogs@changelog-the-pin-reads`. The URL
  assumed GitHub's `/blob/main/` layout, against
  `design@agent-skills@installed-text-requires-no-ci-and-no-pull-request`, and neither the preamble
  nor the selection needs the field. The owner's suggestion to require it, R4, rested on the
  session's statement that the design needed it, which was false (R5, R6).
- **A dedicated `Audit` category, or a structured trigger field, for audit triggers**: lost to
  `thread@per-crate-changelogs@axis-named-in-words`. An entry is both a Migration and a trigger,
  and a category is one; a trigger field is specific to this repository, so the generic grammar
  would carry a field one project uses.
- **One `citation-free` level that refuses every reference**: lost to
  `thread@per-crate-changelogs@citation-free-paths`, on the owner's word at R7. The installed copies
  may cite what every project holds, and that is useful.
- **The changelogs at the level `entries`, citing installed text**: considered by the owner, R7,
  and kept strict, `thread@per-crate-changelogs@changelog-holds-no-citation`: a released section is
  frozen and cannot follow a citation that later changes.
- **A check that a released section equals its content at the version's tag**: ruled out,
  `thread@per-crate-changelogs@released-content-check`. Its comparison with a tag reads history
  before the branch, against `thread@per-crate-changelogs@no-history-beyond-the-branch`; its rival
  that compares within the branch alone is defeated by the owner's argument that a released text
  may be reworded without a change of meaning (R2), which no comparison tells apart.
- **One spec on one branch**: lost to the milestone, R9. It would put the seven Components whose
  Rust source the slices change, `model`, `check`, `cargo-reader`, `testing`, `cli`, `query` and `xtask`,
  counted from the five specs' "Builds", and the conversion of a 1892-line file into one review, where a failure of the checks could not be told
  from a failure of the conversion.

## Readings

The work reads three external sources, each recorded where the slice that reads it says:

- **Cargo's semver compatibility**, the meaning of patch, minor and major, and that under 0.x a
  major or a minor change both bump the second number: read by `thread@per-crate-changelogs@changelog-bump-check`,
  recorded by slice 3 in a grounding file of `check`, per the root CLAUDE.md's knowledge row for a
  finding read outside the project.
- **Cargo's manifest format**: `[package]`, `publish`, `repository`, `include`, `version` and the
  workspace inheritance of `version.workspace = true`; read by slice 2, recorded in
  a grounding file of `cargo-reader`.
- **The output of `cargo metadata --format-version 1`**: the resolved graph, each package's
  `manifest_path` and `repository`, and whether `--locked --offline` write `Cargo.lock` or reach the
  network, which the discussion assumed and slice 4 measures; recorded in a grounding file of
  `cargo-reader`.

## Premortem

Assuming the work shipped and failed, the session named these causes at R7, before the owner's word
on the checkpoint table:

| # | cause | thread stressed | verdict |
| --- | --- | --- | --- |
| 1 | The first pin move across the conversion runs step 2 from the old installed setup skill, which step 3 replaces, so it reads the binary crate's file, now `cli`'s entries alone, and misses the Migration entries of `check`, `model` and `agent-skills` | `thread@per-crate-changelogs@changelog-the-pin-reads` | converted into a clause of the design: the Migration entry of `cli`'s changelog in the converting release, "What a project moving its pin reads" above; slice 5 |
| 2 | `--locked` fails after the pin's edit and before `cargo fetch` | `thread@per-crate-changelogs@registry-changelog-command` | a claim of slice 4: run in the order the skill gives, the command succeeds over a mock consumer |
| 3 | An entry is placed in the wrong crate, and that crate's consumers never see it | `thread@per-crate-changelogs@entry-owning-component` | named as a cause for slice 5's harvest to judge as a tripwire: no check tells a misplacement from a placement |
| 4 | A pin move misses an audit trigger worded otherwise than the axis's section title | `thread@per-crate-changelogs@axis-named-in-words` | named as a cause for slice 5's harvest to judge as a tripwire; it shaped the thread: the entry names the axis by its section title, verbatim |
| 5 | The level `entries` refuses a citation the installed copies hold on purpose | `thread@per-crate-changelogs@citation-free-paths` | claim 9 of slice 1: the current install passes the check |

## Acceptance criteria

The one acceptance criterion, on `thread@per-crate-changelogs@registry-changelog-command`, is
judged by slice 4 and lives in its spec, [the command](changelog-command/README.md).

## Implementation sequence

1. [Files declared free of citations](harvested/citation-free-check/README.md): `[walk.citation-free]` with
   its levels `all` and `entries`, applied to the 11 CRATES-IO.md files and to the installed copies;
   the root head that no work implements. Fails alone on a citation-free file's finding.
2. [The `cargo-reader` crate](cargo-library/README.md): the library that reads Cargo's manifest and runs
   `cargo metadata`, in lockstep, with no user yet but `check`'s read of a package name, moved into it. Fails alone on a fact it reads wrong
   from a fixture `Cargo.toml` or a fixture metadata output.
3. [The changelog model and its checks](changelog-checks/README.md): the parse and the rendering in
   `model`, the `[changelog]` table, and the checks in `check`, opt-in, tested over mock projects
   alone. Fails alone on a mock project's finding or its assembled file.
4. [`klarch changelog`](changelog-command/README.md): the command, the rewrite of the pin move's
   step 2, and the pin refusal's message. Fails alone on the command's output over a mock consumer.
5. [The conversion of this repository](repository-conversion/README.md): the per-crate files, the
   declaration, the generated root file, the removal of `cargo x changelog`, and the rewrites of the
   heads and texts listed above. Fails alone on this repository's own `cargo klarch check`.

Threads by the slice whose spec carries their decided design: slice 1,
`thread@per-crate-changelogs@citation-free-paths`, `thread@per-crate-changelogs@no-history-beyond-the-branch`,
`thread@per-crate-changelogs@released-content-check`; slice 2, `thread@per-crate-changelogs@cargo-crate`;
slice 3, `thread@per-crate-changelogs@changelog-entry-categories`,
`thread@per-crate-changelogs@changelog-bump-check`, `thread@per-crate-changelogs@absent-version-section`,
`thread@per-crate-changelogs@changelog-include-check`, `thread@per-crate-changelogs@assembled-changelog-location`,
`thread@per-crate-changelogs@assembled-changelog-layout`,
`thread@per-crate-changelogs@assembled-file-not-overwritten`,
`thread@per-crate-changelogs@category-order-in-preamble`, with the three threads of "Where the
changelog's model and its checks live"; slice 4, `thread@per-crate-changelogs@registry-changelog-command`;
slice 5, `thread@per-crate-changelogs@entry-owning-component`,
`thread@per-crate-changelogs@changelog-holds-no-citation`, `thread@per-crate-changelogs@axis-named-in-words`.
The milestone document carries `thread@per-crate-changelogs@changelog-per-component`,
`thread@per-crate-changelogs@changelog-the-pin-reads` and
`thread@per-crate-changelogs@changelog-model-home`, the last built by slice 3.

## Order rationale

- Slice 1 before slice 2: slice 1 depends on nothing, and it must land before slice 5, which
  declares the changelogs at the level `all`.
- Slice 2 before slice 3: the `publish` default of the changelog crates, the bump check's lockstep
  test and the `include` check read Cargo through `cargo-reader`.
- Slice 3 before slice 4: the command parses and renders with the model's functions.
- Slice 4 before slice 5: the conversion uses every mechanism, and its Migration entry for the
  pin move names the command.

## Defaults awaiting the owner

- **D1**, on `thread@per-crate-changelogs@changelog-the-pin-reads`, raised by the author at
  assembly. Slice 4 rewrites the installed pin move's step 2 to run `klarch changelog`, which reads
  per-crate changelogs; until slice 5 merges, the crates ship the shared changelog, which the
  model's parse does not read. A release cut between the two merges would ship a pin move that a
  consuming project cannot follow. Default: no release is cut between the merge of slice 4 and the
  merge of slice 5. The rival, moving the rewrite of step 2 into slice 5, changes the order the
  owner approved at R9, so it is not applied. **Ruled**, after the review of this document: the
  default kept, on the owner's word that a release happens only on the owner's initiative and
  approval, so no delivery of this constraint into `skill@klarch-release` is added.
- **D2**, on `thread@per-crate-changelogs@citation-free-paths`, raised by the review of this
  document. The level `entries` over the installed copies rested on the premise that they are in
  the walk, which is false: `design@check@installed-files-outside-the-walk` and
  `design@model@installed-entities-from-the-tree` keep them out, and no reference in them is read.
  Default A: the checker reads the installed copies for references at the level `entries` and for
  nothing else. Rival B: `entries` applied to crates/agent-skills/content/, with its `%%` comments
  exempt. **Closed A by the session**, on the owner's argument after the review of this document, in
  the message that opens "I don't think there is any real reason", which ruled no shape and left
  the weighing to the session: it is no ruling of the owner. The owner argued that no rule needs the installed text
  outside the walk for this check; that its one cost, a finding naming an installed file whose
  repair is in content/ and a re-install, is left to the agent's judgement, maybe with a reminder in
  the root CLAUDE.md, which the session chose to add in slice 1; the finding's message not adapted to this repository; and that
  B loses because it must handle the fragments and a `%%` exemption that applies in this
  repository alone. The session's addition: the reason of `design@check@installed-files-outside-the-walk`,
  that a finding would report those paths in a project that can repair none of them, keeps every
  other rule, since the level `entries` reads an installed copy only where a project declares a
  glob that matches it; so both heads are narrowed, not reversed, as "What is already decided" lists
  them, and the read is a head of its own, `design@check@installed-copies-read-at-the-level-entries`,
  since it loses to a rival of its own, B.
- **D3**, on `thread@per-crate-changelogs@citation-free-paths`, raised by slice 1's design audit.
  `setup-default-crates-io-page`, which slice 1's harvest touches, meets no
  entry test of `primer@design-heads`: it is no interface others consume, its one site is
  `skill@knowledge-architect-setup@rust-project`, which states the proposal with its reason, and it
  turns on no external behaviour. It was removed by the commit "The design homes lose the heads no entry
  test admits", and restored by the commit "[review] The last transcript review read the cleanup:
  test 4 asks the owner, and the owner's answers decide which heads stand on the owner's intent",
  under a test of the owner's intent that the current rules no longer hold, and
  it quotes the owner twice. Default: the head leaves `register@agent-skills@design`, and its
  citations in this milestone's documents are re-pointed to
  `skill@knowledge-architect-setup@rust-project`. Rival: the head stays, without its second
  paragraph, which holds the quotations. The default removes a head that rests on the owner's
  word, so the owner rules on it.
  **Ruled**, after the review before the merge of slice 1: the default kept, on the owner's
  approval.
- **D4**, on `thread@per-crate-changelogs@citation-free-paths`, raised by slice 1's design audit.
  `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` names as its re-entry point the next
  branch that takes in a tool that judges a project's quality, and slice 1 adds such a check, with
  the setup skill proposing its declaration. Default: the discussion is put off again, as it was
  when the gate `cross_clippy` met the same point; the slice builds as planned, the declaration
  opt-in, so it asks nothing of a project that declares nothing. Rival: hold that discussion before
  slice 1 merges, which could argue whether the setup skill's proposal is an ask.
  **Ruled**, after the review before the merge of slice 1: the default kept, on the owner's
  approval.
- **D5**, on `thread@per-crate-changelogs@citation-free-paths`, raised by slice 1's design audit.
  `issue@check@no-checked-form-cites-a-point-inside-a-section` names as its re-entry point the next
  change to `design@model@section-homes-carry-slugs`, which slice 1's harvest narrows by the member
  of the level `entries`. Default: the discussion that issue asks for is not held in this slice,
  whose change bears on no citation of a point inside a section, and the issue stays open at its
  re-entry point. Rival: hold that discussion before slice 1 merges.
  **Ruled**, after the review before the merge of slice 1: the default kept, on the owner's
  approval.
- **D6**, on `thread@per-crate-changelogs@citation-free-paths`, raised by slice 1's design audit.
  `design@model@every-path-names-its-anchor`, which slice 1's harvest touches and splits, is
  enforced by code of `model` alone: `path@model@src/resolver.rs` refuses `..`, computes the plan
  document's form and is the one Rust file that cites the head. Read one way,
  `skill@knowledge-architect-decision-recording@owning-component` places the head where the code
  that enforces it lives; read the other, the cut divides the decision, since `check` renders the
  finding. No record states which reading holds. Default: the head and the one its split creates
  move to `register@model@design`, beside `design@model@path-to-a-home-is-reported`. Rival: both
  stay in `register@check@design`. The same reading bears on `design@check@deepest-anchor-wins`,
  which the slice does not touch.
  **Ruled**, after the review before the merge of slice 1: the default kept, on the owner's
  approval.
- **D7**, on `thread@per-crate-changelogs@citation-free-paths`, raised by the review before the
  merge of slice 1. A glob of `[walk.citation-free]` refuses a `.` segment, where a declared
  `[walk]` path drops one, as `design@model@citation-free-glob-spelling` records: one rule for
  every segment a glob must not spell, over a respelling the drop would spare, which no goal
  ranks. The argument that a star beside an empty segment leaves its segment open covers the
  empty segment alone. Default, built: the `.` segment is refused with the empty one. Rival: a
  glob's `.` segment is dropped, as a `[walk]` path's is, and only the empty segment refused.
  **Ruled**, after the review before the merge of slice 1: the default kept, on the owner's
  approval.
- **D8**, on `thread@per-crate-changelogs@cargo-crate`, raised by slice 2's design audit. The issue
  `the-checker-may-depend-on-every-crate-below-it` named as its closing occasion the first plan that
  wants a crate `check` does not yet depend on, which slice 2 is, since `check` comes to depend on
  `cargo-reader`. Default: the discussion is put off, as for D4 and D5, and `design@knowledge-architect@check-crate` gains
  `cargo-reader` on its own argument. Rival: hold the discussion before slice 2 merges.
  **Ruled**, at slice 2's design audit: neither; the owner closed the issue, on the owner's word that
  a discussion of which crates `check` may depend on serves no purpose before a feature needs its
  answer, and arises with that feature's own design discussion.

## Harvest

Each slice's row is in its spec. The milestone document's own row:

| what | thread | home |
| --- | --- | --- |
| premortem causes 3 and 4 | `thread@per-crate-changelogs@entry-owning-component`, `thread@per-crate-changelogs@axis-named-in-words` | judged at slice 5's harvest by `skill@knowledge-architect-decision-recording@ruled-tripwires`, a tripwire of the root's tripwires home where admitted |
| the decided design of "One changelog per published crate" and "What a project moving its pin reads" | `thread@per-crate-changelogs@changelog-per-component`, `thread@per-crate-changelogs@changelog-the-pin-reads` | the heads slice 5 writes in the root design home, in place of `design@knowledge-architect@the-changelog-ships-in-every-crate` |
| the decided design of "Where the changelog's model and its checks live" | `thread@per-crate-changelogs@changelog-model-home` and its three threads | the division between `model` and `check`, a member of `design@knowledge-architect@model-crate`, with its rival in `register@knowledge-architect@rejected` where a recording test admits it and otherwise in that head's body; what is one Component's own, in the heads slice 3 writes in the design homes of `model` and `check` |
| the acceptance criterion | `thread@per-crate-changelogs@registry-changelog-command` | reported at slice 4's landing and at the deletion of this document; judged then as a recurring cause |

## Later consequences

- **The release procedure of a consuming project**, which the owner intends to ship later (R2),
  builds on the changelog's model, the bump check and the `cargo-reader` crate; its plan document says what
  of `skill@klarch-release` it generalises.
- **`issue@knowledge-architect@every-crate-takes-a-version-at-each-release`**, before the next
  release: a reversal of the lockstep changes which branch of the bump check runs, and gives each
  crate's changelog its own versions, which this work already supports.
- **`issue@knowledge-architect@the-documents-do-not-render-as-a-site`**: a rendered changelog that
  wants clickable references moves the changelogs from `all` to another level, one line of the
  manifest, and that discussion weighs the frozen released sections the owner weighed at R7.
- **`issue@knowledge-architect@the-publish-order-is-spelled-by-hand`** can read the published set
  and the dependencies through `cargo-reader`.
- **`issue@knowledge-architect@the-rule-of-one-program-for-the-tools-is-argued-by-no-head`** keeps
  its question; this work adds one instance placed in `klarch`, with its argument.
- **`issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`**: the
  level `entries` over the installed output is the check a future bundler crate reuses on the text
  it builds for another project. A premise that crate's design discussion weighs, not a decision of
  this milestone: the owner reported, after the review of this document, in the message that opens
  "About the bundler argument", a shape where a project bundles a skill only its consumers use,
  such as the usage instructions of a CLI tool it ships, and does not use the tool itself, so it
  holds no installed copy for the level `entries` to read. The owner's recommendation, a judgement of
  expected use open to challenge: such a project should install the skill locally anyway, since its tests would likely
  have agents use the tool under the skill's guidance.
- **A citation of one of this repository's own skills, `skill@<project skill>`, from the installed
  copies**, the third candidate of the closed issue `shipped-text-citing-an-entry-is-unchecked`:
  the level `entries` admits it, since it names no entry and no anchor, and the issue recorded as
  not established whether the checker's test, which holds each reference of the shipped set to
  resolve within the set, refuses it. Slice 1's landing opens an issue entry for it, or argues in
  its commit that none is needed, with the test's behaviour measured.
- **A guard on the placeholder spans of the shipped text**, the issue's second candidate shape,
  from the design discussion of the register kind, where checking every placeholder span of the
  tree was ruled out: the same check reads the spans holding an angle bracket in the shipped text,
  and reports one whose literal kind is no kind, whose segment count is not its kind's, or whose
  literal anchor is not one every installing project holds. The issue's census found no defect
  among 354 placeholder spans, so it is a guard on the text that teaches every installing project
  the grammar, not a repair. Slice 1's landing opens an issue entry for it, or argues in its commit
  that none is needed. Its design audit found it a design that
  `rejected@model@placeholder-literal-part-check` records, scoped to the shipped text: an issue entry
  for it needs a new argument against that entry.
