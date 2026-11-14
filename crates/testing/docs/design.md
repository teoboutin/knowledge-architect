# Design

How `testing` is built, and why. That it exists, and how it depends on the other crates, is
`design@knowledge-architect@testing-crate`.

## Its tests

### A test of the program's crates that needs the shared means of running over a project sits here `##binary-tests-in-testing`

A test of the crates `klarch` links that needs what such tests share, the build of `klarch` or the copy of a project into a repository of its own, sits in this Component, per `design@knowledge-architect@mock-projects-in-one-crate`, whatever its project's storage: the tests of `klarch` over a mock project, and `path@testing@tests/scaffold.rs`, which builds what `klarch init --rust` writes. `path@cli@tests/init.rs` runs the binary through cargo's own `CARGO_BIN_EXE_klarch` over scratch projects it writes, needs none of that, and stays in `cli`. The tests of `gates` and `mutate` stay with them: those crates need nothing of `klarch`. On the owner's word, the line is what a test drives and how much shared means it needs, not where its project is stored.
The nearest rival, `cli` dev-depending on this crate for the fixtures, with its tests left there and this crate's dev-dependency on `cli` removed, keeps those tests out of this home and needs no build started by a test. It loses to the direction of `design@knowledge-architect@no-crate-depends-on-testing`: no crate depends on this one, a dev-dependency included, so that the graph holds no cycle, as `design@agent-skills@shared-test-component` asks. The cost is `design@testing@nested-cargo-builds-the-binary`. `klarch` links nothing of this crate, so it refuses a mock in place, per `design@check@checked-sentinel-values`: a test of the binary runs in a copy that carries the version, and `path@cli@tests/fixture_pin.rs` holds the acceptance of `"fixture"` over `cli`'s own directory.

### A test here builds the real `klarch` by asking the cargo that runs it `##nested-cargo-builds-the-binary`

`klarch()` in `path@testing@tests/binary.rs` runs `$CARGO build -p knowledge-architect --bin klarch` once per run, in the test's profile, and runs the path cargo reports.
`CARGO_BIN_EXE_klarch` is set for the binary's own package alone, a dev-dependency does not build its binary, and an artifact dependency requires `-Z bindeps`, each read by a test in a workspace of two packages, `cargo test -p b` with `a = { path = "../a" }` and then with `artifact = "bin"`.
A run in process does not replace it: a command prints to the real stdout and a closed stdout ends the process, per `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, and `Cli::parse` sits in the binary's `main`.
The cost is a cargo started by a test, so a binary that does not build fails a test rather than the build.

### This crate's test files are outside this repository's walk `##test-files-outside-the-walk`

`[walk] exclude` covers `path@testing@tests/` whole, the mock projects and the test files. The mock projects are foreign projects. The test files hold
literals that are rule-shaped on purpose, which the walk would read as prose, since `klarch` links nothing of this crate, per
`design@check@checker-source-literals-are-data`, as copying them under a directory the walk reads and running `cargo klarch check` shows. On the owner's word to take the easiest shape to build, the session chose the row over binding each
literal to a name and over `klarch` naming this directory in its own source, which reopens that head, and no goal ranks the three. The cost is that a reference in a comment of these files is
checked by nobody, which `issue@model@string-literals-in-code-may-not-need-reading-at-all` holds until the question of reading literals at all is settled. The row names
this repository's directory and exempts nothing else, and a project can write such a row today, so it opens no way around a rule that `rejected@check@exempt-directory-manifest-row` kept shut: that row exempted literals in files the walk still reads.
