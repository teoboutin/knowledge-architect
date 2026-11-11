# The register `goal` is the root's alone, and is cited with no anchor

The second and last slice of `milestone@plans@goals-at-the-root`. It leaves when it lands. The threads,
arguments and decided design it shares with the milestone are in the milestone document; those only
this slice uses are below. Its design audit amended this spec in place, and its commit lists each
amendment.

## Builds

- **`model`**:
  - the register `goal` leaves the registers every Component carries, and is carried by the
    project's root alone, by a rule compiled into the checker that no declaration changes;
  - a location that declares the register `goal` is refused in phase 1, by the manifest's parse, a
    finding with the register dropped from that location's list, per
    `design@model@a-wrong-declaration-is-a-finding`;
  - a file docs/goals.md, or a directory docs/goals/, of a Component other than the root is refused
    in phase 2, by `tree`, computed over the walked set, per
    `design@model@an-entry-is-a-heading-at-the-register-level`, so that no phase-3 finding on a
    slugged heading in it hides the repair;
  - a goal is cited `goal@<id>` and the root's goals home `register@goal`, resolved by the resolver
    from the kind and the register, never from the segment count alone; the forms
    `goal@<anchor>@<id>`, `register@<anchor>@goal` and register@*@goal are refused, each finding
    naming the form with no anchor as its one repair, with no switch that keeps accepting them.
    A declared register keeps `<kind>@<anchor>@<id>`.
- **`check`**: the repair the refusal of a Component's goals home names has two cases, per
  `design@check@finding-names-the-repair`: a home that holds no goal is deleted; a home that holds
  goals has each goal moved to the root's goals home under the goal-setting skill, or turned into a
  design head. The rule on what a goals home cites, with one home left, through
  `admitted_in_goals_home` and `GOALS_HOME_REPAIR` of `path@check@src/check/references.rs`, which
  the lints on ids share: each still lists every reference a goals home admits, and `--fix` writes
  nothing there. The lints on a reference written without backticks, on a bare id and on a plain
  word recognise the forms with no anchor and name them as their repairs.
- **`create`**: `klarch init` and `klarch new-component` write no goals home for a Component.
  `create` keeps no list of homes of its own: `owed_documents` in `path@create@src/command.rs`
  writes what `model` lists for each anchor, so the change comes through `model`, and `home_text`
  keeps its one `goal` arm, which the root still needs.
- **`query`**:
  - the goal context of `klarch worklist` is the root's goals alone, in `title_units` of
    `path@query@src/graph.rs`;
  - `klarch id` prints a goal as `goal@<id>`;
  - `worklist` writes goal title lines in that form and resolves a `--seed` written in it;
  - `show` answers the forms with no anchor and the refused forms as the check does: its `*` arm
    and `cited_at` in `path@query@src/show.rs` serve a Component's home only.
- **The shipped text**:
  - the 5 lines of the source that write register@*@goal write `register@goal`;
  - the primer's statement of the reference grammar, "A reference is one backticked span,
    `<kind>@<anchor>@<id>`", states the forms with no anchor too, per
    `design@agent-skills@workflow-axis-lenses`;
  - every sentence that says what the checker requires of a Component, `primer@checker-model`
    among them;
  - the citation rule of rule 8 of the goal standard, `path@agent-skills@fragments/goal-standard.md`,
    with its twin in `primer@where-knowledge-goes`, which still speak of a location's goals home and
    of a goal's own Component;
  - the shipped lines that speak of goals homes in the plural:
    `path@agent-skills@content/agents/workflow-auditor.md`,
    `path@agent-skills@fragments/admission-record-check.md` and the line before the
    register@*@goal line of `path@agent-skills@content/skills/retrospective-intake/SKILL.md`;
  - the goal reviewer's row on a goal in a goals home other than the root's, in
    `path@agent-skills@content/agents/goal-reviewer.md`, judged once `anchors` lists no such home;
  - the 11 `%%` lines of content/ that cite a goal.
- **This repository**:
  - each Component's empty docs/goals.md is deleted;
  - every goal reference is written `goal@<id>`, and every reference to the root's goals home
    `register@goal`, the goals' own bodies included: a respelling of reference form that changes no
    word of a goal's statement, which the first commit's message says, per
    `design@agent-skills@goals-change-through-goal-setting`;
  - CHANGELOG.md leaves the walk, by a row of `[walk] skip-files` in the manifest, and its released
    0.7.0 line stays as released, on the owner's word at the review before the merge,
    `thread@goals-at-the-root@released-changelog-line-repaired`;
  - the developer documents the slice makes false: `path@check@README.md`, on what a goals home may
    cite and on what the word component means; `path@cli@README.md`, on the work list's goal title
    lines; `path@cli@CRATES-IO.md`, which lists goals as a Component's.
- **The record**:
  - the entries the milestone document's table gives slice 2, and the entries the design audit found
    the slice makes false, listed under What is already decided;
  - `design@check@every-path-names-its-anchor` and `design@model@reserved-anchors` split, per
    `issue@check@core-heads-bundle-decisions-with-many-citing-sites`, every citer of either
    re-pointed to the part it relies on, among them `spec@plans@path-quickfixes`, four issues of
    `check`, two of `model`, the `rejected` entries each head cites, and the two `rejected` entries
    of `check` that name the head they lost to, rewritten to name no winner per
    `design@agent-skills@rejected-citation-direction`;
  - the issues the slice changes: `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`,
    whose paragraph on goals homes is rewritten;
    `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`, whose case of a goals
    home declared at its carrier's path leaves;
    `issue@check@texts-citing-the-relocation-goal-state-its-old-measure`, one of whose texts loses
    its object; `issue@agent-skills@a-slice-s-spec-with-no-section-of-decided-entries-gives-its-audit-no-seed`,
    which gains this audit's instance;
  - the changelog's entries: slice 1's entries in the `Next release` section are edited rather than
    new ones written, per `design@knowledge-architect@changelog-entries`; a Component's goals home
    and a location's declaration of `goal` refused are major, a change to the homes a project must
    carry and a manifest that stops being accepted; the refused anchored goal forms are minor, a
    stricter check, per `design@knowledge-architect@versioning-policy`; the Migration entry names
    the whole-set review and the rewrite of a consumer's goal references, its mock projects
    included, with the repair each new finding names.

## Claims

- **A project with a goals home in a Component other than the root is refused, with a finding that
  names the file and the repair of its case.** Test: the tests of `path@testing@tests/binary.rs`
  over a copy of `core` given a Component, its goals home absent, empty, holding a goal, or holding
  a heading with no slug, in either shape; and the planted defect of `unsound`. Shown to fail:
  `cargo x mutate` on the condition of the new refusal in `tree`, and on its test of what the home
  holds, the test then passing a tree it must refuse or naming the wrong repair.
- **A location that declares the register `goal` is refused, the register dropped from its row, and
  the run stops at phase 1.** Test: a copy of `core` whose manifest gains such a location. Shown to
  fail: `cargo x mutate` on that condition in the manifest's parse.
- **A project whose only goals home is the root's passes, the home being one file or a directory
  of files behind a README.** Test: the conformant mocks, `core` with a file and `dirhome` with a
  directory. Shown to fail: `cargo x mutate` on the test that keeps the root out of the refusal in
  `tree`.
- **`klarch init` and `klarch new-component` write no goals home for a Component, and the tree
  they write passes the check.** Test: the tests of `create` over a scaffolded project, and
  `a_component_added_either_way_passes_the_check` of `path@cli@tests/init.rs`. Shown to fail:
  `cargo x mutate` on the root-scoped arm of `carried_by_component` in
  `path@model@src/manifest.rs`, which `create` reads.
- **A goal is cited `goal@<id>` and the root's goals home `register@goal`, whatever theme file holds
  the goal.** Test: a copy of `core` given a Component that cites a goal and the goals home, and a
  copy of `dirhome` citing a goal of each of two theme files. Shown to fail: `cargo x mutate` on the
  arms of `candidate` in `path@model@src/entity.rs` that read the form with no anchor.
- **Each anchored goal form is refused, with the form with no anchor as its one repair.** Test: a
  copy of `core` writing `goal@<anchor>@<id>`, `register@<anchor>@goal` and register@*@goal.
  Shown to fail: `cargo x mutate` on the arm of `candidate` that refuses the anchored form.
- **A goals home cites a goal or a register of the root, and every other reference there is
  refused.** Test: the slice's own test of each refused kind, the suite holding survivors there
  today, per `issue@model@the-resolver-s-arms-have-mutation-survivors`. Shown to fail:
  `cargo x mutate` on each arm.
- **The work list gives each agent the root's goals and no other, in the form with no anchor.**
  Test: the tests of `query` over a mock project. Shown to fail: `cargo x mutate` on the gathering
  of the goal context.
- **`show` answers each form with no anchor and each refused form as the check does.** Test: the
  binary's tests of `show` and `id` in `path@testing@tests/binary.rs`. `show` reads a reference
  through the resolver, so its arms needed no change. Shown to fail: `cargo x mutate` on `spelled`
  in `path@model@src/entity.rs`, the one printer of a reference, which `id` prints with.
- **Every form the shipped text writes for the goals home resolves in a project other than this
  one.** No check reads a reference in an installed copy, per
  `design@check@installed-files-outside-the-walk`, so the test is a shipped-set test of
  `path@model@src/shipped_set.rs`, per `design@knowledge-architect@shipped-set-tests-in-the-model`:
  every register and goal reference of the shipped set is well formed against a mock project whose
  name is not this one's, and every register reference with no placeholder resolves there; a goal
  reference is checked for its shape alone, since the shipped text writes a goal with a placeholder
  id. Shown to fail: the form of today kept at one site.

Each mutation is run with `cargo x mutate`, as `skill@knowledge-architect-development` asks.

## Fixtures

Two of the five mock projects of `path@testing@tests/` declare a Component and hold a goals home in
it, `planted` and `unsound`. `planted`'s Component loses its goals home. `unsound`'s Component keeps
its goals home, which holds a goal, as a planted phase-2 defect, since a phase-2 defect is
`unsound`'s, per `path@testing@CLAUDE.md`. No new mock project is written: the refusal's other
cases are built by the tests of `path@testing@tests/binary.rs` in a copy of `core`. Every goal reference of the mock projects and of
the tests is written in the form with no anchor. `path@testing@tests/` is outside this repository's
walk, per `design@testing@test-files-outside-the-walk`, so an anchored form left there is found by a
search and by each mock's own test, never by `cargo klarch check`. Tests that write a goals home
inline are edited too: the audit lists them by a search for docs/goals in the sources of `check`,
`model`, `query`, `testing` and `cli`. All are in the vocabulary that exists, but the form with no
anchor, which this slice adds.

## What is already decided

The entries this slice rewrites, those the milestone document's table gives it and those its design
audit found it makes false, so that a later audit of this slice's work seeds from them:

- the milestone's rows: `design@agent-skills@shipped-text-cites-no-entry`,
  `design@check@every-path-names-its-anchor`, `design@check@goals-home-cites-only-goals-and-root-homes`,
  `design@check@nothing-of-a-project-is-compiled-in`, `design@create@component-command`,
  `design@model@components-carry-the-same-documents`, `design@model@registers-are-declared`,
  `design@query@goal-context-once`;
- found at the audit, on the register: `design@agent-skills@goal-standard-rules`,
  `design@model@anchors-are-components-and-locations`,
  `design@model@a-component-carries-what-a-project-attaches`;
- found at the audit, on the form with no anchor: `design@model@reserved-anchors`,
  `design@model@register-home-reference`, `design@model@an-entity-belongs-to-its-anchor`,
  `design@model@path-to-a-home-is-reported`, `design@model@at-span-candidate-rule`,
  `design@agent-skills@a-reference-claims-a-revisit`, and
  `rejected@model@register-reference-without-anchor`, which comes back on a new argument;
- judged at the audit and left as they stand, since none states a reference form that the form with
  no anchor makes false: `design@model@harness-kinds-cited-without-anchor`,
  `design@model@one-entity-table` and `design@model@an-unresolved-reference-is-reported`;
- split out of `design@check@every-path-names-its-anchor` and `design@model@reserved-anchors`, per
  `issue@check@core-heads-bundle-decisions-with-many-citing-sites`:
  `design@check@unanchored-path-first-segment-here`, `design@check@deepest-anchor-wins`,
  `design@model@escape-anchor-elsewhere`, `design@model@foreign-file-names-its-project` and
  `design@model@generic-anchor-names-every-component-copy`;
- written at the review before the merge, each out of a head above that held two decisions:
  `design@model@goal-register-carried-by-the-root-alone`, out of
  `design@model@registers-are-declared`, and
  `design@model@anchored-root-register-reference-is-malformed`, out of
  `design@model@root-register-reference-has-no-anchor`, the head this slice writes.

## Audit subjects

- `path@model@src/manifest.rs`: `GOAL_REGISTER` and the list of the registers every Component
  carries.
- `path@create@src/command.rs`: the list of homes written, and the list
  `design@create@init-command` derives.
- `path@query@src/graph.rs`, which gathers the goal context, with its test whose claim is the
  root's goals, then each enclosing Component's, then the unit's own; and
  `design@query@goal-context-once`.
- `design@model@registers-are-declared`, whose reason is that the storage of the component
  registers "is what the word component means": the audit says how the head reads once one of
  them is the root's alone.
- The form the shipped text writes for the root's goals home. The audit put it to the owner, who
  chose the form with no anchor, `thread@goals-at-the-root@root-register-reference-has-no-anchor`.
- `issue@check@some-findings-in-a-goals-home-name-a-form-refused-there`, which names code the
  slice edits.
- `spec@plans@path-quickfixes`, which cites `design@check@every-path-names-its-anchor`: the split
  of that head changes its citations there.
- Each test that counts goals homes.

## Fails alone on

- A tree that holds a goals home outside the root and passes, or the root's goals home as a
  directory refused.
- A goal cited `goal@<id>` that the check refuses, or one cited with an anchor that it accepts.

## Premises that expire

None: this is the milestone's last slice. The plan registers keep their anchor, on the owner's
word, `thread@goals-at-the-root@plan-register-references-have-no-anchor`, so what this slice writes
of the form with no anchor, in the code, the primer and the heads, states it for `goal` alone.

## Threads

### A register whose one carrier the tool fixes, the root's `goal`, is cited with no anchor `##root-register-reference-has-no-anchor`

Proposed by the owner at the design audit of this slice, round 1, answering Q1, the form the
shipped text writes for the root's goals home. Final state: `approved`, on the owner's word at the
fork, round 2, "1. both", after the record search of this thread had run and returned no entry that
defeats it. Arguments: `argument@goals-at-the-root@arg-123`, `argument@goals-at-the-root@arg-124`,
`argument@goals-at-the-root@arg-125`. Its shape is under Decided design; it is harvested into the
rewrites of `design@model@reserved-anchors`, `design@model@register-home-reference` and
`design@model@harness-kinds-cited-without-anchor`, and into
`rejected@model@register-reference-without-anchor`, narrowed to the registers a project declares or carries in every Component.

The owner's words, round 1:

> `register@goal` would probably be a much more natural way to reference a register that can only exists at root. The same should probably apply to plans and other root only registers

Round 2, to the question whether entries drop the anchor too, or only the home:

> 1. both

The fork's choice, in the weak form: the owner chose the form with no anchor over `*` read as every
anchor that carries the register, a new reserved anchor word for the root, and the root's goals home
named in words with no reference, which no goal ranks.

### The released 0.7.0 line that writes register@*@goal stays as released, and the changelog leaves the walk `##released-changelog-line-repaired`

Proposed by the session in round 1 of this slice's audit, as its default: the line rewritten in
place. Approved on the owner's word, round 2, "3. a", on `argument@goals-at-the-root@arg-124`. The
review before the merge found that the rewrite changed a released section's content, against
`design@knowledge-architect@changelog-section-fixed-at-release`, whose premise the default had
missed. The owner then proposed `argument@goals-at-the-root@arg-128` and approved the session's
default, `argument@goals-at-the-root@arg-129`. Final state: `approved`, on that word. The line is
restored as released, CHANGELOG.md leaves the walk, and that head is rewritten to say no check
reads the changelog. The rule that the changelog holds no reference to real content is left to
`issue@knowledge-architect@the-changelog-holds-references-no-check-reads`. The rival, register@*@goal
kept resolving as a second spelling, loses to one shape per meaning,
`rejected@model@anchored-goal-form-as-a-second-spelling`.

## Decided design

### The root's `goal` is cited with no anchor

The shape: `goal@<id>` for an entry, `register@goal` for the root's goals home; each anchored form
refused with the form with no anchor as its repair; a declared register keeps its anchor.

The argument: `rejected@model@register-reference-without-anchor` lost on two reasons. The first, that
the tool cannot know which register is single-instance, fails for `goal` once a rule compiled into the
checker fixes the root as its one carrier, which no declaration changes. The second, one three-part
grammar with no special case, is the cost `design@model@harness-kinds-cited-without-anchor` already
accepts for the harness kinds, whose carriers the tool fixes; here the owner accepts it for `goal`. A
declared register keeps the anchor because a manifest edit changes its carriers, not because the tool
cannot read them.

The nearest rival, `*` read as every anchor that carries the register, keeps today's 5 shipped lines
and the released 0.7.0 line resolving. The fact that ranks it no higher: none; it is a fork, and the
owner chose. A new reserved word for the root loses besides on a measured cost: `is_anchor_word` in
`path@model@src/entity.rs` covers every reserved word, so every backticked `root@<host>` span of
every project would become a finding.

## Losing alternatives

- `*` read as every anchor that carries the register, lost to
  `thread@goals-at-the-root@root-register-reference-has-no-anchor` at the owner's fork.
- A new reserved anchor word for the root, lost at the same fork, with the cost to other tools' spans
  measured in the code. The idea stays open for a root path, which
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check` holds; this loss
  covers register references only.
- The root's goals home named in words with no reference, lost at the same fork; it would leave the
  5 shipped sites with no checked pointer.
- register@*@goal kept resolving beside `register@goal`, lost to
  `thread@goals-at-the-root@released-changelog-line-repaired`, and recorded as
  `rejected@model@anchored-goal-form-as-a-second-spelling`.

## Implementation sequence

1. `model`, `check` and `create`: the register is the root's, both refusals, the form with no
   anchor and the refusal of the anchored goal forms, no goals home written for a Component; this
   repository's empty goals homes deleted, every goal reference respelled and the mock projects
   moved. These go in the branch's first commit, or are folded into it before the review, per
   `primer@branch-commits`, since each sets what the tip's checker requires of every tree. The
   design audit's amendments of this spec and of the milestone document land in that commit too.
   Fails alone on the gates.
2. `query`: the goal context, `id`, `worklist` and `show`. Fails alone on its tests.
3. The shipped text, the shipped-set test of its forms, and the install. Fails alone on that test.
4. The record, the two split heads and the changelog. Fails alone on `cargo klarch check`.
5. The harvest, in commits of their own. The spec and the milestone document leave in a commit of
   their own after the review before the merge, as
   `skill@knowledge-architect-planning@plan-document-leaves` says, every entry that cites either
   re-pointed in that commit.

## Harvest

| what | where | when |
| --- | --- | --- |
| the register `goal` is the root's alone | `design@model@goal-register-carried-by-the-root-alone`, new; `design@model@registers-are-declared`, `design@model@components-carry-the-same-documents`, `design@model@anchors-are-components-and-locations` and `design@model@a-component-carries-what-a-project-attaches`, rewritten in place | at this slice's landing |
| the refusal of a goals home outside the root, and what a goals home cites | `design@check@goals-home-cites-only-goals-and-root-homes`, rewritten and renamed from `goals-home-cites-upward-only`; `design@check@nothing-of-a-project-is-compiled-in`; `design@agent-skills@goal-standard-rules` | at this slice's landing |
| what `init` and `new-component` write | `design@create@component-command` | at this slice's landing |
| the goal context of the work list | `design@query@goal-context-once` | at this slice's landing |
| a register whose one carrier the tool fixes is cited with no anchor, #root-register-reference-has-no-anchor | `design@model@root-register-reference-has-no-anchor` and `design@model@anchored-root-register-reference-is-malformed`, new; `design@model@reserved-anchors`, split into `design@model@escape-anchor-elsewhere`, `design@model@foreign-file-names-its-project` and `design@model@generic-anchor-names-every-component-copy`; `design@model@register-home-reference`, `design@model@an-entity-belongs-to-its-anchor`, `design@model@path-to-a-home-is-reported`, `design@model@at-span-candidate-rule` and `design@agent-skills@a-reference-claims-a-revisit`, rewritten; `rejected@model@register-reference-without-anchor`, narrowed to the registers a project declares or carries in every Component | at this slice's landing |
| the losing alternatives above, and `thread@goals-at-the-root@plan-register-references-have-no-anchor` | judged by `skill@knowledge-architect-decision-recording@losing-alternatives` | at this slice's landing |
| the form the shipped text writes for the goals home | `design@agent-skills@shipped-text-cites-no-entry` | at this slice's landing |
| the premortem's cause on a consumer's pin | dropped, on the doubt clause of the tripwire standard: the owner reports running every consumer whose pin crosses this version, round 1 of this slice's audit | at this slice's landing |
