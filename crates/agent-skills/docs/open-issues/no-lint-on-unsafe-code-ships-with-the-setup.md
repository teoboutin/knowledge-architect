---
kind: todo
---
# No lint on `unsafe` code ships with the setup, which the decision against a security axis assumes

## Summary

`design@agent-skills@no-security-axis` leaves code marked `unsafe` to a lint gate rather than a
reviewer. The setup skill names no such lint, so a target project's gates refuse no `unsafe` code
unless the project adds one itself.

## Details

### What

The milestone that shipped the development skill rejected a security review axis on two grounds: the
adversarial reviewer attacks accepted inputs, and `unsafe` is a lint gate. The first is built; the
setup skill's clippy gate carries no lint on `unsafe`, such as `unsafe_code = "forbid"` in a crate's
`[lints.rust]` table, or `clippy::undocumented_unsafe_blocks` where `unsafe` is allowed.

### Why it matters

The half of the decision that keeps `unsafe` out of a reviewer's job rests on a gate no target
project is asked to run, so `unsafe` code reaches a merge judged by nobody.

### What would close it

The setup skill asks, with the owner's word since it adds to an ask, a lint on `unsafe` code in the
clippy gate, and `design@agent-skills@no-security-axis` names it; or the owner rules that none ships,
and the head says what judges `unsafe` code instead.
