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

- **`[project]` is read as `[package]` where the manifest holds no `[package]`**, measured:
  `[project]` with `name = "a"` and `version = "1.0.0"` reports the package `a`; beside a
  `[package]`, the `[package]` is the one read, and a `[project]` with no `name` is refused as
  "missing field `package.name`". Under the 2024 edition it is refused, measured with
  `edition = "2024"`, written or inherited: "`[project]` is not supported as of the 2024 Edition,
  please use `[package]`".

## Inheritance from `[workspace.package]`

- **A member inherits a key with `{key}.workspace = true`**: "The workspace.package table is where
  you define keys that can be inherited by members of a workspace. These keys can be inherited by
  defining them in the member package with {key}.workspace = true." Among the keys supported are
  `version`, `publish`, `repository` and `include`; "include and exclude are relative to your
  package root". The inline form `repository = { workspace = true }` is the same TOML, measured
  inherited.
- **A table whose `workspace` is `true` inherits, whatever other key it holds**, measured with
  `version = { workspace = true, other = 1 }` under a `[workspace.package]` setting `version`:
  Cargo exits 0 and reports the workspace's version; `cargo check` warns "unused manifest key:
  package.version.other". A table with no `workspace` key is refused, measured with
  `version = { other = 1 }`: "missing field `workspace`".
- **`workspace = false` is refused**, measured: "`workspace` cannot be false", with another key
  beside it or not.
- **A key inherited that `[workspace.package]` does not set is refused**, measured with
  `publish.workspace = true` under a `[workspace.package]` setting no `publish`: "error inheriting
  `publish` from workspace root manifest's `workspace.package.publish`".
- **Which manifest is the workspace's**: "Cargo will automatically search the parent directories
  for a Cargo.toml file with a [workspace] definition to determine which workspace to use. The
  package.workspace manifest key can be used in member crates to point at a workspace's root to
  override this automatic search."

## Which package Cargo would publish

Read from the passages of `[package]` above: `version` "is required for publishing packages", and
omitting it keeps a package from being published; `publish = false` disables publishing; and
`publish = [...]` names the registries a package may be published to. So a package Cargo would
publish sets a `version`, its own or inherited, with `publish` absent, `true`, or a list naming a
registry. **An empty list names no registry, and is read as `false`**: the reference does not say so
in words, and `cargo metadata` was measured to report `false` and `[]` alike, as `[]`. The reader of
`path@cargo-reader@src/manifest.rs` applies none of this, per
`design@cargo-reader@a-manifest-is-read-as-written`: the caller makes the reading, as the checker's
model does for the changelog crates.

## The `include` patterns, as `cargo package` matches them

The book's passage under "The `exclude` and `include` fields": "The patterns should be
gitignore-style patterns": `foo` "matches any file or directory with the name foo anywhere in the
package", `/foo` "only in the root of the package", `foo/` "any directory with the name foo";
`*`, `?` and `[]` are supported and match no `/`; `**/` "matches in any directory", `/**` "matches
everything inside", `/**/` "matches zero or more directories"; and "! prefix negates a pattern".

Measured with `cargo package --list --allow-dirty --offline`, Cargo 1.98.0, in a scratch package
`a` holding a `main.rs` in its directory `src` and one file more, its `Cargo.toml` setting `include`
to the patterns named, each verdict whether the listing holds that file. The match of
`path@cargo-reader@src/include.rs` answers each of them so. The patterns are in fenced blocks, one
per line, since a pattern is no pointer to a file of this project.

- **A file `CHANGELOG.md` at the package's root** is listed under each of these:

  ```text
  CHANGELOG.md  /CHANGELOG.md  *.md  /*.md  CHANGELOG*  CHANGE*.md  **/CHANGELOG.md
  /**/CHANGELOG.md  /**  **  /*  *  /  /[C]HANGELOG.md  /?HANGELOG.md
  ```

  and not under `changelog.md`, so the match is case-sensitive, nor under these, the first of
  which names a directory, then an empty pattern and `.`:

  ```text
  /CHANGELOG.md/  /src
  ```

- **The last matching pattern decides**: the file is not listed under `/CHANGELOG.md` then
  `!CHANGELOG.md`, nor under `/*` then `!/CHANGELOG.md`, and is listed under `!CHANGELOG.md` then
  `/CHANGELOG.md`.
- **A file `CHANGELOG.md` in a directory `docs` of the package** is listed under each of these:

  ```text
  docs  docs/  /docs/*  docs/**  CHANGELOG.md  */CHANGELOG.md  /*/CHANGELOG.md  **/docs
  docs/CHANGELOG.md  do*  /docs/**/CHANGELOG.md
  ```

  and not under these, nor under `docs` then `!docs/CHANGELOG.md`, nor under `docs/` then
  `!CHANGELOG.md`:

  ```text
  /docs/CHANGELOG.md/  a/**/b
  ```

  So a pattern matches a file through a directory that holds it, and a pattern with a `/` before
  its end is anchored at the root, as `*/CHANGELOG.md` shows.
