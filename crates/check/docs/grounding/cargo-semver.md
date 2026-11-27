# Cargo's semver: which number a release of each class bumps

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/reference/semver.html` of that toolchain,
and online at <https://doc.rust-lang.org/cargo/reference/semver.html>. They ground the bump check
of `path@check@src/check/changelog/bump.rs` and the bump of a version in
`path@model@src/changelogs/mod.rs`. Each is read again at the source it names.

## The classes

- **Three levels of change**: "Major change: a change that requires a major SemVer bump. Minor
  change: a change that requires only a minor SemVer bump. Possibly-breaking change: a change that
  some projects may consider major and others consider minor." The bump check knows the classes
  patch, minor and major; a possibly-breaking change is the writer's to class as one of the two, a
  judgement no check makes.

## Under `0.x`

- **The book's terms assume `1.0.0` or later**: "This guide uses the terms "major" and "minor"
  assuming this relates to a "1.0.0" release or later. Initial development releases starting with
  "0.y.z" can treat changes in "y" as a major release, and "z" as a minor release. "0.0.z" releases
  are always major changes. This is because Cargo uses the convention that only changes in the
  left-most non-zero component are considered incompatible."
- **What the check reads of it**: under `0.y.z`, a major change bumps `y`, and a patch bumps `z`.
  For a minor change the passage admits either number: `z`, as "a minor release", or `y`, which is
  always admitted, since a bump of the left-most non-zero component is one Cargo reads as
  incompatible and so never forbids. The check takes `y`, the reading the decided design of slice 3
  of `milestone@plans@per-crate-changelogs` gives, "under 0.x, major and minor both bump the second
  number and patch the third, as Cargo treats them", which is the one
  `design@knowledge-architect@versioning-policy` applies to this repository's releases.
- **`0.0.z` is read as any other `0.x`**: the check bumps `y` for a major change there too, where
  the passage says every `0.0.z` release is major, so a release that bumps `z` there is a finding
  of the check where the passage admits it.
