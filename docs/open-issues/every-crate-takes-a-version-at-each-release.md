---
kind: deferred
---
# Every published crate takes a new version at each release, whether its code changed or not

## Summary

`design@knowledge-architect@version-lockstep` gives every published crate one version, so each
release publishes a new version of every crate. A crate that changes rarely, such as `gates`, still
takes one each time. Versioning each crate on its own would publish only the crates a release
changes. The owner deferred the question; the trigger is the design discussion of the next published
crate, the first expected being the split of the skill-bundling passes out of agent-skills.

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

### Trigger

The design discussion of a new published crate, the first expected being the split of the
skill-bundling passes out of agent-skills that
`issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills` records,
whose crate the owner expects to change rarely. That discussion decides how its crate is versioned
and released, and may reopen `design@knowledge-architect@version-lockstep` under
`skill@knowledge-architect-design`, which a Component creation alone weighs and never sets. It cites
this entry, and decides whether `design@agent-skills@split-cost-by-publication` names crates.io's
limits on new crates among the costs a split weighs.
