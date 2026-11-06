# create

Creates a project and adds a Component to one. What it is for is `register@create@goal`; what it does for a user is
`path@create@README.md`; why it is shaped this way is `register@create@design`.

**Every refusal comes before the first write**, per `design@create@init-asks-no-confirmation`: exit 2 promises an untouched tree,
and after a write a failure is exit 1.

**This crate depends on `check` for how a run ends**: `finish` runs `check --fix` through `commands::check`. A new call into `check`
goes through its facade, per `design@check@the-facade-publishes-what-a-consumer-needs`.

Its tests that run the binary are `path@cli@tests/init.rs`, since `CARGO_BIN_EXE_klarch` is set there alone.
