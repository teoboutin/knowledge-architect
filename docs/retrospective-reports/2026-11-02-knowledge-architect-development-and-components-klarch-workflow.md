# 2026-11-02-knowledge-architect-development-and-components-klarch-workflow

- **Received file**: 2026-11-02-knowledge-architect-development-and-components-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: built from its own source, main at `v0.9.0-20-g5d23ee23`.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`, `issue@agent-skills@the-retrospective-counts-no-review-cost`.

## W1 and C3: the design audit's whole reading, and nothing packs the files

Analysed with W1 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## W2, with C4 of 11-07 integrated-tests: the seeds of a slice's audit are every decision of its milestone (cluster A2)

- **a.** What it says. W2: the last slice of a five-slice milestone was seeded with 51 decisions,
  among them the core library's API, the anchors and the mutation crate. An entry citing a seed is
  graded `may-bear` at least, so one searcher graded 11 entries `may-bear` "only because each is a
  seed or cites one", and `may-bear.md` reached 155 entries, which W1's reading had to cover.
  Proposal: seed a slice's audit with the decisions the slice's spec cites and those the milestone
  lists for that slice. 0 instructions added, one changed. The standing answers of the same file
  name the seed list of each audit, extracted with `awk` and `grep` and resolved with `show`. C4 of
  11-07 integrated-tests: 54 seeds extracted by `grep` from the plan's "What is already decided" and
  the slice spec, each passed as `--seed`. Proposal: `worklist --seeds-from <plan document>`.
  Another project's file gives the same mechanism: W3 of 11-02 thaum fuzz-campaign-slice, about 110
  of 125 `may-bear` entries graded so for citing a seed, with no element of the work meeting them.
- **b.** Whether it still applies. It does. `path@agent-skills@content/skills/planning/SKILL.md`,
  lines 393 to 397, the same at `5d23ee23` and HEAD: "as seeds, the decisions the milestone document
  lists under "What is already decided", those the design rests on and those it reverses or rewrites,
  and the decisions the slice's spec cites". `worklist` takes seeds by `--seed` alone:
  `path@cli@src/cli/mod.rs`, line 130.
- **c.** Whether it is already recorded. The seeds are decided:
  `design@agent-skills@seeds-name-decisions`, "the design audit of a slice or a spec with the
  decisions its plan documents cite", and `design@agent-skills@record-searched-before-the-work`,
  "with the slice's spec and the milestone document as the work and the decisions they cite as
  seeds". The same head's argument against a goal seed, "a seed that many entries cite forces each of
  them to a `may-bear` grade at least", is the mechanism W2 reports for decisions of other slices;
  `rejected@agent-skills@goal-seeds` records it for goals. No entry records it for a milestone's
  other slices, nor the scripted extraction.
- **d.** Its kind. A defect of the text, a structure the work did not need: one fixed list where the
  slice varies. C4: a missing capability, a mechanical task done by a script twice.
- **e.** Its scope. `goal@knowledge-architect@agents-work-without-drift` (what the search reads
  against the work) and `goal@knowledge-architect@mechanical-tasks-are-the-tool-s` for C4. The owner:
  the installed planning skill, and `query` or `cli` for an option. Narrowing the seeds contradicts
  the statements of `design@agent-skills@seeds-name-decisions` and
  `design@agent-skills@record-searched-before-the-work` quoted above, so it is a decision.
- **f.** A better fix. Rival: keep the whole list, the heads' choice; it costs the `may-bear` entries
  measured above. Rival: fold this into cluster A's issue, since the seeds feed the size; it loses
  because the question differs, what is seeded rather than how it is read, under another head.
  `--seeds-from` follows the seed rule, whichever it is, so it is the rule's mechanical half; it must
  take decisions alone, never a goal a plan document cites, or it brings back
  `rejected@agent-skills@goal-seeds`. Which references count as decisions, design entries only or
  `rejected` entries too (C4 took both), is part of the rule.
- **g.** Its route. `skill@knowledge-architect-design`, then the planning text and the option.
- **h.** The proposal. Open a design issue in `agent-skills`, proposed id
  `issue@agent-skills@a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`, kind `design`, with the
  counts above, the two heads, the mechanism shared with `rejected@agent-skills@goal-seeds`, and
  `--seeds-from` as its mechanical half; it names cluster A's issue as a neighbour.
- **Default**: open an issue: `agent-skills`, kind `design`, `issue@agent-skills@a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`.
- **Outcome**: decided by the session: open an issue, as the default says. Ground: the fix rewrites
  the statements of `design@agent-skills@seeds-name-decisions` and
  `design@agent-skills@record-searched-before-the-work`, a decision for
  `skill@knowledge-architect-design` per the backstop of `primer@design-heads`; the option's reading
  of "decision" depends on that rule.

## C5: nothing assembles the audit's record from the graded entries

Analysed with C3 of 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.md`.

## W3: a design audit met an issue whose closing asks for a design discussion, and no sort of the audit fitted it

- **a.** What it says: the audit's sorts in `skill@knowledge-architect-planning@working-a-slice`,
  "Applied in place", "A change to the slices", "A choice between shapes", "Load-bearing". An issue's
  closing condition asked for a design discussion at the audit. The session applied a change in place
  as a gap; the decision-record review reported that the change answered another question than the
  head it was written into and weighed no rival, as the backstop of `primer@design-heads` asks; the
  record moved twice. Proposal: the sorts say such an issue is a choice between shapes put to the
  owner with a default, unless the record decides it. +1.
- **b.** Whether it still applies: the sorts read the same at `v0.9.0-20-g5d23ee23` and at HEAD,
  `path@agent-skills@content/skills/planning/SKILL.md`, line 407: "**Applied in place.** The gap has
  one answer the document's decisions already imply". The issue was closed at that audit, in
  01bd29bc, which "kept" two heads and "added the goals named per thread".
- **c.** Whether it is already recorded: `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`
  is about a re-entry point no review reads; here the searcher read it and the audit met it, so this
  is not its instance.
- **d.** Its kind: a lapse, once. A gap whose own entry asks for a design discussion has, by its
  words, no one answer the documents imply, so "Applied in place" excludes it; the backstop of
  `primer@design-heads` sends a decision that creates or changes a head to
  `skill@knowledge-architect-design`, which the review then applied.
- **e.** Its scope: installed text, the planning skill. Reverses nothing.
- **f.** A better fix: the file's +1 sentence is the rival; it loses on one instance, since the
  first sort's condition and the backstop already decide the case. A conformance rule is worth
  text only where the default behaviour is systematically wrong.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: "one answer the document's decisions
  already imply" excludes a gap whose entry asks for a discussion, and the backstop of
  `primer@design-heads` routes the decision; one instance.

## W5: a head-changing repair's re-review, missed at its moment; whether a split adds a head (cluster K2)

Members: W5 of 2026-11-02-knowledge-architect-development-and-components-klarch-workflow (home); W1
of 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow.

- **a.** What it says.
  - W5: in the third slice, the first repair commit widened a head's rule; the session ran the
    transcript review next and found the missed decision-record review only on re-reading the skill;
    that review, out of order, found three firm defects; in the fourth slice it ran in order and found
    five. Kind given: a lapse, recorded because the sentence sits in the paragraph read at the
    review's start and is needed when the repair is committed. Proposal: none; if it recurs, the
    sentence moves to where a repair is committed.
  - fuzz-campaign W1: the review skill's "A repair that adds, removes or reverses a design head is
    reviewed by the decision-record axis" against the primer's "A change that relocates or rewords
    recorded decisions, a split of a head included, and adds or removes none, is not a decision". The
    session read a split as adding heads and sent it to the decision-record axis twice more, three
    passes before the transcript review. Proposal: the review skill's re-review sentence points to
    the primer's sentence and re-reviews a repair only where the primer's test finds a decision
    added, removed or reversed; nearest rival, listing "a split" in the review skill.
- **b.** Whether it still applies. Unchanged, `path@agent-skills@content/skills/review/SKILL.md`,
  lines 165 to 168. The primer's sentence is the fragment
  `path@agent-skills@fragments/head-entry-tests.md`, lines 59 to 62, the same sentence at `v0.9.0`:
  it says when a change needs the design skill, and that "the routing and
  fidelity-of-relocation review axes judge that it adds or removes none"; it says nothing of the
  decision-record re-review.
- **c.** Already recorded: `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`
  settles both. Its trigger: "a heading with a slug that appears or disappears, or a reversal": a
  split writes at least one new heading, so it is re-reviewed, which is the reading the thaum session
  took. Its argument rejects the proposal of fuzz-campaign W1 by name: "A trigger on any repair that
  "makes a decision" lost: whether a rewording makes one is a judgement". `rejected@agent-skills@head-split-by-independent-reversal`
  is not brought back by either member. The cost of the passes is the subject of
  `issue@agent-skills@the-retrospective-counts-no-review-cost`, which lists sessions' review rounds
  as impressions; these two are not in it. That the passes reached the split one decision at a time
  is W2 of 2026-11-02-thaum-fuzz-campaign-slice-klarch-workflow, another cluster.
- **d.** Kind: W5, a lapse, once (the fourth slice ran it in order); fuzz-campaign W1, unclear in
  the installed text, since the head's trigger, "a heading with a slug", is in the design home and
  not in the sentence a session reads, while the primer's sentence on a split is read beside it.
- **e.** Scope: `goal@knowledge-architect@design-is-recorded-with-its-arguments`, which the head
  serves. Owner: the installed text. Proposal W1 of fuzz-campaign reverses the head's trigger; the
  rewording below does not.
- **f.** A better fix. The nearest rival to the proposal of fuzz-campaign W1 is the rival it names
  itself, the review skill's sentence naming a split, which delivers the head's trigger at the
  sentence the session reads: "A repair that adds, removes or reverses a design head, a split
  included". The fact that decides: the head's argument records the proposal's judged trigger as
  lost, and the clause states the trigger the head already holds, so it adds no instruction and
  changes no decision. For W5 the lapse rule applies: one miss, followed in order at the next slice,
  so no move of the sentence.
- **g.** Route: a rewording of installed text whose design is settled, edited at its source as
  `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says.
- **h.** Proposal: in `path@agent-skills@content/skills/review/SKILL.md`, line 165, write "A repair
  that adds, removes or reverses a design head, a split of a head included,", with a `%%` line
  above the paragraph citing `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` for
  the trigger. Instructions: +0
  (a rewording that states an existing rule's reach). The issue on review cost gains the two
  sessions' counts: three decision-record passes before the transcript review (fuzz-campaign), and
  the out-of-order and in-order decision-record reviews of the third and fourth slices
  (development-and-components). W5: no change.
- **Default**: handle now, the rewording; the review-cost issue gains the two counts; no change for
  W5.
- **Outcome**: decided by the session: handle now (the clause "a split of a head included"); the
  issue `issue@agent-skills@the-retrospective-counts-no-review-cost` gains two instances; W5 no
  change, a lapse. Ground: `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`
  states the trigger, a heading whose slug appears, and records fuzz-campaign's proposal as the lost
  rival; the clause is its member, recorded directly, per
  `design@agent-skills@extension-follows-the-ground`; W5 is one miss of a clear instruction.

## W4: the need test and the self-consistency axis give opposite verdicts on one sentence (K3)

- **a.** What it says: a sentence placed before three closure rules of the design skill, "Every
  closure below waits for the record search of loop step 1, as the next paragraph says", failed the
  need test, since the next paragraph carries the act, and was deleted; in the same review the
  self-consistency axis reported the closure rules above that paragraph as unconditioned. The
  session followed the need test. Proposal: in the agent-facing-text reviewer, a lead that conditions
  instructions before the one it points to is a qualifier, not a lead. +0, a rewording.
- **b.** Whether it still applies. The sentence was deleted by 8cf468fc, whose message lists "the
  lead before the closure rule" among "three statements that failed the need test" and judges to need
  nothing the item that "the closure rule of "Who moves what" conditions every closure on the
  search". At HEAD, `path@agent-skills@content/skills/design/SKILL.md`, line 322 opens "Who moves
  what" with "The owner's explicit word closes a fork", and lines 348 to 356 state "No thread closes
  before the record search of loop step 1 has run, in any closed state, by the owner's word or by
  yours", the next paragraph of the same section. The need test's lead clause is unchanged,
  `path@agent-skills@fragments/admission-need-test.md`, lines 22 and 23.
- **c.** Already recorded: `design@agent-skills@statement-names-an-act`: "a duplicate inside one
  text, a lead, an emphasis and a remark about the text fail, since a loaded text is read whole and
  they add no act", and its rejected rival: "a narrower test under which a second reason, a duplicate
  inside one text, a lead and an emphasis pass, loses". The proposal is that rival, for one class of
  lead. `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` asks that "The criteria of the
  reviews and of the audits agree with each other".
- **d.** Kind: an observation. The two standards do not diverge: the need test assumes a text read
  whole, and the self-consistency row asks for "Two instructions a reader cannot both obey"; a rule
  followed in the next paragraph by a stated restriction can be obeyed whole, so the self-consistency
  report was a reader's verdict the session judged and answered with its reason, as
  `skill@knowledge-architect-review@review-invariants` asks of every finding. One instance.
- **e.** Scope: the goal above. The proposal reverses part of
  `design@agent-skills@statement-names-an-act`.
- **f.** A better fix. The nearest rival to no change is moving the condition above the rules it
  conditions, the rival the file names; it is an edit of one text's order that any writer may make
  where a reader stumbles, needing no rule. The fact that decides against the proposal: it brings
  back the narrower need test the head records as lost.
- **g.** Route: nothing.
- **h.** Proposal: no change.
- **Default**: no change.
- **Outcome**: decided by the session: no change, an observation. Ground: the proposal is the rival
  `design@agent-skills@statement-names-an-act` records as lost; the self-consistency row and the need
  test agree once a text is read whole, which the need test states; 8cf468fc records the session's
  answer to the self-consistency item.

## C1: `fork-context-ref`

Analysed with C2 of 2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-02-knowledge-architect-component-rules-revision-klarch-workflow.md`.

## C2: `transcript show --from` starts only at a line the owner typed

- **a.** What it says: the transcript reviewer's brief names "the message where the branch's work
  begins"; in a session that carried four branches, that message was the agent's, after a merge. The
  reviewer extracted from the session's opening, 12,209 lines, and searched for the place.
- **b.** Whether it still applies: yes. `transcript show --help` at HEAD: "--from <TEXT> Start at
  the first line the owner typed whose text contains TEXT". The installed text assumes an owner's
  line: `path@agent-skills@content/skills/review/SKILL.md`, lines 72 to 75, the option
  `--containing "<opening words>"` finds "the session holding the message where the work begins as one the owner typed";
  `path@agent-skills@content/agents/transcript-reviewer.md`, line 42,
  `transcript show <session> --from "<opening words>" --full`.
- **c.** Whether it is already recorded: no. `design@transcript@transcript-query-verbs` gives `show`
  "`--from` a text the owner typed"; `design@agent-skills@transcript-found-by-listing` names a
  session "by the opening words of the message where its work begins". Neither argues against a
  start at another line.
- **d.** Its kind: missing, for a session shape the texts do not foresee; one instance.
- **e.** Its scope: `goal@transcript@a-session-reads-past-sessions`; owner `transcript`, with the
  installed text's sentence on the brief. A start at a line number `show` prints, or at an agent's
  line, is a new member of `design@transcript@transcript-query-verbs`, within its argument (the
  verbs compute what the review needs).
- **f.** A better fix: the nearest rival is no change, the brief naming the last owner's message
  before the branch's work, which costs the reviewer the lines between it and the work. What
  decides: one instance, and a cost in reading, not a lost item; the reviewer found its place by
  search.
- **g.** Its route: an issue, with a re-entry on recurrence.
- **h.** The proposal: open an issue in `transcript`, kind `todo`, id
  `issue@transcript@show-starts-only-at-a-line-the-owner-typed`.
  - What: in a session that carries several branches, a branch's work can begin at the agent's
    line after a merge, and `show --from` reaches only a line the owner typed, so the reviewer reads
    from an earlier line.
  - Why it matters: the transcript reviewer reads the whole of what it extracts, so every line
    before the work is a cost in its context, against `goal@transcript@a-session-reads-past-sessions`.
  - What would close it: `show` starting at a line number it prints, or at an agent's line, with the
    brief's sentence in the review skill naming it; or a ruling that the last owner's message
    suffices. Re-entry: a second instance.
- **Default**: open the issue.
- **Outcome**: decided by the session: open `issue@transcript@show-starts-only-at-a-line-the-owner-typed` in
  `transcript`, kind `todo`. Ground: no record holds it, and its fields can be written, per
  `primer@met-outside-the-task` row 3.

## C4: nothing applies the repair a commit-message finding names

Analysed with C2 of 2026-11-01-thaum-design-record-audit-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-thaum-design-record-audit-klarch-workflow.md`.

## The standing answers

- **a.** Items:
  - Installed file: sources edited and installed. An observation.
  - The commits after a failed staged check, chained on `tail` and on `;`: cluster H2 (G6), home
    P1 of 2026-11-01-knowledge-architect-development-milestone-plan.
  - The false claims in commit messages: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - The attribution of a fragment's second host to the owner, and two quotations of the owner
    altered by a `sed` rename: analysed with the attribution cluster of
    2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.
  - A forked agent judging entries from the searchers' reasons: W1 of this file, cluster A (G1).
  - A deleted milestone named in words: outside the checker's syntax. An observation.
  - Hand tasks: C3 is cluster A (G1); C4 is cluster G (G4), home C2 of
    2026-11-01-thaum-design-record-audit-klarch-workflow; C5 is cluster B (G1); the seed list of
    each audit is cluster A2 (G1), home W2 of this file.
  - The trap of a test filter without its module path: analysed here.
- **b.** Repaired: 540df107 added to the development skill "The command names the test under
  check by its module path, with `-- --exact` after it", at HEAD
  `path@agent-skills@content/skills/development/SKILL.md`, line 95.
- **Default**: no change.
- **Outcome**: decided by the session: no change, the trap already stated in the installed text,
  and the other items analysed here observations. Ground: 540df107.

---
