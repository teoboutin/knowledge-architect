# This repository is split into Components of one purpose each: `git`, `model`, `check`, `query`, `transcript`, `create`, `cli` and `testing`, with `bundle`, `render` and `rewrite` placed for later work

## Status and audience

This milestone document is the plan of the split of the core into eight Components, each a crate
of its own, and of the placement of three more that later work creates. It holds what crosses the
slices; each slice's spec holds what only that slice builds. It leaves when its last slice has
landed and every default it lists is ruled.

It comes from one design discussion, held in one session, which began at the owner's message "New
rules relative to how components and code in projects following knowledge-architect should be split
have just been added." The discussion applied `skill@knowledge-architect-component-creation` to this
repository, compared the result with the owner's own plan, and revised the skill; that revision,
planned in a spec of its own which the roadmap ordered before this milestone on the owner's word,
has landed in the installed text and in `register@agent-skills@design`.

- Written for a session that did not witness the discussion.
- Where this document or a slice's spec and a design home disagree on a point "What is already
  decided" does not list as reversed or rewritten, the design home wins, and the disagreement is a
  defect of the plan document.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. Its defaults D1, D2 and D3 are ruled; D4 awaits the owner.

## How the work is done

Read `skill@knowledge-architect-planning@working-a-slice` whole before each slice, then follow it.
This repository adds:

- `skill@klarch-development`, with `skill@knowledge-architect-development`, before each slice's code;
- **each new published crate owes, in the slice that creates it**: an `include` whitelist and the
  license copies, per `design@knowledge-architect@package-include-whitelist`; a CHANGELOG.md copy
  and its `skip-files` row, which `cargo x changelog` then writes, since it takes every directory
  under crates/ that holds a Cargo.toml, per
  `design@knowledge-architect@the-changelog-ships-in-every-crate`; a CRATES-IO.md, per
  `design@knowledge-architect@crates-io-page-file`; the checkout tie in its library root and build
  script, per `design@knowledge-architect@a-build-is-tied-to-its-checkout`, which
  `path@xtask@tests/checkout.rs` asserts; the version of the workspace, per
  `design@knowledge-architect@version-lockstep`; its place in the publish order, in
  `path@knowledge-architect@.github/workflows/release.yml` and in
  `design@knowledge-architect@partial-publish-rerun`; its directory and package name in the lists of
  `design@core@a-foreign-build-is-refused` and `design@core@checker-source-literals-are-data`; and
  a README and library documentation that state what an adopter of it does, per
  `goal@knowledge-architect@adoption-is-easy`, and the four conditions of a target project, the asks
  and the limit of the guarantee, per `goal@knowledge-architect@serves-one-kind-of-project`; a row of
  the Component table of the root `CLAUDE.md`; its place in the lists of `skill@klarch-release`; and
  the Component's documents and register homes, declared in the manifest with
  `cargo klarch new-component`;
- **each new Component's goals**, and the moves and rewordings of the core's goals a slice makes,
  are drafted by the session under `skill@knowledge-architect-goal-setting` and put to the owner;
  the slice is built on the drafts, and its branch merges once the owner has ruled on them, per
  `primer@question-authority`. #check-renamed sends each goal of `register@core@goal` to the
  Component that holds its subject, and drafts the rewording of each that names "the core", such as
  `goal@core@checks-a-commit-from-its-hook`, whose measure names "the core's tests", and
  `goal@core@projects-add-their-own-checks`, which names "no change to the core";
- each slice moves the design heads, `rejected` entries, issues and tripwires whose subject its new
  Component holds, per `skill@knowledge-architect-component-creation@creation-procedure` point 4,
  and re-points every reference `cargo klarch check` then reports;
- **a citation the checker does not read**, in a file that is neither Markdown nor Rust, is found
  by a search before each head a slice rewrites, `git grep -n '<slug>' -- ':!*.md' ':!*.rs'`. Five
  lines in four files stand at the branch that adds this document: `path@core@Cargo.toml` cites
  `design@core@the-core-cli-is-a-library-module`; `path@knowledge-architect@knowledge-architect.toml`
  cites `design@core@nothing-of-a-project-is-compiled-in` and
  `design@knowledge-architect@no-component-directory-named-after-the-project`;
  `path@knowledge-architect@Cargo.toml` cites the latter; and
  `path@knowledge-architect@.github/workflows/release.yml` cites
  `design@knowledge-architect@partial-publish-rerun`;
- **the maintenance tool follows each move**: `path@xtask@Cargo.toml` depends on the core by path,
  four of its modules import `knowledge_architect::MANIFEST_NAME`, and its test
  `path@xtask@tests/scaffold.rs` calls `cli::init` with `InitArgs`; the setup snippet the test
  compiles, `path@agent-skills@snippets/xtask-main.rs`, imports `knowledge_architect::MANIFEST_NAME`
  too. Each slice that moves or renames one of them updates xtask in the same change, and `cli`
  re-exports `MANIFEST_NAME` and `init` under the paths they have today, so the snippet a project
  copies is unchanged;
- **a change that sets what every commit's tree must pass goes in the slice's first commit**, per
  `primer@branch-commits`: this repository builds its checker from the tree, so a slice that adds
  a crate to the compiled lists of the checker's own directories, or renames the core, can make an
  earlier commit of its branch fail under the tip's checker. Each slice's audit states whether its
  first change does, and orders its commits so;
- **the revised rules for splitting a project into Components have landed before any slice**, as
  the roadmap ordered: the slices cite the code-shape fragment and
  `design@agent-skills@integrated-test-through-public-api`, and `spec@component-split@cli-on-top`
  writes the two heads of `register@agent-skills@design` that the component-creation skill's
  comment lines owe once a second text states the top-level binary crate and where a command is
  parsed.

## Names

- **R1 to R6**: the six rounds of the discussion, each one message of the owner and the reply;
  the owner's words quoted in this document carry their round.
- **the table, the checkpoint**: the session's tables of threads, the layout table of R1 and R2
  and the checkpoint of R5, which the owner's words "I agree with your table now" (R3) and "Agreed
  on the table" (R6) answer; each thread's item says which.
- **hard point 1 to 4**: the four places where the session found the component-creation skill hard
  to apply, R1, which the revision of the component-creation skill revises.
- **finding 1 to 10**: the ten numbered findings the session drew from the discussion's search of
  the whole record, R4.
- **B′**: the shape of #installed-text-check-home that won: the installed-file check moved to
  `check` as a check family of the last phase. A was the check kept in the foundation, B the check
  moved to `check` and kept in phase 2.
- **the port**: the Python tool the owner wrote, `git-history-tools`, ported to Rust as the
  `rewrite` Component; its leads are in
  `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`.
- **the core**: the Component `core`, crates/core, package `knowledge-architect`, today's checker
  library and binary. The slice #check-renamed renames it `check`. Before that slice, `check` in
  this document's tables names the core.
- **the writers**: the commands that write files over a model the foundation must pass first:
  `index`, `index --staged`, `check --fix`, `new-component` and `init`.
- **A1**: the acceptance criterion #a1-no-check-below-the-tools.
- **the foundation**: `check::foundation` in `path@core@src/check/mod.rs`, which runs phases 1 to 3
  of a run: the manifest's complaints, `check::tree` and `check::agents` (phase 2), and the entity
  table's definition findings (phase 3). `cli::complete_working_tree` in `path@core@src/cli/mod.rs`
  refuses over a model the foundation stops; `index`, `id`, `new-component` and `worklist` call it,
  `check --fix` calls `check::foundation` itself, and `index --staged` reaches the foundation
  through `commit_tree` of `path@core@src/cli/history.rs`.
- **the installed-file check**: `check::agents` in `path@elsewhere@crates/core/src/check/agents.rs`, which compares the
  installed agent files with the shipped text, byte for byte.
- **the resolver and the lints**: in `path@core@src/check/references.rs`, the functions that resolve
  a reference against the entity table (`table`, `harness`, `register`, `path`, `anchored_target`,
  `planned_target`) and report what fails, one function each today.
- **the mock tests**: the 36 `#[test]` functions of `path@core@src/mock_projects.rs`, a
  `#[cfg(test)]` module that runs the core over the mock projects of `path@core@tests/projects/`.
- **an integrated test**: a test that drives the project through its public API, as a consumer
  would, as the code-shape fragment, `path@agent-skills@fragments/default-code-shape.md`, defines it,
  per `design@agent-skills@integrated-test-through-public-api`.
- **P1, P2**: the two shapes of #command-parsing-home. P1: the `cli` Component parses every klarch
  command, the domain crates expose a typed API with no clap in it. P2: each domain crate derives
  clap's types for its own commands, and `cli` flattens them.
- **the phase-1 contribution**: what an extension gives phase 1 today, in
  `path@core@src/extension.rs`: the tables it claims, through `Extension::tables`; its complaints,
  the paths it declares and the files it generates, through `Extension::resolve`, which returns
  them as a `Resolution`; `extension::configure` turns the tables into complaints and merges the
  rest into the manifest.

## What the work is

What exists today, read at the branch that adds this document:

- **One crate holds the checker.** crates/core holds the library and the binary `klarch`, per
  `design@core@single-crate`; its `src/` holds 473 `#[test]` functions, 36 of them the mock tests,
  counted with `grep -rc '#\[test\]' crates/core/src | awk -F: '{s+=$2} END {print s}'` and
  `grep -c '#\[test\]' crates/core/src/mock_projects.rs`, so 437 are unit tests that use no mock
  project. Of its modules, `graph.rs` is imported by `cli/worklist.rs` alone, and `git.rs` and
  `transcript.rs` import nothing of the crate outside their tests.
- **The commands are one enum**, `cli::Command` in `path@core@src/cli/mod.rs`, which a binary that
  registers extensions flattens, per `design@core@the-core-cli-is-a-library-module`.
- **Every git invocation of the core's library code is in `path@elsewhere@crates/core/src/git.rs`**, per
  `design@git@git-is-asked-through-one-module`; a unit test of `path@elsewhere@crates/core/src/source/rs.rs` and
  the tests of `path@core@tests/init.rs` run git themselves, and so do `gates`, in
  `path@gates@src/gates.rs`, and `mutate`, in `path@mutate@src/harness.rs` and `path@mutate@src/run.rs`.
- **`id`, `worklist` and the writers run the foundation**, through `complete_working_tree` or
  `check::foundation` itself; `show` builds the model and runs no phase, and calls two functions of
  the resolver, `check::references::register` and `check::references::planned_target`. None of them
  runs phase 4.
- **The installed-file check is phase 2**, which the issue `installed-file-findings-belong-in-phase-four` recorded as
  misplaced.
- **The core's design home** holds 156 heads, counted with
  `grep -c '^### ' crates/core/docs/design/*.md | awk -F: '{s+=$2} END {print s}'`.
- **References naming the anchor `core`**: 1,475 backticked spans in 153 files on `origin/main`, the
  main branch this document's branch starts from, counted with
  ``git grep -o '`[a-z]*@core@[^`]*`' origin/main | wc -l`` and
  ``git grep -l '`[a-z]*@core@' origin/main | wc -l``; the plan documents add more, which
  #check-renamed re-points with the rest.
- **The commits that change the core's source**: of the 161 commits reachable from `origin/main`
  that change crates/core/src, 35 change the model's files and no file of crates/core/src/check/,
  20 change crates/core/src/check/ and none of the model's files, 44 change both, and 62 neither.
  The model's files are those the mapping table sends whole to `model`, and `git.rs`. Re-taken on
  any revision with:

  ```sh
  git log --format=%H <revision> -- crates/core/src | while read h; do
    f=$(git show --name-only --format= "$h" -- crates/core/src)
    m=$(echo "$f" | grep -cE 'src/(manifest|walk|survey|scan|document|entity|records|model|index|finding|git)\.rs|src/source/')
    c=$(echo "$f" | grep -c 'src/check/')
    echo "$m $c"
  done | awk '{ if ($1 && $2) b++; else if ($1) m++; else if ($2) c++; else n++ } END { print m, c, b, n }'
  ```

  The figures above are its output at `origin/main` when this document was written.

Outside this work, each with what owns it:

- `bundle`, `render` and `rewrite`, placed here and created by their own work:
  `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`,
  `issue@core@the-documents-do-not-render-as-a-site` and
  `issue@knowledge-architect@no-published-command-rewrites-a-branch-s-history`, each with the leads
  of this discussion, in the roadmap's order;
- `gates` and `mutate`, unchanged: moving them onto the `git` crate is possible and not required
  (#arg-19); `xtask` changes only to follow the moves, as "How the work is done" says;
- the revision of the component-creation skill and the code-shape fragment, which has landed;
- the first publication of each new crate, which the release workflow cannot do, per
  `design@knowledge-architect@publish-credential`: the release after the milestone, on the owner's
  word.

## What is already decided

**Decisions the split rests on, not argued again:**

- `goal@knowledge-architect@projects-are-split-into-components`,
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`,
  `goal@core@documents-render-as-a-linked-site`, `goal@core@projects-add-their-own-checks`,
  `goal@core@relocation-needs-only-goal-changes`, `goal@core@records-reach-their-reader`,
  `goal@core@document-rules-are-checked`, `goal@core@documents-stay-current`,
  `goal@core@checks-a-commit-from-its-hook`: the criteria below;
- `design@knowledge-architect@version-lockstep`: every crate keeps one version;
- `design@knowledge-architect@no-component-directory-named-after-the-project`, which already names
  "crates/core and crates/cli" for a future split, and its rival
  `rejected@knowledge-architect@crate-directory-named-after-project`;
- `design@core@model-then-checks`, `design@core@phases-gate-the-report` (its rule that a finding is
  classed by the phase that produces it), `design@model@one-entity-table`,
  `design@model@one-list-of-generated-files`, `design@core@anchors-listing-command`,
  `design@core@id-lookup-command`, `design@core@worklist-writes-and-verifies-the-reports`: each holds
  across the new boundaries;
- the git heads that move whole to `git`: `design@git@per-user-ignore-file-pinned`,
  `design@git@repository-variables-dropped`, `design@git@index-file-variable-kept`,
  `design@git@a-path-is-bytes`, `design@git@git-floor-for-commit-trees`; and
  `design@git@failed-git-is-an-error`, moved with its exit-code clause stated as the crate's error
  contract beside a citation of `design@core@exit-code-ladder`, as #git-out wrote it;
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
| `design@core@single-crate` | #cli-on-top, reversed, its successor in the root's design home and the head leaving the core's in the same change | `path@core@docs/design/transcripts.md` |
| `design@knowledge-architect@installed-text-crate` | #git-out, rewritten in a form each later slice keeps true, under this slug, since the head's earlier slug said two crates | none |
| `design@core@the-core-cli-is-a-library-module` | #cli-on-top, moved to `cli` under a new slug, since it no longer names the core | `path@agent-skills@docs/design/project-configuration.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/run-model.md`, `path@core@docs/design/transcripts.md`, `path@core@README.md`, `path@core@src/cli/create.rs`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `path@core@tests/binary.rs` |
| `design@core@arguments-parse-through-clap` | #cli-on-top | `path@core@docs/design/creating.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/manifest-table-exposes-the-toml-crate.md`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `register@xtask@design` |
| `design@core@transcript-command-in-core` | #consumers-out, its successor under a new slug in `transcript`, the part saying that the reader is a crate of its own that `cli` depends on in the root's design home | none |
| `design@core@api-facade` | #integrated-tests; each slice writes its crate's facade | `path@core@CLAUDE.md`, `path@core@docs/design/transcripts.md`, `path@core@docs/open-issues/an-extension-cannot-see-which-register-a-document-belongs-to.md`, `path@core@docs/open-issues/manifest-table-exposes-the-toml-crate.md`, `register@core@tripwire`, `path@core@src/lib.rs` |
| `design@core@phases-gate-the-report` | #model-out (the foundation), #check-renamed (the phase table) | `path@agent-skills@docs/open-issues/audit-axes-beyond-the-design-record.md`, `path@agent-skills@docs/open-issues/the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit.md`, `path@core@CLAUDE.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/entities-and-headings.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/registers.md`, `path@core@docs/design/run-model.md`, `path@core@docs/design/worklist.md`, `register@core@tripwire`, `path@core@README.md`, `path@core@src/cli/create.rs`, `path@core@src/cli/mod.rs`, `path@core@src/cli/worklist.rs`, `spec@plans@path-quickfixes` |
| `design@core@owned-namespace-check` | #check-renamed, split as its issue says | `path@knowledge-architect@CLAUDE.md`, `path@agent-skills@docs/design/project-configuration.md`, `register@agent-skills@design`, `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/design/shipped-set.md`, `path@agent-skills@docs/open-issues/patching-an-installed-skill.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/a-project-workflow-can-declare-an-installed-workflow-s-name.md`, `path@core@docs/open-issues/core-heads-bundle-decisions-with-many-citing-sites.md`, `register@core@rejected`, `path@core@README.md`, `path@core@src/agents.rs`, `path@elsewhere@crates/core/src/check/agents.rs`, `path@core@src/check/mod.rs`, `path@core@src/cli/history.rs`, `path@core@src/cli/mod.rs`, `path@elsewhere@crates/core/src/manifest.rs`, `path@core@src/mock_projects.rs`, `path@elsewhere@crates/core/src/survey.rs`, `path@elsewhere@crates/core/src/walk.rs`, `path@core@tests/binary.rs`, `register@knowledge-architect@design`, `path@knowledge-architect@docs/open-issues/the-skill-bundling-passes-are-tied-to-this-project-s-skills.md`, `spec@plans@path-quickfixes` |
| `design@core@fix-before-the-checks` | #check-renamed | `path@core@docs/design/run-model.md`, `path@core@README.md`, `path@core@src/cli/mod.rs`, `path@core@tests/binary.rs`, `spec@plans@path-quickfixes` |
| `design@core@model-then-checks` | #model-out | `path@core@docs/design/extensions-and-api.md`, `path@core@src/check/references.rs`, `path@core@src/cli/gathered.rs`, `path@core@src/extension.rs`, `spec@plans@path-quickfixes` |
| `design@git@git-is-asked-through-one-module` | #git-out | `path@core@CLAUDE.md`, `path@core@docs/design/creating.md` |
| `design@core@inputs-are-gathered` | #model-out, for the two gatherings; #integrated-tests, for its sentence on the mock tests | `register@core@rejected` |
| `design@core@checker-source-literals-are-data` | each slice that adds a crate; #cli-on-top for the list the binary hands over | `path@agent-config@skills/klarch-development/SKILL.md`, `path@core@CLAUDE.md`, `path@core@docs/design/prose.md`, `path@core@docs/design/run-model.md`, `register@core@rejected`, `register@core@tripwire`, `path@core@README.md`, `path@core@src/check/references.rs`, `path@core@src/check/registers.rs`, `path@core@src/cli/history.rs`, `path@core@src/cli/mod.rs`, `path@elsewhere@crates/core/src/entity.rs`, `path@core@src/graph.rs`, `path@elsewhere@crates/core/src/index.rs`, `path@core@src/lib.rs`, `path@core@src/main.rs`, `path@elsewhere@crates/core/src/model.rs`, `path@elsewhere@crates/core/src/records.rs`, `path@elsewhere@crates/core/src/scan.rs`, `path@elsewhere@crates/core/src/source/mod.rs`, `path@elsewhere@crates/core/src/source/rs.rs`, `path@core@tests/binary.rs` |
| `design@core@a-foreign-build-is-refused` | each slice that adds a crate; #cli-on-top | `path@agent-skills@src/lib.rs`, `path@core@CLAUDE.md`, `path@core@docs/design/extensions-and-api.md`, `path@core@docs/design/prose.md`, `path@core@docs/design/run-model.md`, `path@core@src/build_origin.rs`, `path@core@src/cli/mod.rs`, `path@core@src/main.rs`, `register@knowledge-architect@design` |
| `design@core@installed-binary-version-check` | #cli-on-top | `path@agent-skills@docs/design/project-configuration.md`, `path@agent-skills@docs/open-issues/co-development-through-a-cargo-patch.md`, `path@core@docs/design/agent-configuration.md`, `path@core@docs/design/commit-messages.md`, `path@core@docs/design/git-and-the-walk.md`, `path@core@docs/design/transcripts.md`, `path@core@src/cli/mod.rs`, `path@core@src/lib.rs`, `path@core@src/main.rs`, `path@elsewhere@crates/core/src/manifest.rs`, `path@core@tests/binary.rs`, `path@gates@docs/open-issues/the-gate-list-head-bundles-the-library-s-independence.md`, `register@knowledge-architect@design` |
| `design@knowledge-architect@partial-publish-rerun` | each slice that adds a published crate, for the publish order, with the release workflow's list found by the search above | `path@agent-config@skills/klarch-release/SKILL.md`, `register@knowledge-architect@design` |
| `design@knowledge-architect@mutation-command-host` | each slice that adds a published crate, for its sentence on the publish order | `path@agent-skills@docs/design/project-configuration.md`, `register@mutate@design`, `register@xtask@design` |
| `design@core@nothing-of-a-project-is-compiled-in` | #cli-on-top | `path@agent-skills@docs/open-issues/pointers-written-bare-to-clear-a-finding-reach-no-check.md`, `path@core@CLAUDE.md`, `path@model@docs/design/anchors.md`, `path@core@docs/design/creating.md`, `path@core@docs/design/plan-documents.md`, `path@core@docs/design/prose.md`, `path@core@README.md`, `path@core@src/cli/create.rs` |
| `design@gates@the-library-owns-the-flags` | #cli-on-top, for its clause "as it is of the core's", which turns false once `check` holds no clap type | none |
| `design@knowledge-architect@no-component-directory-named-after-the-project` | #check-renamed and #cli-on-top, for its instance, crates/core for the package `knowledge-architect`, and its cost sentence | `path@agent-skills@content/skills/component-creation/SKILL.md` |
| `design@mutate@report-not-gate` | #cli-on-top, for its clause that `design@core@exit-code-ladder` "binds the core's commands alone" | none |
| `design@core@exit-code-ladder` | #cli-on-top, which moves it to `cli`: it binds every command `cli` offers | as `cargo klarch show` lists it at that slice |
| `design@core@an-extension-plugs-in-through-phased-hooks` | #model-out | `path@core@docs/design/generated-files-and-fixes.md`, `path@core@docs/design/plan-documents.md`, `path@core@docs/design/run-model.md`, `path@core@docs/open-issues/citation-regime-over-any-pinned-text.md`, `path@core@README.md`, `path@core@src/check/mod.rs`, `path@elsewhere@crates/core/src/check/tree.rs`, `path@core@src/cli/mod.rs`, `path@core@src/extension.rs`, `spec@plans@path-quickfixes` |

The head on the installed text's crate turned false at #git-out, which adds a crate: "A user names
only `knowledge-architect`, and cargo fetches the other. The two are published together". #git-out
rewrote it, as `design@knowledge-architect@installed-text-crate`, in a form the later slices keep
true: the installed text lives in its own crate, which the crate holding the installed-file check depends on by path and by exact version, a user names
only `knowledge-architect`, cargo fetches every other crate, and all are published together at one
version. The bundling work rewrites it again. `design@core@transcript-command-in-core` is rewritten
at #consumers-out: its argument from `design@core@single-crate` falls with that head. Each row's
slice writes the rewrite into its harvest.

**The discussion's record search** graded 193 entries `bears` and 110 `may-bear`, for this milestone
and the spec of the revision of the rules together, counted in the files its check assembled, which
the tree does not hold. Those that bear on the split, with their outcome:

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
- `design@model@at-span-candidate-rule`: an anchor `git` makes a finding of a backticked span whose
  head is the word git, such as a git remote written in backticks; on `origin/main` the one such
  span is a string literal of the checker's own source, in `path@core@src/check/references.rs`,
  which is data, and these documents write the remote in plain text; the name `git` stays;
- `design@model@kind-names-refused-as-anchors`, `design@model@anchor-names-unique`,
  `design@model@components-carry-the-same-documents`, `design@agent-skills@goals-required`: every new
  name is free, and each Component carries its documents and a goal;
- `design@model@an-entity-belongs-to-its-anchor`, `design@core@every-path-names-its-anchor`,
  `design@agent-skills@a-reference-claims-a-revisit`: every moved entry's references are re-pointed,
  and heads that depend across the new Components cite each other;
- `issue@core@core-heads-bundle-decisions-with-many-citing-sites`: a move of a head is no touch of it,
  per `design@agent-skills@existing-heads-on-touch`; #check-renamed rewrites
  `design@core@owned-namespace-check` and splits it in two as the issue says, and the issue's other
  heads are split by the slice that rewrites one, if any does;
- the issue `installed-file-findings-belong-in-phase-four`: closed by the commit that adds this document, since #check-renamed schedules its work, which that slice's spec carries;
- `issue@core@manifest-table-exposes-the-toml-crate`, `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`,
  `tripwire@core@private-item-needed`: the facades of `model` and `check` meet them; nothing is
  decided here on them, and the tripwire is unchanged: an item an integrated test of this
  repository cannot reach is opened as a `todo`, by the guideline of
  `design@agent-skills@integrated-test-through-public-api`;
- the issues whose code moves, re-anchored by the slice that moves their files:
  `issue@model@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`,
  `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`,
  `issue@core@finding-texts-are-not-audited-for-a-needless-cause`,
  `issue@model@a-span-whose-head-shortens-a-kind-passes-the-check-silent`,
  `issue@model@a-span-with-an-empty-head-is-malformed-against-its-head`,
  `issue@model@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`,
  `issue@git@index-staged-clears-an-entrys-flags`,
  `issue@core@the-missing-directory-refusal-does-not-hold-the-no-loss-claim`,
  `issue@core@show-lists-no-subdocument-of-a-directory-home`,
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
  `issue@core@a-forked-subagent-s-first-line-is-an-unrecognised-type`,
  `issue@model@check-under-a-sparse-checkout-reports-every-file-outside-the-cone`,
  `issue@model@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`,
  `issue@core@the-tests-read-the-machine-s-git-configuration`,
  `issue@core@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
  `issue@core@a-component-states-at-least-one-goal`,
  `issue@model@a-kind-name-refusal-quotes-a-future-migration-reason`,
  `issue@model@the-walk-may-exclude-a-file-the-model-requires`,
  `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`; an issue whose
  sites stay on both sides, `issue@core@a-slug-finding-names-a-repair-blind-to-what-the-slug-resolves-to`
  and `issue@core@finding-texts-are-not-audited-for-a-needless-cause`, stays in the core with its
  references re-pointed, and `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`
  stays too, its subject returning to the core at #check-renamed;
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: P1's typed API offers data
  by a library function, which its open policy names; the policy stays open;
- `issue@knowledge-architect@this-repository-warns-on-none-of-the-documentation-lints`,
  `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`,
  `issue@agent-skills@the-upstream-set-omits-the-published-libraries`,
  `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: each new crate adds one more site
  these issues count; none is closed or decided here;
- `the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface`, an issue of agent-skills, closed by the design discussion that revised the rules on
  Components: a reversal's successor goes to the Component that the rule on the owning Component
  gives it, so the successor of `design@core@single-crate`, a decision about which Components exist
  and how they depend on each other, is written in the root's design home at #cli-on-top, and
  `design@core@single-crate` leaves the core's design home in the same change;
- `design@mutate@report-not-gate`: its clause naming "the core's commands" is rewritten at
  #cli-on-top, per the table above;
- `design@xtask@one-module-per-subcommand`, `design@xtask@view-included-not-exported`,
  `rejected@xtask@source-view-exported-from-the-library`, `rejected@xtask@source-view-in-the-checker`:
  `MANIFEST_NAME` moves to `model`; the source view is the bundling work's, unchanged here.

The other entries the search graded `bears`, each read whole in the search's assembly, with its
outcome:

- `goal@knowledge-architect@adoption-is-easy`: each new crate's README and library documentation
  state what an adopter of it does, an obligation of the slice that creates it;
- `goal@knowledge-architect@documentation-stays-consistent`, `goal@knowledge-architect@structure-and-workflow-work-together`:
  every guarantee of the check and both listings stay computed by `klarch`, which `cli` keeps; each
  slice's claims that verdicts and outputs are unchanged hold them;
- `goal@gates@gates-from-a-list`, `design@gates@gates-crate`: `gates` keeps its own flags and its
  library, and `cli` flattens no command of it;
- `goal@xtask@repeated-tasks-are-automated`, `issue@xtask@the-reword-the-fold-and-the-merge-predicate-have-no-command`:
  the rewriting ships in a published crate, by its own work, after this milestone;
- `goal@knowledge-architect@projects-are-split-into-components`,
  `issue@core@the-core-s-purpose-leaves-the-rendered-site-outside-it`: each new Component's goals
  home states its purpose, in the goals each slice drafts for the owner;
- `design@agent-skills@content-mirrors-the-install-layout`, `design@agent-skills@one-placeholder-reader-per-kind`,
  `design@agent-skills@saved-workflow-name-is-its-file-stem`, `design@agent-skills@shipped-text-line-comments`,
  `design@xtask@source-view-in-the-tool`, `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`:
  the bundling work's; this milestone leaves the build passes in `agent-skills`;
- `design@agent-skills@rust-templates-one-home`: `init --rust` moves to `create` at #consumers-out,
  and its test, which builds a project from the snippets, moves with it; the snippets stay in
  `agent-skills`;
- `design@knowledge-architect@binary-bundles-workflow`: the binary in `cli` still carries the
  installed text, through `check`'s dependency on `agent-skills`;
- `design@agent-skills@installed-fix-goes-upstream`: its set names "the checker", which the split
  turns into several crates; `issue@agent-skills@the-upstream-set-omits-the-published-libraries`,
  listed above, already holds the question;
- `design@agent-skills@rejected-citation-direction`, `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`:
  they govern each slice's harvest of a losing alternative;
- `design@agent-skills@audit-net-growth`: the edits this milestone makes are of heads and issues,
  which add no installed instruction; nothing to count;
- `design@agent-skills@documentation-presence-is-linted`: the new crates are under the same ask
  `issue@knowledge-architect@this-repository-warns-on-none-of-the-documentation-lints` records this
  repository not following; nothing is decided here;
- `design@core@orphan-rejected-entry`, `design@core@heading-register-two-shapes`: their text names
  files of `path@core@src/check/` that #model-out cuts; that slice re-points each sentence that
  names a moved file;
- `design@core@index-staged-write`, `design@core@check-fix-flag`: `index`, `index --staged` and
  `--fix` stay in `check`; the list of generated files they share lives in `model`, per
  `design@model@one-list-of-generated-files`;
- `design@model@git-supplies-the-walk`, `design@model@links-are-no-documents`: the walk's meaning stays
  with the walk, in `model` from #model-out, and the listing it reads moves to `git`;
  `tripwire@model@walked-count-differs-between-machines` stays with the walk;
- `design@core@staged-tree-source`: the assembly of the staged tree stays with `commits` in `check`,
  and its git invocations move to `git`;
- `design@core@an-extension-reads-a-snapshot`, `design@core@an-extension-builds-its-own-model`:
  `extension::Snapshot` stays in the core, over the `Source` and `SnapshotEntry` of `git`, and the
  parse moves to `model`; the extension reads both through the facades;
- `design@core@init-command`, `design@core@init-runs-git-init`, `design@core@component-command`,
  `design@core@init-has-no-extension-hook`: `create`'s heads, moved at #consumers-out;
- `design@model@rust-parser-keeps-every-comment`: moves with the parsing to `model`; its sentence
  naming the core's Cargo.toml is re-pointed there;
- `rejected@core@sentinels-accepted-everywhere`: its doubt is the subject of #integrated-tests, where
  the fixtures move out of the core's compiled directory; that slice's audit reads it.

The entries the search graded `may-bear` that no sentence above or in the spec of the revision of
the rules names, each with its outcome:

- `design@agent-skills@checker-model-in-primer`: not read; the searcher's reason, "cites the seed version-lockstep, which the work keeps ('every crate keeps one version'); the head also names the core test the_primer_s_model_section_names_every_kind_and_reserved_word as a drift bound, and the testing (cut)"
- `design@agent-skills@branch-commits-judged-at-tip`: not read; the searcher's reason, "renaming core to check and moving heads into new Components re-anchors every design@core@... reference, which the tip's checker requires of every tree; the head's sentence 'a change that sets what the tip's checker (cut)"
- `design@agent-skills@gates-convention`: not read; the searcher's reason, "cites the seed gates-crate; the work leaves gates unchanged and the cli crate flattens only the checker's crates' commands, so no element meets 'a command of the maintenance crate run by the published gates"
- `design@agent-skills@maintenance-crate-ask`: not read; the searcher's reason, "the head names klarch init --rust and 'the setup skill's blocks' for writing the maintenance crate; the create crate takes init --rust, and a moved library entry point (the cli crate) would change the dependency those (cut)"
- `goal@knowledge-architect@setup-brings-quality-tools`: not read; the searcher's reason, "the create crate holds init --rust, which may set up the quality tools the goal lists; the work moves it without saying what it sets"
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`: not read; the searcher's reason, "section 2's 'a Component takes decisions about approximately one main subject' bears on which Component owns a head, which the issue says owning-component settles; the split will also move heads between Components; the (cut)"
- `issue@agent-skills@co-development-through-a-cargo-patch`: not read; the searcher's reason, "the issue rests on 'accepts a binary only when its version equals the core library's'; the split moves that library and publishes many crates at one version, which could change what a patched build must match; the work (cut)"
- `issue@agent-skills@patching-an-installed-skill`: not read; the searcher's reason, "the bundle crate takes the install-time rendering, where the issue's lead 'a patch stored as a diff ... applied by the install and checked by the check' would land; the work does not propose"
- `design@agent-skills@design-record-axis`: not read; the searcher's reason, "the axis judges 'the Component it sits in' by the questions of owning-component; section 2's 'a Component takes decisions about approximately one main subject' may change what those questions find; the work names no (cut)"
- `issue@agent-skills@audit-axes-beyond-the-design-record`: not read; the searcher's reason, "cites the seed phases-gate-the-report as a head that 'look[s] like a goal left unnamed'; the split moves phases 1 to 3 into model and leaves phase 4 in check, and the new Components' goals could name it; the issue's (cut)"
- `design@agent-skills@audit-outcomes`: not read; the searcher's reason, "Section 1 moves core's design heads into new Components' homes; the head sends 'a head in the wrong Component' to the owner in an audit, not in a split, so no relation is"
- `issue@agent-skills@heads-argued-from-another-harness`: not read; the searcher's reason, "its re-entry includes 'one that changes how the shipped fragments are delivered', and design@agent-skills@shared-text-fragment is an instance; moving render_fragments into bundle relocates the pass but does not by (cut)"
- `design@mutate@mutation-command`: not read; the searcher's reason, "Seed design@mutate@mutation-command; the work keeps mutate unchanged and only names it as deriving clap Args, which this head's rule (the caller writes the mutation, the tool generates none) does not"
- `issue@agent-skills@a-lighter-pass-for-untouched-components`: not read; the searcher's reason, "item 1 adds about ten Components to this repository, which makes the issue's cost case ('a project of many Components that change apart') hold here; its re-entry points (a change to the record search, the owner's (cut)"
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: not read; the searcher's reason, "the owner's shape in its Re-entry records watched behaviours 'bundled in the agent skill crate under a command'. Subject 1 splits agent-skills into bundle and content, which would change where such a command lives. The (cut)"
- `issue@gates@the-gate-list-head-bundles-the-library-s-independence`: not read; the searcher's reason, "item 2's 'usable alone by others argues for a split' meets the unargued gates independence (the assumed reason: a published library carries no project's assumptions), and a new git crate could be a dependency of gates; (cut)"
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: not read; the searcher's reason, "a candidate fix changes the seed's summary printed by check --staged, and commits moves to check; the split proposes no change to what a run prints, so no bearing is"
- `tripwire@core@a-second-producer-chain`: not read; the searcher's reason, "its Fires clause and horizon name path@core@src/check/, which the move to a check crate relocates; the split adds no check reading another's findings, so it does not"
- `tripwire@core@phases-gate-the-report-two`: not read; the searcher's reason, "its re-entry is 'any change to the arguments of check, index', which the cli flattening and the clap-placement question touch; no flag letting a writer write over an incomplete model is proposed, so it does not"
- `design@knowledge-architect@changelog-reviewed-at-the-release`: not read; the searcher's reason, "tripwire@agent-skills@record-search-misses records that a previous discussion on the same development and component skills rested on this head unseen; no element of this work touches the release review of entries, so no (cut)"
- `issue@model@a-kind-name-refusal-quotes-a-future-migration-reason`: not read; the searcher's reason, "Part 1 names new Components (git, bundle, model, check, query, render, transcript, rewrite, create, cli, testing); the head this issue questions refuses every kind's name as an anchor. None of the drafted names is a (cut)"
- `issue@knowledge-architect@a-mechanical-changelog-check`: not read; the searcher's reason, "its 'surface from a closed list' may grow with new crates; the work proposes no changelog check, so no bearing is"
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: not read; the searcher's reason, "the split moves the checker's code into new crates (model/check) and leaves open moving the phase-2 byte check of installed files; the head's exception, 'a plan document lands before any work that changes what the (cut)"
- `design@agent-skills@content-is-in-the-walk`: not read; the searcher's reason, "cites the seed design@agent-skills@shipped-text-line-comments; the bundle split moves the passes but content/ stays in agent-skills, so 'putting content/ back in the walk' is unchanged unless a project reusing bundle (cut)"
- `rejected@agent-skills@published-checker-reads-percent-lines`: not read; the searcher's reason, "the bundle crate makes the %% pass reusable by other projects, and the split of model/check reshapes the walk; a session could then propose that the checker read the %% lines of a reusing project's sources, which this (cut)"
- `issue@agent-skills@a-comment-line-may-copy-the-argument-of-the-head-it-cites`: not read; the searcher's reason, "cites the seed design@agent-skills@shipped-text-line-comments; the revisions of subjects 2 and 3 will write %% lines beside the new guidelines, which could copy a head's argument, but no element of the work decides what (cut)"
- `design@core@retired-slug-reference-reported`: not read; the searcher's reason, "part 1's new anchor names (git, cli, model...): the head reports a backticked <word>#<id> 'also when its word is an anchor'; a span like git#<id> would become a finding, but none is shown to"
- `design@core@reference-shape-outside-span`: not read; the searcher's reason, "part 1 splits the checker into model, check, query... crates; the head reads 'an unbound string literal outside the checker's own source', and which crates count as the checker's own source after the split (model.rs (cut)"
- `design@model@section-homes-carry-slugs`: not read; the searcher's reason, "part 1 (bundle crate, testing Component, tests through the public API) moves tests; the head says 'The shipped set is held to the rule ... by a test of the core over the shipped text'; part 2 may rename sections of the (cut)"
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: not read; the searcher's reason, "part 2 rewrites the split signals list of the component-creation skill and the design skill's when-to-split section; the issue says citations of a numbered point stay in words and 'renumbering a list ... silently (cut)"
- `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids`: not read; the searcher's reason, "the quick-fixes spec changes the repairs in src/check/references.rs and adds check --fix respellings, both of which part 1 places (lints and --fix in check, resolution in model); the question is about the spec against (cut)"
- `design@agent-skills@a-past-sentence-is-rewritten`: not read; the searcher's reason, "part 1 renames core to check and relocates heads, renaming every @core@ reference; the head rewrites a sentence about the past whose reference dangles; whether any such sentence is met is not"
- `design@agent-skills@plain-text-is-no-repair`: not read; the searcher's reason, "part 1's rename and relocations dangle many references; the head forbids clearing them as plain text; no conflict established beyond the repair's"
- `design@agent-skills@transcript-extraction-is-the-checker-s`: not read; the searcher's reason, "the transcript crate moves the command out of core; the head says the reviewer extracts with 'the checker's transcript show' (klarch transcript show --from --full). The cli crate keeps the binary klarch and flattens (cut)"
- `design@agent-skills@code-claims-scope`: not read; the searcher's reason, "the split makes new public interfaces that others consume (domain crates exposing 'a typed, clap-free API', a published testing crate). For any public interface a diff changes, this reviewer asks whether 'its (cut)"
- `issue@knowledge-architect@the-workflow-has-no-skill-for-branch-work-or-releasing`: not read; the searcher's reason, "the rewrite subcommands are the commands that a future branch-work skill would name ('rebasing, rewording a commit's message, folding a repair'). The work writes no such skill, so no bearing is"
- `design@model@planned-path-form`: not read; the searcher's reason, "a plan document for the split would name crates/<new>/ before they exist; the entry says a planned path follows 'the named anchor, the deepest anchor' rules, and the anchor a new Component's path names does not exist (cut)"
- `design@model@ignored-targets-are-not-asserted`: not read; the searcher's reason, "cites seed design@model@git-supplies-the-walk; resolution 'asks git check-ignore', a call that would go through the git crate from the model crate's reference resolution; the split moves the call, no rule of the entry (cut)"
- `design@core@links-are-navigation-rows`: not read; the searcher's reason, "the render crate turns references into links in a rendered site; the entry says a relative link 'is legal there and reported everywhere else' and names check::registers for the inverse assertion; rendered output is (cut)"
- `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves`: not read; the searcher's reason, "renaming crates/core and adding crates/<new>/ Components meets the entry's 'Such a reference names the old location after a move'; its grep finds no path@knowledge-architect@crates/<x>/ reference today, so it bears only (cut)"
- `tripwire@model@planned-path-outside-plans`: not read; the searcher's reason, "the split creates crates/<new>/ directories; the clause fires on a path@elsewhere@<path> outside plans, in a sentence saying the file was to be created, that now resolves; no such reference to a new crate is known in (cut)"
- `design@core@agents-table`: not read; the searcher's reason, "the testing crate takes the mock projects; the entry says 'The empty list serves ... the checker's mock projects'; moving them keeps the use, no conflict"
- `rejected@core@install-record-file`: not read; the searcher's reason, "a reusable bundle that generalises the install for other projects must decide how ownership is known; the entry's rival 'a committed file of its own format to know which installed files are its own' is a plausible (cut)"
- `issue@core@a-home-for-developer-contracts-outside-agent-configuration`: not read; the searcher's reason, "each new Component of part 1 owes a scoped CLAUDE.md, which is the home for developer contracts that this issue intends to move away from agent configuration; the split splits core's scoped CLAUDE.md, but no element (cut)"
- `issue@core@a-project-workflow-can-declare-an-installed-workflow-s-name`: not read; the searcher's reason, "check_workflow_name in render.rs, which the issue names, moves to bundle; the issue's fix is a check reading project workflows' declared names, which the move does not"
- `design@agent-skills@unrecognised-entries-reach-upstream`: not read; the searcher's reason, "The transcript crate takes the transcript reader and its command. The head's route starts at the command's header line asking for unrecognised lines to be reported. Nothing in the work changes that header, so no bearing (cut)"
- `issue@core@no-command-counts-a-tripwire-s-horizon`: not read; the searcher's reason, "The query crate is to hold the read-only commands, 'later keyword search and audit worklists', and git the history readers. The counting command this issue asks for would read the history through git and could land in (cut)"
- `design@agent-skills@root-cause-repair`: not read; the searcher's reason, "parts 2 and 3 add guidelines to the component-creation, design and development skills; the head says 'A repair adds an instruction only where none of these closes the cause, and says why' and names the admission (cut)"
- `design@agent-skills@repairs-read-against-the-record`: not read; the searcher's reason, "the head states for 'every actor that writes or judges agent-facing text' that an edit is read against the goals, heads and rejected entries it touches before the owner rules; parts 2 and 3 edit installed skills, but (cut)"
- `design@model@an-entry-is-a-heading-at-the-register-level`: not read; the searcher's reason, "the head says 'Which homes an anchor must carry is check::tree's question, not the table's'; the model crate takes phase 2 (tree) and the table, which renames check::tree, but the split of duties between tree and table (cut)"
- `design@model@headings-open-with-hash-marks`: not read; the searcher's reason, "cites the seed phases-gate-the-report; its phase-2 heading report moves with phases 1 to 3 into the model crate, and no rule of the head"
- `design@model@an-unresolved-reference-is-reported`: not read; the searcher's reason, "part 1 separates 'reference resolution ... from the reference lints' between model and check; the head gives each of five resolution failures its repair; whether these findings sit with resolution (model) or with the (cut)"
- `design@agent-skills@fork-choice-in-weak-form`: not read; the searcher's reason, "The cli shape reverses the seed single-crate decision and others; where a seed head records a fork's choice, it reopens only on an argument its record does not hold. Which seed heads are fork choices is not established (cut)"
- `design@agent-skills@owner-rules-at-a-fork`: not read; the searcher's reason, "The work's open threads (argument parsing in domain crates vs the cli crate; phase-2 check in model or checker) close on the owner's word only if they are forks; whether either has one shape standing is not yet"
- `design@agent-skills@one-record-searcher`: not read; the searcher's reason, "The query shape moves worklist and graph.rs into a new crate and plans 'audit worklists' there; the head names klarch worklist --out/--verify as its interface. A move keeps the command; no change to its behaviour is (cut)"
- `design@core@generated-files-are-pure`: not read; the searcher's reason, "The split moves the generated check out of core into a check crate and the index into model; the head names path@core@src/check/generated.rs as the site that verifies by regenerating, so its pointer would move; no (cut)"
- `design@core@fix-refusal-mixed-state`: not read; the searcher's reason, "The refusal computes the staged side through git snapshots (to the git crate) and phases 1 to 3 (to model), and 'any other failure to compute the staged side refuses ... such as an extension that cannot prepare over a (cut)"
- `design@core@safe-fix-definition`: not read; the searcher's reason, "bundle takes the install-time rendering; the head requires a fix's bytes be determined by the tree and the pinned version; with every crate kept at one version this seems to hold, so no conflict is"
- `design@core@record-is-every-register-entry`: not read; the searcher's reason, "worklist moves to query and reference resolution is separated from the reference lints in model; the head says 'the edges are the references the checker resolves', so worklist must take edges from model's resolution; no (cut)"
- `design@core@graded-entries-assembled`: not read; the searcher's reason, "The snapshot is 'computed by show's own code'; the draft keeps show and worklist together in query, so this seems to hold; no conflict"
- `design@model@tracked-and-ignored-is-a-finding`: not read; the searcher's reason, "cites the seed phases-gate-the-report; the model crate takes phases 1 to 3, check::tree included, and 'the run stops there, as for every phase-2 finding' must still gate check's phase 4; no conflict"
- `rejected@git@every-git-variable-dropped`: not read; the searcher's reason, "the git crate's Invocation builder and its later history-rewriting write side could reach for a clean environment; the entry lost on GIT_INDEX_FILE under a hook; no proposal of it is"
- `design@model@a-placed-name-reads-as-nothing-else`: not read; the searcher's reason, "new Component names such as check or testing would make a later milestone or spec of that name a phase-2 finding; no current placed name is known to"
- `design@core@init-asks-no-confirmation`: not read; the searcher's reason, "cites the seed design@core@arguments-parse-through-clap ('The checker's binaries run blind'); the open question of the cli crate, whether argument parsing lives in each domain crate or in cli, could reach that ground, (cut)"
- `design@core@component-manifest-edit`: not read; the searcher's reason, "new-component 'parses the edited text with Manifest::parse'; the draft moves the manifest into model and the command into create, which needs Manifest::parse exposed across crates; no conflict with the head's rule (cut)"
- `design@core@rust-scaffold`: not read; the searcher's reason, "init --rust 'writes the snippets of design@agent-skills@rust-templates-one-home'; moving it to create, beside a new bundle crate that takes the rendering out of agent-skills, may change which crate it reads the snippets (cut)"
- `rejected@core@exempt-directory-manifest-row`: not read; the searcher's reason, "it holds that 'The compiled path can name only the checker's own source'; with the checker spread over many crates and a testing crate, a declared row could be proposed again as simpler, but the draft proposes"
- `rejected@core@fixture-marker-macro`: not read; the searcher's reason, "it is 'the recorded fallback should a literal outside the tool ever need rule-shaped bytes as data'; mock-based tests moved into a published testing Component used by extension writers might carry fixtures outside the (cut)"
- `rejected@core@literals-as-data-in-every-crate`: not read; the searcher's reason, "'Literals as data in every cfg(test) module of every crate'; the draft's many crates and its moving of tests could lead to extending the exemption beyond the tool's directories, but the draft keeps the checker's crates (cut)"
- `rejected@core@run-time-checker-directory-discovery`: not read; the searcher's reason, "it rejects finding the checker's directories at run time; a split into about ten crates multiplies the compiled-in directories, which may lead someone to propose discovery again; the draft does"
- `design@core@a-commit-message-is-a-document`: not read; the searcher's reason, "commits and the per-commit model move between the drafted model and check crates, and the head says 'The tip checker is the binary built from the working tree'; the split relocates its head and its code (cut)"
- `issue@agent-config@a-rebase-continue-strips-a-message-s-hash-lines`: not read; the searcher's reason, "the drafted rewrite Component (reword, reorder, fixup, squash) could replace the rebase procedure whose rebase --continue drops # lines; the draft does not say it replaces the conflict path, so no closing is"
- `design@agent-skills@goals-change-through-goal-setting`: not read; the searcher's reason, "The skill revision's 'a Component takes decisions about approximately one main subject' could lead the discussion to reword goal@knowledge-architect@projects-are-split-into-components ('each with one purpose'); the head (cut)"
- `design@agent-skills@goal-edit-ruled-alone`: not read; the searcher's reason, "A design discussion that proposes rewording the Components goal (one purpose vs 'approximately one main subject') must rule it as its own item, per 'A goal's wording that ... a design discussion proposes is drafted ... (cut)"
- `design@gates@a-project-holds-its-gate-list`: not read; the searcher's reason, "Its sentence 'The library depends on nothing of the checker' could meet a published git crate extracted from core, if gates (whose rebase gate runs git merge-base) took it as a dependency; the work leaves gates (cut)"
- `design@gates@one-spawn-helper`: not read; the searcher's reason, "The git crate's Invocation builder is a second published child-spawning layer; 'The gates start every child through the library's one spawn helper' would be met only if gates adopted it, which the work does not"
- `issue@gates@the-one-spawn-helper-has-no-recorded-argument`: not read; the searcher's reason, "Its re-entry is 'adds a second way to start a child in the gates library'; the git crate's Invocation builder would be one only if gates used it, which the work does not"
- `design@mutate@record-lives-in-the-git-directory`: not read; the searcher's reason, "Cites the seed design@model@git-supplies-the-walk, which subject 1 moves into the git/model crates; mutate runs git itself (crates/mutate/src/harness.rs, git rev-parse --git-path), and would meet the head only if it took (cut)"
- `design@model@plan-items-by-section`: not read; the searcher's reason, "the model crate takes the entity table and the extension API stays in check; the head argues 'items are the core's, per an-extension-plugs-in-through-phased-hooks'. Moving them to model might change which crate that (cut)"
- `design@core@plan-document-sections-by-kind`: not read; the searcher's reason, "the shared testing Component holds the mock projects and the mock-based tests; the head says 'Fixtures is owed only where a Component drives its tests with authored content'. Moving those tests might change which (cut)"
- `design@agent-skills@tripwire-recording-authority`: not read; the searcher's reason, "the discussion's premortem on the split and on the revised skills ends with causes judged and recorded 'in the tripwires home of the Component that owns the decision'; with new Components (model, check, query...), which (cut)"
- `design@agent-skills@tripwire-admission-test`: not read; the searcher's reason, "a premortem cause on the revised component-creation guidelines (sessions splitting wrongly) watches agent behaviour, which condition 2 ('its evidence lands where tripwires are read') may exclude; the work names no (cut)"
- `design@agent-skills@tripwire-horizon`: not read; the searcher's reason, "moving git.rs, build.rs, render.rs and the model files into new crates makes commits that change named paths; 'A relocation ... is no occasion' governs whether those moves count toward horizons of standing tripwires (cut)"
- `rejected@knowledge-architect@changelog-copies-written-at-packaging`: not read; the searcher's reason, "multiplying published crates multiplies committed changelog copies, which invites the rival of writing them at packaging; nothing in the work proposes"
- `rejected@knowledge-architect@gates-library-hosts-the-mutation`: not read; the searcher's reason, "cites the seed design@gates@gates-crate; the work keeps gates and mutate unchanged, so no element proposes hosting the mutation in"
- `rejected@knowledge-architect@single-published-package`: not read; the searcher's reason, "its argument 'a third crate could not live under it' is the packaging ground of the seed two-crates, which the split widens; the work goes away from one package, so no bringing back is"
- `design@knowledge-architect@git-flow`: not read; the searcher's reason, "moving code across many new crates may leave intermediate trees failing, and 'work whose intermediate trees cannot pass lands as one commit'; the work names no commit plan, so no bearing is"
- `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: not read; the searcher's reason, "rewrite's prune and squash drop or merge commits; a record carried by a message alone 'rides on a commit that changes a file', which such an operation could lose; the work states no behaviour of"

The two `may-bear` entries read are `design@agent-skills@subject-split-standard` and
`design@agent-skills@losing-shape-test`, whose outcomes the spec of the revision of the rules gave.

## Criteria

### Each Component that holds Rust code serves one purpose, with an interface that changes for a reason of its own `##one-purpose-per-component`

Binding, from `goal@knowledge-architect@projects-are-split-into-components`. Met by the eight
Component threads and #no-intermediate-component.

### The mechanical tasks the workflow asks over the scanned record are commands of a published crate that needs no check `##tools-need-no-check`

Binding, from `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, which asks the commands;
that they need no check is the owner's argument, #arg-29 and #arg-30. Met by #model-crate and
#query-crate; judged by #a1-no-check-below-the-tools.

### The documents can render as one linked site from the model alone `##render-from-the-model`

Binding, from `goal@core@documents-render-as-a-linked-site`, which asks the site; that it renders
from the model alone is the owner's argument, #arg-26. Met by #model-crate, which separates the
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
publish by hand. Ten new published names: `git`, `model`, `check`, `query`, `transcript`, `create`,
`testing`, and later `bundle`, `render`, `rewrite`.
Unmet-and-accepted, on the owner's word naming it: D1.

### Re-pointing the references to the renamed anchor `##rename-cost`

Weighed. 1,475 backticked references in 153 files name `core` on `origin/main`. The session weighed
the cost against a descriptive name and proposed the rename; the owner approved it, R5, with the
reason "core would become misleading".

## Threads

### `git` is a Component of its own: every git invocation and the readers of git's objects, later the write side of the history rewriting `##git-crate`

Proposed by the session, R1 (#arg-5 to #arg-7), from the owner's plan, R1 (#arg-2). Final state:
approved, by the owner's word against the table, R3: "For the split: I agree with your table now."
and R6: "Agreed on the table." Arguments: #arg-2, #arg-5, #arg-6, #arg-7, #arg-19, #arg-97, #arg-114.
Shape: landed at #git-out. Harvest: the `git` Component's design home, and the head
of the crate's separation, `design@knowledge-architect@git-crate`, in the root's, since the core's
dependency on it spans two Components.

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
explore." The table the owner approved, R6, also listed the `model` command in `query`; the session
moved it to `check` while writing these documents, on its own decision: the command calls each
extension's dump hook, so it needs the extensions, and in `query` it would make `query` depend on
`check`, which #a1-no-check-below-the-tools refuses. The owner's leads for the later commands, R2,
the keyword search and the audit's work lists, are recorded in
`issue@core@no-command-searches-the-entries-or-lists-an-audit-s-work`. Arguments: #arg-9, #arg-10,
#arg-29, #arg-30, #arg-31, #arg-48, #arg-64, #arg-112. Shape: `spec@component-split@consumers-out`.
Harvest: the `query` Component's design home. Absorbs #worklist-crate.

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
`spec@component-split@cli-on-top`. Harvest: the successor of `design@core@single-crate` in the
root's design home, in the change that reverses it, per point 1 of
`skill@knowledge-architect-decision-recording@owning-component` and the reversal rule as the design
discussion on the rules for Components revised them; the `cli` Component's design home for its own
heads. The session's first routing of the successor to the root, #arg-98, withdrawn at the plan's
review under the reversal rule as it stood, is the routing that rule now gives.

### `testing` is a shared test Component, a published crate an extension's tests depend on for the harness, holding this repository's fixtures and integrated tests where no package ships them; `cli` re-exports nothing of it `##testing-crate`

Proposed by the owner, R2 (#arg-37 to #arg-41), against the session's R1 cut (#arg-18); shaped by
the session, R2 (#arg-55 to #arg-57), and R4 (#arg-83, #arg-84, #arg-87, #arg-88). Final state:
approved, closed by the owner's choice at D2, after the plan's review: "consumers should pull
testing separately, I believe. It seems more natural to me." It had first closed on the owner's word
against the checkpoint, R6, "Agreed on the table.", over the shape the review found cyclic. The
owner, R2:
"our mocking tools need to be published, because we support writing extensions, which needs the
mocking tools publsished. It could be published as its own crate too, though. Also, centralizing
mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in
multiple crates that need them, or crates reaching into mock projects of others." Arguments:
#arg-18, #arg-37 to #arg-41, #arg-55 to #arg-57, #arg-62, #arg-83, #arg-84, #arg-87, #arg-88,
#arg-101, #arg-102, #arg-108, #arg-110, #arg-118. The plan's review found that the approved shape,
`testing` publishing the harness while `cli` re-exports it, makes a dependency cycle; the thread was
reopened on that finding and closed by the owner's choice at D2, between the two shapes that
remove the cycle. Shape:
`spec@component-split@integrated-tests`. Harvest: the `testing` Component's design home;
`design@core@api-facade`, rewritten.

### No intermediate Component groups the new Components `##no-intermediate-component`

Proposed by the session, R1 (#arg-17). Final state: approved, by the owner's word against the
table, R3 and R6, and argued again at the design audit of #git-out under the definition of an
intermediate Component by the design it holds, and argued again under the rules on Components as
revised after that audit, where an intermediate Component groups Components that serve a purpose of
their own: no purpose is stated for a group of these Components. Arguments:
#arg-17. Shape: "Decided design" below. Harvest: a head of absence in the root design home, "no
intermediate Component groups the checker's Components or the installed text's", written at
#git-out.

### The core is renamed `check` `##core-renamed-check`

Proposed by the session, R1 and R2 (#arg-21, #arg-51), and R4 (#arg-95). Final state: approved. The
owner, R5: "core-renamed-check agreed to rename, core would become misleading". Arguments: #arg-21,
#arg-51, #arg-95, #arg-106. Shape: `spec@component-split@check-renamed`. Harvest: no head; the
commit records it.

### The installed-file check is a check family of the last phase, in `check` `##installed-text-check-home`

Proposed by the session, R4 (#arg-69, #arg-70), as shape B; replaced by B′ after the record search
(#arg-81, #arg-82). Final state: approved. The owner, R5: "installed-text-check-home: agreed on phase
4". Reopened at the plan's second review on a finding against part of its premise, #arg-69, and
closed again by the owner's answer at D3, which leaves only a root CLAUDE.md outside the walk in
phase 2. Arguments: #arg-50, #arg-68 to #arg-70, #arg-81, #arg-82, #arg-103, #arg-115. Shape:
`spec@component-split@check-renamed`. Harvest: `design@core@phases-gate-the-report`,
`design@core@owned-namespace-check` and `design@core@fix-before-the-checks` rewritten; the issue
`installed-file-findings-belong-in-phase-four` was closed by the commit that added this
document.

### The arguments of a command are parsed in the crate nearest the binaries among those that every binary offering the command depends on: `cli` for klarch, the crate itself for `gates` and `mutate` `##command-parsing-home`

Proposed by the owner, R2 (#arg-35); challenged at the owner's request, R3 (#arg-61), by the
session, R4 (#arg-71 to #arg-78). Final state: approved. The owner, R5: "command-parsing-home I
think your synthesis is right." Its title said "the lowest crate", the word of #arg-78, which read
literally points away from the binaries its own examples name; put to the owner as Q3 of the design audit of
#git-out, the owner ruled it a wording defect: "Q3 agreed it is a wording defect." Arguments: #arg-35, #arg-53, #arg-61, #arg-71 to #arg-78, #arg-100,
#arg-113. Shape: `spec@component-split@cli-on-top`. Harvest: the rule is stated by the installed
component-creation skill alone today, in two comment lines that keep its two parts, the top-level
binary crate and the parsing in the crate nearest the binaries, without a head; the slice whose
`cli` head states them is their second text, and writes the two heads of
`register@agent-skills@design`, which `cli`'s head on its own parsing cites, with
`design@core@arguments-parse-through-clap` rewritten.

### An intermediate Component exists where several Components share design the rest of the project does not; the component-creation skill's sentence on layering by count goes `##intermediate-by-shared-design`

Put to the owner as Q1 of the design audit of #git-out, which found the skill's "Where many
Components would sit beside one another, nested Components under an intermediate Component carry
the layering" against #no-intermediate-component. Final state: a lead of the owner, not a
ruling: on the owner's later word, the answer at that audit is a proposal to argue, test and
possibly reject, which a design discussion on the revision of the Component rules argues against
its rivals. The owner's answer at that audit: "Q1: the definition that comes later takes precedence. Both instructions should be
revised to align. The first "Where many Components would sit beside one another, nested Components
under an intermediate Component carry the layering." should be removed. It was from an initial
proposal of mine. The second instruction came from me too, later, after more brainstorming and
thought "An intermediate Component holds no Rust code in its own files. It groups nested Components
and holds the design and the documents several of them share but the rest of the project does
not." I think the second one should win here. It is a better shape. The first one should then be
along the line of "when several components share design that is not true of the whole project,
they are grouped under an intermediate component, holding the shared design and no code of its
own". The shape can be either of: the intermediate component is not a crate, each nested one is a
crate (recommended for isolation/independence). Or the other way, if crate count cost is too high,
the intermediate one is a crate, the other components are modules of it. I think this is the right
shape, with the right amount of leeway to organize the layout conveniently." Shape: the head of
absence of #no-intermediate-component, argued from the definition. Outcome: the design discussion
on the rules for Components kept the lead's definition and its first shape, made precise as a
principle on how the code of several Components is written, which they follow as one part of the
project; the review before the merge added, on the session's default the owner approved, that a
principle stated for a kind of Component stays in the root, and the owner added that a decision on
their shape, packaging or layout does too. The owner then raised the case of two principles whose
sets of Components cross, which no two directories hold, refused the session's default of sending
the second to the root since it made the layout depend on the order the principles were met in, and
chose an intermediate Component made from a purpose its Components serve together, stated in its
goals home, a decision on the code of several Components going to the lowest intermediate Component
enclosing them all, or to the root,
over no intermediate Component at all. It ruled out the lead's second shape, an intermediate Component that is a
crate whose nested Components are modules, since its crate root holds Rust code and an interface
and every dependency of one module reaches every consumer of the crate, the owner choosing to keep
the goal as it stands; and it dropped the module-directory Component on the owner's proposal; recorded in `design@agent-skills@intermediate-component-occasion` and
`design@agent-skills@component-crate-mapping`, with `rejected@agent-skills@module-directory-component`.

### The dependencies that span several Components are the root design home's; a rule that one crate depends on no other may be that crate's `##dependency-layout-home`

Put to the owner as Q2 of the design audit of #git-out, which found the skill's "Which Components
exist and how they depend on each other is a decision about the layout, which belongs in the root's
design home" against the harvest rows of #model-crate and #query-crate. Final state: a lead of
the owner, not a ruling: on the owner's later word, the answer at that audit is a proposal to
argue, test and possibly reject, which the same design discussion as #intermediate-by-shared-design
argues against its rivals. The owner's answer at that audit: "Q2: I would say this belongs to root design. It spans multiple
components of the project. It should not be considered spanning a subset of them, or this will
conflict with my answer to q1, and would require every pair of dependent in-project crates to have
their own container component. A decision that one specific crate should not depend on another one,
this can belong to the individual crate. Since it does not bear on the design of the other crate
(because it rules out the dependency, it creates less constraints for the other)." Shape: the
harvest rows of #model-crate and #query-crate stand, each a rule that its crate depends on no crate
of the checks, which the lead admits in that crate's home. Outcome: the design discussion on the
rules for Components kept the lead, the test being what the decision constrains, and settled its
"may" as "is": a rule that one Component does not depend on another is that Component's, since its
rival, the rule in the root, would be loaded by every session reading the root's design home. The
owner chose the root over the lowest Component enclosing the Components a decision constrains; recorded in
`design@agent-skills@dependency-decision-home`, stated by points 1 and 4 of
`skill@knowledge-architect-decision-recording@owning-component`.

### The record worklist is a Component of its own `##worklist-crate`

Proposed by the session, R1 (#arg-9, #arg-10). Final state: superseded by #query-crate, which holds
`worklist` with the other commands that read the model (#arg-31, #arg-48).

## Arguments

The arguments of the discussion that bear on this milestone's threads, numbered as the
discussion's extraction numbered them, so this document and the spec of the revision of the rules
named one argument by one number. Each quotes its key words verbatim. An argument's Threads line is
the extraction's reading of the threads named when it was made; a thread's Arguments line, written
at assembly, lists every argument that moved it, so the two lists need not match.

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

Corrected at the plan's review: `cli/transcript.rs` also calls `main_worktree_path` of the git module.

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

Corrected at the plan's review: `cli/create.rs` calls `fix_then_check`, `complete_working_tree` and `refuse_a_foreign_build`; it names `locate` only in its doc comments.

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

### The premise of the one-crate decision, that a separate binary crate saves its users no dependency, fails once the commands span crates `##arg-20`

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

Corrected at the plan's review: `show` builds the model and runs no phase; `id`, `worklist` and the writers run phases 1 to 3.

### The run-model head explains why the tools need phases 1 to 3 `##arg-43`

R2, the session. Threads: #model-crate.

"The run-model head explains why tools need phases 1–3: "a finding computed over an incomplete model is unreliable"."

### Resolving a reference and reporting a broken one are one function today `##arg-44`

R2, the session. Threads: #model-crate, #render-crate.

"Resolving a reference and reporting a broken one are one function today." / "Render needs the resolving part with no reporting attached, so this function has to be pulled apart whether or not the crates are split."

### My claim that "every change crosses the boundary" was overstated `##arg-45`

R2, the session. Threads: #model-crate.

"My claim that "every change crosses the boundary" was overstated." / "So 44 of the 99 commits that touch either side touch both. More than half stay on one side. Some of the "both" commits are explained by fact 2, since resolution currently sits in `check/`."

"Fact 2" is the second fact the session listed in R2: the resolver and the lints are one function today.

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

"Renaming `core` to `check` moves every head and reference that names the anchor `core`. Per `relocation-is-one-manifest-edit` that should cost one manifest edit, but the old slugs in commit messages stay as they are." The goal it names is now `goal@core@relocation-needs-only-goal-changes`, reworded.

### Its "two concrete changes" test, applied to one subcommand at a time, passes every time `##arg-53`

R2, the session. Threads: #commands-sharing-a-purpose, #command-parsing-home.

"Its "two concrete changes" test, applied to one subcommand at a time, passes every time. The skill should say that commands sharing one purpose and one design are one Component, with one module each. A top-level binary crate holds the parsing, the checks that can be done at parse time, and the interface, and pulls its logic from below."

### 'mock_projects.rs' is a '[cfg(test)]' module that tests internals `##arg-55`

R2, the session. Threads: #test-component-public-api, #testing-crate.

"`mock_projects.rs` is a `#[cfg(test)]` module that tests internals" / "`path@core@tests/binary.rs` (7,026 lines) already tests only through the binary, so after the split it goes naturally to `cli`'s tests." / "So the shared test Component works here if one rule holds: no crate's unit tests use a mock project."

### The mock-based tests of model and check cannot sit in a crate those two dev-depend on `##arg-56`

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

R4, the session. Threads: #installed-text-check-home.

finding 1: "[the issue `installed-file-findings-belong-in-phase-four`] says these findings belong in phase 4 under the core's own placement rule: the files are outside the walk, and no check reads the findings." / "The split moves this code anyway, so the reason for deferring mostly goes away."

### 'model' then depends on neither 'agent-skills' nor 'bundle' `##arg-82`

R4, the session. Threads: #installed-text-check-home.

"`model` then depends on neither `agent-skills` nor `bundle`." / "`index` and `index --staged` are no longer refused by a stale install." / "This is a timing fork you already ruled the other way. It's yours"

### The testing crate moves the 36 mock-based tests `##arg-83`

R4, the session. Threads: #test-through-public-api, #testing-crate.

finding 2: "#testing-crate moves the 36 mock-based tests. That doesn't bring back the rejected entry, which was about moving all unit tests." / "The guideline must be an order of preference" / "As a rule to move every test, it is the rejected design."

### 5 of the 36 build 'Inputs' by hand `##arg-84`

R4, the session. Threads: #testing-crate, #test-through-public-api.

"5 of the 36 build `Inputs` by hand." / "Through the public API, each needs a mock project whose tree really has that state. Making `Inputs` constructible would bring back `rejected@core@inputs-struct-literal`. Default: rewrite the 5 over mock projects."

Made precise at the plan's reviews: 5 struct literals of `Inputs`, two of them in the shared helpers `findings_with`, which 17 tests reach, and `phase_two`, which 1 test reaches; two of the five inject a state, an empty map of committed files.

### The entry says "the test fixtures and the design documents never reach a package `##arg-87`

R4, the session. Threads: #testing-crate.

finding 3: "`design@knowledge-architect@package-include-whitelist` says "the test fixtures and the design documents never reach a package"." / "Extension writers get the tools, not our fixtures."

### Constraint: the entry accepts '"fixture"' only inside a directory the binary names as its own source, so (cut) `##arg-88`

R4, the session. Threads: #testing-crate.

"Constraint: `design@core@checked-sentinel-values` accepts `"fixture"` only inside a directory the binary names as its own source, so `testing`'s directory must be on that list."

### 'bundle' needs a goal first `##arg-89`

R4, the session. Threads: #bundle-crate.

finding 4: "`bundle` needs a goal first" / "It also leaves open which crate holds the install." / "`bundle` holds the generic install" / "`check` holds the comparison family and the command."

### The entry counts three history tasks as taken in: the rebase, the reword and the fold `##arg-90`

R4, the session. Threads: #rewrite-crate.

finding 5: "`goal@knowledge-architect@mechanical-tasks-are-the-tool-s` counts three history tasks as taken in: the rebase, the reword and the fold." / "The port's other commands (`prune-dead-paths`, `split-by-path`, `list-large-paths`) are a generic part taken in on your word" / "That word belongs at the port, not now."

### The entry binds every command 'rewrite' ships `##arg-91`

R4, the session. Threads: #rewrite-crate.

"`design@knowledge-architect@no-operation-loses-content` binds every command `rewrite` ships."

### 'resolve' runs in phase 1, which now sits in 'model' `##arg-93`

R4, the session. Threads: #model-crate.

finding 7: "`resolve` runs in phase 1, which now sits in `model`." / "So `model` must define a phase-1 contribution interface (claimed tables, declared paths, generated files), and `check`'s `Extension` trait builds on it. One trait becomes two. That is a design point for the slice, not a reason against the split."

### A crate's first version can't go through the release workflow: it needs an API token, by hand `##arg-94`

R4, the session. Threads: (every new crate).

finding 8: "a crate's first version can't go through the release workflow: it needs an API token, by hand." / "Default: a crate is created when its slice builds it, so `render` and `rewrite` wait for their work. Each first publish is your word at the release."

### Renaming a directory "renames its anchor `##arg-95`

R4, the session. Threads: #core-renamed-check.

finding 9: "renaming a directory "renames its anchor ... and is no move", so this is outside its guarantee. Most of those references get re-pointed by the move to `model`, `query`, `git` and the rest anyway" / "with `model` split out, "core" no longer says which crate it is."

### The entry already anticipates this split `##arg-96`

R4, the session. Threads: #cli-crate.

finding 10: "`design@knowledge-architect@no-component-directory-named-after-the-project` already anticipates this split" / "Keeping the package name `knowledge-architect` on `cli` leaves the setup snippet's dependency line ... unchanged." / "`rejected@gates@gates-command-of-the-checker` binds `cli`: it flattens no `gates` command."

### An anchor named git makes a backticked git remote a finding `##arg-97`

R4, the session. Threads: #git-crate (Decided).

"`design@model@at-span-candidate-rule` makes a backticked git@host:… span a finding once an anchor `git` exists. The census found one such span ... inside a string literal of the checker's own source, which the checker reads as data. No prose is affected today."

### The successor of the entry is recorded in the root design home `##arg-98`

R4, the session. Threads: #cli-crate (Decided).

"The successor of `design@core@single-crate` is recorded in the root design home. The component-creation skill puts layout decisions there. That also sidesteps the case `the-reversal-rule-fixes-the-component-when-the-successor-decides-another-s-interface` asks about."

Withdrawn at the plan's review: the successor went to the incumbent's Component by the installed rule as it stood; the rule as the design discussion on the rules for Components revised it gives the root again; see #cli-crate.

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

"Your clarification of #test-through-public-api settles the conflict with `rejected@core@unit-tests-moved-under-tests`. The rule applies to integrated tests over mock projects, not to unit tests." (the restated guideline is `design@agent-skills@integrated-test-through-public-api`)

### This doesn't touch the 437 unit tests `##arg-108`

R5, the session. Threads: #test-through-public-api, #testing-crate.

"This doesn't touch the 437 unit tests. The 5 mock tests that build `Inputs` by hand each go one of two ways: the state they inject becomes a real mock tree, such as one with no committed file, or the claim becomes a unit test over an in-memory model, which `design@core@model-then-checks` already provides for."

Made precise at the plan's review: see the note of #arg-84.

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

The types the slices add:

```text
model::extension::Contribution   a struct, an extension's phase-1 contribution as data: the tables it claims, its complaints, its declared paths and its generated files; check computes it from each extension's Extension::tables and Extension::resolve, which keep their signatures, and hands it to model's phase 1, so no extension changes (#model-out)
model::resolve::Outcome          what the resolver returns for a reference: its Target, or the Cause of its failure, whose two texts the lints and show use (#model-out)
```

Every other name a slice adds is that slice's spec's, written in it at its design audit.

## Decided design

The shape of each Component the milestone builds is in the spec of the slice that builds it. The
threads that place work and build nothing:

### `bundle`, `render` and `rewrite` are placed, and created by their own work

Each is placed as a Component of its own, a published crate under crates/, created when its work
starts, in the roadmap's order after this milestone: `rewrite`, then `bundle`, then `render`. The
placement is a lead of that work, not a decision this milestone builds: each work is known and not
designed, so its leads are in its issue, per `skill@knowledge-architect-planning@spec-or-milestone`,
and its own design session judges them. The leads: `rewrite` depends on `git` alone (#arg-13);
`render` on `model` alone (#arg-11); `bundle` on nothing of the checker, and `agent-skills` depends
on it as a build-dependency (#arg-8, #arg-89). Each is created by the work that builds it, so no
crate name is published before its code exists (#arg-94). The nearest rival, creating them empty in
this milestone, reserves names for code no one has designed.

### No intermediate Component groups the new Components

The eight Components sit beside one another under crates/. An intermediate Component groups
Components that serve a purpose of their own, which its goals home states and their goals refine,
per `design@agent-skills@intermediate-component-occasion`; every candidate grouping shares design
that lives in one member, the checker's or the model's, or shares none (#arg-17), and no purpose is
stated for a group. A head of absence in the root design home, written at #git-out, states it.
Each slice that drafts the goals of a new Component under goal-setting asks whether the Components
the core is split into serve a purpose of their own beyond the root's goals; a purpose the owner
states for a group of them reopens the head of absence, and that slice creates the intermediate
Component. Which Components exist is the manifest's, and each
Component's purpose its goals home's; no head lists them.

## Mapping tables

**Every file of crates/core today, and where it goes.** In this table, `check` names the core until #check-renamed renames it. The slice named moves it; a file split
between two crates is cut at that slice's audit along the line the table gives. The crate's
Cargo.toml, its CRATES-IO.md, its CHANGELOG.md copy and its license copies stay with `check` and
are written anew for each new crate, as "How the work is done" lists; its README is split, each
crate's README taking the commands and the contract it holds; its CLAUDE.md and its record under
docs/ are split by subject, each slice moving what its new Component holds.

| today | goes to | slice |
| --- | --- | --- |
| `path@elsewhere@crates/core/src/git.rs` | `git` | #git-out |
| `path@elsewhere@crates/core/src/manifest.rs`, `path@elsewhere@crates/core/src/walk.rs`, `path@elsewhere@crates/core/src/survey.rs`, `path@elsewhere@crates/core/src/scan.rs`, `path@elsewhere@crates/core/src/source/`, `path@core@src/document.rs`, `path@elsewhere@crates/core/src/entity.rs`, `path@elsewhere@crates/core/src/records.rs`, `path@elsewhere@crates/core/src/model.rs`, `path@elsewhere@crates/core/src/index.rs`, `path@elsewhere@crates/core/src/finding.rs` | `model` | #model-out |
| `path@elsewhere@crates/core/src/check/tree.rs` | `model`: phase 2 | #model-out |
| `path@core@src/check/mod.rs` | `model`: `foundation`, with the installed-file check still in phase 2 until #check-renamed; `check`: phase 4, `Report`, `run_with` | #model-out |
| `path@core@src/check/references.rs` | `model`: the resolver; `check`: the lints | #model-out |
| `path@core@src/agents.rs` | `model`: the shipped text as phase 2 reads it, with the tests over it; `check`: the install and its repairs | #model-out |
| `path@core@src/extension.rs` | `model`: the phase-1 contribution; `check`: the rest of `Extension` and `Prepared` | #model-out |
| `path@elsewhere@crates/core/src/check/agents.rs` | `model` from #model-out, in the foundation; `check` from #check-renamed, a family of phase 4 | #model-out, #check-renamed |
| `path@core@src/check/generated.rs`, `path@core@src/check/registers.rs` | `check` | #check-renamed |
| `path@core@src/cli/history.rs` | `check` | #check-renamed |
| `path@core@src/cli/gathered.rs` | `model`: what phases 1 to 3 and the resolver read; `check`: the committed generated files and the `register.toml` files, which phase 4 alone reads | #model-out |
| `path@core@src/cli/mod.rs` | `cli`: the command enum, parsing, dispatch, `locate`; `model`: `complete_working_tree`, the gate of phases 1 to 3 that `worklist` and the writers call, so `query` reaches it with no check; `check`: `fix_then_check`; each domain crate: its commands' logic, behind a typed API | #cli-on-top, `complete_working_tree` at #model-out |
| `path@core@src/cli/output.rs` | `cli`, or the crate whose data it prints, per the pure and output layers of `skill@knowledge-architect-component-creation@component-shapes`, decided at the audit | #cli-on-top |
| `path@core@src/build_origin.rs` | `check`: the refusal of a foreign build, given the list of crates a binary links, which `cli` calls for every command and `create` calls for `init`'s PATH | #cli-on-top |
| `path@core@src/main.rs` | `cli` | #cli-on-top |
| `path@core@src/cli/worklist.rs`, `path@core@src/graph.rs` | `cli`: the clap declarations of `worklist`; `query`: the rest | #cli-on-top for the declarations, #consumers-out for the rest |
| `path@core@src/cli/transcript.rs`, `path@core@src/transcript.rs` | `cli`: the clap declarations of `transcript`; `transcript`: the rest | #cli-on-top for the declarations, #consumers-out for the rest |
| `path@core@src/cli/create.rs` | `create` | #consumers-out |
| `path@core@src/lib.rs` | each crate's own facade | each slice |
| `path@core@src/mock_projects.rs` | `testing` | #integrated-tests |
| `path@core@src/testing.rs` | `testing`'s library, the harness, per D2; until then `check`'s role module, which `cli` re-exports | #integrated-tests; #cli-on-top for the interim re-export |
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
- **The record worklist as a Component of its own**, superseded by #query-crate.
- **`explore` as the query crate's name**, lost on the owner's choice, R3.
- **The core's name kept**, lost to #core-renamed-check: with `model` split out, "core" no longer
  says which crate it is (#arg-95).
- **No shared test Component, the mock tests kept as a unit-test module**, lost to #testing-crate
  (#arg-37, #arg-41, #arg-55).
- **The harness kept a role module of `check`, re-exported by `cli` at `knowledge_architect::testing`,
  with `testing` unpublished**, the session's default at D2, lost to the owner's choice there; it
  is the import path extension writers use, an interface others consume, so the harvest of
  #integrated-tests judges it by the tests for a losing alternative.
- **A missing shipped file kept in phase 2**, the session's default at D3, lost to the owner's
  reading that the model is built just right over a missing installed text.

## Readings

Empty: the work reads no external specification. The two facts of Cargo the split rests on are read
in `path@agent-skills@docs/grounding/crate-boundaries.md`.

## Premortem

Assume the split shipped and failed:

| cause | thread it stresses | verdict |
| --- | --- | --- |
| C1: `check`, `query` and `render` need so many `model` internals that `model`'s public API becomes its module tree, `rejected@core@module-tree-as-public-api` back | #model-crate | a claim of #model-out: `model`'s lib.rs is a facade of role modules under `unreachable_pub` |
| C2: a `query` command needs phase-4 output, so `query` depends on `check` | #query-crate, #model-crate | an acceptance criterion, #a1-no-check-below-the-tools; the session found one such case while writing these documents, the `model` command, which the mapping table keeps in `check`, and a second at the plan's review, `complete_working_tree`, which the mapping table gives to `model` |
| C3: a rule stated in a domain type and in the clap declaration drift apart, so an invalid combination passes clap and is refused later, or with another exit code | #command-parsing-home | a claim of #cli-on-top: `cli` maps every refusal by a domain's options to exit 2 before any work, one test per refusal |
| C4: `rewrite`'s write side needs an environment the git heads say every invocation drops or keeps, such as a temporary `GIT_INDEX_FILE` | #git-crate | named as a cause; the port meets it as a request of its own design, so no tripwire |
| C5: in phase 4, a stale install shows reference findings beside the install finding, and a session repairs the references instead of running the install | #installed-text-check-home | named as a cause, judged at the harvest of #check-renamed; `--fix` repairs the installed files first |
| C8: an extension writer needs a helper that lives in `testing`'s tests, which no package ships | #testing-crate | named as a cause; `tripwire@core@private-item-needed` already watches a consumer needing an item it cannot reach |

C6 and C7 of the discussion stressed the threads of the spec of the revision of the rules, whose
harvest dropped both as tripwires.

## Acceptance criteria

### No crate below the tools depends on the checks: `cargo tree` for `model` and `query` shows no `knowledge-architect-check` `##a1-no-check-below-the-tools`

Guards #model-crate, whose premise is that the tools over the record need the model and no check.
Judged at #consumers-out, for `model` and `query`, and at the milestone's last landing. Fires when
`cargo tree -e normal -p knowledge-architect-query` or `cargo tree -e normal -p knowledge-architect-model`
lists `knowledge-architect-check`: a normal dependency, since a dev-dependency builds into no
consumer. A dependency added this way passes every gate, so the failure is
otherwise silent. Fails to fire by construction until #consumers-out creates `query`; shown to
fire by a scratch dependency of `query` on `check`, which the command lists. The premise that
`render` needs no check either is a lead of its issue, for its own design session.

## Implementation sequence

1. The git invocation layer and the readers of git's objects become the `git` crate, #git-out:
   landed, its spec deleted with it.
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
  slice lands, and that slice moves a module that imports nothing of the crate.
- #model-out before #check-renamed: the rename then renames what is left of the checker, not the
  model.
- #check-renamed before #cli-on-top: `cli` takes the package name `knowledge-architect` from a
  crate already named `check`, so the slice that moves the binary renames no directory.
- #cli-on-top before #consumers-out: `create` depends on `check`, so its command can leave the
  checker only once the binary sits above both.
- #consumers-out before #integrated-tests: the integrated tests drive every crate through its public
  API, which each crate has only once it is out.

## Defaults awaiting the owner

- **D2, on #testing-crate, ruled**: `testing` is a published crate whose library holds the harness
  an extension's tests need, and which an extension's tests depend on separately; `cli` does not
  re-export it and does not depend on it; `testing` depends on `check` and `model` for the harness,
  and on `cli` for its integrated tests alone; its `tests/` holds this repository's fixtures and
  integrated tests, which its package does not ship. The plan's review found that the approved
  shape, `testing` publishing the harness while `cli` re-exports it, makes a cycle: `cli` would
  depend on `testing`, whose integrated tests use `cli`, against
  `design@agent-skills@shared-test-component`. Two shapes remove it: the harness kept a role module
  of `check` that `cli` re-exports, with `testing` unpublished, which spares consumers a change of
  import and one first publish; and this one, which gives the harness and the fixtures one home.
  The session proposed the first; the owner chose the second, which no goal ranks: "consumers
  should pull testing separately, I believe. It seems more natural to me." An extension's tests
  change one import, from `knowledge_architect::testing` to the `knowledge-architect-testing`
  crate, at #integrated-tests, with a Migration entry of the changelog. Until that slice, the
  harness is still `check`'s module, and `cli` keeps re-exporting it under today's path, so a
  release cut between #cli-on-top and #integrated-tests changes no import and a consumer migrates
  once.
- **D3, on #installed-text-check-home, ruled**: every finding of `check::agents` on the installed
  files, a missing shipped file included, and the finding on a root CLAUDE.md that does not import
  the primer, go to the last phase, in `check`; `model` depends on nothing of `agent-skills`. The one
  finding that stays in phase 2, in `model`'s foundation, is a root CLAUDE.md the walk keeps out,
  which needs only the manifest's harness, until
  `issue@model@the-walk-may-exclude-a-file-the-model-requires` replaces it. The plan's second review
  found that #arg-69 holds of a differing file and not of a missing one, whose entities the model
  then lacks, and the session proposed to keep a missing file in phase 2. The owner answered on the
  fact: a missing installed text is no problem for the scanner, the model is built just right in
  that case, and checking that every skill is installed is the checker's, maybe the bundler's. It
  holds of the code: `Model::build` builds over a missing file, and the references to its entities
  are reported as unresolved in the last phase, beside the install finding, which is premortem C5's
  case; a writer over that model writes over the tree as it is, so
  `tripwire@core@phases-gate-the-report-two`'s premise holds. The owner gave a rule, that the walk
  refuses to exclude a file the model requires, and left to the session whether to build it in the
  split; the session left it to that issue, since it is a stricter check that owes a Migration
  entry, and the split's slices relocate behaviour.
- **D1, on #cost-per-crate, ruled**: the fixed cost of each new crate is accepted, and each crate is
  created by the slice that builds it, `bundle`, `render` and `rewrite` by their own work. The owner,
  after the plan documents were committed: "D1 the costs are accepted".

- **D4, on #model-crate, awaiting the owner**: `model`'s facade publishes the entity table and the
  anchors, `Entities` and `Anchors` among them, and the records over them, under `model::table`, and
  the core's facade adds none of them: `extension` offers what it offers today, so the direction the
  owner chose in `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`, a narrow
  read-only view in `extension` rather than these types public as they are, stands for the extension
  API. Its consequences, each read at the design audit of #model-out:
  - an extension that adds `knowledge-architect-model` to its own dependencies can name them, so the
    types are public on crates.io, which that direction did not have in view: they were crate-private
    when it was given, and `check`, `query` and `render` each need them across a crate boundary, so
    #model-crate requires them public somewhere;
  - each method of `Model` and `Manifest` that code staying in the core calls becomes public on
    `knowledge_architect::Model` and `knowledge_architect::Manifest` too, since a re-export carries a
    type's methods: `Model::listing`, `untracked`, `installed`, `links`, `from_documents_under`,
    `set_installed` and `push_unreadable`, and `Manifest::registers`, `plans`, `locations`,
    `components`, `pin` and `serves_claude`, among others;
  - the alternative that keeps them private needs a second set of types over the table, in `model`,
    for `check` and `query` alone, which an extension could name in the same way, so it removes neither
    consequence.

  The session's default is this shape, with a `library` entry of the changelog stating the widening,
  and the issue and `tripwire@core@private-item-needed` reworded to say what the crate now publishes.
  The two sentences of the owner's direction relax for a consumer who adds the dependency, so this is
  a scope change the owner rules.

## Harvest

| what | where | when |
| --- | --- | --- |
| #no-intermediate-component | a head of absence in the root design home, `register@knowledge-architect@design` | at #git-out |
| each slice's threads | its spec's harvest row | at that slice's landing |
| the reversed decisions and the losing alternatives above | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | at the landing of the slice that decides each |
| C4, C8 | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | at the milestone's last landing; C5 at #check-renamed |
| #a1-no-check-below-the-tools | reported at #consumers-out and at the last landing | as `skill@knowledge-architect-planning@acceptance-criteria` says |
| the part of each slice's decisions that says which Components exist, or how they depend on each other where it constrains more than one of them | the root's design home, per point 1 of `skill@knowledge-architect-decision-recording@owning-component`, where it earns a head; a rule that one Component does not depend on another, in that Component's | at the landing of the slice that decides each |

## Later consequences

- `rewrite`, `bundle` and `render` are each created by their own work, in the roadmap's order; their
  issues hold the leads.
- The release after the milestone publishes each new crate's first version by hand, on the owner's
  word, per `design@knowledge-architect@publish-credential`; the release workflow's crate list and
  order, and the package lists of `skill@klarch-release`, take the new crates. **A release cut
  during the milestone** must publish every crate the landed slices created, each first version by
  hand: from #git-out on, the published checker depends on them, and a crate published on
  crates.io cannot depend on one given only by a path. That cost is D1's.
- `gates` and `mutate` may later move onto the `git` crate; nothing schedules it.
