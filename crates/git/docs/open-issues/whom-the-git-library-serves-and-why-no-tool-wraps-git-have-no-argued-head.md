---
kind: design
---
# Whom the git library serves, and why no tool wraps git itself, are argued by no head

## Summary

Two statements about the `git` crate bound the project while a goal of the Component held them,
and no design head states either of them. `design@knowledge-architect@git-crate` is the head on their
subject: it argues the crate against the checker's git module made public, and names the checker's
library and five crates. Nobody has argued the two statements against a rival.

## Details

### What

**The instances.** The statements, as the Component's goal held them:

1. "A tool here is any code of knowledge-architect that needs to interact with git: what a project
   uses, what maintains knowledge-architect itself, and the tests of either, such as a checker of a
   project's documents, a tool offering git history rewriting utilities, a gate that asks git a
   question, or a test that builds a repository as its fixture." It bound whom the crate serves:
   maintenance code, gates and tests as well as the checker's library.
   `design@knowledge-architect@git-crate` has the crate wrap git "for the checker's tools" and
   holds no wider range, and `design@knowledge-architect@checker-asks-git-through-the-crate`
   lets a test of the checker run git itself.
2. "Through the library, such a tool both reads what a repository holds and has git write, so no
   tool needs a wrapper of git of its own." It bound the library's interface to hold what each such
   tool needs. `issue@git@tools-run-git-past-the-wrapper` lists what the interface lacks today.

**Suspected mechanism**: the two were written as a goal of the Component, which bound outright and
needed no argument, so none was written against a rival when the crate was split out.

**What a reading of the record found**: the argument of `design@knowledge-architect@git-crate`
against `rejected@knowledge-architect@git-module-made-public` has a tool that needs git reach it
without the checker's parsers, which is not statement 2: twelve files outside the crate start git
themselves, as `issue@git@tools-run-git-past-the-wrapper` lists, and `mutate`, `gates` and the
maintenance tool declare no dependency on the crate. Statement 1 is wider than that head, which
names the checker's tools alone. A third statement the goal held, that the library knows nothing of
a project's documents, stands on that argument and on `path@git@Cargo.toml`, and is in
`design@knowledge-architect@git-crate`.

**The re-entry point**: the discussion that settles `issue@git@tools-run-git-past-the-wrapper`,
which has to say which sites move onto the library, and so whom it serves.

### Why it matters

`design@knowledge-architect@checker-asks-git-through-the-crate` leans on statement 2 for
`transcript`, on a rule a reader cannot find argued. A session that proposes a wrapper of git in
another crate meets no argument to defeat.

### What would close it

A discussion under `skill@knowledge-architect-design` argues each statement against its nearest
rival: for statement 1, a crate that serves the checker's library alone; for statement 2, a wrapper
of git in each tool that needs one. Each statement that stands on a fact enters `design@knowledge-architect@git-crate`
or a head of `register@git@design`, and each that does not leaves.
