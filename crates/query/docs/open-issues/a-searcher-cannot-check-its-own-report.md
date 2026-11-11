---
kind: defect
---
# A record searcher cannot check its own report before it returns

## Summary

`klarch worklist --verify <dir>` judges every report of a work list at once and first removes the
output files of `<dir>`, so a record searcher, which may write only its own `report.md`, does not
run it. The searcher's definition names no other check. A searcher therefore returns a report it
has not checked, and every report the dispatching session's run of `--verify` fails costs one more
dispatch. In one search, 14 of 18 reports failed the first check and 2 failed the second.

## Details

### What

The command: `--verify` in `path@query@src/worklist.rs` takes the directory `--out` wrote, judges
every `agent-<n>/report.md` against `agents.txt`, `work.txt` and `shown.md`, and, as its help text
says, "first removes every `verdicts-*.md`, the `generated-lines.md`, the `gaps.md` and the
`fit-lines.md` of DIR". It has no form that judges one report and writes nothing.

The definition: `path@agent-skills@content/agents/record-searcher.md` tells the searcher that it
edits one file, its `report.md`, and runs only commands that read the project. Its section on
filling the report states the line shape and the quotation's length, and names no command that
checks the filled report. The description names `--verify` as the dispatching session's step,
after the agents return.

The instance: the record search of the design discussion that made the primer's import line the
root CLAUDE.md's first line, 2026-11-11, 18 agents.

- First run of `--verify`: 193 findings, over 14 of the 18 reports, every one about a quotation:
  not found as a run of whole words of the entry's own text, or shorter than eight words.
- The 14 agents were sent again, each naming its findings. Second run: 24 findings, over 2 reports,
  17 and 7 lines.
- The two were sent again. Third run: passed.

Read in the agents' own reports:

- three said they did not run `--verify` because it deletes and writes files outside their agent
  directory;
- at least eight wrote a quotation checker of their own, in Python. These differed from the
  command's rule. One matched a quotation anywhere in `entries.md` rather than in its own entry,
  and passed the 17 lines the second run then failed. Others included or excluded the heading line
  differently, and their authors reported lines they believed would fail a later run.

Ruled out: that the findings were mostly lines left ungraded. Every finding of the first run named a
quotation. The cause of most of them was a quotation cut before a word's punctuation, which the
check's message now names. This entry is about who can run the check, and it holds whatever the
check's rule is.

### Why it matters

The command keeps the report's shape only after the agents return, so each failing report costs a
round trip of the dispatching session: here 16 re-dispatches, two rounds of waiting for them, and
three runs of the check. Each searcher meanwhile builds an approximation of the check, which can
pass a line the command fails. It strains `design@query@worklist-writes-and-verifies-the-reports`,
whose argument is that the rule is kept "by a command, not by a reader", and
`goal@tasks-a-tool-does-better-are-the-tool-s`. The fix changes
`design@agent-skills@one-record-searcher`, whose dispatcher checks every report and sends back a
report that fails.

### What would close it

A form of the check that judges one agent's report and writes nothing, such as
`klarch worklist --verify <dir> --agent <n>`, exiting 1 with the same findings when the report
fails; and the record searcher's definition telling the agent to run it, and to repair what it
reports, before it returns. A test shows that the form writes no file and reports the findings of
that report alone.
