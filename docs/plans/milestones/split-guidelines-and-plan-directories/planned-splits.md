# Splits planned before the work: module guidelines, the code-split scout, and the record side at planning

The second slice of `milestone@plans@split-guidelines-and-plan-directories`. When it lands, it is
moved under its milestone's `harvested/` and stays there until the milestone leaves, under the rule
slice 1 builds. The threads, arguments and decided design it shares with the other
slices are in the milestone document; those only this slice uses are below.

## Builds

- **The module guidelines**, inside the fragment of the default code shape,
  `path@agent-skills@fragments/default-code-shape.md`, so its hosts stay the same, on the owner's Q5
  word: the criterion, the signals for and against a split, the one role of size, and the move-only
  commit, per this spec's "The module guidelines". Worded with no language's terms.
- **The code-split scout**, a new installed agent, `knowledge-architect-code-split-scout`, whose
  source is a new file of `path@agent-skills@content/agents/`, hosting the fragments of the default
  code shape and `path@agent-skills@fragments/when-to-split.md`, on the owner's Q5 word: two new host
  entries. It reads the draft of a discussion on its full path and the code the draft touches, and
  reports, per module or Component, each signal of the module guidelines or of the component-creation
  skill that the work pushes it past, and each signal against a split that holds.
- **The design skill**: its grounding, on the full path, dispatches the code-split scout once the
  draft exists, beside the search; each split the scout reports that the work bears on is a thread of
  the discussion; one outside the work's area is routed by `primer@met-outside-the-task`.
- **The planning skill**:
  - at planning, `cargo klarch worklist --out <dir>` prints its notice of every cluster above the
    bound across the whole record, since `--subject` names subjects of the work, not design
    subjects; the session reads the notice and keeps the design subjects the harvest writes into,
    those of the heads its harvest rows name; each such subject, and each subject whose
    heads will answer questions about parts that change apart, is split in a step, or a slice when
    the move is large, before the harvest that writes into it;
  - a code split the discussion decided is a move-only step, or slice, before the work that needs it,
    never left to the implementer, and never an issue;
  - the design audit of each slice runs the code-split scout and the `worklist` notice again over the
    slice's area, and each split they find is a gap of the audit, sorted as
    `skill@knowledge-architect-planning@working-a-slice` point 2 sorts any gap.
- **The decision-recording and development skills, and the subject standard's fragment**, where they
  state who splits a subject or a module and when: a subject split stays a move that decides nothing,
  now planned before the harvest where planning finds it, and a fix in a commit of its own where a
  session meets it outside planned work.
- **The code-shape reviewer**, `agent@knowledge-architect-code-shape-reviewer`, which hosts the
  default code shape and so the module guidelines: where a branch's diff meets a signal for a
  module split, it reports it, and the session routes it to a planned step or by
  `primer@met-outside-the-task`, never to the implementer as work inside the task.
- **The record**: the heads of Group D this slice rewrites, and the hosts of
  `design@agent-skills@shared-text-fragment`, below, with every text that cites them. The hosts of
  the two fragments are named in three places, each updated: the test list `FRAGMENT_HOSTS` in
  `path@agent-skills@src/lib.rs`, the `%%` comment at the head of
  `path@agent-skills@fragments/default-code-shape.md`, and the prose of
  `design@agent-skills@default-code-shape`.
- **CHANGELOG.md**: the entries the change owes, in the `Next release` section.

## Claims

1. **The guidelines are worded with no language's terms.** The installed default code shape names no
   construct of one language in the module guidelines: read by the agent-text review against
   `design@agent-skills@the-workflow-teaches-no-rust-idiom`.
2. **Size has one role.** The guidelines state size only as a file a session cannot read whole in one
   pass, and state no number of lines; the assumption the session labelled in round 3, that this
   limit is a signal, is measured first: the Read tool's limit, by reading a file above it, with the
   measurement in the commit, or the sentence leaves.
3. **The scout hosts what the owner ruled.** The installed `knowledge-architect-code-split-scout`
   holds the text of the default code shape and of the fragment on when to split, and the build's host
   lists name it for both: `cargo x skill knowledge-architect-code-split-scout` shows both
   placeholders.
4. **A split found outside the work has an outcome.** The design skill's text routes a split the
   scout reports outside the work's area by `primer@met-outside-the-task`, and a split inside it as a
   thread: read in the installed text.
5. **The record side is a command.** The planning skill names `worklist --out` as the record side at
   planning, and dispatches no agent for it.
6. **A move-only step leaves no stale path.** The planning skill's move-only step searches the prose
   of the tree for each old module path the move changes, per
   `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, and states that the search is a lead,
   since a path split over two lines escapes it.
7. **Each dispatch fixed at a step names its property.** The code-split scout's dispatch names the
   property of `design@agent-skills@work-divided-by-judgement` it rests on, a point of view apart.

## What is already decided

The seeds of this slice's audit, the decisions it rewrites, with every text `cargo klarch show` lists
as referencing each, on the commit before the milestone document's. Each text is judged by this
slice's harvest, as slice 1's spec says of its own.

**Group D, and `design@agent-skills@shared-text-fragment` of Group F.**

- `design@agent-skills@subject-split-standard`:
  `path@agent-skills@content/skills/component-creation/SKILL.md`,
  `path@agent-skills@docs/design/audit-record-axes.md`.
- `design@agent-skills@default-code-shape`: `path@agent-skills@docs/design/components.md`,
  `path@agent-skills@docs/design/development.md` (3 sites),
  `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/design/reviews.md`,
  `path@agent-skills@docs/design/shipped-set.md`, `register@agent-skills@tripwire` (2 sites),
  `path@agent-skills@fragments/default-code-shape.md`,
  `issue@knowledge-architect@a-typed-command-prints-and-exits-rather-than-returning-data`.
- `design@agent-skills@component-skill-delivery`:
  `path@agent-skills@content/skills/component-creation/SKILL.md` (2 sites),
  `path@agent-skills@docs/design/components.md` (2 sites),
  `path@agent-skills@docs/design/development.md`, `path@agent-skills@docs/design/retrospective.md`,
  `path@agent-skills@docs/design/shipped-set.md` (2 sites),
  `path@agent-skills@fragments/when-to-split.md`.
- `design@agent-skills@premortem-as-watch-points`:
  `path@agent-skills@content/skills/retrospective/SKILL.md` (3 sites),
  `path@agent-skills@docs/design/components.md`, `path@agent-skills@docs/design/development.md`,
  `path@agent-skills@docs/design/dispatching.md` (2 sites),
  `path@agent-skills@docs/design/retrospective.md` (2 sites),
  `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`,
  `issue@agent-skills@audit-axes-beyond-the-design-record`,
  `issue@agent-skills@patching-an-installed-skill`.
- `design@agent-skills@design-audit-checks-fit`:
  `path@agent-skills@content/skills/planning/SKILL.md`,
  `path@agent-skills@docs/design/record-search.md` (2 sites).
- `design@agent-skills@shared-text-fragment`: `path@agent-skills@CLAUDE.md`,
  `path@agent-skills@docs/design/components.md`, `path@agent-skills@docs/design/goals.md`,
  `path@agent-skills@docs/design/how-installed-text-changes.md` (2 sites),
  `path@agent-skills@docs/design/project-configuration.md` (2 sites),
  `path@agent-skills@docs/design/retrospective.md`, `path@agent-skills@docs/design/shipped-set.md`,
  `path@agent-skills@docs/design/what-earns-a-record.md` (2 sites),
  `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`
  (2 sites), `issue@agent-skills@heads-argued-from-another-harness`,
  `path@agent-skills@fragments/default-code-shape.md`,
  `path@agent-skills@fragments/finding-outcomes.md`, `path@agent-skills@fragments/goal-standard.md`,
  `path@agent-skills@fragments/losing-alternatives.md`,
  `path@agent-skills@fragments/owning-component.md`,
  `path@agent-skills@fragments/retrospective-defect-kinds.md`,
  `path@agent-skills@fragments/retrospective-two-files.md`,
  `path@agent-skills@fragments/reversal-check.md`,
  `path@agent-skills@fragments/skill-self-sufficient.md`,
  `path@agent-skills@fragments/three-homes.md`, `path@agent-skills@fragments/trigger-test.md`,
  `path@agent-skills@fragments/when-to-split.md`, `path@agent-skills@src/lib.rs`,
  `issue@knowledge-architect@the-skill-bundling-passes-are-tied-to-this-project-s-skills`.

What each is rewritten to state:

- `design@agent-skills@subject-split-standard`: a subject split found at planning is a step before the
  harvest; "a fix in a commit of its own" stays for a split met outside planned work. The bound stays
  a trigger on its ground of cost.
- `design@agent-skills@default-code-shape`: the module guidelines join the shape, and a module split
  the discussion decided is planned before the work, beyond the lines a change writes; the guideline
  status stays.
- `design@agent-skills@component-skill-delivery`: the code-split scout's report at the grounding, and
  its run at each slice's audit, are moments where a Component split is found; the head, the skill's
  description and the primer's list of installed skills are rewritten together. Slice 4 reads it
  again for how the criteria are applied.
- `design@agent-skills@premortem-as-watch-points`: the standing question on an installed rule about
  code or splitting into Components gains the module guidelines and the subject standard.
- `design@agent-skills@design-audit-checks-fit`: a split found at a slice's audit is a gap the audit
  sorts, not a design act of the audit: a split the decided design implies is applied in place, and
  any other goes to the owner or to a design session, as the audit's sorts say.
- `design@agent-skills@shared-text-fragment`: the default code shape and the fragment on when to split
  each gain the code-split scout as a host, on the owner's Q5 word.

**Bound and not rewritten**: `design@agent-skills@work-divided-by-judgement`, of Group F,
unchanged in its rule, which the planning and design skills' new dispatch cites with its property;
`design@agent-skills@one-decision-per-head`, Group E;
`design@agent-skills@component-split-signals`; `design@agent-skills@the-workflow-teaches-no-rust-idiom`;
`design@query@worklist-bound-is-40000-words`; `design@git@git-is-asked-through-one-module`;
`design@xtask@ci-builds-the-tip-alone`; `rejected@agent-skills@one-generic-auditor-agent`;
`rejected@agent-skills@module-directory-component`;
`design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`; and
`design@agent-skills@behaviour-additions-need-real-use`, met by the owner's report of round 4,
`argument@split-guidelines-and-plan-directories@arg-50`.

## Audit subjects

- The fragments `path@agent-skills@fragments/default-code-shape.md`,
  `path@agent-skills@fragments/when-to-split.md` and `path@agent-skills@fragments/subject-split.md`,
  and every text that hosts them.
- The sources of the design skill, the planning skill, the development skill, the decision-recording
  skill and the component-creation skill, and the agents of `path@agent-skills@content/agents/`, for
  the shape of a new one.
- The design homes `path@agent-skills@docs/design/components.md`,
  `path@agent-skills@docs/design/development.md`, `path@agent-skills@docs/design/dispatching.md` and
  `path@agent-skills@docs/design/record-search.md`.
- The standing entries the slice bears on: `tripwire@agent-skills@shape-findings-fragment-code`,
  which watches the code side of P1; `issue@model@a-rust-item-named-in-prose-has-no-checked-form`;
  `issue@check@the-cost-of-a-relocation-is-bounded-by-no-argued-head`;
  `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`;
  `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`.
- thaum's development skill and its head on a module per review scope, in the tree of thaum, the
  owner's sibling project, which the owner names the location of, as the grounding of the
  criterion; a finding read there goes to `path@agent-skills@docs/grounding/`, per the root
  `CLAUDE.md`.
- `agent@knowledge-architect-code-shape-reviewer`'s source, for what it does with a signal of the
  module guidelines a branch's diff meets.

## Fails alone on

- A plan written under the new text with no split step where a signal of the guidelines holds for a
  module the work touches.
- A split the code-split scout reports that has no thread, no step and no outcome of
  `primer@met-outside-the-task`.

## Premises that expire

- The design audit of slices 3 and 4 runs under this slice's text: the scout and the `worklist` notice
  over their areas. Nothing ends that within the milestone.

## Threads

### A split inside a crate is decided and planned at design time, and earns no design head unless a test of the head rules holds `##in-crate-split-unrecorded`

- Proposed by the owner in round 1, `argument@split-guidelines-and-plan-directories@arg-2` and
  `argument@split-guidelines-and-plan-directories@arg-6`; the session's reading of the owner's words,
  `argument@split-guidelines-and-plan-directories@arg-23`, offered in round 1 for correction.
- Final state: `approved`. It stood `presumed-settled` in the checkpoint table as the session's
  reading; the owner's word on the checkpoint table, quoted in the milestone document's Threads,
  closed it. In round 3 the session noted that `design@git@git-is-asked-through-one-module` shows a
  module decision that does earn a head, by test 2 of `primer@design-heads`.
- Its shape: this spec's "Decided design", "A module split's record". Harvest: this slice's, into
  `design@agent-skills@default-code-shape` and the planning skill's text; no head of its own unless
  the entry tests give one.

## Arguments

### A split within a crate needs no decision record `##arg-2`

Round 1, the owner. Bears on `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`.
"in most cases, splitting within one crate should not need decision record, it does not directly
affect what is decided or implemented"

### A module split is decided at design time and earns no head unless a head test holds `##arg-23`

Round 1, the session. Bears on `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`.
"My reading of your words: a module split is *decided at design time*, written in the plan document as
a step of its own, and *earns no head* unless one of the tests in `primer@design-heads` holds. Its
reason goes in a comment at the module and in the message of the move commit. In this repository a
module split never passes test 1, since the facade in `design@knowledge-architect@api-facade` keeps
public paths unchanged. That reconciles "a split is a design decision" with "it needs no record".
Correct me if you meant something else." The claim on test 1 is imprecise: the facade of
`design@knowledge-architect@api-facade` covers `check`, `cli`, `model`, `query`, `transcript`,
`create` and `testing`, and `gates` declares `pub mod process` in `path@gates@src/lib.rs`, so a
split of that module changes a public path. "A module split's record" below states the case
behind the facade only.

## Decided design

### The module guidelines

For `thread@split-guidelines-and-plan-directories@module-split-guidelines`.

- **The criterion**: a module holds what one review reads against one reference. A reference is a
  design head, a section of a specification, an invariant set, or one external effect, as
  `design@git@git-is-asked-through-one-module` makes one module the owner of one effect.
- **Signals for a split**, each with the failure it prevents:

  | signal | the failure it prevents |
  | --- | --- |
  | a second reference enters the module | a review that reads code outside its subject |
  | two parts change apart, counting the changes the goals predict | unrelated diffs in one file |
  | an invariant can only be protected by a visibility boundary | a helper widened to the whole crate |

- **Signals against a split**: two parts every change reads together, such as a process and its
  mutations, which a split fragments into one review across two files; one item large by itself, such
  as an enumeration, which moving code does not divide, since dividing it is a design change; and one
  owner per external effect, which a split that spreads the effect over two modules breaks.
- **Size has one role**: a file a session cannot read whole in one pass, the argument the
  decision-recording skill already uses for turning a design home into a directory. No number of
  lines is picked. The premise that this limit is a signal is measured first, Claim 2.
- **A split is a move-only commit before the work**, so that the review of the work reads the work.
- These are guidelines a project weighs, per the milestone document's "Guidelines, and what stays a
  rule".
- Nearest rival: a size threshold as the criterion, thaum's. Defeated by the owner's
  `argument@split-guidelines-and-plan-directories@arg-4`, and by thaum's own record: its threshold has
  no recorded basis, and its open issue on an event vocabulary of 999 lines in one item shows a size
  a move cannot fix, per the session's investigator, a subagent of round 1 whose report no file
  keeps; this slice's design audit reads thaum's tree again for both facts.

### When a split is found and planned

For `thread@split-guidelines-and-plan-directories@splits-planned-before-work`.

- **The code side, a design decision**: the code-split scout at the design grounding, on the full
  path, once the draft exists; each split it reports that the work bears on is a thread of the
  discussion.
- **The record side, no decision**: `worklist --out` at planning, over the subjects the harvest
  writes into; a subject above the bound, or one whose heads will answer parts that change apart, is
  split before the harvest.
- **Each split the plan takes is a step, or a slice when the move is large, before the work or the
  harvest that needs it.** Never left to the implementer, never an issue.
- **A split a scout reports outside the work's area** is routed by `primer@met-outside-the-task`,
  usually as an issue: the plan does not take work its discussion did not decide.
- **Each slice's design audit** runs the scout and the notice again over the slice's area, since the
  code moves between slices.
- Nearest rivals: a record scout, an agent, which lost to a command needing no point of view apart;
  and a split left to the implementer, today's practice, which the owner reported never done,
  `argument@split-guidelines-and-plan-directories@arg-7` and
  `argument@split-guidelines-and-plan-directories@arg-50`.

### A module split's record

For `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`.

- A module split is decided at design time and planned as a step; it earns no design head unless a
  test of `primer@design-heads` holds, as test 2 does for
  `design@git@git-is-asked-through-one-module`. Its reason goes in a comment at the module and in the
  message of the move commit.
- Nearest rival: a head for every module split. Defeated by the owner's
  `argument@split-guidelines-and-plan-directories@arg-2`, and by `primer@design-heads`, whose tests a
  module split behind this repository's facade, `design@knowledge-architect@api-facade`, does not meet
  by test 1.

## Losing alternatives

- **A size threshold as the criterion of a module split**, under
  `thread@split-guidelines-and-plan-directories@module-split-guidelines`: lost to the owner's
  `argument@split-guidelines-and-plan-directories@arg-4` and to its lack of a recorded basis in thaum.
- **A head for every module split**, under
  `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded`: lost to
  `argument@split-guidelines-and-plan-directories@arg-2` and `primer@design-heads`.
- **The module guidelines and the scout parked until a real session shows the need**, the session's
  default at Q4: lost to the owner's report of round 4, the real-use evidence,
  `argument@split-guidelines-and-plan-directories@arg-50`.

## Implementation sequence

1. **The module guidelines** in the default code shape's fragment: Claims 1 and 2.
2. **The code-split scout**, its source and its two hosted fragments: Claim 3.
3. **The design skill's grounding** and **the planning skill's steps** and audit: Claims 4 to 7.
4. **The record and the changelog**: the heads above, with their citing texts, and the CHANGELOG.md
   entries. This is the harvest.

## Harvest

- **Design heads**: Group D's heads of this slice and `design@agent-skills@shared-text-fragment`,
  as "What is already decided" says, under `skill@knowledge-architect-decision-recording`; the decisions of
  `thread@split-guidelines-and-plan-directories@module-split-guidelines`,
  `thread@split-guidelines-and-plan-directories@splits-planned-before-work` and
  `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded` land there, or in a head of
  their own where the entry tests give one; the part of
  `thread@split-guidelines-and-plan-directories@split-guidelines-not-rules` this slice builds lands in
  `design@agent-skills@subject-split-standard` and `design@agent-skills@default-code-shape`.
- **Losing alternatives**: those above, judged by the tests of
  `skill@knowledge-architect-decision-recording@losing-alternatives`. A lead from the review of the
  plan: a size threshold as the criterion of a module split lost in part to a survey of thaum's
  record, outside the project, which the harvest weighs under test 3.
- **The premortem's causes**: P1 and P8, judged by the standard of
  `skill@knowledge-architect-decision-recording@ruled-tripwires`, P1 folded into
  `tripwire@agent-skills@shape-findings-fragment-code` where that entry guards the same premise.
- **AC1**, reported at this slice's design audit.
