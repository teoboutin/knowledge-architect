# Design

How `testing` is built, and why. That it exists, and how it depends on the other crates, is
`design@knowledge-architect@testing-crate`.

## Its tests

### The tests of the binary sit here and build the real `klarch` with a nested cargo `##binary-tests-in-testing`

A test over a mock project sits in this Component, the binary's included.
`CARGO_BIN_EXE_klarch` is set for the binary's own package alone, and a dev-dependency does not build its binary, each measured in a
scratch workspace, and an artifact dependency requires `-Z bindeps`. So `klarch()` in
`path@testing@tests/binary.rs` runs the cargo that runs the test, `cargo build -p knowledge-architect --bin klarch`, once per run, into the test's
own target directory and profile. A run in process does not replace it: a command prints to the real stdout and a closed stdout
ends the process, per `design@knowledge-architect@a-typed-command-prints-and-returns-its-code`, and `Cli::parse` sits in the binary's `main`.
The nearest rival, `cli` dev-depending on this crate for the fixtures, with its tests left there and this crate's dev-dependency on `cli` removed, keeps a test of the binary out of this home; the owner
chose this shape over it, which no goal ranks. The cost is a cargo started by a test, so a binary that does not build fails a test rather than the build.
A project's tests over a mock of `klarch` in place are refused its pin, since `klarch` links nothing of this crate, so a test here runs in a copy that carries the version, per
`design@check@checked-sentinel-values`, and `path@cli@tests/fixture_pin.rs` holds the acceptance over `cli`'s own directory.

### This crate's test files are outside this repository's walk `##test-files-outside-the-walk`

`[walk] exclude` covers `path@testing@tests/` whole, the mock projects and the test files. The mock projects are foreign projects. The test files hold
literals that are rule-shaped on purpose, which the walk would read as prose, since `klarch` links nothing of this crate, per
`design@check@checker-source-literals-are-data`. On the owner's word to take the easiest shape to build, the session chose the row over binding each
literal to a name, at least 50 of them, and over `klarch` naming this directory in its own source, which reopens that head. The cost is that a reference in a comment of these files is
checked by nobody, which `issue@model@string-literals-in-code-may-not-need-reading-at-all` holds until the question of reading literals at all is settled. The row names
this repository's directory and exempts nothing else, and a project can write such a row today, so it opens no way around a rule that `rejected@check@exempt-directory-manifest-row` kept shut: that row exempted literals in files the walk still reads.
