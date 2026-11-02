# The commands over the model, the transcript reader and the project creator leave the checker

The fifth slice of `milestone@plans@component-split`. It leaves when it lands.

## Builds

- The Component `query`, crates/query, package `knowledge-architect-query`, depending on `model`:
  `show`, `issues`, `tripwires`, `anchors`, `id` and `worklist`, with `graph.rs`.
- The Component `transcript`, crates/transcript, package `knowledge-architect-transcript`, depending
  on `git`, taking the project's root from its caller.
- The Component `create`, crates/create, package `knowledge-architect-create`, depending on `check`,
  `model`, `git` and `agent-skills`: `init`, `new-component`, `init --rust`.
- `cli` dispatching each to its crate.

## Claims

- **`query` and `model` depend on no `knowledge-architect-check`**: the milestone's acceptance
  criterion #a1-no-check-below-the-tools, reported at this landing.
- **Every moved command gives the same output and exit code.** Test: the binary's suite in `cli`
  passes unchanged.
- **`worklist`, `id` and the writers still refuse over an incomplete model.** Test: the existing
  tests of `design@core@worklist-writes-and-verifies-the-reports` and `design@core@id-lookup-command`
  pass.
- **`init`'s tests pass in `create`.** Test: `path@core@tests/init.rs`, moved.

## Audit subjects

`design@core@transcript-command-in-core`, rewritten; the heads of `path@core@docs/design/worklist.md`,
`path@core@docs/design/transcripts.md` and `path@core@docs/design/creating.md`;
`design@core@anchors-listing-command`, `design@core@one-question-one-command`;
`design@core@transcript-directory-resolution`; `issue@core@a-forked-subagent-s-first-line-is-an-unrecognised-type`,
`issue@core@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`,
`issue@core@show-lists-no-subdocument-of-a-directory-home`,
`issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`, each re-anchored.
Each `@core@` reference here names the entry as this document is written; #check-renamed re-points
them.

## Fails alone on

A moved command whose output or exit code differs, or a `cargo tree` of `query` listing the checks.

## Premises that expire

The integrated tests over the mock projects still live in `check` and `model` until
#integrated-tests.

## Decided design

**`query`.** One crate for the commands that read the model and run no check: one purpose, mechanical
tasks over the scanned record, and one design, the graph, the packing and the exchange with agents
(#arg-31, #arg-48). The `model` command stays in `check`: it calls each extension's dump hook, so it
needs the extensions, the case premortem C2 names.

**`transcript`.** It imports nothing of the checker but the project's root and `git`'s
`main_worktree_path`, and changes when the harness's transcript format does (#arg-9, #arg-10).
`design@core@transcript-command-in-core` rejected a second crate on the ground of
`design@core@single-crate`, which #cli-on-top reversed.

**`create`.** It has its own purpose, tests and 13 heads; the cut costs the public API of `check`
the helpers it calls, `fix_then_check`, `locate` and `complete_working_tree` (#arg-14).

## Implementation sequence

1. `transcript`, which depends on nothing of the checker: the mechanism of a crate leaving `check`.
2. `query`.
3. `create`.
4. The report on #a1-no-check-below-the-tools; the harvest.

## Harvest

| what | where |
| --- | --- |
| #query-crate | `query`'s design home: the worklist heads and those of the commands it holds |
| #transcript-crate | `transcript`'s design home: the transcript heads, `design@core@transcript-command-in-core` rewritten |
| #create-crate | `create`'s design home: the heads of creating |
