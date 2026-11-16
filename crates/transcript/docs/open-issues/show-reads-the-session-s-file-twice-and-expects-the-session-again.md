---
kind: observation
---
# `transcript show` reads the session's file twice and expects to find the session again

## Summary

`transcript show` finds the session in a read of every file of the project, then reads that
session's file a second time and takes the session by its place in the file with an `expect`. If
the file changes between the two reads so that no session holds that place, the command panics
instead of reporting what it could not read. Not reproduced: the case is reasoned from the code.

## Details

### What

In `show` in `path@transcript@src/command.rs`, the first `Corpus::read` over every file of the
project resolves the session the caller named, and keeps its file and its `ordinal`, its place in
that file. A second `Corpus::read` over that file alone then cuts the sessions again, since the
handle's `#n` counts over every file read, as the comment there says, and takes the session with
`.find(|s| s.ordinal == ordinal).expect("the session is cut again from its file")`.

A session's file is written by the harness while the session runs, as the transcript of the session
that met this observation was. An append adds lines after the ones the first read cut, which would
keep every earlier session's place. Whether the harness ever rewrites or truncates a session's file,
so that the second read cuts fewer sessions than the first, is not established.

Met by the documentation pass of this crate, on the branch of the spec on the doc gate and
the documentation lints: `clippy::missing_panics_doc` does not report it, since the `expect` sits in
a private function that the public `transcript` calls.

**What would make it reproduce:** a test that writes a session file, starts the first read, then
replaces the file with one that holds fewer sessions before the second read; or an observed run in
which the harness rewrote a session's file during a `show`.

### Why it matters

A panic gives the caller a backtrace in place of the report every other failure of the command
prints, and a nonzero exit that reads as a crash of the reader rather than as a session that
changed under it. A session that reads its own live transcript, as a review does, is the caller
most likely to meet a file that changes between the two reads.

### What would close it

Either a reproduction, which makes it a `defect` whose fix answers the missing session as the
command answers a session it cannot find; or a reading of the harness's writes showing that a
session's file only grows, which closes it as correct behaviour.
