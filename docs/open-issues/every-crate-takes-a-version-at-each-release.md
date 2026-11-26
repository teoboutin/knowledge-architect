---
kind: todo
---
# Every published crate takes a new version at each release, whether its code changed or not

## Summary

`design@knowledge-architect@version-lockstep` gives every published crate one version, so each
release publishes a new version of every crate. A crate that changes rarely, such as `gates`, still
takes one each time. Versioning each crate on its own would publish only the crates a release
changes. The owner wants the question discussed before the next release: a new published crate
alone does not argue how crates are versioned, so no such crate is the occasion for it.

## Details

### What

- **Every crate is published at each release.** The release workflow's `publish` job,
  `path@knowledge-architect@.github/workflows/release.yml`, publishes the version of every crate of
  its list, and skips only a version crates.io already holds. Under the lockstep that version is new
  for every crate at each release, so a release that changes skill text alone publishes every crate
  with identical code, as `design@knowledge-architect@versioning-policy` says.
- **Crates that change rarely.** The owner reports that `gates` changes little, and that an agent
  bundler crate is planned whose updates would be rare. The owner expects such crates to become more
  frequent as the project settles: a report of the owner, not measured.
- **The number of crates does not slow a release.** crates.io's per-account limit on new versions
  does not apply to a publish by trusted publishing, which the release workflow makes, per
  `path@knowledge-architect@docs/grounding/crates-io-publishing.md`; the per-crate cap on versions a
  day that the same file names is met by no release, which publishes one version of each crate.
  Read in crates.io's source, not measured on a release.
- **What per-crate versions would cost** is not established. The lockstep rests on the installed
  skills matching the checker that installed them, and on a project pinning one version for
  all of it, per `goal@every-target-project-can-adopt-it`; per-crate versions must keep both, and the
  exact pins between the crates, the changelog sections and the release skill change with them.

### Why it matters

Each release adds a version of every crate to crates.io, a crate whose code did not change included,
and its consumers see a new version to move to with nothing in it for them. It strains
`design@knowledge-architect@version-lockstep`, whose cost "a fix to one library alone is released as
a version of every crate" grows with each crate the project splits off.

### What would close it

A design discussion, under `skill@knowledge-architect-design`, of how the published crates are
versioned, held before the next release, on the owner's word. It keeps
`design@knowledge-architect@version-lockstep` or reverses it. Where it reverses it, its plan
document schedules the work: the exact pins between the crates, the changelog sections and
`skill@klarch-release`. Where it keeps it, the head records the argument against per-crate
versions. Either way it decides whether `design@agent-skills@split-cost-by-publication` names
crates.io's limits on new crates among the costs a split weighs.

A new published crate is no occasion for it, such as the `cargo` crate that
`milestone@plans@per-crate-changelogs` adds, or the split of the skill-bundling passes that
`issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills` records:
the owner ruled, in the discussion that produced that milestone, that adding a crate never argues
versioning.
