---
kind: deferred
---
# Every published crate takes a new version at each release, whether its code changed or not

## Summary

`design@knowledge-architect@version-lockstep` gives every published crate one version, so each
release publishes a new version of every crate. A crate that changes rarely, such as `gates`, still
takes one each time, and crates.io limits how many versions one account publishes in a short time.
Versioning each crate on its own would publish only the crates a release changes. The owner deferred
the question; the trigger is the next change that adds a published crate.

## Details

### What

- **Every crate is published at each release.** The release workflow's `publish` job,
  `path@knowledge-architect@.github/workflows/release.yml`, publishes the version of every crate of
  its list, and skips only a version crates.io already holds. Under the lockstep that version is new
  for every crate at each release, so a release that changes skill text alone publishes every crate
  with identical code, as `design@knowledge-architect@versioning-policy` says.
- **crates.io's limit on new versions counts per account.** It is a burst of 30, then 1 every
  minute, per `path@knowledge-architect@docs/grounding/crates-io-publishing.md`. A release of more
  than 30 crates meets an HTTP 429 in its `publish` job; a re-run of the failed job after the time it
  names completes it, per `design@knowledge-architect@partial-publish-rerun`. The workspace publishes
  11 crates on 2026-11-12, counted with
  `cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish != []) | .name' | wc -l`.
- **Crates that change rarely.** The owner reports that `gates` changes little, and that an agent
  bundler crate is planned whose updates would be rare. The owner expects such crates to become more
  frequent as the project settles: a report of the owner, not measured.
- **What per-crate versions would cost** is not established. The lockstep rests on the installed
  skills matching the checker that installed them, and on a project pinning one version for
  all of it, per `goal@every-target-project-can-adopt-it`; per-crate versions must keep both, and the
  exact pins between the crates, the changelog sections and the release skill change with them.

### Why it matters

Each release adds a version of every crate to crates.io, a crate whose code did not change included,
and its consumers see a new version to move to with nothing in it for them. Past 30 published crates,
every release also waits on the rate limit. It strains `design@knowledge-architect@version-lockstep`,
whose cost "a fix to one library alone is released as a version of every crate" grows with each crate
the project splits off.

### Trigger

A change that adds a published crate to the workspace: a Component creation or a split, which weighs
the cost of publishing the new crate, per `design@agent-skills@split-cost-by-publication`. Its work
then decides whether the new crate follows the lockstep, or this entry opens a design discussion
on per-crate versions, citing this entry. That discussion also decides whether
`design@agent-skills@split-cost-by-publication` names crates.io's limits on new crates and new
versions among the costs a split weighs.
