---
kind: todo
---
# The line type `session-owner` of the harness's transcripts is not classed

## Summary

`klarch transcript list` and `klarch transcript show` report a line type they do not recognise,
`session-owner`. The command tolerates it and says so in its header, as its design asks. Nobody
has measured what such a line holds, so it is not known whether a reader of a transcript loses
anything by it.

## Details

### What

On 2026-11-10, `cargo klarch transcript list` printed, over the transcripts of this repository's
project on the owner's machine:

```
# unrecognised type "session-owner": 265, first at -home-teo-dev-knowledge-architect/221be2f0-cdd5-5e76-b391-13ea5eba2fec.jsonl:1
```

The count grows with every session. The first such line is the first line of a session's file. A
search of `path@transcript@src/` for the type's name finds nothing. Not established: what keys the
line carries, which versions of the harness write it, and whether it holds anything a session or a
review reads. An agent that extracted one session reported it in files of harness versions 2.1.306
to 2.1.321; that range was not taken again.

### Why it matters

`design@transcript@transcript-format-tolerated-and-reported` has the command report what it does
not recognise, so that the gap is classed. Until it is, every extraction carries the header line,
and a line that held something of the owner's would be read by no review, the failure
`design@transcript@transcript-labels-in-code` is built against: one of the owner's messages dropped
from a review.

### What would close it

The measurement that head asks before a value is classed, over the transcripts: the keys of the
lines of this type and the harness versions that write them; then the type classed in the reader,
with its label or as a line that carries nothing to label, and a test of it.
`issue@transcript@no-verb-takes-the-inventory-of-the-transcripts-line-kinds` records that no verb
takes that measurement.
