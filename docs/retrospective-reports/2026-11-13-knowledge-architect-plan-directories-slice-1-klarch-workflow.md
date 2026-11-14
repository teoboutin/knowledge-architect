# 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow

- **Received file**: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: the main commit the session's tree contained when its work began, `v0.10.0-5-ga0454f46`; branch `plan-directories`; no uncommitted changes at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@branch-commits-judged-at-tip`: constrains, respect.
  - `design@agent-skills@milestone-split-by-lifetime`: constrains, respect.
  - `design@agent-skills@plan-landing-is-not-tied-to-its-work`: constrains, respect.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `design@check@every-commit-judged-by-the-tip-checker`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@knowledge-architect@git-flow`: constrains, respect.
  - `goal@agents-get-a-complete-workflow`: constrains, respect.
  - `issue@query@no-command-counts-a-tripwire-s-horizon`: touches, respect.
  - `design@agent-skills@standing-state-review-before-every-merge`: constrains, respect.
  - `design@agent-skills@tripwire-stands-on-the-standard`: constrains, respect.
  - `tripwire@agent-skills@silent-read-too-widely`: constrains, respect.
  - `goal@design-is-recorded-with-its-arguments`: constrains, respect.
  - `design@query@occasion-shaped-output`: conflicts, check-reversal.
  - `issue@transcript@an-option-anonymizes-what-transcript-prints`: touches, respect.
  - `design@agent-skills@discussion-record-apart`: constrains, respect.
  - `design@agent-skills@ledger-from-transcript`: constrains, respect.
  - `design@agent-skills@committed-transcript`: constrains, respect.
  - `design@agent-skills@transcript-pushed-on-the-owner-s-word`: constrains, respect.
  - `design@agent-skills@session-acts-on-the-verdicts`: constrains, respect.
  - `design@model@plan-document-directory`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@transcript@transcript-command-reads-any-project`: constrains, respect.
  - `design@transcript@transcript-query-verbs`: conflicts, check-reversal.
  - `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree`: touches, respect.
  - `design@agent-skills@transcript-found-by-listing`: constrains, respect.
  - `design@knowledge-architect@transcript-crate`: constrains, respect.
  - `design@knowledge-architect@query-crate`: constrains, respect.

## worklist-plural-seeds: `worklist` prints "1 seeds"

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow C3.

- **a. What it says.** `worklist --out` with `--seeds-from` prints "1 seeds" where the section gives one seed; a test asserts the wording.
- **b. Whether it still applies.** Yes. At origin/main, `path@query@src/worklist.rs`, near line 400, writes `"{citation}: {taken} seeds from its section \"{DECIDED_SECTION}\""` with no singular, and `path@testing@tests/binary.rs`, near line 8432, asserts `"spec@a-milestone@a-slice: 1 seeds from its section \"What is already decided\""`. The line beside it, near line 407, `"{} entries changed since the commit \"{}\", marked ..."`, has the same defect for one entry (the tests near lines 8245 and 8289 assert "5 entries").
- **c. Whether it is already recorded.** No; the searchers returned no entry on it beyond `design@knowledge-architect@query-crate` (`constrains`), which places the change in `query`. The output is no declared contract, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`, so changing its wording decides nothing a consumer depends on.
- **d. Its kind.** A defect of the checker: wrong text in one case.
- **e. Its scope.** Component `query`; no head, no goal.
- **f. A better fix.** Fix both count lines of the same function, `seed`/`seeds` and `entry`/`entries`, rather than the one reported. Rival: no change, the cost being a reader's moment; but the fix is two format strings and one test assertion.
- **g. Its route.** A change to the Rust source, `skill@klarch-development`, small, its design settled.
- **h. The proposal.** Handle now: the two lines take the singular for a count of one, and the assertion near line 8432 of `path@testing@tests/binary.rs` expects "1 seed from its section". Each singular arm is shown to discriminate with `cargo x mutate`, as the development skill asks. Whether a CHANGELOG entry is owed is the implementer's reading of `design@knowledge-architect@changelog-entries`.

Default: handle now.
Outcome: decided by the session: handle now, under `skill@klarch-development`. Ground: the wording is wrong for one, the fix touches two format strings and their tests, and it changes no head and no declared contract, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

## transcript-branch-attribution-wrong: `transcript list --branch` lists a session that did no work on the branch

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow C2; 2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow C1(a); 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow C2.

- **a. What it says.** Three sessions found `transcript list --branch` (or `--containing`) naming
  their branch among the branches of the previous session, which did no work there: its last lines,
  a retrospective after its own merge, ran after the checkout had moved (plan-directories C2), or
  the lines were written into the old session's file at the new session's start (conformance C1(a),
  planned-splits C2). Each transcript reviewer had to read the session to rule it out. Proposed
  fixes: count a session on a branch from the first line whose checkout is that branch and that its
  own tools wrote (plan-directories C2); derive a session's branches from the owner's or the agent's
  lines, or mark a branch seen only in the harness's lines (conformance C1(a)); list a session only
  where one of its tool calls ran on the branch's checkout (planned-splits C2).
- **b. Whether it still applies.** Yes. At origin/main,
  `cargo klarch transcript list --branch planned-splits` lists `80036d81` (2026-11-13 14:17 to
  21:05, branches `main, plan-directories, plan-directories-new, planned-splits`) beside `3e0588d2`,
  the session that worked there, which starts at 21:04. In `80036d81`'s file, 1,282 lines, the lines
  carrying `gitBranch` `planned-splits` are 6, the first at line 1271 at 21:04:21, four `assistant`
  and two `user` lines: its last lines, written after the other session's checkout moved, measured
  by a `python3 -I` script over that file. The blamed text,
  `design@transcript@transcript-query-verbs`: "`list --branch` computes the sessions that worked on
  a branch from the `gitBranch` field every chain line carries". The reader keeps each distinct
  value of that field as a session's branches, in `Corpus::cut` of `path@transcript@src/reader.rs`,
  as the recorded issue below states.
- **c. Whether it is already recorded.** Not as such. Its converse is
  `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree` (verdict `touches`): a
  session that worked from a worktree carries the branch in none of its lines. That issue states,
  as an assumption it labels "an inference from these counts", "The field names the branch of the
  session's start, not always the branch worked on". The measurement above contradicts it:
  `80036d81` started on `main`, carries four branches, and its line 1270 carries `main` at 21:04:19
  and its line 1271 `planned-splits` at 21:04:21, when the other session had moved the checkout the
  two shared. So in that session the field followed the branch the checkout held when each line was
  written, not the branch of the session's start. That issue closes by "A discussion under `skill@knowledge-architect-design`, since a
  fix changes what `design@transcript@transcript-query-verbs` says `--branch` computes". Every
  proposal here also changes what that head says `--branch` computes (verdict `conflicts`,
  `check-reversal`). `design@agent-skills@transcript-found-by-listing` (verdict `constrains`) keeps
  the listing as the way the review finds a branch's sessions, which each proposal keeps.
- **d. Its kind.** A defect of the checker: a false positive of `--branch`, beside the recorded
  false negative.
- **e. Its scope.** The transcript review's finding of a branch's sessions,
  `design@agent-skills@transcript-found-by-listing`; the owner of the fix is `transcript`. The fix
  rewrites a head, so it goes through `skill@knowledge-architect-design`.
- **f. A better fix.** The measured lines rule out each proposal as stated. In `80036d81`, the six
  lines of 1271 to 1280 that carry a branch carry `planned-splits`: the session's own `thinking`,
  two `Write` tool calls, their two results and its last `text`, all with `cwd` the main checkout,
  after line 1270, its last line on `main`; the other four are metadata lines with no branch. So a rule reading only the agent's lines, or the lines its own tools wrote, or its tool
  calls run on the branch's checkout, still lists it: the session wrote outside the tree while the
  checkout it shares with the next session had moved. What tells the two apart is what the session
  did on the branch, not which lines carry its name, which is a question for the discussion, with
  the worktree case beside it, since one reading of the field decides both: a rule that narrows
  `--branch` can widen or narrow what the worktree case needs. The rival is adding this as an
  instance of the worktree issue; the deciding fact is that that entry's title and "What would close
  it" state a miss and a widening, so a reader of it would not meet a false listing. A new issue
  that names the other as decided in the same discussion keeps each defect findable by its own
  title.
- **g. Its route.** An issue only: a design discussion and a branch of the `transcript` Component.
- **h. The proposal.** Open a defect issue in `transcript`,
  `list-branch-lists-a-session-that-did-no-work-on-the-branch`, with the measurement above, the
  three proposals, and "What would close it": the same discussion as
  `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree`, measuring which lines
  carry which branch in sessions whose checkout moved, with a test over a fixture of a session whose
  last lines carry the next branch.

The worktree issue's assumption is corrected in place in the same commit, after the sentence it
labels, as `primer@verify-before-relying` asks of a diagnosis evidence contradicts.

Default: open the issue, and correct the worktree issue's assumption.
Outcome: decided by the session: open an issue, `transcript`, kind `defect`, `list-branch-lists-a-session-that-did-no-work-on-the-branch`, and correct the assumption of `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree` with the measurement. Ground: the defect stands at origin/main by measurement; its fix rewrites `design@transcript@transcript-query-verbs`, which needs a design discussion, the one the worktree issue already asks for.

Text to add to `issue@transcript@list-branch-misses-a-session-that-worked-in-a-worktree`, in "What", after the sentence that labels the reading an assumption and ends "which harness events write the worktree's branch is not established.":

```
Measured at the intake of 2026-11-14, a session that shared the main checkout contradicts that
reading: it started on `main`, its lines carry four branches, and one of its lines carries `main`
while the next, written two seconds later, after another session had checked out `planned-splits`
in the same checkout, carries `planned-splits`. In that session the field followed
the branch the checkout held when each line was written, not the branch of the session's start. The false listing this causes is
`list-branch-lists-a-session-that-did-no-work-on-the-branch`, decided in the same
discussion.
```

The reference in that block resolves only once the new issue exists: the commit that adds the block opens `list-branch-lists-a-session-that-did-no-work-on-the-branch`, or comes after the one that does, and the analysis that carries the block is committed no earlier.

## passed-horizon-left-standing: a tripwire whose horizon is said to have passed is left standing

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow W2. One session.

- **a. What it says.** The harvest's agent read `tripwire@agent-skills@silent-read-too-widely`, whose horizon names "the fifth plan document deleted at its landing", found that occasion passed long ago, and left the tripwire standing; the standing-state reviewer did not report it. Whether a lapse of the lifecycle's delivery or of one agent: not established. Proposal: none until the cause is established.
- **b. Whether it still applies.** The horizon line, `register@agent-skills@tripwire` lines 283 to 285, unchanged since `v0.10.0-5-ga0454f46`: "**Watches until:** the fifth plan document deleted at its landing on the main branch, and the second report file of the tripwire axis added under docs/audit-reports/ on the main branch, both counted from the commit that writes this line." The line in its current wording was written by `b1e171d7`, "[review] Repairs of the review of the decision that a tripwire stands on the standard", on the main branch (`git blame -L 283,285` on the file); the tripwire itself was added six commits earlier by `4852e12c`, with a horizon of one occasion. Measured on origin/main: `git log --oneline --first-parent b1e171d7..origin/main --diff-filter=D -- docs/plans/` gives 27 commits that delete a file under docs/plans/, so the first occasion has passed; `git log --diff-filter=A --name-only 4852e12c..origin/main -- docs/audit-reports/` gives nothing, so no report of `skill@knowledge-architect-project-audit@tripwire-axis` has been added and the second has not. The tripwire standard reads the line as "the occasions after which the cause, unfired, no longer will", `path@agent-skills@fragments/tripwire-standard.md` line 54: after both. Under that reading the horizon has not passed, and the agent and the reviewer were right to leave it. The finding's premise holds only if the horizon passes at its first occasion.
- **c. Whether it is already recorded.** The tripwire itself (constrains), not fired: no issue on a silent failure lists a cause dropped under condition 1. `design@agent-skills@tripwire-stands-on-the-standard` (respect) makes a change of a tripwire's wording the session's, listed under Decided.
- **d. Its kind.** A defect of the tripwire's wording, unclear: "and ... both counted" was read by one agent as passing at the first occasion. No defect of the lifecycle.
- **e. Its scope.** `register@agent-skills@tripwire`; the two occasions exercise two paths of the guarded decision, a cause dropped at a plan's premortem and a tripwire removed at a tripwire audit, which is the reason to keep both.
- **f. A better fix.** Rival: the first occasion alone, deleting the tripwire now; it loses because the second path, a removal under condition 1 at a tripwire audit, has not been exercised once. The rewording keeps the scope and states the conjunction.
- **g. Its route.** A rewording of the tripwire, the session's per `path@agent-skills@fragments/tripwire-lifecycle.md`, "its wording and its scope are the session's to change while it watches the same cause".
- **h. The proposal.** Handle now, in `register@agent-skills@tripwire`, lines 283 to 285:

```markdown
**Watches until:** the later of two occasions, each counted from the commit that writes this line:
the fifth plan document deleted at its landing on the main branch, and the second report file of
the tripwire axis added under docs/audit-reports/ on the main branch.
```

  Instructions added 0, removed 0. Nearest rival: deletion at the first occasion, which loses as item f says.

Default: handle now, the rewording above; the tripwire stays.
Outcome: decided by the session: handle now (a rewording of the tripwire's horizon line); the tripwire stands, its horizon not passed. Ground: 0 reports of `skill@knowledge-architect-project-audit@tripwire-axis` added under docs/audit-reports/ since `4852e12c`, which added the tripwire, and so none since `b1e171d7`, which wrote its line, and the standard's "the occasions after which" reads both occasions as owed; one reader took the line otherwise, which the rewording closes.

## audit-commit-before-first-change: the audit's amended documents must land in a commit that does not exist yet

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow W1.

- **a. What it says.** `skill@knowledge-architect-planning@working-a-slice` point 2, "Applied in
  place": "Where the slice's first change sets what the per-commit gate requires, per
  `primer@branch-commits`, the amended documents land in that change's commit instead, and its
  message carries the audit's subject and list." The audit ran before any code. Agents that
  implemented the slice in parallel needed the amended spec committed in their worktrees before the
  checker change existed. The session committed the audit alone, then squashed it into the
  checker's commit after the implementation, rebuilt the branch and force-pushed. Proposal: the
  audit may commit on its own, and is folded into the first change's commit when the gate needs it
  (a rewording).
- **b. Whether it still applies.** Yes. At the file's version, a0454f46 (`v0.10.0-5`), line 427 of
  `path@agent-skills@content/skills/planning/SKILL.md` holds the sentence. On main it is unchanged,
  near line 617, in point 2's sort "Applied in place".
- **c. Whether it is already recorded.** No issue or head on this sentence. Every verdict is
  `constrains` or `touches`, action `respect`. The ones that bound a fix:
  `design@check@every-commit-judged-by-the-tip-checker` ("Such a branch puts that change in its
  first commit, with every fix the tree needs to pass it, or is squashed to one commit before its
  review"), `design@agent-skills@branch-commits-judged-at-tip`, `design@knowledge-architect@git-flow`
  ("So work whose intermediate trees cannot pass lands as one commit, squashed before review"), and
  `design@agent-skills@plan-landing-is-not-tied-to-its-work`. The issue
  `issue@agent-skills@how-parallel-work-is-partitioned-among-agents` (`touches`) holds who commits
  when agents work in parallel; its trigger is not met by an intake.
- **d. Its kind.** A defect of the text, unclear. The sentence states where the documents must
  end, and "instead" reads as the moment they are committed. `primer@branch-commits`, which the
  sentence cites, judges the end state: the change "goes in the branch's first commit, with every
  fix the tree needs to pass it, or the branch is squashed to one commit before its review", and "A
  repair that would leave an earlier commit failing is folded into the earliest commit it repairs".
  The session committed the audit alone and squashed it into the checker's commit after the
  implementation, which reaches the first of those end states; the planning sentence reads as
  forbidding that route.
  The analysis that wrote the sentence, the cluster K2 of the intake carried out in d1a3095a, met a
  session that had folded the audit into the code commit, and weighed no rival on when the audit is
  committed (`git show d1a3095a^:docs/retrospective-reports/2026-10-08-knowledge-architect-agent-configuration-entities-klarch-workflow.md`,
  its section on cluster K2): the fold was never argued and lost.
- **e. Its scope.** The installed text, at its source in agent-skills. The fix reverses no head:
  the heads above state the end state, and keep it.
- **f. A better fix.** The cause is a restatement that drifted from its home: the planning
  sentence applies `primer@branch-commits` as a rule on the moment of the commit, where the primer
  rules on the commits the branch holds at its review. The repair brings it back: the sentence
  keeps the end state and admits a fold before the review. Nearest rival: no change, since the
  session found the legal route with the text as it stands. The fact that decides: the session read
  the sentence as excluding the route it then took by a rebuild and a force-push, and parallel
  implementers need the amended documents committed before the first change exists. The report's
  own wording, "may commit on its own and is folded", says the same; the text below keeps the
  primer's word "folded".
- **g. Its route.** A fix in the installed text: edited at its source, as
  `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says, under
  `skill@knowledge-architect-agent-configuration`.
- **h. The proposal.** Handle now. In `path@agent-skills@content/skills/planning/SKILL.md`, point 2,
  sort "Applied in place", replace the sentence quoted in a by the text below. Instructions added:
  0; removed: 0 (a rewording that makes the instruction's intent match its home).

```text
Where the slice's first change sets what the per-commit gate requires, the amended documents end
in that change's commit, committed with it or folded into it before the review, per
`primer@branch-commits`, and its message carries the audit's subject and list.
```

Default: handle now, the rewording above.
Outcome: decided by the session: handle now, the rewording above. Ground: `primer@branch-commits`
and `design@check@every-commit-judged-by-the-tip-checker` judge the commits a branch holds at its
review, which a fold before the review meets; the planning sentence reads as a rule on the moment
of the commit, a restatement brought back to its home, which changes no instruction.

## plan-search-artifacts-by-hand: a plan's search copy and its transcript cut and anonymised by hand

Members: 2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow C1.

- **a. What it says.** The planning skill asks the assembly to commit a transcript cut at the
  owner's last message before the merge, with every path under a home directory, e-mail address and
  credential removed, and the audit to copy `fit-lines.md` into the slice's
  `search/design-audit.txt`. The session did each by hand or by an ad hoc script: the cut line found
  in `transcript show` output, a regex replacing 100 home paths, a grep for e-mail addresses and
  tokens, a copy of the audit's lines. Proposal: a `worklist --verify` option that writes the
  audit's lines into the plan's `search/`, and the transcript's anonymising option extended to cut
  at a named message.
- **b. Whether it still applies.** Yes. No such option exists on main: the planning skill's point 2,
  near line 586 of `path@agent-skills@content/skills/planning/SKILL.md`, still has the audit copy
  `fit-lines.md` "as `search/design-audit.txt` beside the audited `README.md`", and the
  anonymising option is still an open issue.
- **c. Whether it is already recorded.** In part.
  `issue@transcript@an-option-anonymizes-what-transcript-prints` (`todo`, `touches`) records the
  owner's proposal of an option of `show` that removes home paths and branch SHAs, "its name and its
  exact replacements decided by a discussion", and names the scripted search for home paths and
  e-mail addresses as part of the same pass. The cut at a named message is not in it. The copy of
  the fit lines: `design@query@occasion-shaped-output` (`conflicts`, `check-reversal`) fixes the
  files a passing `--verify` writes, and `design@model@plan-document-directory` fixes the files of a
  plan's `search/`; writing into a plan's directory from `--verify` would change the first.
  `design@agent-skills@committed-transcript` keeps the session's whole reading of the transcript
  before its commit, whatever a command removes.
- **d. Its kind.** A defect of the checker's tools, missing: a task with no judgement carried by
  hand. For the copy, a cost of one command.
- **e. Its scope.** The `transcript` and `query` Components. `goal@tasks-a-tool-does-better-are-the-tool-s`:
  "A task is taken in by a decision of knowledge-architect's owner, never because the workflow asks
  it." Its verdict carries an owner-question for each such task. This section proposes no command,
  so the question is not put: the anonymising part is already the owner's proposal on its issue,
  and the copy is not worth a command.
- **f. A better fix.** For the transcript part, one fix: the cut and the anonymising pass are the
  same extraction of the same transcript, so they belong in one discussion, that of the recorded
  issue. For the copy, the nearest rival to the proposal is no change: `cp` of one file the check
  already wrote; the fact that decides is that the copy is one command with no choice in it, while
  the proposal reverses `design@query@occasion-shaped-output`.
- **g. Its route.** An instance added to a recorded issue; no change for the copy.
- **h. The proposal.** Add the instance below to
  `issue@transcript@an-option-anonymizes-what-transcript-prints`, at the end of its "What"; no
  change for the copy of the fit lines.

```text
A plan document's assembly in the first slice of the milestone on split guidelines and plan
directories carried the whole extraction by hand: the cut line found in `transcript show` output,
a regex replacing 100 home paths, and a grep for e-mail addresses and tokens. The same discussion
weighs a cut at a named message of the owner, where the planning skill's extraction ends,
per C1 of the retrospective file `2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow`.
```

Default: an instance added to the recorded issue; no change for the copy.
Outcome: decided by the session: an instance added to
`issue@transcript@an-option-anonymizes-what-transcript-prints`; no change for the copy of the fit
lines. Ground: the issue holds the owner's proposal and the discussion it owes; the copy is one
command, and a writing `--verify` would reverse `design@query@occasion-shaped-output` on no
argument beyond that command's cost.

## standing-2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow, lines 69 to 103. Its counts
are in the lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`; its tool
report is item 3 of the owner item in the analysis of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- the Rust lint repair written without the development skills, and the project's development skill
  missed in it (lines 74 and 75, 84): `rust-edit-skips-development-skill`; the change decided against
  a head's recorded reading without the design skill (lines 76 and 77):
  `head-changed-without-design-skill`; the issue entry opened without the issue-tracking skill, and
  the installed text edited by an agent without the agent-configuration skill loaded whole (lines 75
  to 78): carried to `rust-edit-skips-development-skill` as further loads skipped at the same point
  of delivery, `primer@installed-skills`, as the standing section of
  2026-11-11-knowledge-architect-primer-import-first-line-klarch-workflow says;
- three agents' commit-message claims found false (a SHA said inside the range; "two instructions
  added"; a suite said passed) (lines 79 to 83, 93 and 94): `message-claims-written-unchecked`;
- C1, the squash and rebuild of W1, and four rewords (lines 86 and 87):
  `plan-search-artifacts-by-hand`, `audit-commit-before-first-change` and
  `no-command-rewords-branch-messages`.

Analysed here:

- **Work an agent could have carried** (lines 89 and 90): "the session read about 33,000 characters
  of the audit's gap verdicts to sort them; a dispatched sorter with the spec would have cost less
  context." Kind: an observation of the dispatch judgement of `primer@dispatching-work`, which the
  session judges; no text blamed.
- **An agent returned unfinished** (lines 91 and 92): "the migration agent returned uncommitted,
  blocked by a false finding of a lint whose fix lay in the slice's own code; the brief had kept code
  out of its scope." Kind: an observation; the report named its blocker, so it was acted on without
  redoing, and the trigger of `issue@agent-skills@dispatched-agents-write-their-report-as-they-work`
  is not met.
- **Missed entry**: "none known" (line 95). **Installed files changed**: by the slice's design, at
  the source. **Backticks**: none. **Code rules**: none met. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: observations of the dispatch judgement and of
an agent that reported its blocker, both acted on without redoing.

## tripwire-silent-read-too-widely: analysed under the subject `passed-horizon-left-standing`

Members: `tripwire@agent-skills@silent-read-too-widely`; W2 of
2026-11-13-knowledge-architect-plan-directories-slice-1-klarch-workflow.

Whether its horizon passed, at its first occasion or only at both, and whether the batch's dropped
premortem causes (conformance-before-harvest W2; transcript-reader-gaps, standing, line 54) fire
it, are analysed once, in the section of the subject `passed-horizon-left-standing`, in the
analysis of the first file that holds it.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## head-changed-without-design-skill: code or an agent's edit contradicted, narrowed or rewrote a head with no deliberation

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-issue-triage-klarch-workflow.md`, in its section on `head-changed-without-design-skill`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.

## rust-edit-skips-development-skill: a small Rust edit inside another task, made without the development skills

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `rust-edit-skips-development-skill`, which gives the outcome.
