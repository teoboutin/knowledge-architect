# The commands over the model, the transcript reader and the project creator leave the checker

The fifth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `query`, `path@knowledge-architect@crates/query/`, package `knowledge-architect-query`,
  depending on `model`, and on `git` for `last_changed`, the last column of `issues`: the logic of `show`,
  `issues`, `tripwires`, `anchors`, `id` and `worklist`, with `graph.rs`.
- The Component `transcript`, `path@knowledge-architect@crates/transcript/`, package
  `knowledge-architect-transcript`, depending on `git` for `main_worktree_path`, taking the project's root
  from its caller, since `manifest.root()` is all its verbs read of a manifest.
- The Component `create`, `path@knowledge-architect@crates/create/`, package `knowledge-architect-create`,
  depending on `check`, `model`, `git` and `agent-skills`: `init`, `new-component`, `init --rust`. `cli`
  locates the project and hands it in, so `create` calls no `locate`. It calls `model`'s
  `foundation::complete_working_tree`, and of `check`:
  - public already: `commands::check` with `CheckMode::Fix`, which stands for `fix_then_check`,
    `refuse_a_foreign_build`, `Library`, `extension::Extension`, `CHECKER_VERSION` and `output::write`;
  - published by this slice under `commands`: `shipped`, `install_into`, `Installed` and `IMPORT_LINE` as they
    stand, since `finish` prints its own lines of what `install_into` did, and `init` refuses a destination
    in `shipped`; and one new predicate, `staged_changes_stop_the_fix`, in place of `staged_generated`, since
    `new_component` asks only that.
- `check` loses `regex`, `serde_json` and `toml_edit`, which no other module of it uses. No gate reports an
  unused dependency, so this slice removes them by hand.
- `cli` dispatching each command to its crate; the goals of the three Components, those of `query` from the owner's lead in the thread #query-crate, with a goal of `query` refining `goal@knowledge-architect@records-reach-their-reader`, from the owner's statement at the goals of #check-renamed: "query allows records-reach-their-reader by offering convenient ways to consult the records, especially when the workflow requires to consult them under a given constraint, methodology.", drafted and put to
  the owner as the milestone document says.

## Claims

- **`query` and `model` depend on no `knowledge-architect-check`**: the milestone's acceptance
  criterion #a1-no-check-below-the-tools, reported at this landing.
- **Every moved command gives the same output and exit code.** Test: the binary's suite in `cli`
  passes with no assertion on a moved command's output edited; shown to discriminate by a recorded
  mutation of one moved command's output, which a binary test fails on. Its diff is confined to: the three
  lines that read the two moved fixtures, the `checker source:` line and its count over the directories,
  the list of packages in the refusal's `cargo clean`, and the tests this slice adds. The `checker source:` line and the list of packages are text of commands that do not move, and each owes a changelog entry. The one exception to the
  claim is the line D6 adds to `init`'s output.
- **A closed stdout ends each crate's commands with exit 2.** Test: one test per copy of the writer, over a
  command of `query` and a command of `transcript`, beside the test over `model` and the one over `init`, since
  those two reach no copy; shown to fail by a recorded mutation that exits 0 from the copy.
- **A tree holding a command's crate at another path is refused**, per
  `design@check@a-foreign-build-is-refused`. Test: a test over the three new packages; shown to fail by a
  recorded mutation that drops `query` from the list `klarch` hands over.
- **`query`'s commands read a project with an extension's table as `check` does, through `cli::run`.** Test: an
  integrated test over a mock project that declares a table an extension claims, which `issue@model@a-tool-over-the-model-reads-a-project-with-an-extension-otherwise-than-the-checker`
  asks; shown to fail by a recorded mutation that removes `extension::configure` from `cli::run`. This is D8's.
- **`worklist`, `id` and the writers still refuse over an incomplete model.** Test: the existing
  tests of `design@query@worklist-writes-and-verifies-the-reports` and
  `design@query@id-lookup-command` pass; shown to fail by a recorded mutation that skips the gate in
  `worklist`.
- **`init`'s tests pass over `create`'s code.** Test: the file #cli-on-top moved to `cli`, since `create`
  has no binary and `CARGO_BIN_EXE_klarch` is set in `cli` alone, unchanged but for the test of D6;
  shown to discriminate by a recorded mutation of `init`'s refusal of a directory holding files, which a test
  fails on.

## Audit subjects

`issue@create@init-outside-a-worktree-deletes-an-owned-file-silently`, which reopens two heads of `init` at this audit;
`transcript-is-a-command-of-every-binary`, rewritten as `design@transcript@transcript-command-reads-any-project`; the heads of
`path@elsewhere@crates/check/docs/design/worklist.md`, `path@elsewhere@crates/check/docs/design/transcripts.md` and
`path@elsewhere@crates/check/docs/design/creating.md`; `design@query@anchors-listing-command`,
`design@check@one-question-one-command`; `design@transcript@transcript-directory-resolution`;
`design@agent-skills@rust-templates-one-home`, whose test, `path@xtask@tests/scaffold.rs`, stays in
xtask and calls `init` through `cli`'s function `cli::init` and its `InitArgs`;
`issue@transcript@a-forked-subagent-s-first-line-is-an-unrecognised-type`,
`issue@query@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
`issue@query@show-lists-no-subdocument-of-a-directory-home`,
`issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
`issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work`, each re-anchored;
`issue@model@the-structural-findings-might-be-checked-at-scan-time`, D7's;
`issue@model@a-tool-over-the-model-reads-a-project-with-an-extension-otherwise-than-the-checker`, D8's;
`issue@create@init-and-new-component-run-an-extension-on-a-manifest-it-did-not-configure`, which moves open,
since its repair changes what an extension's binary does and this slice relocates.

**What every commit's tree must pass.** This slice's first change sets nothing of it: adding a directory to the
list a binary hands over only reads its literals as data, and `snapshot_checker_dirs` of `path@model@src/model.rs`
reads a literal as data only in a document under a listed directory, so an earlier tree of the branch is judged as before, which
`cargo klarch commits origin/main..HEAD` over the built branch shows. A crate and its rows in `path@cli@src/main.rs`
go in one commit, or that commit's own check reads its literals as prose.

**The printing's home, which #cli-on-top's audit found and left here,** is D5's consequence, placed by this
slice's audit as D9 of the milestone document says: one copy of the writer in `query` and one in `transcript`, and
`create`, which depends on `check`, a macro of its own over `check`'s public `output::write`.

## Fails alone on

A moved command whose output or exit code differs, or a `cargo tree` of `query` listing the checks.

## Premises that expire

The integrated tests over the mock projects still live in `check` and `model` until
#integrated-tests; that slice's claim that no unit test uses a mock project guards it.

## Decided design

**`query`.** One crate for the commands that read the model and run no check: one purpose, mechanical
tasks over the scanned record, and one design, the graph, the packing and the exchange with agents
(#arg-31, #arg-48). The `model` command stays in `check`: it calls each extension's dump hook, so it
needs the extensions, the case premortem C2 names; the session moved it there while writing these
documents, from the table the owner approved.

**`transcript`.** It imports nothing of the checker but the project's root and `git`'s
`main_worktree_path`, and changes when the harness's transcript format does (#arg-9, corrected at
the plan's review, #arg-10). `transcript-is-a-command-of-every-binary`, now `design@transcript@transcript-command-reads-any-project`, said "The reader is internal";
that sentence is what this slice rewrites, since #cli-on-top removed the one that argued from
`rejected@knowledge-architect@single-crate`.

**`create`.** It has its own purpose, tests and 13 heads, counted with
`grep -c '^### ' crates/create/docs/design.md` as the slice left it; the cut costs the public API of `check` the five items
"Builds" lists, D10's subject, and of `model` nothing (#arg-14, corrected at this slice's audit).

**The commands' documentation.** Command-line usage and exit codes stay in `path@cli@README.md`, since `cli` parses
every command, per #command-parsing-home. Each new crate's README states what an adopter of the library does, that
a project running the tools names `knowledge-architect` alone, and the conditions of a target project.

## Implementation sequence

1. `transcript`, which depends on nothing of the checker: the mechanism of a crate leaving `check`.
2. `query`.
3. `create`.
4. The report on #a1-no-check-below-the-tools; the harvest.
5. The goals of the three Components, drafted last, since a batch of more than three goals stays uncommitted in the
   tree until the owner rules, and an uncommitted tree blocks every folding of a repair.

## Harvest

| what | where |
| --- | --- |
| #query-crate | the root's design home: a head for the crate and what it depends on, under the thread's slug. `query`'s design home: a head that it depends on no crate of the checks; the heads of `path@elsewhere@crates/check/docs/design/worklist.md` but `design@check@no-citation-direction-rule`, which stays in `check` beside `design@check@goals-home-cites-upward-only`; `design@query@anchors-listing-command` and `design@query@id-lookup-command`; with the `rejected` entries those alone cite and the issues on its commands |
| #transcript-crate | the root's design home: a head under the thread's slug, the part saying that the reader is a crate of its own that `cli` depends on. `transcript`'s design home: the transcript heads, and the successor of `transcript-is-a-command-of-every-binary` under a new slug, with the `rejected` entry and the issues on them |
| #create-crate | the root's design home: a head for the crate and what it depends on, under the thread's slug. `create`'s design home: the heads of creating, with the `rejected` entries and the issues on them |
| what each head that names the crates says | `design@knowledge-architect@git-crate`, `design@knowledge-architect@checker-asks-git-through-the-crate`, `design@knowledge-architect@model-crate` for "the writers run them and no check", which `init` and `new-component` refute, `design@knowledge-architect@api-facade`, `design@check@the-facade-publishes-what-a-consumer-needs`, `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, `design@check@a-foreign-build-is-refused`, `design@check@checker-source-literals-are-data`, `design@knowledge-architect@partial-publish-rerun` |
| the issues whose text names moved code or the sites of the crates | `issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data`, `issue@knowledge-architect@a-typed-command-trusts-its-caller-to-have-configured-the-manifest`, `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter`, `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`, `issue@agent-skills@the-upstream-set-omits-the-published-libraries`, `issue@cli@some-moved-behaviour-of-the-commands-survives-a-mutation`, `issue@model@the-resolver-s-arms-have-mutation-survivors`, and each sentence that cites this spec, rewritten before it leaves |
| the changelog | the entry on `knowledge-architect-check` edited; a `library` entry per new crate; D6's `cli` entry |
