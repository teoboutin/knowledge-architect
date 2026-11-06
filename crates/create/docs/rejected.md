# Rejected designs

The designs create did not keep, and why.

### A confirmation prompt before `init` writes `##init-asks-for-confirmation`

`live`. It guards against no loss: `init` refuses before its first write and overwrites no
hand-written file outside the installer's namespace, which every install rewrites. It blocks an agent, whose shell has no terminal, or is answered by a `--yes`
every caller passes. `cargo init` asks nothing, measured with cargo 1.98.0.

### A hook through which an extension contributes its manifest tables to `init` `##init-extension-hook`

`live`. It would add a hook to the extension interface for a case the check already reports with
its repair, and a project with an extension would not need to run `init`, on the owner's word.

### `init` running `git init` in any directory outside a worktree, as `cargo init` does `##init-git-init-anywhere-outside-a-worktree`

`live`. In a directory that already holds files, the files there become untracked beside the
project's, as `git status --short` after `cargo init` shows, so `git clean`, the obvious cleanup of
a mistaken run, deletes them too. The owner chose a refusal there over following `cargo init`.

### `init` refused wherever PATH is outside a git repository, or not its root `##init-refused-outside-a-repository-root`

`live`. A project in a subdirectory of a repository passes the check, so a refusal there refuses a
legal layout, and a new or empty directory outside every repository loses nothing to `git init`.
The kept rule does not spare every mistaken run a messy cleanup, and
`issue@create@init-outside-a-worktree-deletes-an-owned-file-silently` reopens the question, with this design among its candidates.
