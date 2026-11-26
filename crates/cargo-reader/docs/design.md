# Design — cargo-reader

How the `cargo-reader` crate reads Cargo's formats, and why. Each level-three heading is one
decision with its standing argument.

## Reading Cargo

### A failure of Cargo, or a text of a shape Cargo refuses, is an error, never an empty answer `##failed-cargo-is-an-error`

`read_manifest` returns an error for a text that is no TOML, a `[package]` with no `name`, and a
field read whose value has a type Cargo refuses for it; `run_metadata` returns Cargo's exit status
and standard error when Cargo fails, and `read_metadata` an error for an output of another shape or
another format version. A caller that judges a project on these facts would otherwise read a
manifest it could not read as one stating nothing, and a failed `cargo metadata` as a graph with no
dependency, a verdict on a project Cargo itself refuses: the same reason that
`design@git@failed-git-is-an-error` gives for git, against `goal@documentation-stays-consistent`,
which needs a verdict to be about the project's own tree. The shapes Cargo refuses are those
`path@cargo-reader@docs/grounding/cargo-manifest.md` records as measured. A key the reader does not
read is ignored, whatever its value, so a manifest Cargo accepts is never refused for a field no
caller asked about.

### A manifest is read as it is written, with no default of Cargo's applied `##a-manifest-is-read-as-written`

Each field of `Package` is `Absent`, `Set` to the value the manifest writes, or `Workspace` for
`{ workspace = true }`; no default of Cargo's is applied, and `Package::inherit` replaces each
`Workspace` with the value of a `[workspace.package]` its caller hands it, which the caller finds.
Cargo's defaults are a property of a Cargo version, as
`path@cargo-reader@docs/grounding/cargo-manifest.md` reads in the `version` field, optional since
Cargo 1.75: a reader that applied them would answer for one Cargo, and a caller could not tell a
manifest that writes a value from one that writes none, which a finding that quotes the manifest
needs. That is why `rejected@cargo-reader@cargo-defaults-applied` lost. What Cargo makes of the
facts, such as whether it would publish a package, is the caller's, read from the same grounding
file. The workspace's manifest is found by the caller, as Cargo finds it by `package.workspace` or
by a search of the parent directories, so the read of a manifest touches no file and its tests
hold their fixtures inline.
