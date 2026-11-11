---
kind: todo
---
# No test runs a shipped saved workflow's logic: the counts the audit workflows report as exact are untested

## Summary

The three audit workflows compute the counts their report copies as exact, and no test runs that
logic: it sits in each script's top-level body, beside the harness's `agent`, `phase` and `log`
calls, so nothing can call it apart from a real run. The parse of each installed saved workflow is
tested, by `path@agent-skills@src/workflow_parse.rs`; what the scripts compute is not.

## Details

### What

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

### Leads for the shape of the test

- **The official documentation offers no test of a saved workflow.** Read on the Claude Code
  documentation's pages on workflows, on plugin evals and the plugin reference: a syntax error is
  reported "when you run the workflow"; `claude plugin eval` "loads the target plugin's skills,
  hooks, and agents", workflows not among them, and its runs are real model calls; `claude plugin
  validate` checks "a plugin's files for syntax and schema errors rather than its behavior" and the
  paths under each component key, and no page says it parses a workflow's script. A plugin can ship
  saved workflows, in a `workflows/` directory at its root.
- **A sandbox that runs a script whole**, with stubs of `agent()`, `parallel()`, `pipeline()`,
  `phase()` and `log()` returning fixed results, would check the counts in what the script returns
  with no change to the scripts. A community project, `vannifr/workflow-script-harness` on GitHub,
  states that it runs workflow scripts unmodified under Node's test runner, in Node's `vm` module,
  with scripted agent responses; not established: it was not read, only its description in a search
  result, and it says it is no project of Anthropic. Its cost: the stubs are a second statement of
  the harness's API, which a change of the harness leaves stale until a real run shows it.
- **Pure functions the script's body calls**, which a test calls with fixed returns, the shape the
  code-shape finding above named, change each script and need no stub of the harness.
- Either runs on `node`, which the parse test already requires where `cargo x gates --ci` runs
  and skips without elsewhere, per `design@gates@ci-flag-for-ci-only-requirements`.

### Why it matters

The counts a report states as exact are checked by nobody, against
`goal@tasks-a-tool-does-better-are-the-tool-s`, which the commit that added them cites:
a wrong count reaches the owner's list as a fact the session copied, not one it took.

### What would close it

A test, run by `cargo x gates` with `node`, that runs the count logic of the three audit workflows
over fixed agent returns and fails on a planted mutation such as the one above, by one of the two
shapes of the leads, chosen under `skill@knowledge-architect-design`.
