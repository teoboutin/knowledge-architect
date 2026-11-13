# Plan documents as directories, with model types of their own, and this milestone migrated into them

The first slice of `milestone@plans@split-guidelines-and-plan-directories`. It leaves when it lands,
under the rule of the planning skill as it stands at that landing. The threads, arguments and decided
design it shares with the other slices are in the milestone document; those only this slice uses are
below.

## Builds

- **`model`**: plan documents as model types of their own, beside the generic Directory shape, which
  stays as a declared register uses it, per
  `thread@split-guidelines-and-plan-directories@plan-model-first-class`:
  - a spec is a directory of specs/, `docs/plans/specs/<id>/`, holding `README.md`, and an anchor of
    its own, cited `spec@plans@<id>`, its items `<kind>@<id>@<item>`; a spec file of specs/ is a
    finding whose repair names the directory;
  - a milestone is a directory of milestones/, holding `README.md` and one directory per slice,
    `<slice>/README.md`, cited `spec@<milestone>@<slice>`; its items, and those of every slice, are
    cited `<kind>@<milestone>@<item>`, from inside the milestone's directory only;
  - in each plan document's directory, and in a milestone's directory: `deliberation.md`, live and
    item-bearing, holding the sections Threads and Arguments, whose level-three headings are items as
    today; `search/`, holding Markdown files; `transcript.txt`; nothing else, outside the slice
    directories of a milestone;
  - the regime of each file of the milestone document's "The regime of each file": `search/` and a
    harvested slice's `README.md` judged when written, `transcript.txt` never parsed, and the P3
    clause, a change to a frozen file after its first commit, other than its deletion, a finding;
  - a slice's state harvested, by a marker whose form the slice's design audit chooses, the
    discussion having fixed none; a citation from outside the milestone's directory of a harvested
    slice, or of an item it defines, refused, naming the milestone as the repair.
- **`check`**: the sections each document owes. `README.md` of a spec or of a milestone owes the
  sections of today's list but Threads and Arguments; `README.md` of a slice owes its entry's five
  sections and carries "What is already decided" when the slice has entries to seed; `deliberation.md`
  owes Threads and Arguments. The lints on ids and the reference checks read the new layout.
- **`query`**: `klarch anchors` lists the new anchors; `klarch worklist` takes the slice's
  `README.md` and the milestone's `README.md` as the work of a slice's audit, and `--seeds-from` the
  slice's `README.md`; `klarch show` lists the inbound references of an item wherever its document
  sits in the plan's directory.
- **`create`**: `klarch init` and `klarch new-component` write the plans directory as `model`
  defines it, through `owed_documents` and `home_text` in `path@create@src/command.rs`, with no home
  text that turns false at its first entry.
- **The shipped text**:
  - the planning skill: its terms, its layout, what a spec holds and in which file, the assembly
    (the transcript committed after the removal step; the owner's words attributed, citing the
    round; the search's output copied into `search/` after `--verify`), the slice audit's `--work` and
    `--seeds-from`, the P2 clause, the life of a slice's spec (harvested, then the milestone's
    directory leaving whole), and the deletion that ends a plan document's life;
  - the primer's source: `primer@checker-model` on plan documents and their anchors,
    `primer@where-knowledge-goes` on the plans directory row and on where the owner's verbatim words
    stay, `primer@design-heads` on the same, and `primer@coined-terms`;
  - every other shipped text that names a plan document's file, a slice's spec as a file, or the
    deletion of a slice's spec at its landing: among them the design skill, the review skill, the
    record searcher's, the transcript reviewer's, the cold implementer's and the design-conformance
    reviewer's definitions, found by reading the texts of Group A's `cargo klarch show` lists below.
- **This repository**:
  - `spec@plans@path-quickfixes`, migrated minimally: moved into its directory, its Threads and
    Arguments into `deliberation.md`, nothing else changed, per
    `thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`;
  - this milestone, migrated properly: its document and each slice's spec into the new layout, every
    condition of a decided thread in a document the search reads, the owner's verbatim words of its
    Threads and Arguments turned into attributions citing the round, the trimmed transcript of the
    discussion committed after the removal step, and the search's output committed in `search/`
    where the session's scratch copy of it is still on the machine, the commit saying so where it is
    not;
  - the 32 heads of Group A and the 13 of Group B rewritten, below, with every text that cites them,
    and `design@knowledge-architect@no-branch-sha-is-cited`, so that a SHA a transcript quotes is no
    citation;
  - CHANGELOG.md: the entries the change owes under the `Next release` section, a Migration entry
    among them, for adopting projects, which move their plan documents into directories, per
    `design@knowledge-architect@changelog-entries` as it stands.

## Claims

Each claim's test is shown to fail against a wrong implementation by `cargo x mutate`, per the
development skill, where the claim is a code claim; a claim on shipped text is checked by reading
the installed copy.

1. **A spec directory is an anchor, and a spec file is refused.** A mock project with
   `docs/plans/specs/<id>/README.md` and `deliberation.md` resolves `spec@plans@<id>` and each item
   of `deliberation.md`; the same project with `docs/plans/specs/<id>.md` reports one finding naming
   the directory. Mutation: the arm that admits a spec directory.
2. **A slice is a directory, and its items resolve in its milestone.** A mock milestone with two
   slice directories resolves `spec@<milestone>@<slice>` and an item of each slice's
   `deliberation.md` from the other slice's `README.md`.
3. **A declared Directory register is unchanged.** The tests of `design@model@declarable-directory-register`
   pass with no edit, and a declared Directory register still carries one register.
4. **A frozen file is judged when written, and never after.** In a mock project under git, a dangling
   reference in `search/a.md` is reported while the file is untracked or changed against HEAD, and
   is not reported once committed and its target removed in a later commit.
5. **The P3 clause.** In the same project, an edit of a committed `search/a.md` or of a harvested
   slice's `README.md` other than its deletion is reported; its deletion is not.
6. **The transcript is never parsed.** A reference-shaped span, a slug definition and a branch SHA in
   `transcript.txt` produce no finding of `check` or `commits`.
7. **A harvested slice is cited from inside its milestone only.** A citation of a harvested slice, or
   of an item it defines, from a file outside the milestone's directory is reported; from a sibling
   slice's `README.md` it resolves.
8. **A slice audit's work is two documents.** The planning skill, as installed, gives a slice audit's
   search `--work` the milestone's `README.md` and the slice's `README.md` and `--seeds-from` the
   slice's `README.md`, and names no other file; AC1's observable, read at slice 2's audit, is the
   test of record.
9. **The P2 clause.** The installed planning skill states that every condition of a decided thread
   is in the decided design of a document the search reads, and that `deliberation.md` holds only
   the argument; this milestone's migrated documents meet it, read by the slice's review.
10. **The P4 removal step.** The installed planning skill names a step before the transcript's commit
    that reads it and removes private information and pasted material; this milestone's committed
    transcript holds no path under the machine's home directory and no e-mail address, by `grep` over
    it before the commit.
11. **The migration keeps every citation resolving.** After the migration, `cargo klarch check`
    passes; `spec@plans@path-quickfixes`, `tripwire@check@fix-makes-a-choice` and
    `issue@check@no-command-applies-the-repair-a-message-finding-names`, which cite it, resolve; the
    spec's arguments that `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids`
    names are readable in its `deliberation.md`.
12. **Every commit of the branch passes the tip's checker.** `cargo klarch commits origin/main..HEAD`
    passes: the checker change and the migration of every plan document land in the branch's first
    commit, per `primer@branch-commits`.
13. **A new project gets the new layout.** `klarch init` over an empty directory writes a plans
    directory the check passes, and no home text of it is false after a first spec is added.

## Fixtures

The mock projects of `path@testing@tests/projects/`: a project with a spec directory, a milestone with
two slices, one harvested, and a `search/` and a `transcript.txt` in each, written in the layout this
slice builds.

## What is already decided

The seeds of this slice's audit, the decisions it rewrites, with every text `cargo klarch show` lists
as referencing each, on the commit before the milestone document's. **Each text is judged by this
slice's harvest**, which rewrites the head and every text that states what the head changes, and
re-points or rewrites every other citing sentence that the rewrite makes false. A text that is itself
rewritten by the slice, the planning skill among them, is judged by the step that rewrites it.

**Group A: the plan model and the plan documents' layout, the work's own reversals.**

- `design@model@plan-register`: `path@agent-skills@docs/design/owner-and-intent.md`,
  `path@check@README.md`, `path@model@docs/design/anchors.md` (3 sites),
  `path@model@docs/design/paths-and-links.md` (2 sites), `path@model@docs/design/registers.md`.
- `design@model@plans-split-dirs`: `path@check@README.md`.
- `design@model@plan-document-kinds`: `path@agent-skills@docs/design/plan-documents.md`,
  `path@check@docs/design/paths-and-links.md`, `path@check@README.md`,
  `path@model@docs/design/anchors.md`, `path@model@docs/design/paths-and-links.md` (2 sites),
  `path@model@docs/design/plan-documents.md`, `path@model@docs/design/registers.md`,
  `path@model@src/resolver.rs` (3 sites), `spec@plans@path-quickfixes` (2 sites).
- `design@model@plan-items-by-section`: `path@agent-skills@content/skills/planning/SKILL.md`,
  `path@agent-skills@docs/design/what-earns-a-record.md`, `path@check@README.md`,
  `path@model@docs/design/anchors.md`, `path@model@docs/design/plan-documents.md`,
  `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`.
- `design@model@plan-item-scope`: `path@agent-skills@docs/design/documents-point.md`,
  `path@agent-skills@docs/design/plan-documents.md` (3 sites),
  `path@check@docs/design/reference-lints.md`,
  `issue@check@a-plain-hash-item-name-is-read-by-nothing` (3 sites), `path@check@README.md`,
  `path@check@src/check/references.rs` (2 sites), `path@model@docs/design/entities-and-headings.md`,
  `path@model@docs/design/plan-documents.md`.
- `design@model@items-as-section-registers`: `path@model@docs/design/entities-and-headings.md`.
- `design@model@spec-file-is-an-anchor`: `path@check@README.md`,
  `path@model@docs/design/anchors.md`.
- `design@model@milestone-entries-are-directories`: `path@model@docs/design/plan-documents.md`,
  `path@model@src/manifest.rs`.
- `design@model@entry-anchor-carries`: `path@model@docs/design/registers.md`.
- `design@model@entry-anchor-path-kind`: `path@model@docs/design/registers.md`.
- `design@model@anchors-are-components-and-locations`:
  `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head`,
  `path@model@docs/design/anchors.md`, `path@model@docs/design/entities-and-headings.md`,
  `path@model@docs/design/plan-documents.md`, `register@query@design`,
  `spec@plans@path-quickfixes`.
- `design@model@an-entry-is-a-heading-at-the-register-level`:
  `path@agent-skills@content/agents/routing-reviewer.md`, `path@agent-skills@docs/design/goals.md`,
  `path@check@README.md`, `path@model@docs/design/entities-and-headings.md` (2 sites),
  `path@model@docs/design/reference-lints.md`,
  `issue@model@a-heading-line-markdown-renders-as-no-heading-defines-an-entry`.
- `design@check@plan-document-sections-by-kind`: `path@check@README.md`,
  `path@model@docs/design/plan-documents.md`.
- `design@check@a-file-register-is-a-directory-of-entries`:
  `path@agent-skills@content/agents/standing-state-reviewer.md`, `path@check@README.md`,
  `path@check@src/check/registers.rs`, `path@model@docs/design/entities-and-headings.md`.
- `design@check@every-path-names-its-anchor`:
  `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@check@docs/design/paths-and-links.md` (2 sites), `path@model@docs/design/anchors.md`,
  `path@model@docs/design/entities-and-headings.md`, `path@model@docs/design/plan-documents.md`,
  `path@model@src/resolver.rs`, `spec@plans@path-quickfixes` (5 sites).
- `design@check@unanchored-path-first-segment-here`:
  `path@agent-skills@content/agents/routing-reviewer.md` (2 sites),
  `path@check@docs/design/commit-messages.md`, `path@check@docs/design/paths-and-links.md`,
  `path@check@src/check/references.rs`, `path@model@docs/design/entities-and-headings.md`,
  `path@model@docs/design/reference-lints.md`, `spec@plans@path-quickfixes` (5 sites).
- `design@check@deepest-anchor-wins`: `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@check@docs/design/paths-and-links.md`,
  `issue@check@an-extension-cannot-see-which-register-a-document-belongs-to`,
  `issue@check@texts-citing-the-relocation-goal-state-its-old-measure`,
  `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head` (4 sites),
  `path@check@README.md`, `path@check@src/check/references.rs`, `path@model@docs/design/anchors.md`,
  `path@model@docs/design/paths-and-links.md`, `path@model@docs/design/reference-lints.md`,
  `path@model@docs/design/registers.md`,
  `issue@model@a-reference-to-an-anchor-s-own-directory-breaks-when-the-anchor-moves` (2 sites),
  `path@model@src/manifest.rs`, `path@model@src/resolver.rs`,
  `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`,
  `spec@plans@path-quickfixes`.
- `design@query@anchors-listing-command`: `path@query@src/anchors.rs`.
- `design@query@record-is-every-register-entry`: `path@check@docs/design/entities-and-headings.md`,
  `register@query@design`, `path@query@src/worklist.rs` (2 sites).
- `design@query@seeds-from-the-plan-s-decided-section`:
  `path@agent-skills@docs/design/record-search.md`, `register@query@design`.
- `design@agent-skills@milestone-is-a-directory`: no text cites it.
- `design@agent-skills@milestone-split-by-lifetime`:
  `path@agent-skills@docs/design/plan-documents.md` (2 sites),
  `a-slice-audit-s-search-reads-the-whole-milestone-document-in-every-agent` (closed by the commit that adds this milestone).
- `design@agent-skills@plan-document-leaves-at-landing`:
  `path@agent-skills@docs/design/owner-and-intent.md`,
  `path@agent-skills@docs/design/plan-documents.md` (2 sites).
- `design@agent-skills@ledger-from-transcript`: `path@agent-skills@docs/design/plan-documents.md`,
  `path@agent-skills@docs/design/record-search.md`,
  `path@agent-skills@docs/design/what-earns-a-record.md`.
- `design@agent-skills@plan-records-the-exchange`:
  `path@agent-skills@docs/design/owner-and-intent.md`,
  `plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them` (closed by the commit that adds this milestone).
- `design@agent-skills@attribution-replaces-quotation`:
  `path@agent-skills@docs/design/documents-point.md`,
  `path@agent-skills@docs/design/plan-documents.md`, `path@agent-skills@docs/design/reviews.md`.
- `design@agent-skills@no-progress-record`: no text cites it.
- `design@agent-skills@retiring-plan-opens-issue`: no text cites it.
- `design@agent-skills@standing-argument-in-head`:
  `path@agent-skills@docs/design/plan-documents.md`,
  `path@agent-skills@docs/design/project-configuration.md`,
  `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude` (2 sites),
  `issue@agent-skills@no-review-compares-the-readings-heads-take-of-one-goal`.
- `design@agent-skills@tripwire-horizon`:
  `path@agent-skills@content/agents/standing-state-reviewer.md`,
  `path@agent-skills@docs/design/audit-record-axes.md`,
  `path@agent-skills@docs/design/premortem-and-tripwires.md`,
  `path@agent-skills@docs/design/reviews.md`, `path@agent-skills@fragments/shallow-history.md`,
  `path@agent-skills@fragments/tripwire-lifecycle.md`,
  `path@agent-skills@fragments/tripwire-standard.md`, `path@check@docs/design/registers.md`.
- `design@agent-skills@search-reads-the-draft`: `path@agent-skills@content/skills/design/SKILL.md`,
  `path@agent-skills@docs/design/plan-documents.md`,
  `path@agent-skills@docs/design/record-search.md` (2 sites), `register@agent-skills@rejected`.
- `design@agent-skills@record-searched-before-the-work`:
  `path@agent-skills@content/skills/design/SKILL.md`,
  `path@agent-skills@content/skills/planning/SKILL.md`,
  `path@agent-skills@docs/design/project-configuration.md`,
  `path@agent-skills@docs/design/record-search.md`,
  `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/design/reviews.md` (2
  sites), `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search` (2 sites),
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`,
  `register@agent-skills@tripwire`.

`design@agent-skills@tripwire-horizon` names "plan documents deleted at their landing" among the
occasions a tripwire's horizon counts: a slice's spec is no longer deleted at its landing. Every
horizon written on that occasion is read by the harvest, found with
`grep -rn 'deleted at their landing'` over the tripwires homes, a lead to read beside, since a phrase
split over two lines escapes it.

**Group B: the heads that state the regime of every reference, which gain one member, the dated
record judged when written.**

- `design@agent-skills@a-reference-claims-a-revisit`:
  `path@agent-skills@docs/design/documents-point.md`,
  `path@agent-skills@docs/design/project-configuration.md`,
  `path@agent-skills@docs/design/what-earns-a-record.md`, `register@agent-skills@rejected`,
  `path@check@docs/design/registers.md`, `register@query@design`.
- `design@agent-skills@a-past-sentence-is-rewritten`:
  `path@agent-skills@docs/design/documents-point.md` (2 sites),
  `path@agent-skills@docs/design/owner-and-intent.md`,
  `issue@model@a-slug-in-a-quotation-is-reported-as-a-misplaced-definition` (2 sites),
  `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`.
- `design@agent-skills@plain-text-is-no-repair`: `path@agent-skills@content/PRIMER.md`,
  `path@agent-skills@docs/design/documents-point.md`,
  `path@agent-skills@docs/design/owner-and-intent.md`,
  `path@agent-skills@docs/design/record-search.md`,
  `path@agent-skills@docs/design/retrospective.md`,
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`,
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` (4 sites),
  `path@check@docs/design/entities-and-headings.md`, `path@check@docs/design/reference-lints.md` (3
  sites), `issue@check@a-plain-hash-item-name-is-read-by-nothing`,
  `issue@check@cross-project-references`, `path@check@src/check/references.rs` (3 sites),
  `path@model@docs/design/paths-and-links.md` (2 sites), `path@model@src/scan.rs`,
  `spec@plans@path-quickfixes` (5 sites).
- `design@check@the-regime-has-no-opt-out`: `path@agent-skills@docs/design/documents-point.md`,
  `path@check@docs/design/entities-and-headings.md`, `path@check@docs/design/extensions-and-api.md`,
  `path@check@docs/design/prose.md` (2 sites), `register@check@rejected` (2 sites),
  `path@model@docs/design/anchors.md`, `path@model@src/tree.rs`.
- `design@check@nothing-of-a-project-is-compiled-in`:
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`,
  `path@check@CLAUDE.md`, `path@check@docs/design/prose.md`, `path@check@README.md`,
  `register@create@design`,
  `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`,
  `path@create@src/command.rs` (2 sites), `path@model@docs/design/anchors.md` (2 sites),
  `path@model@docs/design/plan-documents.md`.
- `design@check@reference-shape-outside-span`:
  `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@agent-skills@docs/design/documents-point.md`,
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` (2 sites),
  `path@check@docs/design/reference-lints.md`, `path@check@README.md`,
  `path@check@src/check/references.rs`,
  `issue@model@string-literals-in-code-may-not-need-reading-at-all`,
  `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids`.
- `design@check@backticked-bare-id`: `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@agent-skills@docs/design/documents-point.md` (2 sites),
  `path@check@docs/design/paths-and-links.md`, `path@check@docs/design/reference-lints.md` (3
  sites), `issue@check@tooling-for-project-skills`, `path@check@README.md`,
  `path@check@src/check/references.rs`, `path@model@docs/design/entities-and-headings.md`,
  `path@model@docs/design/paths-and-links.md`, `path@model@docs/design/reference-lints.md`,
  `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` (3 sites),
  `spec@plans@path-quickfixes` (7 sites).
- `design@check@plain-word-lint`: `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@check@docs/design/reference-lints.md` (2 sites),
  `issue@check@a-plain-hash-item-name-is-read-by-nothing`,
  `issue@check@no-command-applies-the-repair-a-message-finding-names`,
  `issue@check@tooling-for-project-skills`, `path@check@README.md`,
  `path@check@src/check/references.rs` (3 sites), `path@model@docs/design/entities-and-headings.md`,
  `path@model@src/scan.rs`,
  `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` (2 sites).
- `design@model@a-reference-is-live-in-every-prose`:
  `path@agent-skills@docs/design/documents-point.md`, `path@check@docs/design/paths-and-links.md`,
  `path@check@docs/design/reference-lints.md`, `path@check@README.md`,
  `path@model@docs/design/entities-and-headings.md`.
- `design@model@an-unresolved-reference-is-never-dropped`:
  `path@agent-skills@docs/design/documents-point.md`.
- `design@model@an-unresolved-reference-is-reported`:
  `path@agent-skills@content/agents/routing-reviewer.md`,
  `path@model@docs/design/entities-and-headings.md`,
  `issue@model@the-resolver-s-arms-have-mutation-survivors`, `path@model@src/resolver.rs` (2 sites).
- `design@model@planned-path-form`: `path@agent-skills@docs/design/documents-point.md` (2 sites),
  `path@check@src/check/references.rs`, `path@model@docs/design/entities-and-headings.md` (3 sites),
  `path@model@docs/design/paths-and-links.md`, `path@model@src/entity.rs`,
  `path@model@src/manifest.rs`, `path@model@src/records.rs`, `path@model@src/resolver.rs` (3 sites),
  `path@query@src/show.rs`.
- `design@model@git-supplies-the-walk`: `path@check@README.md`, `path@check@src/check/mod.rs`,
  `path@check@src/commands/mod.rs`, `register@create@design`, `register@git@design` (2
  sites), `path@git@src/git.rs`, `path@model@CLAUDE.md`,
  `path@model@docs/design/paths-and-links.md`,
  `issue@model@check-under-a-sparse-checkout-reports-every-file-outside-the-cone` (2 sites),
  `issue@model@the-walk-does-not-report-what-the-per-clone-exclude-file-removes`,
  `register@model@tripwire`, `path@model@src/manifest.rs`, `path@model@src/walk.rs`,
  `register@mutate@design`, `register@knowledge-architect@design`.

**Rewritten for the committed transcript**: `design@knowledge-architect@no-branch-sha-is-cited`, so
that a SHA a transcript quotes is a quotation, not a citation.

**Bound and not rewritten**: `design@agent-skills@thread-slug-is-entry-id` and
`design@agent-skills@arguments-as-items`, Group C, since `deliberation.md` keeps the items; the slice
rewrites only a sentence of theirs that names the file holding the items. `design@agent-skills@branch-commits-judged-at-tip`,
`design@agent-skills@agents-write-in-scratch`, `design@agent-skills@verdicts-reach-the-conformance-review`,
`design@agent-skills@transcript-extraction-is-the-checker-s`, `design@agent-skills@transcript-found-by-listing`,
`design@create@init-command` and `design@create@component-command`, as the milestone document says of
each. `design@model@declarable-directory-register`, which the plan model leaves as it is.

**To read again**: `rejected@model@subdirectory-anchors-location` rests its argument in part on the
milestone needing the Directory shape; once the milestone has a type of its own, the harvest judges
whether that argument still stands, per `skill@knowledge-architect-decision-recording@losing-alternatives`.

## Audit subjects

- `path@model@src/manifest.rs`: `Registers::built_in`, the plan constants, `PLAN_SECTIONS` and
  `SLICE_SECTIONS`; `path@model@src/entity.rs`, `path@model@src/tree.rs`,
  `path@model@src/resolver.rs`, `path@model@src/walk.rs` (`LIVE_SUFFIXES`) and
  `path@model@src/shipped_set.rs`.
- `path@check@src/check/registers.rs` and `path@check@src/check/references.rs`.
- `path@query@src/worklist.rs`, `path@query@src/graph.rs` and `path@query@src/anchors.rs`.
- `path@create@src/command.rs`: `owed_documents` and `home_text`.
- The planning skill's source, the primer's source, and every shipped text the lists above name.
- The design homes `path@model@docs/design/plan-documents.md`,
  `path@agent-skills@docs/design/plan-documents.md` and
  `path@agent-skills@docs/design/record-search.md`.
- The standing entries the slice's code touches: `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`,
  `issue@check@a-plain-hash-item-name-is-read-by-nothing`,
  `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`,
  `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`,
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`,
  `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids`,
  `issue@knowledge-architect@path-quickfixes-read-against-the-renamed-checker` and
  `issue@model@the-resolver-s-arms-have-mutation-survivors`.

## Fails alone on

- `cargo klarch check` refusing the migrated tree, or a mock project of the new layout.
- A slice audit's `--work`, at slice 2's audit, holding anything but the two `README.md` files.

## Premises that expire

- The milestone document and the slice specs this slice migrates are written in the layout of today;
  this slice ends that, and its migration is their first real use. Watched by Claim 11 and by AC1 at
  slice 2's audit.
- Until this slice lands, a slice's spec is deleted at its landing, and the tripwires' horizons count
  that occasion; this slice ends it, and its harvest reads every horizon written on it.

## Threads

### Every plan document is a directory: a spec, a milestone, and each slice inside its milestone `##plan-document-directory`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-9` and
  `argument@split-guidelines-and-plan-directories@arg-11`; shaped by the session in round 1,
  `argument@split-guidelines-and-plan-directories@arg-27`, on the measurement of
  `argument@split-guidelines-and-plan-directories@arg-20`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-45`: adopting projects migrate their
  plan documents, and `init` and `new-component` write the new layout.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, a spec as one file,
  has no place for the artifacts. The owner's word on the checkpoint table, quoted in the milestone
  document's Threads, left it standing.
- Its shape: this spec's "Decided design", and the milestone document's "The plan document's
  directory". Harvest: this slice's, into `design@model@spec-file-is-an-anchor`,
  `design@model@milestone-entries-are-directories`, `design@agent-skills@milestone-is-a-directory`
  and the other heads of Group A.

### Plan documents get model types of their own in `model`, rather than riding on the generic Directory shape `##plan-model-first-class`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-14`; shaped by
  the session in round 1, `argument@split-guidelines-and-plan-directories@arg-31`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-45`.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, extending the
  Directory shape, collides with the owner's ruling that a declared Directory register carries one
  register, `design@model@declarable-directory-register`. The owner's word on the checkpoint table
  left it standing.
- Its shape: this spec's "Decided design". Harvest: this slice's, into `design@model@plan-register`,
  `design@model@plan-document-kinds`, `design@model@entry-anchor-carries` and the other model heads
  of Group A, with `rejected@model@subdirectory-anchors-location` read again.

### The first slice migrates the existing plan documents: `spec@plans@path-quickfixes` minimally, this milestone properly `##plan-migration-in-first-slice`

- Proposed by the owner in round 2, `argument@split-guidelines-and-plan-directories@arg-36`, answering
  Q3, whose default the session had put in round 1, `argument@split-guidelines-and-plan-directories@arg-33`.
- Final state: `approved`.
- The owner's words, round 2: "Q3 looks right for now. The first slice will also need to migrate the
  existing plans, path-quickfixes, and also the milestone we will write in this session. The
  path-quickfix migration should be kept minimal, the milestone one maybe not, because having it done
  properly will also provide a first real test of the new structure." Closed by the owner's word on
  the checkpoint table.
- Its shape: this spec's "Builds", "This repository", and its mapping table. Harvest: none of its
  own; the migration is work, and its decision is recorded by the heads of Group A.

## Arguments

### Plan documents as directories hold artifacts of their own `##arg-11`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`.
"I think that specs should be one directory, and not just one file. Because standalone specs tend to
show the same defect, they become very large documents. That would allow them to hold separate data
as artifacts too, such as the transcript."

### Specs and milestones as first-class models, leaving the Directory shape as it is `##arg-14`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@plan-model-first-class`.
"Implementation wise, I think that we should consider giving plan documents (specs and milestones) and
their directory structure their own arm of the model, not reusable. They diverge quite a bit from the
others, with lots of additional items, and we are changing them again. I like the current Directory
shape that is available for custom registers, so rather than changing it, I think that giving Specs
and Milestones their own first class models in the model crate is a better direction."

### A directory reopens the rival of `design@model@spec-file-is-an-anchor`, on artifacts with no other home `##arg-27`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`.
"Every plan document becomes a directory. This reopens the rival recorded in
`design@model@spec-file-is-an-anchor` ("Making every spec a directory was the rival"). The new
argument is that a plan document now has artifacts with no other home."

### The milestone already departs from the declared Directory shape in three ways `##arg-31`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-model-first-class`.
"The investigator mapped about 15 code sites across five crates that already special-case plans. The
milestone already departs from the declared Directory shape in three ways: it carries five registers,
where a declared Directory register carries one, "on the owner's ruling"; it has no `path` kind; its
sections are compiled into the checker. Extending the generic shape instead would collide with that
ruling. One consequence: `rejected@model@subdirectory-anchors-location` partly rests its argument on
the milestone needing the Directory shape, so that argument has to be read again." The milestone
document's "What the work is" re-counts the files that name the plan model's constants: 10 files of
four crates.

### One milestone, plans first, so the later slices are planned in the new shape `##arg-33`

Round 1, the session, putting Q3. Bears on
`thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`. "the default is one
milestone, slices in this order: 1. plan documents: model, checker, planning skill; 2. split
guidelines and the two scouts; 3. pre-harvest conformance; 4. the Component argument, and the goal.
Plans go first so the later slices are planned in the new shape. The catch: this milestone's own
README would be written in the old shape, so it would be about as large as the ones it fixes."

### A proper migration of this milestone is the first real test of the new structure `##arg-36`

Round 2, the owner. Bears on
`thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`. "The first slice will
also need to migrate the existing plans, path-quickfixes, and also the milestone we will write in this
session. The path-quickfix migration should be kept minimal, the milestone one maybe not, because
having it done properly will also provide a first real test of the new structure."

### Adopting projects migrate their plan documents too `##arg-45`

Round 3, the session. Bears on `thread@split-guidelines-and-plan-directories@plan-document-directory`
and `thread@split-guidelines-and-plan-directories@plan-model-first-class`. "**Adopting projects must
migrate their plan documents too.** Every pin crossing the release that makes plan documents
directories gets a Migration entry, and `init` and `create` read the new plan model."

## Decided design

### Every plan document a directory

For `thread@split-guidelines-and-plan-directories@plan-document-directory`. The layout is the
milestone document's "The plan document's directory". What this slice adds to it:

- a spec of specs/ and a slice of a milestone take the same shape, `README.md` and the files beside
  it, so the planning skill states one shape of a plan document's directory;
- the anchor of a spec is its directory, and its items are cited by its id, as today's
  `design@model@spec-file-is-an-anchor` states for a spec file;
- the nearest rival, a spec as one file, which `design@model@spec-file-is-an-anchor` chose, lost to the
  artifacts a plan document now has, `deliberation.md`, `search/` and `transcript.txt`, which a file
  cannot hold beside it without a second home for each.

### Model types of their own

For `thread@split-guidelines-and-plan-directories@plan-model-first-class`.

- The plan model leaves `Registers::built_in`'s use of the Directory and File shapes for plan
  documents, and gets types of its own in `model`; the slice names them. The Directory shape stays as
  a declared register uses it, carrying one register, per
  `design@model@declarable-directory-register`.
- The nearest rival, extending the Directory shape to carry plan documents, lost to the owner's ruling
  recorded in `design@model@declarable-directory-register`, that a declared Directory register carries
  one register, where a milestone carries five, and to the two other departures of
  `argument@split-guidelines-and-plan-directories@arg-31`.

### The migration of the tree's plan documents

For `thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`, in the branch's
first commit with the checker change, per `primer@branch-commits`.

- `spec@plans@path-quickfixes`: moved into its directory, its Threads and Arguments sections into its
  `deliberation.md`, with no other edit: its words stay as written, the owner's verbatim words
  included, and no transcript is committed for it.
- This milestone: each document into its place in the mapping table below; the conditions of each
  decided thread written into the decided design of the document the search reads, per P2; the
  owner's verbatim words in its Threads and Arguments turned into attributions citing the round, and
  the transcript of the discussion committed, after the removal step, as its `transcript.txt`; the
  search's output committed in `search/` where the session's scratch copy is still on the machine.

## Mapping tables

**Each plan document of the tree, and where it goes.** Over every plan document the tree holds when
the slice starts; a plan document added later is mapped by the same rule.

| today | after this slice |
| --- | --- |
| `docs/plans/specs/<id>.md` | `docs/plans/specs/<id>/README.md`, its Threads and Arguments sections in `deliberation.md` beside it |
| `docs/plans/milestones/<id>/README.md` | the same path, its Threads and Arguments sections in `docs/plans/milestones/<id>/deliberation.md` |
| `docs/plans/milestones/<id>/<slice>.md` | `docs/plans/milestones/<id>/<slice>/README.md`, its Threads and Arguments sections in `deliberation.md` beside it |
| `docs/plans/milestones/<id>/index.md`, generated | whatever generated listing the slice's model defines, written by `cargo klarch index` |
| the discussion's transcript, not in the tree | `transcript.txt` beside the milestone document, this milestone only |
| the search's output, in the session's scratch space | `search/` beside the milestone document, this milestone only, where it is still on the machine |

## Losing alternatives

- **A spec as one file**, under `thread@split-guidelines-and-plan-directories@plan-document-directory`:
  lost to the artifacts with no other home, `argument@split-guidelines-and-plan-directories@arg-27`.
- **The generic Directory shape extended to carry plan documents**, under
  `thread@split-guidelines-and-plan-directories@plan-model-first-class`: lost to
  `design@model@declarable-directory-register` and the three departures of
  `argument@split-guidelines-and-plan-directories@arg-31`.
- **This milestone migrated as minimally as `spec@plans@path-quickfixes`**, under
  `thread@split-guidelines-and-plan-directories@plan-migration-in-first-slice`: lost to the owner's
  argument that a proper migration is the first real test of the new structure,
  `argument@split-guidelines-and-plan-directories@arg-36`.

## Implementation sequence

1. **The plan model, empty.** The new types in `model`, the anchors and the regime of each file,
   with the mock projects of the fixtures, and no shipped text changed: Claims 1 to 7. With the
   migration of step 2 in the same commit, since the tree's plan documents fail the new checker
   until they move.
2. **The migration of the tree.** `spec@plans@path-quickfixes` and this milestone, by the mapping
   table: Claims 11 and 12, and the P4 grep of Claim 10 before the transcript's commit.
3. **`query` and `create`.** The work of a slice audit, `--seeds-from`, `anchors`, `show`, and the
   layout `init` writes: Claims 8, in its code half, and 13.
4. **The shipped text.** The planning skill, the primer and the other texts: Claims 8, 9 and 10, in
   their text halves.
5. **The record and the changelog.** The heads of Groups A and B and
   `design@knowledge-architect@no-branch-sha-is-cited` rewritten with their citing texts, and the
   CHANGELOG.md entries with the Migration entry. This is the harvest.

## Harvest

- **Design heads**: the heads of Groups A and B, and `design@knowledge-architect@no-branch-sha-is-cited`,
  rewritten under `skill@knowledge-architect-decision-recording`, each with the texts the lists above
  give; `rejected@model@subdirectory-anchors-location` judged again. The decisions of
  `thread@split-guidelines-and-plan-directories@plan-document-directory`,
  `thread@split-guidelines-and-plan-directories@plan-model-first-class`,
  `thread@split-guidelines-and-plan-directories@discussion-record-apart`,
  `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`,
  `thread@split-guidelines-and-plan-directories@committed-transcript` and
  `thread@split-guidelines-and-plan-directories@harvested-slice-kept` land in those heads, or in a head
  of their own where the entry tests of `primer@design-heads` give one.
- **Losing alternatives**: those above and those of the milestone document's threads this slice
  builds, judged by the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`.
- **The premortem's causes**: P5 judged as a cause, by the standard of
  `skill@knowledge-architect-decision-recording@ruled-tripwires`; and the count of attribution slips
  that the closing condition of the issue on verbatim words named, judged as a cause guarding the
  rewritten `design@agent-skills@attribution-replaces-quotation`.
- **This slice's spec** is marked harvested after the review before the merge, under the rule it
  builds, rather than deleted.
