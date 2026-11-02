# git

Every git invocation of the checker. What it does for a user is `path@git@README.md`; why it is
shaped this way is `register@git@design`.

**A new question to git is one more method chain on `Invocation` in `path@git@src/git.rs`**,
never a `Command::new("git")` anywhere else in this crate or in the checker's library code. `run` is
where the environment of every invocation is set, the per-user ignore file pinned and the
repository variables dropped; a child spawned past it reads another environment, per
`design@core@git-is-asked-through-one-module`.

**Every function a verdict depends on returns an `Err` on any failure of git, never an empty
answer**, per `design@core@failed-git-is-exit-two`: a listing that comes back empty reads as a
project with no documents.

**Every item public here is the checker's interface**: `path@git@src/lib.rs` re-exports it, and
`#![warn(unreachable_pub)]` refuses any other. An item is made public because a crate that calls
this one needs it, never for a test, per `design@core@api-facade`.
