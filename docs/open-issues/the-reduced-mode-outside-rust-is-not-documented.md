---
kind: todo
---
# No document tells a project in another language than Rust what its reduced mode lacks

## Summary

`goal@knowledge-architect@rust-projects-come-first` gives a project in another language than Rust
a reduced mode: it adopts the checker and the workflow without what depends on Rust. No document
says what that mode lacks, so such a project learns it only by meeting each gap.

## Details

### What

A statement, in the documentation a project reads when it adopts the workflow, of what a project
in another language than Rust does not get. Two gaps the owner named when stating the goal:

- **References in source files.** The checker reads Markdown, and in Rust source the comments and
  the unbound string literals, per `primer@checker-model`. A reference in a comment of any other
  language is checked by nobody.
- **Pinning the checker.** A Rust project pins the binary through its maintenance crate, per
  `skill@knowledge-architect-setup@pin-the-checker`; any other project installs a binary of the
  pinned version itself, which nothing keeps in step with the pin but the version refusal.

The owner said a few other things could be missing as well; they are not established. Others may
be: `klarch init --rust` and the gates library, `knowledge-architect-gates`, serve Rust alone.

### Why it matters

`goal@knowledge-architect@adoption-is-easy` is met while the installed skills and the core's README
state the whole of adopting it, and `goal@knowledge-architect@rust-projects-come-first` is met only
while the core's README and the setup skill state what a project in another language adopts. A
project in another language that relies on a reference in its source being checked finds out only
when a reference goes stale unreported.

### What would close it

A survey of every feature that depends on the project's language, and a section of the core README
and of `skill@knowledge-architect-setup` listing what a project in another language lacks, each
item citing where the Rust project gets it.
