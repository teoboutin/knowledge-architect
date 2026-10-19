# 2026-10-18-thaum-chance-slice-merge-klarch-workflow

- **Received file**: 2026-10-18-thaum-chance-slice-merge-klarch-workflow.md, the workflow file of a
  session in thaum that worked the last slice of a milestone from its design audit to its merge.
- **Version used**: the published checker, 0.7.0, read at the tag `v0.7.0`.
- **Analysed against**: main at `v0.7.0-62-g6e2b300`.
- **Standing entries that bear on it**:
  - for W1: `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`
    and `issue@agent-skills@the-material-finding-duty-has-no-head`, near it; four tripwires judged
    not met in its section;
  - for W3: `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it`,
    whose deferred trigger the file does not meet: it reports no reviewer judging without the
    deleted plan documents;
  - for W4, with W2 of 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow:
    `issue@agent-skills@the-retrospective-counts-no-review-cost`;
  - for C1: analysed with C1 of 2026-10-15-knowledge-architect-primer-relocation-klarch-workflow.

## W1. A member of an approved head that the specification forbids: narrowed by the session, or put to the owner

- **a. What it says.** The owner approved a head's rule. A reviewer showed that one new member of it
  contradicts the specification the project implements. The session narrowed the head on the
  specification's text and reported it under "Decided" after the merge. The file finds three texts
  pointing two ways: the design-heads clause on a head written from an approval, the source rule of
  `primer@owner-word-premise`, and the review skill's "an item whose outcome the record, a
  measurement or a source settles takes it from the session". The fix proposed: one sentence in
  `primer@design-heads`, the narrowing applies at once and goes to the owner before the merge.
- **b. Whether it still applies.** The texts are unchanged at both commits. Read whole,
  `path@agent-skills@content/PRIMER.md` gives one answer:
  - the design-heads clause: "A clause that widens it, narrows it, or drops part of it … is still
    not what was approved: it goes to the owner, as a change of the decision";
  - the source rule is scoped to "An answer of the owner on a reading or a fact": what a text says,
    or what the code or a tool does;
  - the next paragraph, which the file does not quote: "**A ruling on intent whose premise proves
    false is a contradiction.** … Put the corrected premise to the owner at the top of the turn,
    quoting the word it defeats; the part of the work that rests on it waits for the answer".

  The approval of a head's rule is a ruling on intent, as the file itself says: "The owner approved
  a design, not a reading". The source rule's scope excludes it, and the other two agree.
- **c. Whether it is already recorded.** Near it:
  `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default` asks whether
  the planning skill's default sends a session past the primer's hold, and
  `issue@agent-skills@the-material-finding-duty-has-no-head` records a missing head. Neither covers
  this case. `rejected@agent-skills@owner-reading-answer-put-back` covers answers on a reading, not
  approvals. Judged not met: `tripwire@agent-skills@departure-overridden-as-a-reading`, since no
  deliberate departure was overridden; `tripwire@agent-skills@member-beyond-the-argument`, which
  watches a member recorded directly, not one excluded;
  `tripwire@agent-skills@record-answered-questions-reach-the-owner`, the opposite direction; and
  `tripwire@agent-skills@expectation-set-closes-a-contradiction`, since no expectation set is cited
  here.
- **d. Its kind.** A rule applied outside its scope: the source rule, read without its scope. Not a
  defect of the installed text, which sends the narrowing to the owner by two of the three texts
  and excludes the case from the third. The project in question sets aside, on its owner's word, the
  primer's premise procedure for a reading its specification answers, as
  `issue@agent-skills@patching-an-installed-skill` records; that departure may have shaped the
  session's reading, and is the project's own.
- **e. Its scope.** The proposal, "applies at once" for a ruling on intent, contradicts the third
  paragraph of `primer@owner-word-premise` and changes `design@agent-skills@owner-word-holds-as-far-as-its-premise`.
- **f. A better fix.** Rivals: the proposal, a change of that head; a pointer from the design-heads
  clause to the paragraph on a ruling on intent, which restates what the clause already says; no
  change. No change wins: the clause already says where the narrowing goes, and the precedent the
  file cites, an earlier narrowing of the same rule put to the owner, followed it.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.
- **Default**: no change: read whole, the primer sends a narrowing of an approved rule to the owner,
  and the source rule covers answers on a reading only.
- **Outcome**: accepted.

## W2. Several transcript files hold the same opening words

- **a. What it says.** `skill@knowledge-architect-review@review-axes` finds a transcript by "the
  opening words of that message". Five files held them, since each slice of the milestone was opened
  with the same sentence. The session chose the one that also held the owner's answer to the audit's
  question. The fix proposed: name the file that also holds a later message of the work.
- **b. Whether it still applies.** In part. `8ff0441`, "[agent-skills] A transcript file is named
  once it holds the opening message as the owner sent it, not as a quotation", changed the sentence,
  `path@agent-skills@content/skills/review/SKILL.md` lines 62-66: "name a file only once it is seen
  to hold the message as one the owner sent, not as a quotation of it in a later session". That
  settles quotations, not five sessions each opened by the owner with the same sentence.
- **c. Whether it is already recorded.** No.
- **d. Its kind.** Room to judge: the session chose by a later message of the work, which is what
  the proposal would write, and the brief said so.
- **f. A better fix.** No change: a rule for this one case is a narrow rule where judgement already
  decides, per the repair by cause of
  `skill@knowledge-architect-agent-configuration@admission-standard`.
- **h. The proposal.** No change.
- **Default**: no change, judgement decided the case as the proposal would.
- **Outcome**: accepted.

## W3. How the deleting commit repairs a citation of a leaving plan document

- **a. What it says.** `skill@knowledge-architect-planning@plan-document-leaves` says the deleting
  commit repairs each citation of the leaving document, and not how. The deletion left 50 dangling
  citations in 31 files; the session followed the project's precedent. The fix proposed: a citation
  stating a dependency is replaced by the reference to the entry its harvest wrote, any other is
  rewritten or removed as the issue-tracking skill says of a deleted entry.
- **b. Whether it still applies.** Yes, `path@agent-skills@content/skills/planning/SKILL.md`, lines
  589-591, unchanged: "the deleting commit repairs each". The issue-tracking rule, lines 288-293 of
  `path@agent-skills@content/skills/issue-tracking/SKILL.md`: "**A sentence about the past whose
  reference dangles is rewritten to state the present, or removed** … It is never retargeted to the
  new name, which would make it false, and never turned into plain text". The file's quotation of it
  is a composite of two sentences.
- **c. Whether it is already recorded.** No.
  `issue@agent-skills@a-plan-document-deleted-at-its-harvest-is-unseen-by-the-acts-that-need-it` is
  about the acts that need the deleted spec, not the repair of its citations.
- **d. Its kind.** Missing: a pointer from the act to the rule that decides it. The rule exists.
- **f. A better fix.** The proposal's first clause, "replaced by the reference to the entry its
  harvest wrote", is a retargeting, which the issue-tracking rule forbids for a sentence about the
  past; it holds only where the sentence is rewritten to state the dependency in the present, as the
  session did. The rival: one pointer, "as `skill@knowledge-architect-issue-tracking` says of a
  sentence whose reference dangles", which carries both cases. Instructions added: 0.
- **g. Its route.** Installed text edit.
- **h. The proposal.** Handle now: the planning sentence points to the issue-tracking rule.
- **Default**: handle now, as h says.
- **Outcome**: accepted.

## W4, with W2 of 2026-10-19-knowledge-architect-ship-retrospective-intake-klarch-workflow. Review rounds and their cost

- **a. What they say.** This file: each repair of a head led to a new pass of the decision-record
  axis, three in all, each finding real defects; the instruction worked. The other: three review
  rounds before the merge found about 30, 45, 8 and 2 items, each round finding defects in the last
  round's repairs. Both are observations; the second cites the issue below.
- **b.** The instruction is unchanged, line 145 of `path@agent-skills@content/skills/review/SKILL.md`.
- **c. Whether it is already recorded.** Yes: `issue@agent-skills@the-retrospective-counts-no-review-cost`
  holds two such impressions already, and closes when the retrospective counts the rounds, not on
  more impressions.
- **d. Its kind.** Observations.
- **h. The proposal.** No change: the issue's closing is the instrument, and these add no new kind
  of evidence to its design.
- **Default**: no change, observations the issue already covers.
- **Outcome**: accepted.

## C1

Analysed with C1 of 2026-10-15-knowledge-architect-primer-relocation-klarch-workflow, in that file:
the 75 tokens are the class A that `spec@plans@defined-ids-outside-references` plans to report.

- **Default**: no change, scheduled by the spec.
- **Outcome**: accepted.

## The standing answers

- **a.** A tripwire put to the owner whose firing clause was already true when written, then
  reworded with no ruling; a reviewer's drop of a false statement in a commit message; 75 plain-text
  references (C1); and a project skill's addition first missed. Each was caught before the merge.
- **b.** The quoted rules hold at both commits. Two claims of the answers are not accurate: no
  deletion rule prescribes naming a deleted plan document in words, the primer only places "a
  description in words" outside its rule; and "A reference is one backticked span" sits in
  `primer@checker-model` on main.
- **d.** Lapses, caught before the merge.
- **Default**: no change, lapses caught before the merge.
- **Outcome**: accepted.
