---
kind: todo
---
# `show` on a register home in the directory shape prints its path and not its subdocuments

## Summary

`cargo klarch show register@<anchor>@<register>` on a home in the directory shape prints the
directory and the references to it. An instruction to read such a home sends a reader to a
directory of subdocuments that the command does not list.

## Details

### What

Instance: C3 of the retrospective of the session that moved two design homes to the directory
shape, 2026-10-25: `cargo klarch show register@core@design` printed the directory path and the
references only. `design@core@register-home-reference` states that output as decided.

### Why it matters

Every text that asks a session to read a design home whole, by its register reference, reaches a
directory whose index the session must find itself, against
`goal@core@records-reach-their-reader`.

### What would close it

`show` on a home in the directory shape lists each subdocument with its title, with a test, and
`design@core@register-home-reference` states the output, through
`skill@knowledge-architect-design` since it adds to that head.
