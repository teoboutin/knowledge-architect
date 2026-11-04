# Model design: What is prose

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
`rejected@model@every-literal-read-as-prose`, with the bound literals outside the checker that carry
a rule-shaped number by accident. The binding rule does not serve the checker's own
fixtures, which `design@core@checker-source-literals-are-data` answers.

The design this decision did not keep: `rejected@model@hand-rolled-indentation-scanner`.

### Rust source is parsed by one parser that keeps every comment and is released on its own cadence, `tree-sitter-rust` `##rust-parser-keeps-every-comment`

The core reads Rust source with `tree-sitter-rust`, through the `tree-sitter` crate, both declared in
`path@core@Cargo.toml`. Its syntax tree keeps every comment, doc comments and ordinary `//` and
`/* */` comments alike, with their byte ranges, so one pass tells which bytes of a Rust file are
prose, as `design@model@grammars-not-prefixes` requires, and `pulldown-cmark` reads those ranges as
it reads a markdown file.

The choice turns on two external tools' behaviour, entry test 3 of `primer@design-heads`. The
nearest rival, `syn`, drops ordinary comments while lexing, as the prototype that
`rejected@model@syn-rust-parser` records showed, so a reference in a `//` comment inside a function
body would need a second pass over the raw text: two readers that each answer half of the question.
Rust-analyzer's syntax crate keeps comments, but it is a crate of compiler internals released with
the rust-analyzer releases, whose interface changes from one release to the next; this cadence is an
assumption, which no measurement or documentation in this tree checks.
An upgrade of `tree-sitter-rust` that renames a node kind or a field the extractor asks for fails
the test `every_rust_extractor_node_kind_is_one_the_grammar_defines`, and a Rust file of the tree
that yields no prose region while it holds a comment, or no item scope while it holds a top-level
function, fails `every_tracked_rust_file_yields_prose`. An upgrade of `pulldown-cmark` that renames
an event fails to compile, since the extractor matches its events as Rust enum variants. The designs this decision did not keep: `rejected@model@syn-rust-parser` and
`rejected@model@rust-analyzer-syntax-crate`.

### A Rust name is collected where it is declared, never where it is used `##names-collected-where-declared`

**A name is the one thing prose cannot reach**, and the identifier form of a rule marker exists
for names. Names are collected where they are DECLARED. Collecting every occurrence would make a call
site and a `use` import citations owing the rule's whole body in the caller's scope, where there
is nowhere to put it.
