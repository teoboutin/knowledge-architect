# knowledge-architect-git

Every question the checker of knowledge-architect asks git, as a library. A project that uses
knowledge-architect names the package `knowledge-architect` alone, which depends on this one, and
never names this crate. A Rust program can depend on it directly to ask git the same questions
with the same environment; its interface is at <https://docs.rs/knowledge-architect-git>.

## What it asks

- **The walk's listing**: every entry git tracks, with its kind read off the mode the index
  records, and every file git neither tracks nor ignores; and the files git tracks that its ignore
  rules also cover.
- **The ignore rules**: one `git check-ignore` batch for every path a caller names.
- **The dates of a listing's last change**: one `git log` over the directories named.
- **History**: the revisions of a range, a revision's object name, a commit's message, the main
  worktree's path.
- **A snapshot**: the entries, blobs and blob bytes of one commit's tree, or of the tree git's
  index would commit, read from git objects alone.
- **The index**: whether it differs from HEAD, its unmerged paths, and the one locked write that
  stages generated files.

## What it needs and guarantees

- **`git` 2.36 or newer on the path**: a commit's tree is read with `cat-file --batch -z`.
- **A project inside a git worktree**, whose `.git` sits at or above the directory it is given.
- **Every invocation runs from the directory it is given**, with the per-user ignore file pointed at
  the null device, without the variables that point git at another repository, and with
  `GIT_INDEX_FILE` kept, so a pre-commit hook reads the index its commit records.
- **A failure is an `Err` carrying what was run and git's reason, never an empty answer.** The one
  exception is the dates of a listing's last change, which degrade to no date.
- **A path is git's bytes**, never decoded.

Why it is shaped this way is `register@git@design`.
