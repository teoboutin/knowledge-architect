---
kind: defect
---
# A git variable in the environment, such as `GIT_DIR` or `GIT_INDEX_FILE`, makes every command read another repository's files

## Summary

Every git invocation of the checker inherits the caller's environment, so with `GIT_DIR`,
`GIT_WORK_TREE` or `GIT_INDEX_FILE` set, the last as git sets it for a pre-commit hook, `check`
and every other command read the index and the tree those variables name rather than the
project's. `init` refuses to run with any of them set; no other command does.

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

`GIT_INDEX_FILE` misdirects the walk the same way: `GIT_INDEX_FILE=$PWD/other/.git/index klarch
init g1`, run with `init`'s refusal of it absent, exits 1 on the same finding about `f`. A
pre-commit hook gets `GIT_INDEX_FILE` and neither `GIT_DIR` nor `GIT_WORK_TREE`, measured with git
2.43.0 by a hook running `env`: `.git/index` for `git commit`, a path relative to the repository's
root, and the absolute path of `.git/index.lock` for `git commit -a`, each the index the commit
records.

Not established: what each other command does under the variables; whether `GIT_COMMON_DIR` or
`GIT_OBJECT_DIRECTORY` misdirect the walk the same way; and what a hook's relative
`GIT_INDEX_FILE` names for a project in a subdirectory of its repository, where git runs with the
project's root as its working directory. Ruled out: the per-user ignore file, which
`Invocation::run` already pins away with `core.excludesFile=/dev/null`.

### Why it matters

`design@core@git-supplies-the-walk` makes git's listing of the project the walk; under these
variables the listing can be another repository's or another index's, so a check run where they are
set can judge the wrong tree, and can pass or fail regardless of the project. It strains
`goal@core@documents-stay-current`, met while the checker's verdict fails on a tree where a
reference, an entry's shape or a generated listing fails: here the verdict can pass on a tree it
never read.

### What would close it

`Invocation::run` removes the git variables that select a repository, `GIT_DIR`, `GIT_WORK_TREE`,
`GIT_INDEX_FILE`, `GIT_COMMON_DIR` and `GIT_OBJECT_DIRECTORY` at least, from every invocation's
environment, or the binaries refuse to run with them set, as `init` does; with a test that runs
`check` with `GIT_DIR` naming another repository and asserts the project's own verdict. The
`init` refusal in `path@core@src/cli/create.rs` then follows whichever is chosen.
