# The command line

**This crate holds the binary, the clap declarations, the dispatch and what a consumer names**, and a
command's logic is mostly the typed function of its tool's crate, such as `knowledge_architect_query`,
per `goal@cli@one-home-for-the-command-line`. A new command is a declaration here and a function there, and a check of a plain
value goes in the declaration.
A combination of arguments the domain refuses is a type of that function, and `cli::run`
refuses a value that holds it, per `design@cli@command-parsing-home`.

**An item is public only through the facade**, per `design@knowledge-architect@api-facade`: `path@cli@src/lib.rs` and the
module `cli`. `path@testing@tests/extension_api.rs` uses the public paths alone, and
`path@cli@tests/public_paths.rs` holds the whole list, so an item a consumer names that the facade leaves out
fails to compile there. A new public item is added to that list.

**`CARGO_BIN_EXE_klarch` is set in this package alone, and a test of the binary lives in `testing`**, which builds it, per `design@testing@binary-tests-in-testing`. `path@cli@tests/init.rs` stays here since it uses no mock project, and
`path@cli@tests/fixture_pin.rs` since its claim is about this crate's own directory.

Read `register@cli@design` before changing how it works.
