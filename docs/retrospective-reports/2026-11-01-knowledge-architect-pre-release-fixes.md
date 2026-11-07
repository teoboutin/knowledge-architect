# 2026-11-01-knowledge-architect-pre-release-fixes

- **Received file**: 2026-11-01-knowledge-architect-pre-release-fixes.md, the project file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.8.0-234-geb899523`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`.

## P1, with P3 of 2026-11-01-knowledge-architect-release-0-9-0: the parse check of a saved workflow depends on a runtime the machine may lack

- **a.** What it says: the parse check of `path@agent-skills@CLAUDE.md` runs through `node --check`;
  in two sessions in a row the machine held no `node`, `deno` or `bun`, and no test parses a saved
  workflow, so the gates passed over edited scripts nobody parsed. Proposal of P1: a test of the
  agent-skills crate that parses each shipped saved workflow with a JavaScript parser compiled into
  the test, and the CLAUDE.md pointing to it (instructions −1, +0); rival, a CI step with
  `node --check`, which the local gates would not run. Proposal of P3: a CI gate with `node`;
  rival, naming `gjs` beside `node`.
- **b.** Whether it still applies. Yes. `path@agent-skills@CLAUDE.md`, lines 9 to 15, at `eb899523`
  and at HEAD: "The harness runs the file's body as an async function, so a top-level `return` is
  legal there and `node --check` refuses it on the bare file: check it parses by wrapping the body in
  `async function body() {…}`, with `export` removed." No test parses a saved workflow: a search of
  `path@agent-skills@build.rs`, `path@agent-skills@src/` and the crates' tests for the names of
  JavaScript parser crates (swc, oxc, boa, quickjs, esprima) finds none, and `path@knowledge-architect@Cargo.lock` names none. The parse-check issue the first session opened was closed
  by the commit "[agent-skills] The two edited audit workflows pass the parse check, which closes its
  issue" (`df8ff61a`), run with `gjs`, shown to fail on an injected syntax error. `gjs` is on this
  machine (`which gjs` prints its path); `node` is not.
- **c.** Whether it is already recorded. No open entry asks for a test. Bearing on it:
  `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`, which records that a `%%`
  line in a saved workflow's source is no JavaScript, so a parse runs on the installed copy, which
  the build has stripped.
- **d.** Its kind: missing (a check whose means depends on the machine), two instances.
- **e.** Its scope: `goal@knowledge-architect@documentation-stays-consistent` is the nearest; the
  saved workflows are shipped text, `agent-skills`. The fix reverses no decision: no head decides how
  a saved workflow is checked, and the sentence is a contract in a scoped CLAUDE.md, `path@agent-skills@CLAUDE.md`.
- **f.** A better fix. The test of P1 against the CI gate of P3: the local gates, which a session
  trusts, run the test and would not run a CI step, which decides for the test. The test reads the
  installed text the crate exposes, `path@agent-skills@src/lib.rs`, which answers the `%%` constraint
  of the issue above. Naming `gjs` keeps the check manual and per machine. Assumption, not checked:
  a Rust crate that parses modern JavaScript can be a dev-dependency of the agent-skills crate; the
  choice of crate is part of the work.
- **g.** Its route: a change to the Rust source, `skill@klarch-development`, with a new
  dev-dependency, which needs a branch of its own.
- **h.** The proposal: open an issue in `agent-skills`, kind `todo`, proposed id
  `issue@agent-skills@no-test-parses-a-saved-workflow`: the sentence, the two sessions, `gjs` as the means used at
  `df8ff61a`, the constraint of the comments issue, and as closing condition a test that fails on a
  planted syntax error in an installed saved workflow, with the CLAUDE.md sentence replaced by a
  pointer to it.
- **Default**: open an issue, `agent-skills`, kind `todo`, `issue@agent-skills@no-test-parses-a-saved-workflow`.
- **Outcome**: decided by the session: open an issue. Ground: the defect holds at HEAD; a test is
  run by the gates on every machine, which no manual step is; adding a dependency is work for a
  branch of its own.

## P2: no procedure moves a commit to the front of the branch

Analysed with P2 of 2026-11-01-knowledge-architect-narrowed-goals, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals.md`.

## P3: a subagent's count relayed to the owner without re-counting

Analysed with the claim-check lapses of the standing answers of
2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.
