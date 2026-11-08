---
kind: defect
---
# `init` accepts an ignored destination, and an ignored Component where it creates the repository, and `new-component` accepts an ignored directory

## Summary

`init` asks git about PATH and each Component only where it creates no repository, and about no file it writes, so a run can pass over a project whose manifest no commit will hold, or end on a Component the check reads as missing. Each predates the move to `create`, whose adversarial review found them.

## Details

### What

Reproduced with the binary, in a repository whose `.gitignore` names the path:
- a manifest ignored: `init . --name p` exits 0, `PASSED: no findings`; after `git add -A` and a commit, `check` in a clone exits 2;
- `docs/` ignored: exit 1, with findings that the plans directory does not exist while it is on disk;
- `.cargo/` and `Cargo.toml` ignored under `--rust`: exit 0, and the aliases behind `cargo klarch` never reach a commit;
- a Component ignored by a `.gitignore` PATH already holds, where `init` creates the repository: `printf 'svc/\n' > q/.gitignore`, then `klarch init q --vcs git --component svc`, outside every repository, exits 1 after every write with `svc` reported as not existing; with `--rust`, a `.gitignore` holding `target` and `--component sub/target` do the same. `refuse_ignored` of `path@create@src/command.rs` runs only where `init` runs no `git init`, and a PATH that holds files outside every repository reaches `git init` only under `--vcs git`;
- `new-component build` with `build/` ignored: exit 1 with `build` reported as not existing, having written the manifest, where `init --component build` refuses, exit 2.

`init --rust` over a legacy cargo configuration, a file named `config` beside where it writes `config.toml`, also exits 0 and passes, and `cargo klarch` then fails with `no such command`, since cargo prefers that file; `rust_files` refuses an existing `config.toml` there for this reason.

### Why it matters

`design@create@init-asks-no-confirmation` refuses a PATH or a Component git ignores "whose files the check would read as missing with no repair but the rule", which holds of each file written, and `goal@knowledge-architect@every-target-project-can-adopt-it` asks that setup lead a new project to a passing check.

### What would close it

`init` and `new-component` refuse each destination git ignores before the first write, and `--rust` refuses that legacy file, with a test per case.
