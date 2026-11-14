# 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow

- **Received file**: 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source, main at `v0.9.0-304-gd687aa43` when the work began; branch `primer-import-on-first-line`; the tree was clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@intake-shares-the-admission-standard`: constrains, respect.
  - `rejected@agent-skills@owner-words-verbatim-in-plan-items`: constrains, respect.
  - `design@agent-skills@transcript-reviewer-agent`: constrains, respect.
  - `design@agent-skills@misstated-ruling-rated-by-what-it-changes`: constrains, respect.
  - `design@agent-skills@heads-stand-on-argument`: constrains, respect.
  - `design@agent-skills@attribution-replaces-quotation`: constrains, respect.
  - `rejected@agent-skills@head-cites-every-owner-ruling`: constrains, respect.
  - `rejected@agent-skills@owner-words-quoted-in-head`: brings-back, new-argument-needed.
  - `issue@agent-skills@attribution-slips-are-not-counted-against-the-regime-of-committed-transcripts`: touches, none.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@the-owner-decides`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.
  - `goal@adoption-is-easy`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `goal@records-reach-their-reader`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `design@agent-skills@version-line-names-the-main-commit`: constrains, respect.
  - `design@agent-skills@at-least-one-independent-verification`: constrains, respect.
  - `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`: touches, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.

## report-check-first-line-misread: "never from its first line" read as the heading

Members: 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow C2(b).

- **a. What it says.** Five searchers read the report check's message "never from its first line" as forbidding a quotation from the entry's heading, which the check accepts. It names the issue `the-report-check-refuses-a-quotation-cut-before-punctuation-without-saying-so` of the `query` anchor.
- **b. Whether it still applies.** No. The file used `v0.9.0-304-gd687aa43`. The commit 8af05fc3 (`v0.9.0-320`), "[query] The report check says what keeps a quotation from being found, and which line it may not come from", closed that issue; its message says "A searcher also read "its first line" as the entry's heading, which the check accepts". At origin/main the refusal in `path@query@src/worklist.rs`, near line 1729, reads "copy it verbatim from the entry's text, its heading included, never from the line naming the entry and its site above it, nor from the sites that cite it", and `agent@knowledge-architect-record-searcher@fill-the-report` says "from the entry's text inside its fence, or the whole of a shorter one; never from its `===` line".

Default: no change.
Outcome: decided by the session: no change, already repaired. Ground: 8af05fc3 reworded the message to name the heading as quotable and the line it may not come from.

## searcher-cannot-check-own-report: a searcher could not judge its own report

Members: 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow C1.

- **a. What it says.** A searcher could not check its own report, since `--verify` removes the outputs of the directory every agent shares and the searcher may write only its report. The first run failed 14 of 18 reports with 193 findings; at least eight searchers wrote a quotation checker of their own; the dispatcher split the output per agent by hand. It names the issue `a-searcher-cannot-check-its-own-report` of the `query` anchor, and proposes a `--verify` form that judges one agent's report and writes nothing, with a line in the searcher's definition to run it (+1).
- **b. Whether it still applies.** No. The file used `v0.9.0-304-gd687aa43`. The commit 33f5c59d (`v0.9.0-355`), "[agent-skills] A searcher checks its own report with `worklist --check`, every agent holds `Write`, and a reviewer writes its own report", built `klarch worklist --check <dir>/agent-<n>`, which judges one report by the function `--verify` uses and writes nothing, and closed that issue. At origin/main, `design@query@worklist-check-judges-one-report` states it, and `path@agent-skills@content/agents/record-searcher.md`, near line 117, says "**Before you return, run `{{command}} worklist --check <your agent directory>`**". That is the proposed fix, the definition's line included.

Default: no change.
Outcome: decided by the session: no change, already repaired. Ground: 33f5c59d built the read-only check of one report and the searcher's instruction to run it, as the finding proposed.

## searcher-fills-report-by-script: searchers still fill their reports by scripts of their own

Members: 2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow C2; 2026-11-12-knowledge-architect-transcript-reader-gaps-klarch-workflow C2; and the standing answer on scripts of 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow.

- **a. What it says.** The searcher-check-reviewer-reports file (`v0.9.0-354`): "In the first search, 12 of 18 reports failed the check, nearly all on a quotation cut before its punctuation, and most searchers wrote a quotation checker of their own that missed it. With the new `--check` they loop on the command, and still fill the lines by scripts." The transcript-reader-gaps file (`v0.9.0-360`): "Several of the 36 searcher agents wrote `fill.py`, `fix.py` or `fixq.py` to fill their report and to extend each quotation to whole words". The primer-import file (`v0.9.0-304`), its standing answer, line 108: "Every searcher also filled its report by a script or a whole rewrite". Each says the project already records it, and proposes nothing new.
- **c. Whether it is already recorded.** Yes: `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` (kind `todo`, verdict `touches`, action `respect`). Its closing condition is unchanged by these files: "A form in which a searcher records its grades without writing a script, read by `worklist --verify` as the reports are today, with its shape argued under `skill@knowledge-architect-design`", or the owner's ruling that the filling stays by hand. The new facts these files add are that `--check`, built at 33f5c59d (`v0.9.0-355`), did not stop the scripts, and that part of each script is the quotation repair of the cluster quotation-check-refuses-cut-punctuation, a cause the issue does not name.
- **d. Its kind.** Missing work of the checker, recorded as a `todo`, further instances.

Default: instances added to the recorded issue.
Outcome: decided by the session: instances added to `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`. Ground: the files bring three more sessions and one new fact (the scripts survive `--check` and serve the quotation repair too), and propose no shape the issue does not already send to the design skill.

Text to add to the issue, in "Details", "What", after the last bullet of the instances from the intake of 2026-11-07 and before the paragraph "The harness's edit tool replaces one exact string per call":

```markdown
Further instances, from retrospective files received at the intake of 2026-11-14:

- three sessions of this repository, 2026-11-11 and 2026-11-12, on trees from `v0.9.0-304` to
  `v0.9.0-360`: in one, every searcher filled its report by a script or a whole rewrite; in
  another, most searchers of its first search wrote a quotation checker of their own, and those of
  its second search, once `worklist --check` was built, looped on it and still filled their lines
  by scripts; in the third, several of 36 searchers wrote `fill.py`, `fix.py` or `fixq.py` to fill
  their report and to extend each quotation to whole words, the repair of
  `issue@query@a-quotation-cut-from-its-punctuation-is-refused-as-out-of-order`;
- a fourth session, 2026-11-11: one searcher's script overwrote the preamble of its report, which
  the check does not read, so nothing was refused.
```

## reversal-lists-omit-readme: the reversal check's repair list omits user documentation

Members: 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow W1. Related:
`plan-row-points-to-show-only`, whose repair points to the list this one repairs.

- **a. What it says.** `skill@knowledge-architect-decision-recording@reversal-check` has two
  enumerations: the incumbent search greps "every document that carries decisions (the design
  homes, the `rejected` homes, the scoped `CLAUDE.md` files) and the comments of the code the
  subject touches"; the repair bullet reads "**Repair what pointed at the old behaviour**: skills,
  subagent definitions, scoped `CLAUDE.md` files, generated headers. Grep the behaviour's wording as
  well as the slug". The session reversed a head on where the primer's import line goes and did
  not search the READMEs. A user README kept two sentences stating the old rule; four reviewers
  found them. A build-script comment the grep returned, holding the old wording in other words,
  was missed too. Proposal: replace both enumerations by "every text that describes the behaviour,
  the README of each Component that exposes it included", 0 added, 0 removed.
- **b. Whether it still applies.** Yes. At the file's version, d687aa43 (`v0.9.0-304`), lines 10
  and 45 of `path@agent-skills@fragments/reversal-check.md` hold the two lists; on main both are
  unchanged. The fragment's hosts, by `FRAGMENT_HOSTS` in `path@agent-skills@src/lib.rs`: the
  decision-recording skill and the decision-record reviewer.
- **c. Whether it is already recorded.** No issue or head on these lists; every verdict is
  `constrains`, `respect`. `goal@documentation-stays-consistent` supports the fix in every target
  project: a reversal that leaves a user's README false leaves the documentation inconsistent with
  the code. `goal@adoption-is-easy` ("finds what to do in its documentation, where they already
  look for it") bears on this instance, a README of knowledge-architect itself. Three more sessions
  of the batch carried the reversal check in part, as the standing answers of
  2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow and of two other files
  report: one grepped half of a changed title, one stated no search, one judged an installed
  sentence still true after its reversal. Each lapsed on another step of the check, and none on its
  lists, so they are no evidence for this fix and no recurrence at one point of delivery. The knowledge table of `primer@where-knowledge-goes`
  gives a `README.md` "how a user can use a Component, and what to respect", so it describes
  behaviour a reversal changes.
- **d. Its kind.** A defect of the text, missing: the repair list names a subset of the texts that
  describe a behaviour. The missed build-script comment is a lapse: the grep returned it, and the
  bullet asks to grep the wording; it needs no text.
- **e. Its scope.** The installed text, in a fragment of agent-skills. No head is reversed.
- **f. A better fix.** The report's fix applies the class to both lists. The first list finds the
  incumbent decision, and a README carries no decision, so widening it adds reading with no act;
  the fix belongs in the repair bullet alone, which is where the act is. Nearest rival: adding
  "README" to the repair list, which the report rejects since the next unlisted kind is missed the
  same way; the class with its examples keeps the named kinds and closes the list. The rival "no
  change" loses to the four reviewers' findings and the two goals above.
- **g. Its route.** A fix in the installed text, in the fragment at its source, as
  `path@agent-skills@CLAUDE.md` says; the review names both hosts' installed copies.
- **h. The proposal.** Handle now. In `path@agent-skills@fragments/reversal-check.md`, replace the
  first sentence of the repair bullet, "**Repair what pointed at the old behaviour**: skills,
  subagent definitions, scoped `CLAUDE.md` files, generated headers.", by the text below; the next
  sentence, "Grep the behaviour's wording as well as the slug, [...]", stays. Instructions added:
  0; removed: 0 (the act exists; its list becomes the class it stands for).

```text
- **Repair every text that points at or describes the old behaviour**: the skills, the subagent
  definitions, the scoped `CLAUDE.md` files, the `README.md` of each Component that exposes it, and
  the generated headers.
```

Default: handle now, the rewording above.
Outcome: decided by the session: handle now, the rewording above. Ground: the knowledge table
makes a Component's `README.md` a text that describes its behaviour; `goal@documentation-stays-consistent`; the incumbent search's list is kept, since it finds
decisions and a README holds none.

## owner-attribution-slips: the owner quoted, or given a ruling or a reason the owner did not give

Members: 2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow (its items that
are no finding), and the standing answers of
2026-11-12-knowledge-architect-dispatch-judgement-klarch-workflow,
2026-11-12-thaum-triggers-design-klarch-workflow and
2026-11-13-knowledge-architect-split-guidelines-plan-directories-klarch-workflow.

- **a. What it says.** Four files report a slip against the attribution rules of
  `primer@design-heads`, each caught before the merge, none proposing a fix:
  - primer-import-first-line: the first commit's message attributed to the owner a premise the
    session had supplied, and recorded a lean as a preference; the transcript reviewer found it;
  - dispatch-judgement: a brief said a sentence was "ruled by the owner" where the owner had given
    an argument and approved the session's wording; the agent carried it into two commit messages
    and a head; the review before the merge found it;
  - thaum triggers-design: the first commit's message quoted the owner verbatim; the
    decision-record reviewer found it, and a history edit repaired it;
  - split-guidelines-plan-directories: the first goal commit gave the owner a reason the owner did
    not give, and dropped a hedge; the goals reviewer found it, and the message was reworded.
- **b. Whether it still applies.** The rules hold at origin/main:
  `path@agent-skills@fragments/head-ground.md`, line 41, "**A durable record attributes a decision
  to the owner; it does not quote the owner.**", and the same paragraph, "An attribution keeps the
  scope and the hedges of the owner's words". The table of what the owner brings, in the same
  fragment, line 23, gives "an approval of the session's default, threshold or weighing" "no
  ground: the session's decision".
- **c. Whether it is already recorded.** Yes, as a series with a deferred count:
  `issue@agent-skills@attribution-slips-are-not-counted-against-the-regime-of-committed-transcripts`,
  read whole, verdict `touches`. Its trigger is "the intake of the first batch of retrospective
  files whose sessions ran a release tagged after the merge of the slice that made plan documents
  directories". The newest tag is v0.10.0 (`git tag --sort=-creatordate`), and that slice's session
  began at v0.10.0-5 (its file's version line), so no release has been tagged after its merge, and
  the trigger is not met by this batch. `rejected@agent-skills@owner-words-quoted-in-head`
  (verdict `brings-back`, action `new-argument-needed`) says "this is the candidate if a misstated
  attribution recurs": no outcome here proposes quoting the owner in a head. One slip of this batch reached a head,
  dispatch-judgement's, and every slip was caught before the merge. Whether the recurrence the
  entry names comes back under the new regime is what the deferred count measures; this batch's
  sessions ran before any release that count names, so it does not reopen the candidate.
  `design@agent-skills@misstated-ruling-rated-by-what-it-changes` rates them: the dispatch-judgement
  slip, an approval recorded as a ruling, is "a decision attributed to the owner that the owner
  did not make", major; the other three are minor, and "a minor ruling is no finding of the
  retrospective once caught before the merge". The other verdicts, on
  `design@agent-skills@attribution-replaces-quotation`, `design@agent-skills@heads-stand-on-argument`,
  `design@agent-skills@transcript-reviewer-agent`,
  `rejected@agent-skills@owner-words-verbatim-in-plan-items`,
  `rejected@agent-skills@head-cites-every-owner-ruling`, `goal@the-owner-decides` and
  `goal@only-needed-instructions`, are `respect`.
- **d. Its kind.** Lapses of clear instructions, each caught by a review before the merge.
- **e. Its scope.** `goal@the-owner-decides`. Nothing is reversed.
- **f. A better fix.** Rival: count these four now in the deferred issue. It loses: the issue's
  comparison is with a baseline under the old regime, and its trigger names the first batch whose
  sessions ran under a release tagged after the slice; a count of this batch, whose sessions ran
  before any such release, mixes the two regimes it is meant to compare. Second rival: an added
  instruction; it loses by the lapse rule, and by the owner's weighing recorded in the rating head,
  that "the owner does not believe any instruction would do better".
- **g. Its route.** Nothing.
- **h. The proposal.** No change. The issue's trigger stays unmet.

Default: no change.
Outcome: decided by the session: no change, four lapses caught before their merges, and the
deferred count's trigger not met. Ground: the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules`; the trigger of
`issue@agent-skills@attribution-slips-are-not-counted-against-the-regime-of-committed-transcripts`
names a release no tag has made yet; no proposal brings back
`rejected@agent-skills@owner-words-quoted-in-head`.

## standing-2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow: the standing answers and the items that are no finding

Members: the section "Standing questions" of
2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow, lines 89 to 111, and its
section "Items that are no finding", lines 113 to 128. The file predates the questions on agents,
misses, review cost and tools.

Carried by a subject, named only:

- the code started before the premortem's causes were written (lines 97 to 100):
  `premortem-written-after-code`;
- the primer's import line, a span with an empty head, which cannot be written in backticks in a
  commit message, described in words instead (lines 102 to 105): graded under
  `refused-form-escapes-bare-lint`, where its outcome is given; the instance the searchers routed
  there;
- the searchers' quotation checkers and the dispatcher's per-agent split of the check's output, and
  every searcher filling its report by a script (lines 106 to 109):
  `searcher-cannot-check-own-report`, `quotation-check-refuses-cut-punctuation` and
  `searcher-fills-report-by-script`;
- the first commit message attributing to the owner a premise the session had supplied and
  recording a lean as a preference (lines 115 to 117): `owner-attribution-slips`.

Analysed here:

- **docs/roadmap.md edited without loading the planning skill** (lines 94 to 96): "The primer's list
  of installed skills names that load for 'before editing the roadmap'. The session reported this
  to the owner in the turn." Kind: a lapse against `primer@installed-skills`. The batch holds four
  more loads the list names that one other session skipped (plan-directories-slice-1, standing,
  lines 74 to 78): the development skills for a Rust edit, the issue-tracking skill for an issue
  entry, the design skill for a decision against a head's recorded reading, and, by a dispatched
  agent, the agent-configuration skill for an edit of installed text. The first is a member of
  `rust-edit-skips-development-skill` and the third of `head-changed-without-design-skill`. One
  point of delivery, the primer's list read at the session's start and needed at the moment it
  names, recurs across five loads in two sessions. `rust-edit-skips-development-skill` weighs that
  delivery for the development skills; the roadmap load here and the two loads of plan-directories
  no subject carries are sent to it as evidence of the same point of delivery, and its outcome
  decides whether the list's delivery moves.
- **Review catches of the session's own lapses** (lines 118 to 123): a first head bundling two
  decisions, a first measurement of the harness with no control, an incumbent search's account
  listing fewer hits than the grep returns: "Each instruction exists and is clear, and the review
  caught each one." **The in-change path chosen** where the issue fix and the discussion began
  together, "said so, and the owner confirmed it" (lines 124 to 128); the reading of the path's
  condition is weighed by `in-change-path-taken-for-unstarted-work`. Observations.
- **Installed file changed**: the setup skill's source, as the work. **Project skill addition
  missed**: no. **Code rules** (lines 110 and 111): four survivors found by the adversarial reviewer,
  each made a test. Observations.

Default: no change in this section; the three skipped loads no subject carries go as evidence to
`rust-edit-skips-development-skill`.
Outcome: decided by the session: no change here. Ground: each lapse was caught before the merge and
is a lapse against an existing, clear instruction; the recurrence across loads of one list is the
evidence the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules` asks for
a move of delivery, and it is weighed once, under `rust-edit-skips-development-skill`.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## premortem-written-after-code: code written on the in-change path before the premortem's causes

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-transcript-unrecognised-lines-klarch-workflow.md`, in its section on `premortem-written-after-code`, which gives the outcome.

## session-owner-line-unrecognised: the transcript reader reports the harness's `session-owner` line type as unrecognised

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-10-knowledge-architect-record-search-verdicts-klarch-workflow.md`, in its section on `session-owner-line-unrecognised`, which gives the outcome.
