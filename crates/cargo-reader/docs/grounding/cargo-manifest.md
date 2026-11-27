# Cargo's manifest format: what a `Cargo.toml` states of a package

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/reference/manifest.html` and
`path@elsewhere@share/doc/rust/html/cargo/reference/workspaces.html` of that toolchain, and online at
<https://doc.rust-lang.org/cargo/reference/manifest.html>; and measured with that Cargo, by
`cargo metadata --format-version 1 --no-deps --offline` in a scratch directory holding the manifest
named. They ground the reader of `path@cargo-reader@src/manifest.rs` and the match of the patterns
in `path@cargo-reader@src/include.rs`. Each is read again at the source it names, or measured again
with the command and manifest given.

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
- **`include`** lists "gitignore-style patterns" of the files packaged, and **`exclude`** "the
  patterns specified in the exclude field identify a set of files that are not included"; "The
  Cargo.toml file of the package itself is always included, it does not need to be listed in
  include", "If a license-file is specified, it is always included", and "setting include will
  override an exclude". The book's list of the files always included does not name the `readme`;
  Cargo packages it all the same, measured below.
- **`readme`** "should be the path to a file in the package root (relative to this Cargo.toml)"; a
  boolean is accepted too, measured: `readme = true` and `readme = false` are read.
- **`license-file`** "contains the path to a file containing the text of the license (relative to
  this Cargo.toml)".
- **`workspace`** "can be used to configure the workspace that this package will be a member of",
  a path; a value that is no string is refused as any field of the wrong type is.

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
  `version`, `publish`, `repository`, `include`, `exclude`, `readme` and `license-file`;
  "license-file and readme are relative to the workspace root; include and exclude are relative to
  your package root". The inline form `repository = { workspace = true }` is the same TOML,
  measured inherited.
- **`[workspace]` names its members and what it excludes**: `members` is "an array of strings
  containing directories with Cargo.toml files", which "also supports globs", and "The exclude key
  can be used to prevent paths from being included in a workspace".
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

## The `include` and `exclude` patterns, as `cargo package` lists the files

The book's passage under "The `exclude` and `include` fields": "The patterns should be
gitignore-style patterns": `foo` "matches any file or directory with the name foo anywhere in the
package", `/foo` "only in the root of the package", `foo/` "any directory with the name foo";
`*`, `?` and `[]` are supported and match no `/`; `**/` "matches in any directory", `/**` "matches
everything inside", `/**/` "matches zero or more directories"; and "! prefix negates a pattern".

Measured with `cargo package --list --allow-dirty --offline`, Cargo 1.98.0, in a scratch package
`a` holding a `main.rs` in its directory `src`, `README.md`, `LICENSE`, `CHANGELOG.md`, and a
`CHANGELOG.md` in its directory `docs`, the nested one, its `Cargo.toml` setting `include` or
`exclude` to the patterns named, each verdict whether the listing holds each of the two changelogs; the same in a copy that is a git repository with every file
committed, with the same verdicts. Every verdict of the tables in the tests of
`path@cargo-reader@src/include.rs` was measured so, and the match of that file answers each of
them so. A pattern shaped like a path is written in a fenced block, one per line, since a pattern
is no pointer to a file of this project. What the verdicts show:

- **An empty `include` is no `include`**: under `include = []` both files are listed, and an
  `exclude` beside it is read; under a list whose patterns are all empty or comments, `[" "]`,
  nothing is.
- **The patterns that match the file itself decide, the last of them winning; only where none
  matches it do those matching a directory that holds it, the nearest first**: the nested
  changelog is listed under the first of these then `!docs`, and not under `!docs/CHANGELOG.md`
  then `docs`, nor under `!CHANGELOG.md` then `/docs/`:

  ```text
  docs/CHANGELOG.md
  ```

- **The root is a directory with an empty name**: `*/`, `/*/`, `**/` and `/**/` list
  `CHANGELOG.md`, and `?*/` does not; `!/` excludes nothing a pattern of the file includes.
- **A trailing `/**` matches inside a directory alone**: the first three of these list neither
  file, and the last lists the nested changelog alone:

  ```text
  CHANGELOG.md/**  /CHANGELOG.md/**  **/CHANGELOG.md/**  docs/**
  ```

- **A pattern is read as a gitignore line**: one opening with `#` is a comment; trailing spaces and
  tabs are taken off, `CHANGELOG.md ` listing the file, unless a `\` escapes the last space,
  `CHANGELOG.md\ ` listing neither; a leading space is kept; a `\` makes the next character
  literal, `CHANGELOG\.md` listing both and `CHANGELOG.m\?` neither, and `\!` or `\#` opens a
  pattern with that character, measured with files named `!x` and `#x`; `!` alone negates every
  path, as `CHANGELOG.md` then `!` lists neither file under `include`, and both under `exclude`.
- **The grammar of a glob**: `{a,b}` matches either alternative, a group inside a group included,
  an empty alternative being dropped, as `{,docs/}CHANGELOG.md` lists the nested changelog alone; a
  `**` that is not a segment of its own is a `*`, as `CH**` lists both; in a bracket expression, a
  `]` first is a member, a `-` first or last is a member, and a `\` is a member, as
  `/[\]]HANGELOG.md` lists neither; an unclosed `[` is the character itself, measured with a file
  named `[C`.
- **A pattern Cargo refuses lists nothing**: under an unclosed `{`, a `}` that closes no group, a
  range whose end is below its start, as `[z-a]`, and a `\` that ends the pattern, Cargo fails with
  "failed to determine list of files", in `include` or in `exclude`, and in an `exclude` beside an
  `include` that is not empty.
- **Under `exclude`, a directory left out leaves out every file in it**, which no `!` pattern brings
  back: the nested changelog is not listed under `docs` then `!docs/CHANGELOG.md`; the root is never
  left out so, `CHANGELOG.md` being listed under `*/` then `!CHANGELOG.md`, and under `**` then
  `!CHANGELOG.md`.
- **`include` overrides `exclude`**: under `include = ["/src/**", "/CHANGELOG.md"]` beside
  `exclude = ["/CHANGELOG.md"]`, `CHANGELOG.md` is listed.
- **The files `readme` and `license-file` name are packaged whatever the patterns say**: under
  `include = ["/src/**"]`, `readme = "CHANGELOG.md"`, `readme = "./CHANGELOG.md"`,
  `readme = "docs/../CHANGELOG.md"` and `license-file = "CHANGELOG.md"` each list `CHANGELOG.md`;
  so they do under `exclude = ["/CHANGELOG.md"]`, and under `include` holding `!CHANGELOG.md`.
  Under `include = ["/src/**"]`, `readme = true` and `readme = false` list no changelog.
