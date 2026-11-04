# The commands over the model, the transcript reader and the project creator leave the checker

The fifth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `query`, crates/query, package `knowledge-architect-query`, depending on `model`:
  the logic of `show`, `issues`, `tripwires`, `anchors`, `id` and `worklist`, with `graph.rs`.
- The Component `transcript`, crates/transcript, package `knowledge-architect-transcript`, depending
  on `git` for `main_worktree_path`, taking the project's root from its caller.
- The Component `create`, crates/create, package `knowledge-architect-create`, depending on `check`,
  `model`, `git` and `agent-skills`: `init`, `new-component`, `init --rust`. It calls
  `check`'s `fix_then_check` and refusal of a foreign build, and `model`'s `complete_working_tree`;
  `cli` locates the project and hands it in, so `create` calls no `locate`.
- `cli` dispatching each command to its crate; the goals of the three Components, those of `query` from the owner's lead in the thread #query-crate, with a goal of `query` refining `goal@knowledge-architect@records-reach-their-reader`, drafted and put to
  the owner as the milestone document says.

## Claims

- **`query` and `model` depend on no `knowledge-architect-check`**: the milestone's acceptance
  criterion #a1-no-check-below-the-tools, reported at this landing.
- **Every moved command gives the same output and exit code.** Test: the binary's suite in `cli`
  passes unchanged; shown to discriminate by a recorded mutation of one moved command's output,
  which a binary test fails on.
- **`worklist`, `id` and the writers still refuse over an incomplete model.** Test: the existing
  tests of `design@check@worklist-writes-and-verifies-the-reports` and
  `design@check@id-lookup-command` pass; shown to fail by a recorded mutation that skips the gate in
  `worklist`.
- **`init`'s tests pass in `create`.** Test: `path@check@tests/init.rs`, moved; shown to discriminate
  by a recorded mutation of `init`'s refusal of a directory holding files, which a test fails on.

## Audit subjects

`design@check@transcript-command-in-core`, rewritten; the heads of
`path@check@docs/design/worklist.md`, `path@check@docs/design/transcripts.md` and
`path@check@docs/design/creating.md`; `design@check@anchors-listing-command`,
`design@check@one-question-one-command`; `design@check@transcript-directory-resolution`;
`design@agent-skills@rust-templates-one-home`, whose test, `path@xtask@tests/scaffold.rs`, stays in
xtask and calls `init` through `cli`'s re-export of `cli::init` and `InitArgs`;
`issue@check@a-forked-subagent-s-first-line-is-an-unrecognised-type`,
`issue@check@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
`issue@check@show-lists-no-subdocument-of-a-directory-home`,
`issue@check@no-command-lists-the-standing-entries-citing-a-set-of-entries`,
`issue@check@no-command-searches-the-entries-or-lists-an-audit-s-work`, each re-anchored. Each
reference naming the anchor `core` here names the entry as this document is written;
#check-renamed re-points them.

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
the plan's review, #arg-10). `design@check@transcript-command-in-core` rejected a second crate on the
ground of `design@check@single-crate`, which #cli-on-top reversed.

**`create`.** It has its own purpose, tests and 13 heads, counted with
`grep -c '^### ' crates/check/docs/design/creating.md`; the cut costs the public API of `check` and
`model` the three functions it calls (#arg-14, corrected at the plan's review).

## Implementation sequence

1. `transcript`, which depends on nothing of the checker: the mechanism of a crate leaving `check`.
2. `query`.
3. `create`.
4. The report on #a1-no-check-below-the-tools; the harvest.

## Harvest

| what | where |
| --- | --- |
| #query-crate | `query`'s design home: the worklist heads and those of the commands it holds |
| #transcript-crate | `transcript`'s design home: the transcript heads, and the successor of `design@check@transcript-command-in-core` under a new slug, its part saying that the reader is a crate of its own that `cli` depends on in the root's design home, per the milestone document's harvest |
| #create-crate | `create`'s design home: the heads of creating |
