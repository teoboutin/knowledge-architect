# The output of `cargo metadata --format-version 1`

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/commands/cargo-metadata.html` of that
toolchain, and online at <https://doc.rust-lang.org/cargo/commands/cargo-metadata.html>; and
measured with that Cargo in this repository and in scratch consumers. They ground the run and the
reader of `path@cargo-reader@src/metadata.rs`. Each is read again at the source it names, or
measured again with the command given.

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
  current working directory (if --manifest-path is not given). This is null if there is a virtual
  workspace."
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
- **With `--manifest-path`, `root` is the package of that manifest**, measured in a scratch
  workspace of two members `a` and `b`: run in `a` with `--manifest-path ../b/Cargo.toml`, `root`
  is `b`'s ID; run at the virtual root with `--manifest-path a/Cargo.toml`, it is `a`'s, where the
  same run without the flag gives `null`.

## The flags `--locked` and `--offline`, measured in scratch consumers

Measured with that Cargo, outside this repository, in two scratch consumers, each a package with a
`[workspace]` table of its own and an empty library source file: `c`, whose one dependency is a path
dependency `d = { path = "../d", version = "=0.1.0" }` on a package `d` that sets `repository`;
and `r`, whose one dependency is `itoa = "=1.0.9"` from crates.io, run with `CARGO_HOME` set to an
empty directory. They ground the flags that the command `changelog` of the crate `query` passes
to the run. Each is taken again by the command named, run in that consumer, its lock file's hash
taken with `sha1sum Cargo.lock` before and after.

- **With no `Cargo.lock`, `--locked` refuses to write one, and `--offline` alone writes it.**
  `cargo metadata --format-version 1 --locked --offline` in `c` exits 101: "error: cannot create
  the lock file <dir>/Cargo.lock because --locked was passed to prevent this", followed by "help:
  to generate the lock file without accessing the network, remove the --locked flag and use
  --offline instead.", and no file is written. `cargo metadata --format-version 1 --offline`
  writes it, "Locking 1 package to highest compatible version".
- **A lock that an edit of the versions left stale fails under `--locked`, until `cargo fetch`
  updates it.** In `c`, with `d`'s `version` raised to `0.2.0` and the requirement to `=0.2.0`,
  `--locked --offline` exits 101: "error: cannot update the lock file <dir>/Cargo.lock because
  --locked was passed to prevent this", followed by the same help, the lock's hash unchanged.
  `cargo fetch` then rewrites the lock, "Updating d v0.1.0 (<dir>/d) -> v0.2.0", and `--locked
  --offline` passes, the lock's hash unchanged by it.
- **`--locked --offline` over a current lock writes nothing**: in `c`, the lock's hash is the same
  before and after the run.
- **Without `--offline`, the run reaches the registry; with it, a package the local cache lacks is
  not found.** In `r` with an empty `CARGO_HOME`, `--offline` exits 101: "error: no matching
  package named `itoa` found", "location searched: crates.io index". With no flag, the run prints
  "Updating crates.io index", "Downloading crates ...", "Downloaded itoa v1.0.9", and writes the
  lock.
- **`--locked --offline` unpacks a source from the local cache, with no network.** In `r`, after
  `rm -rf "$CARGO_HOME/registry/src"`, the run exits 0, `registry/src/<index>/itoa-1.0.9/` is
  written again, and the lock's hash is unchanged.
- **A package's `source` and `manifest_path` say where its files are.** A path dependency reports
  `"source": null` and a `manifest_path` at its path, `<dir>/d/Cargo.toml` in `c`; a registry one
  reports `"source": "registry+https://github.com/rust-lang/crates.io-index"` and a
  `manifest_path` under `$CARGO_HOME/registry/src/<index>/<name>-<version>/`, in `r`
  `index.crates.io-1949cf8c6b5b557f/itoa-1.0.9/Cargo.toml`. `d` reports its `repository` as its
  manifest sets it.

## `--manifest-path` and an enclosing package, measured in scratch packages

Measured with that Cargo, outside this repository, in a package `outer` with a path dependency
`d`, holding a directory `noproj` with no `Cargo.toml` and a package `inner` of its own
workspace. They ground the run of the command `changelog` of the crate `query` with
`--manifest-path`. Each is taken again by the command named, run in that directory.

- **Without `--manifest-path`, a directory with no `Cargo.toml` reads the graph of the package
  that encloses it.** `cargo metadata --format-version 1 --locked --offline` in `outer/noproj`
  exits 0, `root` naming `outer` and `packages` holding `d` and `outer`.
- **`--manifest-path` naming a file that does not exist exits 101**: in `outer/noproj`, with
  `--manifest-path <dir>/outer/noproj/Cargo.toml`, "error: manifest path `<path>` does not exist".
- **`--manifest-path` naming a package inside another package's directory reads that package's
  graph alone**: in `outer/inner`, with `--manifest-path <dir>/outer/inner/Cargo.toml`, `root` is
  `inner`'s ID and `packages` holds `inner` alone.

## A binary built by `cargo run` after the pin's edit, measured in a scratch workspace

- **The build resolves the edited pin, writes the lock, and the run of the binary then passes under
  `--locked --offline`**: in a workspace of a member `xtask`, a binary that runs
  `$CARGO metadata --format-version 1 --locked --offline`, with a path dependency `dep` at `=0.1.0`
  and its lock generated by `cargo generate-lockfile --offline`, then `dep`'s version and the
  requirement raised to `0.2.0`, and the binary run through an alias `k = "run -q -p xtask --"`:
  `cargo k` printed the inner run's exit 0, and `sha1sum Cargo.lock` changed. Taken again with
  those files and `cargo k`, comparing `sha1sum Cargo.lock` before and after.
