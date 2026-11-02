# knowledge-architect-git

Asks git what the checker of knowledge-architect reads: the listing a walk reads, the ignore rules,
the dates of a listing's last change, and the trees, blobs and messages of a commit or of the tree
git's index would commit. Every answer comes from `git` run as a child process; a failure is an
error carrying git's reason, never an empty answer.

The checker, the package `knowledge-architect`, depends on it; a project names that package alone.

## Documentation

- The library API: [docs.rs](https://docs.rs/knowledge-architect-git).
- The [repository README of this crate](https://github.com/teoboutin/knowledge-architect/blob/main/crates/git/README.md).
- The project: [the repository](https://github.com/teoboutin/knowledge-architect).
