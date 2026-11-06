# query

Tools over a project's scanned record. What it is for is `register@query@goal`; what it does for a user is
`path@query@README.md`; why it is shaped this way is `register@query@design`.

**This crate depends on no crate of the checks**, per `design@query@query-depends-on-no-check`. A command that needs a
check's output, or an extension's hook, is no command of this crate: `model` stays in `check` for that reason.

**A new command is a function here and a declaration in `cli`**, per `design@cli@command-parsing-home`, in a module of its own,
re-exported from `path@query@src/lib.rs`. It holds no clap type.

**A writer's gate is `model`'s**: `id` and `worklist --out` call `foundation::complete_working_tree` before they answer, per
`design@check@phases-gate-the-report`; `show`, `issues`, `tripwires` and `anchors` answer ungated, since what they print
exists whatever else the model misses.

**`path@query@src/output.rs` is a copy of the writer**: every write to stdout goes through it, and a closed stdout ends the command
with exit 2, per `design@cli@exit-code-ladder`. A test holds that of this copy.

The tests of the commands' output are the binary's, in `path@cli@tests/binary.rs`.
