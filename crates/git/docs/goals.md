# Goals — git

Every goal here serves one purpose: one library wraps git for every tool of knowledge-architect that
needs to interact with git.

## Every tool of knowledge-architect that needs to interact with git can do so through one library that wraps git alone `##a-pure-wrapper-over-git`

A tool here is any code of knowledge-architect that needs to interact with git: what a project
uses, what maintains knowledge-architect itself, and the tests of either, such as a checker of a
project's documents, a tool offering git history rewriting utilities, a gate that asks git a
question, or a test that builds a repository as its fixture. Through the library, such a tool both
reads what a repository holds and has git write, so no tool needs a wrapper of git of its own.

The library wraps git alone: it models operations on a repository, each stated in git's own terms,
such as the paths, objects, commits, refs and index it reads or writes, however many git commands
one takes, such as rewording a commit's message, or folding a fixup commit into the commit it
fixes. A step that decides for the matter a tool works on, such as which commits to fold or what a
check of the result must find, is the tool's, composed from those operations, however many tools
share it. It knows nothing of a project's documents or of the workflow, and depends on no part of
knowledge-architect that does.

It is met while the library's public interface lets each such tool run every git invocation it
needs and read what a repository holds, holds each operation in git's terms that a tool composes
from several invocations, has no function that reads a project's documents or decides for the
matter a tool works on, and its dependencies hold no part of knowledge-architect that knows a
project's documents or the workflow.

- Requires, for instance: the library offers folding a fixup commit into the commit it fixes as one
  operation.
- Admits, for instance: a tool that runs git itself, past the library.
- Rules out, for instance: a function of the library that chooses which commits a tool folds.
