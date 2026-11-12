---
kind: defect
---
# A shallow clone that holds the base shortens the range `commits` judges, with no error

## Summary

In a shallow clone, `git rev-list <base>..HEAD` stops at the shallow boundary and exits 0. The
range `cargo klarch commits` judges is then shorter than the branch, and the run passes over the
commits beyond the boundary without saying so. The commits it never judged can carry any finding.

## Details

### What

`commits` in `path@check@src/commands/history.rs` takes its commits from `rev_list` in
`path@git@src/git.rs`, which runs `git rev-list --reverse --topo-order <range>` and reads its
output. A shallow clone has no parents beyond its boundary, so git lists the commits it holds
between the boundary and HEAD, exits 0, and prints nothing on stderr. `rev_list` returns that list
as the range.

Measured with git 2.53.0, on a repository holding `main` with one commit and a branch `feat` with
2 commits over `main`:

```sh
git clone --depth 1 --no-single-branch file://<origin> clone
cd clone && git checkout feat
git rev-parse --is-shallow-repository    # true
git rev-list origin/main..HEAD           # exits 0, lists 1 commit; the full clone lists 2
git merge-base --is-ancestor origin/main HEAD   # exits 1
```

The same repository, built from a copy of the mock project `path@testing@tests/projects/minimal/`
with its `checker-version` set to the binary's version: `klarch commits origin/main..HEAD` in the
shallow clone prints "1 commit, 1 passed, 0 failed" and `PASSED: no findings`, exit 0; in the full
repository, `klarch commits main..feat` prints "2 commits, 2 passed, 0 failed".

Ruled out:

- The base missing from the clone: here `origin/main` resolves, since `--no-single-branch` fetches
  every branch's tip. A base that does not resolve already fails, as an `Err` carrying git's reason.
- This repository's CI: `path@knowledge-architect@.github/workflows/ci.yml` checks out with
  `fetch-depth: 0`, so its run is a full clone. Whether a project's own CI, or a session's checkout
  in a cloud container cloned to a fixed depth, runs `commits` in a shallow clone is not
  established.
- The gate `rebased` of `cargo x gates --require-rebased`, in `path@gates@src/gates.rs`, asks
  `git merge-base --is-ancestor`, which exits 1 in the shallow clone above, so that gate fails
  there. A run without `--require-rebased`, and `cargo klarch commits` run alone, are not guarded.

### Why it matters

The range check before the merge is where the guarantee of
`design@check@a-commit-message-is-a-document` lies: a message with a finding never reaches the
main branch, since every commit of the branch is judged. A shorter range gives that guarantee for
part of the branch and reports the verdict of the whole. It strains
`design@git@failed-git-is-an-error`: an invocation a verdict depends on gives an incomplete answer,
and the caller cannot tell it from a complete one. A run that passes while it skipped commits
reads as a clean verdict, against `goal@documentation-stays-consistent`.

### What would close it

`rev_list`, or `commits`, refuses a range it cannot list in full, as an `Err` carrying the reason
and the repair, such as `git fetch --unshallow`; and a test over a shallow clone of a repository
built in the test shows the refusal. Two candidate checks, the choice left open:

- refuse whenever `git rev-parse --is-shallow-repository` answers true; simple, and it refuses a
  shallow clone whose boundary lies below the base, where the range is complete;
- refuse only when the range's base is not an ancestor of HEAD reachable in the clone, such as by
  `git merge-base --is-ancestor`; it refuses exactly the incomplete ranges of a shallow clone, and
  also a branch that is not rebased on the base, which then needs its own repair named.
