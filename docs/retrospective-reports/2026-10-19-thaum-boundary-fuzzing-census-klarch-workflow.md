# 2026-10-19-thaum-boundary-fuzzing-census-klarch-workflow

- **Received file**: 2026-10-19-thaum-boundary-fuzzing-census-klarch-workflow.md, the workflow file
  of a session in thaum that ran the first slice of a milestone, from its design audit to its merge,
  compacted once.
- **Version used**: the published checker, 0.7.0, read at the tag `v0.7.0`.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**:
  - for W2: `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names`;
  - for W3: `issue@agent-skills@the-retrospective-counts-no-review-cost`, for its cost;
  - for W4: `tripwire@agent-skills@split-rule-from-exception`, not met, since the split separated
    three decisions with different rivals, not a rule from its exception;
  - for the standing answers: `tripwire@agent-skills@attribution-loses-the-scope`, judged in
    `path@knowledge-architect@docs/retrospective-reports/2026-10-15-knowledge-architect-primer-relocation-klarch-workflow.md`.

## W1. The transcript reviewer labels the compaction summary as the owner's words

- **a. What it says.** The harness writes a compaction summary as a `user` line with
  `isCompactSummary` true, no `isMeta`, no `origin`, and prose at its start. The reviewer's
  extraction labelled it the owner's; the reviewer's own judgement caught it. Read as the owner's, it
  would let a ruling be checked against the session's paraphrase of it. The fix proposed: name
  `isCompactSummary` as a field of a harness line, beside `isMeta` and `origin`.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/agents/transcript-reviewer.md`,
  lines 48-61, unchanged since the tag: injected lines are those whose `isMeta` is true or whose
  `origin.kind` is not `human`; "Label each `user` line … by those fields", and "Select and label by
  the fields of each line, never by a substring of its text". A line with neither field is the
  owner's by that rule. `git grep isCompactSummary origin/main` finds nothing.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Wrong: following the rule labels a harness line as the owner's. A fact of the
  harness's format that no text supplies and judgement cannot derive, a provable gap; its fill adds
  to the list of fields, so it goes to the owner, as this proposal does.
- **e. Its scope.** `goal@knowledge-architect@the-owner-decides`: the owner's word is what the
  reviewer checks rulings against.
- **f. A better fix.** Rivals: name the field, the proposal; leave it to judgement, which caught it
  here, but which the rule tells to label by fields and never by text. The field is one more member
  of the list the rule already holds. Instructions added: 0, a fact added to an existing instruction.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: a `user` line whose `isCompactSummary` is true is injected, a
  summary written by the harness.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W2. A review's reports exist only in the conversation, and a compaction removes them before their outcomes are recorded

- **a. What it says.** Seven reviewers returned their reports as messages. The session repaired most
  items and was compacted before writing the record; the summary kept a list of repairs, not the
  reports. The session recovered each report from the subagents' own transcripts. Without that, items
  judged to need nothing would have had no outcome. The fix proposed: the dispatcher writes each
  report to a file of its scratch directory when it arrives, and builds the record from those files.
- **b. Whether it still applies.** Yes. `path@agent-skills@content/skills/review/SKILL.md`, lines
  140-143, "Every item of a report gets an outcome" and "The record of the review lists each item
  with its outcome", unchanged; no text says where the reports are kept until the record. The
  scratch directory of lines 109-111 is for a subagent's own files.
- **c. Whether it is already recorded.** In part:
  `issue@agent-skills@a-reviewer-cannot-write-the-file-its-brief-names` names the same loss, "A
  reviewer whose report comes back only as a message leaves the session no file to read again", for
  the case where the reviewer is refused its own write. The proposal does not depend on that write:
  the dispatcher writes. `rejected@agent-skills@ledger-written-during-discussion` lost a file updated
  every round, "An artifact an agent must update every round is one it forgets to update"; a report
  saved once, verbatim, on arrival is not that.
- **d. Its kind.** Missing: a capability, keeping what the record is built from beyond a compaction,
  seen in real use.
- **e. Its scope.** `goal@knowledge-architect@agents-work-without-drift`; installed text, the review
  skill.
- **f. A better fix.** Rivals: the proposal; rely on the harness's subagent transcripts, which held
  the reports here but only as a hand-back the session had to find in raw transcripts; have each
  reviewer write its own report file, which the issue above shows refused for general-purpose
  agents. The dispatcher's write is the one that does not depend on the agent type. Instructions
  added: 1.
- **g. Its route.** Installed text edit, a behaviour addition on real use: the owner rules on it
  here.
- **h. The proposal.** Handle now: one sentence in `skill@knowledge-architect-review@what-review-leaves`,
  the dispatcher saves each report as it arrives in a file of its scratch directory and builds the
  record from those files.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W3. The fidelity axis and the project's rule that a workflow decision earns no head pulled one sentence two ways

- **a. What it says.** The harvest moved a sentence of a scoped `CLAUDE.md` into a head without a
  clause. The fidelity reviewer reported the clause lost; the session restored it. The
  decision-record reviewer then reported the clause as a statement about the review workflow, which
  earns no head in that project; it was removed again. The fix proposed: the fidelity question adds
  "and was what was lost true where it stood".
- **b. Whether it still applies.** The row is unchanged, line 36 of
  `path@agent-skills@content/skills/review/SKILL.md`: "where content moved, was anything lost? A
  reason dropped, a number changed, an argument compressed to an assertion".
- **c. Whether it is already recorded.** Only its cost, in
  `issue@agent-skills@the-retrospective-counts-no-review-cost`.
- **d. Its kind.** An interaction with the project's own departure from entry test 2, which it
  states on its owner's word. Under the installed text alone the two axes would not have disagreed on
  that ground. A reviewer's finding is a claim the dispatcher judges: the review skill lets the
  session judge an item to need nothing, with the reason, which settles a lost clause that was not
  owed.
- **f. A better fix.** No change: the dispatcher's judgement of each item already asks whether the
  lost content was owed, and the clause proposed would put that judgement in the reviewer, which
  lacks the project's rule.
- **h. The proposal.** No change.
- **Default**: no change: the session judges each finding, and the conflict came from the project's
  own departure.
- **Outcome**: accepted.

## W4. The harvest instruction names the entry tests as governing over the harvest row, not the rule that a head holds one decision

- **a. What it says.** The slice spec's harvest row mapped three threads to one slug; the session
  wrote one head of three decisions, each losing to a different rival. The decision-record reviewer
  reported it, and the head was split. The fix proposed: point 6's governing clause extends to the
  shape rules of `primer@design-heads`.
- **b. Whether it still applies.** In part. Point 6 of `skill@knowledge-architect-planning@working-a-slice`,
  `path@agent-skills@content/skills/planning/SKILL.md` lines 439-446, unchanged: "Read
  `primer@design-heads` again, whole, before writing a head: it holds what earns one and how it is
  shaped … The row names what is judged; the entry tests and the tests for a losing alternative
  decide whether each decision and each alternative earns an entry, and they govern where the two
  disagree". The reading is asked; the governing clause names only the entry tests.
- **c. Whether it is already recorded.** No; `issue@core@four-heads-bundle-decisions-with-many-citing-sites`
  shows bundles from another cause.
- **d. Its kind.** Unclear: the clause that says which text governs is narrower than the reading the
  same point asks for, so a row that bundles reads as authoritative on the cut.
- **f. A better fix.** Rivals: the proposal, a rewording of the clause; plan reviewers checking the
  row's cut at design time, which cannot work since the row precedes the heads' text; no change,
  since the decision-record reviewer caught it. The rewording costs nothing and closes the reading.
  Instructions added: 0.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: "the entry tests, the rules on a head's shape, and the tests for
  a losing alternative … govern where the two disagree".
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## C1. A front-elided quote that drops the clause scoping its rule verifies

- **a. What it says.** A quotation of a rule, elided at its front, verified while it dropped the
  clause that limited the rule. A lint is suggested.
- **b.** Not about this repository's checker: the core has no quotation check; the verification is
  the project's own extension, which `issue@core@citation-regime-over-any-pinned-text` mentions as
  that project's.
- **d. Its kind.** Out of scope: it belongs to the project's extension.
- **h. The proposal.** No change.
- **Default**: no change, the project's own checker extension.
- **Outcome**: accepted.

## The standing answers

- **a.** Five primer rules acted on as if absent, each found by a reviewer or the session: a head
  holding three decisions (W4); two hedges dropped, with a commit message claiming them kept; a
  head and a tripwires home holding an account of the past; three code modules restating their
  heads' arguments with no reference; a head describing a later slice's work in the present.
- **b.** The rules quoted hold. "A design head is built intent" misquotes "A design home is built
  intent", in `primer@intent-and-claims`.
- **d.** Lapses. The dropped hedges are judged with the others in the first file's last section.
- **Default**: no change, lapses caught before the merge.
- **Outcome**: accepted.
