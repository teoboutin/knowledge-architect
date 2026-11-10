# knowledge-architect-transcript

The reader of the session transcripts Claude Code writes on disk: a project's sessions listed, one shown, or searched, each
line labelled by what the harness recorded of it, and every line or value it does not know counted and reported, never a failure.

A project that uses knowledge-architect names the package `knowledge-architect` alone, whose binary `klarch` offers the
command `transcript`, and never names this crate; its usage is `path@cli@README.md`. A Rust program that reads transcripts
depends on it directly; its interface is at <https://docs.rs/knowledge-architect-transcript>.

## Who it serves

knowledge-architect serves one kind of project, a target project, which holds all of these, per
`goal@knowledge-architect@serves-one-kind-of-project`:

- the code it builds and ships is Rust, built by Cargo, apart from the code in another language that a platform it is built for requires;
- AI agents drive its development: its code, its documents, its entries and its reviews;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect.

It asks more of a target project than these conditions: among what it asks, the project is a git
repository, and its agents run under the `claude` harness. Nothing is guaranteed outside these
conditions, or where an ask is not followed.

## What a program does with it

Call `transcript` with the project's root and a verb, `List`, `Show` or `Search`. It prints as it works and returns its exit
code. It reads the directory of Claude Code's transcripts, and from a linked worktree it also reads the sessions of the same
path in the main checkout.

Why it is shaped this way is `register@transcript@design`.
