# 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow

- **Received file**: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: the main commit the slice's tree contained when its work began is `v0.9.0-257-gcb8d5a22`, branch `goals/standard-and-rewrite`, no uncommitted change."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@knowledge-table-home`: constrains, respect.
  - `design@agent-skills@a-goal-reading-is-reported`: constrains, respect.
  - `design@agent-skills@a-goal-carries-an-example-only-for-its-direction`: conflicts, check-reversal.
  - `design@agent-skills@goals-change-through-goal-setting`: constrains, respect.
  - `design@agent-skills@whole-set-rounds`: conflicts, check-reversal.
  - `rejected@agent-skills@goal-carries-reading-examples`: brings-back, new-argument-needed.
  - `issue@agent-skills@no-review-compares-the-readings-heads-take-of-one-goal`: touches, none.
  - `design@agent-skills@standing-argument-in-head`: constrains, respect.
  - `rejected@agent-skills@owner-intent-entry-test`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: conflicts, check-reversal.
  - `rejected@agent-skills@repair-defects-left-to-next-pass`: constrains, respect.
  - `design@agent-skills@at-least-one-independent-verification`: constrains, respect.
  - `rejected@agent-skills@a-check-is-not-checked-again`: constrains, respect.
  - `goal@agents-work-without-drift`: constrains, respect.
  - `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, respect.
  - `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`: touches, none.
  - `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect.
  - `design@agent-skills@statement-names-an-act`: constrains, respect.
  - `design@agent-skills@admission-rules-home`: constrains, respect.
  - `design@agent-skills@root-cause-repair`: constrains, respect.
  - `design@agent-skills@audit-net-growth`: constrains, respect.
  - `design@agent-skills@repairs-read-against-the-record`: constrains, respect.
  - `rejected@agent-skills@repair-check-left-to-session`: constrains, respect.
  - `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect.
  - `design@check@quotation-read-by-no-id-lint`: constrains, respect.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it`: touches, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@agent-skills@milestone-split-by-lifetime`: constrains, respect.
  - `design@agent-skills@discussion-record-apart`: constrains, respect.
  - `design@agent-skills@plan-records-the-exchange`: constrains, respect.
  - `rejected@agent-skills@deliberation-in-the-searched-document`: constrains, respect.
  - `rejected@agent-skills@milestone-holds-every-slice-design`: constrains, respect.
  - `design@model@plan-document-directory`: constrains, respect.
  - `goal@projects-are-split-into-modular-units`: constrains, respect.
  - `design@agent-skills@harness-shell-fails-on-a-pipe`: conflicts, check-reversal.
  - `issue@agent-skills@a-command-chained-after-a-failing-check-recurs`: touches, respect.
  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `rejected@check@verdict-only-output`: constrains, respect.
  - `design@knowledge-architect@push-after-the-commits-check`: constrains, respect.
  - `design@agent-skills@refused-report-returned-as-text`: constrains, respect.
  - `design@agent-skills@version-line-names-the-main-commit`: constrains, respect.
  - `design@agent-skills@report-written-through-the-shell`: constrains, respect.
  - `design@agent-skills@synthetic-evidence-not-built`: constrains, respect.
  - `design@agent-skills@code-split-scout`: constrains, respect.
  - `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`: touches, respect.
  - `design@transcript@transcript-labels-in-code`: constrains, respect.
  - `design@transcript@transcript-format-tolerated-and-reported`: constrains, respect.
  - `rejected@transcript@fields-alone-place-every-line`: constrains, respect.
  - `design@agent-skills@unrecognised-entries-reach-upstream`: constrains, respect.
  - `design@agent-skills@transcript-extraction-is-the-checker-s`: constrains, respect.
  - `design@knowledge-architect@transcript-crate`: constrains, respect.

## searcher-script-overwrote-preamble: a searcher's fill script overwrote its report's preamble

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W5(a).

- **a. What it says.** Under W5, "Thin, from the compaction summary only": "A record searcher's script overwrote the preamble of its report. I hold no detail of the instruction or the fix." No text is blamed and no fix proposed.
- **b. Whether it still applies.** The preamble carries nothing the check needs: `judge_report` in `path@query@src/worklist.rs`, near line 1176, reads only the lines that open with `- ` before the closing sections, and the subjects come from `work.txt`, so an overwritten preamble is refused by nothing and loses nothing a later step reads. The script that wrote it is the fill by script the cluster searcher-fills-report-by-script analyses.
- **c. Whether it is already recorded.** As a case of `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` (`touches`, `respect`).
- **d. Its kind.** An observation, too thin to diagnose (the file holds no detail).

Default: instance added to the recorded issue, in the text of searcher-fills-report-by-script.
Outcome: decided by the session: instance added to `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`, as the last bullet of the text the cluster searcher-fills-report-by-script adds, with no other change. Ground: the overwrite caused no refusal and no loss, since the check reads no preamble, and it is one more failure of the per-searcher scripts that issue records.

## goal-citers-not-reread: nothing has a session read the citers of a reworded goal against its new text

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W1, C3.

- **a. What it says.** W1, missing: the whole-set review re-pointed every reference to a goal that
  moved or left, and `check` passed; "The review before the merge then found about 30 heads, issue
  entries and one source comment that still attributed to a goal a measure, a clause or a definition
  the rewritten goal no longer holds", "Five of its ten axes reported instances of this one class,
  each a different subset. The re-read of the repairs found three more." Cause: "The act 'a goal's
  text changed, so read what cites it' has no home". Proposal: in the whole-set review's step that
  writes ruled goals, and in goal-setting's step that writes one reworded goal, "for each goal whose
  text changed, read every text `cargo klarch show` ... lists against the new text, and reword each
  sentence that states what the goal no longer says" (+1). Nearest rival named: the review before
  the merge, against which "five reviewers each found part of the class and none the whole". C3, a
  blind spot and "the mechanical part of W1": "Three heads quoted a goal in quotation marks beside
  its reference, and the goal no longer held the quoted words"; `worklist --verify` checks a
  quotation against an entry's text, `check` does not; the file did not measure how many quotations
  beside a reference the record holds, "nor how many would be paraphrases the check would refuse
  wrongly".
- **b. Whether it still applies.** Yes, both. In `path@agent-skills@content/skills/goal-setting/SKILL.md`,
  section "When this runs again", at `v0.9.0-257-gcb8d5a22` and at `v0.10.0-59-g605d1538` alike,
  the one sentence on citers covers a removal only: "When one is removed, every reference to it
  dangles, and `{{command}} check` lists each one: each is a text that derived something from the
  goal, and is read again." The six commits to the file since the file's version, from
  "[agent-skills] A goal is a purpose, a scope or a ranking of the project, stated in the root's
  goals home alone, with no measure, no block of cases and no refinement" to "[agent-skills] The
  questions of a goal-review record take the prefix `G`", add none for a rewording. No lint of
  `check` compares a quotation with the entry its paragraph cites.
- **c. Whether it is already recorded.** No entry holds either. Bearing:
  `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries` (`touches`), a command
  that would list the citers of several goals at once, which W1's act would use;
  `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear` (`touches`,
  action none); `design@agent-skills@behaviour-additions-need-real-use` (`constrains`), below;
  `design@check@quotation-read-by-no-id-lint` (`constrains`): a quotation check stays outside the
  lints on ids and leaves that exemption standing; and
  `issue@knowledge-architect@the-checker-may-depend-on-every-crate-below-it` (`touches`): the
  matching a quotation check needs lives in `query`, where `worklist --check` holds it, and where
  `check` may get it is that issue's open question.
- **d. Its kind.** W1: missing, a conformance rule for a move the session has, `show`, and did not
  make; its default behaviour was systematically wrong in this session, about 30 sites and five
  partial subsets. C3: a blind spot of the checker, unmeasured.
- **e. Its scope.** W1: the installed goal-setting skill; `goal@records-reach-their-reader`, "a
  session changing the item can list what to read again". Its fix reverses no head: no design head
  states the citers' re-read for goals (a search of `register@agent-skills@design` for the sentence
  returns none). C3: the checker; `goal@documentation-stays-consistent`.
- **f. A better fix.** For W1, one home, not the two steps the file names: the paragraph of "When
  this runs again" on the commit that writes a round's goals is where both the single rewording and
  the whole-set review write goals, and it already holds the removal's repair. The rival the file
  names, the review before the merge, lost on the file's own fact: five axes each found a part of
  the class. The same act for a design head is already installed, in
  `path@agent-skills@content/skills/decision-recording/SKILL.md`, in the step after the check of a
  recorded change: "for each head the change edits, whatever the edit, run `{{command}} show` on it
  and read every text that cites it", with its reason, "an edit that keeps the slug dangles no
  reference, so no check reports a citing text it made false"; it entered with the commit "The
  analyses of the 0.6.0 release retrospective, with the owner's ruling". A goal rewording meets the
  same mechanism, and the added sentence follows that step's shape. The parallel shows the act is
  wanted for heads; it does not supply the owner's fact for goals, which the head on real use asks
  per addition. C3's check is no rival to W1: it reaches the three quotations, not the other sites, which the file reports as attributions and does not say were quoted.
- **g. Its route.** W1: a text edit of the installed source, under
  `skill@knowledge-architect-agent-configuration`, as `path@agent-skills@CLAUDE.md`, "Editing an
  installed skill or agent", says, read whole. C3: an issue only, since its cost and false
  positives are unmeasured.
- **h. The proposal.** W1 handled now, at the paragraph above, if the owner holds the fact the
  standard asks. `design@agent-skills@behaviour-additions-need-real-use` admits an instruction about
  how an agent behaves on "a behaviour seen in a real session ... with the owner naming what the
  session would have lacked without it". The behaviour was seen: the review's 30 sites. What the
  session would have lacked is the owner's to name: the default reads it as the list of citers to
  re-read before the review. C3: open an issue in `check`, kind `design`, slug
  `a-quotation-beside-a-reference-is-not-checked-against-its-entry`: the three heads; the mechanism,
  that `check` reads no quotation against the entry its paragraph cites; what is not measured, the
  count of quotations beside references in this tree and the share that paraphrase; the re-entry
  point, a design discussion after that census, or on the owner's word. Its `Why it matters` names
  `design@check@quotation-read-by-no-id-lint` and `goal@documentation-stays-consistent`. C3's
  outcome is the session's, whatever the owner rules on W1.

The new text of the paragraph that closes "When this runs again" in
`path@agent-skills@content/skills/goal-setting/SKILL.md`, the last sentence added:

```markdown
The goals of one round of rulings are written in one commit, whose message records each ruling,
attributed to the owner, as `primer@design-heads` says. A
goal's removal and the repair of every reference it leaves dangling are one commit, so the commit
passes the check. A reworded goal leaves every reference to it resolving, so the check lists none:
`{{command}} show goal@<slug>` lists each text that cites it, which is read against the new
wording, and a sentence that states what the goal no longer says is reworded in the same commit.
```

Instructions: 1 added, 0 removed. Nearest rival: the file's two sentences, one per step, which add
2 and give one act two homes; the fact that decides is that both steps write goals through this
paragraph's commit.

Default: W1 handled now with the text above; C3 opened as the issue
`a-quotation-beside-a-reference-is-not-checked-against-its-entry` in `check`.
Outcome: for the owner (a fact only the owner holds): pending. The question: what the session would
have lacked without the sentence, which `design@agent-skills@behaviour-additions-need-real-use`
asks the owner to name before an instruction about how an agent behaves is added.

## transcript-unlabelled-lines: `transcript` counts lines it leaves unlabelled, without locating them, and asks a finding for each

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow C1(b); 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow C3.

- **a. What it says.** goals-first-slice C1(b) (`v0.9.0-257`): an owner-typed `/compact` left
  unlabelled, "where the design gives such a line the label of an owner's command", and "4 lines are
  counted unlabelled". searcher-check-reviewer-reports C3 (`v0.9.0-354`): "`transcript list` reports
  four unlabelled lines across all sessions"; the header calls each "a gap of this command: report
  it as a finding on the checker in the retrospective"; "they were not located". No fix proposed.
- **b. Whether it still applies.** In part.
  - The `/compact` line is repaired: "[transcript] A command the harness writes with no `commandRun`
    is the owner's, named by its leading `<command-name>` tag" (`ba463928`, `v0.9.0-362`, in
    `v0.10.0`) labels a field-less `user` line opening with `<command-name>` as `owner-command`, as
    `design@transcript@transcript-labels-in-code` now says.
  - The count stands, and its lines are still not located. At origin/main,
    `cargo klarch transcript list --all-projects` prints `# unlabelled lines: 6`, with no file or
    line, where each unrecognised value gets "its count and its first file and line"
    (`design@transcript@transcript-format-tolerated-and-reported`, which gives `unlabelled` lines a
    count alone). `cargo x transcript-kinds` gives them one row, "top user unlabelled (no field): 6
    lines; versions 2.1.309, 2.1.321", with keys `promptSource`, `turnOrigin` and `turnPosition`. A
    `python3 -I` script over `~/.claude/projects/*/*.jsonl`, listing each top-level `user` line with
    `promptSource` and none of `origin`, `isMeta`, `isCompactSummary` and `toolUseResult`, finds
    exactly six: the first prompt of six headless sessions, each `promptSource` `sdk` and
    `turnOrigin` `sdk`, five of them probes a review of this repository ran with `claude -p`. None
    is in this project's transcript directory: each lies in the transcript directory the harness names
    after the scratch directory the probe ran in, so a run over this project alone, as `cargo klarch transcript list --branch
    planned-splits`, prints no `unlabelled` line, and only a run with `--all-projects`, or
    `cargo x transcript-kinds`, which reads every project's files, counts them.
  - These six are unlabelled by decision. `design@transcript@transcript-labels-in-code`: "A file
    with no field is a session run with `claude -p` ... its prompt carries `turnOrigin` `sdk` where
    a typed message carries `human`, and no field names who called `claude -p`, a person or a
    script, so its prompt is unlabelled and reported rather than given to the owner by a test on
    what it says". The decision entered with "[transcript] The reader places a fork's first line,
    an interrupt with no id by its whole text, and an owner's `!` command, and reads no tag in a
    file with no field" (`db80559b`, `v0.9.0-210`), as "its lines are unlabelled", and took the
    wording quoted, "its prompt is unlabelled", in "[transcript] Repairs of the review before the
    merge of the transcript reader's gaps, and the record of the review" (`4fa02a00`,
    `v0.9.0-212`), both before both members. That the four lines of
    2026-11-11 were of this kind is an assumption: the transcripts of that day are no longer told
    apart from the later ones by the count.
  - So the header's last line asks a finding for a line the design leaves unlabelled on purpose,
    in every run over all projects on a machine where `claude -p` ran:
    `design@transcript@transcript-format-tolerated-and-reported` ends the header, "where any of them
    is non-zero, with the line asking for it to be reported as a finding". Two findings of this
    batch, and the header of every run at origin/main, are that request met by decided lines.
- **c. Whether it is already recorded.** No entry records it.
  `rejected@transcript@fields-alone-place-every-line` (verdict `constrains`) keeps the text rows;
  `design@agent-skills@unrecognised-entries-reach-upstream` (verdict `constrains`) is the route the
  request serves, which a request met by decided lines weakens, since a reader learns to skip it.
- **d. Its kind.** A defect of the checker: two heads of `transcript` that, together, make the
  header ask a finding for a line the labels head decided to leave unlabelled; and a gap, since the
  header does not say where an unlabelled line is.
- **e. Its scope.** `goal@the-workflow-improves-through-real-use`, through
  `design@agent-skills@unrecognised-entries-reach-upstream`. The owner of the fix is `transcript`.
  Any fix rewrites `design@transcript@transcript-labels-in-code` or
  `design@transcript@transcript-format-tolerated-and-reported`, so it goes through
  `skill@knowledge-architect-design`.
- **f. A better fix.** Three shapes, none argued yet: a label of its own for a `claude -p` prompt,
  placed by its field `turnOrigin` `sdk`, which gives it neither to the owner nor to injected text,
  so no text test is made; the header counting such prompts apart, with no request; or the header
  naming the first file and line of the unlabelled lines, as it does for an unrecognised value,
  which locates them and keeps the request. The rival is no change, on the ground that the request
  is cheap; the deciding fact is that it fires in every run over all projects while the six lines exist,
  and in `cargo x transcript-kinds`, two sessions of this batch spent a finding on it, and searcher-check C3 could not locate them.
- **g. Its route.** An issue only: a design discussion and a branch of `transcript`.
- **h. The proposal.** Open a design issue in `transcript`,
  `a-claude-p-prompt-asks-for-a-finding-in-every-header`, with the measurement above, the three
  shapes and the two heads they would rewrite.

Default: open the issue.
Outcome: decided by the session: open an issue, `transcript`, kind `design`, `a-claude-p-prompt-asks-for-a-finding-in-every-header`; the `/compact` part needs no change, repaired by `ba463928`. Ground: the six lines are those `design@transcript@transcript-labels-in-code` leaves unlabelled on purpose, and `design@transcript@transcript-format-tolerated-and-reported` asks a finding for each; every fix rewrites one of the two heads.

## exempt-repairs-held-the-defects: the repairs the re-review rule exempts held all eight record defects of one review

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W2.

- **a. What it says.** The repair commit of the review before the merge changed about 40 heads and
  entries. Two changes met the re-review rule, "a head narrowed and three `rejected` entries moved".
  The decision-record axis, sent on the whole commit for those two, "reported eight findings and two
  lesser points. It judged the two changes that met the rule clean. Every finding was in a repair
  the rule exempts": two attributions to the owner that dropped a hedge or kept a pick whose premise
  had gone, a `rejected` reason widened from two goals to three, a statement answering another
  question, a rival said to lose both to a fork and to an argument, two count commands that did not
  give their figure. All eight were repaired and folded. It blames
  `skill@knowledge-architect-review@what-review-leaves`, "Any other repair, a rewording or an
  argument, a mention or a reference added inside an existing head included, and a title brought to
  what its head's body already argues, is reviewed by no axis again". Proposal: "none that I can
  argue on one instance"; the figure, 8 of 8 in exempt repairs and 0 in the two the rule sends back.
- **b. Whether it still applies.** The sentence is the same at cb8d5a22 (`v0.9.0-257`, the file's
  version), lines 170 to 173, and at origin/main, `path@agent-skills@content/skills/review/SKILL.md`,
  lines 180 to 183: "Any other repair, a rewording or an argument, a mention or a reference added
  inside an existing head included, and a title brought to what its head's body already argues, is
  reviewed by no axis again; one made before the last transcript review is still read by it."
- **c. Whether it is already recorded.** The rule is
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` (verdict `conflicts`,
  `check-reversal`). Its argument: "A trigger on any repair that "makes a decision" lost: whether a
  rewording makes one is a judgement, and read wide it re-reviews repairs that only reword a head.
  The rival that sends every axis again at every repair lost on the same cost." The cost of leaving
  a repair's defects to a later pass is `rejected@agent-skills@repair-defects-left-to-next-pass`
  (verdict `constrains`), measured on an audit: 16 of 38 findings sat on text earlier repairs had
  written. No issue or tripwire holds this evidence; `cargo klarch tripwires` lists none guarding
  the head.
- **d. Its kind.** An observation: the evidence that exempt repairs carry record defects is one
  instance, and in it no defect was lost, since the decision-record axis read the whole commit for
  the two changes that met the rule. Two of the eight, attributions to the owner, are also in the
  transcript reviewer's remit, which reads every repair made before it. Whether the exemption loses
  defects is not established: it would be by a review whose repairs met no trigger and whose record
  defects were found after the merge, or by a later round of the axis.
- **e. Its scope.** `goal@design-is-recorded-with-its-arguments`, which the decision-record axis
  serves; the installed review skill owns the text. A change would reverse the trigger of
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`.
- **f. A better fix.** Rivals: (1) no change, the head's cost argument standing; (2) a trigger read
  from the diff rather than judged, such as any repair commit that changes a design home or a
  `rejected` home goes to the decision-record axis alone, which the head's argument against a judged
  trigger does not cover and its argument against "every axis again" covers only in part; (3) the
  same widened to every axis. The fact that would decide between (1) and (2), the rate at which
  exempt repairs carry defects that reach the main branch, is not measured: one instance, with no
  loss. Text is not changed on one instance, per `skill@knowledge-architect-retrospective-intake@admission-rules`
  ("A cost one session met in one task is weighed against every other occasion the instruction
  serves"), and the reporter declined a proposal for the same reason. It pulls against
  `head-repair-rereview-unbounded`, which asks for fewer rounds: a wider trigger adds rounds.
- **g. Its route.** An issue only: the evidence must outlive the analysis, so that a second instance
  can reach it.
- **h. The proposal.** Open an issue in `agent-skills`, kind `observation`, slug
  `the-repairs-the-re-review-rule-exempts-held-every-record-defect-of-one-review`. Its content: the
  instance and its eight findings by class; the conditions (a repair commit changing about 40 heads
  and entries, the axis sent on the whole commit); that it does not reproduce a loss, since the
  axis read the exempt repairs anyway; what would make it so (a branch whose repairs met no trigger
  and whose record defect was found after the merge or by a later round); the head it strains,
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, and rival (2) above as the
  shape a discussion would weigh. Held by the session.

Default: open the observation issue above; no change to the review skill.
Outcome: decided by the session: open an issue (`agent-skills`, `observation`,
`the-repairs-the-re-review-rule-exempts-held-every-record-defect-of-one-review`). Ground: one
instance, with no defect lost, does not establish that the exemption of
`design@agent-skills@only-a-head-changing-repair-is-reviewed-again` loses defects, so no reversal is
argued; the evidence is kept where a second instance finds it, per
`skill@knowledge-architect-issue-tracking`.

## report-file-write-refused: the harness refuses a subagent's write of a report file by its name

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W5(b); 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow W5; 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow standing answer "A dispatched agent that returned unfinished"; 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow standing answer "An agent returned unfinished".

- **a. What it says.** goals-first-slice W5, second bullet (version `v0.9.0-257-gcb8d5a22`): "Named reviewer agents, like general-purpose ones, returned their reports as text: the harness refused their writes. `skill@knowledge-architect-review@review-invariants` says so of a general-purpose subagent only." No fix. searcher-check-reviewer-reports W5 (version `v0.9.0-354-gb97596f1`): the harness refused "`Write` of a new `report.md`, of a new `findings.md` and over an existing `report.md`", with "Subagents should return findings as text, not write report files", and allowed other names, `Edit` and a shell redirect; "Done instead. The installed texts now write a report file with a shell redirect, and every installed agent holds `Write`". 2026-11-12-knowledge-architect-new-crate-placeholder (version `v0.9.0-379-gf9137c97`), lines 102-104: "Several agents returned their report as text, since a write with their file tool was refused, as the review skill foresees for reviewers; the triage brief had not asked for a shell redirect." conformance-before-harvest (version `v0.10.0-35-g2abdaa89`), lines 93-94: "The scout could not write its report file: the harness refused it. It returned the report as text."
- **b. Whether it still applies.**
  - W5(b) and W5 are repaired, on main, by the commit "[agent-skills] A searcher checks its own report with `worklist --check`, every agent holds `Write`, and a reviewer writes its own report" (`git describe --tags 33f5c59d` gives `v0.9.0-355-g33f5c59d`; it is the oldest hit of `git log -S 'shell redirect' -- crates/agent-skills/content`). At origin/main, `path@agent-skills@content/skills/review/SKILL.md`, the bullet that opens "Every brief names a path for the reviewer's report", speaks of every subagent: "The harness can refuse a report file a subagent writes with its file tool, and lets a shell redirect write it".
  - The triage brief (v0.9.0-379, after the repair) was a brief the session wrote for a triage agent, not a reviewer. The review skill's sentence binds a review's briefs; no installed text tells a session what a brief for another kind of agent asks about the write of its report. The agent returned its report as text, the fallback the record keeps, and the file reports no content lost.
  - The code-split scout still applies, and is a defect of the installed text. Its definition at origin/main, `path@agent-skills@content/agents/code-split-scout.md`, section "Reporting", line 63: "Write the report as `report.md` in your scratch directory, and return it whole." It names the file name the harness refuses, and no means. The sentence entered with the definition, in "[agent-skills] The code-split scout, an installed agent hosting the module guidelines and the fragment on when to split" (`v0.10.0-24-gf2168859`, the one hit of `git log -S 'Write the report as' -- crates/agent-skills/content/agents/code-split-scout.md`), after the repair; the file's last change is `v0.10.0-31-g15350653` (`git log -1 origin/main -- <file>`), and `git show v0.10.0-35-g2abdaa89:<file>` holds the sentence at line 63, so the conformance session ran the sentence quoted. `grep -rn 'report\.md' crates/agent-skills/content | grep -v redirect` returns, besides it, the record searcher's lines, which already say "by `Edit` or through the shell", and the report steps of the three audit workflows under `path@agent-skills@content/workflows/`, each of which ends with "Return its path." and a constant that reads "Write it with a shell redirect" (or, in the agentic-workflow audit, that sentence inline).
  - Further instances, in this intake: the harness refused, with the message quoted above, a subagent's `Write` of a new file named `report-file-write-refused.md`, and of a new `summary.md`, in the drafters' scratch directories of several groups (per the intake's ledger); the verifier of this section took both again in its own scratch directory, both refused, while a `Write` of a new `verdicts.md` beside them passed. The refusal reaches names beyond the `report.md` and `findings.md` that `design@agent-skills@report-written-through-the-shell` measured, which its argument already foresees: "A file name the harness does not refuse depends on a pattern no documentation states".
- **c. Whether it is already recorded.** `design@agent-skills@report-written-through-the-shell` (agent 6, `constrains`, `respect`) holds the measurement and the rule: "A reviewer, a record searcher filling its `report.md`, and the report step of an audit's saved workflow write their report file with a shell redirect ... and fall back to returning the report as text where the harness refuses the write." The scout is not among its members. `design@agent-skills@refused-report-returned-as-text` (agent 4, `respect`) keeps text as the fallback only, against the always-as-text rival, which loses to a cut result. `design@agent-skills@code-split-scout` (agent 10, `respect`): "Its report is never committed"; the report stays in the scratch directory, which the repair keeps.
- **d. Its kind.** W5(b) and W5: wrong, already repaired. The triage brief: no defect; a brief for an agent outside a review is the session's to write, per `primer@room-to-judge`, and the fallback lost nothing. The scout: a defect of the text, wrong: following it meets the refusal the head measured.
- **e. Its scope.** The installed text: `agent@knowledge-architect-code-split-scout`, owned by `agent-skills`. No goal of the project's. The fix adds the scout as a member of `design@agent-skills@report-written-through-the-shell`, whose argument, the harness refusing a `report.md` by its name, covers it; per `primer@design-heads`, a member the argument covers is recorded directly. It reverses nothing.
- **f. A better fix.** The nearest rival: the dispatching skills, design and planning, have the scout's brief ask for a shell redirect, as the review skill does for a reviewer. The scout's definition itself names the file, so the means belongs beside it; two dispatch sites would be two homes for one act. A second rival, a file name the harness does not refuse, is rejected by the head: it "depends on a pattern no documentation states". For the triage brief, no change: a rule for every ad hoc brief would be a rule written for one interaction, and nothing was lost.
- **g. Its route.** A fix in the installed text, at its source under `path@agent-skills@content/`, as `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, with `skill@knowledge-architect-decision-recording` for the head's member; then the install into this repository's .claude.
- **h. The proposal.** Handle now. In `path@agent-skills@content/agents/code-split-scout.md`, section "Reporting", the first sentence becomes:

```markdown
Write the report whole as `report.md` in your scratch directory with a shell redirect,
`cat > <path> <<'EOF'`, since the harness can refuse a report file written with your file tool, and
return it whole.
```

  In `design@agent-skills@report-written-through-the-shell`, the first sentence's subject becomes "A reviewer, a record searcher filling its `report.md`, the code-split scout, and the report step of an audit's saved workflow". Instructions: 0 added, 0 removed, 1 reworded (the existing "Write the report" gains its means). Nearest rival: the brief naming the redirect, in two dispatching skills; it loses on two homes for one act.

  In the same head, the measurement's list of refused writes gains the two names measured here, so the sentence that opens "Claude Code refuses a subagent's `Write` of a new `report.md`" reads "of a new `report.md`, of a new `findings.md`, of a new `summary.md`, of a new file whose name begins with `report-`, and over an existing `report.md`". A measurement of a premise, in a design home: no instruction of agent-facing text changes.

Default: handle now, as above.
Outcome: decided by the session: W5(b) and W5 no change, already repaired by "[agent-skills] A searcher checks its own report with `worklist --check`, every agent holds `Write`, and a reviewer writes its own report"; the triage brief no change, the session's judgement, nothing lost; the scout handle now, its definition brought to `design@agent-skills@report-written-through-the-shell`, the scout added to that head's members, and the head's measurement given the two names refused in this intake. Ground: the head's measurement covers the scout's `report.md`, and its argument covers the new member, which `primer@design-heads` records directly.

## agreed-goal-reading-has-no-home: a reading of a goal the owner agrees to, with no text change, has no durable home

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W3. One session.

- **a. What it says.** In three rounds of the review of the whole goal set, the owner answered several questions by keeping a goal's text and agreeing to a reading of it, such as how far a ranking reaches. One reading went into an issue entry; the others stayed in the deleted record, in history and in one commit message. It blames `primer@decided-section` and the knowledge table's row for the goal-setting record, which leaves "the commit that writes the goals the owner ruled last". It proposes to reword the whole-set review's sentence so that such an answer is written into the goal's body as a sentence of the goal, or is said to bind nothing once the record leaves (0 added). It names its nearest rival, a register of goal readings, as withdrawn.
- **b. Whether it still applies.** The primer's row is unchanged at origin/main: `path@agent-skills@content/PRIMER.md` line 461 gives the record "one file per batch in docs/goal-reviews/" leaving with "the commit that writes the goals the owner ruled last". No text gives an agreed reading a home of its own.
- **c. Whether it is already recorded.** Yes, decided against the first branch of the proposal. `design@agent-skills@a-goal-carries-an-example-only-for-its-direction` (conflicts, check-reversal), `path@agent-skills@docs/design/goals.md`, line 42, says: "A reading that a decision needs is stated by the head that derives a constraint from the goal, where it is argued." Its rival, `rejected@agent-skills@goal-carries-reading-examples` (brings-back, new-argument-needed), "kept the cases so that the reading the owner ruled on is not lost when the review's record leaves", which is W3's motive word for word in substance; the head defeats it on the measurement that "every case either repeated its goal's statement or closed a question of how much". W3 brings no new argument: that no head has yet taken the reading is the case the rejected entry already covered, since its cases were also readings no head had taken. `design@agent-skills@standing-argument-in-head` (respect) defeats a register of interpretations, the rival W3 names. `design@agent-skills@goals-change-through-goal-setting` (respect) makes a sentence added to a goal's body a goal edit, ruled by the owner. The second branch, "said to bind nothing once the record leaves", states what the head already decides: a reading binds where a head states and argues it.
- **d. Its kind.** Already recorded: the gap W3 names is a decided absence.

Default: no change.
Outcome: decided by the session: no change, already recorded. Ground: `design@agent-skills@a-goal-carries-an-example-only-for-its-direction` keeps a reading in the head that derives a constraint from the goal, and its rejected rival `rejected@agent-skills@goal-carries-reading-examples` had W3's motive; no new argument is brought, per item c of `skill@knowledge-architect-retrospective-intake@what-to-establish`. A reading the owner wants to bind before any head takes it is a goal rewording, which goes through `skill@knowledge-architect-goal-setting`.

## milestone-document-outgrows-grounding: a milestone document that records every argument hides a passage the slice made false

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow W4.

- **a. What it says.** The milestone document of the goals milestone held 2,473 lines. Three
  commits of the slice edited it. A passage the slice made false, which listed a head as one the
  design rests on unchanged while the slice rewrote that head, survived all three edits. The
  design-conformance axis found it at the merge. The file blames
  `skill@knowledge-architect-planning@spec-contents`, which had the milestone document hold "every
  item that more than one slice's document cites" and an Arguments section of "one item per
  argument of the discussion [...] and its key words verbatim". It proposes no fix: "The instance is
  one document after one long discussion."
- **b. Whether it still applies.** No. The file's version is `v0.9.0-257-gcb8d5a22`. The
  milestone on split guidelines and plan directories, added by 1beacf9d (`v0.10.0-2`), made each
  plan document a directory and took the Threads and Arguments out of the document the searches and
  the grounding read. Its installed text landed in f93bc0d9 (`v0.10.0-7`), "[agent-skills] The
  installed text for plan documents as directories". On main, `path@agent-skills@content/skills/planning/SKILL.md`,
  in the section on what a spec holds, near line 210: "Threads and Arguments in `deliberation.md`,
  every other in `README.md`". Two heads now hold the arrangement:
  `design@agent-skills@discussion-record-apart` (the deliberation "made that document too large",
  about 80% of the bytes of the goals milestone's README) and
  `design@agent-skills@milestone-split-by-lifetime` ("a slice's spec holds what only that slice
  builds, and the README holds what crosses slices or outlives one"). The last slice of that
  milestone landed, and the milestone left in 7e180a76.
- **c. Whether it is already recorded.** Yes, as decided and built design: the two heads above,
  with `rejected@agent-skills@deliberation-in-the-searched-document` and
  `rejected@agent-skills@milestone-holds-every-slice-design` as their rivals. Every verdict against
  this subject is `constrains`, action `respect`
  (the verdict file of the search for this subject).
  `goal@projects-are-split-into-modular-units` fixes no rule of splitting, so no size bound on a
  document is owed on this instance.
- **d. Its kind.** A defect of the text, wrong, on one instance, at the version the file used.
- **h. The proposal.** No change: the defect is repaired by the milestone's work. The stale passage
  was found by the design-conformance axis before the merge, which is that axis's role.

Default: no change, already repaired.
Outcome: decided by the session: no change, already repaired. Ground: the arrangement the file
blames was replaced by `design@agent-skills@discussion-record-apart` and
`design@agent-skills@milestone-split-by-lifetime`, built in f93bc0d9; the planning skill on main
puts the deliberation in `deliberation.md`.

## push-chained-on-failing-check: a push that ran after a failing `commits` check

Members: 2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow (standing answer),
2026-11-11-knowledge-architect-issue-triage P1.

- **a. What it says.**
  - goals-first-slice, standing answer: "The session pushed once after a check whose output went
    through a pipe, against `primer@checker-model` ... The commit failed `commits`; it was amended
    and pushed again". The file does not name the separator before the push.
  - issue-triage P1: "A push chained after a failing check with semicolons; amended and
    force-pushed", against `instructions@git-workflow` point 1. It names
    `issue@agent-skills@a-command-chained-after-a-failing-check-recurs` and proposes no fix.
- **b. Whether it still applies.** The rule is unchanged at origin/main,
  `path@knowledge-architect@CLAUDE.md`, line 200: "**A commit is pushed only after `cargo klarch
  commits` has passed on it**: in a command of its own, or behind `&&` on the bare check. A push
  after `;`, or after a check whose output went through a pipe in a shell without `pipefail`, runs
  whatever the check found". `path@agent-config@settings.json` sets `"SHELLOPTS": "pipefail"`, line 3.
  The separator of the goals-first-slice push, from that session's transcript
  (session `221be2f0-cdd5-5e76-b391-13ea5eba2fec` of this repository, the Bash call of
  2026-11-10T21:48, its line 1884 counting from 1):
  `... && cargo klarch commits origin/main..HEAD 2>&1 | tail -1; git push -q 2>&1 | tail -1`, whose result was `PASSED: no findings` from the staged check, then `FAILED: 10 findings above`
  from `commits`; the next command, "Push the amended commit over the one that failed its check",
  is the repair. The separator was `;`.
- **c. Whether it is already recorded.** Yes:
  `issue@agent-skills@a-command-chained-after-a-failing-check-recurs`, read whole, verdict
  `touches`. It already lists the issue-triage instance, "the branch that triaged the issue record,
  2026-11-11 ... Reported by its session, not by a retrospective file", so that file adds no
  instance. The goals-first-slice push is not listed. The verdict on
  `design@agent-skills@harness-shell-fails-on-a-pipe` is `conflicts`, `check-reversal`, conditional
  on the separator: had it been `&&`, the head's measured premise, "a command chained after a piped
  verdict does not run over a failing check", would have failed. The transcript shows `;`, which
  the head says the option does not cover ("The primer's rule against that chain stays, for `;`"),
  so the premise holds and no reversal is owed. The verdicts on
  `design@knowledge-architect@push-after-the-commits-check`, `rejected@check@verdict-only-output`,
  `design@agent-skills@intake-shares-the-admission-standard`,
  `design@agent-skills@behaviour-additions-need-real-use` and `goal@only-needed-instructions` are
  `respect`: no outcome loosens the rule or rests on a verdict-only option.
- **d. Its kind.** A lapse of a clear instruction, recurring at one point of delivery, which the
  issue collects as the evidence for moving the delivery.
- **e. Its scope.** `goal@agents-work-without-drift`, which the issue cites. Nothing is reversed.
- **f. A better fix.** Rival: raising the issue's discussion now. It loses: its re-entry point is
  the discussion of
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
  or the owner's word, and `message-claims-written-unchecked` finds that discussion not raised by
  this intake's outcome on that subject.
- **g. Its route.** An issue already open: an instance added, and its counts brought up to date.
- **h. The proposal.** The issue gains the bullet below, and its two counts change: the summary's
  "nine instances in six sessions of this repository" becomes "ten instances in seven sessions of
  this repository", and "That is two pushes and seven commits in this repository" becomes "That is
  three pushes and seven commits in this repository". The triage bullet's last sentence, "Reported
  by its session, not by a retrospective file.", now false, becomes "Reported by its session, and
  by P1 of its retrospective project file."

Default: the issue gains the instance, its two counts, and the triage bullet's corrected source.
Outcome: decided by the session: an instance added to
`issue@agent-skills@a-command-chained-after-a-failing-check-recurs`, with its counts, and no
reversal of `design@agent-skills@harness-shell-fails-on-a-pipe`. Ground: the separator was `;`,
read in the session's transcript, which the head leaves to the primer's rule; the recurrence is the
issue's subject.

Text to add to the issue's list of instances, after the bullet on the branch that triaged the
issue record:

```
- the first slice of the goals milestone, 2026-11-10: one command ran the staged check, `git
  commit`, the `commits` check with its output read by `tail`, then `git push`, the push after a
  semicolon; `commits` failed on a branch SHA a committed file cited, the push went out, and
  the amended commit was force-pushed over it. Reported in the standing answers of its workflow
  file; the separator is read from the session's transcript. `pipefail` was set, and a semicolon
  is outside what it stops.
```

## standing-2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow, lines 135 to 161. The file
predates the questions on agents, misses, review cost and tools; it was written after the session's
third compaction.

Carried by a subject, named only:

- the push after a check whose output went through a pipe (lines 140 to 142):
  `push-chained-on-failing-check`;
- a false premise given to the owner in the second round (lines 145 to 148):
  `owner-premise-unchecked`;
- C2 and C4, and the reword of an older message by a script written each time (lines 156 to 159):
  `no-command-saves-subagent-report`, `named-repairs-applied-by-hand` and
  `no-command-rewords-branch-messages`.

Analysed here:

- **The tree edited while record searchers read it, at the design audit** (lines 143 and 144):
  "The searchers graded from snapshot files; one reported the edits." Kind: a lapse with no harm
  found. It is the dispatcher's side of the interaction the subject
  `searcher-build-broken-by-live-edit` analyses (C3 and P1 of transcript-reader-gaps), where the
  installed text "tells the dispatcher nothing"; a further instance for that subject's count.
- **A reviewer wrote one file under `/tmp` against its brief** (line 149), "No rule of the primer".
  Kind: a lapse against the owner's global instructions, carried by the brief. No installed text.
- **The project's section on editing an installed skill not read again after the compaction**
  (lines 150 to 153), for repairs that "removed one clause and restated one sentence with its
  pointer". Kind: an observation the session could not verify; no defect found in those repairs.
- **Backticks** (lines 154 and 155): a slice named as a hash sign and its id beside its milestone's
  reference, "which the primer admits", per `primer@where-knowledge-goes`. **Installed file
  changed**: as the task. **Code rules**: no Rust beyond two doc comments. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: the live-tree edit is counted under
`searcher-build-broken-by-live-edit`, where its outcome is given; the rest are lapses against a
brief or observations, which the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` does not repair by text.

## named-repairs-applied-by-hand: the checker names each repair of a reference, and sessions apply them by scripts

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `named-repairs-applied-by-hand`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## no-command-saves-subagent-report: the dispatcher saved each reviewer's report from the message, by hand or by a script over transcript files

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `no-command-saves-subagent-report`, which gives the outcome.

## session-owner-line-unrecognised: the transcript reader reports the harness's `session-owner` line type as unrecognised

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow.md`, in its section on `session-owner-line-unrecognised`, which gives the outcome.

## owner-premise-unchecked: questions put to the owner on premises the session had not checked

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md`, in its section on `owner-premise-unchecked`, which gives the outcome.
