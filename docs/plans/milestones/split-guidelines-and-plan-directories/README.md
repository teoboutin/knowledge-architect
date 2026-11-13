# Split guidelines, splits planned before the work, and plan documents as directories

## Status and audience

This milestone document is the plan of four pieces of work that one design discussion joined:

- split guidelines a project weighs, for its Components, for the modules of a Component's code and
  for the subjects of a design home, with the decisions they lead to recorded where the record
  needs them;
- splits of code and of the design record found and planned before the work that needs them, never
  left to an implementer;
- the design heads a harvest will touch, judged against the rules on heads before the harvest;
- plan documents made directories, with model types of their own, so that the record of the
  discussion sits beside the documents a slice's record search reads, never inside them.

It holds what crosses the slices; each slice's spec holds what only that slice builds. It leaves
when its last slice has landed and every default it lists is ruled. Slice 1 migrates this document
and its slice specs into the layout it builds, so the layout below is the one
`skill@knowledge-architect-planning@plans-layout` states today.

- Written for a session that did not witness the discussion.
- Where this document or a slice's spec and a design home disagree on a point that "What is already
  decided" does not list as reversed or rewritten, the design home wins, and the disagreement is a
  defect of the plan document.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. No default awaits the owner: D1, from the review of this document, on the regime of the
  frozen files, is ruled; see "Defaults awaiting the owner".

**Its source.** It was assembled from one session's transcript, extracted with
`cargo klarch transcript show`. The discussion begins at the owner's message that opens "I want to
discuss three subjects together, because they overlap quite a bit: the guidelines about how
projects, docs and code are split", and ends at the owner's message "documentation-has-a-structure-and-a-model
wording approved, no further change needed". A round is one message of the owner and the session's
reply to it:

| round | the owner's message, by its opening words | the session's reply |
| --- | --- | --- |
| 1 | "I want to discuss three subjects together" | headed "Round 1": grounding, criteria, twelve threads, Q1 to Q3 |
| 2 | "Q1 yes, with a step that removes private information" | the draft written and the record search dispatched; two threads opened |
| 3 | "Try again", after a search agent failed twice on an API error | headed "Round 3": the search's verdicts, Q4 to Q6 |
| 4 | "frozen-plan-artifacts: agreed on those shapes" | headed "Round 4": the goal draft, Q7 to Q9, the premortem, the checkpoint table |
| closing | "Q7: right now, would a project be able to define its own shapes via an extension ?", then "documentation-has-a-structure-and-a-model wording approved" | the answer to Q7's factual part; the goal committed |

The session put nine questions to the owner, Q1 to Q9, numbered in the conversation; their labels
are kept here. The search of the whole record ran once, between rounds 2 and 3, over a draft of
this milestone: 19 agents of `agent@knowledge-architect-record-searcher`, whose reports passed
`cargo klarch worklist --verify` and gave 301 entries a verdict other than `unrelated`. Its output,
the draft it read and the goal reviewer's two reports sit in the session's scratch space on the
owner's machine, outside the tree: the search's output in `$CLAUDE_CODE_TMPDIR/split-design/search/`.
Slice 1 commits the search's `generated-lines.md` as this milestone's `search/discussion.txt`, and
nothing else of that output; this document carries each verdict's outcome under "What is already
decided", and the reports of single search agents and of the session's investigators, which no
file of the tree keeps, are cited only beside what a reader can check in the tree. An issue the
commit that adds this document closes, named here by its bare slug, sits in the history under
crates/agent-skills/docs/open-issues/: `git log --diff-filter=D --format=%H -1 -- <its path>` names
the commit that deleted it, and `git show <that commit>^:<its path>` prints it.

## How the work is done

Read `skill@knowledge-architect-planning@working-a-slice` whole before each slice, then follow it.
This repository adds:

- `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", read whole before any edit
  of the installed text's source, and `cargo x skill <name>` to read a shipped text with its `%%`
  comments;
- `skill@klarch-development`, with `skill@knowledge-architect-development`, before the Rust code of
  slice 1;
- a slice whose first change sets what the per-commit gate requires, as slice 1's checker change
  does, puts that change and every fix the tree needs in the branch's first commit, per
  `primer@branch-commits`; so the branch that adds this document merges before slice 1 starts, per
  `skill@knowledge-architect-planning@plan-reviews`, and slice 1 starts on a branch of its own;
- each slice writes its CHANGELOG.md entries in the `Next release` section, per
  `instructions@git-workflow` point 1, and a change of the rules a project's recorded content must
  meet gets a Migration entry, per `design@knowledge-architect@changelog-entries`.

## Names

| name | what it names |
| --- | --- |
| the planning skill | `skill@knowledge-architect-planning`, whose source is `path@agent-skills@content/skills/planning/SKILL.md` |
| the primer's source | `path@agent-skills@content/PRIMER.md`, installed as the primer the root `CLAUDE.md` imports |
| the component-creation skill | `skill@knowledge-architect-component-creation`, source `path@agent-skills@content/skills/component-creation/SKILL.md` |
| the design skill | `skill@knowledge-architect-design`, source `path@agent-skills@content/skills/design/SKILL.md` |
| the default code shape | the fragment `path@agent-skills@fragments/default-code-shape.md`, hosted today by the development skill and `agent@knowledge-architect-code-shape-reviewer`; its decision is `design@agent-skills@default-code-shape` |
| the fragment on when to split | `path@agent-skills@fragments/when-to-split.md`, hosted today by the development skill and the design skill |
| the subject standard | `design@agent-skills@subject-split-standard`, delivered by the fragment `path@agent-skills@fragments/subject-split.md` |
| the search | the search of the whole record by `cargo klarch worklist` and `agent@knowledge-architect-record-searcher`, at a discussion's grounding and at a slice's design audit |
| the draft | the draft of this milestone the search read, written at round 2 under `skill@knowledge-architect-planning@when-planning-starts` |
| the plan model | the plan registers `model` compiles in today: `spec` of the File shape, `milestone` of the Directory shape carrying `spec` and the four item registers, and the item registers of the Section shape, built by `Registers::built_in` around the constants `SPEC_REGISTER`, `MILESTONE_REGISTER`, `ITEM_REGISTERS`, `PLAN_SECTIONS`, `SPECS_HOME` and `MILESTONES_HOME`, with the helper `plan`, all in `path@model@src/manifest.rs` |
| a frozen file | a file under a slice's directory moved to its milestone's `harvested/`, the one frozen location. It leaves the walk by construction, parsed for definitions only, per D1. `transcript.txt` and the files of `search/`, being `.txt` files, are never parsed and need no such rule. See "The regime of each file" |
| harvested | the state of a slice after its slice lands: its directory moved under its milestone's `harvested/`, which is the marker, kept there and frozen until the milestone leaves, cited from inside the milestone only. See `thread@split-guidelines-and-plan-directories@harvested-slice-kept` |
| the code-split scout | the installed agent slice 2 adds, which judges the code a piece of work touches against the split guidelines. See `thread@split-guidelines-and-plan-directories@splits-planned-before-work` |
| the scoped auditor | one `agent@knowledge-architect-design-record-auditor`, dispatched at planning on the heads a harvest will touch, not a whole audit. See `thread@split-guidelines-and-plan-directories@pre-harvest-conformance` |
| the twelve root heads | the heads of `register@knowledge-architect@design` that argue a Component's existence: `design@knowledge-architect@installed-text-crate`, `design@knowledge-architect@model-crate`, `design@knowledge-architect@check-crate`, `design@knowledge-architect@git-crate`, `design@knowledge-architect@query-crate`, `design@knowledge-architect@transcript-crate`, `design@knowledge-architect@create-crate`, `design@knowledge-architect@cli-crate`, `design@knowledge-architect@testing-crate`, `design@knowledge-architect@gates-crate`, `design@knowledge-architect@mutation-command-host`, `design@knowledge-architect@xtask-crate`, one per Component other than the root |
| P1 to P8 | the causes of the premortem, under "Premortem" |
| Q1 to Q9 | the questions the session put to the owner, each put in the session's reply of the round the table above gives and answered in the owner's next message, each restated below |
| D1 | the default awaiting the owner, under "Defaults awaiting the owner" |
| the Groups A to F | the six groups of the search's `conflicts` verdicts, under "What is already decided" |
| thaum | a sibling project of the owner's, which also uses the workflow, read by the session in round 1 for its guidelines on splitting code |

The questions, each with the default the session gave:

- **Q1**, round 1: is committing the trimmed transcript acceptable, as the default for every
  adopting project? Default: yes, with a step before the commit that reads it and removes private
  data and pasted material.
- **Q2**, round 1: are slice specs kept until the milestone leaves, or kept indefinitely? Default:
  until the milestone leaves.
- **Q3**, round 1: one milestone, with the slices in the order of "Implementation sequence"?
  Default: yes, plans first, so the later slices are planned in the new layout.
- **Q4**, round 3: which real session left a split undone, or cut code badly, for lack of the
  module guidelines, the evidence `design@agent-skills@behaviour-additions-need-real-use` asks?
  Default if none: the module guidelines and the scout parked as an issue.
- **Q5**, round 3: the delivery, whose host lists are the owner's per
  `design@agent-skills@shared-text-fragment`. Default: the module guidelines inside the fragment
  of the default code shape, and the code-split scout a new agent hosting that fragment and the one
  on when to split, two new host entries.
- **Q6**, round 3: is reopening `design@agent-skills@one-record-searcher` part of this milestone?
  Default: no.
- **Q7**, round 4: is "from a variety of published shapes" an outcome the goal states, or a means
  left to the design? Default: left out of the goal, so both readings stay admitted.
- **Q8**, round 4: for a domain whose knowledge fits none of the shapes offered, must the goal
  require a new shape, or only admit one? No default.
- **Q9**, round 4: what "cover" means, on the case of a catalogue of hardware parts. A: a procedure
  covers knowledge only where it says how to record that kind; B: wherever it gives the knowledge a
  home. Default: A.

## What the work is

**What exists today.**

- **Splitting guidance, by layer.** Components: the default criteria of the component-creation
  skill, which says "These criteria are guidelines: a Component that departs from one does not
  fail on that alone." Design subjects: the subject standard, whose two triggers are the
  `worklist` bound and heads that "answer questions about parts of the Component that change
  apart". Code inside a Component: the default code shape alone, which its head,
  `design@agent-skills@default-code-shape`, calls "a guideline, not a rule a unit fails alone",
  and whose units the fragment names "a function, a type or a module"; nothing says how a
  Component's code is cut into modules, or when.
- **Where a split is recorded.** The component-creation skill has a Component's purpose recorded
  as the statement of the design head that argues its existence, in the root's design home, and
  says "A Component kept against the default criteria is a decision about the layout, recorded in
  the root's design home." Nothing asks that head to reason over the criteria.
- **When a split happens.** A Component split is decided by the design skill. A module split is
  left to whoever writes the code, and the default code shape weighs "only the lines the change
  writes" in a unit a change edits. A subject split is "a fix in a commit of its own", by whoever
  meets the `worklist` notice. A head a change touches is brought to the rules on heads in that
  change, per `design@agent-skills@existing-heads-on-touch`, so a harvest carries the conformance
  of every head it touches, planned or not.
- **Plan documents.** A spec is one file of `register@plans@spec`; a
  milestone is a directory holding its document, `README.md`, and one spec file per slice. A slice's
  spec leaves when its slice lands, the milestone document when the last slice lands. The milestone
  document holds the discussion's threads and arguments, the owner's words verbatim, and every
  verdict of the search; a slice's design audit gives the whole milestone document to every search
  agent. The measurement under `argument@split-guidelines-and-plan-directories@arg-20` gives the
  sizes.
- **The plan model.** It is compiled into `model` as built-in registers, defined in
  `path@model@src/manifest.rs`; the constants `SPEC_REGISTER`, `MILESTONE_REGISTER`,
  `ITEM_REGISTERS` and `PLAN_SECTIONS`, with `PLANS_DIR` and `SLICE_SECTIONS` of the same file,
  `PLANS_ANCHOR` of `path@model@src/entity.rs` and `Registers::is_plan_register`, are named in 10
  files of four crates, `model`, `check`, `create` and `query`: `grep -rlE` of these eight names
  over crates/*/src and tools/*/src, on the commit before this document's. `SPECS_HOME` and
  `MILESTONES_HOME` of manifest.rs, which locate the two homes, add no file to that count.

**Outside the work**, each with what owns it:

- a check that enforces any split: `rejected@check@split-enforced-by-a-check` stands;
- the anonymization of what `cargo klarch transcript` prints, home paths and branch SHAs among it:
  `issue@transcript@an-option-anonymizes-what-transcript-prints`, a `todo` the owner left for later
  in round 2;
- reopening `design@agent-skills@one-record-searcher`: the owner's Q6 word in round 4 keeps it out;
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it` stays open;
- the design of what a changelog entry may cite: the owner moved
  `issue@knowledge-architect@the-changelog-holds-references-no-check-reads` out of this milestone in
  round 4, to be designed with `issue@knowledge-architect@a-mechanical-changelog-check`;
- a whole pass of the design-record axis over this repository:
  `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`; the scoped auditor
  conforms only the heads one harvest touches;
- splitting the bundled heads that `issue@check@core-heads-bundle-decisions-with-many-citing-sites`
  lists: this milestone gives such work a planned place, and does not do it.

## What is already decided

**The decisions the design rests on and does not argue again**, the seeds the draft named:

- `design@agent-skills@component-skill-delivery`: the Component criteria are guidelines, delivered
  by the component-creation skill at the moments its description names;
- `design@agent-skills@component-split-signals`: the signals against a split, judged, never
  computed;
- `design@agent-skills@default-code-shape`: the default shape of code, a guideline;
- `design@agent-skills@the-workflow-teaches-no-rust-idiom`: what the text says about code is said
  from a higher vantage than one language;
- `design@agent-skills@subject-split-standard`: when a design subject is split;
- `design@agent-skills@existing-heads-on-touch`: a touched head is brought to the rules in the change
  that touches it;
- `design@agent-skills@design-record-axis`: the audit axis whose auditor the scoped auditor reuses;
- `design@agent-skills@milestone-is-a-directory`, `design@agent-skills@milestone-split-by-lifetime`,
  `design@agent-skills@plan-records-the-exchange`, `design@agent-skills@ledger-from-transcript`,
  `design@agent-skills@plan-document-leaves-at-landing`: the plan document's shape, its assembly and
  its life;
- `design@model@spec-file-is-an-anchor`, `design@model@milestone-entries-are-directories`,
  `design@model@plans-split-dirs`, `design@model@declarable-directory-register`,
  `design@check@plan-document-sections-by-kind`: the plan model.

Several of these are rewritten by the work, as the groups below say; the rest bind it.

**The verdicts of the search.** 301 entries got a verdict other than `unrelated`. Their outcomes:

**`conflicts`, `check-reversal`: 56 entries, in six groups.** Each entry the work rewrites is listed,
with every text `cargo klarch show` lists as referencing it and the harvest that judges each text,
in the spec of the slice that rewrites it.

- **Group A, 32 entries, rewritten by slice 1** as the plan model and the plan documents' layout
  change: the work's own reversals. Listed in `spec@split-guidelines-and-plan-directories@plan-directories`.
- **Group B, 13 entries, rewritten by slice 1** to admit one member they do not state today, the
  frozen location of a milestone's directory, `harvested/`, which leaves the walk by construction,
  each citing the one
  head that slice 1's harvest writes for that regime rather than restating it, per
  `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts` and D1. Listed in the same
  spec.
- **Group C, 2 entries, answered with no reversal**: `design@agent-skills@thread-slug-is-entry-id`
  and `design@agent-skills@arguments-as-items`. `deliberation.md` stays a live, item-bearing file of
  the plan, so a thread stays an item under a Threads section and an argument an item under an
  Arguments section; only the file that holds them changes, which slice 1 states in their text
  where it names the file.
- **Group D, 6 entries, rewritten by slices 2 and 3**: `design@agent-skills@subject-split-standard`,
  `design@agent-skills@default-code-shape`, `design@agent-skills@component-skill-delivery`,
  `design@agent-skills@premortem-as-watch-points` and `design@agent-skills@design-audit-checks-fit`
  by slice 2, with the host lists of `design@agent-skills@shared-text-fragment` of Group F;
  `design@agent-skills@existing-heads-on-touch` by slice 3.
  Listed in `spec@split-guidelines-and-plan-directories@planned-splits` and
  `spec@split-guidelines-and-plan-directories@conformance-before-harvest`. Slice 4 reads
  `design@agent-skills@component-skill-delivery` again, since the Component argument changes how
  the skill's criteria are applied.
- **Group E, 1 entry, answered by narrowing a thread**: `design@agent-skills@one-decision-per-head`
  stays a rule, and `thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`
  excludes heads.
- **Group F, 2 entries, argued in round 3**: `design@agent-skills@work-divided-by-judgement`, whose
  property a dispatch fixed at a step needs, which the code-split scout and the scoped auditor meet
  as a point of view apart (`argument@split-guidelines-and-plan-directories@arg-41`); and
  `design@agent-skills@shared-text-fragment`, whose host lists are the owner's, ruled at Q5 in
  round 4. Slice 2 writes the new hosts into the text that lists them.

**`conflicts`, `goal-setting`: 2 goals.**

- `goal@documentation-stays-consistent`: the goal binds "in the texts the checker reads", and the
  frozen files leave the walk by construction, as the project's installed files and generated
  indexes already do with no declaration, per `design@check@installed-files-outside-the-walk`; the
  session's decision at the review of this document, replacing the reading it took in round 3; see
  `criterion@split-guidelines-and-plan-directories@documentation-stays-consistent`.
- `goal@documentation-has-a-structure-and-a-model`: reworded under
  `skill@knowledge-architect-goal-setting` and ruled by the owner in the closing exchange,
  `thread@split-guidelines-and-plan-directories@custom-register-goal`; committed on this branch
  before this document.

**`closes`, `close-with-the-work`: 2 issues**, closed by the commit that adds this document, as the
work that schedules them:

- `a-slice-audit-s-search-reads-the-whole-milestone-document-in-every-agent`, by
  `thread@split-guidelines-and-plan-directories@discussion-record-apart`;
- `plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`, by
  `thread@split-guidelines-and-plan-directories@committed-transcript`. Its closing condition adds
  that the attribution slips of a release's retrospectives are then counted against the new
  regime. That count stays a condition of this milestone, not a cause the harvest may drop: slice
  1's harvest opens a `todo` issue for it, as "Later consequences" says. The baseline it is
  counted against, from the closed issue: in the intake of forty received retrospective files,
  thirteen of its 23 workflow files report slips against the attribution rules of
  `primer@design-heads`, `primer@owner-word-premise` or `primer@question-authority`, at least 24
  slips, each caught before the merge; by kind, an approval or an answer recorded as a ruling 10,
  an act or an argument of the session attributed to the owner 6, a hedge widened 1, a quotation 5,
  in three files, and a premise not shown 2. Only the quotation kind bears on the owner's
  hypothesis, hedged as the owner gave it, that the planning skill's demand for exact quotes
  probably confuses sessions. The major ones, a decision attributed that the owner did not make,
  are in six files, and one slip changed what was built. The issue's text is read as "Status and
  audience" says.

The same commit closes `a-slice-s-spec-with-no-section-of-decided-entries-gives-its-audit-no-seed`,
graded `touches` below, since slice 1 has a slice's own document hold its section "What is already
decided", and with its two members the cluster `the-slice-audit-s-search-on-one-branch`.

**`trigger-met`, `rule-before-work`: 3 issues.**

- `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: the draft argued from the
  "[review] Repairs … bundled heads split" commits, which is review cost. The session dropped that
  argument in round 3, and the case of
  `thread@split-guidelines-and-plan-directories@pre-harvest-conformance` stands on the record
  alone, so the trigger is not met. The issue stays open.
- `issue@knowledge-architect@the-changelog-holds-references-no-check-reads`: its re-entry point
  was the next Migration entry citing installed text, which slice 1 writes. On the owner's word in
  round 4, it becomes a `todo` with no re-entry point, designed with
  `issue@knowledge-architect@a-mechanical-changelog-check`, in the commit that adds this document.
  Slice 1's Migration entry follows `design@knowledge-architect@changelog-entries` as it stands.
- `issue@agent-skills@a-record-search-misses-what-is-added-after-it`: the work changes the
  grounding's steps, which meets its re-entry point. Reopening
  `design@agent-skills@one-record-searcher` is not part of this milestone, on the owner's Q6 word
  in round 4. The issue stays open, with this discussion as an occasion that met its re-entry; a
  thread added after the search gets a search of its own, as the design skill asks. Its re-entry
  point is rewritten on this branch to name an occasion whose own work includes that reopening, so
  that slice 2, which changes the grounding's steps again, does not meet it again.

One issue the search graded `unrelated` has its re-entry point met by the work:
`issue@git@whom-the-library-serves-and-what-it-knows-have-no-argued-head`, whose re-entry point is
the next change that edits `design@knowledge-architect@git-crate`, which slice 4 is. Slice 4's
design audit sorts it as a gap of the slice.

**`touches` and `worsens`: 27 issues.** `respect`, the work keeps within the issue, each named with
the slice that meets it:

- `issue@check@core-heads-bundle-decisions-with-many-citing-sites`: the evidence that splits left
  to whoever meets them sit undone; a conforming step slice 3 plans budgets the reading of every
  citing site, 6 to 39 per head by the issue's count.
- `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head`: slice 2's guidelines become a
  text that tells a session when to split; it argues no bound on a relocation's cost.
- `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`: slices 1
  to 3 add a layout the checker enforces and fixed planning steps; each argues its own need.
- `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`: slice 1's text on specs,
  milestones and slices assumes neither, or names this issue at the site.
- `a-slice-s-spec-with-no-section-of-decided-entries-gives-its-audit-no-seed` and
  `the-slice-audit-s-search-on-one-branch`: closed with this document, above.
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`
  and `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: slice 3's conforming step
  routes a misfiled head as the record settles, and sends the owner only the closed list.
- `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: slice 1's frozen
  files are read by no check once written, the harm this issue names, bounded by their leaving with
  the milestone and by the P3 clause.
- `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` and
  `issue@knowledge-architect@path-quickfixes-read-against-the-renamed-checker`: slice 1's minimal
  migration keeps `spec@plans@path-quickfixes` resolving and its arguments readable, and takes over
  none of the readings those issues ask for.
- `issue@knowledge-architect@command-output-is-not-declared-a-contract`: `transcript.txt` is a frozen
  copy; nothing is built on the output of `transcript show` staying stable.
- `issue@query@no-command-lists-the-numbers-in-use-in-a-plan-across-its-history`: harvested slices
  keep their item numbers in the tree, and `deliberation.md` stays live, so the duplicate check
  still sees every number.
- `issue@check@a-plain-hash-item-name-is-read-by-nothing`: slice 1 redraws which file of a plan
  holds an item, and states which `#<id>` names it reads.
- `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`: slice 1's change
  to `home_text` and `owed_documents` in `path@create@src/command.rs` adds no home text that turns
  false at its first entry.
- `issue@model@a-rust-item-named-in-prose-has-no-checked-form` (`worsens`): planned module splits
  move modules more often; each move-only step of slice 2's procedure searches the prose for the
  old path, a claim of slice 2.

`none`, the issue limits nothing in the work: `issue@query@no-command-counts-a-tripwire-s-horizon`,
`issue@query@no-command-searches-the-entries-or-lists-an-audit-s-work`,
`issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`,
`issue@agent-skills@how-parallel-work-is-partitioned-among-agents`,
`issue@agent-skills@this-repository-s-heads-await-the-design-record-axis`,
`issue@agent-skills@a-lighter-pass-for-untouched-components`,
`issue@check@texts-citing-the-relocation-goal-state-its-old-measure`,
`issue@knowledge-architect@the-documents-do-not-render-as-a-site`,
`issue@knowledge-architect@every-crate-takes-a-version-at-each-release`,
`issue@model@a-slug-in-a-quotation-is-reported-as-a-misplaced-definition`,
`issue@model@the-resolver-s-arms-have-mutation-survivors`. The work limits nothing in the render
issue, but it adds a question to its design, in answer to the search's verdict on
`goal@documents-render-as-a-linked-site`: see "Later consequences".

**`constrains`: 211 entries.** The design keeps within each quoted sentence, action `respect`,
except seven graded `none`, which bear on nothing the design changes:
`design@agent-skills@review-judges-additions`,
`design@agent-skills@enclosing-design-read-at-grounding`, `design@agent-skills@test-economy`,
`design@agent-skills@integrated-test-through-public-api`,
`tripwire@agent-skills@shape-findings-fragment-code`,
`issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data` and
`goal@the-workflow-improves-through-real-use`. Those whose sentence shapes the design, and how:

- `design@agent-skills@branch-commits-judged-at-tip`: slice 1's checker change and every migration
  the tree needs land in its branch's first commit.
- `design@knowledge-architect@no-branch-sha-is-cited`: rewritten by slice 1 so that a transcript's
  quotation of a SHA is no citation; see
  `thread@split-guidelines-and-plan-directories@committed-transcript`.
- `design@knowledge-architect@versioning-policy` and `design@knowledge-architect@changelog-entries`:
  slices 1 and 4 write Migration entries for adopting projects.
- `design@agent-skills@audit-triggers`: slice 4's Migration entry cites the design-record axis, so a
  project's pin move runs it over the heads that argue a Component's existence.
- `design@agent-skills@agents-write-in-scratch`: a committed `search/` holds only the generated
  lines the session copies after `--verify` passes, never the searchers' own directories.
- `design@agent-skills@verdicts-reach-the-conformance-review`: the design-conformance reviewer
  reads the verdict files in the session's scratch space at the review of the plan, as today;
  `search/` commits the generated lines only.
- `design@agent-skills@transcript-extraction-is-the-checker-s` and
  `design@agent-skills@transcript-found-by-listing`: the committed transcript is the output of the
  checker's own command, found by its opening message.
- `design@agent-skills@design-record-axis-dispatch`: the scoped auditor's load stays under that
  head's bound on the entries one auditor reads.
- `design@query@worklist-bound-is-40000-words`: the subject standard's bound stays a trigger, on
  that head's ground of cost; see `argument@split-guidelines-and-plan-directories@arg-38`.
- `design@git@git-is-asked-through-one-module`: one owner per external effect is a reason against a
  split in slice 2's guidelines.
- `design@xtask@ci-builds-the-tip-alone`: a move-only commit is built by the session before its
  push, since CI builds only the tip.
- `design@create@init-command` and `design@create@component-command`: `klarch init` writes the
  plans directory slice 1 defines; `klarch new-component` writes only the new Component's documents
  and register homes, since the `wanted` filter it hands `owed_documents` in
  `path@create@src/command.rs` admits that Component alone, so slice 1 leaves it unchanged.
- `rejected@agent-skills@one-generic-auditor-agent`: the code-split scout is an installed agent,
  never a briefed general-purpose one.
- `design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists` and
  `rejected@agent-skills@module-directory-component`: slice 2's module guidelines state no occasion
  for an intermediate Component, and do not make a module directory a Component.

Every entry graded `constrains`, `respect`, grouped by its kind and anchor:

- Goals (18): `goal@serves-one-kind-of-project`, `goal@every-target-project-can-adopt-it`,
  `goal@adoption-is-easy`, `goal@projects-add-their-own-checks`,
  `goal@documents-render-as-a-linked-site`, `goal@tasks-a-tool-does-better-are-the-tool-s`,
  `goal@design-is-recorded-with-its-arguments`, `goal@records-reach-their-reader`,
  `goal@the-owner-decides`, `goal@the-owner-s-word-is-verified`,
  `goal@design-with-agents-as-counterparts`, `goal@agents-work-without-drift`,
  `goal@generic-tools-and-procedures-are-shipped`, `goal@agents-get-a-complete-workflow`,
  `goal@only-needed-instructions`, `goal@the-workflow-leads-to-better-code`,
  `goal@projects-are-split-into-modular-units`, `goal@session-orchestrates-the-work`.
- `agent-skills`, design (100): `design@agent-skills@knowledge-table-home`,
  `design@agent-skills@primer-limit`, `design@agent-skills@checker-model-in-primer`,
  `design@agent-skills@rarer-checker-rules-by-findings`,
  `design@agent-skills@code-change-reads-its-record`, `design@agent-skills@goals-bind`,
  `design@agent-skills@branch-commits-judged-at-tip`,
  `design@agent-skills@questions-go-to-their-authority`,
  `design@agent-skills@answer-keeps-its-authority`,
  `design@agent-skills@met-outside-the-task-outcomes`,
  `design@agent-skills@a-goal-reading-is-reported`, `design@agent-skills@one-home-for-head-rules`,
  `design@agent-skills@restatement-size-test`, `design@agent-skills@context-cost-of-agent-text`,
  `design@agent-skills@plans-directory-in-primer`, `design@agent-skills@audit-outcomes`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@audit-agent-per-axis`,
  `design@agent-skills@audit-repairs-the-whole-record`, `design@agent-skills@retro-trigger`,
  `design@agent-skills@plan-read-against-the-record`,
  `design@agent-skills@verdicts-reach-the-conformance-review`,
  `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@agents-write-in-scratch`,
  `design@agent-skills@report-written-through-the-shell`,
  `design@agent-skills@reviews-judge-by-the-writer-s-criteria`,
  `design@agent-skills@transcript-reviewer-agent`,
  `design@agent-skills@transcript-extraction-is-the-checker-s`,
  `design@agent-skills@transcript-found-by-listing`,
  `design@agent-skills@goals-axis-in-the-review-before-a-merge`,
  `design@agent-skills@code-shape-axis`, `design@agent-skills@code-claims-scope`,
  `design@agent-skills@design-record-axis`, `design@agent-skills@design-record-axis-dispatch`,
  `design@agent-skills@created-heads-checked-in-the-run`,
  `design@agent-skills@component-crate-mapping`, `design@agent-skills@split-cost-by-publication`,
  `design@agent-skills@component-split-signals`,
  `design@agent-skills@shared-code-decisions-go-to-the-lowest-enclosing-component`,
  `design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`,
  `design@agent-skills@dependency-decision-home`,
  `design@agent-skills@new-or-reshaped-head-needs-design`,
  `design@agent-skills@a-head-is-owed-by-an-entry-test`,
  `design@agent-skills@losing-alternatives-filter`, `design@agent-skills@development-skill`,
  `design@agent-skills@the-workflow-teaches-no-rust-idiom`,
  `design@agent-skills@instruction-record-is-minimal`,
  `design@agent-skills@behaviour-additions-need-real-use`,
  `design@agent-skills@synthetic-evidence-not-built`,
  `design@agent-skills@capabilities-not-structure`,
  `design@agent-skills@no-two-installed-instructions-contradict`,
  `design@agent-skills@statement-names-an-act`, `design@agent-skills@term-defined-at-first-load`,
  `design@agent-skills@admission-rules-home`,
  `design@agent-skills@at-least-one-independent-verification`,
  `design@agent-skills@tripwire-recording-authority`, `design@agent-skills@goal-entry-shape`,
  `design@agent-skills@a-goal-carries-an-example-only-for-its-direction`,
  `design@agent-skills@goals-change-through-goal-setting`,
  `design@agent-skills@goal-standard-rules`,
  `design@agent-skills@a-goal-is-a-purpose-the-project-exists-for`,
  `design@agent-skills@goal-outcome-survives-empty-skeleton`,
  `design@agent-skills@goal-one-meaning-any-breadth`,
  `design@agent-skills@requires-admits-rules-out`, `design@agent-skills@goal-holds-one-intent`,
  `design@agent-skills@goal-drafted-to-move-the-tree`,
  `design@agent-skills@what-shows-a-goal-met-is-design`,
  `design@agent-skills@no-goal-refines-another`, `design@agent-skills@goals-home-at-the-root-only`,
  `design@agent-skills@goal-draft-shows-reach`,
  `design@agent-skills@goal-owner-statements-go-to-the-reviewer`,
  `design@agent-skills@reach-finding-asks-not-narrows`, `design@agent-skills@goal-reviewer-agent`,
  `design@agent-skills@goal-edit-ruled-alone`,
  `design@agent-skills@content-mirrors-the-install-layout`,
  `design@agent-skills@shipped-text-cites-no-entry`,
  `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`,
  `design@agent-skills@no-external-handoff`, `design@agent-skills@one-skill-per-activity`,
  `design@agent-skills@search-reads-the-whole-record`,
  `design@agent-skills@session-acts-on-the-verdicts`,
  `design@agent-skills@search-runs-once-the-shapes-settle`,
  `design@agent-skills@seeds-name-decisions`, `design@agent-skills@ruled-items-labelled`,
  `design@agent-skills@design-home-is-built-intent`, `design@agent-skills@local-intent-binds`,
  `design@agent-skills@heads-stand-on-argument`, `design@agent-skills@fork-choice-in-weak-form`,
  `design@agent-skills@unargued-approval-is-argued`,
  `design@agent-skills@default-ruled-before-merge`, `design@agent-skills@document-vocabulary`,
  `design@agent-skills@step-is-no-commit`, `design@agent-skills@spec-and-milestone`,
  `design@agent-skills@in-change-path`, `design@agent-skills@design-hands-off-to-planning`,
  `design@agent-skills@plan-written-in-the-converging-session`,
  `design@agent-skills@acceptance-criteria-in-the-document`,
  `design@agent-skills@plan-landing-is-not-tied-to-its-work`,
  `design@agent-skills@undesigned-work-is-an-issue`, `design@agent-skills@roadmap-orders-issues`.
- `agent-skills`, rejected (24): `rejected@agent-skills@directive-restated-everywhere`,
  `rejected@agent-skills@finding-reported-not-recorded`,
  `rejected@agent-skills@audit-without-calibration-or-recheck`,
  `rejected@agent-skills@head-repaired-only-when-touched`,
  `rejected@agent-skills@design-listing-cut-in-groups-of-sixty`,
  `rejected@agent-skills@subject-groups-bounded-at-sixty`,
  `rejected@agent-skills@one-generic-auditor-agent`,
  `rejected@agent-skills@module-directory-component`,
  `rejected@agent-skills@head-split-by-independent-reversal`,
  `rejected@agent-skills@entry-test-for-unbuilt-work`,
  `rejected@agent-skills@every-losing-thread-recorded`,
  `rejected@agent-skills@a-goal-for-every-component`,
  `rejected@agent-skills@goal-carries-reading-examples`,
  `rejected@agent-skills@goal-measure-in-what-the-project-builds`,
  `rejected@agent-skills@ledger-written-during-discussion`,
  `rejected@agent-skills@conformance-reader-at-the-checkpoint`,
  `rejected@agent-skills@search-work-carries-the-deliberation`,
  `rejected@agent-skills@search-work-is-the-question`,
  `rejected@agent-skills@slice-audit-seeded-by-the-milestone`,
  `rejected@agent-skills@work-statement-written-for-the-search`,
  `rejected@agent-skills@owner-words-quoted-in-head`,
  `rejected@agent-skills@milestone-holds-every-slice-design`,
  `rejected@agent-skills@plan-document-deleted-with-its-harvest`,
  `rejected@agent-skills@spec-plus-detailed-plan`.
- `check`, design (11): `design@check@model-then-checks`, `design@check@phases-gate-the-report`,
  `design@check@finding-names-the-repair`, `design@check@a-commit-message-is-a-document`,
  `design@check@every-commit-judged-by-the-tip-checker`,
  `design@check@message-references-resolve-in-either-tree`, `design@check@branch-shas-are-refused`,
  `design@check@generated-files-are-pure`, `design@check@groups-are-subdirectories`,
  `design@check@goals-home-cites-only-goals-and-root-homes`,
  `design@check@links-are-navigation-rows`.
- `check`, rejected (1): `rejected@check@split-enforced-by-a-check`.
- `create`, design (2): `design@create@init-command`, `design@create@component-command`.
- `git`, design (1): `design@git@git-is-asked-through-one-module`.
- `knowledge-architect`, design (12): `design@knowledge-architect@model-crate`,
  `design@knowledge-architect@check-crate`, `design@knowledge-architect@query-crate`,
  `design@knowledge-architect@mock-projects-in-one-crate`, `design@knowledge-architect@api-facade`,
  `design@knowledge-architect@git-flow`, `design@knowledge-architect@no-branch-sha-is-cited`,
  `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@changelog-entries`,
  `design@knowledge-architect@the-branch-writes-its-changelog-entries`,
  `design@knowledge-architect@capability-cost-before-migration`,
  `design@knowledge-architect@the-record-lives-in-the-repository`.
- `knowledge-architect`, rejected (1): `rejected@knowledge-architect@model-inside-the-checker`.
- `model`, design (21): `design@model@one-entity-table`, `design@model@two-segment-ids`,
  `design@model@a-slug-elsewhere-defines-nothing`, `design@model@an-entity-belongs-to-its-anchor`,
  `design@model@registers-are-declared`, `design@model@frontmatter-subset`,
  `design@model@declarable-directory-register`, `design@model@item-registers-built-in`,
  `design@model@a-file-register-index-is-rows`, `design@model@one-list-of-generated-files`,
  `design@model@register-home-reference`, `design@model@path-to-a-home-is-reported`,
  `design@model@plans-is-a-reserved-anchor`, `design@model@anchor-collisions-refused`,
  `design@model@kind-names-refused-as-anchors`,
  `design@model@a-component-carries-what-a-project-attaches`,
  `design@model@a-placed-name-reads-as-nothing-else`,
  `design@model@tracked-and-ignored-is-a-finding`, `design@model@model-facade`,
  `design@model@resolver-outcome`, `design@model@cause-words-in-the-resolver`.
- `model`, rejected (4): `rejected@model@subdirectory-anchors-location`,
  `rejected@model@cross-plan-item-citation`,
  `rejected@model@plan-register-references-have-no-anchor`,
  `rejected@model@cause-words-in-the-lints`.
- `query`, design (5): `design@query@cluster-rooted-at-a-subject`,
  `design@query@worklist-writes-and-verifies-the-reports`, `design@query@occasion-shaped-output`,
  `design@query@clusters-packed-under-a-word-bound`, `design@query@worklist-bound-is-40000-words`.
- `transcript`, design (3): `design@transcript@transcript-command-reads-any-project`,
  `design@transcript@transcript-query-verbs`, `design@transcript@cut-text-reported`.
- `xtask`, design (1): `design@xtask@ci-builds-the-tip-alone`.

## Criteria

### The split guidance is a direction a project weighs, and fixes no rule of splitting `##split-is-a-direction`

Binding. Source: `goal@projects-are-split-into-modular-units`, "One purpose per unit is a direction,
not a rule a unit fails alone: this goal fixes no rule of splitting." Satisfied by
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules` and
`thread@split-guidelines-and-plan-directories@module-split-guidelines`.

### The workflow states what an agent needs, and leaves the rest to its judgement `##only-needed-instructions`

Binding. Source: `goal@only-needed-instructions`. Satisfied: the guidelines are judged, never
computed, and each fixed dispatch rests on a property of its step, per
`argument@split-guidelines-and-plan-directories@arg-41`.

### A slice's search meets what bears on the slice `##records-reach-their-reader`

Binding. Source: `goal@records-reach-their-reader`. Satisfied by
`thread@split-guidelines-and-plan-directories@discussion-record-apart`, and watched by
`acceptance@split-guidelines-and-plan-directories@slice-search-reads-the-readmes-only`.

### A kept plan document is no stale text a reader takes for current `##documentation-stays-consistent`

Binding. Source: `goal@documentation-stays-consistent`, which binds "in the texts the checker
reads". Satisfied by `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts` under D1:
a frozen file leaves the walk by construction, parsed for definitions only, as the project already
takes its installed files and its generated indexes out of the walk with no declaration, per
`design@check@installed-files-outside-the-walk`. A frozen file joins that class, so the goal's
words reach it as they reach those files. This is the session's decision at the review of this
document. It replaces the argument the session gave in round 3,
`argument@split-guidelines-and-plan-directories@arg-48`, which left the frozen files in the walk
with their references unjudged and did not defeat the rival reading the search put to the owner:
that their references stay judged, or that they leave the tree at landing.

### A decision is recorded with its arguments `##design-recorded-with-arguments`

Binding. Source: `goal@design-is-recorded-with-its-arguments`. Satisfied by
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`.

### What the workflow says about code is said from a higher vantage than one language `##no-rust-idiom`

Binding, as a presumption. Source: `design@agent-skills@the-workflow-teaches-no-rust-idiom`.
Satisfied: the module guidelines of
`thread@split-guidelines-and-plan-directories@module-split-guidelines` are worded with no language's
terms.

### The words a slice's search reads are bounded by the slice's own documents and a fixed part, whatever the slice count `##slice-search-words-bounded`

Weighed: the owner's aim in round 1, which would become binding with a bound. Satisfied by
`thread@split-guidelines-and-plan-directories@discussion-record-apart`, watched by
`acceptance@split-guidelines-and-plan-directories@slice-search-reads-the-readmes-only`.

### Nothing the owner did not choose to publish is committed `##nothing-published-unchosen`

Weighed: it bears on the committed transcript. Satisfied by the removal step of
`thread@split-guidelines-and-plan-directories@committed-transcript` and by the owner's word on each
transcript before its push, claims of slice 1, per P4 and `goal@the-owner-decides`.

## Threads

Fourteen threads, all closed. The ten below are cited by this document or by more than one slice.
Slice 1's spec holds three: plan documents as directories, their model types, and the migration of
the tree's plan documents. Slice 2's holds one: the record of a split inside a crate.

**The checkpoint table.** In round 4 the session put to the owner a table of every thread with the
state it proposed, under the heading "Checkpoint: contest any of these". The owner answered in the
closing exchange: "I have nothing to contest in the table, you can start planning once we finish the
goal draft and have it commited. Put the milestone on the same branch." That word closed each
thread at the state the table proposed, and promoted the one it proposed `presumed-settled` to
`approved`; it is cited below as "the owner's word on the checkpoint table". Where a thread is
`argued`, the table's argument closed it, and the owner's word left it standing.

### Splitting guidance is a guideline a project weighs, for Components, modules and design subjects; one decision per head stays a rule, and the `worklist` bound stays a trigger on its ground of cost `##split-guidelines-not-rules`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-1`; opened by
  the session in round 1.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-16`, which found the owner's premise
  already true for Components and code, and
  `argument@split-guidelines-and-plan-directories@arg-21`, which kept the `worklist` bound as a
  trigger on a ground `argument@split-guidelines-and-plan-directories@arg-38` replaced, cost.
  `argument@split-guidelines-and-plan-directories@arg-39` narrowed it: "one decision per head" is a
  rule, and the goal names subjects, not heads.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, guidelines for heads
  too, is out on `design@agent-skills@one-decision-per-head`. The owner's word on the checkpoint
  table left it standing.
- Its shape: "Decided design", "Guidelines, and what stays a rule". Harvest: slice 2's, into
  `design@agent-skills@subject-split-standard` and `design@agent-skills@default-code-shape`, which
  slice 2 rewrites to hold the module guidelines.

### Splits of code and of the design record are found and planned before the work: a code-split scout at the design grounding, a `worklist` notice at planning, each split a step or a slice before the work `##splits-planned-before-work`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-6` and
  `argument@split-guidelines-and-plan-directories@arg-7`; opened by the session in round 1 with two
  scouts, `argument@split-guidelines-and-plan-directories@arg-25`.
- Final state: `approved`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-18`, the record's evidence that
  splits left to whoever meets them stay undone;
  `argument@split-guidelines-and-plan-directories@arg-41`, by which the record side became a command
  and the code-split scout a point of view apart; and
  `argument@split-guidelines-and-plan-directories@arg-42`, by which a split a scout finds outside the
  work's area is routed by `primer@met-outside-the-task`. Q5 and Q6 were put to the owner in round 3,
  and `argument@split-guidelines-and-plan-directories@arg-47` asked the real-use evidence that
  `argument@split-guidelines-and-plan-directories@arg-50` gave.
- The owner's words, round 4: "Q5: agreed" and "Q6 no." Closed by the owner's word on the
  checkpoint table. The table's row for this thread also named the P2 clause, which stresses
  `thread@split-guidelines-and-plan-directories@discussion-record-apart` alone, per the premortem's
  table; no premortem cause on this thread became a clause, and P1 and P8 are named for slice 2's
  harvest.
- Its shape: slice 2's spec, "Decided design", "When a split is found and planned". Built and
  harvested by slice 2,
  into `design@agent-skills@subject-split-standard`, `design@agent-skills@default-code-shape`,
  `design@agent-skills@component-skill-delivery` and `design@agent-skills@design-audit-checks-fit`.

### Module guidelines: a module holds what one review reads against one reference, with signals for and against a split, size only as a file a session cannot read whole `##module-split-guidelines`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-4` and
  `argument@split-guidelines-and-plan-directories@arg-5`; shaped by the session in round 1,
  `argument@split-guidelines-and-plan-directories@arg-24`.
- Final state: `approved`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-38`, which labelled the size signal an
  assumption until measured; and `argument@split-guidelines-and-plan-directories@arg-47`, which asked
  for real-use evidence under `design@agent-skills@behaviour-additions-need-real-use`.
- The owner's words, round 4: "Q4: without the guidance thaum gives, modules are basically never
  split, and can grow to thousands of lines, at which point no one will bother. And if the task is
  not planned ahead of time, it never gets done. Either issues accumulate, or nothing at all gets
  done." and "Q5: agreed". The Q4 answer is the owner's report, the real-use evidence; Q5 ruled the
  delivery. Closed by the owner's word on the checkpoint table.
- Its shape: slice 2's spec, "Decided design", "The module guidelines". Built and harvested by
  slice 2, into
  `design@agent-skills@default-code-shape` and the fragment it decides.

### The heads a harvest will touch without reversing them are listed in the plan, judged by one scoped design-record auditor at planning, and conformed by a step before the harvest `##pre-harvest-conformance`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-8`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-19` and
  `argument@split-guidelines-and-plan-directories@arg-26`, the limit that conforming a head can owe
  a decision; `argument@split-guidelines-and-plan-directories@arg-41`, the property of the scoped
  auditor; and `argument@split-guidelines-and-plan-directories@arg-46`, which dropped the argument
  from review cost.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, conforming touched
  heads in the harvest, is the failure the owner reported. The owner's word on the checkpoint table
  left it standing.
- Its shape: slice 3's spec, "Decided design". Harvest: slice 3's, into
  `design@agent-skills@existing-heads-on-touch`, rewritten to narrow "in that change".

### The record of the discussion and the search's output sit beside the documents a slice's search reads: its work is the milestone's and the slice's own documents `##discussion-record-apart`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-9` and
  `argument@split-guidelines-and-plan-directories@arg-13`; shaped by the session in round 1,
  `argument@split-guidelines-and-plan-directories@arg-28`, on the measurement of
  `argument@split-guidelines-and-plan-directories@arg-20`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-40`: `deliberation.md` is live and
  item-bearing, and a slice's own document holds its section "What is already decided"; and by
  `argument@split-guidelines-and-plan-directories@arg-52`, the P2 clause.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, the deliberation
  inside the document the search reads, lost to the measurement of 44 search agents. The owner's
  word on the checkpoint table left it standing.
- Its shape: "Decided design", "The plan document's directory". Built by slice 1; harvest: slice
  1's, into `design@agent-skills@milestone-split-by-lifetime`,
  `design@agent-skills@record-searched-before-the-work` and
  `design@query@seeds-from-the-plan-s-decided-section`.

### A slice moved under `harvested/` is frozen by its location and leaves the walk by construction; `deliberation.md` stays live; the search's generated lines and the transcript are committed as `.txt` files, never parsed; a commit that modifies a harvested file is a finding `##frozen-plan-artifacts`

- Proposed by the session in round 2, `argument@split-guidelines-and-plan-directories@arg-37`.
- Final state: `approved`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-40`, which replaced "the checker
  does not judge them" with "judged when written" after the search found 13 heads in collision, and
  `argument@split-guidelines-and-plan-directories@arg-48`, the reading of
  `goal@documentation-stays-consistent`. The premortem added the P3 clause,
  `argument@split-guidelines-and-plan-directories@arg-52`, stricter than the shape the owner agreed
  to, which the session put to the owner to contest.
- The owner's words, round 4: "frozen-plan-artifacts: agreed on those shapes." The P3 clause was
  closed by the owner's word on the checkpoint table.
- Revised at the review of this document by D1, which the owner ruled: the shape the owner agreed to,
  "judged when written" as a comparison against HEAD, cannot be built as written. No phase of the
  checker may spawn git, per `path@model@CLAUDE.md`, and the shape names no question the gathering
  asks and hands in; and slice 1's own search output fails it, since its `generated-lines.md`
  cites the issues the commit that adds this document deletes. D1 defines a frozen file by its
  location instead, and gives the harvested marker the form the discussion had left open. On D1
  the session recommended committing, of a search's output, the generated lines only, as a `.txt`
  file, so that no rule of a frozen location is needed for them; the owner approved it, ruling D1.
  The owner said then that they had understood from the start that one artifact of the search was
  meant, not its whole directory: the agreement on the shapes in round 4 rested, for `search/`, on
  that misunderstanding, which the session's sketch, naming "the record search's verdict files"
  without saying what they hold, had left open. The title above states D1 as ruled.
- Its shape: "Decided design", "The regime of each file". Built by slice 1; harvest: slice 1's, into
  one head of the regime, which the 13 heads of Group B cite, and a head of its own for the P3
  clause unless the one-decision test of
  `skill@knowledge-architect-decision-recording@head-shape` joins it to the regime's.

### The trimmed transcript of each session of the discussion is committed in the plan document's directory, after a step that removes private information; plan items attribute the owner's words and cite the round `##committed-transcript`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-10`; shaped
  by the session in round 1, `argument@split-guidelines-and-plan-directories@arg-29`, with its
  consequence for every adopting project put to the owner as Q1.
- Final state: `approved`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-34`, the owner's answer, and
  `argument@split-guidelines-and-plan-directories@arg-44`: a SHA the transcript quotes is a
  quotation, not a citation.
- The owner's words, round 2: "Q1 yes, with a step that removes private information. I think we
  might also be able to add a bit of automatic postprocess to the extraction, such as anonymizing
  paths, maybe under an option of the transcript command. This can be kept for later though."
  Closed by the owner's word on the checkpoint table, with the exemption from the rule on branch
  SHAs.
- Its shape: "Decided design", "The committed transcript". Built by slice 1; harvest: slice 1's, into `design@agent-skills@plan-records-the-exchange`,
  `design@agent-skills@attribution-replaces-quotation`, `design@agent-skills@ledger-from-transcript`
  and `design@knowledge-architect@no-branch-sha-is-cited`. The anonymizing option is
  `issue@transcript@an-option-anonymizes-what-transcript-prints`.

### A slice's spec stays in its milestone's directory once its slice lands, marked harvested and frozen, until the milestone leaves `##harvested-slice-kept`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-12`; shaped
  by the session in round 1, `argument@split-guidelines-and-plan-directories@arg-30`, which put the
  duration to the owner as Q2.
- Final state: `approved`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-35`, the owner's argument against
  keeping a harvested spec indefinitely, and by
  `argument@split-guidelines-and-plan-directories@arg-40`, the regime of a frozen file.
- The owner's words, round 2: "Q2 I meant kept until the milestone leaves. There is an argument
  against keeping indefinitely, actually: they would accumulate, pollute grep or other searches with
  outdated matches, and also increase migration work when the checker evolves for little value."
  Closed by the owner's word on the checkpoint table.
- The marker's form, the move of the slice's directory under its milestone's `harvested/`, is D1's,
  which the owner ruled.
- Its shape: "Decided design", "The life of a slice's spec". Built by slice 1; harvest: slice 1's,
  into `design@agent-skills@plan-document-leaves-at-landing`, rewritten at the slice level, and
  `design@agent-skills@tripwire-horizon`.

### The head that argues a Component's existence, or a Component that departs from a criterion, reasons over each shipped criterion; the twelve root heads are brought to the rule `##component-split-argues-the-criteria`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-3`; shaped
  by the session in round 1, `argument@split-guidelines-and-plan-directories@arg-17` and
  `argument@split-guidelines-and-plan-directories@arg-22`.
- Final state: `argued`.
- Moved by `argument@split-guidelines-and-plan-directories@arg-43`, by which slice 4 brings the
  twelve root heads to the rule.
- Closed on `argument@split-guidelines-and-plan-directories@arg-53`: the rival, no argument over the
  criteria required, fails `goal@design-is-recorded-with-its-arguments`. The owner's word on the
  checkpoint table left it standing.
- Its shape: slice 4's spec, "Decided design". Harvest: slice 4's, into a head of `agent-skills`
  slice 4 writes, and `design@agent-skills@component-skill-delivery`.

### The purpose of a project's own kinds of entries is stated by rewording an existing goal `##custom-register-goal`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-15`; the
  session proposed a rewording of `goal@documentation-has-a-structure-and-a-model` rather than a goal
  of its own, `argument@split-guidelines-and-plan-directories@arg-32` and
  `argument@split-guidelines-and-plan-directories@arg-51`.
- Final state: `approved`. The goal reviewer read the draft in two rounds: its first found that the
  draft had replaced the owner's test, knowledge "not covered by the generic procedures we ship",
  with coverage by a shipped kind of entry, and the draft returned to the owner's test; its second
  reported no blocking finding.
- Q7, Q8 and Q9 were put to the owner in round 4. The owner's words, closing exchange: on Q7,
  `argument@split-guidelines-and-plan-directories@arg-54`, whole; "Q8 admit only. This would be
  taken in on my request anyway."; "Q9 A"; and, on the goal by its slug,
  "documentation-has-a-structure-and-a-model wording approved, no further change needed".
  `argument@split-guidelines-and-plan-directories@arg-55` answered the factual part of Q7 from the
  record.
- Its shape: the goal, committed on this branch before this document in the commit "[goals] A project
  declares kinds of entries of its own to record the knowledge of its own domain". No slice builds
  it, and no harvest records it: the goals home is its record.

## Arguments

One sequence, from `argument@split-guidelines-and-plan-directories@arg-1` to
`argument@split-guidelines-and-plan-directories@arg-55`, in order of appearance, shared with the
slice specs. An argument cited only by a thread of one slice's spec is in that spec: seven in slice
1's, two in slice 2's, so the numbers missing below are there. An argument below that also bears on a
slice's thread says so in words; that thread cites it. Key words are verbatim, the owner's spelling
kept.

### Splitting guidance is a set of guidelines, for the reasons that the guidance on dispatching is `##arg-1`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`.
"First, I want to make it clear that on this subject, what we are providing are guidelines to
consider during design, and not hard rules. This concerns splitting project into components, design
and other records into multiple files and subsections by subjects, and code into multiple files or
functions within one component. For nearly the same reasons as the guidelines about how to split
work into subagents: consumer projects will have diverse needs and circumstances, which are hard to
anticipate, so we let target projects reason over their own needs, taking our guidelines into
consideration."

### The argument for a Component's existence reasons over the shipped guidelines, followed or not `##arg-3`

Round 1, the owner. Bears on
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`. "for the
splitting of project into components, such decisions should be recorded at project level. I think
we already have this guideline, but I am unsure whether we had it apply only locallly, or also in
installed text. And I want to add to it that the arguments for the decision should reason about the
guidelines we ship, and not only about the project's local circumstances. If a guideline was
followed, keep record of why it was considered beneficial. If one was not followed, record the
arguments against it, or the costs."

### Code inside a crate has little guidance, and file size is a poor argument for a split `##arg-4`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@module-split-guidelines`.
"I think our guidelines for splitting in Components are already fine. However, I think we have
little to no guidelines about how to architecture code inside a crate. We can maybe take a bit of
inspiration from thaum (../games/thaum), which has a few guidelines about this. They rely on file
size though, and I think we should find better arguments than just file size for the splits."

### The aim: work partitioned into smaller units of change, for agents and for searches `##arg-5`

Round 1, the owner. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules` and
`thread@split-guidelines-and-plan-directories@module-split-guidelines`. "Overall, the aim of those
guidelines for splitting projects is so that work can be more easily partitionned across large
projects, into smaller units of changes. This is what I want to work toward. Because this helps
improve the work efficiency of agents, as well as the efficiency of design searches." On design
documents: "The point of those splits is to have more fine grained, coherent threads for the design
record."

### Splits are thought of at design or planning, and planned as steps or slices; a code split is a design decision, a record split is none `##arg-6`

Round 1, the owner. Bears on
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`, and on a thread of slice
2's spec, which cites it. "I think that splitting
(either at crate or file level, and for both of docs like design and for Rust code) should be thought
of at either of design or planning steps, and planned as their own steps or slices depending on size
of relocation. Probably by dispatching an agent that search for touched areas that are (or are going
to) hit a split condition. Probably one agent for the code side and one for the design record side. I
think the code side probably belongs to the design steps, because a crate or module split is a design
decision. While the splitting of design record is not a decision in itself."

### A split left to implementation is never done `##arg-7`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@splits-planned-before-work`.
"Leaving the splitting decision to implementation never works, because an implementer considers the
task to big to carry as part of same work. And an issue on splitting is likely to be left sitting for
very long, untouched."

### A head the harvest changes without conforming it cannot be judged fairly, and conforming it in the harvest brings unplanned work `##arg-8`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@pre-harvest-conformance`.
"the planning step of anticipating design splits should also cover design entries that are currently
non conformant with the design standard and will be changed by the harvest. Because if the harvest
only changes it without fixing non conformity, the design record review cannot judge fairly against
the standard. And doing such splits in the work/harvest that touches the head brings unplanned work
in scope, which is often hard to accomplish properly. I think this can be done at planning steps, by
dispatching a (single or few, but not full audit) design record auditor manually, which would look at
entries that are expected to be touched by the work and might be non conformant, so that fixing them
can be planned ahead of time (probably as its own slice, or just before harvest if bringing to
conformance is not too large of a task)."

### A milestone's document centralises everything, and grows too large for the searches of later slices `##arg-9`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@discussion-record-apart`,
and on a thread of slice 1's spec, which cites it. "There is
an issue about the fact that plan documents, in particular milestone's README, become far too large.
This severely hinders record searches of later slices. To solve that, I think that plans should
update their structure. Right now, the README centralize everything, which is the root of the
defect." and "The README should stop centralizing everything. More content should be directed at
the slices of milestones (right now they are very thin, far too much). And the README itself could
probably be split."

### The owner's verbatim words have little value out of the context they answered, and the extracted transcript could be committed `##arg-10`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@committed-transcript`.
"For the conversation record, right now the README holds my verbatim words. There is an issue about
that. I now thinks this is positive, but badly done. Out of context of what they answered to, they
have little value. I wonder if the extracted transcript could not just be committed in the milestone
directory, and beside standalone specs too. They should not be large files, once metadata is
trimmed."

### A slice's spec kept, marked harvested, cited from inside its milestone only `##arg-12`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@harvested-slice-kept`.
"Maybe spec documents should not be deleted at each slice. They can be useful for following slices,
especially if they start getting more content. Instead, we can consider a way to mark one as
"harvested", which would make citing it externally (of the containing milestone directory)
forbidden. This would also remove the need to refactor the plan documents themselves after finishing
one slice. And would allow slice specs to hold more items citeable inside the milestone."

### A slice's search should read a document of bounded size, and each slice's artifacts sit in documents of their own `##arg-13`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@discussion-record-apart`.
"The aim of that is so that a design record search of one slice gets a reasonnably sized document,
which does not gorw with slice counts. Artifacts that happen at each slice should be recorded in their
own documents, such as what the previous design searches surfaced (committing the search report as an
artifact directly inside one spec might be enough, it would also be easier than putting it inside
something else)"

### A goal: target projects keep records of domain knowledge in registers of their own `##arg-15`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@custom-register-goal`.
"Related to that: I think it could be made a project goal that "target projects can make their own
registers, from a variety of published shapes, so that they can keep records of custom, domain
related knowledge that is not covered by the generic procedures we ship". Maybe this is already
covered though, or at least partially."

### The guidance is already a guideline for Components and for code, and design subjects have a standard `##arg-16`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`. "**Already true at two of
the three layers.**" It quoted `goal@projects-are-split-into-modular-units`, "One purpose per unit is
a direction, not a rule a unit fails alone: this goal fixes no rule of splitting."; the
component-creation skill, "These criteria are guidelines: a Component that departs from one does not
fail on that alone."; and `design@agent-skills@default-code-shape`, "a guideline, not a rule a unit
fails alone." On design subjects, "**They exist**": the subject standard splits a subject when
`worklist` reports its cluster above the bound, "or its heads answer questions about parts of the
Component that change apart. A split moves headings and decides nothing".

### The installed text records a Component's existence at the root, and asks nothing of its argument `##arg-17`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`. "**It is in the
installed text**, not only here." On the argument: "**Absent.** The one related sentence, SKILL.md:166,
says only: "A Component kept against the default criteria is a decision about the layout, recorded
in the root's design home." It says nothing of what that record argues, and "kept" reads two ways.
This repository's own crate heads do argue from the criteria (`design@knowledge-architect@git-crate`),
but that is practice, not a rule."

### The record shows splits left to whoever meets them stay undone `##arg-18`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`. "**The record supports
it.** `issue@check@core-heads-bundle-decisions-with-many-citing-sites` has stood open since
2026-11-04, "left unsplit because a split must first read every site that cites the head, 6 to 39
sites". The default code shape says outright: "In an existing unit the change edits, only the lines
the change writes are weighed; the rest of the unit is a lead, never an obligation." So no
instruction ever moves an existing debt." The date is wrong: the entry has been open since
2026-10-09, under another path in `core`, and moved to `check` on 2026-11-04, by
`git log --follow --diff-filter=AR --date=short` over its file.

### The harvest carries conformance work nobody planned `##arg-19`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@pre-harvest-conformance`. "`design@agent-skills@existing-heads-on-touch`
says: "A head that a change touches is brought to the rules on design heads in that change", every
bundled decision included. The decision-record reviewer judges the whole head, not the diff. On
2026-11-11 and 11-12 the history holds a run of "[review] Repairs … bundled heads split" commits.
No planning step lists the heads a harvest will touch without reversing them." The last part, the
run of review commits, is an argument from review cost, which
`argument@split-guidelines-and-plan-directories@arg-46` dropped. Its dates are imprecise: the
main branch dates the commits whose subject holds "bundled heads split" 2026-10-25, 2026-11-10
and 2026-11-11.

### The sizes of the plan documents this repository deleted `##arg-20`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@discussion-record-apart`, and on a thread of slice 1's spec, which cites it. Taken with
`git show <deleting-commit>^:<file> | wc -c` over every plan document deleted in this repository's
history: the milestone `goals-at-the-root` README 205,119 bytes at its deletion (182,145 at its first
commit); `development-and-components` 193,273 (172,301); `component-split` 152,106 (82,250); the
standalone spec of the agentic-workflow axis 217,685; the largest slice spec 39,406, "most fall between 20
and 30 KB". Per section of the `goals-at-the-root` README: "Arguments 87,018, What is already decided
57,978, Threads 18,815, Decided design 10,585, everything else under 6,000 each." "So **the
deliberation record (Arguments and Threads) and the copy of the search's outcomes (What is already
decided) make up about 80% of a README**, and most of that mass is already there at its first commit."
The issue it cites, now closed by this document's commit, "measured the consequence: 44 search agents
where the slice spec alone would have needed about 15."

### The `worklist` bound is a measured limit of the tool, so it stays a trigger `##arg-21`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`. "**The worklist bound is
not a guideline.** Above it, a search agent cannot read its cluster whole. That is a measured limit of
the tool, so it rightly reads as a trigger." Overtaken by
`argument@split-guidelines-and-plan-directories@arg-38`, which kept the conclusion on another ground.

### One line per criterion in the head, judged at two sites, so the instruction earns a head `##arg-22`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`. "The head that
argues a Component's existence, or a layout kept against the criteria, takes one line per shipped
criterion" in the shape "one purpose: met — <why it pays here>", "independent reason to change:
departed — <the argument, or the cost accepted>", "signals against a split: <which ones were weighed,
and why none prevails>". "The instruction lives in component-creation step 3, and the
decision-record reviewer judges it there. That makes two sites, so the instruction earns a head in
`agent-skills`. This also rewrites SKILL.md:166 so it means "a Component that departs from a
criterion", which removes the second reading of "kept"."

### A module holds what one review reads against one reference `##arg-24`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@module-split-guidelines`. "The proposed criterion is
thaum's, generalised: **a module holds what one review reads against one reference**. The reference
can be a design head, a section of a specification, an invariant set, or one external effect (as in
`design@git@git-is-asked-through-one-module`)." The signals, each with the failure it prevents: "a
second reference enters the module" (a review that reads code outside its subject; "thaum had 16
modules above its threshold"); "two parts change apart, counting the changes the goals predict"
(unrelated diffs in one file); "an invariant can only be protected by a visibility boundary" (a helper
widened to the whole crate); against a split, "two parts every change reads together, such as a
process and its mutations" (fragmentation) and "one item, such as an enum, is large by itself"
("moving code does not divide an item; dividing it is a design change"). "Size keeps exactly one
role. It is not a number we pick: it is "a file a session cannot read whole in one pass"." thaum's
800-line threshold has no recorded basis in thaum's history, per the session's investigator, a
subagent of round 1 whose report no file keeps; slice 2's design audit reads thaum's tree again,
as its audit subjects say. "The
text stays language-neutral, to respect the no-Rust-idiom head. It also adds a procedure: the split
is a **move-only commit** that comes before the work, so that the review reads the work."

### Two scouts: the code side at the design grounding, the record side at planning; a split found is a step, never an issue `##arg-25`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`. A code-split scout at the
design grounding asks, "for each module/Component the draft touches, does the work push it past a
signal" of the module guidelines or of component-creation, "→ a thread in the discussion"; a record
scout at planning reads "`worklist` notices on the subjects the harvest writes into; subjects whose
heads will answer parts that change apart → a step or slice "split subject X", before the
harvest". "The design audit of each later slice reruns both over that slice's area, since the code
moves between slices. A split the scouts find goes into the plan as a step, or as a slice when it is
large. It is never left as an issue." Revised by
`argument@split-guidelines-and-plan-directories@arg-41` and
`argument@split-guidelines-and-plan-directories@arg-42`.

### Conforming a head can owe a decision `##arg-26`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@pre-harvest-conformance`. "The plan gets a new row
under "What is already decided": **the heads the harvest touches without reversing them**. For
those heads, one `agent@knowledge-architect-design-record-auditor` is dispatched with a narrow brief,
not a whole audit. Each non-conformant head gets a step that conforms it before the harvest. One limit
you should know: bringing a head into conformance is not always only a move. Writing the missing
argument of a head, or the constraint it takes from a goal, can be a decision, so such a step can owe
the design skill or your ruling."

### A slice's search reads the documents of the milestone and of the slice only `##arg-28`

Round 1, the session. Bears on
`thread@split-guidelines-and-plan-directories@discussion-record-apart`. "The search's `--work` reads
only the README files: the milestone's and the slice's. The deliberation and the search output sit
beside them; they are cited and never graded. This closes the issue that measured 44 agents. It also
gives "What is already decided" back its intended role, the seeds and nothing more", which bears
on the issue on a slice's spec with no seed.

### The transcript committed outside the walk, and what that asks of every adopting project `##arg-29`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@committed-transcript`.
"The output of `cargo klarch transcript show`, without tools, is committed as `transcript.txt`. The
`.txt` extension keeps it outside the walk, so the references inside it are never judged and never
break. The deliberation's items then *attribute* your words and cite the round." "The size is
comparable to today's READMEs. The plans investigator measured three sessions on this machine: 16,432,
23,950 and 26,231 words without tools." "**Consequence you must weigh:** the installed text would tell
*every adopting project* to commit the owner's raw words to its repository, public ones included:
every aside, every pasted log. You hold this question (Q1 below)." The plans investigator was a
subagent of round 1 whose report no file keeps; its word counts are taken again with
`cargo klarch transcript show <session> | wc -w` over a session of the project.

### A kept slice's spec defeats both halves of the argument for deleting it `##arg-30`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@harvested-slice-kept`.
"This reverses `design@agent-skills@plan-document-leaves-at-landing` at the slice level. That head's
argument was that a kept document is either walked, so its references break at every reversal, or
unwalked, "unchecked text that a grep finds with no marker that it is stale". The `harvested`
marker defeats the "no marker" half. Freezing its references defeats the "breaks at every
reversal" half. The milestone directory as a whole still leaves when its last slice lands. That
keeps the head's argument at the level where no later slice needs the document."

### Reword the existing goal: a published variety of shapes is a means, the domain knowledge the purpose `##arg-32`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@custom-register-goal`.
"`goal@documentation-has-a-structure-and-a-model` already says "a project can declare kinds of
entries of its own". What your wording adds is a purpose: to keep domain knowledge that the shipped
procedures do not cover. "A variety of published shapes" is the means to that purpose, not a purpose
itself. My default is therefore to reword the existing goal rather than add a new one."

### The committed transcript is acceptable, with a step that removes private information; anonymization later `##arg-34`

Round 2, the owner. Bears on `thread@split-guidelines-and-plan-directories@committed-transcript`.
"Q1 yes, with a step that removes private information. I think we might also be able to add a bit of
automatic postprocess to the extraction, such as anonymizing paths, maybe under an option of the
transcript command. This can be kept for later though."

### Harvested specs kept indefinitely would accumulate, pollute searches and raise migration work `##arg-35`

Round 2, the owner. Bears on `thread@split-guidelines-and-plan-directories@harvested-slice-kept`.
"Q2 I meant kept until the milestone leaves. There is an argument against keeping indefinitely,
actually: they would accumulate, pollute grep or other searches with outdated matches, and also
increase migration work when the checker evolves for little value."

### The search's verdict files would break at the first later reversal unless frozen `##arg-37`

Round 2, the session. Bears on `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`.
"the deliberation, the search output, the transcript and a harvested slice spec are frozen: their
items still resolve inside the milestone, and the checker does not judge the references inside them",
"without it, the verdict files (`.md` full of references) would break at the first later reversal".
Revised by `argument@split-guidelines-and-plan-directories@arg-40`.

### The `worklist` bound rests on cost, and stays a trigger on that ground; the size signal is an assumption `##arg-38`

Round 3, the session, on the search's verdicts. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules` and
`thread@split-guidelines-and-plan-directories@module-split-guidelines`. "**My ground for keeping the
worklist bound as a trigger was wrong.** `design@query@worklist-bound-is-40000-words` rests on
*cost*, not on what an agent can read: "the bound keeps a searcher's peak below that point". A
cluster above the bound is still read whole, by one agent alone. The trigger stays, on that ground. My
claim that a file a session "cannot read whole in one pass" is a size signal for modules is also
unchecked. It stays labelled as an assumption until I measure the Read tool's limit."

### One decision per head is a rule, and the goal names no heads `##arg-39`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`. "**"Splitting is a
guideline" cannot reach heads.** `design@agent-skills@one-decision-per-head` is a rule: a head that
bundles two decisions is a finding. Your goal names three layers: the parts, the code, and design
documents by subject. It does not name heads. So **#split-guidelines-not-rules** covers subjects and
documents, and "one decision per head" stays a rule."

### Freezing collides with 13 heads; a record judged when written answers them `##arg-40`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@frozen-plan-artifacts` and
`thread@split-guidelines-and-plan-directories@discussion-record-apart`. "**Freezing references
collides with 13 heads.**" among them `design@check@the-regime-has-no-opt-out`, "`check` offers
none", `design@model@a-reference-is-live-in-every-prose` and `design@agent-skills@plain-text-is-no-repair`.
"That head's own argument covers the case, though: "A file named is one that is leaving the tree, so
the exemption expires with its subject". Everything frozen leaves with its milestone. There is also a
precedent for a regime other than "re-judged forever": a commit message is judged against the tree
of its own commit, per `design@check@message-references-resolve-in-either-tree`." The regime: "`deliberation.md`
**live**, as a README is today: it defines the thread and argument items, and its
references are judged. This also answers `design@agent-skills@thread-slug-is-entry-id` and
`design@agent-skills@arguments-as-items`, so
neither needs reversing"; the search's output and a harvested slice spec "**judged when written**: its
references are checked by `check` while the file is new or changed against HEAD, and by `commits` in
the commit that writes it; never again after that"; `transcript.txt` "never parsed: the walk reads
`.md` and `.rs` only, so no new way out of the walk is needed". "Each of the 13 heads is rewritten to
state this one member, the dated record, and it leaves the tree with its milestone."

### A dispatch fixed at a step needs a property: the record side is a command, the code-split scout and the scoped auditor are each a point of view apart `##arg-41`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@splits-planned-before-work` and
`thread@split-guidelines-and-plan-directories@pre-harvest-conformance`.
"`design@agent-skills@work-divided-by-judgement`: an instruction fixes a dispatch only "on a property
of the step that holds whatever the task's size", and "Any other 'always dispatch X' is a size claim
in disguise"." "**The record side needs no agent.** At planning, `worklist --out` lists the subjects
the harvest writes into that exceed the bound. A command does it. **The code-split scout** judges the
code the draft touches against a written standard, the split guidelines. That is "a point of view
apart", the property the reviews and the audits use. **The scoped design-record auditor** of
#pre-harvest-conformance is a point of view apart too, and it uses the agent that already exists."

### A split a scout reports outside the work is routed as met outside the task `##arg-42`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`. "**The scouts' findings
need a home when the plan does not take them.** Agent-3 found that "never an issue" is safe only for
a split the work bears on. A split a scout reports outside the work's area is routed by
`primer@met-outside-the-task`, usually as an issue." Agent 3 of the search gave that finding as its
verdict on `design@agent-skills@met-outside-the-task-outcomes` against this thread, with its
reason, in the search's verdict file of that thread, which is not committed; the outcome it led to
is this argument's, and its line in `search/discussion.txt` gives the verdict.

### The twelve root heads would fail the new rule, and slice 4 brings them to it `##arg-43`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`. "**Twelve root
heads would fail the new Component rule.** These are `git-crate`, `model-crate` and the ten others,
which argue a Component's existence without one line per criterion.
**#component-split-argues-the-criteria** now has slice 4 bring all twelve to the rule. That is the
rule's first real use, and the shipped version's Migration entry cites the design-record axis for
adopting projects, per `design@agent-skills@audit-triggers`."

### A SHA the transcript quotes is a quotation, not a citation `##arg-44`

Round 3, the session. Bears on `thread@split-guidelines-and-plan-directories@committed-transcript`.
"`design@knowledge-architect@no-branch-sha-is-cited` refuses a citation of a branch's own commit. The
transcript quotes the conversation verbatim, and the conversation names such SHAs. My default is that
the transcript is exempt, as a quotation and not a citation, and the head is rewritten to say so. The
anonymizing pass you left for later could also replace them."

### The argument from review cost is dropped; the case stands on the record `##arg-46`

Round 3, the session. Bears on
`thread@split-guidelines-and-plan-directories@pre-harvest-conformance`.
"`issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: my case cited the "[review]
Repairs … bundled heads split" commits, which is an argument from review cost. I drop that argument.
The case stands on the record alone: the core-heads issue has been open since 2026-11-04, and the
rule that a touched head is brought to the rules in the same change." The issue has been open
since 2026-10-09, as the note under `argument@split-guidelines-and-plan-directories@arg-18` says.

### An instruction about agent behaviour needs real-use evidence `##arg-47`

Round 3, the session, putting Q4. Bears on
`thread@split-guidelines-and-plan-directories@module-split-guidelines` and
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`.
"`design@agent-skills@behaviour-additions-need-real-use` admits an instruction about agent behaviour
only on evidence from a real session. That evidence is the session, what it lacked, and the mechanism.
thaum's issue listing 16 modules above its threshold is one candidate." Default if none: "both are
parked as an issue stating what a real session must show".

### A dated record is consistent when it was consistent with the tree it was written in `##arg-48`

Round 3, the session, under "Decided". Bears on
`thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`.
"`goal@documentation-stays-consistent` is read so that a dated record is consistent when it was
consistent with the tree it was written in. The rival reading is that every committed file stays
consistent with the current tree. The commit-message regime of
`design@check@message-references-resolve-in-either-tree` already reads the goal my way. Under the
rival reading, no record of a past exchange could ever be committed."

### Deferred issues bring unrelated work into scope at bad moments `##arg-49`

Round 4, the owner. Bears on no thread: it moved
`issue@knowledge-architect@the-changelog-holds-references-no-check-reads` out of the milestone.
"defer that one again. Make it just a todo. These deferred issues are problematic, they often bring
unrelated work in scope at bad moments IMO. I think I will handle this one at the same time when we
design and publish a model and checks for the changelog, which would assert its structure, and take
care of what can be referenced in a changelog and how (there should already be an issue for this)."

### Without guidance modules are never split, and an unplanned split never gets done `##arg-50`

Round 4, the owner. Bears on
`thread@split-guidelines-and-plan-directories@module-split-guidelines` and
`thread@split-guidelines-and-plan-directories@splits-planned-before-work`. "Q4: without the guidance
thaum gives, modules are basically never split, and can grow to thousands of lines, at which point no
one will bother. And if the task is not planned ahead of time, it never gets done. Either issues
accumulate, or nothing at all gets done."

### The goal's added clause qualifies the capability it follows `##arg-51`

Round 4, the session. Bears on `thread@split-guidelines-and-plan-directories@custom-register-goal`.
"I propose the rewording rather than a separate goal. The clause gives the existing capability its
purpose, and a qualifier belongs to the goal it qualifies (rule 5)." Its boundary block: it requires
that "a target project can declare a kind of entry for knowledge of its domain that no shipped
procedure covers"; it admits "one shape or several", "a shape the project defines itself", "plan
documents with model types of their own" and "a declared kind for knowledge a shipped kind could also
hold"; it rules out "nothing new".

### The premortem converts two causes into clauses `##arg-52`

Round 4, the session. Bears on
`thread@split-guidelines-and-plan-directories@discussion-record-apart` and
`thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`. "**The last new argument I
brought** was the premortem itself. It produced two clauses, P2 and P3, and no reopening." P2:
"every condition of a decided thread is in the README's decided design, and the deliberation holds
only the argument; a claim of slice 1". P3: "once its first commit lands, any change to a frozen file
other than its deletion is a finding. This is stricter than the shape you agreed to; contest it if you
disagree".

### The nearest rival of each argued thread, and the fact that defeats it `##arg-53`

Round 4, the session, in the checkpoint table. Bears on
`thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`,
`thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`,
`thread@split-guidelines-and-plan-directories@pre-harvest-conformance` and
`thread@split-guidelines-and-plan-directories@discussion-record-apart`, and on two threads of slice 1's spec, which cite it. "narrowed by
`design@agent-skills@one-decision-per-head`; the rival, guidelines for heads too, is out on that rule"; "the rival, no
argument required, fails `goal@design-is-recorded-with-its-arguments`; slice 4 brings the 12 root
heads to the rule"; "the rival, conforming touched heads in the harvest, is the failure you
reported"; "the rival, a spec as one file, has no place for the artifacts"; "the rival, the
deliberation inside the README, lost to the 44-agent measurement"; "the rival, extending the Directory
shape, collides with your ruling that a declared Directory register carries one register".

### An extension cannot define a shape today; a new shape is taken into knowledge-architect `##arg-54`

The closing exchange, the owner. Bears on
`thread@split-guidelines-and-plan-directories@custom-register-goal`. "Q7: right now, would a project
be able to define its own shapes via an extension ? Or is the entry point only on checks ? The end
result is that if I find a new shape that is useful, by working in another project, I might take it in
knowledge-architect, so another project can easily pick it up if needed. TBH, knowledge-architect
already handles most record itself, and the available shapes are already quite flexible. I suppose we
can leave that out of the goal. In any case, if I want to take it a new shape, I think the goal as
worded already supports it, no ? Or at least admits it."

### An extension enters through checks alone, and the goal as drafted admits a new shape `##arg-55`

The closing exchange, the session, answering Q7's factual part from the record. Bears on
`thread@split-guidelines-and-plan-directories@custom-register-goal`. "No: today an extension cannot
define a shape. It enters through checks alone." It quoted `design@check@an-extension-builds-its-own-model`:
"An extension cannot add a kind to the entity table. A reference kind is `check`'s, so an extension's
subject cannot be cited as `<kind>@<anchor>@<id>`. Nothing needs that today." "The goal as drafted
admits that: "a project can declare kinds of entries of its own under it" fixes no shape and no
number of shapes. With Q8 as admit only, adding a shape stays open and is never required."

## New names, in one place

The names the discussion fixed. The discussion left the model's new types, functions and fields
open: slice 1's spec proposes them under its own "New names, in one place", names its design audit
may change; each slice's spec lists the names it adds.

```
docs/plans/specs/<id>/                     a spec, a directory (slice 1)
docs/plans/milestones/<id>/<slice>/        a slice's spec, a directory (slice 1)
README.md                                  in each plan document's directory: the document the search reads (slice 1)
deliberation.md                            beside it: the Threads and Arguments items (slice 1)
search/<occasion>.txt                      beside it: a search's generated lines, discussion.txt or design-audit.txt, never parsed (slice 1, D1)
transcript.txt                             beside it: the trimmed transcript of each session (slice 1)
harvested/                                 in a milestone's directory: each landed slice's directory, moved there, kept and frozen (slice 1, D1)
the code-split scout                       an installed agent under crates/agent-skills/content/agents/ (slice 2)
```

## Decided design

The shapes that cross slices. The shapes only one slice builds are in that slice's spec.

### Guidelines, and what stays a rule

For `thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`.

- **Guidelines a project weighs**, never a rule a unit fails alone: the Component criteria and their
  signals against a split, as today; the module guidelines slice 2 adds; the second trigger of the
  subject standard, heads that answer questions about parts that change apart.
- **Rules that stay rules**: one decision per head, per `design@agent-skills@one-decision-per-head`,
  since the goal names Components, code and design documents by subject, never heads; and the
  `worklist` bound as the first trigger of the subject standard, on the ground of
  `design@query@worklist-bound-is-40000-words`, cost: a cluster above the bound is still read whole,
  by one agent alone, at a peak the bound exists to keep down.
- **What records a split.** A Component's existence: its head in the root's design home, reasoning
  over each shipped criterion, slice 4. A module split: no head unless a test of
  `primer@design-heads` holds, slice 2. A subject split: no decision, a move.
- Nearest rival: the guideline status extended to heads. Defeated by
  `design@agent-skills@one-decision-per-head`, which makes a bundled head a finding, and by the goal's
  words, which name no heads (`argument@split-guidelines-and-plan-directories@arg-39`).

### The plan document's directory

For `thread@split-guidelines-and-plan-directories@discussion-record-apart`. Slice 1 builds it, and
every later slice is planned and audited in it.

```
docs/plans/specs/<id>/
  README.md          the document the search reads: the decided design, the steps, the criteria,
                     the acceptance criteria, the harvest, and its section "What is already decided"
  deliberation.md    the Threads and Arguments items, attributing the owner's words, each citing its round
  search/            discussion.txt, the generated lines of the discussion's search, and design-audit.txt
  transcript.txt     the trimmed transcript of each session, private information removed
docs/plans/milestones/<id>/
  README.md          what crosses slices
  index.md           generated: the slices, as the model of slice 1 defines the listing
  deliberation.md
  search/
  transcript.txt
  <slice>/           each live slice a spec directory as above
  harvested/
    <slice>/         each landed slice's directory, moved here, kept until the milestone leaves
```

- **What a directory owes**: `README.md`, always. `deliberation.md` where the document has a thread
  or an argument; slices 3 and 4 of this milestone have none. `search/` where a search ran for the
  document, at its discussion's grounding or at its design audit, and `transcript.txt` where the
  document was assembled from a discussion's transcript; `spec@plans@path-quickfixes` holds
  neither. The checker refuses any other file or directory, and a milestone's directory admits
  besides its generated `index.md`, its slice directories and `harvested/`.
- **What `search/` holds**, on the owner's choice in ruling D1: of what `worklist` writes, only
  the generated lines, each as a `.txt` file named by its occasion: `generated-lines.md` of the
  discussion's search as `search/discussion.txt`, and, at a slice's or a spec's design audit,
  `fit-lines.md` as `search/design-audit.txt`, in the audit's commit, under the slice's own
  `search/`. Nothing else is committed: not the `verdicts-<subject>.md` files, `shown.md`,
  `gaps.md`, the agent directories, `agents.txt` or `work.txt`, the last three the searchers'
  working files, per `design@agent-skills@agents-write-in-scratch`. Being `.txt`, the walk never
  parses them, so they never break and need no rule of a frozen location. The searchers' reasons
  are not kept: a reason that changed something is carried by the outcome the `README.md` records.
- **What "What is already decided" keeps of the search**: each verdict whose outcome changed
  something, with that outcome, and a link to `search/discussion.txt` for the rest; it no longer
  copies the full list of verdicts. A rule of the planning skill slice 1 writes; this document's own
  list stays as it is until slice 1's migration applies the rule to it.
- Rival, committing the verdict files and the reports: on this discussion's search, by `wc -c` over
  its output in the session's scratch space, the whole output holds 8,193,395 bytes, the verdict
  files 1,922,848 holding 617 copied entry blocks for 301 entries, `shown.md` 2,051,313 and the 19
  reports 413,365, where `generated-lines.md` holds 22,581; and a report's quotations go stale as
  the record changes. Lost to the session's recommendation of the generated lines only, which the
  owner approved.
- **How a `README.md` reaches the files beside it**: by relative navigation links, which the
  checker resolves in a `README.md`, to `deliberation.md`, `transcript.txt` and `search/`; and an
  item of `deliberation.md` by its item reference, as today.
- **A slice's search reads the documents of the milestone and of the slice**: `--work` is the
  milestone's `README.md` and the slice's `README.md`, and `--seeds-from` the slice's `README.md`,
  which holds its own section "What is already decided". `deliberation.md`, `search/` and
  `transcript.txt` are linked and never graded.
- **The P2 clause**: every condition of a decided thread is in the decided design of a document the
  search reads; `deliberation.md` holds only the argument. A claim of slice 1.
- Nearest rival: the deliberation kept inside the document the search reads. Defeated by the
  measurement of `argument@split-guidelines-and-plan-directories@arg-20`: the deliberation record and
  the copy of the search's outcomes made 86% of the `goals-at-the-root` milestone's document at its
  first commit and 80% at its deletion, and a slice's audit took 44 search agents where its spec
  alone needed about 15. The same closed issue on a slice audit's search reported that seven of
  the first 27 reports of that audit said the agent could not read the document whole, and read it
  in parts and by search, a strain on `design@agent-skills@search-reads-the-whole-record` and on
  `design@agent-skills@milestone-split-by-lifetime`: a second premise, on the quality of each
  agent's reading, which a larger `worklist` bound would not answer.

### The regime of each file

For `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`.

This is D1's design, which the owner ruled. **A frozen file is defined by its location, a slice's
directory under `harvested/`, and leaves the walk by construction**, parsed for definitions only, as
`design@check@installed-files-outside-the-walk` makes the installed files:

| file | regime | how it gets there |
| --- | --- | --- |
| `README.md` of a plan document or of a live slice | live, as today | written by the session |
| `deliberation.md` of a plan document or of a live slice | live: it defines the thread and argument items, and its references are judged | written by the session |
| `search/<occasion>.txt` | never parsed: a `.txt` file, as `transcript.txt` | written by `worklist --verify` from the tree of its time, and copied there |
| any file under a slice's directory moved under its milestone's `harvested/`, its `README.md` and `deliberation.md` included | frozen | the move is the harvested marker, in a commit of its own after the review before the slice's merge; the slice is live until that commit |
| `transcript.txt` | never parsed: the walk reads `.md` and `.rs` only (`LIVE_SUFFIXES` in `path@model@src/walk.rs`) | committed after the removal step, on the owner's word |

- **"Judged when written" holds as**: a search's generated lines are written by `worklist --verify`
  from the tree of its time and never parsed after, and a slice's documents are judged live until
  the commit that moves them.
- **Parsed for definitions only**: an item a frozen file defines resolves from inside its
  milestone, and a live item that reuses one of its ids is still reported, so the duplicate check
  sees every number; a frozen file raises no finding of its own.
- **The P3 clause**: `cargo klarch commits` reports a commit that modifies a file under
  `harvested/`, other than adding it, deleting it, or the move that freezes a slice. It applies to
  every commit, on the branch that wrote the file as on any later one.
- **A citation of a harvested slice**, or of an item it defines, from outside its milestone's
  directory is refused, as `thread@split-guidelines-and-plan-directories@harvested-slice-kept`
  decided.
- **The regime is one decision, with one head**, written by slice 1's harvest. The 13 heads of
  Group B each admit the frozen location by citing it, rather than each restating it; the P3
  clause takes a head of its own unless the one-decision test of
  `skill@knowledge-architect-decision-recording@head-shape` joins it to the regime's. A frozen file
  leaves the tree with its milestone, as `design@check@the-regime-has-no-opt-out` argues of an
  exemption whose file is leaving the tree.
- Nearest rival: the git comparison against HEAD, "judged when written" as the round-3 shape the
  owner agreed to stated it, a frozen file's references judged while it is new or changed against
  HEAD. Defeated by two facts: no phase of the checker may spawn git, per `path@model@CLAUDE.md`,
  and the shape names no question the gathering would ask and hand in; and slice 1's own search
  output fails it, since its `generated-lines.md` cites the issues the commit that adds this
  document deletes, so the commit that adds the output would report a dangling reference for each.
- Earlier rival: the frozen files left in the walk with their references unjudged, the round-2
  shape. Defeated by the 13 heads of Group B, `design@check@the-regime-has-no-opt-out` among them,
  which a file in the walk whose references go unjudged contradicts
  (`argument@split-guidelines-and-plan-directories@arg-40`).

### The committed transcript

For `thread@split-guidelines-and-plan-directories@committed-transcript`.

- For each session the discussion spans, the output of `cargo klarch transcript show`, from the
  message where the discussion begins to the message where the plan document is committed, trimmed
  of tool calls, is committed as the plan document's `transcript.txt`, after a step, named in the
  planning skill, that reads it and removes private information and pasted material. A claim of
  slice 1, per P4.
- **The removal step removes**: every path under a home directory, every e-mail address, every
  credential and token, and the pasted material the owner marks.
- **The owner decides each publication.** Once a transcript is pushed, it cannot be taken back, an
  action `goal@the-owner-decides` reserves to the owner. So after the removal step the session
  shows the owner the transcript, and pushes it only on the owner's word for that transcript, in
  every project the planning skill instructs. The owner's Q1 word here rules the policy, not each
  publication, and it does not stand for another project's owner.
- Plan items attribute the owner's words and cite the round of the transcript; the verbatim words
  stay in the transcript. This is the closing condition of the issue
  `plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`, closed by the
  commit that adds this document.
- A commit SHA the transcript quotes is a quotation, not a citation:
  `design@knowledge-architect@no-branch-sha-is-cited` is rewritten to say so.
- The anonymization of home paths and branch SHAs by an option of `klarch transcript` is
  `issue@transcript@an-option-anonymizes-what-transcript-prints`, outside the work.
- Nearest rival: the owner's words verbatim in the plan items. Defeated by the owner's report that
  out of the context they answered they have little value
  (`argument@split-guidelines-and-plan-directories@arg-10`), and by the slips the closed issue
  counted where the two regimes meet: of at least 24 attribution slips in 13 workflow files, only
  the quotation kind bears on the hypothesis that the verbatim regime confuses sessions, 5 slips in
  three files, as "What is already decided" gives the baseline.

### The life of a slice's spec

For `thread@split-guidelines-and-plan-directories@harvested-slice-kept`.

- When a slice lands, its spec is marked harvested instead of deleted: its directory is moved under
  its milestone's `harvested/`, which is the marker, per D1, in a commit of its own after the review
  before the merge, so the message that cites it resolves in its parent's tree. It is frozen from
  that commit on, and live until it.
- Its items still resolve from inside the milestone; a citation of it from outside the milestone's
  directory is refused.
- The milestone's directory leaves whole when its last slice lands, as a milestone leaves today.
- Nearest rival: a harvested spec kept indefinitely. Defeated by the owner's argument
  (`argument@split-guidelines-and-plan-directories@arg-35`): they would accumulate, put outdated
  matches in every search, and raise the migration work of every change of the checker.

## Mapping tables

Each slice's spec holds the tables of its own work: slice 1's maps each plan document of the tree to
its place in the new layout. None crosses slices.

## Losing alternatives

The losing alternatives of the threads above. Those of slice 1's and slice 2's own threads are in
their specs.

- **Guidelines for heads too**, under `thread@split-guidelines-and-plan-directories@split-guidelines-not-rules`:
  lost to `design@agent-skills@one-decision-per-head`.
- **The `worklist` bound kept as a measured limit of what an agent reads**, the ground of
  `argument@split-guidelines-and-plan-directories@arg-21`: superseded by the ground of cost,
  `argument@split-guidelines-and-plan-directories@arg-38`; the conclusion stands.
- **A record scout, an agent, at planning**, under
  `thread@split-guidelines-and-plan-directories@splits-planned-before-work`: lost to a command,
  `worklist --out`, which needs no point of view apart
  (`argument@split-guidelines-and-plan-directories@arg-41`).
- **A split never left as an issue, wherever found**, under the same thread: lost to the search's
  verdict on `design@agent-skills@met-outside-the-task-outcomes`, that this holds only for a split
  the work bears on; one outside it is routed by `primer@met-outside-the-task`
  (`argument@split-guidelines-and-plan-directories@arg-42`).
- **Conformance in the harvest**, under
  `thread@split-guidelines-and-plan-directories@pre-harvest-conformance`: the failure the owner
  reported (`argument@split-guidelines-and-plan-directories@arg-8`).
- **The deliberation inside the document the search reads**, under
  `thread@split-guidelines-and-plan-directories@discussion-record-apart`: lost to the 44-agent
  measurement.
- **`--seeds-from` taking the rows of the milestone document's table that name the slice**, the
  other form the closed issue on a slice's spec with no seed offered, under the same thread: lost
  to a slice's own `README.md` holding its section "What is already decided", which needs no new
  form of `--seeds-from`. The issue measured what an unseeded audit costs: at the second slice's
  audit of the milestone that moved the goals to the root, of the 53 entries that are or cite one
  of the slice's 8 decided entries, 7 had been graded `unrelated` and their agents graded them
  again.
- **The git comparison against HEAD**, under
  `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`: lost to the two facts of
  "The regime of each file", that no phase may spawn git and that slice 1's own search output
  fails it.
- **Frozen files left in the walk and never judged**, under the same thread: lost to the 13 heads of
  Group B.
- **The search's verdict files and reports committed beside its generated lines**, under the same
  thread: lost to the owner's choice of the generated lines only, in ruling D1, on the sizes
  "The plan document's directory" gives and the staleness of a report's quotations.
- **The owner's words verbatim in plan items**, under
  `thread@split-guidelines-and-plan-directories@committed-transcript`: lost to the owner's report,
  `argument@split-guidelines-and-plan-directories@arg-10`, and to the count of quotation slips,
  5 of at least 24 attribution slips.
- **Harvested specs kept indefinitely**, under
  `thread@split-guidelines-and-plan-directories@harvested-slice-kept`: lost to the owner's argument,
  `argument@split-guidelines-and-plan-directories@arg-35`.
- **No argument over the criteria required**, under
  `thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria`: fails
  `goal@design-is-recorded-with-its-arguments`.
- **A goal of its own for a project's kinds of entries**, under
  `thread@split-guidelines-and-plan-directories@custom-register-goal`: lost to rule 5 of
  `skill@knowledge-architect-goal-setting@goal-or-not`, a qualifier belongs to the goal it qualifies.
  **The phrase "from a variety of published shapes"** was left out of the goal on the owner's Q7
  answer.

## Readings

None: the work reads no external specification the project implements.

## Premortem

The session assumed the milestone shipped and failed, in round 4, and named these causes:

| # | cause | thread it stresses | verdict |
| --- | --- | --- | --- |
| P1 | the code-split scout proposes a split on every module it touches; plans fill with move-only steps that buy nothing | `thread@split-guidelines-and-plan-directories@splits-planned-before-work`, `thread@split-guidelines-and-plan-directories@module-split-guidelines` | named as a cause for slice 2's harvest to judge as a tripwire; the guidelines' signals against a split are the guard, and `tripwire@agent-skills@shape-findings-fragment-code` already watches the code side |
| P2 | a slice's search reads only the documents of the milestone and the slice, and misses a condition stated only in `deliberation.md` | `thread@split-guidelines-and-plan-directories@discussion-record-apart` | converted into a clause of the design, "The plan document's directory": every condition of a decided thread is in a document the search reads; a claim of slice 1 |
| P3 | a frozen file is edited later, for example re-pointed by a migration; the checker then judges it against the tree of the day, and sessions rewrite history to pass | `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts` | converted into a clause of the design, "The regime of each file": a commit that modifies a file under `harvested/`, other than adding it, deleting it or the move that freezes a slice, is a finding, as D1 restates it; a claim of slice 1 |
| P4 | private data reaches a public repository through `transcript.txt`, which cannot be undone | `thread@split-guidelines-and-plan-directories@committed-transcript` | a claim of slice 1's removal step and of the owner's word on each transcript before its push; a check that could be scripted, such as finding home-directory paths and e-mail addresses, goes into `issue@transcript@an-option-anonymizes-what-transcript-prints`, never a tripwire |
| P5 | a later slice cites an item of a harvested slice whose decision a later slice of the same milestone reversed | `thread@split-guidelines-and-plan-directories@harvested-slice-kept` | named as a cause for slice 1's harvest to judge; the status section of every plan document says that a design home wins over it |
| P6 | the harvest touches heads the planning step did not list, and the unplanned conformance work comes back | `thread@split-guidelines-and-plan-directories@pre-harvest-conformance` | named as a cause for slice 3's harvest to judge; the spec-conformity review reports a touched head missing from the list |
| P7 | the one line per criterion turns into a box ticked "met" with no reason | `thread@split-guidelines-and-plan-directories@component-split-argues-the-criteria` | named as a cause for slice 4's harvest to judge as a tripwire: its evidence lands in the design-record audit's reports |
| P8 | a planned split step is dropped, or turned into an issue, at the landing: the same failure moved earlier | `thread@split-guidelines-and-plan-directories@splits-planned-before-work` | named as a cause for slice 2's harvest to judge; the spec-conformity review reports a step that was not built |

## Acceptance criteria

### AC1: a slice audit's search reads the milestone's and the slice's own documents, and no record of the discussion `##slice-search-reads-the-readmes-only`

- Guards `thread@split-guidelines-and-plan-directories@discussion-record-apart`.
- Judged at the design audit of slices 2, 3 and 4.
- Fires when the work given to a slice audit's search, its `--work` files, holds a Threads or an
  Arguments section, or any file other than the milestone's `README.md` and the slice's own
  `README.md`. Basis: the measurement of the closed issue on a slice audit's search, 44 agents
  against about 15 for the slice's spec alone, `argument@split-guidelines-and-plan-directories@arg-20`.
- Admitted on conditions 1, 3 and 4 of the standard, per
  `skill@knowledge-architect-planning@acceptance-criteria`: the failure is silent, since the search
  still passes `--verify`; the evidence is what the decision says must not happen; the clause names
  an event, no bound.

## Implementation sequence

1. [Plan documents as directories, with model types of their own](plan-directories.md),
   `spec@split-guidelines-and-plan-directories@plan-directories`. Builds the plan model, the layout,
   the regime of each file, the planning skill's and the primer's text, and migrates
   `spec@plans@path-quickfixes`, minimally, and this milestone, properly. Fails alone on: the
   checker refusing the migrated tree, or a slice audit's `--work` still the whole milestone.
2. [Splits planned before the work](planned-splits.md),
   `spec@split-guidelines-and-plan-directories@planned-splits`. Builds the module guidelines inside
   the default code shape, the code-split scout, the record side at planning and the routing of a
   split found outside the work. Fails alone on: a plan written under the new text with no split step
   where a guideline's signal holds, or a scout's finding with no outcome.
3. [The heads a harvest touches, conformed before it](conformance-before-harvest.md),
   `spec@split-guidelines-and-plan-directories@conformance-before-harvest`. Builds the list of touched
   heads, the scoped auditor's dispatch at planning and the conforming step. Fails alone on: a
   harvest that conforms a head the list did not name.
4. [The head of a Component's existence argues each criterion](component-arguments.md),
   `spec@split-guidelines-and-plan-directories@component-arguments`. Builds the rule, rewords the
   component-creation skill's sentence on a Component kept against the criteria, and brings the
   twelve root heads to the rule. Fails alone on: a root head with no line for a shipped criterion.

## Order rationale

- Slice 1 before slice 2: the later slices are planned and audited in the new layout, and slice 1
  migrates this milestone so that their audits are its first real use, on the owner's Q3 word.
- Slice 2 before slice 3: both add steps the planning skill places before the work; slice 2 writes
  the first of them, the split step, and slice 3 adds the conforming step beside it.
- Slice 3 before slice 4: slice 4's harvest touches twelve heads, and its plan is the first whose
  list of touched heads slice 3's step judges.

The order is the session's default at Q3, which the owner's word of round 2 approved.

## Defaults awaiting the owner

None awaits. Every default the discussion put to the owner was ruled by the owner's word on the
checkpoint table, in the closing exchange, or by an answer to a labelled question. The defaults the
session decided on the record are written where they apply, with their ground: the threads closed
`argued`, each with the argument that closed it; the Premortem's verdicts; and the satisfaction
line of each criterion. One default came from the review of this document:

- **D1**, ruled by the owner, on `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`
  and the marker of `thread@split-guidelines-and-plan-directories@harvested-slice-kept`. A frozen
  file is defined by its location, a slice's directory moved under its milestone's `harvested/`,
  the move being the harvested marker, in the commit after the review before the slice's merge; it
  leaves the walk by construction, parsed for definitions only, as
  `design@check@installed-files-outside-the-walk` makes the installed files. The P3 clause
  becomes: `commits` reports a commit that modifies a file under `harvested/`, other than adding
  it, deleting it, or the move that freezes a slice. The regime is one head, which the 13 heads of
  Group B cite. The part of the default on the search's output was replaced on the session's
  recommendation, which the owner approved: a plan directory commits, of a search's output, only
  the generated lines, `generated-lines.md` of the
  discussion's search and `fit-lines.md` of a design audit, each as `search/<occasion>.txt`, which
  the walk never parses, as it never parses `transcript.txt`, so neither needs the rule of a frozen
  location. What D1 changes from what the owner agreed to in round 4: "judged when written" becomes
  a consequence of where the file sits or of its suffix, not a comparison against HEAD, which no
  phase can make and which slice 1's own search output fails; and the harvested marker takes a
  form the discussion had left open. Written into "The plan document's directory", "The regime of
  each file", "The life of a slice's spec", the Names, the Premortem's P3 and slice 1's spec.

## Harvest

Each slice's row is in its spec. The row of this document:

- **When the last slice lands**: the milestone's directory leaves whole, in a commit of its own,
  after the review before the merge, per `skill@knowledge-architect-planning@plan-document-leaves`.
- **The premortem's causes** P1, P5, P6, P7 and P8 are judged at the harvest of the slice the table
  names, by the standard of `skill@knowledge-architect-decision-recording@ruled-tripwires`; P2, P3
  and P4 are claims of slice 1, and slice 1's harvest row carries their outcomes into its commit
  message.
- **AC1** is reported on at each of slices 2, 3 and 4, and once more when this document leaves.
- **The design** of the five subsections above lands in the heads each thread's item names, at the
  harvest of the slice that builds it.

## Later consequences

- `design@agent-skills@one-record-searcher` is not reopened, on the owner's Q6 word:
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it` stays open, and its re-entry
  point is now a design discussion whose own work includes that reopening.
- The render's design, `issue@knowledge-architect@the-documents-do-not-render-as-a-site`, decides
  whether a frozen file's references link to their targets as of the commit that froze it, or the
  frozen files stay out of the site, for `goal@documents-render-as-a-linked-site`: a frozen file's
  reference to an entry a later commit removed names nothing in the current tree. No render exists
  yet, so nothing is built for it here.
- The count of attribution slips that the closed issue on verbatim words asked for: slice 1's
  harvest opens a `todo` issue in `agent-skills` that counts the slips of the retrospectives of the
  next release after slice 1's, by the kinds of the baseline under "What is already decided", and
  compares the count with that baseline.
- The anonymizing option of `klarch transcript`,
  `issue@transcript@an-option-anonymizes-what-transcript-prints`, would replace part of the manual
  removal step of slice 1, and carry P4's scriptable check.
- What a changelog entry may cite, and how, is designed with its model:
  `issue@knowledge-architect@the-changelog-holds-references-no-check-reads` and
  `issue@knowledge-architect@a-mechanical-changelog-check`.
- The bundled heads of `issue@check@core-heads-bundle-decisions-with-many-citing-sites` are split
  when a plan touches them, by slice 3's step, or by a plan of their own.
