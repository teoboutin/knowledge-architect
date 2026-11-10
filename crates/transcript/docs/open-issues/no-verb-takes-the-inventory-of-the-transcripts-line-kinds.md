---
kind: todo
---
# No verb of `transcript` takes the inventory of the transcripts' line kinds and their keys

## Summary

Before a new value of the transcript format is classed, the heads of `transcript` ask for a
measurement over the transcripts: per line kind, the key sets, the harness versions and the lines
carrying a key. No verb takes it. The heads hold hand-written `find` and `jq` pipelines as their
measurement, and one of them read 42 of 963 files without anyone noticing.

## Details

### What

`design@transcript@transcript-format-tolerated-and-reported` lists the line types with a command
over the transcripts; `design@transcript@transcript-labels-in-code` holds one
`find … | xargs jq` command as its measurement. `transcript --help` lists the verbs `list`, `show`
and `search`, none of which counts line kinds or keys.

The instance: the session that closed seven transcript issues, 2026-10-31. Three of the issues
asked for such a measurement before a value was classed; the session wrote four Python scripts and
several `jq` pipelines to count, per line kind, the key sets, the harness versions and the lines
carrying a key. One count showed that every top-level interrupt carries `interruptedMessageId`,
against an issue's recorded diagnosis. A reviewer found that the format head's own `find` read 42 of
963 files: it missed the subagent files. The change "[core] The line-type command of the transcript
format head reads the subagent files too" repaired that command with a second `find` branch.

Ruled out: the pipelines kept and corrected, as that change did. The depth defect stood unnoticed
from the head's writing until a reviewer found it, and the labels head still holds one such
pipeline; a verb tested over fixtures removes the class. Against the verb: its only users are the
maintainers of the reader. `design@transcript@transcript-query-verbs` states that the lists a
session reads "for a survey of past sessions" are verbs of `transcript`, which admits it.

### Why it matters

A pipeline in a head is code no test runs, and a wrong one misclasses a value that the header then
stops reporting, against `design@transcript@transcript-format-tolerated-and-reported`, which has
every run report each value the reader does not know. The format head makes the measurement owed
at each new value.

### What would close it

A verb that prints, over the files the reader reads, one row per line kind with its count, its
harness versions and its keys with their counts, tested over the fixtures, and the heads'
pipelines replaced by it; or a ruling that the pipelines stay, deleting this entry with that reason
in the commit.
