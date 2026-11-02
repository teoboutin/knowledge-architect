# Crate boundaries: Cargo's rules and one project's testing crate

Findings established outside this project that `design@agent-skills@component-crate-mapping`,
`design@agent-skills@split-cost-by-publication` and `design@agent-skills@shared-test-component` lean
on. Each is read again at the source it names.

## Cargo's rules, read in the Cargo Book

- **A crate published on crates.io cannot depend on a crate given only by a path.** "Specifying
  Dependencies", section "Local paths in published crates": "Crates that use dependencies specified
  with only a path are not permitted on crates.io."
- **A dev-dependency is the exception.** "Specifying Dependencies", section "Development
  dependencies": "When a package is published, only dev-dependencies that specify a `version` will be
  included in the published crate." A dev-dependency given only by a path is dropped from the
  published crate rather than refused.
- **Cargo refuses a dependency cycle outside dev-dependencies.** "Dependency Resolution", section
  "dev-dependency cycles": "Usually the resolver does not allow cycles in the graph, but it does allow
  them for dev-dependencies."
- **A dev-dependency cycle links two copies of the library into its unit tests.** The same section:
  "In the case of building library unit tests, there are actually two copies of the library linked
  into the final test binary", and "the types between the two are not compatible". Measured with
  cargo 1.99.1 on a workspace of two crates, `a` with a dev-dependency on `b` and `b` depending on
  `a`, where a unit test of `a` passes a value of `a`'s type `T` to a function of `b`: `cargo test -p a`
  fails to compile with "mismatched types", "expected `a::T`, found `T`".

## Many small crates published from one repository

- **gitoxide publishes several dozen crates from one repository**, each a part of one Git
  implementation, released in dependency order. Read through the crates.io API, counting the crates
  whose name begins with `gix` and whose `repository` field names GitoxideLabs/gitoxide:
  `curl -s 'https://crates.io/api/v1/crates?q=gix&per_page=100&page=<n>'` for each page until one
  holds no crate. A count that fell to a few crates would weaken the claim that Rust practice accepts
  many small published crates.

## thaum's testing crate

thaum, a Rust project whose agents follow this workflow, holds the shape of a shared test Component:
its crate `thaum-testing`, at `path@elsewhere@thaum/tools/thaum-testing/`, depends on `thaum-engine`
and `thaum-ai`, and the only crate that depends on it is thaum's bench tool,
`path@elsewhere@thaum/tools/bench/`. Read in the `Cargo.toml` files of thaum's workspace.
