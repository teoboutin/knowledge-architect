---
kind: defect
---
# A declared command holding a quote or a backslash installs saved workflows that do not parse

## Summary

The install renders `{{command}}` into each installed file by a plain replace, and every placeholder
of the three saved audit workflows sits inside a single-quoted JavaScript string literal. A
`[project] command` holding `'` or `\` therefore installs workflows whose script does not parse, and
the parse test of this repository, which parses the copies installed for its own command, does not
see it.

## Details

### What

- The render: `render` in `path@check@src/agents.rs` is
  `lf(&template.replace(PLACEHOLDER, command))`, with no escaping for the file it writes.
- The sites: `grep -n "{{command}}" crates/agent-skills/content/workflows/*.js` lists each
  placeholder inside a `'…'` string, as in design-record-audit.js,
  `'- run {{command}} issues and save its output to '`.
- The admission: `design@check@declared-command` refuses a command that is empty, holds a line break
  or holds a backtick; a `'` or a `\` passes.
- The test: `path@agent-skills@src/workflow_parse.rs` parses `crate::FILES`, the copies rendered for
  this repository's command, `cargo klarch`, which holds neither character.
- Reproduction, not run: declare `command = "cargo 'klarch'"` in a mock project's manifest, run the
  install, and run `node --check` on an installed workflow wrapped as the parse test wraps it.

Found by a record searcher of the in-change discussion that added the parse test, in its report's
"Met outside the task", and confirmed by reading `render` and the workflows. Ruled out: that the
markdown texts are affected the same way; a quote in a markdown code span parses.

### Why it matters

A project whose command holds a quote meets a failing audit run at its first use, after every gate
passed, against `goal@every-target-project-can-adopt-it`.

### What would close it

The install escapes the command for the literal it renders into, or `design@check@declared-command`
refuses a command holding `'` or `\`, with a test that installs the workflows for such a command and
parses them.
