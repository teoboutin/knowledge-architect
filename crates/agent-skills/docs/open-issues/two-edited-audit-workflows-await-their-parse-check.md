---
kind: todo
---
# Two saved workflows edited on the pre-release branch have not passed the parse check

## Summary

The saved workflows of the design-record axis and of the tripwire axis were edited on the branch
`pre-release-fixes`, and the parse check that `path@agent-skills@CLAUDE.md` asks of a saved
workflow was not run on either: the machine that made the edits holds no JavaScript runtime.

## Details

### What

The edits, in the commit "[agent-skills] The two audit workflows' defects, and seven shipped texts
each read one way":

- in `path@agent-skills@content/workflows/design-record-audit.js` and
  `path@agent-skills@content/workflows/tripwire-audit.js`, the function `cut` labels a packed group
  `'packed+' + ++p`, and the drafting brief lists the sample through
  `sample.filter((e) => !g.entries.some((y) => id(y) === id(e)))`;
- in `path@agent-skills@content/workflows/design-record-audit.js`, the application run builds a
  constant `verdicts` and passes it to its report step in one more prompt line, in the shape the
  tripwire workflow already holds.

The parse check: wrap each installed copy's body in `async function body() {…}`, with `export`
removed, and run `node --check` on it. On the editing machine `node`, `deno` and `bun` were absent,
and the snap's `geckodriver` could not start Firefox under the session's shell, so no engine parsed
the files. Two reviewers read the edits; reading is not a parse. No test of the repository parses a
saved workflow: a search of `*.rs`, `*.toml` and `*.yml` for `node --check` found none.

### Why it matters

A saved workflow that does not parse stops the audit pass that runs it, in every installing
project, per `design@agent-skills@one-run-pass`; the release ships both files.

### What would close it

The parse check run on both installed copies, `path@agent-config@workflows/knowledge-architect-design-record-audit.js`
and `path@agent-config@workflows/knowledge-architect-tripwire-audit.js`, on a machine that holds `node`,
before the release, its result in the closing commit's message, and any syntax error repaired.
