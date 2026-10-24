# Report of pass 1 of the tripwire axis

Scratch root, named `<scratch>` below:
/home/teo/.cache/claude-tmp/claude-1000/-home-teo-dev-knowledge-architect/00654c6d-ba37-4196-bc80-21e64964b15d/scratchpad/audit1.
A draft is named `<group>/<nn>`, the file `<scratch>/<group>/<nn>-<id>.md`. Group abbreviations:
as1 = agent-skills-1, as2 = agent-skills-2, pk = packed-1. This is a first pass, not a re-check:
no `previous`, no kept list, no old/new classification.

## 1. Header

**Commit audited**: the commit of the spec's branch
("[review] Repairs of the review of the tripwire standard's work, steps 1 to 3, AC1 fired and
answered, and the owner's rulings on D3 and on a doubt's weight").

**Agents of each step** (crates/agent-skills/content/workflows/tripwire-audit.js; `AUDITOR` is
`agent@knowledge-architect-tripwire-auditor`, line 36):

| step | agent | output |
| --- | --- | --- |
| preflight | a general agent (no `agentType`) | `<scratch>/preflight/` (tripwires.txt, entries.json, issues.txt, history.txt, reports.txt) |
| setup | a general agent | `<scratch>/setup/groups.json`, one worktree per group under .claude/worktrees/audit1/ |
| drafts | one `agent@knowledge-architect-tripwire-auditor` per group, 3 | `<scratch>/<group>/<nn>-<id>.md`, `<scratch>/<group>/return.md` |
| calibration | one `agent@knowledge-architect-tripwire-auditor` | `<scratch>/calibration/calibration.md` |
| verify | one `agent@knowledge-architect-tripwire-auditor` per group, 3 | `<scratch>/verify/<group>/verdicts.md` |
| cross-check | a general agent | `<scratch>/cross-check/cross-check.md` |
| report | a general agent (this file) | `<scratch>/report/report.md` |

**Groups** (`<scratch>/setup/groups.json`):

| group | first entry | last entry | entries | file |
| --- | --- | --- | --- | --- |
| agent-skills-1 | `tripwire@agent-skills@a-comment-escalation-is-overruled` | `tripwire@agent-skills@goal-missed-though-graded` | 22 | crates/agent-skills/docs/tripwires.md |
| agent-skills-2 | `tripwire@agent-skills@head-created-without-deliberation` | `tripwire@agent-skills@verifiers-wave-findings-through` | 21 | crates/agent-skills/docs/tripwires.md |
| packed-1 | `tripwire@core@candidate-rule-silence` | `tripwire@xtask@tip-alone-is-built` | 21 | crates/core/docs/tripwires.md (20), tools/xtask/docs/tripwires.md (1) |

**Sample** (5 entries), each with its draft of record: `tripwire@agent-skills@ruling-lost-in-assembly` and
`tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap` (as2/14, as2/17); `tripwire@core@grammars-not-prefixes`,
`tripwire@core@walked-count-differs-between-machines` and `tripwire@xtask@tip-alone-is-built` (pk/07, pk/20, pk/21). The
other groups' sample drafts are not of record: as1/23 to 27, as2/22 to 24, pk/22 and 23. pk/24,
25 and 26 differ from pk/07, 20 and 21 "in their first line alone (`diff`)" (calibration, line 11);
re-run for this report, `diff` prints only the heading line for each pair.

**Counts**

- Entries: 64 (22 + 21 + 21), one draft of record each. The cross-check: "That is 64 entries,
  one draft of record each, which is every line of `preflight/tripwires.txt`."
- Drafts of record per outcome, as drafted:

  | outcome | as1 | as2 | pk | total |
  | --- | --- | --- | --- | --- |
  | reworded | 11 (01, 02, 05, 06, 07, 12, 17, 18, 19, 21, 22) | 13 (01, 03, 06, 07, 09, 10, 11, 12, 13, 14, 18, 20, 21) | 8 (01, 02, 04, 06, 08, 10, 14, 18) | 32 |
  | kept | 4 (03, 13, 15, 16) | 5 (02, 04, 05, 15, 17) | 1 (16) | 10 |
  | removed | 5 (04, 08, 09, 11, 14) | 2 (08, 19) | 10 (03, 05, 09, 11, 12, 13, 15, 17, 19, 20) | 17 |
  | converted | 2 (10, 20) | 0 | 2 (07, 21) | 4 |
  | fired | 0 | 1 (16) | 0 | 1 |
  | absorbed, decision gone, horizon passed | 0 | 0 | 0 | 0 |

- Proposed outcomes after the verifiers' repairs, the calibration and the cross-check
  (cross-check part 3): reworded 36, kept 9, removed 14 (condition 1: 10; condition 3: 4;
  condition 2: 0), converted 4, fired 1.
- Verifier verdicts: verified 53 (as1 18, as2 19, pk 16); refuted 6 (as1 2, as2 1, pk 3);
  plausible 5 (as1 2, as2 1, pk 2).
- Marks, as the verifiers give them:
  - bearing on a ruling: 1 (as1/07);
  - firing: 6 (as1/02, as1/07, as2/16, pk/01, pk/06, pk/18);
  - known: 8 (as1/07; as2/03, 06, 07, 13; pk/01, 02, 04);
  - on the kept list: 0 (first pass, no kept list).
- Re-check counts of `skill@knowledge-architect-project-audit@record-of-an-audit`: not owed, this
  is a first pass. For the pass's commit message, the figures that section asks of every pass
  are left to the session (the count of rulings against the default exists only after the
  owner's rulings).

**Entries missing from a return**: none. Each return lists every entry of its group (checked
for this report by matching the 64 ids of groups.json against the three return.md files: no id
missing).

**Refuted drafts, with the verifier's reason** (quoted from `verify/<group>/verdicts.md`):

| draft | entry | reason |
| --- | --- | --- |
| as1/03 | `tripwire@agent-skills@a-shortcut-decision-earns-a-head` | "\"at the second instance\" asks for no separate pieces of work: two commits on the bounded path in one branch would fire it. The draft's \"each instance is a commit of a separate piece of work\" does not follow from the clause. Under the slip rule it is reworded, the way the drafter reworded the same \"second time\" in 01 and 02. Repair: \"... at instances in two separate pieces of work, or once on the main branch with no review reporting it\"." |
| as1/12 | `tripwire@agent-skills@axis-not-run-at-a-pin-move` | "The slip strike is right, but the edit attributes to the owner a ground the owner's words did not answer. At the thaum 0.7.0 intake (03f21a6^), the session proposed \"not fired\" on its own reasoning (...), and the owner's answer was \"Outcome: accepted\", an approval, which primer@design-heads says is no ground. Repair: state the exception on the head (\"The owner's request is the main occasion\"), unattributed." |
| as2/17 | `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap` | "Under calibration reading R3: 6e451b8 records the owner proposing the critical bound (...), so the entry's scope sentence carries a premise the owner stated and is attributed, keeping \"not load-bearing\" (...). The draft's \"no attribution is owed for it\" fails. The outcome, kept with a horizon (6 specs; command rerun: 13), stands; the edit gains the attribution." |
| pk/07 | `tripwire@core@grammars-not-prefixes` | "The outcome, converted, stands. The draft's ground for the upgrade half is false: \"A renamed node empties those results and fails the `test` gate\". rs.rs:27-39 lists 11 `ITEMS` kinds, and the tests of rs.rs exercise none of `impl`, `union`, `trait`, `enum`, `static`, `type` or `macro_rules` (...). The conversion owes a node-kind test against the grammar as well, as agent-skills-1 and agent-skills-2 propose." |
| pk/20 | `tripwire@core@walked-count-differs-between-machines` | "Under the calibration's R1. Condition 2 does not clearly exclude the entry: the comparison reached the history, in 4a7b000 (...) and in 48095d3, and it reached the intake in f964be0 and cebb978. A check on one clone can compute the per-clone exclude file's effect, which is the premise `design@core@per-user-ignore-file-pinned` names (design.md:516-518). So converted, not removed." |
| pk/21 | `tripwire@xtask@tip-alone-is-built` | "Under the calibration's R2. The head says \"A commit before the tip that does not compile passes CI, and is found only by whoever builds it later\": it accepts the event the clause counts, which condition 3's first sentence excludes. So the outcome is removed, which comes before converted. The proposed release-range build also contradicts \"nothing compiles or tests a commit before the tip\", which makes it a reversal of the head." |

## 2. The calibration

Rules the calibration read, in the worktree at the commit audited: ".claude/agents/knowledge-architect-tripwire-auditor.md
(the standard and the lifecycle, as installed: it carries the doubt paragraph's \"unless the owner
would see the failure anyway\" clause and condition 4's \"its head, or its plan document\")", and
the sections `skill@knowledge-architect-issue-tracking@tripwire-entry`, `skill@knowledge-architect-project-audit@tripwire-axis` and `skill@knowledge-architect-project-audit@finding-outcomes`.

| entry | as1 | as2 | pk | of record | reading |
| --- | --- | --- | --- | --- | --- |
| `tripwire@agent-skills@ruling-lost-in-assembly` | reworded, slip | reworded, slip | reworded, slip | as2/14 | agree |
| `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap` | kept | kept | kept | as2/17 | agree on the outcome; R3 on the edit |
| `tripwire@core@grammars-not-prefixes` | converted, cond. 1 | converted, cond. 1 | converted, cond. 1 | pk/07 | agree |
| `tripwire@core@walked-count-differs-between-machines` | converted, cond. 1 | removed, cond. 2 (converted second) | removed, cond. 2 | pk/20 | R1: converted |
| `tripwire@xtask@tip-alone-is-built` | removed, cond. 1 and 3 | removed, cond. 3 (converted second) | converted, cond. 1 | pk/21 | R2: removed, cond. 3 |

"Proposed readings: 3 (R1, R2 on the outcome; R3 on an edit, the outcomes agreeing). Entries whose
rule admits both outcomes: 0."

### `tripwire@agent-skills@ruling-lost-in-assembly`: agree

The three outcomes rest on one sentence of the standard: "Where a clause fires on both a slip and
something more, strike the slip." Differences in the edit, which the calibration names as no
reading of the standard:

- the "**Evidence when written:**" paragraph: pk strikes it, as2 calls its strike optional, as1
  keeps it. "No sentence of the standard or the lifecycle requires the strike: it is the
  session's judgement. Its owner's ruling (\"the owner ruled that such a slip is no critical
  defect\") is also the ground of `design@agent-skills@misstated-ruling-rated-by-what-it-changes`,
  so a strike drops no premise the owner stated."
- as1's horizon command `git log --format=%h --diff-filter=D <W>..main -- docs/plans/ | wc -l`
  "counts commits that delete any file under docs/plans/, not plan documents; the other two filter
  to `specs/<id>.md` and `milestones/<id>/README.md`."

### `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`: R3, the scope sentence attributed

- as1 adds "The owner ruled that the tripwire guards against a critical failure of the workflow,
  not a small mistake that is cheap to repair."; pk adds "On the owner's word, the tripwire guards
  against ..."; as2, of record, adds none: "It is the entry's own statement of scope, and the bound
  it supports is now also the standard's; no attribution is owed for it and none is dropped."
- R3: "the sentence is attributed to the owner". It rests on:
  - the history, 6e451b8: "On the owner's word, the tripwire now requires a critical gap:
    \"Instead, change the tripwire to have it require a critical finding. The point of this kind of
    tripwires is to guard against critical failures in the workflow, not small mistakes that are
    not load bearing. ...\"". "The owner proposed the bound; the session did not."
  - `primer@design-heads`: "The owner's words are a ground, attributed to the owner, only where the
    decision came from the owner: the owner proposed it", and "A durable record attributes a
    decision to the owner ... A head, a commit message, a tripwire, a `rejected` entry and an issue
    entry state what the owner decided"; the auditor's judging rule, "keeping any premise the owner
    stated as an attribution".
  - the scope: an attribution "keeps the scope and the hedges of the owner's words". "The attributed
    sentence keeps \"not load-bearing\", and the definition stays unattributed." The entry's "cheap
    to repair" comes from the owner's not-fired reasoning, and the definition "A gap is critical
    when ..." "is the session's, written in 6e451b8".

### `tripwire@core@grammars-not-prefixes`: agree

All three: converted on condition 1, "Where a script could compute the firing, write that check
instead". The checks differ: as1 a node-kind test in the branch and a `todo` for the
no-item-scope check; as2 the node-kind test and a workspace test, both in the branch; pk the
workspace test alone. A factual note for the verifier: "crates/core/src/source/rs.rs:27 lists 11
kinds in `ITEMS` (`impl_item`, `union_item`, `macro_definition` among them) beside `NAMES`,
`BINDINGS`, `string_content`, `doc_comment` and `closure_parameters`, and `grep -rn
id_for_node_kind crates/` returns nothing. (...) packed-1's check alone leaves a rename of an
unexercised kind silent."

### `tripwire@core@walked-count-differs-between-machines`: R1, converted

- Removal does not follow. The lifecycle: "An entry is proposed removed only when the words of a
  condition, quoted, exclude it." The standard: "A cause that no condition clearly excludes is
  recorded with a short horizon." Condition 2 names "the tree, the history, an audit report, or a
  retrospective file the intake receives". The record shows the comparison in the history and at
  the intake:
  - 4a7b000: "is `walk: 144 file(s)` on a local run of this tree; CI's count for the same commit
    is compared before the merge";
  - 48095d3: "The walk count against CI's, for `tripwire@core@walked-count-differs-between-machines`:
    judged to need no record here; that tripwire states the comparison, and it is read against the
    CI run on this branch's head before the merge";
  - f964be0, "a sparse checkout changes the files the walk reads, but C1 reports a refusal on one
    machine, not a differing count; not fired"; cebb978, "not met: no walked-file count differs".
  "The doubt paragraph's exception, \"unless the owner would see the failure anyway\", does not
  apply: the entry's case is \"both runs green\"."
- Converted follows, on condition 1, "Where a script could compute the firing, write that check
  instead." The head `design@core@per-user-ignore-file-pinned`: "What still decides the walk beside
  the tree's own ignore files is git's per-clone exclude file, which git offers no way to pin, and
  that is what `tripwire@core@walked-count-differs-between-machines` is left guarding." "A check on
  one clone computes whether that file takes a file out of the walk, against the listing without
  it; it needs no second machine."
- The order: "Where more than one applies, name each in the draft and propose the first in that
  order".
- For the verifier: "the git installed" and "the two git versions", which the entry names, are a
  cause the head does not name as the one left guarded; "whether it needs a tripwire of its own is
  a new cause, not this entry's verdict". The untracked-files point (pk, as2) "bears on a fallback
  rewording, if the owner rules the conversion out: the clause then names a clean tree."

### `tripwire@xtask@tip-alone-is-built`: R2, removed on condition 3

- Condition 3: "The evidence is what the decision says must not happen. A clause that counts what
  the decision instructs or endorses fires on the decision working." The head
  `design@xtask@ci-builds-the-tip-alone`: "A commit before the tip that does not compile passes CI,
  and is found only by whoever builds it later." The entry: "**Fires when:** a `git bisect` over
  `main` stops on a commit that does not compile". "The head nowhere says such a commit must not
  happen; it states that one passes and is found later. The condition's first sentence excludes
  the clause whatever \"endorses\" is read to cover, so packed-1's doubt, on \"endorses\", does not
  reach it."
- "No count rescues it": condition 4 asks a basis "the decision's record carries"; "the head
  carries the suite's run time, not a rate of broken commits."
- pk's check "is also the rival the head rejected: the Response names \"a build-only pass over
  each commit of the range\" as the reopening's candidate (...). That is the decision's reversal,
  not a check beside it."
- The group: condition 3. as1's condition 1 ("stops the work that meets it": the bisect is the
  work) "is also quotable and is named in the item".

### Drafts the verifiers judged again under a proposed reading

| draft | reading | result |
| --- | --- | --- |
| as2/17 | R3 | "refuted on its edit, its outcome kept" |
| as2/14 | the calibration's "agree" | "no change" |
| pk/20 | R1 | "refuted. The outcome is converted." |
| pk/21 | R2 | "refuted. The outcome is removed, on condition 3." |
| pk/07 | the calibration's factual note on unexercised node kinds | "Refuted on the ground of its upgrade half." |

as1's verifier: "The calibration's readings R1 to R3 bear only on sample entries, whose drafts of
record are other groups'. None bears on drafts 01 to 22." Drafts judged again for other reasons:
as1 re-read 01, 02, 13 and 16 after refuting 03, and compared 04 and 09 with 08 and 14; as2
re-read 04, 05, 07, 14 for "no attribution owed", and 08 and 19 for a keeping reading; pk re-read
09 and 13 after 18 ("09 becomes plausible"), and 03 after 07 ("it stands").

## 3. The findings

Grouped by the outcome the cross-check proposes (cross-check part 3). The outcomes applied in the
branch come first, by anchor and file. `<W>` in a command is the commit that writes the line. Not
a re-check: no old or new text.

### 3.1 Applied in the branch: agent-skills, crates/agent-skills/docs/tripwires.md

#### 3.1.1 Kept (8)

##### `tripwire@agent-skills@calibration-reading-rejected` (as1/13, verified)

- Outcome: kept, on a doubt on condition 2; the `T1` label reworded.
- Evidence: "**Fires when:** in one audit, the session's verification rejects a reading the
  calibration agent proposed, in two passes." The head: "a reading it rejects is corrected in the
  calibration file, and the run starts again from its verify step." Condition 2, a doubt: "the
  rejection is corrected \"in the calibration file\", in the run's scratch directory. The committed
  record is the pass's commit message (...). Whether it shows a rejected reading is not
  established."
- Edit: "T1 of the premortem of the discussion that made the decision. The decision rests on ..."
  becomes "The decision rests on ...".
- Horizon: "**Watches until:** two report files of the design-record axis added under
  docs/audit-reports/ on the main branch, counted from the commit that writes this line."
  Command: `git log --format=%h --diff-filter=A <W>..main -- 'docs/audit-reports/*design-record-axis*' | wc -l`
- Verifier: verified. "T1 was agent-proposed (design-record-workflow spec at aaf0e52^, premortem
  row 1), so no owner premise is dropped. \"in two passes\" reads as recurrence across separate
  runs, each with its own report and commit. The condition-2 doubt is fair."

##### `tripwire@agent-skills@conforming-draft-wrong` (as1/15, verified), pending X2

- Outcome: kept; the `T2` label and an approval cited as a ground reworded. The cross-check marks
  it "pending X2" (part 5, readings): "If X2 is ruled the way as2/20 reads it: reworded to \"in two
  passes\", kept with a short horizon".
- Evidence: "**Fires when:** in any pass, the session's own sample of conforming drafts holds one
  that a reading shows failing a rule, and that the report does not list among the drafts whose
  verdict it doubts." "The slip rule does not bind it: the clause watches the coverage of the
  verifier's sample, not a session failing to follow an instruction." "\"as the owner agreed with
  the rulings on that pass\" cites an approval."
- Edit: "... the clause leaves such a draft out, as the owner agreed with the rulings on that
  pass." becomes "... the clause leaves such a draft out, since a draft the report lists as
  doubtful is one the session reads."; the `T2` label removed.
- Horizon: "**Watches until:** two report files of the design-record axis added under
  docs/audit-reports/ on the main branch, counted from the commit that writes this line."
  Command: `git log --format=%h --diff-filter=A <W>..main -- 'docs/audit-reports/*design-record-axis*' | wc -l`
- Verifier: verified. "Pass-1 report at aaf0e52^: the exception was the session's default, \"and
  the owner agreed with it: \"Agreed on all defaults except F2 and F4\"\". That is an approval, so
  rewording it to its argument is right. The slip rule does not bind a clause on the coverage of
  the verifier's sample."

##### `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` (as1/16, verified)

- Outcome: kept, on a doubt on condition 3.
- Evidence: "**Fires when:** a second instance is recorded of a deferred trigger met by work that
  did not go through the design skill, and found only at the standing-state review." The head:
  "A search before every other piece of work stays parked: (...) the evidence for it is one
  instance". "Condition 4: \"a second instance\" has its basis in the head's \"one instance\"."
- Edit: horizon only.
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: verified. "The head quote \"the evidence for it is one instance\" was checked. A second
  instance defeats that argument, so keeping on a doubt on condition 3 holds."
- Cross-check: "Read it again when the session rules on search-missed's reopening (1c)"; the
  draft's "stays true" of the agent-behaviour issue, line 90, fails once search-missed is deleted
  (part 4).

##### `tripwire@agent-skills@overlap-missed` (as2/02, verified), horizon command corrected

- Outcome: kept, on a doubt on condition 1; `T3` label reworded.
- Evidence: condition 1 "doubtful, not excluded. A collision met during the application is
  resolved by the session in the work; (...) No test or check counts a missed overlap." T3: "The
  owner, round 3: \"Keep all proposed tripwires and acceptance criterion.\"" "An approval".
- Edit: "T3 of the premortem of the discussion that made the design-record axis's saved workflow.
  The decision rests on the premise ..." becomes "The decision rests on the premise that the
  cross-check maps every pair of findings that edit one paragraph or one slug, so the application
  meets no collision the report did not show."
- Horizon: "**Watches until:** the application of 2 passes of the design-record axis, each counted
  by its report file." Draft's command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*design-record-axis-pass-*.md' | wc -l`.
  Correction (verifier, cross-check): "the horizon says \"the application of 2 passes\" but its
  command counts report files added (`--diff-filter=A`); `--diff-filter=D`, the file's deletion at
  its last outcome, counts applications", or the line is reworded to "2 report files added".
- Verifier: verified. "Kept on a doubt is sound; T3 is an approval (...); 82fd028 is an ancestor of
  aaf0e52."

##### `tripwire@agent-skills@piped-chain-runs-under-pipefail` (as2/04, verified)

- Outcome: kept, on a doubt on condition 1; "`T1` label and \"ruled by the owner at the intake of
  2026-10-24\" are reworded to the entry's own premise".
- Evidence: 8ca8d6b: "The owner ruled its labels: T1, recorded as
  `tripwire@agent-skills@piped-chain-runs-under-pipefail`". "A ruling to record, stating no
  premise". Slip: "the clause watches the option's reach, not how a session follows the primer's
  rule; one instance shows the premise false."
- Edit: "T1 of the premortem of the decision, ruled by the owner at the intake of 2026-10-24. The
  decision rests on the premise that the option reaches every shell the harness starts." becomes
  "The decision rests on the premise that the option reaches every shell the harness starts."
- Horizon: "**Watches until:** 10 retrospective analyses, each a file added under
  docs/retrospective-reports/." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- docs/retrospective-reports/ | grep -vc 'README.md$'`
- Verifier: verified. "8ca8d6b alone touches the id; (...) a ruling to record with no premise of
  the owner's; no analysis added after 8ca8d6b (empty output, rerun); kept on a doubt on condition
  1 is sound."

##### `tripwire@agent-skills@pipefail-status-misread` (as2/05, verified)

- Outcome: kept, on a doubt on condition 1; `T2` label and "ruled by the owner" reworded.
- Evidence: the premise "the statuses the option adds ... fail safe" is the head's own argument
  ("Its costs fail safe: a producer cut short by `head` exits 141, and a `grep` that matches
  nothing fails its pipeline"). Slip: "no instruction a working session reads tells it how to read
  these statuses, so the clause watches the premise \"fail safe\", not a lapse. Not struck; a
  doubt."
- Edit: "T2 of the premortem of the decision, ruled by the owner at the intake of 2026-10-24. The
  decision rests on the premise that the statuses the option adds, 141 after `head` and 1 after a
  `grep` that matched nothing, fail safe." becomes the same sentence from "The decision rests on".
- Horizon: "**Watches until:** 8 retrospective analyses, each a file added under
  docs/retrospective-reports/." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- docs/retrospective-reports/ | grep -vc 'README.md$'`
- Verifier: verified. "Same history as 04; \"Its costs fail safe\" is the head's own argument
  (design.md:2718 head, checked); kept on a doubt."

##### `tripwire@agent-skills@ruling-lost-in-change` (as2/15, verified), X4

- Outcome: kept, no label to reword. Cross-check X4: "the session decides whether it gains the
  critical bound its two siblings carry".
- Evidence: the clause fires on a ruling found misstated "after the branch's transcript review",
  "so it counts an instance no procedure reported before the main branch: the rule on slips' second
  case". "a bound on criticality, as its sibling carries, is not required by any condition and is
  left to the session."
- Edit: horizon only.
- Horizon: "**Watches until:** 10 commits on the main branch whose message carries a deliberation
  on the in-change path." Command: `git log --format=%h -i --grep='on the in-change path' <W>..origin/main | wc -l`
  ("tested over cbe804f..the commit audited: 22").
- Verifier: verified. "The clause counts only what is found \"after the branch's transcript
  review\", the last review before the merge: no slip. (...) horizon command rerun: 22 (it counts
  messages naming the phrase, a loose instrument for \"carries a deliberation\")."

##### `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap` (as2/17, refuted on the edit; sample)

- Outcome: kept; the scope sentence attributed to the owner, per R3.
- Evidence: 6e451b8, the owner: "Instead, change the tripwire to have it require a critical
  finding. The point of this kind of tripwires is to guard against critical failures in the
  workflow, not small mistakes that are not load bearing." Slip: "the clause is not \"on how
  sessions follow an instruction\": the spec's work follows the instruction, which allows the skip;
  it watches the instruction's premise. One instance is the premise shown false. Not struck."
- Edit, as the cross-check carries it: "the scope sentence **attributed to the owner**, keeping
  \"not load-bearing\"; the definition of \"critical\" left unattributed". Before: "The tripwire
  guards against a critical failure of the workflow, not a small mistake that is cheap to repair."
  The other drafts' afters, for the session's wording: as1/24, "The owner ruled that the tripwire
  guards against a critical failure of the workflow, not a small mistake that is cheap to repair.";
  pk/23, "On the owner's word, the tripwire guards against a critical failure of the workflow, not
  a small mistake that is cheap to repair." The calibration: the attributed sentence "keeps \"not
  load-bearing\"", and "cheap to repair" comes from the owner's not-fired reasoning.
- Horizon: "**Watches until:** 6 specs deleted at their landing." Command: `git log --diff-filter=D --name-only --format= <W>..origin/main -- docs/plans/specs/ | wc -l`
  ("tested over cbe804f..the commit audited: 13"). pk/23's command filters docs/plans/specs/README.md out.
- Verifier: refuted on the edit (reason in part 1). "The outcome, kept with a horizon (6 specs;
  command rerun: 13), stands; the edit gains the attribution."

#### 3.1.2 Reworded (27)

##### `tripwire@agent-skills@a-comment-escalation-is-overruled` (as1/01, verified), X1

- Outcome: reworded: the slip rule; kept on a doubt on conditions 1 and 2.
- Evidence: "\"the second time\" is a recurrence, which the slip rule admits only across
  \"separate pieces of work\"; two rulings in one session would fire it." Condition 2, a doubt: "it
  reaches no record if no retrospective runs. The doubt-weight sentence of the standard applies
  (\"unless the owner would see the failure anyway ... a cause in doubt may be dropped\"): the owner
  rules each instance. It weighs a doubt only; the lifecycle proposes a removal only on a quoted
  condition, so the entry is kept, with a short horizon."
- Before: "**Fires when:** the owner rules, the second time, that a session's escalation of a
  change because a comment states a reason was unneeded, the comment holding no decision worth the
  question." After: "**Fires when:** the owner rules, in a second piece of work, that a session's
  escalation of a change because a comment states a reason was unneeded, the comment holding no
  decision worth the question."
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: verified. "Keeping on a doubt is allowed: the doubt weight permits a drop but does not
  require one." Cross-check: "X1 is put to the owner beside it."

##### `tripwire@agent-skills@a-head-verdict-is-overruled` (as1/02, verified, fires), X1

- Outcome: reworded: the slip rule, and the owner's ruling at the thaum intake; **not fired**
  (cross-check).
- Evidence: a7db37a: "this is the first overruling of a review's verdict on record, and the
  tripwire fires at the second". The thaum analysis 2026-10-13-thaum-slice-numbers-merge, its
  Outcome: "no change, and the tripwire not fired, replacing the default. The owner, in the file:
  \"I think there is nothing to do here ? Isn't this just the project applying different rules
  than ours ?.\" And, in the conversation, on reading the project's overruling as its own scope
  choice, with no instance in this repository: \"Agreed, do nothing.\""
- Before: "**Fires when:** the owner overrules a review's verdict on whether a decision earns a
  head, the second time." After: "**Fires when:** the owner of this repository overrules a review's
  verdict on whether a decision earns a head, in a second piece of work; the first is recorded in
  the message of the commit \"[review] Five reviewers read the cleanup: the owner's own intent
  becomes a fourth entry test, three heads that carry it come back, and the published workflow's
  heads all live in agent-skills\". An adopting project's owner setting the project's own rule does
  not fire it, on the owner's ruling at the intake of that project's file."
- Horizon: "**Watches until:** ten commits on the main branch whose subject opens with `[review]`,
  counted from the commit that writes this line." Command: `git log --format=%s <W>..main | grep -c '^\[review\]'`
- Verifier: verified, fires. "The owner raised the reading himself (...), then \"Agreed, do
  nothing\", so the attribution keeps to what he answered. The thaum overruling met the clause's
  letter as a second instance, and the owner ruled it not fired: that is firing evidence, for the
  session."

##### `tripwire@agent-skills@a-shortcut-decision-earns-a-head` (as1/03, refuted)

- Outcome: reworded, on the verifier's repair (the draft proposed kept).
- Evidence: "**Fires when:** a decision-record review finds, in a commit made on the bounded path, a
  decision that passes an entry test of `design@agent-skills@a-head-is-owed-by-an-entry-test`, at
  the second instance." 22f3a96: "did not fire: no commit was made on the bounded path."
- Edit, the verifier's repair: the clause ends "... at instances in two separate pieces of work,
  or once on the main branch with no review reporting it".
- Horizon, from the draft: "**Watches until:** three commits on the main branch whose message names
  the bounded path, counted from the commit that writes this line." Command: `git log --format=%h -i --grep='bounded path' <W>..main | wc -l`
- Verifier: refuted (reason in part 1). Cross-check 1c: "The verifier refuted 03 on this, and
  repaired it to the reading of 02. That settles it: no difference stays."

##### `tripwire@agent-skills@agent-text-findings-need-nothing` (as1/04, plausible): cross-check departs from the draft

- Draft's outcome: removed, condition 4 then condition 3. Cross-check's: "**reworded, kept on a
  doubt**, not removed; the bounds struck on condition 4; the clause names a recurrence, for
  example \"in the reviews of two separate pieces of work, every finding of the axis on
  agent-facing text is judged to need nothing\"; horizon over commits that change
  `path@agent-skills@content/`".
- Evidence: entry: "**Fires when:** in one review where the axis on agent-facing text reports at
  least four findings, the session or the owner judges more than half of them to need nothing."
  The spec at ca41ed0^: "tripwire T2, on the owner's word (O5)". Draft, condition 3: the head
  `design@agent-skills@statement-names-an-act`, "The test's reach, stripping what is not fully
  necessary at the price of some variation between its readers, is the owner's weighing: a
  statement one reader passes is met by a later pass."
- Draft's edit: "Delete the heading and its body." "referenced by nothing".
- Verifier: plausible. "The removal rests on condition 3 applied to the bare event, and a
  counter-reading is not excluded: the agent's own text says \"a report padded to look productive
  costs the dispatcher a verification pass per invented finding\" (...). On that reading, needless
  findings recurring across separate reviews may be what the decision says must not happen."
- Cross-check reason: "The lifecycle removes an entry \"only when the words of a condition, quoted,
  exclude it\". The doubt's weight does not clearly apply, since the session, not the owner, judges
  most review findings. If the session holds the draft's reading: owner list, removals on condition
  3." No horizon command is given by any step.

##### `tripwire@agent-skills@agent-text-written-without-the-standard` (as1/05, verified)

- Outcome: reworded: the slip rule; `T1` label reworded; kept on a doubt on condition 2.
- Evidence: the slip, "never fires on one instance that a review, a check or a retrospective
  reported under its own procedure"; "The clause fires on one instance found by \"a review or a
  retrospective\". Struck." Condition 2: "whether a session loaded a skill shows in its transcript
  alone".
- Before: "T1 of the premortem of the discussion that made the decision. The decision rests on the
  premise ..." / "**Fires when:** a review or a retrospective finds an addition ... made in a
  session that did not load the agent-configuration skill." After: "The decision rests on the
  premise ..." / "**Fires when:** an addition to agent-facing text that fails the need test of
  `design@agent-skills@statement-names-an-act` or the real-use rule of
  `design@agent-skills@behaviour-additions-need-real-use`, made in a session that did not load the
  agent-configuration skill, is reported in two separate pieces of work, or reaches the main branch
  with no review reporting it."
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: verified. "T1 was agent-proposed and approved, so no owner premise is dropped. The
  condition-2 doubt is properly kept short."

##### `tripwire@agent-skills@argued-closure-reopened` (as1/06, verified), horizon command corrected, X1

- Outcome: reworded: condition 4 and the slip; `T1` label reworded.
- Evidence: "**Fires when:** in one discussion, the owner reopens 2 or more threads closed
  `argued`, saying a rival was not defeated." Head: "Both show in the next delta and in the
  checkpoint, where the owner contests or overrules them". "\"2 or more\" appears in neither the
  head nor the slice's plan document as a measurement or an argument."
- Before: "T1 of the premortem (...) The decision rests on ..." / the clause above. After: "The
  decision rests on ..." / "**Fires when:** in two separate discussions, the owner reopens a thread
  closed `argued`, saying a rival was not defeated."
- Horizon: "**Watches until:** three plan documents deleted at their landing on the main branch,
  counted from the commit that writes this line." Draft's command `git log --format=%h --diff-filter=D <W>..main -- docs/plans/ | wc -l`;
  corrected by the cross-check to `git log --diff-filter=D --name-only --format= <W>..main -- docs/plans/ | grep -Ec '(specs/[^/]+|milestones/[^/]+/README)\.md$'`.
- Verifier: verified. "\"2 or more\" was agent-proposed (\"reply to R4, agent\") and approved
  (\"tripwire, on the owner's word\"), with no basis. (...) Defect in the edit: the horizon command
  (...) counts commits that delete under docs/plans/, not plan documents".

##### `tripwire@agent-skills@attribution-loses-the-scope` (as1/07, verified, known, bears-on-ruling, fires), X4

- Outcome: reworded: the slip rule, a critical bound added; `T4` label reworded; **not fired**
  (cross-check).
- Evidence: `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`: "was met by its
  letter again in two sessions, one of this repository and one of thaum ... none load-bearing ...
  The owner ruled it not fired, on the ground the earlier instance above was ruled on."
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: "That such a slip is no critical
  defect is the owner's weighing"; "A misstated ruling is critical only when it is reversed, or
  when its state or scope changes what is built or a load-bearing decision."
- Before: "T4 of the premortem (...)" / "**Fires when:** the transcript reviewer reports a misstated
  ruling that traces to an attribution in a durable record, where the owner's words would have
  shown the scope." After: "The decision rests on ..." / "**Fires when:** an attribution in a
  durable record misstates a ruling of the owner where the owner's words would have shown its
  scope, and the misstatement is critical under
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`, in two separate pieces of work,
  or once on the main branch with no review having reported it."
- Horizon: "**Watches until:** ten commits on the main branch whose subject opens with `[review]`,
  counted from the commit that writes this line." Command: `git log --format=%s <W>..main | grep -c '^\[review\]'`
- Verifier: verified, known, bears-on-ruling, fires. "The bound \"critical\" follows the owner's
  weighing that this head records. The letter was met in several separate pieces of work and ruled
  not fired each time. Under the standard's recurrence branch that bears on those rulings; the
  session settles."
- Cross-check: "Each instance met by the letter was ruled not load-bearing by the owner (...). The
  reworded clause counts only a critical misstatement, so none of the past instances meets it."

##### `tripwire@agent-skills@audit-drafts-mostly-noise` (as1/09, plausible): cross-check departs from the draft

- Draft's outcome: removed, condition 4 then condition 3. Cross-check's: "**reworded, kept on a
  doubt**, not removed; \"more than half\" struck; the clause names a recurrence across two passes
  of the axis; horizon two agentic-workflow-axis report files".
- Evidence: "**Fires when:** in one pass, more than half of the drafts fail the reading." The spec
  at 39bf4fb^: "The bounds of T1 to T4 are the ones round 2 put to the owner and the owner ruled to
  record." Draft, condition 3: the head, "A finding is admitted in a class whose evidence a reading
  checks"; "A draft the reading refuses is that admission test working."
- Draft's edit: "Delete the heading and its body." "referenced by nothing".
- Verifier: plausible. "The other: the auditor's own text admits a finding only \"when the session
  can confirm it by reading\", and refuted drafts recurring across passes could be the failure of
  the finding standard rather than the verifier working. Neither reading is settled by the text."
- Cross-check: "the session alone sees refuted drafts, so the doubt's weight does not clearly
  apply. If the session holds the draft's reading: owner list, removals on condition 3."

##### `tripwire@agent-skills@axis-not-run-at-a-pin-move` (as1/12, refuted)

- Outcome: reworded: the slip struck; the exception stated unattributed (verifier's repair).
- Evidence: the thaum 0.7.0 analysis: "met by its letter, not fired. (...) here the owner chose
  the occasion's scope, which `design@agent-skills@audit-triggers` makes the owner's"; Outcome
  "accepted".
- Before: "T7 of the premortem (...)" / "**Fires when:** a retrospective finds a pin moved across
  such a version with the axis not run." Draft's after: "**Fires when:** in two separate pin moves,
  received retrospective files show a pin moved across such a version with the axis not run. An
  axis that the owner chose to replace or to scope is not counted, on the owner's ruling at the
  intake of a consumer's pin move." Repair: the exception unattributed, "The owner's request is the
  main occasion".
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: refuted (reason in part 1).

##### `tripwire@agent-skills@departure-overridden-as-a-reading` (as1/17, verified)

- Outcome: reworded: the slip; `T3` label reworded; kept on a doubt on condition 1.
- Evidence: condition 1, "`primer@owner-word-premise` has the owner \"told at the top of the turn,
  with the source cited\", so the owner may see the override at once."
- Before: "**Fires when:** a session applies a source's text over an answer the owner gave as a
  deliberate departure from that source." After: "**Fires when:** a session applies a source's text
  over an answer the owner gave as a deliberate departure from that source, in two separate pieces
  of work, or once in a record that reaches the main branch with no review reporting it." The `T3`
  label removed.
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: verified. "The T3 row of the owner-authority milestone is an approved agent proposal."

##### `tripwire@agent-skills@design-conformance-findings-needless` (as1/18, verified), horizon command corrected

- Outcome: reworded: condition 4; bounds replaced by the event the head's second rule names.
- Evidence: "The count, two moments, and the proportion, half, are defaults the owner may reset."
  The spec at 287cfc2^: "keep T1 only"; "defaults marked as the owner's to reset". The head: "The
  second rule keeps it from reporting each widening the document makes on purpose."
- Before: "**Fires when:** at two moments in a row at which the planning skill sends a plan
  document's reviewers, a re-review after a revision included, more than half of the
  design-conformance reviewer's findings end judged to need nothing, by the dispatching session or
  by the owner. The count, two moments, and the proportion, half, are defaults the owner may
  reset." and "The owner ruled on it as T1 at the premortem of its discussion." After: "**Fires
  when:** in the reviews of two separate plan documents, the design-conformance reviewer reports as
  a finding a widening or a reversal of a head that the document's \"What is already decided\"
  lists as reversed or rewritten." "The sentence on T1 is removed."
- Horizon: "**Watches until:** three plan documents deleted at their landing on the main branch,
  counted from the commit that writes this line." Command corrected as for as1/06.
- Verifier: verified. "The replacement event is the head's own (...). Same horizon-command defect
  as 06."

##### `tripwire@agent-skills@design-entry-search-misses` (as1/19, verified), horizon command corrected

- Outcome: reworded: the slip; the two `T<n>` attributions reworded.
- Evidence: "One miss \"reported under its own procedure\", the plan-document review, which the head
  names as the backstop, is the review working." "The attributions \"The owner ruled on it as T2\"
  and \"as T1\" record approvals of the tripwire under the reversed per-tripwire rule".
- After: "The decision rests on the premise that a search at grounding, sent again for a later
  thread, puts before the session every design entry and `rejected` entry its proposals bear on.
  **Fires when:** in two separate discussions or intakes where the design-entry search ran, a head
  contradicted, or a `rejected` entry brought back without a recorded reopening, is found that the
  search did not return: by the design-conformance review of a plan document, or by an outcome the
  owner ruled in the intake. Or once, on the main branch, with no review having reported it."
- Horizon: "**Watches until:** three plan documents deleted at their landing, and three commits that
  add analyses of received retrospective files under docs/retrospective-reports/, on the main
  branch, counted from the commit that writes this line." Commands: the plan-document count
  (corrected as for as1/06) and `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`.
- Verifier: verified. "094ed34 and 48a5995 record \"T1 recorded\" and T2 as the owner's rulings to
  record a tripwire (...). They state no owner premise."

##### `tripwire@agent-skills@expectation-set-closes-a-contradiction` (as1/21, verified)

- Outcome: reworded: the slip; the heading gains its premise.
- Evidence: heading "## Guarding `design@agent-skills@expectation-set-bounds-scope` (slug expectation-set-closes-a-contradiction)";
  the standard's shape "## Guarding `design@<anchor>@<id>`: <the premise watched> `##<slug>`".
- After, heading: "## Guarding `design@agent-skills@expectation-set-bounds-scope`: a contradiction
  between two installed instructions is closed as outside a skill's scope (slug expectation-set-closes-a-contradiction)";
  premise: "The decision rests on the premise that an expectation set scopes the owner's behaviour
  only, so that no finding of two installed instructions leaving no move satisfying both is closed
  by citing one."; "**Fires when:** a finding that two installed instructions leave no move
  satisfying both is closed by citing an expectation set, in two separate analyses or reviews, or
  in a record that reaches the main branch with no review reporting it."
- Horizon: "**Watches until:** three commits on the main branch that add analyses of received
  retrospective files under docs/retrospective-reports/, counted from the commit that writes this
  line." Command: `git log --format=%h --diff-filter=A <W>..main -- docs/retrospective-reports/ | wc -l`
- Verifier: verified. "The premise added is the head's own sentence, \"The set scopes the owner's
  behaviour only\"."

##### `tripwire@agent-skills@goal-missed-though-graded` (as1/22, verified), horizon command corrected

- Outcome: reworded: the slip; the `T2` attribution reworded.
- Before: "The owner ruled on it as T2 of the discussion that had the design-entry searcher grade
  each goal of its Component." / "**Fires when:** a retrospective or a review reports a goal missed
  in a design discussion or a retrospective intake where the design-entry searcher graded that
  goal's Component's goals." After: "**Fires when:** a goal is missed in a design discussion or a
  retrospective intake where the design-entry searcher graded that goal's Component's goals, in two
  separate pieces of work, or once in a record that reaches the main branch with no review
  reporting it."
- Horizon: "**Watches until:** three plan documents deleted at their landing, and three commits that
  add analyses of received retrospective files (...)". Commands as for as1/19.
- Verifier: verified. "The attribution only records the ruling to record a tripwire, and the
  owner's premise stays in the head."

##### `tripwire@agent-skills@head-created-without-deliberation` (as2/01, plausible): then judged for a firing

- Outcome: reworded, applied, "**then judged for a firing at once**" (cross-check).
- Evidence: the decision-record reviewer checks "the backstop, for a decision that should have
  gone to the design skill". Ruling history: "\"Agreed on D1,D3,D4 defaults\": an approval".
- Before: "**Fires when:** a commit creates a design head, contradicts a statement of one, or adds
  to it a member its argument does not cover, and neither its message nor a plan document carries
  the deliberation: no thread, no rival, and no record of the owner's ruling where the owner ruled.
  A relocation or rewording that adds or removes no decision does not count." After: "**Fires
  when:** a commit on the main branch creates a design head, contradicts a statement of one, or adds
  to it a member its argument does not cover, and neither its message nor a plan document carries
  the deliberation: no thread, no rival, and no record of the owner's ruling where the owner ruled;
  or reviews of two separate branches each report such a commit of their branch. A relocation or
  rewording that adds or removes no decision does not count."
- Horizon: "**Watches until:** 20 commits on the main branch that add a level-three heading to a
  design home." Command: ``git log --format=%h -G'^### .*`##[a-z0-9-]+`$' <W>..origin/main -- docs/design.md 'crates/*/docs/design.md' 'tools/*/docs/design.md' | wc -l``
  ("tested over cbe804f..the commit audited: 106").
- Verifier: plausible. "the edit's new branch, \"reviews of two separate branches each report such a
  commit\", was not read against cebb978's finding c (\"The backstop lapse recurs in this intake:
  here; in the init workflow file, two rules added to a head under a relocation said to add no
  decision; in the driven-tests workflow file ...\"), each caught by a review before its merge: the
  reworded clause may be met on its first day, and overlaps
  `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`. (...) Horizon
  command reruns: 106 over cbe804f..the commit audited; it counts commits whose diff touches a heading line,
  not only additions."
- Cross-check: "If the session judges these met: **fired**, owner list. The response opens no
  second issue for the same instances, but names that issue, with
  `design@agent-skills@new-or-reshaped-head-needs-design` reopened beside it. The horizon (20
  commits that add a heading line) passes fast: 106 over cbe804f..the commit audited."

##### `tripwire@agent-skills@owner-intent-stripped` (as2/03, verified, known)

- Outcome: reworded: condition 2 (transcript struck), the slip, the response brought to the shape;
  `T2` label removed.
- Evidence: condition 2, "Evidence that exists only in a session's transcript ... reaches no
  occasion that reads the tripwires."; the agent-behaviour issue: "the transcript reviewer does not
  read the tripwires homes". Response "restore the owner's ground, attributed, and reopen ..."
  "opens with a direct action, which the shape does not name".
- Before: "**Fires when:** a commit removes, or rewords as an argument, the owner's ground in a
  design head, quoted or attributed, and the transcript or the plan document shows that the
  decision, or that part, came from the owner. (...) **Response:** restore the owner's ground,
  attributed, and reopen `design@agent-skills@head-ground-is-the-argument` (...) **Re-entry:** the
  standing-state review before every merge, and the transcript review of the change." After:
  "**Fires when:** a commit on the main branch has removed, or reworded as an argument, the owner's
  ground in a design head, quoted or attributed, where a plan document or a commit message shows
  that the decision, or that part, came from the owner, and no review reported it before the merge;
  or reviews of two separate branches each report such a removal. (...) **Response:** reopen
  `design@agent-skills@head-ground-is-the-argument` on how a ground is told apart, with the rival
  that cites every ruling of the owner among the candidates; the removed ground is restored as the
  repair of the instance. **Re-entry:** the standing-state review before every merge, and the
  design-record axis of `skill@knowledge-architect-project-audit`."
- Horizon: "**Watches until:** 20 commits on the main branch that change a design home's heads'
  text, counted as commits that add or remove a level-three heading of a design home." Command: as
  for as2/01.
- Verifier: verified, known (eight-responses issue). "T2 is an approval (\"All tripwires and AC
  approved.\", 68841c5^ line 1365)".

##### `tripwire@agent-skills@pointer-not-followed` (as2/06, verified, known), edit corrected

- Outcome: reworded: the slip; `T5` label removed.
- Before: "**Fires when:** a review or a retrospective finds a directive broken in a session where
  its restatement had been replaced by a pointer under the decision." Draft's after: "**Fires
  when:** reviews or retrospectives of two separate pieces of work each find the same directive
  broken where its restatement had been replaced by a pointer under the decision, or one such break
  reaches the main branch with no review or retrospective reporting it." Correction: "\"a directive
  broken\" in each of two pieces of work, not \"the same directive broken\"".
- Horizon: "**Watches until:** 12 retrospective analyses, each a file added under
  docs/retrospective-reports/." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- docs/retrospective-reports/ | grep -vc 'README.md$'`
- Verifier: verified, known (severity issue). "\"the same directive broken\" narrows the standard's
  \"the same failure recurring\"; the failure watched is a pointer not followed".
- Stale: the pointer issue, line 99 (part 4).

##### `tripwire@agent-skills@rarer-rule-in-the-model` (as2/07, verified, known)

- Outcome: reworded: the response brought to the shape; `T2` label removed; the owner's exception
  stays attributed.
- Before: "**Response:** remove the rule from the section, and reopen with the owner the scope of
  the ruling that the rarer checker rules are delivered by their findings." After: "**Response:**
  reopen `design@agent-skills@checker-model-in-primer` with the owner, on the scope of the owner's
  ruling that the rarer checker rules are delivered by their findings; the rule found is removed
  from the section as the repair of the instance. **Re-entry:** that review or audit."
- Horizon: "**Watches until:** 10 commits on the main branch that change
  `path@agent-skills@content/PRIMER.md`." Command: `git log --format=%h <W>..origin/main -- crates/agent-skills/content/PRIMER.md | wc -l`
- Verifier: verified, known (eight-responses and severity issues). "the owner's exception clause
  stays attributed."

##### `tripwire@agent-skills@record-answered-questions-reach-the-owner` (as2/09, verified)

- Outcome: reworded: condition 4 and the slip; `T2` label removed; an attribution narrowed
  ("listed under \"Decided\"").
- Evidence: the head carries "at least three of the eleven questions one session put to the
  owner", "and no argument for 2". The owner's words quoted in the misstated-ruling head, "I do not
  believe any amount of instruction anywhere would help improve more than the current situation:
  LLM agents are trained to take the input they receive from humans seriously and follow them
  closely", answered "the rating of misstated rulings, not the routing of questions".
- Before: "(...) which `design@agent-skills@misstated-ruling-rated-by-what-it-changes` records the
  owner judging that no further instruction would reduce. **Fires when:** a retrospective of a
  session after this decision's landing finds 2 or more questions put to the owner that a goal, a
  design head or a source's text answered." After: "(...) which
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes` records the owner observing in
  agents. **Fires when:** the retrospectives of two separate pieces of work each find a question
  put to the owner that a goal, a design head or a source's text answered."
- Horizon: "**Watches until:** 15 retrospective analyses, each a file added under
  docs/retrospective-reports/." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- docs/retrospective-reports/ | grep -vc 'README.md$'`
- Verifier: verified. "The narrowed attribution is right".

##### `tripwire@agent-skills@record-check-misses-a-head` (as2/10, verified)

- Outcome: reworded: the slip, condition 3 bearing on the single instance; `T8` label removed.
- Evidence: the head: "a head it misses is found at the review of the audit's branch, which reads
  each repair against the design heads." "It endorses a miss, not misses that recur".
- Before: "**Fires when:** the review of an audit's branch finds a repair that contradicts a design
  head, or brings back a rejected alternative, which the check of its proposal did not name."
  After: "**Fires when:** the reviews of two separate audits' branches each find a repair that
  contradicts a design head, or brings back a rejected alternative, which the check of its proposal
  did not name; one such miss, which the head's argument expects that review to find, does not fire
  it."
- Horizon: "**Watches until:** 3 passes of the agentic-workflow axis, each counted by its report
  file." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*agentic-workflow-axis-pass-*.md' | wc -l`
- Verifier: verified. Draft: "Should the owner read condition 3 as excluding it, it would go to
  the owner list as a removal on condition 3."

##### `tripwire@agent-skills@rejected-entry-cited-by-the-wrong-head` (as2/11, verified)

- Outcome: reworded: the slip; "ruled on it as T1" removed.
- Before: "(...) The owner ruled on it as T1 at the premortem of its discussion. **Fires when:** a
  review or an audit finds a `rejected` entry whose only citing heads decide a question other than
  the one the design it records answered." After: "**Fires when:** an audit, or the review of a
  branch that did not add the citation, finds on the main branch a `rejected` entry whose only
  citing heads decide a question other than the one the design it records answered; or the reviews
  of two separate branches each find one that their branch added."
- Horizon: "**Watches until:** 15 commits on the main branch that add a level-three heading to a
  `rejected` home, and one pass of the design-record axis." Commands: ``git log --format=%h -G'^### .*`##[a-z0-9-]+`$' <W>..origin/main -- docs/rejected.md 'crates/*/docs/rejected.md' 'tools/*/docs/rejected.md' | wc -l``
  and `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*design-record-axis-pass-*.md' | wc -l`.
- Verifier: verified. "094ed34^ line 1113 \"You can keep both tripwires and both AC\" is an approval
  (...); horizon commands rerun (12 rejected-heading commits over cbe804f..the commit audited)."

##### `tripwire@agent-skills@rule-misread-across-groups` (as2/12, verified)

- Outcome: reworded: condition 4; `T4` label removed.
- Before: "**Fires when:** in one pass, the session traces findings in at least three groups to one
  misreading of one rule, and the calibration did not catch it." After: "**Fires when:** in one
  pass, the session traces findings of more than one group to one misreading of one rule, and the
  calibration did not catch it."; premise "The decisions rest on the premise that the design-record
  axis's findings need no grouping by cause, since the calibration sample catches a rule the
  auditors read apart."
- Horizon: "**Watches until:** 2 passes of the design-record axis, each counted by its report
  file." Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*design-record-axis-pass-*.md' | wc -l`
- Verifier: verified. "\"at least three groups\": no basis in either head, nor in the deleted plan
  document (aaf0e52^ T4 is the session's proposal)".

##### `tripwire@agent-skills@rule-title-wider-than-its-argument` (as2/13, verified, known)

- Outcome: reworded: the slip and the response; `T3` label removed.
- Before: "**Fires when:** a review finds a title wider than its argument in a head written or
  rewritten after the decision. **Response:** repair the title, and reopen
  `design@agent-skills@title-states-the-rule`." After: "**Fires when:** the reviews of two separate
  branches each find a title wider than its argument in a head their branch writes or rewrites, or
  an audit finds one on the main branch in a head written or rewritten after the decision.
  **Response:** reopen `design@agent-skills@title-states-the-rule`; the title found is repaired as
  the instance."
- Horizon: "**Watches until:** 20 commits on the main branch that add a level-three heading to a
  design home, and one pass of the design-record axis." Commands: as for as2/01 and as2/12.
- Verifier: verified, known (eight-responses, severity). "its wide title,
  `a-build-is-tied-to-its-checkout`, was written in 4fe466b (2026-10-04), before the decision
  (68841c5, 2026-10-09); not a firing."

##### `tripwire@agent-skills@ruling-lost-in-assembly` (as2/14, verified; sample, agree)

- Outcome: reworded: the slip; "the strike of \"Evidence when written\" left to the session"
  (cross-check).
- Evidence: "A plan document is committed on its branch, and the branch's transcript review before
  the merge (`design@agent-skills@transcript-review-last-before-merge`) reports such a misstatement
  under its own procedure: that is a slip the clause counts."
- Before: "**Fires when:** a ruling of the owner is found missing from, or misstated in, a committed
  plan document, after the plan document's own reviews, where the misstatement is critical under
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: reversed, or changing what is
  built or a load-bearing decision." After: "**Fires when:** a ruling of the owner is found missing
  from, or misstated in, a plan document that reached the main branch with no review reporting it,
  or in the plan documents of two separate pieces of work, where the misstatement is critical under
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: reversed, or changing what is
  built or a load-bearing decision."
- Horizon: "**Watches until:** 5 plan documents deleted at their landing." Command: `git log --diff-filter=D --name-only --format= <W>..origin/main -- docs/plans/ | grep -Ec '(specs/[^/]+|milestones/[^/]+/README)\.md$'`
  ("tested over cbe804f..the commit audited: 16").
- Verifier: verified. "The optional cut of \"Evidence when written\" drops an attribution the
  guarded head also carries, so no premise is lost." pk/22 records a past firing, handled: 6c5d768,
  "**The fired tripwire.** `tripwire@agent-skills@ruling-lost-in-assembly` fired."

##### `tripwire@agent-skills@split-rule-from-exception` (as2/18, verified)

- Outcome: reworded: the slip, for the review half; `T4` label removed; the merge-back half kept.
- Before: "**Fires when:** a commit merges back two heads split under the decision, or a review
  finds an exception split from its rule." After: "**Fires when:** a commit merges back two heads
  split under the decision; or the reviews of two separate branches each find an exception split
  from its rule in a head their branch writes, or an audit finds one on the main branch."
- Horizon: "**Watches until:** 20 commits on the main branch that add a level-three heading to a
  design home, and one pass of the design-record axis." Commands: as for as2/13.
- Verifier: verified. "the review half is a slip, the merge-back half kept; design.md:2947 citation
  stays true."

##### `tripwire@agent-skills@verifier-refutes-a-true-finding` (as2/20, verified), pending X2

- Outcome: reworded: the slip, on a doubt; `T5` label removed. "If X2 is ruled the other way, as
  as1/15 reads it: kept on one instance."
- Evidence: "A verifier is an agent following its brief, and the session's verification (...) is
  the procedure that reports the instance. One is a slip; a recurrence across passes is the
  failure." The doubt: "the sample is the only reading of refuted drafts, so one wrong refutation
  in it may stand for others unread".
- Before: "**Fires when:** in any pass, the session's sample of refuted drafts holds one that a
  reading confirms." After: "**Fires when:** in two passes, of any axis, the session's sample of
  refuted drafts holds one that a reading confirms."; premise "The decision rests on the premise
  that the adversarial verifier refutes only drafts a reading refutes, since the session reads only
  a sample of the refuted drafts, chosen by its judgement."
- Horizon: "**Watches until:** 3 passes of any audit axis, each counted by its report file."
  Command: `git log --diff-filter=A --name-only --format= <W>..origin/main -- 'docs/audit-reports/*-axis-pass-*.md' | wc -l`
- Verifier: verified. "Slip on the session's sample holds; 841409f line 19 \"the seven refuted
  drafts ... accepted\"".

##### `tripwire@agent-skills@verifiers-wave-findings-through` (as2/21, verified), pending X2

- Outcome: reworded: condition 4; `T6` label removed.
- Before: "**Fires when:** in one pass, the session rejects more than a quarter of a report's
  confirmed findings; the bound is the owner's to reset." After: "**Fires when:** in two passes, of
  any axis, the session's verification rejects a finding a verifier confirmed, on a refutation the
  verifier's brief gives it the record to make."
- Horizon: "**Watches until:** 3 passes of any audit axis, each counted by its report file."
  Command: as for as2/20.
- Verifier: verified. "\"more than a quarter\" is the session's proposal (39bf4fb^ line 2171);
  neither the head nor the plan document carries a measurement or an argument for it (...); \"the
  bound is the owner's to reset\" is a setting."

### 3.2 Applied in the branch: core, crates/core/docs/tripwires.md

#### 3.2.1 Kept (1)

##### `tripwire@core@planned-path-outside-plans` (pk/16, verified), count corrected

- Outcome: kept, on a doubt over conditions 1 and 3.
- Evidence: condition 1, "The check does report it (...). But the finding is the conversion the
  head designs (\"the escape anchor is refused once the work creates the file\"), and its repair is
  a respelling: the finding reaches the reference, not the scope decision." Condition 3: "the
  clause's qualifier, \"in a sentence that said the file was to be created\", is what the head says
  an issue does not do: \"An issue entry states what is missing rather than the shape of its
  fix\"."
- Edit: "**Watches until:** the next two release tags." Command: `git tag --contains <W> --list 'v*' | wc -l`,
  reaching 2. "When it passes unfired, `rejected@core@planned-path-in-every-document` is judged
  again by the recording tests, per the lifecycle."
- Verifier: verified. "Correction to the fired exit: the draft's search gives 18 removed escape
  lines in 15 commits, not 31, and the draft read 10 of those commits. I read the other five
  (78b8ae9, baac6b5, 3a68509, a7db37a, 32ded30). None is a `check` report of a resolved planned
  path. baac6b5 deletes an audit report that named an issue file to be created, in the same commit
  that creates it, so it is no firing, but it bears on the doubt."

#### 3.2.2 Reworded (9)

##### `tripwire@core@candidate-rule-silence` (pk/01, verified, known, fires)

- Outcome: reworded: the response to the shape; `standing` moved to a Watches-until line with its
  reason; **not fired anew** (cross-check).
- Evidence: past firings effc2f8, "`tripwire@core@candidate-rule-silence` fired on the census and
  is narrowed to except the spec's work", and f84d19c, "The possessive token it then met, a
  reference followed by an apostrophe, is now reported". Known: the eight-responses issue records
  "`tripwire@core@candidate-rule-silence`, \"widen the candidate rule to the shape found\"".
- Before: "**Response:** widen the candidate rule to the shape found, at
  `design@core@at-span-candidate-rule`, with a false-positive census over the tree for the widened
  class; or re-accept the silence knowingly and record the shape as the reviewer's. Open a `defect`
  naming the span if the decision is not reopened in the same change. **Re-entry:** standing, and
  the review of the migration that follows the grammar change." After: "**Response:** reopen
  `design@core@at-span-candidate-rule` on the shape found, with the rule widened to it, under a
  false-positive census over the tree for the widened class, among the candidates; or, where the
  decision is not reopened in the same change, open a `defect` naming the span. **Re-entry:** the
  review of the migration that follows a grammar change. **Watches until:** standing: a span the
  scanner leaves silent is reported by no later run, and the class stays after each shape found is
  repaired, so its failure stays silent and permanent."
- Verifier: verified, known, fires ("two past firings, effc2f8 and f84d19c, handled as a standing
  entry"). "`standing` sits in Re-entry with no reason given. The rewording keeps the cause."

##### `tripwire@core@checker-source-literals-are-data-purpose` (pk/02, verified, known), after the ruling on pk/03

- Outcome: reworded, kept on a doubt (conditions 1 and 3): the response to the shape. Applied
  "after the ruling on pk/03" (cross-check).
- Evidence: the response "opens with an account of the firing, not a reopening or an issue";
  condition 3, "a constant added from habit shows the decision unused rather than wrong".
- Before: "**Response:** the interpolation habit has outlived its reason, or a literal is still
  read where the decision says it is not. Establish which: if the checker reported the plain
  literal, that is the self-location tripwire above; otherwise open an `observation` and rewrite
  the fixture inline. **Re-entry:** standing." After: "**Response:** open an `observation` naming
  the constant and the fixture; a plain literal the checker reported is the self-location
  tripwire's instead. **Re-entry:** a review of a change that adds a test module under the
  checker's own directories. **Watches until:** the next five commits on the main branch that
  change a file under `path@core@src/`." "If (...) self-location is deleted (draft 03), the
  response's last clause is dropped with it".
- Horizon command: `git log --format=%h <W>..origin/main -- crates/core/src/ | wc -l`, reaching 5.
- Verifier: verified, known. "The doubt on condition 3 holds, since the head argues against
  interpolated constants (design.md:2304-2306) but does not forbid them."

##### `tripwire@core@checker-source-literals-are-data-whole-source-scope` (pk/04, verified, known)

- Outcome: reworded, kept on a doubt (condition 2): the response to the shape.
- Evidence: "`git grep -nE 'CR: ?[0-9]' -- '*.rs'` returns nothing"; condition 2, "the `CR:` marker
  is thaum's rules extension's (...). The evidence then lands in thaum's tree, and reaches this
  project only through a retrospective file the intake receives".
- Before: "**Response:** the whole-source scope gave that citation up on a count of zero. Either
  move the citation into the comment above the literal, or reopen the scope half of the decision
  and narrow it to test modules, which costs an attribute lookup in the extractor. **Re-entry:**
  standing." After: "**Response:** reopen the scope of `design@core@checker-source-literals-are-data`,
  with the rule narrowed to test modules among the candidates, at the cost of an attribute lookup
  in the extractor; moving the citation into the comment above the literal repairs the instance.
  **Re-entry:** the intake of a retrospective file from a project whose binary registers an
  extension. **Watches until:** the next three commits on the main branch that add an analysis
  under `path@knowledge-architect@docs/retrospective-reports/`."
- Horizon command: `git log --diff-filter=A --format=%h <W>..origin/main -- docs/retrospective-reports/ ':!docs/retrospective-reports/README.md' | wc -l`, reaching 3.
- Verifier: verified, known. "The condition 2 doubt (thaum's tree, through a retrospective) is
  sound."

##### `tripwire@core@fix-makes-a-choice` (pk/06, verified, fires), citation corrected

- Outcome: reworded: "proposed" struck on condition 1; **not fired anew**.
- Evidence: the spec's table: "`tripwire@core@fix-makes-a-choice`: it fired on the proposal this
  spec plans, per D4 | ... | per D4: its firing clause narrowed in this spec's branch, on the
  owner's word; deleted at the harvest". Condition 1: "A proposal of such a fix is \"a request for
  what the decision refused\"".
- Before: "**Fires when:** a fix is proposed for, or added to, `check --fix` whose bytes are not
  fully determined by the tree and the pinned version, ... **Response:** reopen
  `design@core@safe-fix-definition` before the fix is added, rather than widen what `--fix` does
  under its present argument." After: "**Fires when:** a fix is added to `check --fix` whose bytes
  are not fully determined by the tree and the pinned version, ... [the rest of the clause
  unchanged, the exception of `spec@plans@path-quickfixes` and \"That harvest deletes this entry.\"
  included] **Response:** reopen `design@core@safe-fix-definition`, rather than widen what `--fix`
  does under its present argument. (...) **Watches until:** the commit on the main branch that
  deletes `spec@plans@path-quickfixes` at its landing, which deletes this entry."
- Horizon command: `git log --diff-filter=D --format=%h <W>..origin/main -- docs/plans/specs/path-quickfixes.md`, one commit.
- Verifier: verified, fires ("the past firing on the proposal of `spec@plans@path-quickfixes`,
  handled by the narrowing per D4"). "One correction: the spec's row is at path-quickfixes.md:227,
  not :223."

##### `tripwire@core@init-cleanup-beyond-the-listed-files` (pk/08, verified)

- Outcome: reworded: condition 2 strikes "the owner's observation"; the `T1` "on the owner's word"
  reworded to the premise; kept on a doubt over condition 1.
- Evidence: 32a0ab4^, "Q6. Which of the five premortem items, AC1, AC2, AC3, T1 and T2, are applied
  or recorded? Answer: all five." Condition 2: "Evidence that exists only in a session's
  transcript, or only in what the owner sees while driving a session, reaches no occasion that
  reads the tripwires."
- Before: "T1 of the premortem of the plan that built `init`, on the owner's word. **Fires when:**
  one report, an issue, a retrospective finding or the owner's observation, of an `init` run in an
  unintended directory that needed a cleanup beyond deleting the files its output listed, or that
  lost a file. (...) **Re-entry:** the standing-state review." After: "The decisions rest on the
  premise that the worst a mistaken run does is add files, which the cleanup removes by deleting
  the files the run's output lists, under the refusal of a directory holding files outside every
  worktree that the owner chose over following `cargo init` exactly. **Fires when:** an issue or a
  retrospective finding reports an `init` run in an unintended directory that needed a cleanup
  beyond deleting the files its output listed, or that lost a file. (...) **Re-entry:** the
  standing-state review, and each intake of a retrospective file. **Watches until:** the third
  release tag after the commit that writes this line, the first of them the release that ships
  `init`."
- Horizon command: `git tag --contains <W> --list 'v*' | wc -l`, reaching 3.
- Verifier: verified. "The owner's round-2 cleanup worry (#arg-16) was hedged (\"IMO\", \"Maybe\")
  and set aside by D3 \"on the session's argument\". So no stated premise is dropped. The
  attribution of shape B (Q5: B) is kept".

##### `tripwire@core@inputs-builder-needed` (pk/09, plausible): cross-check departs from the draft, X3

- Draft's outcome: removed, condition 1. Cross-check's: "**reworded, kept on a doubt**, aligned with
  private-item-needed (pk/18): the response brought to the shape (one of the eight, \"open a
  `todo`\" for the builder), horizon the next three release tags".
- Evidence: "**Fires when:** an extension's test needs an `Inputs` value that `Gathered::over`
  cannot produce from a tree on disk, such as an `Outside::Unreadable` planted in memory. `Inputs`
  is non-exhaustive and `Gathered`'s fields are private, so such a test cannot be written." Draft:
  "By its own words, the failure is a test that \"cannot be written\": the work that meets it is
  stopped".
- Draft's edit: delete; repair the extension issue line 74 "to state the constraint itself, for
  example \"a field of `Inputs` cannot be set by an extension's test, since `Gathered`'s fields are
  private\"".
- Verifier: plausible. "the same auditor's draft 18 keeps the analogous `private-item-needed` on the
  reading that a consumer working around a missing item \"fails nothing\". Here, an extension
  author who tests over a mock project or omits the test fails nothing either. (...) The drop then
  rests only on the doubt's weight (the extension is the owner's own, thaum's), which the draft does
  not invoke".
- Cross-check: "If the session drops it on that weight instead (...): owner list, removals on
  condition 1, the ground named as the doubt's weight; and pk/18 is weighed alike." Horizon
  command, as for pk/18: `git tag --contains <W> --list 'v*' | wc -l`, reaching 3. No step wrote
  the reworded response's text.

##### `tripwire@core@issue-kind-list-grows` (pk/10, verified)

- Outcome: reworded: condition 4.
- Evidence: "the commit that wrote the bound, a9ea4f2 (`git log -S'exceeds ten kinds'`), argues
  none: its message says only \"The tripwire on the closed issue-kind list recorded with a slug.\""
- Before: "**Fires when:** the declared kind list of any issue register exceeds ten kinds, or a
  kind added after the list was closed — one outside the six compiled defaults — ends up with a
  single member across the whole project." After: "**Fires when:** a kind outside the six compiled
  defaults, declared in an issue register of this project or of a consuming project as a
  retrospective file the intake receives reports, ends up with a single member across that
  project. **Watches until:** the next three release tags."
- Horizon command: `git tag --contains <W> --list 'v*' | wc -l`, reaching 3.
- Verifier: verified. "Dropping the first clause rather than naming its event is right, since a
  declared kind is what the head endorses (\"a reviewed manifest diff\")."

##### `tripwire@core@phases-gate-the-report-two` (pk/14, verified): split, creates `a-second-producer-chain`

- Outcome: reworded: split into two entries; "proposed or" struck on condition 1.
- Evidence: "The entry says \"Two clauses, one decision\", and has two `**Fires when:**` and two
  `**Response:**` lines. The premises differ: (a) the whole-report stop is not felt as an obstacle
  to route around; (b) there is one producer chain. They lose to different rivals". "A proposal of
  a way around the gate is \"a request for what the decision refused\"".
- After, entry 1: "## Guarding `design@core@phases-gate-the-report`: a way around the whole-report
  stop (slug phases-gate-the-report-two)", "**Fires when:** a flag, an environment variable or a
  manifest row is added that lets a run report a later phase while an earlier one holds findings,
  or lets a writer write over an incomplete model (...) **Watches until:** the next three release
  tags." Entry 2: "## Guarding `design@core@phases-gate-the-report`: a second producer chain
  (slug a-second-producer-chain)", "**Fires when:** a check of the core, under `path@core@src/check/`,
  or a check of an extension reads another check's findings, or a check's output is stored for
  another check to consume. **Response:** open a `design` issue proposing the new phase and its
  place in the order (...) **Watches until:** the next ten commits on the main branch that change a
  file under `path@core@src/check/`." (Full text in pk/14.)
- Horizon commands: `git tag --contains <W> --list 'v*' | wc -l`, reaching 3;
  `git log --format=%h <W>..origin/main -- crates/core/src/check/ | wc -l`, reaching 10.
- Verifier: verified. "the head's \"One producer chain\" paragraph is separate from the whole-report
  stop and \"A tripwire names the event\", so the split is right." Cross-check: "The split creates a
  tripwire. The application judgement of the axis then judges the new entry, since its text
  changed."

##### `tripwire@core@private-item-needed` (pk/18, verified, fires), X3

- Outcome: reworded, kept on a doubt over condition 1: the response's issue given its kind;
  **not fired anew**.
- Evidence: "the issue records the silent way out: \"An extension that has to guess membership from
  paths quietly breaks where the manifest says something else\". A consumer that works around the
  missing item fails nothing."
- Before: "**Response:** open an issue for the instance. Its fix gives ..." After: "**Response:**
  open a `todo` for the instance. Its fix gives ..." and "**Watches until:** the next three release
  tags."
- Horizon command: `git tag --contains <W> --list 'v*' | wc -l`, reaching 3.
- Verifier: verified, fires ("the past firing, recorded as
  `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`, handled as standing").

### 3.3 Applied in the branch: xtask, tools/xtask/docs/tripwires.md

None. Its one entry, `tripwire@xtask@tip-alone-is-built`, is proposed removed (3.6). pk/21 proposed
the SHA repair "applied in the branch whatever the ruling"; under removal the cross-check calls it
moot.

### 3.4 Converted (4): owner list

##### `tripwire@agent-skills@audit-passes-do-not-converge` (as1/10, verified)

- Rests on condition 1. Evidence: "**Fires when:** from one pass to the next of an audit, the
  violations left on old text do not fall, or the severe ones among them, those whose outcome is
  the owner list, do not fall while they are above zero." Ground: "AC4 of the discussion that made
  the decision, proposed by the owner as an acceptance criterion and kept as a tripwire, on the
  owner's word, to watch future audits." First disjunct: "A pass where that count does not fall
  goes to the owner as a question, with the counts, and starts no pass on its own." Second:
  `skill@knowledge-architect-project-audit@record-of-an-audit` asks for "the count of violations left on old text and on new text ... and of
  those on old text that went on the owner list".
- Edit: "in the re-check's report, a line comparing the count of violations left on old text that
  went on the owner list with the previous pass's count (...). It is not small enough for this
  branch, since it touches three saved workflows: a `todo` issue. The entry leaves once the check
  exists." "referenced by nothing".
- Verifier: verified. "The stop rule quote was checked (...). So the first disjunct is not silent.
  record-of-an-audit asks for the counts (...), so the second disjunct is computable. The owner's
  criterion goes to the owner list."

##### `tripwire@agent-skills@duplicate-auditors-add-nothing` (as1/20, verified)

- Rests on condition 1. Evidence: "**Fires when:** in one pass, for every lens dispatched more than
  once, one of its agents alone reported every confirmed finding of that lens." The head: "Its two
  auditors by default rest on the premise above, that a second agent of one lens finds what the
  others missed."
- Edit: "For each lens with more than one agent, a script groups the confirmed findings by the
  agents whose drafts they merge, and reports when one agent holds them all. It runs in the saved
  workflow of the agentic-workflow axis, after the dedup step". "Not small enough for this branch:
  it needs the confirmed-findings file to name each finding's drafts in a shape a script reads. A
  `todo` issue." "referenced by nothing".
- Verifier: verified. "The firing is computable from the confirmed findings and their drafts. A
  `todo` is right, since the data shape is missing."

##### `tripwire@core@grammars-not-prefixes` (pk/07, refuted on its ground; sample, agree)

- Rests on condition 1, "Where a script could compute the firing, write that check instead."
  Evidence: "**Fires when:** a `.rs` file in the tree yields no prose region, or no item scope,
  while its text holds a top-level `fn` — or a `tree-sitter-rust` or `pulldown-cmark` upgrade
  changes a node name the extractor asks for." as1/25: "`grep -rn \"id_for_node_kind\|node_kind_is\|node-types\"
  crates/core/src crates/core/tests` returns nothing. pulldown-cmark's events are Rust enums, so a
  renamed one fails to compile. Only tree-sitter's string names change silently."
- Edit, as the cross-check carries it: "both tests written in the branch: a node-kind test for
  every kind the extractor names, against `tree_sitter_rust::LANGUAGE`, and a test over the tracked
  `.rs` files, as as2/22 proposes; design.md:2285 repointed". as2/22's tests: "1. a unit test in
  crates/core/src/source/rs.rs asserting, for every node kind the extractor names (`ITEMS`,
  `NAMES`, the comment and literal kinds, `string_content`, `doc_comment`, `closure_parameters`),
  that the grammar knows it: `tree_sitter::Language::from(tree_sitter_rust::LANGUAGE).id_for_node_kind(kind, true) != 0`;
  2. a test over the workspace's tracked `.rs` files: each file whose text holds a `//` comment
  yields at least one prose region, and each holding a top-level `fn` at least one item scope."
- Verifier: refuted (reason in part 1).

##### `tripwire@core@walked-count-differs-between-machines` (pk/20, refuted; sample, R1)

- Rests on condition 1 (R1). Evidence: the head, "What still decides the walk beside the tree's own
  ignore files is git's per-clone exclude file, which git offers no way to pin, and that is what
  `tripwire@core@walked-count-differs-between-machines` is left guarding."
- Edit: "`todo` issue for the check as1/26 gives (the walk reports the files the per-clone exclude
  file alone removes); design.md:518 and 505 repaired when it leaves". as1/26: "the checker, at the
  walk, lists the untracked files that git's per-clone exclude file alone removes, and reports them
  in its summary block or as a finding. It runs at every `check`, in every project." Fallback if the
  owner rules the conversion out: "a rewording that names a clean tree".
- Verifier: refuted (reason in part 1).

### 3.5 Removed, condition 1 (10): owner list

##### `tripwire@agent-skills@auditor-skips-part-of-the-corpus` (as1/11, verified)

- Evidence: "**Fires when:** an auditor's return says it did not read every file of the corpus
  whole." The head: "an auditor's return that says it did not read every file whole reopens how the
  corpus is shared out." agentic-workflow-audit.js:301 writes into the report header "each auditor
  return that says a corpus file was not read whole".
- Edit: delete; `rejected@agent-skills@workflow-corpus-shared-out` (rejected.md:274), kept on a
  doubt naming this entry: draft's default "the repointing" to the head's clause, "Or the entry is
  judged again by the recording tests, its removal going to the owner if none holds."
- Verifier: verified. "The event is reported by a procedure and reopens the decision by the head's
  own text, so condition 1 excludes it."

##### `tripwire@agent-skills@real-use-refusal-overruled` (as2/08, verified), X1

- Evidence: "**Fires when:** the owner overrules a refusal, made under the real-use rule, of an
  addition to a project's own agent-facing text." "The firing is the owner's own request for the
  addition the rule refused". Keeping reading met by the doubt's weight, "unless the owner would see
  the failure anyway ... then a wrong drop costs little".
- Edit: delete; "referenced by nothing".
- Verifier: verified. "the known consumers being the owner's."

##### `tripwire@agent-skills@structure-rejection-overruled` (as2/19, verified), X1

- Evidence: "**Fires when:** a review or a design discussion rejects an instruction by citing
  `design@agent-skills@capabilities-not-structure` or `goal@agent-skills@installed-text-leaves-room-to-judge`,
  and the owner overrules that rejection." "No reading makes the failure silent."
- Edit: delete; "referenced by nothing".
- Verifier: verified. "the keeping reading (a pattern of overrules no single one reopens) is a doubt
  the owner's weight lets drop, since the owner makes each overrule."

##### `tripwire@core@checker-source-literals-are-data-self-location` (pk/03, verified)

- Evidence: first half, "a `cargo klarch check` run reports a finding on a string literal in a Rust
  file under one of the tool's own directories": "a finding of `cargo klarch check`". Second half:
  "The test `the_core_binary_names_its_own_directory_and_counts_the_files_under_it` in
  `path@core@tests/binary.rs` (line 295) (...) `a_tree_holding_the_tool_at_another_path_than_the_binarys_is_refused`
  (line 1441)."
- Edit: delete; the eight-responses issue:24; the purpose entry's response, "the self-location
  tripwire above" (pk/02).
- Verifier: verified. "I found no silent reading." Cross-check: "If ruled kept, its response must be
  brought to the shape for the eight-responses issue to close."

##### `tripwire@core@extension-defines-a-kind` (pk/05, verified)

- Evidence: "**Fires when:** a proposed feature, in a design discussion or a review, needs a
  reference whose kind an extension defines rather than the core ... and the need is stated as a
  requirement rather than as an option." The head: "An extension cannot add a kind to the entity
  table. A reference kind is the core's ... Nothing needs that today".
- Edit: delete; design.md:241-242, "Nothing needs that today, and `register@core@tripwire` names
  the event that would." becomes "Nothing needs that today."
- Verifier: verified. "The clause's \"stated as a requirement\" is that request."

##### `tripwire@core@nested-entries-asked` (pk/12, verified)

- Evidence: "**Fires when:** a project's owner, in a design discussion, an issue or a review, asks
  for a Directory register carried by an entry of a Directory register." The head: "a Directory
  register carried by an entry would nest anchors inside anchors, which no case has asked for."
- Edit: delete; "referenced by nothing".
- Verifier: verified. "T1 carries no attribution."

##### `tripwire@core@new-component-blocked-by-the-gate` (pk/13, verified)

- Evidence: "**Fires when:** one reported case of `new-component` refused over a finding of phases
  1 to 3 that the user could not repair before adding the Component, so that the user added it by
  hand." The head: "over a model phases 1 to 3 find incomplete it refuses, exit 2, writing nothing".
  Q6 "Answer: all five" is an approval.
- Edit: delete; "referenced by nothing".
- Verifier: verified. "A case nobody reports leaves no evidence (condition 2)." Re-read after pk/18:
  "13 stands, because its clause fires only on a report".

##### `tripwire@core@plain-word-neither-pointer-nor-subject` (pk/15, plausible), on the doubt's weight

- Evidence: "**Fires when:** at a consuming project's upgrade, or here, the owner rules a finding of
  the plain-word lint to be on a word that is neither a pointer nor a subject slug". The head:
  "**The rule is the owner's**, given against the session's proposal of a listing in `show`".
- Edit: delete; design.md:2025-2026, "`register@core@tripwire` watches the premise: ...", repaired.
- Verifier: plausible. "a session that renames the slug counts the false finding as the lint
  working. The failure then reaches the decision only through the owner's ruling. Condition 1 is
  therefore not clear. The removal rests on the doubt's weight, which the draft names and which
  applies, since the firing is the owner's own ruling." Cross-check: "the item states that condition
  1's words are not clear here, and that the drop rests on the doubt's weight".

##### `tripwire@core@plans-directory-split-asked` (pk/17, verified)

- Evidence: "**Fires when:** a project's owner, in a design discussion, an issue or a review, asks
  for a plans directory per component, or for more than one plans directory in a project." The
  head: "A project has one plans directory, in its root component". The owner's two premises stay
  in the head.
- Edit: delete; design.md:899, "`tripwire@core@plans-directory-split-asked` watches the premise.",
  repaired.
- Verifier: verified.

##### `tripwire@core@staged-pass-fails-the-checkout` (pk/19, verified)

- Evidence: "**Fires when:** a commit whose tree passed `check --staged` just before it was made
  then fails `check` in CI, or the tree half of `commits`, with no file edited in between." "The
  failure is a red CI run". Condition 2 for the staged half: "a run in the session's shell, recorded
  nowhere else."
- Edit: delete; the staged-check issue:45 in words; path-quickfixes.md:248; design.md:631-632.
- Verifier: verified. "The round-3 words at 1543579^ are \"Table approved, keep all tripwires and
  acceptance criteria.proceed to the spec\". That is an approval to record and states no premise".

### 3.6 Removed, condition 3 (4): owner list

##### `tripwire@agent-skills@audit-defaults-overruled` (as1/08, verified)

- Evidence: "**Fires when:** in one audit, the owner rules against the default on more than a
  quarter of the items of the owner list; the bound is the owner's to reset." Condition 4 then 3:
  "A ruling against a default is the owner's word, given where the head sends it." The premise
  "gives the owner defaults the owner mostly accepts" is not in `design@agent-skills@audit-method`.
- Edit: delete; SKILL.md:136 `%%` line "%% The count of rulings against the default is read by
  `tripwire@agent-skills@audit-defaults-overruled`." deleted in the same commit.
- Verifier: verified. "The owner rules every item, so the doubt weight also applies."

##### `tripwire@agent-skills@component-judgement-noisy` (as1/14, verified)

- Evidence: "**Fires when:** in one pass, the session or the owner rejects more than half of the
  findings of a head in the wrong Component, on at least 4 such findings; the bounds are the
  owner's to reset." The head sends such a finding to the owner list; "The premise \"mostly right\"
  is not in the head."
- Edit: delete; `rejected@agent-skills@wrong-component-outcome-removed` (rejected.md:425) judged
  again by the recording tests; tripwire-standard.md:199.
- Verifier: verified.

##### `tripwire@core@item-of-another-plan-named` (pk/11, verified)

- Evidence: "**Fires when:** a plan document's prose names an item of another plan document, by its
  id or by its statement, because no citation of it can be written." The head: "the repair names the
  document to cite whole (...) with the item named beside it as `#<id>`". "every session that
  follows the finding's repair fires the clause."
- Edit: delete; design.md:960, "`tripwire@core@item-of-another-plan-named` watches it.", dropped.
- Verifier: verified. "That is exactly what the clause counts."

##### `tripwire@xtask@tip-alone-is-built` (pk/21, refuted; sample, R2)

- Evidence: the head, "A commit before the tip that does not compile passes CI, and is found only
  by whoever builds it later." The entry: "**Fires when:** a `git bisect` over `main` stops on a
  commit that does not compile, among the commits from the restructure commit d2f0ce4 onwards."
- Edit: delete; "referenced by nothing". tools/xtask/docs/tripwires.md is left with no entry.
- Verifier: refuted (reason in part 1).

### 3.7 Fired (1): the session judges

##### `tripwire@agent-skills@search-missed-before-the-work` (as2/16, verified, fires)

- Evidence: the clause, "across sessions, 2 standing-state reviews report a standing entry the work
  bears on, which no search before the work returned". The entry's instance: 91565fc, "gains its
  first instance". The unlisted one, 2712ecd: "Recorded here for the count
  `tripwire@agent-skills@search-missed-before-the-work` keeps across sessions: one instance of its
  firing evidence, the standing-state reviewer of this branch's spec reporting two open entries the
  search before the work did not return (...). One instance; the tripwire fires at two." Both on
  main's history, "2712ecd at 13:08 and 91565fc at 17:06 on 2026-10-19, on two specs". "Not
  established: that the two were separate sessions".
- Edit: "Open a `design` issue, title proposed: \"The search before the work missed standing entries
  the work bore on in two pieces of work\" (...) and reopen
  `design@agent-skills@standing-entries-searched-before-the-work` on its occasions, with a search
  when convergence is proposed, before the premortem, among the candidates." "Delete the entry."
- Verifier: verified, fires. "Error of detail: cebb978's \"not met: a searcher returned the four
  entries\" is in the analysis of the thaum driven-tests workflow file, not of the lint spec; it
  does not bear on either instance."

## 4. The overlap map and the stale texts

From the cross-check, parts 1 and 2. "No two drafts of record judge the same tripwire: each entry
has exactly one. The overlaps are findings whose edits touch one *other* entry, a tripwire or an
issue, or one text." Pairs proposed absorbed: none ("I found no pair that guards one premise and
that the drafts missed").

### 4.1 Findings that touch one entry or one text

| entry or text touched | findings | how they meet |
| --- | --- | --- |
| `issue@agent-skills@eight-tripwires-respond-with-an-action-the-tripwire-shape-does-not-name`, lines 19 to 27 | as2/03, 07, 13; pk/01, 02, 04 (reworded); pk/03 (removed); pk/09 (removed in its draft, kept on a doubt by the cross-check) | "The issue closes only if every one of the eight lands. If the owner keeps `checker-source-literals-are-data-self-location`, or the session keeps `inputs-builder-needed` (part 3 proposes it), neither removal draft gives a response rewording to fall back on, and the session writes one." |
| `issue@agent-skills@a-procedure-tripwire-states-no-severity-bound`, lines 32 to 38 | as1/07; as2/06, 13, 14, 15, 17 | "After the rewordings, three of its \"four\" no longer fire on one instance." Closes at step 5 of the spec (spec line 208). X4. |
| `tripwire@core@checker-source-literals-are-data-purpose`, its Response | pk/02, pk/03 | "The two are applied together, after the owner's ruling on 03." |
| `tripwire@agent-skills@search-missed-before-the-work`, Instances line (tripwires.md:81 to 84) | as2/16 (deleted); as2/14 and as1/07 (reword the two cited entries) | "No conflict: the rewordings keep both slugs, and the deletion removes the citing line." |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, lines 42, 86, 90 | as2/16; as1/16 | Line 90 "begins \"A second tripwire of the same class\", counted from the tripwire at line 86. Once search-missed is deleted, \"A second\" counts from nothing." |
| `issue@agent-skills@a-pointer-delivers-its-directive-only-when-the-session-fetches-it`, line 99 | as2/06 | "One report no longer fires it after the rewording." |
| `issue@core@an-extension-cannot-see-which-register-a-document-belongs-to`, lines 55 and 74 | pk/18; pk/09 | Line 74 repaired only if 09's removal is ruled. X3. |
| `design@core@phases-gate-the-report` (design.md:112) | pk/13 (removed); pk/14 (split, new slug `a-second-producer-chain`) | design.md:112 "stays true". "The split creates a tripwire." |
| `rejected@agent-skills@workflow-corpus-shared-out` (rejected.md:274) | as1/11 | kept on a doubt naming the deleted entry |
| `rejected@agent-skills@wrong-component-outcome-removed` (rejected.md:425) | as1/14 | the same |
| crates/agent-skills/content/skills/project-audit/SKILL.md, lines 136 and 143 | as1/08 | line 143 "is left with no act needing it" |
| docs/plans/specs/path-quickfixes.md | pk/01 (240), 06 (223 to 1168), 14 (226, 424), 19 (248) | "Only 19 deletes. 248 is repaired in the deleting commit." |
| docs/plans/specs/tripwire-standard.md, lines 193, 196, 199 | as2/06, as2/15, as1/14 | 193 and 196 stay true; 199 dangles once `tripwire@agent-skills@component-judgement-noisy` is deleted |
| crates/core/docs/design.md, `register@core@tripwire` pointers at 112, 242, 505, 632, 1175, 1699, 2025 | pk/14 (112), 05 (242), 20 (505, 518), 19 (632), 10 (1175), 01 (1699), 15 (2025) | "`cargo klarch show` does not list these, since they cite the register, not the entry." |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | as2/01 | its recorded instances "may already" meet the reworded clause |

### 4.2 Entries guarding one head, judged differently

- **X2, `design@agent-skills@verifier-in-the-run`**: as1/15 keeps `tripwire@agent-skills@conforming-draft-wrong` firing on
  one instance, "The slip rule does not bind it: the clause watches the coverage of the verifier's
  sample"; as2/20 rewords `tripwire@agent-skills@verifier-refutes-a-true-finding` to "in two passes", "A verifier is an
  agent following its brief (...). One is a slip"; as2/21 likewise. "The two clauses are symmetric
  (...). Two verifiers accepted opposite readings of one sentence of the standard."
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: settled by the refutation of as1/03.
- `design@agent-skills@bounded-path-in-design`: "a different rule applied to a different clause, so
  no difference stays."
- `design@agent-skills@standing-entries-searched-before-the-work`: "If the reopening moves where the
  search runs, the premise the kept entry guards changes with it. The session reads
  deferred-trigger again when it rules on the reopening."
- `design@agent-skills@attribution-replaces-quotation`: X4 applies.
- **X1**, across heads: "Is the owner's own overrule of what a decision produced \"a request for
  what the decision refused\" under condition 1, or a failure the owner sees anyway?" Removed on it:
  as2/08, as2/19. Kept or reworded: as1/01, as1/02, as1/06. "The verifiers verified both sides."
- **X3**, across heads: "Does a consumer that works around a missing library item fail nothing?"
  pk/18 keeps; pk/09 removes.
- **X4**: the severity bound on the three entries on a misstated ruling; "the session's to align.
  It is no owner-list item."

### 4.3 Stale texts: entries deleted

| entry (draft, outcome) | citing text | after the deletion |
| --- | --- | --- |
| `tripwire@agent-skills@audit-defaults-overruled` (as1/08, removed c3) | crates/agent-skills/content/skills/project-audit/SKILL.md:136, `%%` line | dangling; deleted in the same commit |
| | the same file, line 143: "too how many of the owner's rulings on the owner list went against the default" | "not false, but its only reader is gone. The deleting commit judges it by the agent-text standard: delete it, or name another reader. The draft does not name it. It is installed text: edited under crates/agent-skills/content and installed again" |
| `tripwire@agent-skills@auditor-skips-part-of-the-corpus` (as1/11, removed c1) | crates/agent-skills/docs/rejected.md:274, `rejected@agent-skills@workflow-corpus-shared-out`, kept on a doubt naming the entry | dangling; "the entry is judged again by the recording tests, its removal put to the owner if none holds." The draft's repointing "is one outcome of the re-judgement, not a substitute for it." |
| `tripwire@agent-skills@component-judgement-noisy` (as1/14, removed c3) | crates/agent-skills/docs/rejected.md:425, `rejected@agent-skills@wrong-component-outcome-removed`, kept on a doubt naming the entry | dangling; judged again by the recording tests |
| | docs/plans/specs/tripwire-standard.md:199 | dangling. "A reference that resolves to nothing is a `check` finding at the deleting commit, so it cannot wait for the harvest. The spec's own work removes the entry, so the row writes its bare slug in backticks" |
| `tripwire@agent-skills@search-missed-before-the-work` (as2/16, fired) | the agent-behaviour issue, line 42 | dangling; rewritten to name the new `design` issue |
| | the same issue, line 86, quoting the owner's word on the tripwire's wait | dangling; rewritten; the owner's word goes on the owner list with the firing |
| | the same issue, line 90, "A second tripwire of the same class" | false once line 86 names no first; "Not named by any draft." |
| | crates/agent-skills/docs/tripwires.md:84 | inside the deleted entry |
| `tripwire@core@checker-source-literals-are-data-self-location` (pk/03, removed c1) | the eight-responses issue:24 | dangling, unless the issue closes at step 5 |
| | the purpose entry's Response, "the self-location tripwire above" | false; pk/02 repairs it |
| `tripwire@core@extension-defines-a-kind` (pk/05) | crates/core/docs/design.md:242 | false; "Nothing needs that today." |
| `tripwire@core@inputs-builder-needed` (pk/09, if removed) | the eight-responses issue:27; the extension issue:74 | dangling; repaired; "Not needed if part 3's proposal, keep, is taken" |
| `tripwire@core@item-of-another-plan-named` (pk/11) | crates/core/docs/design.md:960 | dangling; the sentence dropped |
| `tripwire@core@nested-entries-asked` (pk/12), `tripwire@core@new-component-blocked-by-the-gate` (pk/13) | referenced by nothing | none |
| `tripwire@core@plain-word-neither-pointer-nor-subject` (pk/15) | crates/core/docs/design.md:2025 | false; repaired |
| `tripwire@core@plans-directory-split-asked` (pk/17) | crates/core/docs/design.md:899 | dangling; repaired |
| `tripwire@core@staged-pass-fails-the-checkout` (pk/19) | the staged-check issue:45 | dangling; repaired in words |
| | docs/plans/specs/path-quickfixes.md:248 | dangling. "A bare slug is not open here, since this spec's work does not remove the entry." |
| | crates/core/docs/design.md:632 | false; repaired |
| `tripwire@agent-skills@real-use-refusal-overruled`, `tripwire@agent-skills@structure-rejection-overruled`, `tripwire@agent-skills@agent-text-findings-need-nothing`, `tripwire@agent-skills@audit-drafts-mostly-noise`, `tripwire@agent-skills@audit-passes-do-not-converge`, `tripwire@agent-skills@duplicate-auditors-add-nothing` | referenced by nothing | none |
| `tripwire@xtask@tip-alone-is-built` (pk/21, R2) | referenced by nothing | "tools/xtask/docs/tripwires.md is left with no entry, like the root's and gates' homes at the commit audited. Its wrong SHA, d2f0ce4, goes with it." |
| `tripwire@core@grammars-not-prefixes` (converted) | crates/core/docs/design.md:2285 | repointed to the tests in the same commit |
| `tripwire@core@walked-count-differs-between-machines` (converted) | crates/core/docs/design.md:518 | repointed to the check |
| | crates/core/docs/design.md:505, "The tripwire is in `register@core@tripwire`." | false once deleted; "the conversion's `todo` issue carries it" |

"No other `rejected` entry kept on a doubt names an entry that an outcome deletes."
`rejected@core@planned-path-in-every-document` (core rejected.md:12) names the kept
`tripwire@core@planned-path-outside-plans` and "is judged again when that horizon passes".

### 4.4 Stale texts: entries reworded or kept with an edit

| entry (draft) | citing text | after the edit |
| --- | --- | --- |
| `tripwire@agent-skills@pointer-not-followed` (as2/06) | the pointer issue:99, "the report also fires `tripwire@agent-skills@pointer-not-followed`" | **false**. "Repairing it in the rewording's commit is `primer@met-outside-the-task` case 2." |
| `tripwire@agent-skills@pointer-not-followed`, `tripwire@agent-skills@rule-title-wider-than-its-argument`, `tripwire@agent-skills@attribution-loses-the-scope`, `tripwire@agent-skills@ruling-lost-in-change` | the severity issue, lines 32 to 38 | **false** for the first three; leave with the issue at step 5, else repaired |
| `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@rule-title-wider-than-its-argument`, `tripwire@agent-skills@rarer-rule-in-the-model`, `tripwire@core@candidate-rule-silence`, `tripwire@core@checker-source-literals-are-data-purpose`, -whole-source-scope | the eight-responses issue, lines 19 to 27 | **false** once each response is reworded; closes at step 5 if the removals also land |
| `tripwire@agent-skills@record-check-misses-a-head` (as2/10) | crates/agent-skills/docs/rejected.md:385 | true; "\"misses one\" now reopens nothing until two audits miss one. Read again; no repair proposed." |
| `tripwire@agent-skills@attribution-loses-the-scope` (as1/07) | crates/agent-skills/docs/rejected.md:47 | true, narrowed |
| `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work` (as1/16) | the agent-behaviour issue, line 90 | false only through the deletion of search-missed |

Judged to stay true after the edit (cross-check 2b): the citing texts of a-shortcut-decision,
`tripwire@agent-skills@agent-text-written-without-the-standard`, `tripwire@agent-skills@argued-closure-reopened`, `tripwire@agent-skills@attribution-loses-the-scope`
(design.md:790, the plan-documents issue:44), `tripwire@agent-skills@calibration-reading-rejected`,
`tripwire@agent-skills@design-entry-search-misses`, `tripwire@agent-skills@expectation-set-closes-a-contradiction`,
`tripwire@agent-skills@head-created-without-deliberation`, `tripwire@agent-skills@owner-intent-stripped`, `tripwire@agent-skills@pointer-not-followed` (rejected.md:204,
the pointer issue:29, tripwire-standard.md:193), `tripwire@agent-skills@rule-misread-across-groups`,
`tripwire@agent-skills@ruling-lost-in-assembly`, `tripwire@agent-skills@ruling-lost-in-change`, `tripwire@agent-skills@split-rule-from-exception`,
`tripwire@agent-skills@verifier-refutes-a-true-finding`, `tripwire@agent-skills@skipped-spec-audit-would-have-found-a-gap`, `tripwire@core@candidate-rule-silence`,
`tripwire@core@fix-makes-a-choice`, `tripwire@core@phases-gate-the-report-two`, `tripwire@core@init-cleanup-beyond-the-listed-files`,
`tripwire@core@private-item-needed`, `tripwire@core@issue-kind-list-grows`.

## 5. The session's verification

The session read this report whole; the calibration's three readings against the sample's drafts,
and confirmed R1, R2 and R3; every item of the owner-list draft against its draft; the five
plausible verdicts; the six drafts marked as firing; and, as samples of its choosing, the kept
drafts as1/13 and pk/16 and the reworded draft pk/14, with no correction. Its decisions on the
points the run left to it:

- **X3**: `tripwire@core@inputs-builder-needed` is removed on condition 1, the work that meets its
  failure being a test its author cannot write, and on the doubt's weight, the extension being the
  owner's own; `tripwire@core@private-item-needed` stays kept on a doubt, since a consumer that
  works around a missing item fails nothing. It joins F2.
- **X4**: `tripwire@agent-skills@ruling-lost-in-change` gains the critical bound its two siblings
  carry, through `design@agent-skills@misstated-ruling-rated-by-what-it-changes`, a rewording.
- **The plausible verdicts**: the cross-check's readings are kept: as1/04 and as1/09 kept on a
  doubt; pk/15 removed on the doubt's weight, in F2.
- **The firings**: `tripwire@agent-skills@search-missed-before-the-work` fired, two instances in
  separate pieces of work, which the rule on slips counts, its default in F4;
  `tripwire@agent-skills@attribution-loses-the-scope` did not, since its reworded clause counts a
  critical misstatement only and the owner ruled each past instance not load-bearing;
  `tripwire@agent-skills@head-created-without-deliberation` did not fire anew, since its instances
  are the evidence `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`
  already carries; pk/01, pk/06 and pk/18 did not fire anew.

## 6. The owner list

Each item is a group; the default holds for every entry it lists unless the `Outcome` line names an
entry ruled otherwise. Write each `Outcome` line, or answer by label in the conversation.

### F1: converted, 4 entries

`tripwire@agent-skills@audit-passes-do-not-converge`, `tripwire@agent-skills@duplicate-auditors-add-nothing`,
`tripwire@core@grammars-not-prefixes`, `tripwire@core@walked-count-differs-between-machines`, as
section 3.4 gives them with their checks. Condition 1: "Where a script could compute the firing,
write that check instead". The two tests of `tripwire@core@grammars-not-prefixes` are written in the branch; the
three others become `todo` issues, and each entry leaves once its check exists, in the same
commit for the tests, with the issue for the others.

Default: converted, as listed.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F2: removed on condition 1, 12 entries

`tripwire@agent-skills@auditor-skips-part-of-the-corpus`, `tripwire@agent-skills@real-use-refusal-overruled`,
`tripwire@agent-skills@structure-rejection-overruled`, `tripwire@core@checker-source-literals-are-data-self-location`,
`tripwire@core@extension-defines-a-kind`, `tripwire@core@nested-entries-asked`,
`tripwire@core@new-component-blocked-by-the-gate`, `tripwire@core@plain-word-neither-pointer-nor-subject`
(on the doubt's weight), `tripwire@core@plans-directory-split-asked`,
`tripwire@core@staged-pass-fails-the-checkout`, and, by the session's decision on X3,
`tripwire@core@inputs-builder-needed`; condition 1: "A failure that fails a test or a check, stops
the work that meets it, or arrives as a request for what the decision refused reaches the decision
without a tripwire". The words each turns on, and the texts each leaves stale, are in sections 3.5
and 4.3. `rejected@agent-skills@workflow-corpus-shared-out` is judged again with the removal of
`tripwire@agent-skills@auditor-skips-part-of-the-corpus`, as F7 says.

Default: removed, all twelve.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F3: removed on condition 3, 4 entries

`tripwire@agent-skills@audit-defaults-overruled`, `tripwire@agent-skills@component-judgement-noisy`,
`tripwire@core@item-of-another-plan-named`, `tripwire@xtask@tip-alone-is-built`; condition 3: "A
clause that counts what the decision instructs or endorses fires on the decision working". Section
3.6. `rejected@agent-skills@wrong-component-outcome-removed` is judged again with the removal of
`tripwire@agent-skills@component-judgement-noisy`, as F7 says.

Default: removed, all four.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F4: fired, `tripwire@agent-skills@search-missed-before-the-work`

Its clause: "across sessions, 2 standing-state reviews report a standing entry the work bears on,
which no search before the work returned"; the instances are in the messages of two commits of
2026-10-19 on two specs, the commit "[review] … the spec of …" pair section 3.7 names. Its response
reopens `design@agent-skills@standing-entries-searched-before-the-work`; the owner's word quoted in
`issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` holds the
discussion of that question for the session that takes up that issue.

Default: fired: the entry is deleted, and a `design` issue opened with the two instances, its
re-entry the discussion the owner's word names, where the reopening is argued.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F5: X1, is the owner's own overrule "a request for what the decision refused"?

Entries on the owner overruling what a decision produced: `tripwire@agent-skills@real-use-refusal-overruled` and
`tripwire@agent-skills@structure-rejection-overruled` (in F2 on this reading), and `tripwire@agent-skills@a-comment-escalation-is-overruled`,
`tripwire@agent-skills@a-head-verdict-is-overruled`, `tripwire@agent-skills@argued-closure-reopened`, which the drafts reword and keep. The
owner's overrule is the owner's act, seen by the owner, and reaches the decision with no tripwire.

Default: yes: the three reworded entries join F2, removed; the two in F2 stay there.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F6: X2, does the rule on slips bind a verifier's error met in the session's sample?

`tripwire@agent-skills@conforming-draft-wrong`, `tripwire@agent-skills@verifier-refutes-a-true-finding` and `tripwire@agent-skills@verifiers-wave-findings-through`,
on `design@agent-skills@verifier-in-the-run`. The standard names no exception for a clause on a
sample's coverage, and a verifier is an agent following its brief.

Default: it binds: `tripwire@agent-skills@conforming-draft-wrong` reworded to "in two passes", kept with a short horizon;
the two others as drafted.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

### F7: the `rejected` entries kept on a doubt that a removed entry watched

`rejected@agent-skills@workflow-corpus-shared-out` and `rejected@agent-skills@wrong-component-outcome-removed`.

Default: each is judged again by the recording tests in the commit that deletes its tripwire; one
that no test holds is put to the owner then, its removal being the owner's.
Outcome: accepted, the default, on the owner's word: "I accept all defaults here, everything look fine, proceed".

## 7. What the run did not decide

### 6.1 Plausible verdicts (5)

| draft | entry | the open point |
| --- | --- | --- |
| as1/04 | `tripwire@agent-skills@agent-text-findings-need-nothing` | removal on condition 3 against a recurrence reading; cross-check proposes kept on a doubt |
| as1/09 | `tripwire@agent-skills@audit-drafts-mostly-noise` | the same; "Neither reading is settled by the text." |
| as2/01 | `tripwire@agent-skills@head-created-without-deliberation` | the new branch may be met on its first day (cebb978's finding c) |
| pk/09 | `tripwire@core@inputs-builder-needed` | X3; cross-check proposes kept on a doubt |
| pk/15 | `tripwire@core@plain-word-neither-pointer-nor-subject` | condition 1 not clear; removal on the doubt's weight |

### 6.2 Firings for the session to judge

- as2/16 `tripwire@agent-skills@search-missed-before-the-work`: proposed fired; "across sessions" not established.
- as2/01 `tripwire@agent-skills@head-created-without-deliberation`: judged for a firing once reworded.
- as1/02 `tripwire@agent-skills@a-head-verdict-is-overruled`: letter met once in thaum, ruled not fired by the owner; the
  rewording narrows the clause to that ruling.
- as1/07 `tripwire@agent-skills@attribution-loses-the-scope`: "The letter was met in several separate pieces of work and
  ruled not fired each time. Under the standard's recurrence branch that bears on those rulings;
  the session settles." The cross-check proposes not fired under the critical bound.
- pk/01 `tripwire@core@candidate-rule-silence`, pk/06 `tripwire@core@fix-makes-a-choice`, pk/18 `tripwire@core@private-item-needed`: past
  firings handled before the commit audited; cross-check proposes "not fired anew".

### 6.3 Notes of a head false of the code or failing a rule on heads

- as1/08: the entry's premise "defaults the owner mostly accepts" is one
  `design@agent-skills@audit-method` "does not state" (as1 return, Met outside the task); as1/14
  likewise for "mostly right" in `design@agent-skills@design-record-axis`. Notes on the entries'
  premises, not on the heads.
- pk/11: `design@core@plan-item-scope` instructs naming another plan's item as `#<id>`, which
  `issue@core@a-plain-hash-item-name-is-read-by-nothing` records "now advises the one form the
  checker never reads" (known).
- pk return, Met outside the task: "Several heads point at a specific tripwire through
  `register@core@tripwire` rather than the entry's reference (design.md:112, 242, 505, 632, 1175,
  1699, 2025), so `cargo klarch show` on the tripwire does not list them as citing texts; a deletion
  has to find them by grep. Noted for the design-record axis; not judged."
- as2/13's verifier: the head `design@knowledge-architect@a-build-is-tied-to-its-checkout` had a title wider than its
  argument, found by design-record pass 1 (841409f); written before the decision, "not a firing".
- pk/21: under a conversion, "the sentence \"nothing compiles or tests a commit before the tip\"
  becomes false for the release range; the head is amended". Moot under R2.
- No draft reports a head false of the code.

### 6.4 "Met outside the task" in the returns

- as1: "tools/xtask/docs/tripwires.md:9, in tripwire@xtask@tip-alone-is-built, cites \"the
  restructure commit d2f0ce4\", which does not resolve (git cat-file -t d2f0ce4: \"Not a valid
  object name\"); the import commit is 831519e. A stale pointer, primer case 2, moot if the entry is
  removed."; and "tripwire@agent-skills@audit-defaults-overruled watches a premise (defaults the
  owner mostly accepts) that design@agent-skills@audit-method does not state; folded into draft 08."
- as2: the d2f0ce4 pointer ("41c2fdc wrote d2f0ce4 while its message says it cites 831519e");
  "2712ecd recorded an instance of tripwire@agent-skills@search-missed-before-the-work in a commit
  message only; the entry's Instances line was never brought to it (draft 16)."; the pointer
  issue, "lines 97 to 99, says one report \"also fires\" tripwire@agent-skills@pointer-not-followed,
  false once draft 06's rewording applies"; "tripwire@agent-skills@record-answered-questions-reach-the-owner
  attributes to the owner a judgement on the routing of questions that the owner's quoted words in
  design@agent-skills@misstated-ruling-rated-by-what-it-changes gave on misstated rulings (draft 09
  narrows it)."
- pk: the d2f0ce4 pointer, given as line 8 ("a fix in a commit of its own (proposed in draft 21
  whatever the conversion's ruling)"); the `register@core@tripwire` pointers (6.3).
- The returns cite the d2f0ce4 line as tools/xtask/docs/tripwires.md:8 (pk), :9 (as1) and :11
  (pk verifier). Read for this report with `git show the commit audited:tools/xtask/docs/tripwires.md`: the
  entry's heading is line 8 and "d2f0ce4" is on line 11. Moot under R2.
- Verifiers: as1, the plan-document horizon command of drafts 06, 18, 19 and 22 "counts commits,
  not plan documents"; as2, the search-missed Instances line; pk, the d2f0ce4 pointer and the
  register pointers.
- Calibration: "none beyond the d2f0ce4 pointer every draft already reports."
- Cross-check: SKILL.md:143 left with no reader; the agent-behaviour issue line 90; tripwire-standard.md:199
  dangling at the deleting commit, not at the harvest. Each routed as bearing on this work.

### 6.5 Steps that failed or returned nothing

None. Every step wrote its file: three drafts directories with return.md, the calibration file,
three verdicts.md, the cross-check; the workflow's brief for this report named no failure and no
entry missing from a return.

### 6.6 Decisions the cross-check took that the session confirms

From the cross-check's "Decided": "agent-text-findings-need-nothing, audit-drafts-mostly-noise and
inputs-builder-needed are proposed kept on a doubt, not removed"; "plain-word-neither-pointer-nor-subject
is proposed removed on the doubt's weight"; X1 and X2 for the owner list; "X3 and X4 are left to
the session". Corrections carried with outcomes: the plan-document horizon command (as1/06, 18,
19, 22, and as1/23 not of record), as2/02's command, as2/06's "a directive broken", pk/06's
citation (path-quickfixes.md:227), pk/16's count (18 lines in 15 commits), as1/03's and as1/12's
repaired edits, as2/17's attribution.
