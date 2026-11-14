# 2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, main at `v0.9.0-369-g8e23dd57`, on a feature branch, with a clean tree at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@mutate@one-command-owns-the-interval`: constrains, respect.
  - `design@agent-skills@test-economy`: constrains, respect.
  - `issue@agent-skills@the-upstream-set-omits-the-published-libraries`: touches, none.
  - `design@knowledge-architect@mutation-command-host`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.
  - `design@agent-skills@transcript-review-last-before-merge`: constrains, respect.
  - `design@agent-skills@transcript-review-repairs-are-additional-commits`: constrains, respect.
  - `design@agent-skills@local-intent-binds`: constrains, respect.
  - `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.

## mutate-replace-needs-unique-text: `cargo x mutate run --replace` takes only a text that occurs once

Members: 2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow C1.

- **a. What it says.** Two declarations in one file carried the same text, `value_parser = since`,
  and each had to be mutated apart; `--replace` is "Refused unless it occurs exactly once". For
  each, the session wrote a patch by hand with `awk` and `diff -u` and passed it with `--patch`, two
  extra steps per mutation. Proposed fix: `--occurrence <n>` beside `--replace`, which picks the
  n-th occurrence and refuses an n past the last (0 instructions added). The file's nearest rival: a
  longer `--replace` text spanning lines until it is unique, which failed where the two
  neighbourhoods differed only in a field name three lines away.
- **b. Whether it still applies.** Yes. `path@mutate@src/lib.rs` line 67 at `8e23dd57` and at
  origin/main: "The text to substitute. Refused unless it occurs exactly once."; `plan_replace` in
  `path@mutate@src/harness.rs` refuses a count above one with `PlanError::NotUnique`.
- **c. Whether it is already recorded.** No entry records it. The reason for the refusal is a
  comment at the code, not a head: `PlanError::NotUnique`, "Replacing all of them is a different
  mutation from the one the author described, and replacing the first is a coin toss". A named
  occurrence is neither: the author names the one site.
  `design@mutate@one-command-owns-the-interval` (verdict `constrains`) keeps each apply and restore
  inside the command, which an option keeps and a hand-made patch also keeps.
- **d. Its kind.** A missing capability of the checker's tools, met once; `--patch` covers the case
  at the cost of a patch written by hand each time.
- **e. Its scope.** `goal@tasks-a-tool-does-better-are-the-tool-s`, for a task already taken in by
  `design@mutate@mutation-command`, and
  `design@knowledge-architect@a-tool-s-writes-lose-nothing-the-project-wrote` (verdict
  `constrains`), which the option keeps since the restore compares bytes. The owner of the fix is
  `mutate`; no head changes, and the comment at `PlanError::NotUnique` and the README's "The two
  input forms" are reworded with it.
- **f. A better fix.** The rival is no change: `path@mutate@README.md` gives `--patch` the mutations
  that are not one substitution, and a repeated text can be reached that way. The deciding fact: the
  patch the session wrote follows from the file and the occurrence's number with no judgement, which
  is the task the tool takes in; and the longer-text workaround fails where the sites differ only
  far from the text. One instance weighs it low, so it is no work for the intake's branch.
- **g. Its route.** An issue only: a change of `mutate` under `skill@klarch-development`, on a
  branch of its own or with another of the crate's.
- **h. The proposal.** Open a todo issue in `mutate`,
  `issue@mutate@a-repeated-text-cannot-be-replaced-at-one-of-its-occurrences`, with the instance, the option
  proposed and its refusal of an n past the last, and the comment and README sentence it rewords.

Default: open the issue.
Outcome: decided by the session: open an issue, `mutate`, kind `todo`, `issue@mutate@a-repeated-text-cannot-be-replaced-at-one-of-its-occurrences`. Ground: the refusal's own reason does not reach a site the caller names; the change reverses no head and is a change of the Rust source, which needs a branch of the crate.

## transcript-review-record-without-repairs: where the record of a transcript review with no repairs goes

Members: 2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow W1. One cause with `review-record-moves-each-round`, whose section holds the repair.

- **a. What it says.** The last transcript review found nothing. The session amended the last repair commit's message to add a paragraph on that review, after the transcript reviewer had read that commit, and checked that the amend changed no file. The file blames `skill@knowledge-architect-review@what-review-leaves`, "Where the last transcript review has repairs, the record of its items is the message of its last repair commit, and the record of the earlier axes stays in the commit that held it when the transcript reviewer read it": it states no case where that review has no repairs, and the general rule covers it only if the reword is allowed on the commit the transcript reviewer read. Kind given: unclear, low; cause, a rule written for one interaction. Proposal: delete the first clause and keep the second as an exception to the general rule (0 added, 1 removed); rival, a sentence for the case with no repairs.
- **b. Whether it still applies.** Yes. The sentence is the same at the file's version, v0.9.0-369-g8e23dd57 (line 219 of `git show v0.9.0-369-g8e23dd57:crates/agent-skills/content/skills/review/SKILL.md`), and at origin/main, `path@agent-skills@content/skills/review/SKILL.md`, in the bullet that opens "A paragraph in the commit message" (line 220).
- **c. Whether it is already recorded.** No entry records it. The sentence came from the earlier intake that `review-record-moves-each-round` item c names. The verdicts: `design@agent-skills@transcript-review-last-before-merge`, `design@agent-skills@transcript-review-repairs-are-additional-commits` and `design@knowledge-architect@a-record-rides-on-a-commit-that-changes-a-file` (constrains, respect), each kept by the repair as that section says. `design@agent-skills@local-intent-binds` (constrains, respect): the reason for the clause is read in the message of the commit that introduced it, 7e0bb594, and in the analysis it carried out, "which keeps one record per review that produced it and edits no history"; the repair keeps that reason.
- **d. Its kind.** A defect of the text, unclear: the case-specific sentence leaves the case without repairs to a reading of the general rule.
- **e. Its scope.** The installed review skill, the agent-skills Component. No head states the rule; the change is a decision the earlier argument covers.
- **f. A better fix.** The file's proposal and the repair of `review-record-moves-each-round` both delete the transcript review's own sentence. The repair there also states the general case per round, so the transcript review with no repairs is a round that leaves no repair commit: its paragraph goes in the branch's last commit, reworded, and no earlier record moves. What decides for the joint repair: the file's own proposal leaves the moves `review-record-moves-each-round` reports, and one repair per cause is the rule.
- **g. Its route.** A text edit of the installed text at its source, as `review-record-moves-each-round` item g says.
- **h. The proposal.** Handle now, by the repair of `review-record-moves-each-round`; no edit of its own.

Default: handle now, by the repair of `review-record-moves-each-round`.
Outcome: decided by the session: handle now, by the repair of `review-record-moves-each-round`, which removes the transcript review's own sentence and states the round without repairs. Ground: one cause, one repair, per the admission standard's "A finding is repaired at the cause that produced it, not at its site"; the general case the repair states answers this case.

## standing-2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow, lines 57 to 76, and its
summary, lines 3 to 7. One item takes a section of its own below, the bounded classification. The
file predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- the claims of each message checked against the tree with no list written beside them, none false,
  "the code-claims reviewer confirmed 23 of 24" (lines 60 to 65): `message-claims-written-unchecked`;
- the issues and tripwires of `cli` read, not those of `transcript` and `create` (lines 66 to 70):
  `development-grounding-skips-tripwires`;
- the mutation patches of C1 (lines 72 and 73): `mutate-replace-needs-unique-text`.

Analysed here:

- **The roadmap's ordered rows renumbered by a short script** after a closed issue's row left (lines
  73 to 75): "the planning skill owns roadmap edits, and no command renumbers. The second is too small
  for a finding." Kind: a task with no judgement, by the file's own account too small for a finding.
  The edit needs no owner's word: "the commit that closes an issue with no plan document scheduling
  it removes its row", per `skill@knowledge-architect-planning@spec-or-milestone`, and the
  renumbering keeps the order.
- **Installed file changed**: no. **Backticks**: no. **Code rules**: none too strict.
  Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: observations, and a mechanical step its own
file judges too small for a finding.

## standing-2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow: a session that classified its work bounded

Members: the file's summary, lines 3 to 7: "One issue handled as bounded work on one branch".

- **Whether it is recorded.** `issue@agent-skills@the-open-space-test-may-admit-every-problem`, a
  `question`, asks whether the open-space test leaves bounded work almost unreachable, and closes on
  "a count of requested changes the design skill grounded, and how many it classified bounded, read
  at the retrospectives ... a run of bounded classifications the owner accepts answers it no." Its
  count, taken on 2026-10-29 with `git log main --oneline -i --grep='as bounded work'`, found none
  in this repository. This file reports one. Its commit on main, 4c4a28e1, "[cli] The checks of a
  date, a pattern and a project's name are made while parsing", names no classification
  (`git show -s 4c4a28e1 | grep -i bounded` finds nothing), so the count's command does not find it: at
  origin/main it returns one commit, 14d69c08, the one that recorded the count, and
  the file does not say whether the owner was asked or accepted it. The searcher of the issue routed
  the file to it, as evidence its closing condition asks for.
- **Kind.** An observation, of the evidence the issue waits for: one instance, its acceptance by the
  owner not stated.

Default: add the instance to the issue's "What", after the count, as below.

```
One instance after that count, read at the intake of 47 files: the branch that moved three value
checks into the command-line declaration, whose workflow file says it handled one issue "as bounded
work" (2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow). Its commit on
main, 4c4a28e1, names no classification, so the count's command does not find it, and the file does
not say whether the owner accepted the classification.
```

Outcome: decided by the session: handle now, the instance added to
`issue@agent-skills@the-open-space-test-may-admit-every-problem`. Ground: the issue's closing
condition counts bounded classifications read at the retrospectives, and this is one, with the
limits of its evidence stated; the edit changes no instruction.

## review-record-moves-each-round: the record of a review sits in the last repair commit, so each later round moves it

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.md`, in its section on `review-record-moves-each-round`, which gives the outcome.

## development-grounding-skips-tripwires: the touched Components' tripwires read in part before a Rust edit

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md`, in its section on `development-grounding-skips-tripwires`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
