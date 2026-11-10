# git

The library that wraps git for the tools of knowledge-architect. What it is for is
`design@knowledge-architect@git-crate`; what it does for a user is `path@git@README.md`; why it is shaped this way is
`register@git@design`.

**A new question to git is one more method chain on `Invocation` in `path@git@src/git.rs`**,
never a `Command::new("git")` anywhere else in this crate or in the library code of a tool of knowledge-architect that names it. `run`
is where the environment of every invocation is set, the per-user ignore file pinned and the
repository variables dropped; a child spawned past it reads another environment, per
`design@git@git-is-asked-through-one-module` and, for the checker,
`design@knowledge-architect@checker-asks-git-through-the-crate`.

**Every function a verdict depends on returns an `Err` on any failure of git, never an empty
answer**, per `design@git@failed-git-is-an-error`: a listing that comes back empty reads as a
project with no documents. `rev_parse` breaks it today, per
`issue@git@rev-parse-answers-none-for-every-git-failure`.

**Every item public here is the library's interface**: `path@git@src/lib.rs` re-exports it, and
`#![warn(unreachable_pub)]` makes any other a warning the clippy gate refuses. An item is made
public because a tool that calls this library needs it, never for a test of this crate, as
`check`'s facade is shaped, per `design@check@the-facade-publishes-what-a-consumer-needs`.
