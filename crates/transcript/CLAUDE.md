# transcript

The reader of Claude Code's session transcripts. What it is for is `register@transcript@goal`; what it does for a user is
`path@transcript@README.md`; why it is shaped this way is `register@transcript@design`.

**This crate knows nothing of a project but its root**, which its caller hands in, and asks git through `git` alone, for the
main worktree's path. It depends on nothing of `model` or of the checks.

**A new value of the transcripts' format is counted, never a failure**, per
`design@transcript@transcript-format-tolerated-and-reported`, and a line is labelled by its fields, never by a substring of its
text, per `design@transcript@transcript-labels-in-code`.

**`path@transcript@src/output.rs` is a copy of the writer**, per `design@knowledge-architect@a-copy-of-the-writer-per-crate`: every write to stdout goes through it, and a closed stdout ends the command
with exit 2, per `design@cli@exit-code-ladder`. 

The fixtures under `path@transcript@tests/transcripts/` are read by the tests of the binary, in `path@cli@tests/binary.rs`, since
`CARGO_BIN_EXE_klarch` is set there alone.
