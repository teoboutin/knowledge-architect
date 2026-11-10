---
kind: todo
---
# No test runs a shipped saved workflow: its parse check depends on a JavaScript runtime the machine may lack, and the counts it computes are untested

## Summary

`path@agent-skills@CLAUDE.md` asks that an edited saved workflow be checked to parse with
`node --check`. In two sessions in a row the machine held no `node`, `deno` or `bun`, and no test
parses a saved workflow, so the gates passed over edited scripts nobody parsed. A test of the
agent-skills crate that parses each installed saved workflow would run on every machine.

A second part needs the same JavaScript engine in a test: the three audit workflows compute the
counts their report copies as exact, and no test runs that logic, which the scripts give no seam
to call apart from the harness.

## Details

### What

The contract, in `path@agent-skills@CLAUDE.md`: "The harness runs the file's body as an async
function, so a top-level `return` is legal there and `node --check` refuses it on the bare file:
check it parses by wrapping the body in `async function body() {…}`, with `export` removed, on the
installed copy under `path@agent-config@workflows/`, since a `%%` line of the source is no
JavaScript."

The saved workflows are the three files under `path@agent-skills@content/workflows/`. What the
build checks of them: their `meta` name, by `check_workflow_name` in `path@agent-skills@src/render.rs`.
Nothing parses them: a search of `path@agent-skills@build.rs`, `path@agent-skills@src/` and the
crates' tests for the names of JavaScript parser crates (swc, oxc, boa, quickjs, esprima) finds
none, and `path@knowledge-architect@Cargo.lock` names none.

The instances: the session of the pre-release fixes of 2026-11-01 and the session of the 0.9.0
release, the same day; in each, the machine held no `node`, `deno` or `bun`. The parse check the
first of them owed was run with `gjs`, and shown to fail on an injected syntax error, in the commit
"[agent-skills] The two edited audit workflows pass the parse check, which closes its issue". `gjs`
is on the owner's machine and `node` is not; neither is a dependency of the build.

A constraint from `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`: a `%%`
line in a saved workflow's source is no JavaScript, so a parse runs on the installed copy, which
the build has stripped. The crate exposes that copy, `path@agent-skills@src/lib.rs`.

The rivals the two retrospectives named: a CI step with `node --check`, which the local gates, the
ones a session trusts, would not run; and naming `gjs` beside `node` in the contract, which keeps
the check manual and per machine. An assumption, not checked: a Rust crate that parses modern
JavaScript, top-level `await` and `return` inside the wrapped body included, can be a
dev-dependency of the agent-skills crate; the choice of crate is part of the work.

Source: P1 of the analysis 2026-11-01-knowledge-architect-pre-release-fixes, with P3 of
2026-11-01-knowledge-architect-release-0-9-0, in the commit "[intake] The analyses of forty
received retrospective files, three outcomes for the owner".

### What, second part: the counts the audit workflows compute

The commit "[agent-skills] The audit workflows find earlier passes by path, compute their counts,
fall back to a text report, and the pin move offers its audits" adds the computation of the numbers
each report copies "as given". The comment at each site says why the script computes them: "The
counts the report copies: the script holds each exactly, so no agent takes one by reading". The
sites, by name:

- `path@agent-skills@content/workflows/design-record-audit.js`: `judgedCount`, the sum of the
  judges' verdicts in the judgement of an application; `ofRecord`, the drafts of record, which
  counts in each group only the entries of that group's own list, so a calibration-sample entry
  drafted by every group counts once; `perComponent` and `countLines`, which split the entries by
  kind on `e.ref.startsWith('design@')` and `e.ref.startsWith('rejected@')`, and per Component on
  `e.ref.split('@')[1]`;
- `path@agent-skills@content/workflows/tripwire-audit.js`: `judgedCount`; `ofRecord`, per outcome,
  with the same own-entries filter; `perAnchor` and `countLines`;
- `path@agent-skills@content/workflows/agentic-workflow-audit.js`: `countLines`, whose verifier line
  branches on `args.from && args.from !== 'verify'`.

Each is computed inline in the script's top-level body, which also makes the harness's `agent`,
`phase` and `log` calls, so no test can call the count logic without the harness. The only check
the commit names is "Each edited script parses". No mutation was run on it. An example of a
mutation nothing would catch: replacing `own.has(j.entry)` by `true` in either `ofRecord` loop
counts each sample entry once per group.

The scripts' earlier logic, such as `cut` and the lists of entries missing from a return, is
untested in the same way. That is a lead of the same work, not established as a defect.

Source: finding 1 of the code-shape axis of the review before the merge of the intake of forty
received retrospective files.

### Why it matters

A saved workflow that does not parse fails at its first run in a project that installed it, after
every gate passed, against `goal@knowledge-architect@documentation-stays-consistent`, the nearest
goal. A check that runs only where the machine holds a runtime is skipped on the others, as twice
in a row.

The counts a report states as exact are checked by nobody, against
`goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`, which the commit that added them cites:
a wrong count reaches the owner's list as a fact the session copied, not one it took.

### What would close it

A test of the agent-skills crate, run by `cargo x gates`, that parses each installed saved workflow
with a JavaScript parser compiled into the test and fails on a syntax error planted in one; with
the contract's sentence in `path@agent-skills@CLAUDE.md` replaced by a pointer to that test; and,
with the same engine, the count logic of the three audit workflows moved into pure functions that
take the agents' returns, called by the script's body, and a test that runs them over fixed returns
and fails on a planted mutation such as the one above. The work adds a dev-dependency, so it needs
a branch of its own.
