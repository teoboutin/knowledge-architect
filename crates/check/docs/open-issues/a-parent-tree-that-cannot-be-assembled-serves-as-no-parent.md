---
kind: defect
---
# `commits` reads a parent tree it cannot assemble as no parent, in silence, so a message's reference into that tree dangles

## Summary

`commits` judges a message's references against its commit's tree or its first parent's. Where it
builds the parent's tree itself, at the range's first commit and wherever the previous commit is
not the first parent, it drops every failure to assemble it with `.ok()`: a git read that fails, an
ignore batch git refuses, or an extension that cannot prepare leaves the message judged against its
own tree alone. A reference into the parent, such as the closing of an issue the commit deletes,
then dangles, with a repair that tells the writer to define the entry the commit rightly deleted,
and nothing names the parent's failure.

## Details

### What

In `commits` in `path@check@src/commands/history.rs`, `parent_owned` calls `commit_tree` over
`Source::Commit(parent)` at `Depth::References`, then `.ok()`, then filters out a tree that stopped
at phase 1. Every `Unloadable` that `read_tree` or `commit_tree` returns becomes `None`: a parent
holding no manifest, one holding it as a link, one whose manifest does not parse, a git read that
fails, a refused ignore batch, and an extension that cannot prepare. Only the first is a parent that
holds no project; the others are a parent the run could not read.

Reproduced by hand with git 2.53.0 and the binary built from the tree, over a copy of the `dirhome`
mock of `path@testing@tests/projects/`, its pin set to the binary's version, in a new repository:

1. add a file `link/data.txt`, append to the mock's README.md a line holding a path reference at
   the anchor `dirhome` with the path `link/data.txt`, and commit;
2. delete the issue entry `an-ungrouped-entry.md` with `git rm`, remove the README.md line, run
   `klarch index`, and commit with a message whose body closes that entry by its reference, of kind
   `issue` at the anchor `dirhome`. `klarch commits HEAD~1..HEAD` passes, exit 0: the parent's tree
   defines the entry;
3. replace the directory `link` by a symbolic link to `docs`, add `skip-files = ["link"]` to the
   manifest, and commit. `klarch check` passes. `klarch commits HEAD~2..HEAD` fails the second
   commit, exit 1, with one finding on its message line 3: the reference "is referenced and
   `dirhome` defines no issue", and the repair "define it in the open-issues home of `dirhome`, or
   repair the reference".

The base commit, the parent of the range's first commit, holds the README.md span through
`link`, and git refuses its ignore batch against the working tree, per
`issue@model@a-path-git-reads-as-a-pathspec-stops-the-run-or-misreads`. The run reports nothing of
that refusal, since the base is outside the range.

Ruled out: a parent inside the range that fails to assemble is reported, as a finding on that
commit, "this commit's tree does not load under the tip checker"; the next commit is still judged
without a parent, so its findings come beside a reported cause.

Not established: which of the causes other than the refused ignore batch a run meets in practice,
and whether a parent holding no manifest, a commit from before the project adopted the workflow,
should keep serving as no parent, as it does now with no finding.

### Why it matters

It strains `design@git@failed-git-is-an-error`, whose rule is that every git invocation a verdict
depends on fails as an error carrying git's reason: here a failed read of the parent moves the
verdict and is never reported. It strains `design@check@message-references-resolve-in-either-tree`,
since the reference is judged in one tree while the head says either, and
`design@check@finding-names-the-repair`, since the repair the finding names undoes a correct
deletion.

### What would close it

`commits` tells a parent that holds no project from a parent it could not read, and reports the
second as a finding of the commit whose message it would judge, naming the parent and the reason,
with a binary test over the reproduction above.
