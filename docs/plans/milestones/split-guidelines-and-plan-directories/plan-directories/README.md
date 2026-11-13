# Plan documents as directories, with model types of their own, and this milestone migrated into them

The first slice of `milestone@plans@split-guidelines-and-plan-directories`. When it lands, it is
moved under its milestone's `harvested/` and stays there until the milestone leaves, under the rule
it builds. The threads, arguments and decided design it shares with the other slices are in the
milestone document; those only this slice uses are below.

It grounds, per `skill@knowledge-architect-planning@working-a-slice` point 1, in the Components its
work touches: `model`, `check`, `query`, `create`, `agent-skills`, `testing` and the root.

## Builds

- **`model`**: plan documents as model types of their own, beside the generic Directory shape, which
  stays as a declared register uses it, per
  `thread@split-guidelines-and-plan-directories@plan-model-first-class`:
  - a spec is a directory of specs/, `docs/plans/specs/<id>/`, holding `README.md`, and an anchor of
    its own, cited `spec@plans@<id>`, its items `<kind>@<id>@<item>`; a spec file of specs/ is a
    finding whose repair names the directory. specs/ keeps the groups the File shape gives it today
    (`spec_files` in `path@model@src/entity.rs` reads `docs/plans/specs/<group>/<id>.md` as the
    spec `<id>`): a subdirectory of specs/ or of a group that holds a `README.md` is a spec, any
    other is a group, per `design@check@groups-are-subdirectories`;
  - a milestone is a directory of milestones/, holding `README.md` and one directory per slice,
    `<slice>/README.md`, cited `spec@<milestone>@<slice>`; its items, and those of every slice, are
    cited `<kind>@<milestone>@<item>`, from inside the milestone's directory only. **A slice's
    directory is no anchor**: the milestone is the anchor of every file under its directory, its
    slices' included, so it owns their items, as today it owns the items of its slice files, per
    `design@model@an-entity-belongs-to-its-anchor`;
  - in each plan document's directory, and in a milestone's directory, beside `README.md`:
    `deliberation.md`, live and item-bearing, holding the sections Threads and Arguments, whose
    level-three headings are items as today, owed where the document has a thread or an argument;
    `search/`, holding the `.txt` files of generated lines the milestone document's "The plan
    document's directory" names, `discussion.txt` and `design-audit.txt`; `transcript.txt`; nothing else, but a milestone's generated `index.md`, its slice
    directories and its `harvested/`, as that section says;
  - the regime of each file of the milestone document's "The regime of each file", D1's design as
    the owner ruled it: the one frozen location, a slice's directory under `harvested/`, left out of
    the walk by construction and parsed for definitions only, as
    `design@check@installed-files-outside-the-walk` makes the installed files; a definition in a
    frozen file still counts against a live one of the same id;
  - a slice's state harvested, marked by the move of its directory under its milestone's
    `harvested/`; a citation from outside the milestone's directory of a harvested slice, or of an
    item it defines, refused, naming the milestone as the repair;
  - the repair strings of `directory_layout` in `path@model@src/tree.rs`, which today report every
    subdirectory of a milestone and say "a milestone holds its README, its index and one file per
    slice", rewritten to the new layout; today every `.md` beside a milestone's `README.md` reads as
    a slice spec, so a `deliberation.md` there would read as a slice named `deliberation`.
- **`check`**: the sections each document owes. `README.md` of a spec or of a milestone owes the
  sections of today's list but Threads and Arguments; `README.md` of a slice owes its entry's five
  sections and carries "What is already decided" when the slice has entries to seed; `deliberation.md`
  owes Threads and Arguments. The lints on ids and the reference checks read the new layout.
  `cargo klarch commits` reports the P3 clause: a commit that modifies a file under `harvested/`,
  other than adding it, deleting it, or the move that freezes a slice. The rule on branch SHAs
  already skips every `.txt` file, `transcript.txt` and `search/` included, since
  `branch_sha_findings` in `path@check@src/commands/history.rs` reads only the walked documents,
  and under D1 it skips a harvested slice too: rewriting `design@knowledge-architect@no-branch-sha-is-cited`
  changes the rule's text, not that check.
- **`query`**: `klarch anchors` lists the new anchors; `klarch worklist --seeds-from` accepts a plan
  document's directory or its `README.md`, where its doc comment in `path@cli@src/cli/mod.rs` today
  refuses "A path that is no plan document", and seeds from the `README.md`'s section "What is
  already decided"; `--work` already takes any file, so a slice audit's work, the slice's and the
  milestone's `README.md`, needs no change; `klarch show` lists the inbound references of an item
  wherever its document sits in the plan's directory.
- **`create`**: `klarch init` writes the plans directory as `model` defines it, through
  `owed_documents` and `home_text` in `path@create@src/command.rs`, with no home text that turns
  false at its first entry. `klarch new-component` writes no plan document, since the `wanted`
  filter it hands `owed_documents` admits the new Component alone, and is left unchanged.
- **The shipped text**:
  - the planning skill: its terms, its layout, what a spec holds and in which file, the assembly
    (the transcript ending at the owner's last message before the branch that adds the plan
    document merges, so that the rulings given at its review are in it, the removal step,
    which removes every path under a home directory, every e-mail address, every credential and
    token, and the pasted material the owner marks, then the transcript shown to the owner and
    pushed only on the owner's word for that transcript; the owner's words attributed, citing the
    round; of the search's output, only `generated-lines.md`, copied after `--verify` as
    `search/discussion.txt`, never its verdict files, `shown.md`, its agent directories,
    `agents.txt` or `work.txt`; and "What is already decided" keeping each verdict whose outcome
    changed something, with that outcome, and a link to `search/discussion.txt` for the rest), the
    slice audit's `--work` and `--seeds-from`, each slice or spec audit committing only its
    `fit-lines.md`, as `search/design-audit.txt` under its own `search/`, never `gaps.md`, the P2
    clause, the life of a slice's spec (moved under `harvested/`,
    then the milestone's directory leaving whole), and the deletion that ends a plan document's
    life;
  - the primer's source: `primer@checker-model` on plan documents and their anchors,
    `primer@where-knowledge-goes` on the plans directory row and on where the owner's verbatim words
    stay, `primer@design-heads` on the same, and `primer@coined-terms`. This work names the case
    the closed issue on verbatim words recorded, for the harvest to rule: the owner asked for an
    issue entry holding the owner's words verbatim, as input to a later discussion, in the slice
    that put `cli` on top, W3 of the retrospective's workflow file of 2026-11-06 on that slice. The
    harvest rules whether an issue entry may quote the owner on the owner's request, or never;
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
    discussion committed after the removal step and on the owner's word, the search's
    `generated-lines.md` committed as `search/discussion.txt` from the session's scratch copy,
    `$CLAUDE_CODE_TMPDIR/split-design/search/` on the owner's machine, where it is still there, the
    commit saying so where it is not, and its "What is already decided" cut to the verdicts whose
    outcome changed something, by the rule the planning skill gains;
  - the mock projects that hold plan documents in today's layout, `planted`, `dirhome` and
    `unsound` under `path@testing@tests/projects/`, migrated to the new layout, with each finding
    `unsound` plants kept or replaced by its new-layout counterpart, and the literal plan paths of
    the unit tests, in `path@model@src/index.rs`, `path@model@src/records.rs`,
    `path@model@src/phases.rs`, `PLANS_TREE` of `path@check@src/check/mod.rs` and
    `path@check@src/commands/history.rs`;
  - the 32 heads of Group A and the 13 of Group B rewritten, below, with every text that cites them,
    the head of the frozen location written, and `design@knowledge-architect@no-branch-sha-is-cited`,
    so that a SHA a transcript quotes is no citation;
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
4. **A frozen file is out of the walk, and parsed for definitions only.** In a mock project, a
   dangling reference in the `README.md` of a slice under `harvested/`, or in
   `search/discussion.txt`, is not reported, untracked or committed; an item that harvested slice defines resolves from a sibling
   slice's `README.md`; and a live item that reuses one of its ids is reported.
5. **The P3 clause.** In a mock project under git, `commits` reports a commit that edits a file of a
   slice under `harvested/`; it reports none for a commit that adds one, deletes one, or moves a
   live slice's directory under `harvested/`.
6. **The transcript is never parsed.** A reference-shaped span, a slug definition and a branch SHA in
   `transcript.txt` produce no finding of `check` or `commits`.
7. **A harvested slice is cited from inside its milestone only.** A citation of a harvested slice, or
   of an item it defines, from a file outside the milestone's directory is reported; from a sibling
   slice's `README.md` it resolves.
8. **A slice audit's work is two documents.** In its code half, `klarch worklist --seeds-from`
   accepts a slice's directory or its `README.md` and seeds from that `README.md`'s section "What is
   already decided". In its text half, the planning skill, as installed, gives a slice audit's
   search `--work` the milestone's `README.md` and the slice's `README.md` and `--seeds-from` the
   slice's `README.md`, `--since` the commit that last changed the slice's `README.md`, and names
   no other file; AC1's observable, read at slice 2's audit, is the
   test of record.
9. **The P2 clause.** The installed planning skill states that every condition of a decided thread
   is in the decided design of a document the search reads, and that `deliberation.md` holds only
   the argument; this milestone's migrated documents meet it, read by the slice's review.
10. **The P4 removal step, and the owner's word.** The installed planning skill names a step before
    the transcript's commit that reads it and removes every path under a home directory, every
    e-mail address, every credential and token, and the pasted material the owner marks, and has
    the session show the owner the transcript and push it only on the owner's word for that
    transcript; this milestone's committed transcript holds no path under the machine's home
    directory and no e-mail address, by `grep` over it before the commit, and is pushed on the
    owner's word.
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
one live slice and one under `harvested/`, and a `search/discussion.txt` and a `transcript.txt` in each, written in
the layout this slice builds. The existing mocks that hold plan documents in today's layout,
`planted`, `dirhome` and `unsound`, are migrated, as "This repository" says; the invariants
`path@testing@CLAUDE.md` states of them, which mock reports no finding and which reports a finding
of which phase, hold after the migration.

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

What each of Group A is rewritten to state, as the session reads each head's conflict with the
decided design; the slice's design audit checks each reading against the head read whole:

- `design@model@plan-register`: plan documents are model types of their own in `model`; the anchor
  `plans` carries `spec` and `milestone` as plan types, not as a File and a Directory register.
- `design@model@plans-split-dirs`: specs/ and milestones/ each hold one directory per plan document;
  a spec file under specs/ is a finding.
- `design@model@plan-document-kinds`: `spec@plans@<id>` names a spec's directory, and
  `spec@<milestone>@<slice>` a slice's directory, live or under `harvested/`.
- `design@model@plan-items-by-section`: the items are the level-three headings of the item sections
  of a plan's `README.md` and `deliberation.md`; Threads and Arguments sit in `deliberation.md`.
- `design@model@plan-item-scope`: an item is cited from inside its plan's directory only, a
  harvested slice's items included.
- `design@model@items-as-section-registers`: an item register's home is the `README.md` and the
  `deliberation.md` of a plan's directory, a frozen file counting for definitions only.
- `design@model@spec-file-is-an-anchor`: a spec's directory is its anchor; the incumbent, a spec
  file as its anchor, becomes the rival that lost.
- `design@model@milestone-entries-are-directories`: a milestone is a plan type of its own carrying
  one directory per slice and the four item registers, no longer an entry of the Directory shape.
- `design@model@entry-anchor-carries` and `design@model@entry-anchor-path-kind`: each holds for a
  declared Directory register alone, the plan types no longer using that shape.
- `design@model@anchors-are-components-and-locations`: the constructed anchors include each spec's
  directory, and each milestone's as a plan type rather than as an entry; a slice's directory is
  none.
- `design@model@an-entry-is-a-heading-at-the-register-level`: a plan's items have their home in its
  `README.md` and `deliberation.md`, where the head says "a milestone's README included".
- `design@check@plan-document-sections-by-kind`: a spec's or a milestone's `README.md` owes today's
  list but Threads and Arguments, a slice's `README.md` its five sections, and `deliberation.md`
  Threads and Arguments.
- `design@check@a-file-register-is-a-directory-of-entries`: the plan types no longer take the File
  shape; the rule holds for `issue` and each declared file register.
- `design@check@every-path-names-its-anchor`, `design@check@unanchored-path-first-segment-here` and
  `design@check@deepest-anchor-wins`: a spec is a directory anchor, from which a path is read like
  any other, where the second says today "A spec is an anchor and a file"; a file of a plan's
  directory has that plan's anchor as its deepest; a `README.md` reaches the files beside it by
  relative links, not by `path` references.
- `design@query@anchors-listing-command`: `anchors` lists each spec's directory and each
  milestone's with its kind; a slice's directory is no anchor.
- `design@query@record-is-every-register-entry`: the record excludes the plan types and the frozen
  location, `harvested/`.
- `design@query@seeds-from-the-plan-s-decided-section`: `--seeds-from` takes a plan's directory or
  its `README.md`, and a slice's `README.md` holds its own section.
- `design@agent-skills@milestone-is-a-directory`: each slice is a directory, live or under
  `harvested/`, beside the milestone's `deliberation.md`, `search/` and `transcript.txt`.
- `design@agent-skills@milestone-split-by-lifetime`: the split by lifetime gains the deliberation
  apart from the documents the search reads, and a slice kept under `harvested/` until its
  milestone leaves.
- `design@agent-skills@plan-document-leaves-at-landing`: at the slice level, a slice is moved under
  `harvested/` at its landing; a spec and a milestone still leave at theirs.
- `design@agent-skills@ledger-from-transcript`: the assembly commits the transcript after the
  removal step and the owner's word, and the search's generated lines as `search/discussion.txt`.
- `design@agent-skills@plan-records-the-exchange`: `deliberation.md` records the whole exchange,
  attributing the owner's words; the verbatim words are in `transcript.txt`.
- `design@agent-skills@attribution-replaces-quotation`: plan items attribute too; the owner's
  verbatim words stay in a committed transcript, and the case of an issue entry the owner asks for
  is ruled by this slice's harvest.
- `design@agent-skills@no-progress-record`: a harvested slice is no progress record: it is a slice's
  plan, frozen, which leaves with its milestone; history lists every plan document and milestone
  that left.
- `design@agent-skills@retiring-plan-opens-issue`: a slice no longer leaves at its landing, so its
  siblings re-point nothing then; the rule holds for a spec and a milestone that leave.
- `design@agent-skills@standing-argument-in-head`: the deliberation stays in `deliberation.md`
  while its plan document exists, a slice's until its milestone leaves, then in history.
- `design@agent-skills@tripwire-horizon`: "plan documents deleted at their landing" counts a spec
  and a milestone leaving, never a slice moved under `harvested/`.
- `design@agent-skills@search-reads-the-draft`: the search reads the draft of the documents the
  assembly writes, whose deliberation the assembly moves into `deliberation.md`.
- `design@agent-skills@record-searched-before-the-work`: a slice audit's search reads the
  milestone's and the slice's `README.md`, never `deliberation.md`.

`design@agent-skills@tripwire-horizon` names "plan documents deleted at their landing" among the
occasions a tripwire's horizon counts: a slice's spec is no longer deleted at its landing. The
harvest reads every horizon written on that occasion: on the commit before the milestone
document's, `git grep -n landing -- '**/docs/tripwires.md' 'docs/tripwires.md'` finds
`tripwire@agent-skills@silent-read-too-widely`, "the fifth plan document deleted at its landing",
and `tripwire@check@fix-makes-a-choice`, which watches until "the commit on the main branch that
deletes `spec@plans@path-quickfixes` at its landing"; the search is a lead, since a phrase split over
two lines escapes it. The migration's moves read as deletions to the commands that count landings,
`git log --diff-filter=D --name-only -- docs/plans/` among them, wherever git's rename detection
does not pair the old path with the new one: a move whose content changes past the rename
threshold, as a move that takes the Threads and Arguments out into `deliberation.md` can; a pure
move shows as a rename (`R100`) and is not listed. The move of `spec@plans@path-quickfixes` into
its directory, and the move of each slice spec into its own, are no landings, and the harvest says
so where those horizons count.

**Group B: the heads that state the regime of every reference, which gain one member, the frozen
location of a milestone's directory, `harvested/`, out of the walk by construction.** Each is rewritten to admit that
member by citing the one head of the regime this slice's harvest writes, per D1 of the milestone
document, rather than by restating it.

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
that a SHA a transcript quotes is a quotation, not a citation. Its citing texts: the root
`CLAUDE.md`, `## Git` point 5; `path@agent-skills@docs/design/documents-point.md`; and
`issue@transcript@an-option-anonymizes-what-transcript-prints`.

**Bound, their decision unchanged**: `design@agent-skills@thread-slug-is-entry-id` and
`design@agent-skills@arguments-as-items`, Group C, since `deliberation.md` keeps the items; the slice
rewrites only the sentence of each that names the file holding the items, where it names one, and
the scope clause of the first, "cited from inside that document only", to the plan's directory, a
milestone's slices included, per the rewritten `design@model@plan-item-scope`.

**Found at the slice's design audit, rewritten by its harvest**, beside the groups above, each with
the texts `cargo klarch show` lists at the harvest:

- `design@agent-skills@design-home-is-built-intent`: a harvested slice is a plan document holding
  built design, frozen and out of the walk, where the head says a plan document holds decided design
  not built yet; its citation of `design@model@plan-register` is re-pointed with it.
- `design@agent-skills@a-seed-and-its-citers-are-never-unrelated`,
  `design@agent-skills@session-acts-on-the-verdicts` and
  `design@agent-skills@design-audit-checks-fit`: "What is already decided" keeps only the verdicts
  whose outcome changed something, with a link to `search/discussion.txt` for the rest, and a
  design audit's `fit-lines.md` is committed as `search/design-audit.txt` beside the commit message
  that carries it.
- `design@check@every-commit-judged-by-the-tip-checker`: the P3 clause is a further source of a
  commit's failure, which the head's list of sources and its per-commit line gain.
- `design@check@groups-are-subdirectories`: specs/ keeps its groups, a subdirectory holding a
  `README.md` being a spec.
- `design@model@an-entity-belongs-to-its-anchor`: unchanged in its rule, since a slice's directory
  is no anchor; read by the harvest for any sentence naming a slice file.
- `design@model@a-placed-name-reads-as-nothing-else`: a spec's directory, not a spec file, is the
  anchor whose name is placed; a slice's name is placed by no anchor.
- `design@model@an-unresolved-reference-is-reported`: a citation of a harvested slice or of an item
  it defines from outside its milestone's directory is a further way, its repair naming the
  milestone.
- `design@model@a-file-register-index-is-rows`: the plan types leave the File and Directory shapes;
  the indexes of specs/ and of a milestone take the form the plan model defines, a milestone's
  index listing its harvested slices from the definitions read of their frozen `README.md`. Their
citing texts: of the first, `path@agent-skills@content/skills/design/SKILL.md` and
`register@query@design`; of the second, none outside this milestone.

**Bound and not rewritten**: `design@agent-skills@branch-commits-judged-at-tip`,
`design@agent-skills@agents-write-in-scratch`, `design@agent-skills@verdicts-reach-the-conformance-review`,
`design@agent-skills@transcript-extraction-is-the-checker-s`, `design@agent-skills@transcript-found-by-listing`,
`design@create@init-command` and `design@create@component-command`, as the milestone document says of
each. `design@model@declarable-directory-register`, which the plan model leaves as it is.

**To read again**: `rejected@model@subdirectory-anchors-location` rests its argument in part on the
milestone needing the Directory shape; once the milestone has a type of its own, the harvest judges
whether that argument still stands, per `skill@knowledge-architect-decision-recording@losing-alternatives`.

## Audit subjects

- `path@model@src/manifest.rs`: `Registers::built_in`, the plan constants, `PLAN_SECTIONS`,
  `SLICE_SECTIONS`, `SPECS_HOME`, `MILESTONES_HOME` and the helper `plan`; `path@model@src/entity.rs`,
  `path@model@src/tree.rs` (`directory_layout` and its repair strings),
  `path@model@src/resolver.rs`, `path@model@src/walk.rs` (`LIVE_SUFFIXES`, and how the installed
  files leave the walk) and `path@model@src/shipped_set.rs`.
- `path@check@src/check/registers.rs`, `path@check@src/check/references.rs` and
  `path@check@src/commands/history.rs` (`branch_sha_findings`, and where the P3 clause's finding
  goes).
- The mock projects `planted`, `dirhome` and `unsound` under `path@testing@tests/projects/`, and
  the unit tests that write plan paths, named under "This repository".
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

## New names, in one place

The names this slice proposes for the model; the discussion fixed none of them, and the slice's
design audit may change any.

```
PlanKind { Spec, Milestone, Slice }        crates/model/src/manifest.rs: the kind of a plan document's directory
Shape::Plan(PlanKind)                      crates/model/src/manifest.rs: the shape of `spec` and `milestone` in Registers::built_in
Constructed::Plan(PlanKind)                crates/model/src/entity.rs: the anchor of a spec's or a milestone's directory, replacing Constructed::Spec and the milestone's use of Constructed::Entry; a slice's directory is no anchor
DELIBERATION_FILE = "deliberation.md"      crates/model/src/manifest.rs
SEARCH_DIR = "search"                      crates/model/src/manifest.rs: holding discussion.txt and design-audit.txt
TRANSCRIPT_FILE = "transcript.txt"         crates/model/src/manifest.rs
HARVESTED_DIR = "harvested"                crates/model/src/manifest.rs
DELIBERATION_SECTIONS                      crates/model/src/manifest.rs: Threads and Arguments, taken out of PLAN_SECTIONS
frozen_file_findings                       crates/check/src/commands/history.rs: the P3 clause's findings of `commits`
```

Existing names the slice reuses: `Registers::built_in`, `Registers::is_plan_register`,
`PLAN_SECTIONS`, `SLICE_SECTIONS`, `PLANS_ANCHOR`, `LIVE_SUFFIXES`, `owed_documents`, `home_text`
and `branch_sha_findings`.

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
  the transcript of the discussion committed, after the removal step and on the owner's word, as
  its `transcript.txt`; the search's `generated-lines.md` committed as `search/discussion.txt`
  where the session's scratch copy is still on the machine; its "What is already decided" cut to
  the verdicts whose outcome changed something.
- The mock projects and unit tests that hold plan documents in today's layout move to the new one,
  in the same commit, since they fail the new checker until they move.
- A migrated `README.md` names a file beside it, `deliberation.md`, `transcript.txt` or a file of
  `search/`, by a relative navigation link, never as a backticked path: the lint of
  `design@check@unanchored-path-first-segment-here` reads a first segment from the document's own
  directory, and a `path` reference into a plan's directory is refused. A slice's `README.md`
  reaches its milestone's `deliberation.md` through the item references alone.

## Mapping tables

**Each plan document of the tree, and where it goes.** Over every plan document the tree holds when
the slice starts; a plan document added later is mapped by the same rule.

| today | after this slice |
| --- | --- |
| `docs/plans/specs/<id>.md` | `docs/plans/specs/<id>/README.md`, its Threads and Arguments sections in `deliberation.md` beside it |
| `docs/plans/milestones/<id>/README.md` | the same path, its Threads and Arguments sections in `docs/plans/milestones/<id>/deliberation.md` |
| `docs/plans/milestones/<id>/<slice>.md` | `docs/plans/milestones/<id>/<slice>/README.md`, its Threads and Arguments sections in `deliberation.md` beside it |
| `docs/plans/milestones/<id>/index.md`, generated | the same path, generated, listing the milestone's slices, live and harvested, in the form the slice's model defines, written by `cargo klarch index` |
| `path@plans@specs/index.md`, generated | the same path, each row naming a spec's directory rather than a file |
| the discussion's transcript, not in the tree | `transcript.txt` beside the milestone document, this milestone only |
| the search's output, in the session's scratch space | `search/discussion.txt` beside the milestone document, its `generated-lines.md` only, this milestone only, where it is still on the machine |
| a mock project's plan document in today's layout | its place by the rows above |

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
2. **The migration of the tree.** `spec@plans@path-quickfixes`, this milestone, and the mock
   projects and unit tests in today's layout, by the mapping table: Claims 11 and 12, and the P4 grep
   of Claim 10 and the owner's word before the transcript's push.
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
- **The head of the frozen location**, one head that the 13 heads of Group B cite, and a head of
  its own for the P3 clause unless the one-decision test of
  `skill@knowledge-architect-decision-recording@head-shape` joins it to the first.
- **Losing alternatives**: those above and those of the milestone document's threads this slice
  builds, judged by the tests of `skill@knowledge-architect-decision-recording@losing-alternatives`.
  A lead from the review of the plan: a spec as one file, the incumbent of
  `design@model@spec-file-is-an-anchor`, is a layout adopting projects write
  (`argument@split-guidelines-and-plan-directories@arg-45`), so it may meet test 1 once reversed;
  and the owner's words verbatim in plan items, the incumbent of
  `design@agent-skills@plan-records-the-exchange`, rests on the slip counts of the closed issue,
  evidence that cost work. Each is judged by the tests as written.
- **The premortem's causes**: P5 judged as a cause, by the standard of
  `skill@knowledge-architect-decision-recording@ruled-tripwires`. The harvest's commit message
  states the outcomes of the three causes that are claims of this slice: P2 converted into the
  clause of the milestone document's "The plan document's directory", P3 into the clause of "The
  regime of each file", each a claim of this slice; P4 kept out of the tripwires, since a script
  could compute its firing, which condition 1 of the standard sends to a check, and routed to
  `issue@transcript@an-option-anonymizes-what-transcript-prints`.
- **The count of attribution slips** that the closing condition of the issue on verbatim words
  named, against the rewritten `design@agent-skills@attribution-replaces-quotation`: a `todo` issue
  of `agent-skills` opened by the harvest, as the milestone document's "Later consequences" says,
  with the baseline its "What is already decided" carries.
- **This slice's spec** is moved under the milestone's `harvested/` after the review before the
  merge, under the rule it builds, rather than deleted.
