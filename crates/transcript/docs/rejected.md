# Rejected designs

The designs transcript did not keep, and why.

### A transcript reader that refuses a line type or a value it does not know `##strict-transcript-reader`

`live`. Over every transcript of the machine the reader was designed on, Claude Code 2.1.306 to
2.1.310, each version added a line type, an attachment type or a key with the feature it shipped,
and a parser that refused the unknown would have failed on each; a third-party parser,
claude-code-log, printed a warning on every line of the new types, its issue 116 reporting that
"Claude Code 2.1.x emits a few new top-level JSONL types" it did not recognise. The window's types are re-taken with the command of
`design@transcript@transcript-format-tolerated-and-reported`. The harness documents the
format as internal and liable to change between versions.
