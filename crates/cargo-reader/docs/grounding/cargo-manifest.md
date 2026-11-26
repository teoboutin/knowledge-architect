# Cargo's manifest format: what a `Cargo.toml` states of a package

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/reference/manifest.html` and
`path@elsewhere@share/doc/rust/html/cargo/reference/workspaces.html` of that toolchain, and online at
<https://doc.rust-lang.org/cargo/reference/manifest.html>; and measured with that Cargo, by
`cargo metadata --format-version 1 --no-deps --offline` in a scratch directory holding the manifest
named. They ground the reader of `path@cargo-reader@src/manifest.rs`. Each is read again at the
source it names, or measured again with the command and manifest given.

## `[package]`

- **`name` is the one field Cargo requires**: "The only field required by Cargo is name." A
  `[package]` without it is refused, measured: "missing field `package.name`".
- **`version` is optional and defaults to `0.0.0`**: "This field is optional and defaults to 0.0.0.
  The field is required for publishing packages", with "MSRV: Before 1.75, this field was
  required". Measured: a manifest with no `version` reports `"version": "0.0.0"` and
  `"publish": []`.
- **`publish` names the registries a package may be published to**: `publish = ["some-registry-name"]`;
  `publish = false` disables publishing, and "To prevent a package from being published to a
  registry (like crates.io) by mistake ... you can omit the version field." Measured, as
  `cargo metadata` reports `publish`: `false` and `[]` both report `[]`; `true` and an absent
  `publish` with a `version` both report `null`; a string, `publish = "yes"`, is refused, "invalid
  type: string "yes", expected a boolean, a vector of strings, or workspace"; and `publish = true`
  with no `version` is refused, "`package.publish` requires `package.version` be specified".
- **`repository`** "should be a URL to the source repository for your package"; Cargo reports it
  as written, or `null` when it is not set.
- **`include`** lists "gitignore-style patterns" of the files packaged; "The Cargo.toml file of the
  package itself is always included, it does not need to be listed in include", "If a
  license-file is specified, it is always included", and "setting include will override an
  exclude". The `readme` file is not named among the files always included.

## Inheritance from `[workspace.package]`

- **A member inherits a key with `{key}.workspace = true`**: "The workspace.package table is where
  you define keys that can be inherited by members of a workspace. These keys can be inherited by
  defining them in the member package with {key}.workspace = true." Among the keys supported are
  `version`, `publish`, `repository` and `include`; "include and exclude are relative to your
  package root". The inline form `repository = { workspace = true }` is the same TOML, measured
  inherited.
- **`workspace = false` is refused**, measured: "`workspace` cannot be false".
- **A key inherited that `[workspace.package]` does not set is refused**, measured with
  `publish.workspace = true` under a `[workspace.package]` setting no `publish`: "error inheriting
  `publish` from workspace root manifest's `workspace.package.publish`".
- **Which manifest is the workspace's**: "Cargo will automatically search the parent directories
  for a Cargo.toml file with a [workspace] definition to determine which workspace to use. The
  package.workspace manifest key can be used in member crates to point at a workspace's root to
  override this automatic search."
