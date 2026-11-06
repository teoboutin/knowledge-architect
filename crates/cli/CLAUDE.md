# The command line

**This crate holds the binary, the clap declarations, the dispatch and what a consumer names**, and a
command's logic is mostly the typed function of its tool's crate, such as `knowledge_architect_query`,
per `goal@cli@one-home-for-the-command-line`. A new command is a declaration here and a function there, and a check of a plain
value goes in the declaration.
A combination of arguments the domain refuses is a type of that function, and `cli::run`
refuses a value that holds it, per `design@cli@command-parsing-home`.

**An item is public only through the facade**, per `design@knowledge-architect@api-facade`: `path@cli@src/lib.rs` and the
module `cli`. `path@cli@tests/extension_api.rs` uses the public paths alone, and
`path@cli@tests/public_paths.rs` holds the whole list, so an item a consumer names that the facade leaves out
fails to compile there. A new public item is added to that list.

**`CARGO_BIN_EXE_klarch` is set in this package alone**, so a test that runs the binary lives here, over the
fixtures of the crate that holds them, as `check_dir` and `transcript_dir` in a test file find them.

**A test that copies a mock project runs `git init` and `git add -A` in the copy**, or the walk
is empty and the test proves nothing; `Sandbox` in `path@cli@tests/binary.rs` does it. Two
consequences a test about ignore behaviour has to choose between, because the ignore rules act on
untracked files alone:

- for a file that must be **ignored**, write the `.gitignore` before the first `git add -A`, so
  the add never tracks it. `Sandbox::seeded` is that order.
- for a file that must be **tracked and ignored**, stage it first and add the `.gitignore`
  after, so the add finds the file already in the index.

**A test that runs git through `Sandbox` never depends on the machine's git identity**:
`Sandbox::git_command`, which `Sandbox::git` runs, gives every invocation the sandbox's identity on
its command line and removes the environment variables that would override it. Other machine configuration still
reaches some helpers, per `issue@check@the-tests-read-the-machine-s-git-configuration`.

A test that deletes or rewrites a fixture file after the first add stages again, or git's listing
still names what the working tree no longer holds. The in-place tests under
`path@check@tests/projects/` need none of this: they build under this repository's own worktree, so a
mock file is walked because this repository's listing holds it. The listing is
`--cached --others --exclude-standard`, so a new fixture file is walked the moment it exists,
tracked or not; what removes one is an ignore rule, and what a deletion needs is staging, or the
path stays in the listing with no bytes behind it.

**A test about commit messages builds its own project.** `commits` fails every commit whose tree
carries a finding, the last one included, so each commit a test makes has a tree with none,
except the one it plants. `dirhome` is the one mock over which every family runs
and finds nothing, so a copy of it could serve as a base; `History` in
`path@cli@tests/binary.rs` writes a project out
anyway, because each test states the exact findings the commits it makes carry and a mock's
contents are shared with every other test over it. `History` then configures `user.name` and
`user.email` in that project's own configuration, and commits. Two orders it has to keep:

- **stage, regenerate the indexes, stage again.** The walk is git's listing, so a deleted entry
  the index still holds is still counted and `cargo klarch index` writes the listing the tree
  no longer has.
- **`--allow-empty`** on a commit whose subject is the message rather than a tree edit.

Read `register@cli@design` before changing how it works.
