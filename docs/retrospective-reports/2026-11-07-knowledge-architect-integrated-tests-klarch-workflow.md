# 2026-11-07-knowledge-architect-integrated-tests-klarch-workflow

- **Received file**: 2026-11-07-knowledge-architect-integrated-tests-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-172-g5a956d02`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@model@string-literals-in-code-may-not-need-reading-at-all`.

## W1: the design audit asks the session to read the whole search output

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## W2 and C1: the audit's record of 365 graded entries in a commit message

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## C4: the record search takes its seeds by hand from a plan document

Analysed with W2 of 2026-11-02-knowledge-architect-development-and-components-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-development-and-components-klarch-workflow.md`.

## W3 and W4: a batch of three or fewer kept in the conversation, and an owner's rewording and the count

- **a.** Six rounds read a new Component's goals, which the owner reworded twice. W3: step 4 keeps
  each round's version "in the conversation for a batch of three goals or fewer"; each round's
  reviewer is fresh, three reported they could not check whether a sentence was unchanged, and the
  session then kept each version in a scratch file named in the briefs. Proposed: the record of a
  small batch is a scratch file the brief names; 1 changed. W4: after the owner reworded every goal,
  the session counted the next round as a first round and stopped at its second; a reviewer had to
  define three classes of wording. Blamed: "Stop when a round, from the second on, reports no
  `blocks` finding on old wording", and step 6, "A goal the owner rewords is reviewed again, with
  the whole batch". Proposed: an owner's rewording starts the count again; 0 added.
- **b.** Both apply at `5a956d02` and HEAD.
  `path@agent-skills@content/skills/goal-setting/SKILL.md`, line 127 to line 130: "Keep the version
  each round read, in the conversation for a batch of three goals or fewer and in the record for a
  larger one, and from the second round sort each finding by the words it quotes against it: on old
  wording, the words the previous round read and no repair changed since, or on new wording." Line
  157, step 6, as quoted. The sort is the session's act, in its own step: the reviewer's definition
  asks no sort (`path@agent-skills@content/agents/goal-reviewer.md`, the modes and the report hold
  none), and the session holds the conversation. W4's ambiguity is real: "no repair changed since"
  does not say whether the owner's rewording counts as a change, and "reviewed again" can read as a
  new review.
- **c.** `design@agent-skills@goal-review-stops-on-old-wording` states "The version each round read
  is kept in the conversation for a batch of three goals or fewer and in the record for a larger
  one", and argues old wording by "each round's repairs write new wording, in which the next round
  finds defects at least as readily". `rejected@agent-skills@goal-review-stop-over-all-wording`
  rejects a stop over all wording. W4's proposal, a restart, would make the first round after each
  rewording sort no wording, which the head's comparison sentence assumes only of the review's first
  round.
- **d.** W3: no defect of the text: the sort it asks is the session's, which reads the
  conversation; the reviewers' notes came from briefs that asked them to judge wording. W4: defect
  of the text, unclear.
- **e.** Installed text, `agent-skills`; serves `goal@agent-skills@goals-state-the-owner-s-intent`.
  W3's proposal contradicts the head's sentence on where the versions are kept. W4's fix below is a
  member the head's argument covers: an owner's rewording writes new wording as a repair does.
- **f.** W4: nearest rival is the finding's restart. It costs one round per rewording, since a
  restarted count cannot stop before its second round, and nothing argues that words the owner
  wrote are read differently from words a repair wrote. What decides: the head's own reason for
  counting old wording. W3: rival is the scratch file; it adds a file and a brief item for an act
  the session already performs from the conversation, and contradicts a head with no failure shown.
- **g.** W4: a text edit whose design is settled, the member recorded directly per
  `primer@design-heads`. W3: nothing.
- **h.** W4: line 130 becomes "on old wording, the words the previous round read and no repair or
  rewording of the owner changed since"; step 6, "is reviewed again, with the whole batch, in the
  next round of the same review". The head's definition of old wording follows, rewritten in place.
  Instructions: 0 added, 2 reworded. A `Next release` Workflow entry. W3: no change.
- **Default**: W4 handle now; W3 no change.
- **Outcome**: decided by the session: W4 handle now; W3 no change, the sort being the session's act
  from the conversation it holds. Ground: `design@agent-skills@goal-review-stops-on-old-wording`,
  whose reason for counting old wording covers the owner's rewording; the reviewer's definition asks
  no sort.

## W6: the review's invariants multiply full build directories, one per reviewer

Analysed with C2 of 2026-10-31-knowledge-architect-transcript-issues-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-transcript-issues-klarch-workflow.md`.

## C2: a test outside the checker's compiled directories turns its rule-shaped literals into findings

- **a.** What it says: test files moved to a crate the binary does not link; their literals gave at
  least 50 findings; the repair was a `[walk] exclude` row, which also leaves their comments
  unchecked. The owner doubts that a string literal should be read at all. "The project records the
  question as a design issue."
- **c.** Whether it is already recorded: yes, `issue@model@string-literals-in-code-may-not-need-reading-at-all`,
  kind `design`, which holds the counts per file (17, 7 and 26), the exclusion, its cost to the
  comments, the owner's report and its re-entry, "A design discussion the owner opens on it".
- **Default**: no change.
- **Outcome**: decided by the session: no change, already recorded with this instance. Ground: the
  issue's What is this file's case.

## C3: `fork-context-ref` and unlabelled lines

Analysed with C2 of 2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow.md`; its unlabelled lines with C3 of 2026-11-04-knowledge-architect-check-renamed-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-04-knowledge-architect-check-renamed-klarch-workflow.md`.

## W5: the shared test Component's shape does not say how its tests reach a binary of a crate it tests

- **a.** What it says: `skill@knowledge-architect-component-creation`, the shared test Component, "a
  Component that holds the integrated tests crossing several Components … and depends on them".
  Cargo sets `CARGO_BIN_EXE_<name>` in the binary's own package alone; a dev-dependency does not
  build the binary; an artifact dependency needs `-Z bindeps`. The plan assumed the binary's tests
  stay in its crate; moving them took a cargo started by the test. Missing: the fact, for an
  adopting project, which "The grounding file of the shape could hold … beside its other two Cargo
  facts".
- **b.** Whether it still applies: the installed text is unchanged on the point:
  `path@agent-skills@content/skills/component-creation/SKILL.md`, lines 108 to 117, names the
  dependency direction and the dev-dependency cycle, and nothing of a binary. The grounding file
  `path@agent-skills@docs/grounding/crate-boundaries.md` holds Cargo's rules on cycles, and nothing
  of `CARGO_BIN_EXE_<name>`.
- **c.** Whether it is already recorded: this repository holds the fact, measured:
  `design@testing@nested-cargo-builds-the-binary`, "`CARGO_BIN_EXE_klarch` is set for the binary's
  own package alone, a dev-dependency does not build its binary, and an artifact dependency requires
  `-Z bindeps`, each read by a test in a workspace of two packages".
- **d.** Its kind: missing at most, for an adopting project that places a binary's tests in such a
  Component; one instance, this repository's.
- **e.** Its scope: the installed text; `design@agent-skills@shared-test-component`. Adding the fact
  reverses nothing.
- **f.** A better fix: the nearest rival is the fact added to the grounding file and one clause to
  the shape. What decides for no change: the fact surfaces at the first build, since a test that
  reads `CARGO_BIN_EXE_<name>` with `env!` in another package does not compile (Cargo's documented
  behaviour of `env!`, an assumption not re-run here); the cost here was a plan's assumption, which
  the plan's own reading caught, and the stale-binary bug was this repository's code, which its
  review caught.
- **g.** Its route: nothing.
- **h.** The proposal: no change; the fact stands in this repository's head where its code needs it.
- **Default**: no change.
- **Outcome**: decided by the session: no change. Ground: one instance, the fact is fixed by Cargo
  and reported by the compiler at the first build, and this repository records it in
  `design@testing@nested-cargo-builds-the-binary`.

## The standing answers

- **a.** Items:
  - Installed file: the setup skill's pointer, at its source. An observation.
  - "Changes no rule" told to the owner unchecked: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - One item listed as "named and not routed", an outcome `primer@met-outside-the-task` does not
    give; the transcript reviewer caught it and it became an issue. Analysed here.
  - Hand tasks: the seeds are C4 of this file, cluster A2 (G1); the split and the assembly of 365
    lines are W1 of this file, cluster A (G1), and its W2 with C1 cluster B (G1); the reword is
    cluster H (G6). The comparison of old and new test names by a script over the test attributes
    of eight files is analysed here.
  - Rules on code: the method published on argument is the rule working; W5 and C2 are analysed by
    G4.
- **b.** The outcome list is closed at HEAD, `path@agent-skills@content/PRIMER.md`, lines 245 to
  247: "fixed (with the commit), decided ..., issue opened (with its id), kept for the
  retrospective's workflow file, waiting for the owner's ruling, or dropped (with the reason)".
- **d.** A lapse of a clear instruction, caught before the merge; and an observation: the name
  comparison is a one-off check of a code move, which the development skill's claims ask the
  session to show, and no workflow step prescribes the script.
- **Default**: no change.
- **Outcome**: decided by the session: no change, a lapse and an observation. Ground: the lapse
  rule of `skill@knowledge-architect-retrospective-intake@admission-rules`.

---
