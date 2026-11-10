# knowledge-architect-git

A library that wraps git for the tools of knowledge-architect: the checker of a project's documents
asks git through it today. A project that uses knowledge-architect names the package
`knowledge-architect` alone, which depends on this one, and never names this crate. A Rust program
can depend on it directly to run git with the same environment; its interface is at
<https://docs.rs/knowledge-architect-git>.

## What it asks

- **The walk's listing**: every entry git tracks, with its kind read off the mode the index
  records, and every file git neither tracks nor ignores; and the files git tracks that its ignore
  rules also cover.
- **The ignore rules**: one `git check-ignore` batch for every path a caller names.
- **The dates of a listing's last change**: one `git log` over the directories named.
- **History**: the revisions of a range, a revision's object name, a commit's message, the commit a
  revision names, the paths a commit and the work tree differ by, untracked ones included, the
  main worktree's path.
- **A snapshot**: the entries, blobs and blob bytes of one commit's tree, or of the tree git's
  index would commit, read from git objects alone.
- **The index**: whether it differs from HEAD, its unmerged paths, and the one locked write that
  stages generated files.

## What it needs and guarantees

- **`git` 2.36 or newer on the path** to read blobs: they are read with `cat-file --batch -z`.
- **`git` 2.30 or newer** to resolve a revision to one commit and list the paths changed since it:
  `rev-parse` reads `--end-of-options` from 2.30.
- **A project inside a git worktree**, whose `.git` sits at or above the directory it is given,
  for every question but whether a directory is inside one and the creation of a repository.
- **Every invocation runs from the directory it is given**, with the per-user ignore file pointed at
  the null device, without the variables that point git at another repository, and with
  `GIT_INDEX_FILE` kept, so a pre-commit hook reads the index its commit records.
- **A failure of an invocation a verdict depends on, such as a listing, is an `Err` carrying
  what was run and git's reason, never an empty answer.** The dates of a listing's last change
  and the main worktree's path answer nothing where git cannot say, since no verdict depends on
  them.
- **A path in a listing is git's bytes**, never decoded.

Why it is shaped this way is `register@git@design`.
