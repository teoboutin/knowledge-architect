# Slice 5: the conversion of this repository to per-crate changelogs

## Builds

- One `CHANGELOG.md` per published crate, 12 with `cargo-reader`, each holding its crate's entries of
  every version, converted from the shared changelog by the mapping table below, with the preamble
  written by hand from the expected text that slice 3's preamble finding gives.
- The `[changelog]` declaration in knowledge-architect.toml, as slice 3's spec illustrates it, and
  the `CHANGELOG.md` of every crate added to `[walk.citation-free] all`.
- The 13 rows of `[walk] skip-files` that list the changelogs removed, the root's and the 12
  crates', `cargo-reader`'s added by slice 2, with their comment.
- The root CHANGELOG.md replaced by the assembled file, which `cargo klarch check --fix` writes once
  the shared changelog is removed from its path, since its writer refuses a file without the
  generated header.
- `cargo x changelog` removed: `path@xtask@src/changelog.rs`, its dispatch, its line of
  `path@xtask@README.md`, and the preamble of `register@xtask@design` that names it. Its test's
  `include` assertion is now slice 3's check.
- The Migration entry of the converting release in `cli`'s changelog, from premortem cause 1: that
  the binary crate's changelog now holds its own entries only, and that a project moving its pin
  runs `cargo klarch changelog knowledge-architect --from <old>`.
- The rewrites and repairs of every text the milestone document's "What is already decided" lists
  for slice 5, and those this spec lists below, among them:
  - the root CLAUDE.md's "**Current reality only**: no changelogs outside CHANGELOG.md", in
    `instructions@language-and-style`, which is false once each crate holds its changelog;
  - step 6 of the on-hold `spec@plans@path-quickfixes`, which writes its entries with surfaces into
    the shared changelog, runs `cargo x changelog` twice, and names as a failure "a changelog copy
    that differs": slice 5 re-points it to the per-crate files and their checks.

## Claims

1. **No entry is lost or changed in content**: a script run in the slice's scratch space, kept in
   the commit message, reads every entry of the shared changelog and every entry of the 12 crate
   files and pairs them by the mapping table; every entry of the source maps to at least one entry
   of the targets with the same version, category, class and text, the text differing only in
   spans rewritten in words, and every target entry maps back. Shown to discriminate by running it
   on a copy with one entry deleted and one class changed.
2. **The conversion checks clean**: `cargo klarch check` on this repository passes with the
   changelogs at the level `all`: no span shaped as a reference, no unread line, no bump finding,
   the assembled file current.
3. **The assembled file holds every released version**: each version heading of the shared
   changelog, `Next release` included, is a version heading of the assembled file.
4. **The pin move across the conversion**: the Migration entry of "Builds" is in `cli`'s
   changelog under `Next release`; and `klarch changelog knowledge-architect --from 0.10.0`, run
   over the mock consumer of the fixture below, prints the release's Migration entries of `check`,
   `model` and `agent-skills`. The command excludes `Next release`, as slice 4 decided, so the
   fixture renames that section to the next version in its copies of the crates' files.
5. **Every rewritten span is a rewrite in words or a placeholder**: no span of the shared changelog
   reappears in a crate file with its backticks removed. Tested by the same script, on each span the
   source holds that the target does not.

## Fixtures

One, for claim 4: the mock consumer of slice 4, built in a copy outside this repository's tree by
the claim's run, its three stand-in packages replaced by copies of the converted `knowledge-architect`,
`knowledge-architect-check`, `knowledge-architect-model` and `knowledge-architect-agent-skills`
crates' `Cargo.toml` and `CHANGELOG.md`, each `## Next release` renamed to `## 0.11.0`, the version
the next release would take. It is a run kept in the commit message, not a committed fixture: the
slice converts this repository's own files, and its tests of the mechanism are slice 3's and
slice 4's.

## Audit subjects

- CHANGELOG.md, `path@xtask@src/changelog.rs`, knowledge-architect.toml, every crate's
  `Cargo.toml` and its `include`.
- Every decision the milestone document lists as reversed or rewritten, every text listed beside
  it, and the heads listed below.
- `skill@klarch-release`, `agent@klarch-changelog-reviewer`, the root CLAUDE.md's `## Git` point 1,
  its Component table and its knowledge table.
- `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`,
  `issue@knowledge-architect@the-publish-order-is-spelled-by-hand`, `spec@plans@path-quickfixes`
  step 6, and
  `path@xtask@docs/open-issues/the-reword-the-fold-and-the-merge-predicate-have-no-command.md`,
  which lists `changelog` among the commands of `path@xtask@src/`.

## Fails alone on

- This repository's own `cargo klarch check`, or claim 1's pairing of the entries.

## Premises that expire

- None: this is the last slice.

## What is already decided

The seeds of this slice's design audit are the design entries this section cites, as
`skill@knowledge-architect-planning@working-a-slice` takes them: those below, and the decisions the
milestone document lists as reversed or rewritten for slice 5, restated here:
`design@knowledge-architect@the-changelog-ships-in-every-crate`,
`design@knowledge-architect@the-changelog-is-outside-the-walk`,
`design@knowledge-architect@changelog-entries`, `design@agent-skills@audit-triggers` and
`design@agent-skills@a-reference-claims-a-revisit`.

Rejected entries this slice's harvest changes, as the milestone document's "What is already
decided" says: `rejected@knowledge-architect@changelog-copies-written-at-packaging`, its citation
moved to the successor head where the successor's argument covers it, or its leaving planned, as
`skill@knowledge-architect-decision-recording` says of a reversal; and
`rejected@knowledge-architect@changelog-read-by-every-check`, narrowed to its first reason, its
second defeated by the owner's reading at R3, and kept `live`.

The heads this slice's harvest touches, without reversing or rewriting them, drawn from the search
the milestone document's "What is already decided" names; the audit of each is owed at this slice's
design audit:

- `design@knowledge-architect@changelog-section-fixed-at-release`: its premise "Every published
  crate ships a copy of the root changelog" and its last paragraph, "No check reads the changelog
  … so no finding asks to change a released section", become false; its question on respelling a
  span is answered: a rewrite in words is structure.
- `design@knowledge-architect@package-include-whitelist`: "and the changelog ships the same way"
  cites the reversed head.
- `design@knowledge-architect@xtask-crate`: its list of subcommands loses `changelog`.
- `design@knowledge-architect@versioning-policy`: its pointer to which change gets a CHANGELOG.md
  entry.
- `design@knowledge-architect@changelog-reviewed-at-the-release`: the review keeps what no check
  asserts, now that the shape is checked.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: the branch writes in the
  crate's file.
- `design@agent-skills@tripwire-audit-axis`: judged at the audit, whether its "names this axis in
  its Migration entry" needs a change.

## Decided design

### An entry goes to the crate whose public surface the consumer meets the change through

From `thread@per-crate-changelogs@entry-owning-component`, with the withdrawal of
`thread@per-crate-changelogs@cross-crate-entries`.

- A finding of a check goes to `check`, even where phase 2 in `model` reports it; a manifest key to
  `model`; a command to `cli`; an API change to the crate whose API changes.
- An entry that concerns several crates is written in each crate whose consumers meet it, since
  every crate can be consumed on its own.
- The surface tag goes: the crate's file carries it.
- The reviewer of this slice checks each placement of a `checks`, `manifest` or `library` entry
  against this rule, entry by entry.
- Argument: `argument@per-crate-changelogs@arg-4`, `argument@per-crate-changelogs@arg-9`,
  Nearest rival: the crate whose code changes, defeated because it shows the reader the checker's internal layering.

### No changelog file holds a reference; the released spans are rewritten in words

From `thread@per-crate-changelogs@changelog-holds-no-citation`.

- Every crate's changelog is at the level `all`, and the assembled file is generated from them.
  The changelogs do not take the level `entries`: a released section is frozen and cannot follow a
  citation that later changes (the owner, R7).
- Each released or `Next release` span shaped as a reference is rewritten in words, or as a
  placeholder where it illustrates the grammar, never with its backticks removed, per
  `design@agent-skills@plain-text-is-no-repair`. This is a change of structure, on the owner's
  reading at R3, and the content of each entry is unchanged.
- The span `path@knowledge-architect@.cargo/config.toml` of the shared changelog's line 101 is rewritten in words, since
  the unanchored-path lint stays in force in a citation-free file.
- Argument: `argument@per-crate-changelogs@arg-2`, `argument@per-crate-changelogs@arg-29`,
  `argument@per-crate-changelogs@arg-45`, `argument@per-crate-changelogs@arg-46`,
  `argument@per-crate-changelogs@arg-58`.

### A Migration entry names an audit axis by its section title, verbatim

From `thread@per-crate-changelogs@axis-named-in-words`.

- A Migration entry for a change of the rules a project's recorded content must meet names the
  section that brings that content to them by its title, verbatim: an audit axis's section of
  `skill@knowledge-architect-project-audit`, or the review of the whole goal set of
  `skill@knowledge-architect-goal-setting`. It cites none.
- Step 2 of the pin move lists each Migration entry that names such a section by its title, and
  step 5 runs each, as today. `design@agent-skills@audit-triggers`, `design@knowledge-architect@changelog-entries`,
  `skill@knowledge-architect-project-audit` and the primer's lines on the Migration entry change to
  that wording.
- `design@agent-skills@a-reference-claims-a-revisit` is rewritten with a member: a citation-free
  file owes no reference, since it is read in a consumer's registry, where no reference resolves, a
  released section could not follow a rename of the axis anyway, and the `Next release` section is
  read by the release's review, which judges each entry's text.
- Argument: `argument@per-crate-changelogs@arg-34`, `argument@per-crate-changelogs@arg-47`. Nearest
  rivals: a dedicated `Audit` category, defeated because one entry is both a Migration and a
  trigger; a structured trigger field, defeated because it is specific to this repository.

### The heads of the reversal

`design@knowledge-architect@the-changelog-ships-in-every-crate` is replaced, in the root design
home, by the decided design of the milestone document's "One changelog per published crate" and
"What a project moving its pin reads", under the tests of
`skill@knowledge-architect-decision-recording`, its reversal done as
`skill@knowledge-architect-decision-recording@reversal-check` says.
`design@knowledge-architect@the-changelog-is-outside-the-walk` is rewritten under the new slug
`changelogs-hold-no-citation`, checked free with `cargo klarch id`, since the old slug and title
would name the rival once every crate's changelog is in the walk: the changelogs are in the walk,
read for their structure, at the level `all`, and no span of them resolves, which keeps the
consequence the head was about (the owner, R3). The harvest commit states the pair
`#changelog-holds-no-citation → changelogs-hold-no-citation`, and re-points every citation of the
old slug.

## Mapping tables

Each entry of the shared changelog, by its surface, to the crate file it goes to. The category is
the entry's subsection, the class and the text are unchanged but for spans rewritten in words, and
each entry of 0.1.0, which has neither subsection nor class, goes under `0.1.0`, `unclassed`.

| surface | crate file, by Component | rule |
| --- | --- | --- |
| `agent-skills` | crates/agent-skills | one crate |
| `cli` | crates/cli, package `knowledge-architect` | one crate |
| `gates` | crates/gates | one crate |
| `mutate` | crates/mutate | one crate |
| `checks` | crates/check, or crates/model where the consumer meets the change through `model`'s API | entry by entry, `thread@per-crate-changelogs@entry-owning-component` |
| `manifest` | crates/model | a manifest key is met through `model`'s read of the manifest |
| `library` | the crate whose API changes, or every crate whose consumers meet the change | entry by entry |

Each subsection to its category:

| subsection | category |
| --- | --- |
| Migration | Migration |
| New features | New features |
| Workflow | Workflow, restricted to agent-skills by `[changelog.category-components]`: each of the 225 Workflow entries of the shared changelog carries the surface `agent-skills`, counted at the commit that adds this spec by reading each entry's subsection with awk |

## Harvest

| what | thread | home |
| --- | --- | --- |
| one changelog per crate, the assembled file once, the pin move's reading | `thread@per-crate-changelogs@changelog-per-component`, `thread@per-crate-changelogs@changelog-the-pin-reads` | heads of `register@knowledge-architect@design` replacing the reversed head; the shared changelog and the shipped assembled file, A and B, in `register@knowledge-architect@rejected` where a recording test holds, and the root source file of `thread@per-crate-changelogs@cross-crate-entries` |
| the entry's crate | `thread@per-crate-changelogs@entry-owning-component` | a head of `register@knowledge-architect@design`, beside the rewritten `design@knowledge-architect@changelog-entries` |
| no citation in a changelog | `thread@per-crate-changelogs@changelog-holds-no-citation` | the rewritten `design@knowledge-architect@the-changelog-is-outside-the-walk`, under the slug `changelogs-hold-no-citation`, the pair `#changelog-holds-no-citation → changelogs-hold-no-citation` |
| the axis named in words | `thread@per-crate-changelogs@axis-named-in-words` | the rewritten `design@agent-skills@audit-triggers`, `design@knowledge-architect@changelog-entries` and `design@agent-skills@a-reference-claims-a-revisit` |
| the two rejected entries | | `rejected@knowledge-architect@changelog-copies-written-at-packaging`, cited by the successor head or leaving; `rejected@knowledge-architect@changelog-read-by-every-check`, narrowed, `live` |
| premortem causes 3 and 4 | `thread@per-crate-changelogs@entry-owning-component`, `thread@per-crate-changelogs@axis-named-in-words` | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires`, a tripwire of `register@knowledge-architect@tripwire` where admitted |
| premortem cause 1 | `thread@per-crate-changelogs@changelog-the-pin-reads` | the Migration entry of "Builds"; no tripwire |
