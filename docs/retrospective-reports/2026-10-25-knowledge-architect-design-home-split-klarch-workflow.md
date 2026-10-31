# 2026-10-25-knowledge-architect-design-home-split-klarch-workflow

- **Received file**: 2026-10-25-knowledge-architect-design-home-split-klarch-workflow.md, the
  workflow file of this repository's session that moved two design homes to the directory shape.
- **Version used**: built from its own source, main at `v0.8.0-95-g85858a7`.
- **Analysed against**: main at `v0.8.0-224-g7e3784aa`.
- **Standing entries that bear on it**: `design@core@heading-register-two-shapes`,
  `design@agent-skills@subject-split-standard`, `design@core@register-home-reference`,
  `design@core@clusters-packed-under-a-word-bound`,
  `issue@core@the-transcript-reader-does-not-know-the-lines-remote-control-writes`,
  `issue@core@the-transcript-reader-does-not-know-three-kinds-of-harness-version-2-1-312`,
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
  `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`.

## W1: when a design home "has outgrown one file"

- **a.** `skill@knowledge-architect-decision-recording@current-design` gives a design home the
  directory shape once the design "has outgrown one file", with no test. The only size test the text
  states is the record search's word bound, which decides a split of subjects and which the
  directory shape does not change. The session had to find that the owner meant a file longer than
  a session reads in one pass. The proposal rewords the sentence to say what each shape serves.
- **b.** Still applies: `path@agent-skills@content/skills/decision-recording/SKILL.md`, lines 74-75,
  unchanged.
- **c.** No issue. `design@core@heading-register-two-shapes` uses the same word, "has outgrown it";
  `design@agent-skills@subject-split-standard` covers the subject split only.
- **d.** Unclear: the word allows two tests, and the text gives the reader no sign that they answer
  different questions.
- **e.** The installed text. It reverses nothing; it states the test the word left implicit.
- **f.** Rival: a fixed line count. It loses: how much a session reads in one pass is the harness's,
  and the installed text works in any harness, per `goal@agent-skills@installed-text-works-anywhere`.
- **g.** The installed text at its source. 0 instructions added: a rewording.
- **h.** Reword the sentence: a directory once the file is too long for a session to read whole in
  one pass; the directory shape changes no cluster of the record search, which a split of subjects
  changes.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: a word that allows two tests is a defect
  provable by reading, per `skill@knowledge-architect-retrospective-intake@admission-rules`, and the
  rewording states the test the owner confirmed in the session.

## W2: the horizon occasions count a relocation

Analysed with P1 of 2026-10-25-knowledge-architect-design-home-split, in
`path@knowledge-architect@docs/retrospective-reports/2026-10-25-knowledge-architect-design-home-split.md`.

## W3: the condition of the decision-recording review axis

- **a.** `skill@knowledge-architect-review@review-axes` sends the decision-recording axis when "a
  decision was made, reversed or harvested". The session skipped it on a branch that held choices it
  listed under "Decided", and the transcript reviewer reported the contradiction. "A decision" reads
  as the primer's sense, anything the owner could have decided, or as a design decision, the
  reviewer's own scope. The proposal: "a design decision was made, reversed or harvested: a design
  head or a `rejected` entry written, rewritten or removed, other than by a move".
- **b.** Still applies: `path@agent-skills@content/skills/review/SKILL.md`, line 41, unchanged.
- **c.** `rejected@agent-skills@two-readings-left-to-judgement` rejects leaving a confirmed case of
  two readings as it is. `design@agent-skills@new-or-reshaped-head-needs-design` already says a
  change that only relocates or rewords recorded decisions "is not a decision".
- **d.** Unclear: two readings, one of them the primer's.
- **e.** The installed text. The decision-record reviewer judges which decisions earned an entry,
  a decision that earned none included, and each tripwire a diff adds, so the proposed wording, a
  design head or a `rejected` entry written, would drop both from the condition.
- **f.** Rival: send the axis on every branch. It loses: a branch that makes no decision about how
  the project is built and adds no tripwire gives the reviewer nothing to judge. The proposed
  wording loses to a wording that keeps what the reviewer judges.
- **g.** The installed text at its source. 0 instructions added.
- **h.** Reword the condition: a decision about how the project is built was made, reversed or
  harvested, whether or not it earned an entry, or a tripwire was added; a change that only
  relocates or rewords recorded decisions makes none.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now, with the wording of item h rather than the
  file's. Ground: a confirmed case of two readings is repaired, per
  `rejected@agent-skills@two-readings-left-to-judgement`, and the reviewer's scope, read while
  carrying the outcome out, fixes which reading is meant.

## C1, with C2 of the graded-entries file, C1 of the primer-terms file and C1 of the met-outside file: the transcript reader does not know some lines

- **a.** `klarch transcript` reports line kinds it does not recognise and asks for each to be
  reported: `bridge-session`, `bridge_status` and a `local_command` line, written under
  `/remote-control`; and `task_reminder`, `stop_hook_summary` and `tools-used`.
- **b.** Still applies. A run of `cargo klarch transcript list --all-projects` on this machine at the
  analysed commit still reports `bridge-session`, `bridge_status`, the `local_command` shape and
  `tools-used` as unrecognised; the source holds neither `task_reminder` nor `stop_hook_summary`.
  The same run reports `model_refusal_no_fallback`, which no file and no record names.
- **c.** Recorded: the first three in
  `issue@core@the-transcript-reader-does-not-know-the-lines-remote-control-writes`, the other three in
  `issue@core@the-transcript-reader-does-not-know-three-kinds-of-harness-version-2-1-312`. The first
  issue warns that the `local_command` line is a command the owner typed, against this file's
  proposal to treat all three as harness lines.
- **d.** A defect of the checker, already recorded.
- **Default**: no change for the recorded kinds; open an issue for `model_refusal_no_fallback`.
- **Outcome**: decided by the session: no change for the six recorded kinds; an issue of the core is
  opened for the new subtype. Ground: each recorded kind has its entry, whose closing condition is
  not met; the new subtype is a gap the command asks to have recorded, per
  `skill@knowledge-architect-issue-tracking`.

## C2: `worklist` cannot print each group's load

- **a.** To see which subjects are near the word bound, the session ran
  `worklist --bound 1`, which writes one agent directory per group, unused. The default output names
  each agent's total only.
- **b.** Still applies: `path@core@src/cli/worklist.rs`, line 153 prints each agent's total.
- **c.** No issue, no `rejected` entry. `design@core@clusters-packed-under-a-word-bound` records only
  the notice for a group above the bound.
- **d.** A missing capability of the checker.
- **e.** The core. It adds output to a command, which
  `issue@knowledge-architect@command-output-is-not-declared-a-contract` leaves unclassed.
- **g.** A change to the Rust source, which needs a branch of its own.
- **Default**: open an issue.
- **Outcome**: decided by the session: open an issue, of kind `todo`, in the core. Ground: the
  question a design-home split asks, which subjects are near the bound, follows from the tree with
  no judgement, per `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`.

## C3: `show` on a home in the directory shape prints only its path

- **a.** `cargo klarch show register@core@design` prints the directory and the references to it, not
  its subdocuments, so "read `register@core@design`" sends a reader to a directory with no list.
- **b.** Still applies.
- **c.** `design@core@register-home-reference` states the current output as decided; no issue.
- **d.** A missing capability of the checker. The fix adds to a head.
- **g.** A change to the Rust source and to that head, which needs a branch of its own.
- **Default**: open an issue.
- **Outcome**: decided by the session: open an issue, of kind `todo`, in the core. Ground: the
  addition changes what `design@core@register-home-reference` decides, which goes through
  `skill@knowledge-architect-design` with the code.

## The standing answers, with the claim-check lapses of six files

- **a.** The session told the owner an unchecked claim on `git blame`, and two commit messages made
  claims that failed. Five other workflow files of this batch report the same lapse of
  `primer@verify-before-relying`, "Verify a claim before writing it": the graded-entries file, four
  claims in commit messages; the grounding-worklist file, W4's blanket attribution; the
  transcript-command file, W1, with a proposal; the tripwire-standard file, commit messages and an
  answer to the owner; the primer-terms file, commit messages. Three of the six include a false
  answer to the owner. The transcript-command file proposes that `cargo klarch commits` print one
  line naming what it does not judge.
- **b.** The primer's subsection is unchanged at each file's version and at HEAD. `commits` prints
  no such line: `path@core@src/cli/history.rs`, lines 1039-1052 prints the verdict last.
- **c.** Recorded:
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  is this subject. Its closing condition is a discussion that picks a delivery at the commit, then a
  count that falls. A line printed by `commits` is not one of the candidates it lists.
  `rejected@core@commit-msg-hook` and `rejected@core@commit-subcommand` do not cover it: the line
  wraps nothing and judges no draft. `design@core@a-commit-message-is-a-document` constrains it, and
  `path@core@README.md`, line 80 keeps the verdict as the last line, so such a line goes above it.
- **d.** A lapse that recurs at one point of delivery: six sessions in this batch.
- **e.** The installed text and the checker. Whether to deliver the check at the commit is the
  issue's discussion.
- **f.** Rival: no change, the lapse left to review. Each of the six sessions had a reviewer or the
  owner find the false claim after the commit. The issue's discussion decides between the candidates.
- **g.** An issue already open.
- **h.** Add the six instances and the new candidate, with the constraint of the last line, to the
  issue. The other lapses of these answers, the listings not run before diagnosing and the bare
  `docs/` in a commit message, which no check reads, were not written to clear a finding and meet
  neither entry.
- **Default**: the issue gains the instances and the candidate.
- **Outcome**: decided by the session: the issue gains the six instances and the candidate. Ground:
  the recurrence is the issue's subject, and its discussion needs the count and every candidate.
