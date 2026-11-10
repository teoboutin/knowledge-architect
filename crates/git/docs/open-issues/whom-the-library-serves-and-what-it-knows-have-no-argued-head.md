---
kind: design
---
# Whom the git library serves, why no tool wraps git itself, and that the library knows nothing of a project's documents are argued by no head

## Summary

Three statements about the `git` crate bound the project while a goal of the Component held them,
and no design head states any of them. `design@knowledge-architect@git-crate` is the head on their
subject: it argues the crate against the checker's git module made public, and names the checker's
library and five crates. Nobody has argued the three statements against a rival.

## Details

### What

**The instances.** The statements, as the Component's goal held them:

1. "A tool here is any code of knowledge-architect that needs to interact with git: what a project
   uses, what maintains knowledge-architect itself, and the tests of either, such as a checker of a
   project's documents, a tool offering git history rewriting utilities, a gate that asks git a
   question, or a test that builds a repository as its fixture." It bound whom the crate serves:
   maintenance code, gates and tests as well as the checker's library.
   `design@knowledge-architect@git-crate` speaks of "Every git invocation of the checker's library"
   and holds no such range, and `design@knowledge-architect@checker-asks-git-through-the-crate`
   lets a test of the checker run git itself.
2. "Through the library, such a tool both reads what a repository holds and has git write, so no
   tool needs a wrapper of git of its own." It bound the library's interface to hold what each such
   tool needs. `issue@git@tools-run-git-past-the-wrapper` lists what the interface lacks today.
3. "It knows nothing of a project's documents or of the workflow, and depends on no part of
   knowledge-architect that does." It bound the crate's dependencies and what its functions read.
   `path@git@Cargo.toml` declares no dependency on a crate of knowledge-architect; no test holds
   that. The nearest recorded statement is the reason of
   `rejected@knowledge-architect@git-module-made-public`: "A tool that needs git would depend on
   the whole checker to reach it".

**Suspected mechanism**: the three were written as a goal of the Component, which bound outright and
needed no argument, so none was written against a rival when the crate was split out.

**What a reading of the record found**: the argument of `design@knowledge-architect@git-crate`
against `rejected@knowledge-architect@git-module-made-public` covers statements 2 and 3, since a
tool that reaches git through the checker builds the checker's parsers. Statement 1 is wider than
that head, which names the checker's library alone.

**The re-entry point**: the next change that edits `design@knowledge-architect@git-crate`, or the
discussion that settles `issue@git@tools-run-git-past-the-wrapper`, which has to say which sites
move onto the library.

### Why it matters

`design@git@an-operation-in-git-s-terms-is-the-library-s` applies statement 3 to the interface, and
`design@knowledge-architect@checker-asks-git-through-the-crate` leans on statement 2 for
`transcript`. Each rests on a rule a reader cannot find argued. A session that proposes a function
of the library that reads a project's manifest, or a wrapper of git in another crate, meets no
argument to defeat.

### What would close it

A discussion under `skill@knowledge-architect-design` argues each statement against its nearest
rival: for statement 1, a crate that serves the checker's library alone; for statement 2, a wrapper
of git in each tool that needs one; for statement 3, a library that reads the manifest to find a
project's root. Each statement that stands on a fact enters `design@knowledge-architect@git-crate`
or a head of `register@git@design`, and each that does not leaves.
