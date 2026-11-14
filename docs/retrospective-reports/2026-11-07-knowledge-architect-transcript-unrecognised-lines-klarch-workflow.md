# 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow

- **Received file**: 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: `v0.9.0-209-gdd13763a`, main commit `dd13763a` when the work began; the branch `transcript-unrecognised-lines`; the tree held no uncommitted change."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@xtask@a-local-task-is-the-maintenance-tool-s`: constrains, respect.
  - `design@xtask@internals-included-not-exported`: constrains, respect.
  - `rejected@xtask@helper-exported-from-a-published-library`: constrains, respect.
  - `rejected@xtask@line-kinds-a-verb-of-transcript`: constrains, respect.
  - `design@transcript@transcript-query-verbs`: conflicts, check-reversal.
  - `design@transcript@format-classed-on-the-inventory`: constrains, respect.
  - `design@agent-skills@transcript-extraction-is-the-checker-s`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@query@query-depends-on-no-check`: constrains, respect.
  - `design@query@worklist-writes-and-verifies-the-reports`: conflicts, check-reversal.
  - `design@query@occasion-shaped-output`: conflicts, check-reversal.
  - `rejected@query@gather-reads-the-model`: brings-back, new-argument-needed.
  - `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`: touches, respect.
  - `design@agent-skills@verdicts-reach-the-conformance-review`: constrains, respect.
  - `design@agent-skills@plan-commits-the-generated-lines`: conflicts, check-reversal.
  - `rejected@agent-skills@search-output-committed-whole`: constrains, respect.
  - `design@agent-skills@plain-text-is-no-repair`: conflicts, check-reversal.
  - `design@check@unanchored-path-first-segment-here`: constrains, respect.
  - `design@check@a-commit-message-is-a-document`: constrains, respect.
  - `issue@check@no-command-applies-the-repair-a-message-finding-names`: touches, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `design@knowledge-architect@cli-crate`: constrains, respect.

## premortem-written-after-code: code written on the in-change path before the premortem's causes

Members: the standing answer on a rule of the primer of 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow; the same answer of 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.

- **a. What it says.** Two sessions on the in-change path wrote code before writing the premortem's
  causes: "the session wrote the code before it wrote the premortem's causes, which it ran before the
  commit. One lapse, no consequence found" (transcript-unrecognised-lines); "The session started
  writing the code before it had written down the premortem's causes. [...] The session ran the
  premortem in its reasoning before the first edit and committed after writing it, so the committed
  record holds it. Nothing else depended on the order" (primer-import-first-line). Both quote the
  design skill's "nothing of the decision is implemented or committed before it has run". Neither
  proposes a fix.
- **b. Whether it still applies.** The instruction stands at both versions, `dd13763a` and
  `d687aa43`, and at origin/main, `v0.10.0-59-g605d1538`, in
  `skill@knowledge-architect-design@decision-authority`: "Where the premortem runs, nothing of the
  decision is implemented or committed before it has run, so the record that carries the
  deliberation carries its causes and its criteria."
- **c. Whether it is already recorded.** No entry records it; `goal@only-needed-instructions`
  (constrains): "A case that judgement handles well enough gets no instruction".
- **d. Its kind.** A lapse of the session, twice: the instruction exists and is clear, and both
  files say so. In both, the premortem ran before the commit, and the files report no consequence
  ("no consequence found"; "Nothing else depended on the order"). The instruction's stated reason is
  that "the record that carries the deliberation carries its causes and its criteria", which the
  second file confirms of its commit ("the committed record holds it"). A
  lapse is not repaired by adding text, per `skill@knowledge-architect-retrospective-intake@admission-rules`.
  The rule's exception, a lapse recurring at one point of delivery because the instruction is read
  at one moment and needed at another, is not shown: on the in-change path the change's code is under
  way through the whole discussion, so the instruction is needed from the path's choice on, which is
  where `skill@knowledge-architect-design@decision-authority` delivers it; neither file names a later moment a move would reach.

Default: no change.
Outcome: decided by the session: no change, a lapse twice with no consequence. Ground: item d; the
instruction is clear and delivered where the path is chosen, and no move of it is shown to reach a
moment it misses.

## unread-may-bear-list-fails-commits: the list of verdicts with the searchers' reasons is a hand-built join that fails `commits`

Members: 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow W3, C2; 2026-11-11-thaum-split-oversized-modules-klarch-workflow C3; 2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow C2; 2026-11-12-thaum-triggers-design-klarch-workflow C4.

- **a. What it says.** The design and planning skills asked a commit message or a plan document to
  carry a list of the search's entries with the searchers' reasons, which no command wrote and whose
  reasons `commits` refused: 64 `may-bear` reasons copied into a message failed `commits` on 10
  findings (transcript-unrecognised-lines W3, C2); a script extracted 45 unread entries and garbled
  two reasons, bare paths and two-segment `path` spans refused (thaum split C3); two scripts merged
  188 lines with reasons and applied 65 repair lines for a plan's list of every verdict with its
  outcome (dispatch-judgement C2); a script built 331 entry lines and 20 rows of referencing texts for
  "What is already decided" (thaum triggers C4). The blamed texts: the design skill's "the `may-bear`
  entries the grounding did not read, each with the searcher's reason"; the planning row "What is
  already decided", "each entry the discussion's search gave a verdict other than `unrelated`, with
  its verdict, its action and its outcome" (dispatch-judgement's version) or "for a `may-bear` entry
  the grounding did not read, 'not read' with the searcher's reason" (thaum's). Fixes proposed: a
  `worklist --unread` mode; `--verify` refusing a reason not in checked forms, or writing the plan's
  list with reasons; a skeleton with empty outcomes; a `show` mode printing the referencing paths of
  several references (thaum triggers C4).
- **b. Whether it still applies.** The list of reasons: no, repaired in two steps.
  - The unread `may-bear` list left with the grades, in the commit "[agent-skills] The installed text
    runs the record search on the draft, with subjects, and acts on its verdicts; the README of `cli`
    describes the verdict report" (48f64a10, on main, `v0.10.0~217`); `grep -rn "may-bear"
    crates/agent-skills/content/` returns nothing at origin/main, `v0.10.0-59-g605d1538`. The
    commit message now carries `generated-lines.md`, which `design@query@occasion-shaped-output`
    writes "with no quotation and no reason, for the commit message the occasion owes".
  - dispatch-judgement's version, `7173deed`, held the verdict shape, and its planning row read, by
    `git show 7173deed:crates/agent-skills/content/skills/planning/SKILL.md`, "each entry the
    discussion's search gave a verdict other than `unrelated`, with its verdict, its action and its
    outcome". At origin/main, the row in `path@agent-skills@content/skills/planning/SKILL.md` reads
    "each entry the discussion's search gave a verdict whose outcome changed something, the design, a
    thread or the work, with that outcome, and a link to `search/discussion.txt` for the other
    verdicts", from the commit "[agent-skills] The installed text for plan documents as directories"
    (f93bc0d9, on main, after `7173deed`).

  The referencing texts: still applies. The same row at origin/main still asks for "each recorded
  decision the work reverses or rewrites, with every text that `{{command}} show` lists as
  referencing it", one `show` per decision, which thaum triggers C4's 20 rows were built from.
- **c. Whether it is already recorded.** The reasons are not kept, by
  `design@agent-skills@plan-commits-the-generated-lines`: "the searchers' reasons are not kept, since
  a reason that changed something is carried by the outcome the plan document records". It is graded
  `conflicts` against dispatch-judgement C2 and thaum triggers C4's reason-carrying list, and its
  rival is `rejected@agent-skills@search-output-committed-whole` ("It kept every searcher's reason
  beside the plan document"). A `--verify` that reads the model to judge reasons brings back
  `rejected@query@gather-reads-the-model`, with no new argument. The referencing texts of several
  entries are the first work list of `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`
  (touches), which already records one instance of the planning row run by a loop of `show`.
- **d. Its kind.** For the list of reasons: a defect of the text, missing, already repaired. For
  thaum triggers C4's referencing texts: a missing command, an instance of a recorded issue.
- **e. Its scope.** The installed text for the list, the `query` Component for the command, per
  `design@knowledge-architect@query-crate`.
- **h. The proposal.** No change for the list of reasons. Add thaum triggers C4's second part as an
  instance of `issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`, in the
  paragraph of further instances of the first list, in the commit that writes the analysis:

  ```text
  A further instance of the first list: C4 of 2026-11-12-thaum-triggers-design-klarch-workflow, a design discussion of another project carried to a milestone plan at the pin `=0.9.0`: for the section "What is already decided", the session built by script, from repeated `show` runs, the 20 rows of texts referencing the decisions its work rewrites, and proposed a `show` mode printing only the referencing paths of several references. Analysed at the intake of forty-seven received files, 2026-11-14.
  ```

Default: no change for the list of reasons; thaum triggers C4's referencing texts added as an
instance of the recorded issue.
Outcome: decided by the session: no change for the list of reasons, already repaired by 48f64a10
and f93bc0d9; the referencing texts of thaum triggers C4 added as an instance of
`issue@query@no-command-lists-the-standing-entries-citing-a-set-of-entries`. Ground: item b's
quotations at each version and at origin/main; `design@agent-skills@plan-commits-the-generated-lines`
keeps no reason, so the reason-carrying proposals are not made.

## transcript-key-measurement-by-pipeline: measuring the keys of a transcript line kind took `jq` pipelines and a Python script

Members: 2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow C3.

- **a. What it says.** Measuring a line kind's keys, before classing a value, took four `jq`
  pipelines and a Python script; a recurrence of the project's own issue on that measurement.
  Proposed fix: "the verb that issue names".
- **b. Whether it still applies.** No. "[transcript] The reader names a cut text, refuses or reports
  the inputs it read as answers, starts `show` at a line number, and counts its line kinds for the
  maintenance tool" (`5b05b9dc`, `v0.9.0-361`, in `v0.10.0`), after the member's `v0.9.0-209`, added
  `cargo x transcript-kinds` and closed `no-verb-takes-the-inventory-of-the-transcripts-line-kinds`.
  At origin/main it prints, per line kind, its versions and each key with its count, as in "top
  system api_error: 6 lines; versions 2.1.321; keys cwd 6, entrypoint 6, error 6, ...".
- **c. Whether it is already recorded.** The measurement is a command of the maintenance tool, not a
  verb, per `rejected@xtask@line-kinds-a-verb-of-transcript` (verdict `constrains`): the verb the
  finding asks for is that rejected design, argued and lost.
  `design@transcript@format-classed-on-the-inventory` (verdict `constrains`) has a value classed on
  that inventory.

Default: no change.
Outcome: decided by the session: no change, already repaired. Ground: `5b05b9dc` built `cargo x transcript-kinds`, after the member's version; the verb proposed is `rejected@xtask@line-kinds-a-verb-of-transcript`.

## standing-2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow, lines 89 to 117. The
file predates the questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- a doc comment that dated the interrupts from a closed issue, and a message saying a closed entry
  was "now named by its subject" where it was described in words (lines 95 to 98):
  `message-claims-written-unchecked`;
- the code written before the premortem's causes (lines 105 to 107): `premortem-written-after-code`;
- the key-set measurement, the list of unread entries and the dispatch of the searchers over five
  messages (lines 113 and 114): `transcript-key-measurement-by-pipeline`,
  `unread-may-bear-list-fails-commits` and `searcher-dispatch-exceeds-harness-cap`.

Analysed here:

- **The reversal check carried in part** (lines 99 to 104): "the session grepped the words of the
  half of the title it changed, 'substring', and not 'by its fields', and did not read the citing
  texts of the head before the first commit. Six reviewers found the 12 restatements of the old
  title." Kind: a lapse against `skill@knowledge-architect-decision-recording@reversal-check`, which
  the file itself classes "One lapse, caught by its review: no finding on the text". Two more
  sessions of the batch carried that check in part: the goals-second-slice session stated no search
  in its reversal's message (its standing answer, lines 98 to 100), and the finding-repairs session
  judged an installed sentence still true after its reversal (its standing answer, lines 89 to 91).
  Each lapsed on another step of the check, so no one point of delivery recurs; the check's own
  lists are weighed by the subject `reversal-lists-omit-readme`, to which these three are further
  evidence.
- **A default awaiting the owner written into a head, then removed** (lines 108 to 110), "when the
  record search returned the entry, before the commit". Kind: a lapse, repaired by the session's own
  procedure before any commit.
- **Installed file changed** (lines 91 to 93): the transcript reviewer's source, as the work.
  **Project skill addition missed**: none. **Backticks**: none. **Code rules** (lines 115 to 117):
  logic tested through the binary alone was given unit tests "at no cost the change felt".
  Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: each is a lapse against an instruction that
exists and is clear, caught before the merge, or an observation; the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules` moves an instruction only where a
lapse recurs at one point of delivery, and the three lapses of `skill@knowledge-architect-decision-recording@reversal-check` of the batch fell on
three different steps.

## bears-file-too-large-to-read-whole: the grounding's "read whole `bears.md`" cannot be followed when the graded set exceeds one session's reading

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `bears-file-too-large-to-read-whole`, which gives the outcome.

## searcher-dispatch-exceeds-harness-cap: the record searcher's dispatch asks for every agent at once, above the harness's cap

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `searcher-dispatch-exceeds-harness-cap`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
