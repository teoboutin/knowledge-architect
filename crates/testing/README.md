# knowledge-architect-testing

What an extension's tests need to run the checks over a mock project, as the binary runs them. A
project with no extension never names this crate.

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

## What an adopter does

A project that writes an extension adds, beside its dependency on `knowledge-architect`:

```toml
[dev-dependencies]
knowledge-architect-testing = "=<the version it pins>"
```

Its test then configures its extensions, gathers the inputs, and runs the phases with `foundation`
and `run_with`, over a mock project it keeps in its own tree. The crate documentation, at
<https://docs.rs/knowledge-architect-testing>, holds the example, what a mock plants and what a test pins.

Why it is shaped this way is `register@testing@design`, and what stands open is `register@testing@issue`.
