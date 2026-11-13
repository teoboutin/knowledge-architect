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
when its last slice has landed and every default it lists is ruled. This document and its slice
specs are in the layout slice 1 builds, the one `skill@knowledge-architect-planning@plans-layout`
states: the threads and the arguments in [the milestone's deliberation](deliberation.md) and in
each slice's own, the discussion's transcript in [transcript.txt](transcript.txt), and the lines of
the discussion's record search in [search/discussion.txt](search/discussion.txt).

- Written for a session that did not witness the discussion.
- Where this document or a slice's spec and a design home disagree on a point that "What is already
  decided" does not list as reversed or rewritten, the design home wins, and the disagreement is a
  defect of the plan document.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. D1, from the review of this document, on the regime of the frozen files, is ruled; D2,
  from the review of slice 1's work, on the P3 clause, awaits the owner; see "Defaults awaiting the
  owner".

**Its source.** It was assembled from one session's transcript, extracted with
`cargo klarch transcript show`, and committed, trimmed of tool calls and after the removal step of
`skill@knowledge-architect-planning@spec-contents`, as [transcript.txt](transcript.txt), on the
owner's word for that transcript. The discussion begins at the owner's message that opens "I want
to discuss three subjects together, because they overlap quite a bit: the guidelines about how
projects, docs and code are split", and its last round is the owner's message
"documentation-has-a-structure-and-a-model wording approved, no further change needed". The
transcript runs on through the review of this document, to the owner's last message before the
branch that added this document merged, so that the rulings given at that review are in it. A round
is one message of the owner and the session's reply to it:

| round | the owner's message, by its opening words | the session's reply |
| --- | --- | --- |
| 1 | "I want to discuss three subjects together" | headed "Round 1": grounding, criteria, twelve threads, Q1 to Q3 |
| 2 | "Q1 yes, with a step that removes private information" | the draft written and the record search dispatched; two threads opened |
| 3 | "Try again", after a search agent failed twice on an API error | headed "Round 3": the search's verdicts, Q4 to Q6 |
| 4 | "frozen-plan-artifacts: agreed on those shapes" | headed "Round 4": the goal draft, Q7 to Q9, the premortem, the checkpoint table |
| closing | "Q7: right now, would a project be able to define its own shapes via an extension ?", then "documentation-has-a-structure-and-a-model wording approved" | the answer to Q7's factual part; the goal committed; then this document written and reviewed, and D1 and Q10 put |
| review 1 | "D1: what exactly would end up in the search/ directory ?" | the output of the search measured; D1 and Q10 restated with a recommendation |
| review 2 | "D1 agreed on recommendation" | D1 written as ruled; the goals home's definition of the record drafted, with Q11 to Q13 |
| review 3 | "Agreed on Q11 Q12 Q13, goal is approved" | the transcript review's repairs; Q14 and Q15 put |
| review 4 | "q14 q15 agreed" | none: the transcript ends at this message |

The session put fifteen questions to the owner, Q1 to Q15, numbered in the conversation; their
labels are kept here. The search of the whole record ran once, between rounds 2 and 3, over a draft
of this milestone: 19 agents of `agent@knowledge-architect-record-searcher`, whose reports passed
`cargo klarch worklist --verify` and gave 301 entries a verdict other than `unrelated`. Of its
output, its `generated-lines.md` alone is committed, as
[search/discussion.txt](search/discussion.txt), one line per entry with its verdict and its action.
"What is already decided" carries the outcome of each verdict that changed something. The verdict
files, the draft the search read, the reports of single search agents and of the session's
investigators, and the goal reviewer's reports are not committed; a claim drawn from one of them is
cited only beside what a reader can check in the tree. An issue the commit that adds this document
closes, named here by its bare slug, sits in the history under crates/agent-skills/docs/open-issues/:
`git log --diff-filter=D --format=%H -1 -- <its path>` names the commit that deleted it, and
`git show <that commit>^:<its path>` prints it.

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
| Q1 to Q9 | the questions the session put to the owner during the discussion, each put in the session's reply of the round the table above gives and answered in the owner's next message, each restated below |
| Q10 to Q15 | the questions the session put to the owner at the review of this document: Q10 to Q13 on the goals home's definition of the record, ruled in the commit "[goals] The record of a project holds the entries of the kinds of entries it declares"; Q14 and Q15 on two attributions, D1's and that of `issue@knowledge-architect@the-changelog-holds-references-no-check-reads`, approved as the review reworded them |
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
  round 4, intending to handle it with the design of the changelog's model and checks, which the
  session identifies as `issue@knowledge-architect@a-mechanical-changelog-check`;
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

**The verdicts of the search whose outcome changed something.** The search gave 301 entries a
verdict other than `unrelated`, and [search/discussion.txt](search/discussion.txt) holds the line of
each, with its verdict and its action. Each verdict below changed the design, a thread or the work,
and is listed with that outcome; a verdict the file holds and this section does not name changed
nothing, the design already keeping within its entry, or the entry bearing on nothing the design
changes.

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
graded `touches`, since slice 1 has a slice's own document hold its section "What is already
decided", and with its two members the cluster `the-slice-audit-s-search-on-one-branch`.

**`trigger-met`, `rule-before-work`: 3 issues.**

- `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`: the draft argued from the
  "[review] Repairs … bundled heads split" commits, which is review cost. The session dropped that
  argument in round 3, and the case of
  `thread@split-guidelines-and-plan-directories@pre-harvest-conformance` stands on the record
  alone, so the trigger is not met. The issue stays open.
- `issue@knowledge-architect@the-changelog-holds-references-no-check-reads`: its re-entry point
  was the next Migration entry citing installed text, which slice 1 writes. On the owner's word in
  round 4, it becomes a `todo` with no re-entry point, in the commit that adds this document; the
  owner intends to handle it with the design of the changelog's model and checks, which the session
  identifies as `issue@knowledge-architect@a-mechanical-changelog-check`.
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

**`touches` and `worsens`: the issues that put a condition on a slice**, each named with the slice
that meets it:

- `issue@check@core-heads-bundle-decisions-with-many-citing-sites`: the evidence that splits left
  to whoever meets them sit undone; a conforming step slice 3 plans budgets the reading of every
  citing site, 6 to 39 per head by the issue's count.
- `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`: slice 1's text on specs,
  milestones and slices assumes neither, or names this issue at the site.
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`
  and `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`: slice 3's conforming step
  routes a misfiled head as the record settles, and sends the owner only the closed list.
- `issue@knowledge-architect@path-quickfixes-read-against-the-lints-on-ids` and
  `issue@knowledge-architect@path-quickfixes-read-against-the-renamed-checker`: slice 1's minimal
  migration keeps `spec@plans@path-quickfixes` resolving and its arguments readable, and takes over
  none of the readings those issues ask for.
- `issue@check@a-plain-hash-item-name-is-read-by-nothing`: slice 1 redraws which file of a plan
  holds an item, and states which `#<id>` names it reads.
- `issue@create@a-new-register-home-still-says-no-entry-yet-after-its-first-entry`: slice 1's change
  to `home_text` and `owed_documents` in `path@create@src/command.rs` adds no home text that turns
  false at its first entry.
- `issue@model@a-rust-item-named-in-prose-has-no-checked-form` (`worsens`): planned module splits
  move modules more often; each move-only step of slice 2's procedure searches the prose for the
  old path, a claim of slice 2.

**`constrains`: the entries whose sentence shapes the design**, and how:

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
- `goal@documents-render-as-a-linked-site`: the work adds a question to the design of
  `issue@knowledge-architect@the-documents-do-not-render-as-a-site`, whether a frozen file's
  references link to their targets as of the commit that froze it; see "Later consequences".

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
- **What `search/` holds**, on the session's recommendation, which the owner approved in review 2,
  ruling D1: of what `worklist` writes, only
  the generated lines, each as a `.txt` file named by its occasion: `generated-lines.md` of the
  discussion's search as `search/discussion.txt`, and, at a slice's or a spec's design audit,
  `fit-lines.md` as `search/design-audit.txt`, in the audit's commit, under the slice's own
  `search/`. Nothing else is committed: not the `verdicts-<subject>.md` files, `shown.md`,
  `gaps.md`, the agent directories, `agents.txt` or `work.txt`, the last three the searchers'
  working files, per `design@agent-skills@agents-write-in-scratch`. Being `.txt`, the walk never
  parses them, so they never break and need no rule of a frozen location. The searchers' reasons
  are not kept: a reason that changed something is carried by the outcome the `README.md` records.
- **What "What is already decided" keeps of the search**: each verdict whose outcome changed
  something, with that outcome, and a link to `search/discussion.txt` for the rest, never the full
  list of verdicts. A rule of the planning skill slice 1 writes, which this document's own section
  follows.
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
- **The P3 clause**, as D2 builds it: `cargo klarch commits` reports a commit that modifies a file
  under `harvested/`, or adds one other than by the move that freezes a slice, which adds each of
  the slice's files with the content its live path held in the commit's first parent; a commit may
  delete one. It applies to every commit, on the branch that wrote the file as on any later one.
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
  message where the discussion begins to the owner's last message before the branch that adds the
  plan document merges, trimmed of tool calls, is committed as the plan document's `transcript.txt`,
  after a step, named in the planning skill, that reads it and removes private information and
  pasted material. A claim of slice 1, per P4. The transcript ends there, rather than at the commit
  of the plan document, so that the rulings given at the plan document's reviews are in it: the
  session's decision at review 3, since this milestone's transcript would otherwise miss the
  rulings on D1 and on Q10 to Q13.
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

### A project's own kinds of entries, in an existing goal

For `thread@split-guidelines-and-plan-directories@custom-register-goal`. No slice builds it: the
goals home is its record, written on the branch that added this document, in the commit "[goals] A
project declares kinds of entries of its own to record the knowledge of its own domain".

- The purpose of a project's own kinds of entries is stated by rewording
  `goal@documentation-has-a-structure-and-a-model`: a project declares kinds of entries of its own
  to record the knowledge of its own domain that the procedures of the workflow do not cover.
- The phrase "from a variety of published shapes" is out of the goal, on the owner's Q7 answer in
  the closing exchange, so the goal admits one shape or several.
- The goal admits a new shape and requires none, on the owner's Q8 answer in the closing exchange.
- "Do not cover" takes reading A of Q9, the owner's answer in the closing exchange: a procedure
  covers a kind of knowledge only where it says how to record that kind.
- Nearest rival: a goal of its own. Defeated by rule 5 of
  `skill@knowledge-architect-goal-setting@goal-or-not`, a qualifier belongs to the goal it qualifies
  (`argument@split-guidelines-and-plan-directories@arg-51`).

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
  thread: lost to the session's recommendation of the generated lines only, which the owner
  approved in ruling D1, on the sizes "The plan document's directory" gives and the staleness of a
  report's quotations.
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

1. [Plan documents as directories, with model types of their own](plan-directories/README.md),
   `spec@split-guidelines-and-plan-directories@plan-directories`. Builds the plan model, the layout,
   the regime of each file, the planning skill's and the primer's text, and migrates
   `spec@plans@path-quickfixes`, minimally, and this milestone, properly. Fails alone on: the
   checker refusing the migrated tree, or a slice audit's `--work` still the whole milestone.
2. [Splits planned before the work](planned-splits/README.md),
   `spec@split-guidelines-and-plan-directories@planned-splits`. Builds the module guidelines inside
   the default code shape, the code-split scout, the record side at planning and the routing of a
   split found outside the work. Fails alone on: a plan written under the new text with no split step
   where a guideline's signal holds, or a scout's finding with no outcome.
3. [The heads a harvest touches, conformed before it](conformance-before-harvest/README.md),
   `spec@split-guidelines-and-plan-directories@conformance-before-harvest`. Builds the list of touched
   heads, the scoped auditor's dispatch at planning and the conforming step. Fails alone on: a
   harvest that conforms a head the list did not name.
4. [The head of a Component's existence argues each criterion](component-arguments/README.md),
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

D2 awaits the owner. Every default the discussion put to the owner was ruled by the owner's word on the
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

One default came from the review of slice 1's work:

- **D2**, awaiting the owner, on `thread@split-guidelines-and-plan-directories@frozen-plan-artifacts`:
  the P3 clause that D1 states admits "adding" a file under `harvested/`. An adversarial reviewer
  of slice 1's code reached three edits of the frozen record that pass under that wording: a
  freezing move that also edits the slice, a rename inside `harvested/` that edits, and a frozen
  file deleted in one commit and re-added with new content in the next. The last is the rewrite
  of history the P3 cause names. The default, built as written: `commits` admits an addition under
  `harvested/` only as the move that freezes a slice, each file added with the content its live
  path held in the commit's first parent; a modification and a type change stay reported, and a
  deletion stays admitted. The rival, D1's wording as written, keeps the three edits admitted.
  Written into "The regime of each file" and slice 1's spec.

## Harvest

Each slice's row is in its spec. The row of this document:

- **When the last slice lands**: the milestone's directory leaves whole, in a commit of its own,
  after the review before the merge, per `skill@knowledge-architect-planning@plan-document-leaves`.
- **The premortem's causes** P1, P5, P6, P7 and P8 are judged at the harvest of the slice the table
  names, by the standard of `skill@knowledge-architect-decision-recording@ruled-tripwires`; P2, P3
  and P4 are claims of slice 1, and slice 1's harvest row carries their outcomes into its commit
  message.
- **AC1** is reported on at each of slices 2, 3 and 4, and once more when this document leaves.
- **The design** of the first five subsections of "Decided design" lands in the heads each thread's
  item names, at the harvest of the slice that builds it; the sixth, the goal's, is in the goals
  home already, and no harvest records it.

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
