---
kind: defect
---
# `GIT_DIR` or `GIT_WORK_TREE` in the environment makes every command read another repository

## Summary

Every git invocation of the checker inherits the caller's environment, so with `GIT_DIR` or
`GIT_WORK_TREE` set, as git sets them for a hook, `check` and every other command read the index
and the tree of the repository those variables name rather than the project's. `init` refuses to
run with either set; no other command does.

## Details

### What

`Invocation::run` in `path@core@src/git.rs` spawns `git` with `current_dir` set to the project root
and the parent's environment, removing nothing. Git honours `GIT_DIR` and `GIT_WORK_TREE` over the
working directory.

Reproduction, with the checker built from this repository, in an empty scratch directory outside
every repository:

```sh
klarch init p                      # a passing project, its own repository
git init -q other && echo x > other/f && git -C other add f
cd p && klarch check               # exit 0, PASSED
GIT_DIR=../other/.git klarch check # exit 1: "f  git lists this file and the working tree does
                                   # not hold it"; the walk is other's index
```

Not established: what each other command does under the variables, and whether `GIT_INDEX_FILE`
or `GIT_COMMON_DIR` misdirect the walk the same way. Ruled out: the per-user ignore file, which
`Invocation::run` already pins away with `core.excludesFile=/dev/null`.

### Why it matters

`design@core@git-supplies-the-walk` makes git's listing of the project the walk; under these
variables the listing is another repository's, so a check run from a git hook, a pre-commit hook
running `cargo klarch check` among them, judges the wrong tree, and can pass or fail regardless of
the project. It strains `goal@core@documents-stay-current`, whose test is the check passing on the
project's own commits.

### What would close it

`Invocation::run` removes the git variables that select a repository, `GIT_DIR`, `GIT_WORK_TREE`,
`GIT_INDEX_FILE`, `GIT_COMMON_DIR` and `GIT_OBJECT_DIRECTORY` at least, from every invocation's
environment, or the binaries refuse to run with them set, as `init` does; with a test that runs
`check` with `GIT_DIR` naming another repository and asserts the project's own verdict. The
`init` refusal in `path@core@src/cli/create.rs` then follows whichever is chosen.
