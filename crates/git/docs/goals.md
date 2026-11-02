# Goals — git

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@git@design`. A goal stays here while it is met, and leaves only when the owner
abandons it. This component is published, so its goals are encouraged to refine a goal of the
project as a whole, `register@knowledge-architect@goal`. Every goal here serves one purpose: one
library wraps git for every tool of knowledge-architect that needs to interact with git.

## Every tool of knowledge-architect that needs to interact with git can do so through one library that wraps git alone `##a-pure-wrapper-over-git`

A tool here is any code of knowledge-architect that needs to interact with git: what a project
uses, what maintains knowledge-architect itself, and the tests of either, such as a checker of a
project's documents, a tool offering git history rewriting utilities, a gate that asks git a
question, or a test that builds a repository as its fixture. Through the library, such a tool both
reads what a repository holds and has git write, so no tool needs a wrapper of git of its own.

The library wraps git alone: it models git's operations that are basic or reusable, and no
procedure of several operations aimed at one subject or one use, such as a fold of a repair into
an earlier commit followed by a check of the tree it leaves, which the tool that offers the
procedure composes from those operations. It knows nothing of a project's documents or of the
workflow, and depends on no part of knowledge-architect that does.

It is met while the library's public interface lets each such tool run every git invocation it
needs and read what a repository holds, and its dependencies hold no part of knowledge-architect
that knows a project's documents or the workflow.
