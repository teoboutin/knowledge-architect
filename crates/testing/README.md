# knowledge-architect-testing

What an extension's tests need to run the checks over a mock project, as the binary runs them. A
project with no extension never names this crate.

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
