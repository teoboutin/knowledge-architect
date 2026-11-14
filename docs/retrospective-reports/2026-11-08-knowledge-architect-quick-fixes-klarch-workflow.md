# 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow

- **Received file**: 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: the main commit the session's tree contained when its work began is `v0.9.0-214-g5b88b090`; the branch was `quick-fixes-create-git-model`; the tree held no uncommitted changes."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@code-change-reads-its-record`: constrains, respect.
  - `design@agent-skills@no-routing-table`: constrains, respect.
  - `rejected@agent-skills@root-routing-table`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@the-workflow-leads-to-better-code`: constrains, respect.
  - `design@mutate@report-not-gate`: constrains, respect.
  - `design@mutate@baseline-runs-by-default`: conflicts, check-reversal.
  - `issue@mutate@an-environment-failure-reads-as-caught`: touches, respect.
  - `issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch`: touches, none.
  - `design@agent-skills@test-economy`: constrains, respect.
  - `issue@agent-skills@the-upstream-set-omits-the-published-libraries`: touches, none.
  - `design@knowledge-architect@mutation-command-host`: constrains, respect.
  - `goal@generic-tools-and-procedures-are-shipped`: constrains, respect.
  - `design@agent-skills@checker-model-in-primer`: constrains, respect.
  - `design@agent-skills@staged-check-before-each-commit`: constrains, respect.
  - `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: trigger-met, rule-before-work.
  - `design@check@staged-tree-source`: constrains, respect.
  - `design@check@staged-check-semantics`: constrains, respect.
  - `design@check@check-judges-no-given-commit`: constrains, respect.
  - `issue@check@no-head-argues-that-the-checker-runs-from-a-pre-commit-hook`: touches, none.
  - `design@check@a-commit-message-is-a-document`: constrains, respect.
  - `design@check@message-judged-byte-for-byte`: constrains, respect.
  - `design@check@one-command-judges-messages-after-the-commit`: conflicts, check-reversal.
  - `design@check@message-references-resolve-in-either-tree`: constrains, respect.
  - `rejected@check@commit-msg-hook`: brings-back, new-argument-needed.
  - `rejected@check@commit-subcommand`: constrains, respect.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `design@knowledge-architect@cli-crate`: constrains, respect.
  - `goal@tasks-a-tool-does-better-are-the-tool-s`: constrains, respect.
  - `design@agent-skills@verification-in-primer`: constrains, respect.
  - `design@agent-skills@questions-go-to-their-authority`: constrains, respect.
  - `design@agent-skills@owner-word-holds-as-far-as-its-premise`: constrains, respect.
  - `design@agent-skills@derived-decision-proceeds-on-default`: constrains, respect.
  - `goal@the-owner-s-word-is-verified`: constrains, respect.
  - `goal@design-with-agents-as-counterparts`: constrains, respect.
  - `goal@only-needed-instructions`: constrains, respect.

## no-check-of-draft-message: a message that fails `commits` while work is uncommitted waits for a history edit

Members: 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow C2.

- **a. What it says.** The primer says of `commits`: "Run `cargo klarch commits <main>..HEAD` after
  each commit, where an amend still repairs a failing message." The file: "The root instructions of
  a project may bar an amend while the tree holds uncommitted changes, as this project's do. Two of
  the session's messages failed the check after they were committed (a bare slug of an entry the
  parent tree holds, and a path written where a register reference was due), with other work still
  uncommitted, so each had to wait for a history edit of the whole branch once the tree was clean."
  Proposal: "`cargo klarch commits` takes a draft message, such as `--message <file>`, judged against
  the staged tree and HEAD as its first parent", and the primer names that run before each commit
  (one instruction reworded, none added).
- **b. Whether it still applies.** Yes. At `v0.9.0-214-g5b88b090` and at `v0.10.0-59-g605d1538`, the
  root `CLAUDE.md`, `## Git` point 2, lists `commit --amend` among the operations run while
  "uncommitted changes exist in the tree" may not, and `primer@checker-model` still says "Run
  `{{command}} commits <main>..HEAD` after each commit, where an amend still repairs a failing
  message" in `path@agent-skills@content/PRIMER.md`. `commits --help` takes a range and no message
  file.
- **c. Whether it is already recorded.** The proposal is recorded as lost.
  `design@check@one-command-judges-messages-after-the-commit`: "No hook judges a draft before the
  commit exists", on three reasons, one of which is that a draft check "resolves a message against
  HEAD, which under an amend, a reword or a squash is the commit being replaced, so it refuses
  history edits the range check accepts". `rejected@check@commit-msg-hook` (`brings-back`,
  `new-argument-needed`) holds the same reason, and `rejected@check@commit-subcommand` repeats it.
  A command in place of a hook answers the per-clone reason, not the parent reason: under an amend
  HEAD is still the commit being replaced. The file's case is new to the record, though: the head's
  premise that "a finding in the newest commit's message is repaired by `git commit --amend`" does
  not hold in a project whose rules bar the amend while work is uncommitted, as this repository's
  do, per `design@knowledge-architect@no-operation-loses-content`. No issue holds that case.
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  is graded `trigger-met`; the next section judges it. Two other verdicts bound a draft check:
  `design@check@message-judged-byte-for-byte` (`constrains`): "`commits` reads the message with
  `%B` and cleans nothing: git applied its own cleanup before the commit existed", so a draft is
  judged on bytes git has not yet cleaned, and a draft check must say which bytes it judges; and
  `issue@check@no-head-argues-that-the-checker-runs-from-a-pre-commit-hook` (`touches`, action
  none), whose discussion argues judging a commit's tree before it exists against "a checker that
  judges a commit only once it exists, through `commits`", the tree half of the same question.
- **d. Its kind.** Missing, at the meeting of two recorded decisions: the checker's after-commit
  repair by an amend, and this repository's bar on an amend with uncommitted work. Neither text is
  wrong alone.
- **e. Its scope.** The checker, `commits`, and the root's git rules;
  `goal@documentation-stays-consistent`. A draft check reverses
  `design@check@one-command-judges-messages-after-the-commit`; a change of the root's rule changes
  `design@knowledge-architect@no-operation-loses-content`.
- **f. A better fix.** Two rivals the file does not name. First, at the root's rule: an amend that
  changes only the message, `git commit --amend --only -F <file>`, which git's manual says "can be
  used to amend the last commit without committing changes that have already been staged", removes
  no content from the working tree, and the commit it replaces keeps its tree in the new one, so
  only the old message leaves; admitting it would repair the newest message at once. It changes
  the root's list of operations, a decision of this repository. Second, a draft check with an
  explicit first parent, which may answer the parent reason of the rejected hook, an assumption the discussion checks. A third is no
  change: the message waits for the clean tree, at the cost of a history edit the root already
  describes. Which wins turns on costs no record weighs, so it is a design question.
- **g. Its route.** `skill@knowledge-architect-design`, then `skill@klarch-development` for a
  checker change, or `skill@knowledge-architect-agent-configuration` for the root's rule. A branch
  of its own.
- **h. The proposal.** Open an issue in `check`, kind `design`, slug
  `issue@check@a-failing-message-waits-for-a-history-edit-while-work-is-uncommitted`: the two instances of the
  quick-fixes session; the mechanism, that the head's repair by an amend is barred by a project
  rule on uncommitted work; the three shapes above, with the reason of
  `rejected@check@commit-msg-hook` that the draft check must answer and the bytes
  `design@check@message-judged-byte-for-byte` asks it to name; a pointer to
  `issue@check@no-head-argues-that-the-checker-runs-from-a-pre-commit-hook`, the tree half; its re-entry point, a design
  discussion when work next changes what a session runs after a commit, or earlier on the owner's
  word. Its `Why it matters` names `design@check@one-command-judges-messages-after-the-commit` and
  `design@knowledge-architect@no-operation-loses-content`.

Default: open the issue.
Outcome: decided by the session: open an issue, `check`, kind `design`, slug
`issue@check@a-failing-message-waits-for-a-history-edit-while-work-is-uncommitted`. Ground: the proposal brings
back `rejected@check@commit-msg-hook` without answering its parent reason, while the case it reports
defeats a premise of `design@check@one-command-judges-messages-after-the-commit` in this repository;
the shapes that answer it change a recorded decision, so the work is a design discussion on a
branch of its own.

## The re-entry point of the issue on the claim check of a commit message

Members: the verdict `trigger-met` on
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
given against three subjects: `no-check-of-draft-message` above, `message-claims-written-unchecked`
and `broad-trigger-met-by-every-change`. This section is the one ruling on it; the sections of the
other two subjects point here.

- **What the entry says.** "Re-entry point: a design discussion under
  `skill@knowledge-architect-design`, raised when work next changes what a session reads or runs at a
  commit, or earlier on the owner's word."
- **Whether it is met.** No. The verdict's three reasons, each judged:
  - quick-fixes C2 proposes a draft check run before each commit. A proposal is no work: the
    outcome above opens an issue and changes no text a session reads, and no command it runs, at a
    commit;
  - the further lapse instances of the files are instances, which the section
    `message-claims-written-unchecked` adds to the entry; its outcome changes nothing a session
    reads or runs at a commit either;
  - plan-directories-slice-1 P2 reports that the slice's own audit graded the entry `trigger-met`
    and the session argued it was not the occasion. That ruling is on main, in the message of the
    commit "[model] Plan documents as directories, with model types of their own, and the tree's
    plan documents migrated": the slice's new steps "bind at three commits of plan work alone", and
    "The entry stays open on its trigger". The file's proposal to reword the re-entry point is
    judged in the section `broad-trigger-met-by-every-change`, which keeps the wording.
- **The response.** None at this intake. The new issue's discussion, if it changes what a session
  runs at a commit, meets this point, and a delivery at the commit may serve both; the new entry
  cites this one in its re-entry point, so that discussion takes it along.

Default: not met; the new issue cites this entry.
Outcome: decided by the session: no change; not met. Ground: the entry's re-entry point is met by
work that changes what a session reads or runs at a commit; no outcome of this intake changes
either, and the one change on main the files report was judged at its own slice, in the commit
named above.

## mutate-zero-tests-reads-survived: a mutation run whose test filter matches no test reads `SURVIVED`

Members: 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow C1; 2026-11-11-knowledge-architect-primer-import-first-line, the slip it notes as no finding.

- **a. What it says.** Two sessions named a test by a wrong module path: the baseline printed
  `running 0 tests` and passed, and the mutated run printed `running 0 tests` and `SURVIVED`
  (quick-fixes C1, `v0.9.0-214`); the same slip, corrected by the session (primer-import, project
  file, `v0.9.0-304`). Blamed: the development skill, "**The command names the test under check by
  its module path, with `-- --exact` after it.** A filter that matches no test runs nothing, cargo
  exits 0, and the mutation reads as `SURVIVED`". Proposed fix: the mutation library refuses a
  baseline whose command reports that it ran zero tests, naming the filter, and the skill's sentence
  shrinks to the `--exact` rule (one instruction removed, none added). Rival named by the file: keep
  the warning alone, which "was present and read, and the run still produced a false verdict".
- **b. Whether it still applies.** Yes. The sentence is at line 96 of
  `path@agent-skills@content/skills/development/SKILL.md` at `5b88b090` and at origin/main,
  unchanged. No code of `path@mutate@src/` reads a test count:
  `grep -rn -i 'running 0\|0 tests' crates/mutate/src/` returns nothing. `--exact` does not guard
  the case: a wrong module path with `--exact` still matches no test, and cargo still exits 0, as
  `cargo test -q -p knowledge-architect-mutate --lib no_such_module::no_such_test -- --exact` shows
  at origin/main: `running 0 tests`, `0 passed; ... 3 filtered out`, exit 0.
- **c. Whether it is already recorded.** Not this state. Its twin is
  `issue@mutate@an-environment-failure-reads-as-caught` (verdict `touches`), a false `caught`, whose
  closing work, "The verdict names what failed, as the failing test's name or the command's last
  lines", is asked of a `caught`; widened to every verdict, the command's last lines would show
  `running 0 tests` beside a `SURVIVED` too. Both misread a run, the subject of
  the cluster `issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch` (verdict `touches`),
  which lists that twin as a member. The proposal widens `design@mutate@baseline-runs-by-default`
  (verdict `conflicts`, `check-reversal`), whose rule is "a failure there refuses the run": a
  refusal of a run of zero tests is a second refusal, and it reads cargo's own output, where the
  tool otherwise takes any command after `--` and reads only its exit code, per
  `design@mutate@report-not-gate`.
- **d. Its kind.** A defect of the checker's tools: a false verdict the tool can see, met twice with
  the warning read. Not a lapse: the instruction says what to look for and gives no check, and both
  sessions read it.
- **e. Its scope.** `goal@the-workflow-leads-to-better-code`, through
  `design@agent-skills@test-economy`: a false `SURVIVED` is a false record of a test's evidence. The
  owner of the fix is `mutate`; the skill's sentence is installed text of `agent-skills`, edited
  only once the tool carries the trap.
- **f. A better fix.** Three shapes: the baseline refuses a run of zero tests (the file's), which
  widens the baseline's head and ties the tool to cargo's output; every verdict line prints the
  command's last lines, the twin issue's closing work widened from a `caught` to every verdict,
  which shows `0 passed` with no new refusal and no reading of cargo's format; or no change, the warning kept. The deciding fact
  against no change is the file's: the warning was read and the verdict was still false, twice.
  Between the first two, the second closes both false verdicts with one change, so it is the
  cluster's to weigh.
- **g. Its route.** An issue only, joining the cluster's branch.
- **h. The proposal.** Open a defect issue in `mutate`,
  `issue@mutate@a-filter-that-matches-no-test-reads-as-survived`, with the two instances, the three shapes and
  the head the first would widen, and add it to the members of
  `issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch`.

Default: open the issue and add it to the cluster.
Outcome: decided by the session: open an issue, `mutate`, kind `defect`, `issue@mutate@a-filter-that-matches-no-test-reads-as-survived`, a member of `issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch`. Ground: the false verdict stands at origin/main; one fix shape widens `design@mutate@baseline-runs-by-default` and another is the twin issue's closing work, so it is weighed on that cluster's branch.

Text to add to `issue@mutate@the-mutation-run-s-false-verdicts-on-one-branch`, in the list of members under "What":

```
- `issue@mutate@a-filter-that-matches-no-test-reads-as-survived`
```

in the sentence that says why they go on one branch, "because both misjudge" becomes "because
each misjudges", since the members become three; and after that sentence:

```
The false `SURVIVED` of a run whose filter matches no test joins them, since it misreads the same
verdict, and the closing work of the false `caught`, a verdict that names the command's last lines,
widened to every verdict, is one of its fixes.
```

The member's reference resolves only once its entry exists: the commit that adds it to this list
opens `issue@mutate@a-filter-that-matches-no-test-reads-as-survived`, or comes after the one that
does, and the analysis that carries this block is committed no earlier.

## development-grounding-skips-tripwires: the touched Components' tripwires read in part before a Rust edit

Members: the standing answers of 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow and
2026-11-12-knowledge-architect-value-checks-while-parsing-klarch-workflow, on a project skill's
addition missed.

- **a. What it says.**
  - quick-fixes: "The project's development skill adds reading the touched Component's tripwires
    before an edit; the session read the Components' scoped instructions and design homes, and not
    their tripwire homes. The standing-state reviewer read every tripwire home and found none
    bearing on the change."
  - value-checks-while-parsing: "The session read the `cli` Component's, but not those of the two
    other Components it edited (`transcript`, `create`). Nothing in them bore on the change, which
    the standing-state reviewer's re-read of every tripwire and deferred trigger confirms."

  Neither proposes a fix.
- **b. Whether it still applies.** The instruction is the same at both versions (5b88b090,
  8e23dd57) and at origin/main: `path@agent-config@skills/klarch-development/SKILL.md`, section
  "Ground before editing", lines 23 to 25 at those versions and 22 to 24 at origin/main: "read
  the Component's own files: its `CLAUDE.md`, for the invariants and traps that hold of the code as it stands; its issues and its tripwires, for
  what is outstanding and what would reopen a decision."
- **c. Whether it is already recorded.** No entry records it. The verdicts on
  `design@agent-skills@code-change-reads-its-record`, `design@agent-skills@no-routing-table`,
  `rejected@agent-skills@root-routing-table` and `goal@the-workflow-leads-to-better-code` are
  `respect`: a repair would be no root table mapping the project skill to the installed one.
- **d. Its kind.** Lapses of a clear instruction, twice, with no consequence: the standing-state
  reviewer re-read every tripwire home before each merge and found none bearing.
- **e. Its scope.** `goal@the-workflow-leads-to-better-code`; the owner of the text is this
  repository's agent configuration. Nothing is reversed.
- **f. A better fix.** Rival: reword "the Component's own files" as "the own files of each
  Component the change edits", a rewording that adds no instruction, for the value-checks case. It
  loses: "the Component" is the one the installed grounding names, "the Component that holds it",
  of each piece of code changed (`path@agent-skills@content/skills/development/SKILL.md`, section
  "Ground before editing"), so a change in three Components already owes three; and the
  quick-fixes session read no tripwire home at all, which no wording repairs.
- **g. Its route.** Nothing.
- **h. The proposal.** No change.

Default: no change.
Outcome: decided by the session: no change, two lapses with no consequence. Ground: the lapse rule
of `skill@knowledge-architect-retrospective-intake@admission-rules`; the instruction reads per
Component with the installed grounding it adds to.

## owner-premise-unchecked: questions put to the owner on premises the session had not checked

Members: the standing answers of 2026-11-08-knowledge-architect-quick-fixes-klarch-workflow,
2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow,
2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow,
2026-11-11-knowledge-architect-issue-triage-klarch-workflow,
2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow,
2026-11-13-thaum-pin-move-0-10-klarch-workflow, 2026-11-14-thaum-casting-process-design-klarch-workflow.

- **a. What it says.** Seven files report a question or a default put to the owner on a premise
  the session supplied and had not checked, each found before the merge and put to the owner again:
  - quick-fixes: an issue presented as quick from a subagent's report, the record unread; the
    grounding found the design head that makes it a design question;
  - goals-first-slice: one question of the second round on a false premise, found by a reviewer of
    the third round;
  - goals-second-slice: the rewrite of a released changelog line put as "a case you rule under" a
    rule that admitted only another case;
  - issue-triage: two questions, one premise false, one leaving out a cost;
  - 2026-11-12-knowledge-architect-new-crate-placeholder: a policy presented as forbidding a rival, and a rate limit reported per
    account without the exemption that removed it;
  - thaum pin-move: the owner's memory called a false premise from a file listing, without
    searching the issues, where an issue recorded the owner's approval;
  - thaum casting-process-design: a fork's cost given as "about 6 more roadmap items" uncounted,
    where the roadmap held nine.

  No file proposes a fix; each calls it a lapse.
- **b. Whether it still applies.** The rule holds at origin/main, unchanged since the versions the
  files name: `path@agent-skills@content/PRIMER.md`, lines 45 and 46, "A premise the session supplied is
  the session's to check before the owner rules on it", and line 166, "The same checks apply to an
  answer given to the owner in the conversation." The design skill restates it at the moment most
  questions are put, `path@agent-skills@content/skills/design/SKILL.md`, line 121: "**A premise you
  supplied is checked before the owner rules on it**".
- **c. Whether it is already recorded.** No entry records premises as such. The verdicts on
  `design@agent-skills@verification-in-primer`,
  `design@agent-skills@owner-word-holds-as-far-as-its-premise`,
  `design@agent-skills@questions-go-to-their-authority`,
  `design@agent-skills@derived-decision-proceeds-on-default`, `goal@the-owner-s-word-is-verified`,
  `goal@design-with-agents-as-counterparts` and `goal@only-needed-instructions` are all `respect`:
  the rule stays in the primer. A premise of a question is an answer given to the owner, which
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  already counts ("Its sentence on answers to the owner is reported lapsed the same way").
  `issue@agent-skills@attribution-slips-are-not-counted-against-the-regime-of-committed-transcripts`
  counts "a premise not shown" among its kinds, for a later batch: its trigger is not met, as
  `owner-attribution-slips` establishes.
- **d. Its kind.** Lapses of a clear instruction. They fall at seven different moments: a triage
  report, a round of the goals slice, a released changelog line, two questions of a triage, a
  publish policy, a memory of the owner and a fork's cost. One, casting-process-design, was in a design discussion, where the design skill's
  restatement is delivered.
- **e. Its scope.** `goal@the-owner-s-word-is-verified`. The owner of a fix would be the installed
  text. Nothing is reversed.
- **f. A better fix.** Rival: a restatement in each skill where it puts a question to the owner.
  It loses: a lapse is not repaired by adding text, the seven moments are not one point of
  delivery, and the one instance at a moment that already restates the rule, the design
  discussion, shows a restatement did not stop it. The
  mechanism, a rule read at the start and needed later, is the claim-check issue's.
- **g. Its route.** An instance of a recorded issue.
- **h. The proposal.** No text change. The seven files are counted in the claim-check issue's
  paragraph that `message-claims-written-unchecked` writes, as questions put to the owner on an
  unchecked premise.

Default: the instances are counted in the claim-check issue, with no text change.
Outcome: decided by the session: an instance added to
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`,
in the paragraph of `message-claims-written-unchecked`. Ground: the lapse rule of
`skill@knowledge-architect-retrospective-intake@admission-rules`; the primer's line 166 makes an
answer to the owner part of that issue's rule; the instances fall at no single moment of delivery.

## standing-2026-11-08-knowledge-architect-quick-fixes-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-08-knowledge-architect-quick-fixes-klarch-workflow, lines 66 to 91. The file predates the
questions on agents, misses, review cost and tools.

Carried by a subject, named only:

- the issue presented to the owner as quick on a triage subagent's "no design decision", the record
  unread (lines 74 to 78): `owner-premise-unchecked`;
- a review record saying the gates had passed "with these repairs" after a later test (lines 79 to
  81): `message-claims-written-unchecked`;
- the touched Components' tripwire homes not read before the edit (lines 82 to 85):
  `development-grounding-skips-tripwires`;
- the reports saved by hand, C3, and the messages judged only after their commit, C2 (lines 87 and
  88): `no-command-saves-subagent-report` and `no-check-of-draft-message`.

Analysed here:

- **The order of four roadmap rows chosen by the session** (lines 70 to 73): "the order among four
  rows the owner asked to put at the top of the roadmap is the owner's, as the planning skill says;
  the session chose it and committed it without putting it to the owner. Three reviewers reported
  it, and the owner ruled before the merge." Kind: a lapse against
  `design@agent-skills@roadmap-order-is-the-owners`, "A row of the roadmap is added or moved on the
  owner's word, since ordering known work is a fork", which the installed planning skill states and
  the file quotes as clear. The record search graded the head `unrelated`, its searcher noting the
  lapse under "Met outside the task". It reached no main branch unruled: the owner ruled before the
  merge, so `open-fork-classed-as-settled` is not met (its own section). One
  instance.
- **Code rules** (lines 89 to 91): a one-line predicate made a pure function with a unit test, "it
  cost one function and one test". An observation; it bears on
  `tripwire@agent-skills@shape-findings-fragment-code` and does not fire it (its own section).
- **Installed file changed**: no. **Backticks**: no. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: the roadmap order is one lapse against a head
the planning skill states and the session's reviewers applied before the merge, which the lapse
rule of `skill@knowledge-architect-retrospective-intake@admission-rules` does not repair by text;
the rest are observations.

## no-command-saves-subagent-report: the dispatcher saved each reviewer's report from the message, by hand or by a script over transcript files

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `no-command-saves-subagent-report`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
