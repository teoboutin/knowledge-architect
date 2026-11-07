# 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow

- **Received file**: 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: knowledge-architect 0.9.0, the version the project pins.
- **Analysed against**: main at `v0.9.0-184-g00533a5c`.
- **Standing entries that bear on it**: `issue@agent-skills@a-lighter-pass-for-untouched-components`, `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`, `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`, `issue@check@a-plain-hash-item-name-is-read-by-nothing`, `tripwire@agent-skills@owner-fact-recorded-unattributed`, `tripwire@agent-skills@ruling-lost-in-change`.

## W1: the stop of the whole-set review at a fifth

Analysed with W1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## W3: the record's boundary blocks go stale

- **a.** The session of a whole-set review wrote each goal's boundary block once, before round 1.
  Later rounds' reviewers reported blocks quoting wording two or three rounds old, and the ruling
  sheet had to be rebuilt from the last round's reports. Blamed:
  `skill@knowledge-architect-goal-setting@whole-set-review` step 4, "Keep the review's record [...]
  and each goal's boundary block", and the closing paragraph, "Every goal is then put to the owner
  with its boundary block [...] the blocks in the record". Proposed: the record holds the blocks of
  the last round only, replaced each round from the reviewers' reports; one sentence reworded.
- **b.** Still applies. At `v0.9.0`, line 165: "for each round, the findings counted per reviewer,
  rule and tag, each finding's outcome, each default with its ground, each question put to the owner
  with the owner's answer verbatim, and each goal's boundary block". At HEAD, line 211 to line 213,
  the same list, with "rule, grade, route tag and old or new wording". Line 224 to line 227: "Every
  goal is then put to the owner with its boundary block [...] the blocks in the record and the goals
  in the tree". The list opens with "for each round", so the record does hold a block per round; no
  sentence says which round's block is put to the owner. The reviewers already give the material
  each round: `agent@knowledge-architect-goal-reviewer`, draft and whole-set modes, "give, for every
  goal, the material of its boundary block".
- **c.** Not recorded. `design@agent-skills@whole-set-rounds` says the record holds "the boundary
  blocks" and "the session added [...] the boundary blocks in the record"; it says nothing of which
  round's. `design@agent-skills@goal-carries-reading-examples`: the block leaves with the ruling, and
  the cases the owner keeps are written in the goal, so a stale block puts stale cases into the goal.
- **d.** Defect of the text, unclear: "for each round" scopes a list of eight items, and the
  sentence that puts the blocks to the owner names no round. One instance.
- **e.** Installed text, `agent-skills`; serves `goal@knowledge-architect@the-owner-decides` (the
  owner rules on the reach of the goals as worded). No head reversed: the proposal says which block,
  which no head states.
- **f.** Nearest rival: no change, the session's misreading counted a lapse. What decides: the owner
  is shown the block at the ruling, and the round that meets the stop writes no repair, so only that
  round's blocks describe the goals the owner reads; the text does not say so, and the session's
  reading, one block written once, is the one the closing sentence admits. The finding's form
  ("replaced each round") and the rewording below are equivalent; the rewording keeps the per-round
  history the record already holds.
- **g.** A text edit of the installed text whose design is settled: `path@agent-skills@CLAUDE.md`,
  "Editing an installed skill or agent".
- **h.** In step 4, "each goal's boundary block" becomes "each goal's boundary block, from the
  round's reviewers"; in the closing paragraph, "the blocks in the record" becomes "the blocks of the
  round that met the stop, in the record". Instructions: 0 added, 0 removed, two rewordings. A
  `Next release` Workflow entry of the changelog, then `cargo x changelog`.
- **Default**: handle now.
- **Outcome**: decided by the session: handle now. Ground: the closing paragraph names no round, and
  the stop round alone reads the goals as shown, per
  `skill@knowledge-architect-goal-setting@whole-set-review` step 6; the rewording changes no
  instruction and contradicts no head, per `design@agent-skills@whole-set-rounds`.

## W2: what a review owes after the last transcript review, and a goal repair at the review (cluster K)

Members: W2 of 2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow (home); W5 of
2026-11-02-knowledge-architect-component-split-git-out-klarch-workflow; W2 of
2026-11-04-knowledge-architect-check-renamed-klarch-workflow; W2 of
2026-11-06-knowledge-architect-cli-on-top-klarch-workflow; the read-back lapse "transcript reviewer
dispatched while an item was put to the owner" of 2026-11-01-knowledge-architect-release-0-9-0-klarch-workflow
and of 2026-11-01-knowledge-architect-release-0-9-0.

- **a.** What it says.
  - narrowed-goals W2: the goals axis of the merge review found eight wordings a goal round's repairs
    had reached in part; drafting them under `skill@knowledge-architect-goal-setting@drawing-out-intent`
    needed a record file, two draft rounds and a ruling, and the owner's answers were written after
    the last review, unreviewed. Proposal: a repair that brings a goal's text to what the owner
    already ruled is checked by the goals axis of the same review and shown to the owner as a diff,
    with no draft batch. +1 instruction.
  - git-out W5: the transcript review's repair, on the owner's answer, listed entries left unread,
    which made the design-conformance axis owed after the transcript review; the session ran it and
    no second transcript review. Proposal: one clause naming an axis a repair makes owed.
  - check-renamed W2: the last transcript review found three goal items unruled; their rulings
    reworded two goals; the session could not tell whether that owes draft rounds, the goals axis or
    nothing, and asked the owner, who asked for one more goals review. Proposal: a goal reworded by a
    repair after the goals axis is read once more by that axis, unless the owner stops it. +1.
  - cli-on-top W2: an item of the last transcript review led to two new goals, a new head and 16
    further rulings; no transcript reviewer read them; the goals axis in merge mode found the commit
    message overstated or left out several. Proposal: the goals axis in merge mode also reads the
    commit message recording rulings made after the last transcript review. Rival named: a second
    transcript review.
  - release-0-9-0: the transcript reviewer was dispatched while an item was recorded "put to the
    owner" but not yet asked; it reported the item without an outcome. A lapse, no finding.
- **b.** Whether it still applies. Unchanged at every version and at HEAD, in
  `path@agent-skills@content/skills/review/SKILL.md`:
  - lines 54 and 55: "A review owes every axis of this table whose condition holds";
  - lines 64 to 66: the transcript reviewer runs "after every other axis has run, its repairs are
    committed and every item of their reports has its outcome";
  - lines 150 and 151: "A repair that rewords a goal is drafted as its own item under
    `skill@knowledge-architect-goal-setting@drawing-out-intent`";
  - lines 165 to 171: "A repair that adds, removes or reverses a design head is reviewed by the
    decision-record axis, and by any other axis whose condition it meets … a repair of the last
    transcript review is reviewed after it, and the review ends with the repairs of that
    decision-record review. Any other repair … is reviewed by no axis again".

  The goals axis in merge mode, `path@agent-skills@content/agents/goal-reviewer.md`, lines 54 to 57,
  reports "each goal the range adds, rewords or removes whose ruling by the owner no commit message
  of the range records": it checks that a ruling is recorded, not that it is recorded faithfully.
- **c.** Already recorded:
  - `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: its title says "no other
    repair is" reviewed again; its argument rejects a trigger on any repair that "makes a decision",
    a judgement, and every axis again at every repair, on cost; its trigger is "a heading with a slug
    that appears or disappears".
  - `design@agent-skills@transcript-review-last-before-merge`: "any other is reviewed by no axis
    again, so the review ends".
  - `design@agent-skills@goal-edit-ruled-alone`: a goal's wording that "a review's repair" proposes
    "is drafted under `skill@knowledge-architect-goal-setting@drawing-out-intent` as its own item,
    with its boundary block".
  - `design@agent-skills@goals-axis-in-the-review-before-a-merge` and
    `design@agent-skills@transcript-reviewer-agent` state each axis's question; the first does not
    make the goals axis a fidelity check of rulings.
  - No issue records the case. The loop of draft rounds itself is cluster C of this intake, under
    goal-setting.
- **d.** Kind: unclear, on one moment, the repairs of the last transcript review. The text gives an
  answer for a goal repair (drafted under goal-setting) and for every repair that changes no head (no
  axis again), and the table's "owes every axis whose condition holds" reads as owing an axis whose
  condition a later repair makes hold: two readings, met in four sessions (git-out, check-renamed,
  cli-on-top, and narrowed-goals for the goal case). The release-0-9-0 item is a lapse of a clear
  instruction, read at the moment of dispatch, once.
- **e.** Scope: `goal@knowledge-architect@design-is-recorded-with-its-arguments` and
  `goal@knowledge-architect@the-owner-decides`, which the rulings recorded after the last transcript
  review serve, against the review's end that
  `goal@knowledge-architect@agents-get-a-complete-workflow` asks to be bounded. The owner of the fix
  is the installed text. Every proposal of the members changes a recorded decision: a re-review of a
  repair that changes no head contradicts the title of
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`; narrowed-goals' lighter route
  for a goal repair contradicts `design@agent-skills@goal-edit-ruled-alone`; cli-on-top's widening
  gives the goals axis a fidelity check that `design@agent-skills@transcript-reviewer-agent` places in
  the transcript reviewer.
- **f.** A better fix. One rule for the cluster: a repair of the last transcript review is reviewed
  by each axis whose condition it newly meets (the decision-record axis for a head, the goals axis
  for a goal, design conformance for a list of unread entries), and the review ends with those
  reviews' repairs. Its trigger is read from the diff and the message, as the head's is, so the
  head's argument against a judged trigger does not cover it, and it is no "every axis again". Its
  rivals survive: the owner ruling each time, as check-renamed did, at one question per branch; a
  second transcript review, at a whole-transcript read per repair, as cli-on-top weighs it; and
  cli-on-top's widening of the goals axis. Choosing among them weighs the review's cost against what
  a ruling recorded after the last transcript review risks, which
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes` rates, so it is a design question,
  not a rewording.
- **g.** Route: `skill@knowledge-architect-design`, since the fix reverses or widens a head; an
  issue, since the work needs its own discussion.
- **h.** Proposal: open a `design` issue at `agent-skills`, proposed id
  `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`, naming
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again` as the decision it strains, the
  four instances, the shapes above and the head each would change, and as its re-entry the next
  review whose last transcript review's repair makes an axis owed, or the owner's word. No edit of
  the text now. The release-0-9-0 item: no change, a lapse.
- **Default**: open the design issue; no change for the lapse.
- **Outcome**: decided by the session: open an issue (`agent-skills`, `design`,
  `issue@agent-skills@a-repair-of-the-last-transcript-review-can-make-another-axis-owed`); no change for the
  release-0-9-0 lapse. Ground: each proposed fix contradicts
  `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`,
  `design@agent-skills@goal-edit-ruled-alone` or `design@agent-skills@transcript-reviewer-agent`,
  and at least three shapes survive with a cost no goal ranks, so the change goes to the design
  skill; the lapse is of `skill@knowledge-architect-review@review-axes`, clear and read at the
  moment it applies.

## W4: where a label's number starts when a second record opens mid-branch

- **a.** What it says: `primer@labelled-items`, "takes the next number of its prefix in the record
  the label reaches, from 1 ... the commit messages of one branch, counting as one". A second record
  file opened on the branch; its questions reached that file and the branch's messages. The session
  went on from Q68, the messages having carried Q1 to Q67; the file says the rule reads equally as a
  restart at Q1. Proposal: say which record numbers a label that reaches two records at once. One
  sentence reworded.
- **b.** Whether it still applies: `path@agent-skills@content/PRIMER.md`, lines 68 to 74, unchanged
  since `v0.9.0` (`git diff v0.9.0 HEAD` on the primer touches other sections only).
- **c.** Whether it is already recorded: `design@agent-skills@ruled-items-labelled`: numbers "are
  never reused within the record the label reaches". No issue.
- **d.** Its kind: no defect. Read whole, the rule admits one act: the number is "never reused
  there", in each record the label reaches. A restart at Q1 in the new file would reuse Q1 in the
  branch's commit messages, which already held it; only a number unused in both, the session's Q68,
  meets the rule in both records.
- **e.** Its scope: installed text, the primer. A change would restate the head's rule.
- **f.** A better fix: the rival is the file's rewording, "the next number unused in every record the
  label reaches", 0 instructions. It loses on the need test: it changes no act, since the words as
  they stand already give the session's choice, and the primer is loaded by every session.
- **g.** Its route: nothing.
- **h.** The proposal: no change.
- **Default**: no change, with the reason.
- **Outcome**: decided by the session: no change. Ground: "never reused there" applied to each
  record the label reaches admits only the number the session took, per `primer@labelled-items` and
  `design@agent-skills@ruled-items-labelled`.

## C1: no command renames an entry

Analysed with C1 of 2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.md`.

## C2: a verbatim quotation holding a backticked slug

Analysed with P1 of 2026-11-01-knowledge-architect-narrowed-goals, in `path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals.md`.

## C3: no command rewords the message of a commit that is not the newest

Analysed with P2 of 2026-11-01-knowledge-architect-narrowed-goals, in
`path@knowledge-architect@docs/retrospective-reports/2026-11-01-knowledge-architect-narrowed-goals.md`.

## `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: its re-entry point is met again

- **a.** The standing answers of four files report a pointer written bare, beside no issue entry:
  2026-11-01-knowledge-architect-narrowed-goals-klarch-workflow (two slugs in the owner's verbatim
  words, with a note), 2026-11-04-knowledge-architect-model-crate-klarch-workflow (Rust module
  names), 2026-11-06-knowledge-architect-consumers-out-klarch-workflow (a drafted goal as a bare
  hash slug), 2026-11-01-thaum-tripwire-audit-klarch-workflow (about 60 horizons bare in an
  application commit, written beside an issue entry after the review). G8 holds the list and the
  count.
- **b.** The issue's re-entry point: "a further instance, a pointer written bare that no check
  reads, or earlier on the owner's word". Each instance above meets it. The entry records a fourth
  instance that already met it, and no discussion followed.
- **c.** Recorded: the entry holds three instances and a fourth.
- **d.** Observation for the entry: a re-entry point met at each intake, with no discussion raised,
  orders nothing.
- **e.** Threatens `goal@knowledge-architect@documentation-stays-consistent`. Owner of the fix: the
  checker or the installed text, per the discussion. A change reverses or keeps
  `design@agent-skills@plain-text-is-no-repair`.
- **f.** Rival: hold the design discussion inside this intake. It loses: an intake's work holds no
  design discussion, as `issue@agent-skills@a-lighter-pass-for-untouched-components` states of its own
  case, and the discussion needs a census of false positives over two trees.
- **h.** The issue gains the instances G8 lists. The re-entry met is reported to the owner, with the
  default that the discussion waits for the owner's word or a roadmap row.
- **Default**: the issue gains the instances; the discussion waits for the owner's word.
- **Outcome**: decided by the session: the issue gains the instances. The scheduling of the
  discussion is the owner's, since the entry's re-entry point is met and no rule orders when a
  discussion runs.

---

## The standing answers

- **a.** Items:
  - Installed file: one `%%` line of a source, which the build removes. An observation.
  - The universal claim in the goals' commit: analysed with the claim-check cluster of
    2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow.
  - The three attribution slips: analysed in the attribution cluster below.
  - The two goal slugs, each with two hash signs before it, written bare: analysed in the pointers cluster below; the missing form
    is C2 of this file, analysed by G4.
  - Hand tasks: the `sed` re-pointing is cluster E (G4), home C1 of
    2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow; the reword is
    cluster H (G6), home P2 of 2026-11-01-knowledge-architect-narrowed-goals with C3 of this file;
    the sum of not-minor findings for the stop rule is cluster C (G2), home W1 of
    2026-11-01-knowledge-architect-development-milestone-plan-klarch-workflow.
- **Default**: no change.
- **Outcome**: decided by the session: no change for the observation analysed here. Ground: no
  text is blamed.

## The attribution slips of thirteen files

- **a.** What it says. The session recorded two goal batches as approved where the owner had
  answered their questions without the word the chosen option named, and attributed a goal's
  sentence to the owner as a ruling shown only in part; the transcript reviewer caught all three.
  The file calls them the session's own slips, no finding of the skill. Twelve other files report
  slips against the attribution rule of `primer@design-heads`, `primer@owner-word-premise` or
  `primer@question-authority`, each caught before the merge:

  | file | slips | what |
  | --- | --- | --- |
  | pre-release-fixes | 1 | a commit message said an approval of a default answered how a root goal is read |
  | component-split-git-out | 2 | the owner quoted in two commit messages |
  | component-split-planning | 2 | an act of the session attributed to the owner; a misquoted phrase |
  | development-and-components | 3 | a fragment's second host attributed to the owner; two quotations of the owner altered by a `sed` rename |
  | component-rules-revision, 11-03 | 2 at least ("attributions", plural) | the owner's arguments |
  | check-renamed | 2 | a goal clause recorded as approved outside the table the owner approved; a goal's cases never shown |
  | model crate | 2 | two unanswered questions recorded as the owner's word: its W3, cluster N6 (G2), home W3 of that file |
  | cli-on-top | 3 | a deferral attributed to an answer on a goal's measure; a tripwire's firing attributed to the owner; a sentence arguing for a goal read as a ruling on its wording, which is cluster N7 (G2), home W1 of that file |
  | thaum design-record-audit | 1 | the owner approved goals on a message that held no goal text |
  | thaum count-ceilings-slice | 1 | a default recorded as "approved" by an answer that did not name it |
  | thaum fuzz-campaign-slice | 1 | the owner's approval of a default recorded as the owner's choice at a fork |
  | thaum log-target-slice | 1, at three sites | a hedged word widened in a head and two commit messages |

  Thirteen of 23 files, 24 slips at least. Four of them are also in the claim-check count. The
  transcript reviewer caught most; the others the decision-record, goals and design-conformance
  reviewers. No file reports a slip that reached a main branch.
- **b.** The rules hold at HEAD: `path@agent-skills@fragments/head-ground.md`, line 22, "an
  approval of the session's default ... no ground: the session's decision", and line 37, "A durable
  record attributes a decision to the owner; it does not quote the owner." The retrospective's rule
  on what is no finding, `path@agent-skills@content/skills/retrospective/SKILL.md`, line 113: "A
  ruling of the owner recorded a little relaxed, or firmer than its hedges, on a detail that is not
  load-bearing, or an argument or a reading attributed to the owner that is not the owner's, caught
  by the transcript reviewer before the merge, is no finding".
- **c.** Recorded:
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`,
  read whole. It holds six files of three projects from the intake of 2026-10-24, a further
  instance from 2026-10-25, the owner's hedged hypothesis that the planning skill's verbatim quotes
  carry into the records that must attribute, and a proposed sentence at the harvest step. Its
  closing condition counts "the attribution slips of a release's retrospectives". Its re-entry
  point, "on the owner's word or when work next changes how a plan document keeps the owner's
  words", is not met: the one later commit whose diff in the planning skill, the decision-recording
  skill, the primer or the fragments touches "verbatim", 3a83190d, rewraps a sentence. The rating
  rule is `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: an attributed decision
  the owner did not make is major, a slip on a detail is minor, and "a minor ruling is no finding
  of the retrospective once caught before the merge". `design@agent-skills@attribution-replaces-quotation`
  holds the regime. The tripwires `tripwire@agent-skills@ruling-lost-in-change` and
  `tripwire@agent-skills@owner-fact-recorded-unattributed` are G9's to judge.
- **d.** Lapses of clear instructions. The major ones by the rating head, a decision attributed
  that the owner did not make, are in narrowed-goals, check-renamed, model crate, and the thaum
  design-record, count-ceilings and fuzz files; each was caught before the merge. Only the cli-on-top
  slip changed what was built, and N7 takes it.
- **e.** `goal@knowledge-architect@the-owner-decides`, which the issue cites. The owner of a fix
  would be the installed text. Nothing here reverses a decision.
- **f.** Rival: a new instruction against these slips. It loses: a lapse is not repaired by adding
  text, and the owner's weighing in `design@agent-skills@misstated-ruling-rated-by-what-it-changes`
  is that the transcript reviewer catches them and no instruction would do better. The issue's own
  discussion is where the two regimes are weighed.
- **g.** An issue already open.
- **h.** The issue gains this intake's count, by kind: approvals or answers recorded as rulings,
  acts or arguments attributed, hedges widened, quotations in records that attribute, and premises
  not shown; with the note that only the quotation kind bears on the issue's hypothesis.
- **Default**: the issue gains the instances.
- **Outcome**: decided by the session: the issue gains the instances, classed by kind. Ground: its
  closing condition counts these slips; no instruction is unclear, and the rating head rules out an
  added instruction.

## The pointers written bare of three files

- **a.** What it says. The owner's message quoted two goal slugs, each backticked with two hash signs before it;
  copied into a record, the checker reported each as a slug mid-line that defines nothing, and the
  session removed the backticks and said so beside them, with no issue entry. Two other files report
  a pointer written bare that no check reads:
  - thaum tripwire-audit: about 60 horizons naming a file whose deletion an occasion counts from
    were written with bare paths in the application commit, beside no issue entry; the routing
    reviewer caught it, and after the review 17 plain paths stand beside an issue entry the project
    opened, which the exception of `design@agent-skills@plain-text-is-no-repair` admits. Its missing
    form is W5 and C1 of that file, analysed by G4.
  - consumers-out: a goal drafted and not committed was named as a hash sign and its slug, in plain
    text, beside no issue entry, after both checked forms failed one of the two trees. Its missing
    form is its C2, cluster N3 (G2), home W2 of
    2026-11-04-knowledge-architect-goal-standard-revision-klarch-workflow.

  The other twenty files answer no; check-renamed and cli-on-top say they ran no search for one.
- **b.** None of the three forms is read by a check: a slug span with its backticks removed is not
  reference-shaped, so `design@check@reference-shape-outside-span` does not report it; a bare path
  is not read, as the issue says; a plain hash name is read by nothing, per
  `issue@check@a-plain-hash-item-name-is-read-by-nothing`. The narrowed-goals case sits in a
  quotation, which `design@check@quotation-read-by-no-id-lint` exempts from the lints on ids, but
  the finding it met was the misplaced-slug finding, which that head does not name, and the session
  changed the owner's words to clear it, so it is not a quotation kept verbatim.
- **c.** Recorded: `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`,
  read whole. It holds four instances and its mechanism, "a writer who meets a finding with no form
  it recognises removes the backticks, and nothing reads text outside backticks unless it is
  reference-shaped", which the narrowed-goals case matches word for word. Its re-entry point, "a
  further instance, a pointer written bare that no check reads, or earlier on the owner's word", was
  met by its fourth instance, as it records, and is met again by each of these three. Its closing
  candidate is a check of path-shaped and name-shaped tokens outside backticks, with a census of
  false positives. G9's list names this issue too; this section gives its instances, and the
  re-entry verdict should be given in one place only.
- **d.** Two lapses of a clear rule (tripwire-audit, before its review; narrowed-goals) and one
  missing form (consumers-out, the case N3 analyses).
- **e.** `goal@knowledge-architect@documentation-stays-consistent`, which the issue cites. The
  owner of a fix: the checker, or the installed text.
- **f.** Rival: a form per case, which G4 and N3 weigh for their own cases. It does not close the
  issue's subject: a writer who meets the next finding with no form it recognises does the same.
- **g.** An issue already open; its re-entry point is met.
- **h.** The issue gains the three instances, with which checks miss each, and the owner is told
  its re-entry point has been met since its fourth instance and is met again three times, so the
  discussion is owed; scheduling it is the owner's order.
- **Default**: the issue gains the three instances; the owner is told the re-entry point is met.
- **Outcome**: decided by the session: the issue gains the three instances, and the owner is told
  that its re-entry point is met again. Ground: the instances are its re-entry evidence and its
  closing census needs them; the order of known work is the owner's, per
  `skill@knowledge-architect-planning@spec-or-milestone`.

---
