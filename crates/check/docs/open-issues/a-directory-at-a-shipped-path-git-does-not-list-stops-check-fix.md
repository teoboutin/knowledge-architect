---
kind: defect
---
# A directory at a shipped path that git does not list stops `check --fix` with an OS error

## Summary

Where the working tree holds a directory at the path of a shipped installed file and git lists nothing
there, `check --fix` tries to write the file over the directory and exits 2 with `Is a directory (os
error 21)`. The plain check names the cause as a missing or ignored copy. When git does list the path, the
same directory is left alone and reported, and the run exits 1.

## Details

### What

Reproduced by the adversarial reviewer of the review before the merge of the branch that added tests around
the installed files, with the binary built from that branch:

```sh
klarch init p2; cd p2; git add -A; git commit -qm init
F=.claude/agents/knowledge-architect-adversarial-reviewer.md
git rm -q $F; git commit -qm del; mkdir $F
klarch check --fix   # exit 2: error: .claude/agents/knowledge-architect-adversarial-reviewer.md: Is a directory (os error 21)
```

- `install-agent-skills` over the same tree also exits 2.
- `klarch check` reports the path as "ships this file, and git lists no installed copy: it is missing,
  or an ignore rule covers it", which names neither the directory nor its repair.
- The mechanism: `repairs` in `path@check@src/agents.rs` handles a path git does not list in its `None`
  arm, which reads the file from disk. `read_to_string` fails on a directory, so the path is pushed to
  `write`, and `fs::write` in `apply` fails. The `Some(Outside::Directory)` arm, whose comment says that
  replacing a directory "is no safe fix", is reached only when git lists the path.
- Ruled out: with the path tracked (`rm $F; mkdir $F` over a committed copy), `--fix` exits 1, leaves the
  directory and reports "this installed file differs". That message names no directory either.
- Not established: whether any other shipped file is written before the failing one in the same run. The
  writes follow the order of the shipped set, so it depends on where the path falls in it.

### Why it matters

`design@check@fix-scope` admits a repair only where it passes `design@check@safe-fix-definition`, and the
`Directory` arm already judges a write over a directory to fail it. Exit 2 promises an untouched tree, per
`design@cli@exit-code-ladder`, and a session reading it is told nothing about the directory. Both strain
`goal@agents-work-without-drift`.

### What would close it

The `None` arm leaving a directory alone as the `Directory` arm does, and both findings naming a directory
in the file's place with its repair, each with a binary test over the two cases above.
