# knowledge-architect-query

Tools over a project's scanned record: one entry whole and what cites it, the issues and the tripwires one row each, the
anchors and each register's home, whether an id is free, and the work list of a search of the whole record with the check
of its reports. They read the model and run no check, and this crate depends on nothing of the checks.

A project that uses knowledge-architect names the package `knowledge-architect` alone, whose binary `klarch` offers these
commands, and never names this crate; their usage is `path@cli@README.md`. A Rust program that consults a record the same
way depends on it directly; its interface is at <https://docs.rs/knowledge-architect-query>.

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

Build a manifest with `Manifest::find` of `knowledge-architect-model`, and call a function: `show`, `issues`, `tripwires`,
`anchors`, `id` or `worklist`. Each prints as it works and returns its exit code. It takes the manifest as a caller's
extensions have configured it, as `cli::run` does before it calls one, per `design@query@a-function-takes-a-configured-manifest`, so a project with an extension's table is read as the
binary that registers the extension reads it only where the caller has called `extension::configure`, an item of
`knowledge-architect-check`, which this crate does not depend on. A program that links no check reads such a project with the
extension's generated files in its walk, and `id` and `worklist --out` refuse its table.

Why it is shaped this way is `register@query@design`.
