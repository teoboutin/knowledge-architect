---
kind: todo
---
# Tools of knowledge-architect run git past the wrapper, whose public interface lacks what they need

## Summary

The git library's public interface does not let every tool of knowledge-architect run every git
invocation it needs: it keeps private the environment of an invocation, its standard input and its
exit code as an answer. Twelve files of the other crates' code, their tests included, start git
themselves.

## Details

### What

What the public interface lacks, in `path@git@src/git.rs`: `Invocation::env`, `Invocation::stdin`
and `Invocation::code` are `pub(crate)`, and so is `Invocation::paths`. A test that commits needs
an identity in the environment; a gate reads the exit code as its answer.

The code outside the crate that starts git itself, found with
`grep -rln 'Command::new("git")' crates tools --include=*.rs`, the git crate's own files left out:

- library code: `path@mutate@src/run.rs` (`git init` and `git apply` in a scratch directory) and
  `path@mutate@src/harness.rs` (`rev-parse --git-path`);
- tests: `path@mutate@tests/harness.rs`, `path@mutate@tests/modes.rs`,
  `path@testing@tests/extension_api.rs`, `path@testing@tests/checks_over_mocks.rs`, `path@cli@tests/init.rs`, `path@cli@tests/fixture_pin.rs`, `path@testing@tests/binary.rs`, the
  unit test of `path@model@src/source/rs.rs`, `path@xtask@tests/gates_bin.rs` and
  `path@xtask@tests/mutate_bin.rs`.

The gates library's `rebased` gate runs `git merge-base --is-ancestor` as a gate's command line,
through its own spawn helper, in `path@gates@src/gates.rs`.

`design@git@an-operation-in-git-s-terms-is-the-library-s` makes an operation in git's terms that a
tool composes from several invocations a function of the library, so these sites' operations are
owed to the library. No decision has these sites move onto it:
`design@knowledge-architect@checker-asks-git-through-the-crate` binds the library code of the
checker's tools alone, and lets a test of the checker run git itself.

Two items of the library's interface may serve the checker rather than git: `component_dir` and
`CRATE_DIR` in `path@git@src/lib.rs` exist for the checker's reading of its own source's literals
as data and for its refusal of a build from another checkout; and `stage_generated` names its files
by the checker's word, though it stages any contents a caller gives. That the library knows nothing
of a project's documents or of the workflow is a rule no head states yet, per
`issue@git@whom-the-library-serves-and-what-it-knows-have-no-argued-head`.

### Why it matters

A tool that cannot reach an invocation through the library keeps a wrapper of git of its own, and
the environment the library sets for every invocation, per
`design@git@repository-variables-dropped`, reaches none of it. That no tool needs a wrapper of its
own is a reason for the crate that no head argues yet, per
`issue@git@whom-the-library-serves-and-what-it-knows-have-no-argued-head`.

### What would close it

The public interface of the library carries what these sites need, each addition argued against
the design heads of `register@git@design`; each site either moves onto it or is left with the
reason it stays, in a comment at the site; and the two items above are renamed, moved out of
the library's interface, or kept with the argument that they fit
`design@git@an-operation-in-git-s-terms-is-the-library-s`, as their design decides: the owner ruled that an item shaped for its first caller, `component_dir` and `CRATE_DIR`
among them, knows nothing of a project's documents.
