# The output of `cargo metadata --format-version 1`

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/commands/cargo-metadata.html` of that
toolchain, and online at <https://doc.rust-lang.org/cargo/commands/cargo-metadata.html>; and
measured with that Cargo in this repository. They ground the run and the reader of
`path@cargo-reader@src/metadata.rs`. Each is read again at the source it names, or measured again
with the command given.

## The format

- **The format is versioned, and grows within a version**: "It is recommended to include the
  --format-version flag to future-proof your code"; "Currently 1 is the only possible value"; and
  "Within the same output format version, the compatibility is maintained", where "Adding new
  fields" and "Adding new values for enum-like fields" are "not considered as incompatible". The
  top-level `version` field is "The version of the schema for this metadata structure".
- **`packages`** is the "Array of all packages in the workspace. It also includes all
  feature-enabled dependencies unless --no-deps is used." Each holds `name`, `version`, `id` ("The
  Package ID for referring to the package within the document"), `manifest_path` ("Absolute path
  to this package's manifest"), and `repository` ("The repository value from the manifest or null
  if not specified").
- **`resolve`** is "The resolved dependency graph for the entire workspace", "null if --no-deps is
  specified", and "By default, this includes all dependencies for all target platforms. The
  `--filter-platform` flag may be used to narrow to a specific target triple." It holds `nodes`,
  each with `id` and `dependencies`, "an array of Package IDs", and `root`, "The package in the
  current working directory ... null if there is a virtual workspace."
- **A Package ID's form is opaque**: "fields related to “Source ID” are treated as opaque
  identifiers"; the reader compares IDs and never parses one.

## Measured in this repository

- **`packages` and the nodes of `resolve` name the same set**, measured by comparing the `id`s of
  both, with `cargo metadata --format-version 1 --locked --offline` and with
  `--filter-platform x86_64-pc-windows-gnu` added: no `id` is in one and not the other.
- **Under `--no-deps`, `resolve` is `null` and `packages` holds the workspace's members alone**,
  measured with `cargo metadata --format-version 1 --no-deps`.
- **Outside a Cargo project, Cargo exits 101 and says why on its standard error**, measured in an
  empty directory with no `Cargo.toml` above it: "error: could not find `Cargo.toml` in `<dir>` or
  any parent directory".
- **A package with `publish = false` reports `"publish": []`**, measured for `xtask`.
