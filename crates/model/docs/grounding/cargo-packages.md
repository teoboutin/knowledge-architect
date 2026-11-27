# Cargo's reading of a package: which it would publish, and which workspace it inherits from

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/reference/manifest.html` and
`path@elsewhere@share/doc/rust/html/cargo/reference/workspaces.html` of that toolchain, and online at
<https://doc.rust-lang.org/cargo/reference/manifest.html>; and measured with that Cargo. They ground
the read of the changelog crates in `path@model@src/changelogs/crates.rs`, which reads each
`Cargo.toml` through `cargo-reader`; how Cargo's manifest format is read is
`path@cargo-reader@docs/grounding/cargo-manifest.md`. Each is read again at the source it names, or
measured again with the command and the layout given.

## Which package Cargo would publish

Read from the passages of `[package]`: `version` "is required for publishing packages", and
omitting it keeps a package from being published; `publish = false` disables publishing; and
`publish = [...]` names the registries a package may be published to. So a package Cargo would
publish sets a `version`, its own or inherited, with `publish` absent, `true`, or a list naming a
registry. **An empty list names no registry, and is read as `false`**: the reference does not say so
in words, and `cargo metadata --format-version 1 --no-deps --offline` was measured to report `false`
and `[]` alike, as `[]`.

## Which workspace a package inherits from

The book: "The workspace field can be used to configure the workspace that this package will be a
member of. If not specified this will be inferred as the first Cargo.toml with [workspace] upwards
in the filesystem", and "The exclude key can be used to prevent paths from being included in a
workspace".

Measured with `cargo metadata --format-version 1 --no-deps --offline`, run in the package's
directory, by the version a package that sets `version.workspace = true` reports. The layout: a
root `Cargo.toml` holding `[workspace]` with `members = ["inner/a"]` and the version `2.0.0`; a
`Cargo.toml` in `inner` holding `[workspace]` with the keys named and the version `1.0.0`; the
package in `inner/a`.

- **A workspace that excludes the package is passed over**: with `exclude = ["a"]` in `inner`, the
  package reports `2.0.0`; so it does with `exclude = ["a/"]`, `exclude = ["./a"]`,
  `exclude = ["."]` and `exclude = [""]`, each naming the package's directory or one above it.
- **An `exclude` entry is a path, never a pattern, and names whole directories**: with
  `exclude = ["*"]`, and with `exclude = ["a"]` for a package in `inner/ab`, the nearer workspace is
  the one Cargo takes, and it refuses the package, "current package believes it's in a workspace
  when it's not".
- **A `members` entry naming the package overrides an `exclude` that names it**: with
  `members = ["a"]` and `exclude = ["a"]` in `inner`, the package reports `1.0.0`; with
  `members = ["sub"]` and `exclude = ["sub/a"]`, for a package in `inner/sub/a` beside a package in
  `inner/sub`, Cargo takes `inner` and refuses the package as no member, so a `members` entry naming
  a directory above the package overrides as well. A glob does not: with `members = ["*"]` and
  `exclude = ["a"]`, the package reports `2.0.0`.
- **A workspace that neither lists nor excludes the package is the one Cargo takes, and it refuses
  the package**: with `members = []` in `inner`, "current package believes it's in a workspace when
  it's not".
- **`package.workspace` names the workspace, the search not made**: `workspace = "../.."` in the
  package reports `2.0.0` beside an `inner` that lists it; `workspace = ".."` takes `inner` even
  where `inner` excludes the package, and is refused as above.

## An inherited `readme` or `license-file`

The book: "license-file and readme are relative to the workspace root; include and exclude are
relative to your package root". Measured with `cargo package --allow-dirty --offline --no-verify`
over a workspace whose root holds a `CHANGELOG.md` and whose member `a` holds its own: with
`readme = "CHANGELOG.md"` in `[workspace.package]`, `readme.workspace = true` and
`include = ["/src/**"]` in the member, the package holds a `CHANGELOG.md` whose text is the root's;
so it does with `license-file = "CHANGELOG.md"` inherited and `exclude = ["/CHANGELOG.md"]`
inherited. The member's own changelog is then in no package.
