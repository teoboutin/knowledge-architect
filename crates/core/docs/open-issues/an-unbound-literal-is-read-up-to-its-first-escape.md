---
kind: defect
---
# An unbound Rust string literal is read as prose only up to its first escape sequence

## Summary

The Rust extractor reads a string literal that is not bound to a name as prose, but only its first
`string_content` node: tree-sitter splits a literal's content at each escape sequence, so the text
after the first `\n`, `\"` or other escape is read by no check. A reference, a path or an id there
is silent.

## Details

### What

`collect` in `path@core@src/source/rs.rs`, in its arm for `string_literal` and
`raw_string_literal`, takes `named_child(child, "string_content")`, the first child of that kind,
and pushes its lines as prose. A literal holding an escape has several `string_content` children,
one on each side of each `escape_sequence` node.

Reproduced with a unit probe over `rs::parse` and `scan` with `Literals::Prose`, removed after the
run:

- `fn f() { g("x\n` + a backticked reference + ` after"); }` yields no `Observation::Span`;
- the same literal with a space in place of the `\n` yields one.

Found by the adversarial review of the branch that built the lints on ids, which met it outside
its subject; the extractor is unchanged by that branch. A raw string literal has no escapes, so it
is read whole: not established by a run.

### Why it matters

`design@core@grammars-not-prefixes` makes an unbound literal prose, since an assertion's message
cites rules and references for a human to read. Text after an escape in such a message is then
checked by nothing, and a reference there dangles unseen, against
`goal@knowledge-architect@documentation-stays-consistent`. The lints on ids miss it too.

### What would close it

The extractor reads every `string_content` child of an unbound literal, each at its own line, with
a test of a literal holding a reference after a `\n` and after a `\"`.
