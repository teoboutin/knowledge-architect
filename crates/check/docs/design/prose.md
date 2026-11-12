# Check design: What is prose

## What is prose

### The checker's own source reads its string literals as data `##checker-source-literals-are-data`

Under the tool's own directories a string literal is never prose. Its comments are prose like
every other file's, and everywhere else in a tree a string literal is prose unless it is bound
to a name, per `design@model@grammars-not-prefixes`.

**The tool's tests are the one place in a tree whose fixtures are rule-shaped on purpose.**
A number, a marker, a slug, a path or a version pin is written so that a test can watch a check
react to it. Read as prose each one is a live claim, and hiding each behind a named constant to
be interpolated makes the tests harder to read than the code they test. Comments stay prose
because the test modules point at decisions and paths for real, and those pointers stay checked wherever the walk reads them; `design@testing@test-files-outside-the-walk` states what it leaves out.

**The directories are compiled in, never declared.** A binary's source spans several
Components: `cli`'s, `check`'s, the `git` crate's, the `model` crate's, the crates of its commands, `create`, `query` and
`transcript`, and the Component of each extension the binary registers. Each library exports
the directory of the Component it belongs to, evaluated at build time, and the binary hands the
list of its libraries' directories to the walk. Each is one crate whose directory is its
Component, so each library exports its own `CARGO_MANIFEST_DIR`, and `klarch` sits inside `cli`'s. An extension whose library crate sits below its Component's directory exports
that Component's directory instead, or the binary and tests beside the library would sit outside
it: a binary sees only the libraries it depends on. A binary run
through a cargo alias over `cargo run` is built from the checkout on every invocation, so each
compiled path is the tree being checked, unless another checkout shares the target directory and
the build is not tied to its checkout, which `design@check@a-foreign-build-is-refused` refuses. Once `check` is consumed as a published crate, its
directory is outside the tree and exempts nothing, which is correct: its source is then not part
of the tree. The root and the compiled path are canonicalised before
the prefix test, so a symlinked checkout does not defeat it; a symlink inside the tree is not
followed. A manifest row is not an option, because a row can be pointed at any directory, which
is the shape `design@check@the-regime-has-no-opt-out` exists to refuse: only the checker's own source
can ever be exempt, and it is exempt by construction. The row's entry is
`rejected@check@exempt-directory-manifest-row`. `klarch` links nothing of `testing`, so the test files there are outside
this list; a row of this repository's manifest takes them out of the walk whole, comments included, per
`design@testing@test-files-outside-the-walk`, which exempts no literal of a file the walk reads.

**The failure that remains is loud.** The summary block names each of the checker's directories, relative to the root when it sits under it and
absolute otherwise, even when the compiled directory no longer exists, and prints the count of
Rust files it covered, so the state is visible in every run. A directory exempts files only when
it sits inside the tree being checked: a tree that sits inside it instead, such as a project an extension's
crate keeps as a mock, is a foreign project and every literal in it is prose.

**Whole source rather than test modules only.** The non-test source holds no literal that cites
a rule, a finding message interpolates the number it names, and a narrower rule would cost an
attribute lookup to protect a class with zero members.

The designs this decision did not keep: `rejected@check@fixture-marker-macro`,
`rejected@check@unit-tests-moved-under-tests`, `rejected@check@literals-as-data-in-every-crate` and
`rejected@check@no-string-literal-read`.

### A binary built from another checkout of the tool is refused before any command `##a-foreign-build-is-refused`

Two checkouts sharing one
target directory leave the last build's binary for both, and cargo does not rebuild it for the
other checkout, whose own package is still fresh; observed on thaum's repository, the other
checkout then runs that binary, its code and not only its compiled paths; the reproduction is in
the message of 4ebd9a2. Before any command,
the binary compares, over the project it locates or, for `init`, over PATH where PATH exists, where its own crate and each library it links were compiled with the tree it
is run over: a tree that holds the same package, by the name its `Cargo.toml` declares, at a
trailing run of the compiled directory but not at the compiled directory itself is a second
checkout, and the run exits 2 naming both directories and the `cargo clean` of every workspace
package the binary links, which rebuilds them from this one. The binary's own crate is listed
because it is rebuilt whenever anything it links is, so its compiled directory is always the last
build's; a worktree nested in the tree is a second checkout like any other. A tree holding no
copy is left alone, which covers a mock project under the tool's directory and a project
consuming a library as a published crate.

**The refusal detects, and a build tied to its checkout prevents.** A project whose every target
is tied to a variable valued at its checkout's root never runs another checkout's build, as
`design@knowledge-architect@a-build-is-tied-to-its-checkout` argues for this repository. The
refusal stays, for a binary built before the tie or outside the configuration that sets it, and
for a project that does not tie its builds. Its message names the tie beside the `cargo clean` that
recovers.

**Four limits.** Three come from reading only names and places. A second checkout that moved the
crate to another relative path is not seen, and runs, unless the build is tied to its checkout; the loud failure of
`design@check@checker-source-literals-are-data`, printed by `check`, still names it. A tree
holding an unrelated crate of the same name at the same relative path, one component long or
more, is taken for a copy, and refused; a binary run through an alias that builds it from the
checkout never meets one. A crate at a checkout's root is not tried, since the empty run would refuse every project whose
root package shares a name with a linked crate, so a second checkout of a project whose extension crate is its root runs. The fourth comes from where the check lives: it is part of the binary it judges, so a binary
built from a checkout older than the refusal, or from one that changed it, runs unrefused.

**The refusal, not a correction of the paths.** Finding the directories at run time would exempt
the right files and leave the other checkout's code judging this tree. Every command is refused,
not only those that read literals, because every command runs that code. Each library a binary
links exports its crate directory and its package name, and the binary hands the list over, so an
extension's library is covered by listing it; this constrains every extension binary. The values
it reads are each crate's own directory and package name, beside the Component directories that
`design@check@checker-source-literals-are-data` compiles in.

**Every binary tests both halves over its own Components.** Over a checkout holding the tool,
the summary's checker-source line names each Component whose library the binary links and
counts the walked Rust files under them. And a tree holding one of the binary's crates at
another relative path is refused. `klarch` does this in
`path@testing@tests/binary.rs`; an extension binary does it in its own suite, because only it
links its extension's library.

The design this decision did not keep: `rejected@check@run-time-checker-directory-discovery`.

### The manifest pins the checker's version, every binary that dispatches a command of the checker refuses to run over a project it does not satisfy, and a binary that dispatches none makes no call `##installed-binary-version-check`

Every manifest declares `[project] checker-version`: the exact version of the checker the project
runs, which is the version of the `check` library whatever binary links it, or one of the two values
of `design@check@checked-sentinel-values`. `Manifest::parse` reads it; an absent key, or a string
of none of the three forms, is a complaint opening with the key's name, and a value that is not a
string fails the parse. Each binary that dispatches a command of the checker calls
`cli::refuse_another_version` in its `main`, right after the build-origin refusal of
`design@check@a-foreign-build-is-refused`, before it dispatches any command but `init`, an
extension's own included: a version is accepted when it equals the `check` library's, and any refusal, the key's own
complaint included, is exit 2, per `design@cli@exit-code-ladder`, naming the two versions and
which side is older. `init`, which every binary hands to `cli::init` before it locates a project,
calls no version refusal: no manifest exists to read a pin from, and the one it writes pins the
`check` library's version, per `design@create@init-command`. `cli::run` does not refuse, so a command
driven in-process needs a key but no confirmed one. A binary that links the library and dispatches no command of the checker, such as a
maintenance crate that uses it only to find the project's root through `MANIFEST_NAME`, does not
call it: nothing it runs reads the documents by the checker's rules, and where it runs the checker,
as a gate does, it starts the project's declared checker command as a child, a binary that makes
the call, as `design@gates@a-project-holds-its-gate-list` has the gates library take how the
project runs its checker.

**Why the manifest, and why required.** Without a pin the binary reads, a binary older than the
skills a project installed reports each installed file as differing, with the repair "run
install-agent-skills", which writes the older text over the newer; under `harness = []` nothing is
compared at all; and two versions whose skill text is identical are not told apart. A pin in the
manifest covers all three; one more place to
edit when the pin moves cannot drift silently, since a binary differing from it refuses at its
first command. Optional, the key would leave unprotected the project that never wrote it, which is
the forgotten pin it exists for. This serves `goal@every-target-project-can-adopt-it`:
a project pins the version it uses, and moves when it chooses.

**The working tree only.** `commits` reads a historical tree's key, so a tree without it stops in
phase 1, and compares no historical value, so moving the pin fails no earlier commit. A branch that
introduces the key therefore opens with the commit that gives every manifest the key, per
`design@check@every-commit-judged-by-the-tip-checker`.

**A binary released before the key** refuses a manifest carrying it at parse, as an unknown field,
since `[project]` denies unknown keys: it runs no command, so it cannot suggest a downgrade, but
its message does not name the pin.

### A checker version that is no version is accepted only where the running binary's build confirms it `##checked-sentinel-values`

Two values of `[project] checker-version` are not versions, for a project's own tests and for a
tree the checker is built from:

- `"fixture"` is accepted when the project lies strictly inside `check`'s compiled directory or
  one of the directories the binary passes as its own source: a mock project of a library the
  binary links. Strictly, because an extension whose crate is the project's root passes the root
  among its directories, and a library is no mock of itself.
- `"self"` is accepted when `check`'s compiled directory lies inside, or is, the project, and the
  project's git tracks that directory's `Cargo.toml`: a tree the checker is built from. The
  tracking tells it from a project whose `CARGO_HOME` is one of its own directories, where a
  registry build's directory also lies inside the root.

Both read the paths canonicalised, as `design@check@a-foreign-build-is-refused` does, and decide
from where the running binary was built, which names no project, per
`design@check@nothing-of-a-project-is-compiled-in`. Anywhere else each is refused, so no value of
the key switches a real project's pin off, the stance `design@check@the-regime-has-no-opt-out`
takes for rules over documents. A copy of a mock project outside its library carries the version.
The same values accepted wherever they are written are
`rejected@check@sentinels-accepted-everywhere`, which this decision did not keep.
