# knowledge-architect-model

What knowledge-architect reads of a project: its manifest, the walk git supplies, the parse of each
document, the entity table over them, and what a reference resolves to. It runs phases 1 to 3 of the
checker, which say whether that model can be trusted. The one check of content it holds, while it stays
in phase 2, compares a project's installed agent files with the text this version ships.

A project that uses knowledge-architect names the package `knowledge-architect` alone, which depends
on this one, and never names this crate. A Rust program that reads a project's documents, with no check,
depends on it directly; its interface is at <https://docs.rs/knowledge-architect-model>.

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

## What a tool does with it

```rust,ignore
use knowledge_architect_model::{foundation, table, Manifest, Model};

let manifest = Manifest::find(std::env::current_dir()?.as_path())?;
let model = Model::build(&manifest, &[])?;
// Refuses a model phases 1 to 3 stop, naming the phase.
foundation::complete_working_tree(&manifest, &model)?;
let anchors = table::Anchors::of(&manifest, model.listing());
let entities = table::Entities::build(&model, &anchors);
```

- `Model::build`'s second argument names the directories whose string literals are data: a tool run
  over this repository hands them, and over any other project hands none.
- A manifest table belongs to an extension, so a tool over a project with one calls
  `extension::contribute` before `Model::build`, or the gate reports the table. The version a manifest
  pins is the binary's to refuse, not this library's.
- [`resolve`](https://docs.rs/knowledge-architect-model/latest/knowledge_architect_model/resolve/) says
  what a reference names, or why it names nothing, with the words of each cause and no finding attached.
- `foundation::Gathered` reads what phases 1 to 3 and the resolver read: one survey, the two git batches and
  the agent files this version ships.

## What it needs and guarantees

- **`git` on the path, and a project inside a git worktree**: the walk is git's listing.
- **Nothing about a project is compiled in**: every list comes from the project's
  `knowledge-architect.toml`.
- **A phase stops the run**: the first of phases 1 to 3 that finds anything is the answer, since what
  a later one computes over an incomplete model is unreliable in both directions.
- **A model is built once and read**: a phase and the resolver spawn nothing and read no file. What
  they need from the tree is gathered first and handed in.

Why it is shaped this way is `register@model@design`.
