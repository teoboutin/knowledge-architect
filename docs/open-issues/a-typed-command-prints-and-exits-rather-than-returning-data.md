---
kind: todo
---
# A typed command prints and can end the process, rather than returning its data

## Summary

Each command's function, in `path@check@src/commands/mod.rs` and in `query`, `transcript` and `create`, takes the domain's types and names no clap type, and still
prints as it works, returns `Result<ExitCode, String>`, and ends the process with exit 2 on a closed stdout. A
frontend that is no command line can call it, and receives stdout text and a process exit. On the owner's word,
refactoring the commands so that they return data is recorded here, to be done later.

## Details

### What

Measured with `grep -cE 'out(ln)?!\(' crates/{check/src/commands,query/src,transcript/src,create/src}/*.rs`: 98 lines
print through `out!` or `outln!`, 30 in `check`, 42 in `query`, 19 in `transcript` and 7 in `create`. Each of `check`, `query` and
`transcript` holds a writer, and `create` a macro over `check`'s. Three things stand in the way of returning data:

- some of those lines sit inside a write loop, as `index_working` and `fix_then_check` print each file as they write it;
- each function chooses between exit 1 and exit 2 itself, as `fix_then_check` does from how many files it wrote;
- `write` of `path@check@src/commands/output.rs`, and each copy of it, calls `std::process::exit(2)` on a closed stdout, per
  `design@cli@exit-code-ladder`.

Ruled out at the design audit of the slice that moved the binary to the top: moving `output` to `cli` alone, since
`cli` sits above this crate, which could not then print.

### Why it matters

`design@knowledge-architect@a-typed-command-prints-and-returns-its-code` states the choice this entry would reverse, and
`design@agent-skills@top-level-binary-crate` argues that a frontend that is no command line gets a typed API. Until this is
done that holds in part: the frontend names no clap type, and gets no value it can lay out itself. A test of a command's
outcome also reads printed text, or runs the binary, where it could read a value. The session's choice to keep the
printing, which the owner approved, weighed a rewrite of these sites against a slice whose first step was the
mechanism alone, and no goal ranks the two.

The owner wants findings returned in the end, and not only printed, and rests that on the development guidelines rather than on a
goal of its own. The default shape of `design@agent-skills@default-code-shape` supports it: "the logic pure, the side effects in a
thin layer that decides nothing a test of the logic should cover", where a command that prints inside its write loop, chooses its
exit code and ends the process holds side effects in its logic. A guideline is a default, so it ranks the shapes as a default
does and no goal ranks them.

`issue@knowledge-architect@command-output-is-not-declared-a-contract` holds the question of which output is a contract;
a command that returns data offers one by a library function.

### What would close it

Each typed command returns a value holding what it found and what it wrote, `cli` prints it and maps it to an exit
code, and `write`'s exit lies in `cli`. The binary's suite holds the output and the codes unchanged, the order of what
`check --fix` lists among them. It can be done one command at a time, and the copies of `write` leave with the last.
