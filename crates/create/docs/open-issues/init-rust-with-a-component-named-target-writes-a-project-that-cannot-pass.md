---
kind: defect
---
# `init --rust --component target` writes a project that cannot pass, and blames the Component's existence

## Summary

`init --rust` writes `/target` in `.gitignore`. A Component at `target` is then ignored, so the walk never holds it, and the
closing check reports that it does not exist, while its files lie on disk. It predates the slice whose adversarial review
found it.

## Details

### What

Reproduced with `klarch init c4 --rust --component target`, in an empty directory outside any repository: exit 1, with
`target  \`target\` is declared an anchor and does not exist`. `c4/target` holds `CLAUDE.md`, `README.md` and `docs`, and
`c4/.gitignore` holds `/target`.

`refuse_ignored` of `path@create@src/command.rs` runs only when `init` does not run git, on the ground that a repository
`init` creates holds no rule yet, which `--rust` makes false, since `init` writes that rule itself. `path@cli@tests/init.rs`
holds no test naming a `target` Component.

### Why it matters

The exit code is honest, exit 1 after a write. But the repair the finding names, creating the directory, cannot work, and the
refusal that exists for this case, with nothing written, is skipped. `design@create@init-asks-no-confirmation` rests on a refusal
coming before the first write.

### What would close it

`init` refuses a `--component` the rules it is about to write would ignore, before any write, with a test. It is one of the
refusals `spec@component-split@consumers-out` moves to `create`.
