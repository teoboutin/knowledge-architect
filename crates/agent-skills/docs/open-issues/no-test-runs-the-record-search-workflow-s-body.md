---
kind: todo
---
# No test runs the body of the record search's workflow: its round loop, its report writer and its outputs are untested

## Summary

Two parts of the record search's saved workflow are tested, its flag builder and its merge of the
failing reports, each a top-level function that `path@agent-skills@src/workflow_parse.rs` takes from
the installed text and runs under `node`. The rest of its logic sits in the script's top-level body,
beside the harness's `agent`, `phase`, `pipeline` and `log` calls, and no test runs it.

## Details

### What

The logic of `path@agent-skills@content/workflows/record-search.js` that no test runs, by name:

- the `maxRounds` cap, 3 where `args` gives none, and the round loop that runs `--verify`, sends the
  reports `failingReports` returns in `sendBack` to their searchers, and returns `failing` once the
  runs are used up or `sendBack` is empty;
- `agentName` on what the worklist step returns, and the error the script throws where a name holds
  no `agent-<n>`; the merge test runs `agentName` only on the agent names of the check's return;
- `delimiter`, which lengthens `END_OF_REPORT` until the report text holds no such line, and
  `writeText`, which writes a report returned as text, and returns the searcher's result as it is
  where the report was written or no text came back;
- the outputs the script returns: the default subject `whole-work`, each subject once in
  `verdictFiles`, and `gaps` and `fitLines` in place of `generatedLines` where `seedsFrom` is given;
- the checks of the `args` types.

Defects were found in this logic by reading only. The development checkpoint's adversarial review,
in the commit "[review] The development checkpoint's repairs of the record search's workflow: every
failing report sent back, the quoting tested on two quotes, and the result's statement made true",
found five, its items 2 to 6, with a stub of the harness run under `node` that no commit holds.
The harvest's agent then found that the returned `failing` was the check's agent's own list, not the
list the loop built, repaired by "[agent-skills] The record search's workflow returns the failing
reports it sent back, not only those the check's agent listed"; that part moved since into
`failingReports`, which a test runs. Examples of mutations nothing would catch here:
`round >= maxRounds` made `round > maxRounds`, which runs `--verify` once more than the cap; and
`while (text.split('\n').includes(d))` made `while (false)`, which ends the heredoc at a line of the
report that is `END_OF_REPORT`.

### Leads

- The shapes of `issue@agent-skills@no-test-runs-a-saved-workflow-s-logic`'s leads apply: more
  top-level pure functions that a test runs on fixed inputs, as `worklistCommand` and
  `failingReports` are, or a sandbox that runs the script whole with stubs of the harness's calls.
- A function taken by the test is read from its `function <name>(` line to the next lone `}`, as
  `path@agent-skills@CLAUDE.md` says.

### Why it matters

A defect of this logic is silent: a cap off by one spends a run of searchers, a heredoc cut short
writes part of a report, which the check then fails and sends back, and a wrong output path points
the session at a file that does not exist. `design@agent-skills@report-loop-in-the-workflow`
describes the loop that this logic carries out, and no test holds the script to it.

### What would close it

A test, run by `cargo x gates` with `node`, that runs each part listed under What on fixed inputs
and catches the two mutations above, by one of the shapes of the leads.
