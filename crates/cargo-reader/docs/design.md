# Design — cargo-reader

How the `cargo-reader` crate reads Cargo's formats, and why. Each level-three heading is one
decision with its standing argument.

## Reading Cargo

### A failure of Cargo, or a text or an output the reader cannot read, is an error carrying the reason, never an empty answer `##failed-cargo-is-an-error`

`read_manifest` returns an error for a text that is no TOML, a `[package]` with no `name` that is a
string, and a field it refuses, per `design@cargo-reader@a-field-is-refused-by-its-type`;
`run_metadata` returns Cargo's exit status and standard error when Cargo fails, and `read_metadata`
an error for an output of another shape or another format version. These items of the library are
named in prose, since no checked form names a Rust item, per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`. A caller that judges a project on
these facts would otherwise read a manifest it could not read as one stating nothing, and a failed
`cargo metadata` as a graph with no dependency, a verdict on a project Cargo itself refuses: the
same reason that `design@git@failed-git-is-an-error` gives for git, against
`goal@documentation-stays-consistent`, which needs a verdict to be about the project's own tree.
That is why `rejected@cargo-reader@an-empty-answer-on-failure` lost.

### A field of a manifest is refused by the type of its value alone, never by a combination of fields or of values `##a-field-is-refused-by-its-type`

`read_manifest` refuses a field it reads whose value has a type Cargo refuses for it, each type as
`path@cargo-reader@docs/grounding/cargo-manifest.md` records it, and reads a table whose `workspace`
is `true` as inheriting whatever other key it holds, as Cargo 1.98 reads it, measured there; that
measurement is why `rejected@cargo-reader@a-key-beside-workspace-refused` lost. It judges no
combination: a manifest Cargo refuses for a combination, such as `publish = true` with no
`version`, is read without error, so a read that succeeds does not say that Cargo accepts the
manifest. Which combinations Cargo refuses is a property of a Cargo version, as `version`, optional
since Cargo 1.75, shows: a reader that refused them would answer for one Cargo, the premise that
`design@cargo-reader@a-manifest-is-read-as-written` stands on, and why
`rejected@cargo-reader@combinations-refused` lost. A key the reader does not read is ignored,
whatever its value, so a manifest Cargo accepts is never refused for a field no caller asked about.
These items are named in prose, per `issue@model@a-rust-item-named-in-prose-has-no-checked-form`.

### A manifest is read as it is written, with no default of Cargo's applied `##a-manifest-is-read-as-written`

Each field of `Package` but its `name`, which Cargo requires as a string, is `Absent`, `Set` to
the value the manifest writes, or `Workspace` for a table whose `workspace` is `true`; no default of
Cargo's is applied, and `Package::inherit` replaces each `Workspace` with the value of the
`[workspace.package]` its caller hands it. These items are named in prose, per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`. The reading of Cargo's fields is the
one `path@cargo-reader@docs/grounding/cargo-manifest.md` records. Cargo's defaults are a property of a
Cargo version, as that file reads in the `version` field,
optional since Cargo 1.75: a reader that applied them would answer for one Cargo, whatever Cargo
its caller runs. That is why `rejected@cargo-reader@cargo-defaults-applied` lost. What Cargo makes
of the facts, such as whether it would publish a package, is the caller's, read from the same
grounding file.

### A `[project]` table is read as the package where the manifest holds no `[package]`, as Cargo reads it `##project-table-read-as-the-package`

`read_manifest` reads a `[project]` table, the older name of `[package]`, as the package of a
manifest that holds no `[package]`, and reads `[package]` where both are written, as Cargo 1.98
reads them, measured in `path@cargo-reader@docs/grounding/cargo-manifest.md`; the item is named in
prose, per `issue@model@a-rust-item-named-in-prose-has-no-checked-form`. Cargo builds such a
manifest that sets no `edition`, as measured there, so a read of it as a manifest with no package would be an
empty answer about a package Cargo builds, against `design@cargo-reader@failed-cargo-is-an-error`,
and the checker's search for a second checkout of the same tool,
`design@check@a-foreign-build-is-refused`, would miss one holding it. A refusal of the table loses
for the same reason: it is an error on a manifest Cargo builds. Cargo's refusal of `[project]` under
the 2024 edition depends on a second field, `edition`, so the read takes no account of it, per
`design@cargo-reader@a-field-is-refused-by-its-type`.

### The caller finds the workspace's manifest, so the read of a manifest touches no file `##the-caller-finds-the-workspace-manifest`

`read_manifest` takes a manifest's text, and `Package::inherit` the `[workspace.package]` of the
workspace's manifest, which the caller finds, as Cargo finds it by `package.workspace` or by a
search of the parent directories; these items are named in prose, per
`issue@model@a-rust-item-named-in-prose-has-no-checked-form`. The read of a manifest is then a
function of its text: it touches no file, and its unit tests hold their fixtures inline. The
caller holds the files already, as the checker holds the tree it walks, so a lookup in the reader
would read again what the caller read: that is why
`rejected@cargo-reader@workspace-manifest-found-by-the-reader` lost.
