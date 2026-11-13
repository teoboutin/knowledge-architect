# Splits planned before the work: module guidelines, the code-split scout, and the record side at planning

The second slice of `milestone@plans@split-guidelines-and-plan-directories`. When it lands, it is
moved under its milestone's `harvested/` and stays there until the milestone leaves, under the rule
slice 1 builds. The threads, arguments and decided design it shares with the other
slices are in the milestone document; those only this slice uses are below, and its own threads
and arguments in [its deliberation](deliberation.md).

## Builds

- **The module guidelines**, a fragment of their own, `path@agent-skills@fragments/module-split.md`,
  on the owner's Q16 word, `thread@split-guidelines-and-plan-directories@module-guidelines-fragment`,
  which replaced Q5's delivery inside the fragment of the default code shape: the criterion, the
  signals for and against a split, the one role of size, and the move before the work, per this
  spec's "The module guidelines". Its hosts are the development skill,
  `agent@knowledge-architect-code-shape-reviewer` and the code-split scout: one new fragment and its
  host list. Worded with no language's terms. The fragment of the default code shape keeps its text
  and its hosts.
- **The code-split scout**, a new installed agent, `agent@knowledge-architect-code-split-scout`, whose
  source is a new file of `path@agent-skills@content/agents/`, hosting the module guidelines'
  fragment and `path@agent-skills@fragments/when-to-split.md`, on the owner's Q16 word. It reads a
  draft of a plan document and the code the draft touches, and reports, per module or Component, each
  signal of the module guidelines or of the component-creation skill that the work pushes it past,
  and each signal against a split that holds. It writes only in the scratch directory its brief
  names, and returns its report; the report is never committed, since a plan document's directory
  holds no such file, per `design@model@plan-document-directory`.
- **The design skill**: its grounding dispatches the code-split scout once the draft of a plan
  document exists, that is where the session expects the full path, and before the search of the
  whole record, so that each split thread the scout opens is in the draft the search reads; each
  split the scout reports that the work bears on is a thread of the discussion, and one outside the
  work's area is routed by `primer@met-outside-the-task`. A split thread opened after the search gets
  a search of its own, as the skill already asks of any thread added after it. The in-change path and
  bounded work dispatch no scout.
- **The planning skill**:
  - at planning, `cargo klarch worklist --out <dir>`, `<dir>` a new directory outside the project,
    with `--work` the `README.md` files the slices' audits will take, since the notice's threshold is
    the bound less the words of the work, prints its notice of every cluster above that threshold
    across the whole record; the session keeps, of the notices on clusters rooted at a design
    subject, those the harvest writes into, the subjects of the heads its harvest rows name, and
    leaves a remainder's and a goals group's notice aside; each such subject, and each subject whose
    heads will answer questions about parts that change apart, is split in a step, or, in a
    milestone, a slice when the move is large, before the harvest that writes into it;
  - a code split the discussion decided is a step, or, in a milestone, a slice, before the work that
    needs it, never left to the implementer, and never an issue; its move lands in commits that hold
    the move alone, each of which builds and passes the project's checks before it is pushed, since a
    commit before a branch's tip may be built by no other run; the text names neither a pull request
    nor continuous integration, per
    `issue@agent-skills@installed-text-assumes-pull-requests-and-ci`;
  - the design audit of each slice, and of a spec where one runs, runs the code-split scout and the
    `worklist` notice again over its area, and each split they find is a gap of the audit, sorted as
    point 2 of `skill@knowledge-architect-planning@working-a-slice` sorts any gap; its outcome is
    listed in the audit's commit message, with the other gaps.
- **The decision-recording and development skills, and the subject standard's fragment**, where they
  state who splits a subject or a module and when: a subject split stays a move that decides nothing,
  now planned before the harvest where planning finds it, and a fix in a commit of its own where a
  session meets it outside planned work.
- **The code-shape reviewer**, `agent@knowledge-architect-code-shape-reviewer`, which hosts the
  default code shape and the module guidelines' fragment: where a branch's diff meets a signal for a
  module split, it reports it, and the session routes it to a planned step or by
  `primer@met-outside-the-task`, never to the implementer as work inside the task.
- **The record**: the heads of Group D this slice rewrites, and the hosts of
  `design@agent-skills@shared-text-fragment`, below, with every text that cites them. The hosts of
  the fragments are named in five places, each updated: the test list `FRAGMENT_HOSTS` in
  `path@agent-skills@src/lib.rs`, a row for the new fragment and the scout added to the row of
  `path@agent-skills@fragments/when-to-split.md`; the `%%` comment at the head of each fragment;
  the prose of `design@agent-skills@default-code-shape`; the sentence of
  `design@agent-skills@component-skill-delivery` that has the design and development skills host the
  fragment on when to split as the two activities of its first two moments; and the sentence of
  `design@agent-skills@shared-text-fragment` that lists the fragments and the extension of that one
  to the development skill.
- **CHANGELOG.md**: the entries the change owes, in the `Next release` section.

## Claims

1. **The guidelines are worded with no language's terms.** The installed default code shape names no
   construct of one language in the module guidelines: read by the agent-text review against
   `design@agent-skills@the-workflow-teaches-no-rust-idiom`.
2. **Size has one role.** The guidelines state size only as a file a session cannot read whole in one
   pass, and state no number of lines; the assumption the session labelled in round 3, that this
   limit is a signal, is measured first: the Read tool's limit, by reading a file above it, with the
   measurement in the commit, or the sentence leaves.
3. **The scout hosts what the owner ruled.** The installed `agent@knowledge-architect-code-split-scout`
   holds the text of the module guidelines' fragment and of the fragment on when to split, and not
   that of the default code shape; the build's host lists name it for both:
   `cargo x skill knowledge-architect-code-split-scout` shows both placeholders.
4. **A split found outside the work has an outcome.** The design skill's text routes a split the
   scout reports outside the work's area by `primer@met-outside-the-task`, and a split inside it as a
   thread: read in the installed text.
5. **The record side is a command.** The planning skill names `worklist --out` as the record side at
   planning, and dispatches no agent for it.
6. **A move-only step leaves no stale path.** The planning skill's move-only step searches the prose
   of the tree for each old module path the move changes, per
   `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, and states that the search is a lead,
   since a path split over two lines escapes it; it re-points no quotation and edits no frozen file,
   since no command performs a move, per
   `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`.
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
- `design@agent-skills@shared-text-fragment`: the module guidelines' fragment joins the list, hosted
  by the development skill, the code-shape reviewer and the code-split scout, and the fragment on when
  to split gains the scout as a host, on the owner's Q16 word; the harvest argues again, on the
  `claude` ask, the head's rival of a harness hook delivering pointed sections, since the rewrite meets
  the re-entry point of `issue@agent-skills@heads-argued-from-another-harness`.
- `design@agent-skills@needless-instruction`, which cites
  `design@agent-skills@premortem-as-watch-points`: its list of the rules whose cost goes to the
  standing question, "code, tests or the split into Components", gains the module guidelines and
  the subject standard as the question does, found at this slice's design audit.

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
  module the work touches and the signals against a split do not outweigh it.
- A split the code-split scout reports that has no thread, no step and no outcome of
  `primer@met-outside-the-task`.

## Premises that expire

- The design audit of slices 3 and 4 runs under this slice's text: the scout and the `worklist` notice
  over their areas. Nothing ends that within the milestone.

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
- **A split is a move before the work, in commits that hold the move alone**, so that the review of
  the work reads the work.
- **The delivery**, on the owner's Q16 word at this slice's design audit, which replaced Q5's of
  round 4: the module guidelines a fragment of their own, hosted by the development skill, the
  code-shape reviewer and the code-split scout, and the scout a new installed agent hosting that
  fragment and the fragment on when to split. The scout does not host the default code shape, whose
  test rules it applies none of, per `design@agent-skills@shared-text-fragment`'s rule that a host
  applies the whole fragment;
  `thread@split-guidelines-and-plan-directories@module-guidelines-fragment`.
- These are guidelines a project weighs, per the milestone document's "Guidelines, and what stays a
  rule".
- Nearest rival: a size threshold as the criterion, thaum's. Defeated by the owner's
  `argument@split-guidelines-and-plan-directories@arg-4`, and by thaum's own record: its threshold has
  no recorded basis, and its open issue on an event vocabulary of 999 lines in one item shows a size
  a move cannot fix, per the session's investigator, a subagent of round 1 whose report no file
  keeps; this slice's design audit reads thaum's tree again for both facts.

### When a split is found and planned

For `thread@split-guidelines-and-plan-directories@splits-planned-before-work`.

- **The code side, a design decision**: the code-split scout at the design grounding, once the draft
  of a plan document exists and before the search of the whole record, so that the threads it opens
  are in the work the search reads; each split it reports that the work bears on is a thread of the
  discussion. Placed before the search at this slice's design audit, from the spec's "beside the
  search", on `design@agent-skills@search-runs-once-the-shapes-settle` and
  `issue@agent-skills@a-record-search-misses-what-is-added-after-it`.
- **The record side, no decision**: `worklist --out` at planning, with the work its audits will
  take, over the subjects the harvest writes into; a subject above the bound, or one whose heads will answer parts that change apart, is
  split before the harvest.
- **Each split the plan takes is a step, or a slice when the move is large, before the work or the
  harvest that needs it.** Never left to the implementer, never an issue.
- **A split a scout reports outside the work's area** is routed by `primer@met-outside-the-task`,
  usually as an issue: the plan does not take work its discussion did not decide.
- **Each slice's design audit**, and a spec's where one runs, runs the scout and the notice again
  over its area, since the code moves between slices and between a spec's writing and its work.
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

- **Attribution**: Q5 and Q16 approved the session's defaults; a head records the delivery as the
  session's choice that the owner approved, in the weak form, never as the owner's ruling, per
  `design@agent-skills@fork-choice-in-weak-form`.
- **Design heads**: Group D's heads of this slice and `design@agent-skills@shared-text-fragment`,
  as "What is already decided" says, under `skill@knowledge-architect-decision-recording`; the decisions of
  `thread@split-guidelines-and-plan-directories@module-split-guidelines`,
  `thread@split-guidelines-and-plan-directories@splits-planned-before-work` and
  `thread@split-guidelines-and-plan-directories@in-crate-split-unrecorded` land there, or in a head of
  their own where the entry tests give one, the module guidelines and the rule that a module split is
  planned before the work being two decisions with two nearest rivals, judged by the one-decision
  test; the part of
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
