# 2026-11-13-knowledge-architect-plan-directories-slice-1

- **Received file**: 2026-11-13-knowledge-architect-plan-directories-slice-1.md, the project file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from this repository: main commit `a0454f46` (`v0.10.0-5-ga0454f46`) at the start; branch `plan-directories`; no uncommitted changes at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `issue@agent-skills@a-command-chained-after-a-failing-check-recurs`: touches, respect.
  - `design@agent-skills@standing-state-review-before-every-merge`: constrains, respect.
  - `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`: touches, respect.
  - `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: touches, respect.
  - `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: trigger-met, rule-before-work.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.

## broad-trigger-met-by-every-change: a trigger or re-entry point written as "the next change that touches X" is met by nearly every change

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1 P2; 2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow W2.

- **a. What it says.** P2: the re-entry point of `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`, "when work next changes what a session reads or runs at a commit", is met by nearly every change; the slice's audit search graded it `trigger-met` since the slice adds steps at three kinds of commit, the session argued it was not the occasion, and the design-conformance reviewer argued the opposite. Proposal: reword that re-entry to a named event, "such as a change to the commit step of an installed skill or to what `cargo klarch commits` prints, or a count of further lapses in the intakes"; 0 added. W2: a `trigger-met` verdict with the action `rule-before-work`, from `skill@knowledge-architect-design@discussion-loop`, pulled three issues into a design; one was put on the milestone's first slice and taken out on the owner's word; in the owner's view, deferred issues with triggers often bring unrelated work into scope at bad moments. Kind, per W2: wrong, in the owner's view, one instance; proposal: none, the project opened a `design` issue.
- **b. Whether it still applies.** The re-entry point stands on origin/main, line 70 of the claim-check entry, as P2 quotes it; it was the same at the version, `a0454f46`. The design skill's sentence on `rule-before-work` is unchanged in substance. It applies.
- **c. Whether it is already recorded.** Yes. W2 is the origin of `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment` (`touches`, respect, agent-7), whose Details open with "The owner's report, in the design discussion of the milestone on split guidelines and plan directories". Its What would close it holds P2's case: "the trigger test made stricter, the search's action on a met trigger changed from a ruling before the work to a report, ...", and its re-entry is "a design discussion under `skill@knowledge-architect-design` on the owner's word". P2 is a further instance of it, with a mechanism the entry names: the trigger test of `skill@knowledge-architect-issue-tracking@trigger-test`, "A trigger must name an occasion whose own work already includes the work the trigger names", passed by the words of a re-entry point whose occasion did not need the work. `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` (`touches`, respect) holds the opposite question, whether a review reads re-entry points; P2 shows a design-conformance reviewer reading one, and the deferred-trigger entry says a decision on either bears on the other, so one instance, in the deferred-trigger entry, reaches both. `design@agent-skills@standing-state-review-before-every-merge` (`constrains`, respect): the reviewer keeps reading every trigger. `issue@agent-skills@a-command-chained-after-a-failing-check-recurs` (`touches`, respect): its re-entry is the claim-check discussion, so a rewording of that re-entry reaches it too. The claim-check entry itself is graded `trigger-met` / `rule-before-work` (agent-10), on other subjects of this intake (quick-fixes C2) as well as this one; its ruling takes a section of its own where the subject that meets it is analysed.
- **d. Its kind.** Recorded: an instance of a recorded design issue.
- **e. Its scope.** `goal@agents-work-without-drift`, which the recorded issue names. The owner of the fix: the recorded issue's design discussion.
- **f. A better fix.** P2's rewording of one re-entry point, now, is the rival. The fact that decides: choosing the event a re-entry names is the question the recorded issue holds for every trigger and re-entry point, and its discussion opens on the owner's word; one entry reworded ahead of it gives one answer before the rule is designed. A re-entry point was rewritten case by case once before, after the owner kept its work out of a milestone, as the recorded issue's second instance shows, so the owner may ask for this one; the session does not take that choice.
- **g. Its route.** An issue only: an instance added to the recorded issue.
- **h. The proposal.** Add the paragraph below to `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`, after its second instance. No rewording of the claim-check entry's re-entry point. Instructions: 0.

Default: an instance added to the recorded issue; the claim-check entry's re-entry point kept.
Outcome: decided by the session: no change to the text, an instance added to `issue@agent-skills@a-deferred-trigger-brings-unrelated-work-into-a-design-at-a-bad-moment`. Ground: W2 is that entry's origin and P2 an instance of its question; rewording one re-entry point ahead of its discussion answers that question for one entry, and the discussion opens on the owner's word.

```text
A third instance, from 2026-11-13-knowledge-architect-plan-directories-slice-1 P2: the re-entry
point of `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
"when work next changes what a session reads or runs at a commit", was graded `trigger-met` at the
audit of the first slice of the plan directories, which adds steps at three kinds of commit; the
session argued the slice was not the occasion, and the design-conformance reviewer argued it was.
Two readers applied the point in opposite directions, so it names no event they agree on. The
file proposed naming the event: a change to the commit step of an installed skill or to what
`cargo klarch commits` prints. It also shows a review reading a re-entry point, the question of
`issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`.
```

## restore-run-with-dirty-tree: a file restored from a commit while the tree held uncommitted changes

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-retrospective-standing-questions.md`, in its section on `restore-run-with-dirty-tree`, which gives the outcome.
