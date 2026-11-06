# knowledge-architect-git

Wraps git for the tools of knowledge-architect. Through it, the checker of a project's documents
reads the listing a walk reads, the ignore rules, the dates of a listing's last change, and the
trees, blobs and messages of a commit or of the tree git's index would commit. Every answer comes from `git` run as a child process; a failure of a
question a verdict depends on, such as a listing, is an error carrying git's reason, never an
empty answer.

The checker and the crates of its commands depend on it; a project names the package `knowledge-architect` alone.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-git).
- The [repository README of this crate](https://github.com/teoboutin/knowledge-architect/blob/main/crates/git/README.md).
- The project: [the repository](https://github.com/teoboutin/knowledge-architect).
