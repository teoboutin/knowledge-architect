---
kind: defect
---
# On Windows `worklist` names its reports with a backslash and its files with a verbatim prefix, which the record search's workflow does not read

## Summary

`worklist --verify` names a failing report as `agent-1\report.md` on Windows, and `work.txt` holds paths that
open with `\\?\`. The saved workflow of the record search finds an agent in the command's output by a pattern
with `/`, so it sends no failing report back there.

## Details

### What

Observed on a GitHub Windows runner, in the tests of `path@testing@tests/binary.rs`:

- `judge_reports` of `path@query@src/worklist.rs` joins the agent's name and the report's, and prints the
  join: "agent-1\report.md  design@widget@gears-turn is given to agent-1 and has no verdict";
- `work.txt` holds `spec spec@plans@an-audit \\?\C:\Users\…\docs\plans\specs\an-audit\README.md`, since
  `absolute` of the same file makes the path canonical.

Read: `path@agent-skills@content/workflows/record-search.js` reads the output with
`/(agent-[0-9]+)\/report\.md/` and a directory with `/(?:^|\/)(agent-[0-9]+)(?=\/|$)/`, neither of which
matches a `\`. Not run on Windows.

The paths are outside the project, so they are printed as the system writes them, by the decision of
`spec@plans@gates-on-windows-and-macos`; the two tests compare them so.

### Why it matters

A record search on Windows whose reports fail would return with no agent to send back, or would stop at the
worklist step, against `goal@agents-get-a-complete-workflow`.
The owner left development on Windows for later, while a project developed there is one that
`design@knowledge-architect@platforms-served` serves.

### What would close it

The report named with `/` by the command, or the workflow's patterns reading both separators, and a search
run on Windows whose failing report is sent back.
