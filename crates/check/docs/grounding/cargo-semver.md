# Cargo's semver: which versions a release of each class may take

Findings read in the Cargo book shipped with the toolchain 1.98.0, `cargo 1.98.0 (797e8a9bc
2026-08-05)`, at `path@elsewhere@share/doc/rust/html/cargo/reference/semver.html`,
`path@elsewhere@share/doc/rust/html/cargo/reference/specifying-dependencies.html` and
`path@elsewhere@share/doc/rust/html/cargo/reference/manifest.html` of that toolchain, and online at
<https://doc.rust-lang.org/cargo/reference/semver.html>; and measured with that Cargo. They ground
the bump check of `path@check@src/check/changelog/bump.rs` and the versions of
`path@model@src/changelogs/mod.rs`. Each is read again at the source it names, or measured again
with the command given. Which of the readings below the check takes, and why, is
`design@check@changelog-bump-check`.

## The classes

- **Three levels of change**: "Major change: a change that requires a major SemVer bump. Minor
  change: a change that requires only a minor SemVer bump. Possibly-breaking change: a change that
  some projects may consider major and others consider minor." A possibly-breaking change is the
  writer's to class as one of the two, a judgement no check makes.

## Which bump Cargo reads as incompatible

- **The book's terms assume `1.0.0` or later**: "This guide uses the terms "major" and "minor"
  assuming this relates to a "1.0.0" release or later. Initial development releases starting with
  "0.y.z" can treat changes in "y" as a major release, and "z" as a minor release. "0.0.z" releases
  are always major changes. This is because Cargo uses the convention that only changes in the
  left-most non-zero component are considered incompatible."
- **The requirements Cargo resolves state the same line**: "Versions are considered compatible if
  their left-most non-zero major/minor/patch component is the same", with `1.2.3 := >=1.2.3, <2.0.0`,
  `0.2.3 := >=0.2.3, <0.3.0` and `0.0.3 := >=0.0.3, <0.0.4`.

So a bump of the left-most non-zero number is incompatible, and a bump of a number to its left is
too, since it changes that number from zero. The readings of a release after each kind of version:

- **At `1.0.0` and above**: a major change bumps the first number, a minor change the second, a
  patch the third, each as the book's terms say.
- **Under `0.y.z`, `y` at least 1**: a major change bumps `y`, or gives `1.0.0`, both incompatible;
  a patch bumps `z`. For a minor change the passage admits either number: `z`, as "a minor
  release", or `y`, a bump Cargo reads as incompatible and so never forbids.
- **Under `0.0.z`**: every release is "always major", so every bump of `z` is one Cargo reads as
  incompatible, whatever the class of its changes; a bump to `0.1.0` or to `1.0.0` changes a number
  from zero and is incompatible too.
- **No version follows the largest number**: measured with the same command as below, Cargo reads
  `version = "0.18446744073709551615.0"`, and refuses `0.18446744073709551616.0`, "value of minor
  version number exceeds u64::MAX", so a number that is `u64::MAX` has no successor.

## What a version is

- **Three numbers, a pre-release and a build part**: "Versions must have three numeric parts, the
  major version, the minor version, and the patch version. A pre-release part can be added after a
  dash such as 1.0.0-alpha", and "A metadata part can be added after a plus, such as
  1.0.0+21AF26D3. This is for informational purposes only and is generally ignored by Cargo." The
  book gives no class to a release whose version is a pre-release.
- **Build metadata orders as the version without it**: item 10 of the Semantic Versioning
  specification 2.0.0, <https://semver.org/spec/v2.0.0.html>, read on 2026-11-27: "Build metadata
  MUST be ignored when determining version precedence. Thus two versions that differ only in the
  build metadata, have the same precedence."
- **No leading zero**: measured with `cargo metadata --format-version 1 --no-deps --offline` over a
  package whose `version` is `01.2.3`, Cargo refuses it, "invalid leading zero in major version
  number".
