---
kind: todo
---
# The walk does not report the untracked files git's per-clone exclude file removes from it

## Summary

The walk is git's listing with the standard ignore rules, which include the per-clone exclude file,
`.git/info/exclude`. Git offers no way to pin that file, so a line in it takes an untracked
document out of every check on one clone and not on another, with both runs green. No check
reports which files it removed. The missing work is that check. Until it exists,
`tripwire@core@walked-count-differs-between-machines` watches the symptom.

## Details

### What

`git::entries` in `path@core@src/git.rs` lists the untracked part of the walk with
`git ls-files -z --others --exclude-standard`. `--exclude-standard` applies three sources of
ignore rules: the tree's own `.gitignore` files, `core.excludesFile`, and the per-clone exclude
file `.git/info/exclude`. Every git invocation already pins `core.excludesFile` to the null
device, per `design@core@per-user-ignore-file-pinned`. The per-clone exclude file stays, since it
lives in the clone and git has no option to ignore it.

The check to write: at the walk, list the untracked files that the per-clone exclude file alone
removes, and report them, in the summary block or as a finding. One way to compute the set, on a
single clone with no second machine: the files that
`git ls-files -z --others --exclude-per-directory=.gitignore` lists and that
`git ls-files -z --others --exclude-standard` does not. Measured with git 2.53.0, in a new
repository holding untracked `a.md`, `b.md` and `c.md`, a `.gitignore` naming `b.md` and a
`.git/info/exclude` naming `c.md`, each listing run with `-c core.excludesFile=/dev/null`: the
first lists `.gitignore`, `a.md` and `c.md`, the second `.gitignore` and `a.md`, so the difference
is `c.md` alone. Not established: the same on the other git versions from the floor, 2.36, up;
and whether the report is a finding or a line of the summary block. A check may spawn nothing, so the second listing is taken by the caller and handed in, as
the walk is.

The check runs at every `check`, in every project.

Related, other causes of a walk that differs between machines, each with its own entry:
`issue@core@git-environment-variables-redirect-the-walk` and
`issue@core@the-tests-read-the-machine-s-git-configuration`.

### Why it matters

`design@core@git-supplies-the-walk` makes git's listing the walk, and
`design@core@per-user-ignore-file-pinned` pins the per-user ignore file so that the walked set is a
property of the commit rather than of the machine. The per-clone exclude file is the one ignore
source left that is not the commit's: a document it removes is out of every check on that clone, and
the summary block's `walk: n file(s)` line differs from CI's count for the same commit with no
finding. This threatens `goal@core@documents-stay-current`, met while the checker's verdict fails on
a tree where any of its three conditions fails: a pass on one clone can judge fewer documents than
the same commit holds.

### What would close it

The check above, written with a test that adds a line to a sandbox's `.git/info/exclude` and
asserts the report names the file it removes; and the sentence of
`design@core@per-user-ignore-file-pinned` that names this entry, and the comment in
`Invocation::run` of `path@core@src/git.rs` on what still decides the walk, repointed to the check;
the commit that writes it narrows `tripwire@core@walked-count-differs-between-machines` to the
cause no check reports, two git versions that walk one commit differently.
