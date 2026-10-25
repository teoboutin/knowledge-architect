# Core design: What is prose

## What is prose

### The grammar for a file's kind decides which of its bytes are prose `##grammars-not-prefixes`

A markdown document is prose entire. A Rust file is prose only where its grammar says so — its
comments, and the string literals that are not bound to a name. Nothing is decided by a line's
prefix. The one file set whose literals are never prose is the checker's own source, per
`design@core@checker-source-literals-are-data`.

**A doc comment's content is markdown**, so the markdown analysis runs over a document and over
every Rust comment alike, and a heading, a fence or a blockquote means the same in both.

**Prefixes were the single cause of four defects in thaum's tree**, each found by a checker or a
review rather than by reasoning: a formatted return type opening a line with an angle bracket read as a
blockquote; a float literal read as a rule number; a test fixture's string literal read as live
content; and a rule quoted inside a block comment lost because the `*` continuation was left in
front of it.

**Three readings of "data" were too broad, and the measurement over thaum's tree, in the message
of the commit "rework how the knowledge tool parses documents and enforces rule citations", caught
each.** A fenced block is not data — a sketch in a design document comments
its rules on purpose, and reading fences as data lost 34 citations. A string literal is data only when BOUND to a name — an assertion's
message cites rules for a human to read on failure, and reading every literal as data lost 32
of them. A macro body is not an argument list, so a binding still applies inside one. **Reading
every literal as prose is a recorded rejected design**,
`rejected@core@every-literal-read-as-prose`, with the bound literals outside the checker that carry
a rule-shaped number by accident. The binding rule does not serve the checker's own
fixtures, which `design@core@checker-source-literals-are-data` answers.

The design this decision did not keep: `rejected@core@hand-rolled-indentation-scanner`.

### Rust source is parsed by one parser that keeps every comment and is released on its own cadence, `tree-sitter-rust` `##rust-parser-keeps-every-comment`

The core reads Rust source with `tree-sitter-rust`, through the `tree-sitter` crate, both declared in
`path@core@Cargo.toml`. Its syntax tree keeps every comment, doc comments and ordinary `//` and
`/* */` comments alike, with their byte ranges, so one pass tells which bytes of a Rust file are
prose, as `design@core@grammars-not-prefixes` requires, and `pulldown-cmark` reads those ranges as
it reads a markdown file.

The choice turns on two external tools' behaviour, entry test 3 of `primer@design-heads`. The
nearest rival, `syn`, drops ordinary comments while lexing, as the prototype that
`rejected@core@syn-rust-parser` records showed, so a reference in a `//` comment inside a function
body would need a second pass over the raw text: two readers that each answer half of the question.
Rust-analyzer's syntax crate keeps comments, but it is a crate of compiler internals released with
the rust-analyzer releases, whose interface changes from one release to the next; this cadence is an
assumption, which no measurement or documentation in this tree checks.
An upgrade of `tree-sitter-rust` that renames a node kind or a field the extractor asks for fails
the test `every_rust_extractor_node_kind_is_one_the_grammar_defines`, and a Rust file of the tree
that yields no prose region while it holds a comment, or no item scope while it holds a top-level
function, fails `every_tracked_rust_file_yields_prose`. An upgrade of `pulldown-cmark` that renames
an event fails to compile, since the extractor matches its events as Rust enum variants. The designs this decision did not keep: `rejected@core@syn-rust-parser` and
`rejected@core@rust-analyzer-syntax-crate`.

### A Rust name is collected where it is declared, never where it is used `##names-collected-where-declared`

**A name is the one thing prose cannot reach**, and the identifier form of a rule marker exists
for names. Names are collected where they are DECLARED. Collecting every occurrence would make a call
site and a `use` import citations owing the rule's whole body in the caller's scope, where there
is nowhere to put it.

### The checker's own source reads its string literals as data `##checker-source-literals-are-data`

Under the tool's own directories a string literal is never prose. Its comments are prose like
every other file's, and everywhere else in a tree a string literal is prose unless it is bound
to a name, per `design@core@grammars-not-prefixes`.

**The tool's unit tests are the one place in a tree whose fixtures are rule-shaped on purpose.**
A number, a marker, a slug, a path or a version pin is written so that a test can watch a check
react to it. Read as prose each one is a live claim, and hiding each behind a named constant to
be interpolated makes the tests harder to read than the code they test. Comments stay prose
because the test modules point at decisions and paths for real, and those pointers stay checked.

**The directories are compiled in, never declared.** A binary's source spans several
Components: the core's, and the Component of each extension the binary registers. Each library exports
the directory of the Component it belongs to, evaluated at build time, and the binary hands the
list of its libraries' directories to the walk. The core is one crate whose directory is its
Component, so the core library exports its own `CARGO_MANIFEST_DIR`, and the core's binary and
tests sit inside it. An extension whose library crate sits below its Component's directory exports
that Component's directory instead, or the binary and tests beside the library would sit outside
it: a binary sees only the libraries it depends on. A binary run
through a cargo alias over `cargo run` is built from the checkout on every invocation, so each
compiled path is the tree being checked, unless another checkout shares the target directory and
the build is not tied to its checkout, which `design@core@a-foreign-build-is-refused` refuses. Once the core is consumed as a published crate, its
directory is outside the tree and exempts nothing, which is correct: its source is then not part
of the tree. The root and the compiled path are canonicalised before
the prefix test, so a symlinked checkout does not defeat it; a symlink inside the tree is not
followed. A manifest row is not an option, because a row can be pointed at any directory, which
is the shape `design@core@the-regime-has-no-opt-out` exists to refuse: only the checker's own source
can ever be exempt, and it is exempt by construction. The row's entry is
`rejected@core@exempt-directory-manifest-row`.

**The failure that remains is loud.** The summary block names each of the checker's directories, relative to the root when it sits under it and
absolute otherwise, even when the compiled directory no longer exists, and prints the count of
Rust files it covered, so the state is visible in every run. A directory exempts files only when
it sits inside the tree being checked: a tree that sits inside it instead, such as a mock project under
`path@core@tests/projects/`, is a foreign project and every literal in it is prose.

**Whole source rather than test modules only.** The non-test source holds no literal that cites
a rule, a finding message interpolates the number it names, and a narrower rule would cost an
attribute lookup to protect a class with zero members.

The designs this decision did not keep: `rejected@core@fixture-marker-macro`,
`rejected@core@unit-tests-moved-under-tests`, `rejected@core@literals-as-data-in-every-crate` and
`rejected@core@no-string-literal-read`.

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

**Three limits.** Two come from reading only names and places. A second checkout that moved the
crate to another relative path is not seen, and runs, unless the build is tied to its checkout; the loud failure of
`design@core@checker-source-literals-are-data`, printed by `check`, still names it. A tree
holding an unrelated crate of the same name at the same relative path, one component long or
more, is taken for a copy, and refused; a binary run through an alias that builds it from the
checkout never meets one. The third comes from where the check lives: it is part of the binary it judges, so a binary
built from a checkout older than the refusal, or from one that changed it, runs unrefused.

**The refusal, not a correction of the paths.** Finding the directories at run time would exempt
the right files and leave the other checkout's code judging this tree. Every command is refused,
not only those that read literals, because every command runs that code. Each library a binary
links exports its crate directory and its package name, and the binary hands the list over, so an
extension's library is covered by listing it; this constrains every extension binary. The values
it reads are each crate's own directory and package name, beside the Component directories that
`design@core@checker-source-literals-are-data` compiles in.

**Every binary tests both halves over its own Components.** Over a checkout holding the tool,
the summary's checker-source line names each Component whose library the binary links and
counts the walked Rust files under them. And a tree holding one of the binary's crates at
another relative path is refused. The core's binary does this in
`path@core@tests/binary.rs`; an extension binary does it in its own suite, because only it
links its extension's library.

The design this decision did not keep: `rejected@core@run-time-checker-directory-discovery`.

### The manifest pins the checker's version, every binary that dispatches a command of the checker refuses to run over a project it does not satisfy, and a binary that dispatches none makes no call `##installed-binary-version-check`

Every manifest declares `[project] checker-version`: the exact version of the checker the project
runs, which is the version of the core library whatever binary links it, or one of the two values
of `design@core@checked-sentinel-values`. `Manifest::parse` reads it; an absent key, or a string
of none of the three forms, is a complaint opening with the key's name, and a value that is not a
string fails the parse. Each binary that dispatches a command of the checker calls
`cli::refuse_another_version` in its `main`, right after the build-origin refusal of
`design@core@a-foreign-build-is-refused`, before it dispatches any command but `init`, an
extension's own included: a version is accepted when it equals the core library's, and any refusal, the key's own
complaint included, is exit 2, per `design@core@exit-code-ladder`, naming the two versions and
which side is older. `init`, which every binary hands to `cli::init` before it locates a project,
calls no version refusal: no manifest exists to read a pin from, and the one it writes pins the
core library's version, per `design@core@init-command`. `cli::run` does not refuse, so a command
driven in-process needs a key but no confirmed one. A binary that links the library and dispatches no command of the checker, such as a
maintenance crate that uses it only to find the project's root through `MANIFEST_NAME`, does not
call it: nothing it runs reads the documents by the checker's rules, and where it runs the checker,
as a gate does, it starts a binary that makes the call, since the gates library depends on nothing
of the checker, per `design@gates@a-project-holds-its-gate-list`.

**Why the manifest, and why required.** Without a pin the binary reads, a binary older than the
skills a project installed reports each installed file as differing, with the repair "run
install-agent-skills", which writes the older text over the newer; under `harness = []` nothing is
compared at all; and two versions whose skill text is identical are not told apart. A pin in the
manifest covers all three; one more place to
edit when the pin moves cannot drift silently, since a binary differing from it refuses at its
first command. Optional, the key would leave unprotected the project that never wrote it, which is
the forgotten pin it exists for. This serves `goal@knowledge-architect@any-project-can-adopt-it`:
a project pins the version it uses, and moves when it chooses.

**The working tree only.** `commits` reads a historical tree's key, so a tree without it stops in
phase 1, and compares no historical value, so moving the pin fails no earlier commit. A branch that
introduces the key therefore opens with the commit that gives every manifest the key, per
`design@core@a-commit-message-is-a-document`.

**A binary released before the key** refuses a manifest carrying it at parse, as an unknown field,
since `[project]` denies unknown keys: it runs no command, so it cannot suggest a downgrade, but
its message does not name the pin.

### A checker version that is no version is accepted only where the running binary's build confirms it `##checked-sentinel-values`

Two values of `[project] checker-version` are not versions, for a project's own tests and for a
tree the checker is built from:

- `"fixture"` is accepted when the project lies strictly inside the core's compiled directory or
  one of the directories the binary passes as its own source: a mock project of a library the
  binary links. Strictly, because an extension whose crate is the project's root passes the root
  among its directories, and a library is no mock of itself.
- `"self"` is accepted when the core's compiled directory lies inside, or is, the project, and the
  project's git tracks that directory's `Cargo.toml`: a tree the checker is built from. The
  tracking tells it from a project whose `CARGO_HOME` is one of its own directories, where a
  registry build's directory also lies inside the root.

Both read the paths canonicalised, as `design@core@a-foreign-build-is-refused` does, and decide
from where the running binary was built, which names no project, per
`design@core@nothing-of-a-project-is-compiled-in`. Anywhere else each is refused, so no value of
the key switches a real project's pin off, the stance `design@core@the-regime-has-no-opt-out`
takes for rules over documents. A copy of a mock project outside its library carries the version.
The same values accepted wherever they are written are
`rejected@core@sentinels-accepted-everywhere`, which this decision did not keep.
