# knowledge-architect-create

Creates a knowledge-architect project, its manifest, the installed agent files, and every document and home the check
requires, and adds a Component to one. Every refusal comes before the first write.

A project that uses knowledge-architect names the package `knowledge-architect` alone, whose binary `klarch` offers `init` and
`new-component`, and never names this crate; their usage is `path@cli@README.md`. A Rust program that creates a project the
same way depends on it directly; its interface is at <https://docs.rs/knowledge-architect-create>.

## Who it serves

knowledge-architect serves one kind of project, a target project, which holds all of these, per
`goal@knowledge-architect@serves-one-kind-of-project`:

- the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need;
- AI agents drive its development: its code, its documents, its entries and its reviews;
- its main branch has a linear history, with no merge commit;
- its owner has decided to use knowledge-architect.

It asks more of a target project: that it uses both the workflow and the checker, git with its `.git`
at or above the project, agents under the
`claude` harness, every gate run through the gates library from one command, and one merge invocation
of that command. Nothing is guaranteed outside these conditions, or where an ask is not followed.

## What a program does with it

Call `init` with `InitOptions` before a project is located, since it makes one, or `new_component` with the manifest of a located
one. Each prints as it works and returns its exit code. `Err` is a refusal, nothing written; after the first write a failure
is exit 1 with the files in place.

Why it is shaped this way is `register@create@design`.
