# 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow

- **Received file**: 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from its own source: the main commit the session started from is `v0.10.0-43-gcac87723`; the branch `component-arguments`; the tree clean at the start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@staged-check-before-each-commit`: constrains, respect.
  - `design@agent-skills@staged-forms-for-a-partial-commit`: constrains, respect.
  - `design@agent-skills@project-file-goes-to-an-intake`: constrains, respect.
  - `rejected@agent-skills@retrospective-finding-straight-to-issue`: constrains, respect.
  - `design@agent-skills@plan-landing-is-not-tied-to-its-work`: constrains, respect.
  - `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`: touches, respect.
  - `design@check@finding-names-the-repair`: constrains, respect.
  - `issue@check@the-installer-s-comparisons-on-one-branch`: touches, none.
  - `design@check@staged-check-semantics`: constrains, respect.
  - `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`: touches, none.
  - `issue@check@a-staged-installed-finding-can-name-a-repair-that-does-nothing`: touches, none.
  - `design@check@inputs-are-gathered`: constrains, respect.
  - `design@check@installed-files-compared-by-bytes`: constrains, respect.
  - `design@check@only-check-compares-installed-files`: constrains, respect.
  - `design@knowledge-architect@check-crate`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `goal@documentation-stays-consistent`: constrains, respect.
  - `design@agent-skills@pre-harvest-conformance`: conflicts, check-reversal.
  - `rejected@agent-skills@conformance-in-the-harvest`: constrains, respect.
  - `design@agent-skills@instruction-record-is-minimal`: constrains, respect.
  - `design@agent-skills@term-defined-at-first-load`: constrains, respect.
  - `design@agent-skills@design-home-is-built-intent`: constrains, respect.
  - `design@agent-skills@scoped-auditor-on-touched-heads`: conflicts, check-reversal.

## check-staged-misjudges-installed-copies: `check --staged` compares staged installed copies with shipped text built from the working tree

Members: 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow C2,
2026-11-14-knowledge-architect-conformance-before-harvest-slice-klarch-workflow C2 (its first
item), 2026-11-14-knowledge-architect-planned-splits-slice-klarch-workflow C3.

- **a. What it says.** Three sessions of this repository, all on 0.10.0 builds, met the same false
  finding. In component-arguments, `cargo klarch check --staged` "reported four installed files as
  differing while their sources' edits were unstaged and the staged tree was consistent"; the
  session committed and judged the commit with `commits` instead. In conformance-before-harvest,
  "`check --staged` reported four installed copies as differing, while the next commit's source
  edits sat unstaged". In planned-splits, a repair agent committing one installed-text fix apart
  from others "checked that commit's tree in a scratch worktree", and "a searcher graded the issue
  that records it unrelated to the slice". Each file calls it already recorded and proposes no fix.
- **b. Whether it still applies.** Yes. The versions are `v0.10.0-43-gcac87723`,
  `v0.10.0-35-g2abdaa89` and `v0.10.0-21-ga9ed6e3f`. The comparison is the first loop of the
  installed-file check in `path@check@src/check/agents.rs`, over the shipped text the binary was
  compiled with. No commit touched it, nor `path@check@src/agents.rs`, between the earliest of the
  three versions and origin/main: `git log a9ed6e3f..origin/main` over both files lists none. The issue entry is open at `v0.10.0-59-g605d1538`, last
  changed by the commit "[intake] Twenty-one issue entries opened and nineteen gaining instances,
  from the intake of forty files".
- **c. Whether it is already recorded.** Yes:
  `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, kind `defect`,
  whose summary states the case word for word: "`check --staged` compares each staged installed
  file with that text". It holds its first reproduction and two further instances, of 2026-10-25 and 2026-11-01.
  The searcher's verdicts on `design@check@staged-check-semantics`,
  `design@check@installed-files-compared-by-bytes`, `design@check@only-check-compares-installed-files`
  and `design@check@inputs-are-gathered` are `constrains`, `respect`: the repair the issue names, a
  comparison with the staged shipped text, keeps each. The neighbour entry
  `issue@check@a-staged-installed-finding-can-name-a-repair-that-does-nothing` is a member of the
  cluster `issue@check@the-installer-s-comparisons-on-one-branch`; this issue is not, and the
  searcher's `touches` on that cluster, action none, asks nothing here.
- **d. Its kind.** A defect of the checker, already recorded; three further instances.
- **h. The proposal.** The three instances are added to the issue's Details. Planned-splits' miss
  of the issue by its searcher is that file's standing answer on a missed entry, which the standing
  section of its file carries; it is named in the instance only as a fact of the session.

Text to add to `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`, as
a paragraph at the end of `### What`:

```markdown
Three more, from the retrospective files of three slices of this repository of 2026-11-14, on
0.10.0 builds: the slice that gave each Component's head its arguments, and the slice that moved
the conformance check before the harvest, each met four installed copies reported as differing
while their sources' edits sat unstaged; the first committed past the finding and judged the
commit with `commits`; in the slice that planned the splits, a repair agent committing one installed-text fix
apart from the others checked that commit's tree in a scratch worktree, and the slice's record
searcher had graded this entry `unrelated` to the work.
```

Default: the three instances are added to the recorded issue.
Outcome: decided by the session: open an issue, as three instances added to
`issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`. Ground: the
issue's summary states the defect each file met, and its closing work, a comparison with the
staged shipped text, is unchanged by them; `skill@knowledge-architect-retrospective-intake@what-to-establish`
item c sends a recorded finding to its entry.

## touched-head-defined-by-harvest: a touched head is defined by what the harvest changes, not by the plan's work

Members: 2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow W2; the first note
for the intake of 2026-11-14-knowledge-architect-conformance-before-harvest-slice bears on it.

- **a. What it says.** `skill@knowledge-architect-planning@cutting-steps-and-slices`: "A head the
  harvest touches is one whose text it changes without reversing or rewriting its decision". In
  the slice, a step of the work brought twelve heads to a new rule, and the harvest wrote one new
  head and the changelog. Read literally, the twelve heads were outside the list. The session read
  the definition by its stated reason, "so that the harvest writes into heads that already conform,
  and the review before the merge judges the harvest's change rather than a head's older defects",
  and kept them in. Proposal: "a head the plan's work or its harvest touches" (a rewording). The
  first note for the intake of the other file reports the same list from its side: the spec of
  slice 4, this slice, lists twelve root heads its harvest "touches without reversing them".
- **b. Whether it still applies.** Yes. At the file's version, cac87723 (`v0.10.0-43`), line 421 of
  `path@agent-skills@content/skills/planning/SKILL.md` holds the definition; on main it is
  unchanged, on the same line.
- **c. Whether it is already recorded.** No issue. Two heads state the list by the harvest:
  `design@agent-skills@pre-harvest-conformance` ("the heads its harvest will touch without
  reversing or rewriting their decision", `conflicts`, `check-reversal`) and
  `design@agent-skills@scoped-auditor-on-touched-heads` ("Once a plan document lists the heads its
  harvest will touch", `conflicts`, `check-reversal`). A third is wider:
  `design@agent-skills@existing-heads-on-touch` says "A head that a change touches is brought [...]
  to the rules", and "in planned work, by a step before the harvest", which covers a step's touch;
  the list and its auditor do not. `rejected@agent-skills@conformance-in-the-harvest` (`respect`)
  needs any widening to keep conformance out of the harvest. `tripwire@agent-skills@touched-heads-missed-by-the-list`
  does not fire on this instance: the heads were in the list, and the harvest named none as missed.
- **d. Its kind.** A defect of the text, missing: the definition's words admit one act, the heads
  whose text the harvest changes, and they leave out the heads a step of the work writes, which the
  review before the merge then reads with their older defects, the failure the definition's own
  reason names. The session filled the gap from the reason. Provable by reading.
- **e. Its scope.** The installed text and two heads of agent-skills. The fix widens the statement
  of `design@agent-skills@pre-harvest-conformance` and of
  `design@agent-skills@scoped-auditor-on-touched-heads`: it contradicts a statement of each, so it
  goes to `skill@knowledge-architect-design` under the backstop of `primer@design-heads`, before
  any text is written.
- **f. A better fix.** Rivals: the report's rewording; no change, since the session read the
  definition by its reason and the reason sits beside it; and a narrower rewording that counts
  only a step's edit of an existing head's text. The fact that decides between them is not
  settled by the record: in a slice whose work is the record itself, the step that writes the heads
  can be the conforming step, and whether a step that brings heads to a new rule must first be
  preceded by a conformance of their older defects, or is that conformance, is a question no head
  answers. `design@agent-skills@existing-heads-on-touch` supports widening; the owner's report under
  `design@agent-skills@pre-harvest-conformance` speaks of the harvest only.
- **g. Its route.** A decision on two heads: an issue, for a design discussion on its own branch.
- **h. The proposal.** Open an issue in agent-skills, kind `design`, slug
  `issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`. Instance: this slice's
  twelve heads, reported by both files. Mechanism: the definition names the
  harvest, while a project whose product is text writes heads in its steps. Re-entry point: the
  planning of the next plan document whose steps write into existing heads, at the moment it writes
  its list. It names the two heads above as strained.

Default: open the issue above.
Outcome: decided by the session: open an issue, agent-skills, kind `design`,
`issue@agent-skills@a-touched-head-is-defined-by-the-harvest-and-not-by-the-plan-s-steps`. Ground: the fix contradicts
statements of `design@agent-skills@pre-harvest-conformance` and
`design@agent-skills@scoped-auditor-on-touched-heads`, and the shape for a step that is itself the
conformance is not settled by the record, so the work needs a design discussion.

## standing-2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow, lines 94 to 145. One
answer takes a section of its own below, the reasons that follow one pattern. Its counts are in the
lines for `issue@agent-skills@the-review-cost-counts-are-not-compared-yet`; its tool reports are
item 6 of the owner item in the analysis of
2026-11-12-knowledge-architect-finding-repairs-klarch-workflow.

Carried by a subject, named only:

- three claims, "no word of the owner named a head's members", the crates depending on one crate
  given as one where three do, and a file count of 12 where the command gives 15 (lines 99 to 107):
  `message-claims-written-unchecked`;
- five rewords by a filter script, and the searchers' quotation-repair scripts of C3 (lines 112 to
  116): `no-command-rewords-branch-messages` and `quotation-check-refuses-cut-punctuation`;
- "The harness refused seven dispatches at the cap (W1)" (lines 127 and 128):
  `searcher-dispatch-exceeds-harness-cap`;
- a searcher's reading of the decision on the home of rules on heads, accepted and proved wrong at
  the review (W4), and the entry "returned with a reading that hid its conflict" (lines 129 to 134):
  `head-rule-home-reads-two-ways`.

Analysed here:

- **Backticks** (lines 109 and 110): "A path the tree no longer holds took" the form for a path the tree does not hold, `path@elsewhere@<path>`. Kind:
  an observation of the form `primer@where-knowledge-goes` gives.
- **Work an agent could have carried** (lines 124 to 126): the twelve heads' first paragraphs written
  by the session "as the decisions of the slice; a review then found two criteria unreasoned in all
  twelve, repaired by an agent". Kind: the dispatch judgement of `primer@dispatching-work`, the
  decisions kept by the session; an observation.
- **The auditor proposed sending two edits to the owner; the session verified both against the
  record and routed them as the record settled them** (lines 130 to 132). Kind: an observation of
  `primer@question-authority`, which has the session answer what the record settles.
- **Installed files changed**: the sources of the component-creation skill, two record agents, the
  project-audit skill and the shape-rules fragment, as the work. **Project skill addition**: none.
  **Missed entry**: none missed. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: observations of rules and judgements working as
the record decides.

## standing-2026-11-14-knowledge-architect-component-arguments-slice-klarch-workflow: reasons on each criterion that follow one pattern

Members: the answer on a rule about code, tests or splitting, lines 119 to 123: "The new rule, which
asks a reason on each criterion, produced at its first use, for 'has its own tests' and 'takes
decisions of one domain', reasons that follow one pattern across twelve heads. That is the
box-ticking its premortem named, though each line names a specific consequence. Nothing was too
strict to follow; the risk is the rule's watch point." With lines 124 to 126: "a review then found
two criteria unreasoned in all twelve, repaired by an agent". The searcher of
`design@agent-skills@component-head-reasons-over-each-criterion` raised it under "Met outside the
task".

- **What the record holds.** The rule is `design@agent-skills@component-head-reasons-over-each-criterion`:
  "A criterion named with no reason, or a fact stated with no reason it pays, is not reasoned over."
  The criteria themselves are `design@agent-skills@component-criteria-are-guidelines`, "corrected by
  what the retrospective's question on a rule about splitting that proved too hard or too strict
  brings back", the watch point of `design@agent-skills@premortem-as-watch-points`. The premortem
  cause the answer recalls was judged at that slice's harvest, 0f841082 on main: "**P7**, 'the one
  line per criterion turns into a box ticked met with no reason' ... fails condition 1, since the
  decision-record reviewer and the design-record auditor now judge each such head by step 3, which
  says a criterion named with no reason is not reasoned over, so the failure is reported by a review
  or an audit, not silent. Dropped."
- **Whether the answer meets either.** The answer to the watch point is "Nothing was too strict to
  follow": no correction of the criteria is asked. On P7's ground, the same session shows the
  premise held: a review found the two criteria unreasoned in all twelve heads, and the repair
  reasoned them. Reasons that share one pattern, "though each line names a specific consequence", are
  not reasons absent; whether a pattern is a fault is the reviewers' reading of step 3, case by case.
- **Kind.** An observation: one use of a new rule, reviewed and repaired, with a risk named for a
  later use.

Default: no change; the answer stays in this analysis as the first reading of the watch point.
Outcome: decided by the session: no change. Ground: the watch point of
`design@agent-skills@component-criteria-are-guidelines` asks for a rule too hard or too strict, and
the answer reports none; the cause dropped as P7 at 0f841082 was dropped on the reviews reporting
it, and in this session a review did.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## searcher-dispatch-exceeds-harness-cap: the record searcher's dispatch asks for every agent at once, above the harness's cap

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `searcher-dispatch-exceeds-harness-cap`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## criterion-evidence-in-no-record: a criterion admitted without condition 2 has its evidence in no record at the leaving

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-component-arguments-slice.md`, in its section on `criterion-evidence-in-no-record`, which gives the outcome.

## head-rule-home-reads-two-ways: the decision on one home for head rules reads two ways on who points and who hosts

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-14-knowledge-architect-component-arguments-slice.md`, in its section on `head-rule-home-reads-two-ways`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.
