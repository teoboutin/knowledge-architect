---
kind: defect
---
# `rev_parse` answers `None` for every failure of git, though its contract says only a missing revision does

## Summary

`rev_parse` in `path@git@src/git.rs` returns `None` when git fails for any reason, while its doc
comment says "Every other git failure still reaches the caller as an error". The `commits`
command reads that `None` as "this commit has no first parent", and the index's snapshot as "no
commit yet".

## Details

### What

`rev_parse(root, expression)` runs `git rev-parse --verify --quiet <expression>`, accepts exit 1 as
the answer "names nothing", and turns every other failure into `None` with `.output().ok()?`:
git missing from the path, a repository git refuses, any fatal error. Its type,
`Option<String>`, has no room for the error the doc comment promises.

Callers outside the tests, found with `git grep -n 'rev_parse(' crates/`:

- `commits` in `path@check@src/cli/history.rs` asks `rev_parse(root, "<sha>^")` for each commit of
  the range, and a `None` means a root commit: no parent model is built, and the commit's message
  references resolve against its own tree alone;
- `snapshot_entries` in `path@git@src/git.rs` asks `rev_parse(root, "HEAD")` for the index's
  snapshot, and a `None` means no commit yet: it skips the listing of HEAD's tree, while
  `git diff --cached` still compares the index with HEAD, so the snapshot holds only the paths
  the index changes;
- `Snapshot::object_id` in `path@check@src/extension.rs` returns it to an extension as "no object
  id".

Not established: a reproduction. Each caller runs after other invocations on the same repository
succeeded, so a failure here needs git to fail between two calls; no test makes it fail. Read from
the code by the goal reviewer of the `git` Component's draft goals, during the work that created
the git Component; the effect on the index's snapshot was read from
`snapshot_entries` and its `git diff --cached` reproduced by the code-claims reviewer of that
work's review, in a repository with two files committed and a third staged, where the diff lists
the staged file alone.

### Why it matters

`design@git@failed-git-is-an-error` holds that every invocation a verdict depends on fails as an
error carrying git's reason, never as an empty answer, against
`goal@check@document-rules-are-checked`. A first parent lost to a failed invocation changes what
`commits` judges a message against, and a HEAD lost to one makes `check --staged` judge a tree
missing every file the index does not change; each run reports a verdict where it should exit 2.

### What would close it

`rev_parse` returns `io::Result<Option<String>>`: `Ok(None)` for exit 1 alone, `Err` with git's
reason otherwise, and each caller routes the error, `commits` to exit 2. A unit test in
`path@git@src/git.rs` runs it on a directory outside every repository and asserts an `Err`, shown to
fail against the current body.
