---
kind: todo
---
# References are read in Markdown and Rust files only

## Summary

The walk parses two file kinds, Markdown and Rust source. A reference written in a comment of any
other language is never read: it is not checked, `show` does not list it, and a deleted or renamed
entry leaves it dangling with no finding. A target project's files in another language, the
installed saved workflows and the code in another language that a platform it is built for
requires, get the document checks and none of the
code-comment checks.

## Details

### What

`LIVE_SUFFIXES` in `path@model@src/walk.rs` lists the suffixes the walk reads, `md` and `rs`. The
Rust grammar is what tells the reader which byte ranges of a source file are prose. Each further
language a target project holds files in, such as the JavaScript of the saved workflows or of a web
target, its HTML, or a shader language, would need a way to find its comments, and a way for the
manifest to say which of those files are read. Nothing about the shape is decided.

The installed saved workflows are JavaScript, so they are not read either. Their prompts name a
skill section in words, inside string literals: the agentic-workflow audit's `SECTION` constant
names the section
`skill@knowledge-architect-project-audit@agentic-workflow-axis`. A rename of that section
leaves the prompt pointing to nothing, with no finding. The fifth pass of the agentic-workflow
audit found one such pointer already broken, a prompt naming a point the section no longer had.

- An instance, from the branch that built the record search's saved workflow, 2026-11-16, C1 of its
  workflow file: a `%%` line of a shipped workflow script under
  `path@agent-skills@content/workflows/` held a `path` reference, which no check read; two review
  axes found it, and it was removed. The scoped rule of `path@agent-skills@CLAUDE.md`, that such a
  script "cites none", stood and was not followed. The same file kind leaves the claim of
  `design@agent-skills@content-is-in-the-walk`, that the references of the comments of content/ are
  checked, false for its saved workflows, whose `%%` lines the walk does not read.
- A further instance, in this repository's port of the gates to Windows and macOS: the spec's list
  of the texts citing the decisions its work rewrote, taken from `show`, missed the comments of
  `path@knowledge-architect@.github/workflows/ci.yml` at two heads and of
  `path@knowledge-architect@rust-toolchain.toml` at one, whose claims the work made false; four
  reviewers found them by grep. The retrospective proposed a shape narrower than reading another
  language: `show` printing, after its list, the number or the names of the tracked files it does
  not read that hold the entry's slug, which would reach YAML and TOML files as well, per C3 of the
  retrospective file 2026-11-25-knowledge-architect-windows-and-macos-gates-klarch-workflow.

### Why it matters

Every target project's code is Rust, per `goal@serves-one-kind-of-project`, but
it holds files in other languages that a reference can sit in: the installed saved workflows, which
are JavaScript in every target project that installs the workflow, and the code in another
language that a platform it is built for requires, such as the JavaScript and the HTML of a web
target. The installed skills tell an agent to name an
issue in the comment at the code that exists because of it, so that closing the issue dangles the
comment. In such a file that instruction produces references nothing reads, and the skills have to
tell the agent to grep instead. A project in another language is no target project, so it is no
case of this entry.

### What would close it

The walk reads the comments of at least one further language, chosen by the manifest, with a test
over a mock project in that language in which a dangling reference in a comment is reported. Or
the owner rules that the checker stays limited to Rust source, and the limit is recorded in `register@model@design`.
