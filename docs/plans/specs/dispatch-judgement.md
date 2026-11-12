# The primer states how a session divides the work between itself and the agents it dispatches, as a judgement, and one independent verification is sufficient for a claim the work acts on

## Status and audience

This spec is the plan of the work that answers
`the-installed-text-has-the-main-session-carry-work-it-could-dispatch`, an issue of agent-skills that the commit adding this spec closes. It adds
to the primer a section on dividing work between a session and the agents it dispatches, a pointer
to it where implementation starts, three standing questions of the retrospective, and the design heads
that argue them, and it opens two `deferred` issues for what the discussion parked.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- One default was put to the owner, D1, under "Defaults awaiting the owner", and the owner has
  ruled on it.
- The spec and its work go on one branch: the work changes no rule of the checker, and
  `cargo klarch commits` compares no installed file, so no earlier commit can fail under the
  branch tip's checker and the spec need not merge first, per
  `skill@knowledge-architect-planning@plan-reviews`.
- It was assembled from one transcript. The discussion begins at the owner's message that opens "I
  want to discuss the goal and issue of having the main session focus on design", and ends at the
  owner's message "Good, I have nothing more to add or contest." It holds 8 owner messages, called
  rounds 1 to 8 below; round 7 was sent in two parts, the first interrupted. The agent's reply to
  round N carries the heading "Round N" for N from 1 to 7, the reply to round 4 coming after the
  record search. Find the session by that opening message, not by a file name.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, in the form its last paragraph gives the work
of a spec. The design audit of point 2 is skipped while the work starts in the session where the
discussion converged and no other commit has landed on main since this spec was written.

## Names

- **the primer**: the source `path@agent-skills@content/PRIMER.md`, installed under .claude by the install.
- **the planning skill**, **the retrospective skill**: the sources
  `path@agent-skills@content/skills/planning/SKILL.md` and
  `path@agent-skills@content/skills/retrospective/SKILL.md`.
- **a session**: the main session or any agent; "the session" alone is the one that dispatches,
  and "the main session" the one the owner exchanges with.
- **a carrier**: the session or an agent that carries a piece of work.
- **a piece**: what a brief gives one agent, or what the session keeps for itself.
- **an independent verification**: the verification of a claim by a session other than the one
  that wrote it, against the evidence its report carries. A command that checks a report's form,
  such as `worklist --verify`, is none. The word "check" alone is not used for it: it names the
  checker's commands and many acts of the installed text, and "verification" alone names the
  writer's own verification as well, as in `primer@verify-before-relying`.
- **the thaum process**: the owner's report, round 3, of a project of the owner's, thaum, whose
  code modules were split into smaller units across the whole project by agents dispatched in
  parallel, one per Component, which made no commits; the main session committed each part after
  the agents returned.
- **the bullets**: the five bullets of the proposed section, named by their bold words, as
  "Decided design" says.
- **the source issue**: `the-installed-text-has-the-main-session-carry-work-it-could-dispatch`,
  which the commit adding this spec closes; read it with `git log --diff-filter=D --name-only --
  crates/agent-skills/docs/open-issues/` and `git show <commit>^:<path>`.
- **the tripwire standard**: the four conditions a cause meets to become a tripwire, held by
  `skill@knowledge-architect-planning@acceptance-criteria`.
- **L<n>**, in an argument's key words: the number of a line of the transcript extraction made by
  `cargo klarch transcript show`, from the message where the discussion begins.
- **the harness's default**: the Agent tool description of Claude Code, read in this session: "Do
  the work yourself when it is a handful of tool calls … When in doubt, don't spawn."
- **install**: `cargo klarch install-agent-skills`, which writes the installed copies under .claude.

## What the work is

Today:

- `goal@session-orchestrates-the-work` has the main session orchestrate agents that carry the work;
  it says neither when a dispatch happens nor for which kind or size of work.
- The primer mentions subagents only in `primer@met-outside-the-task` and `primer@decided-section`:
  a subagent reports those sections, and the dispatcher carries them. No text states when a dispatch
  is worth it.
- The installed skills dispatch absolutely at named steps: the record search, the review axes before
  a merge and at a development checkpoint, the plan reviews, the goal reviewer, the audits and their
  saved workflows, and the transcript extraction at a plan's assembly.
- The installed text checks a returned claim in several places, each with its own words: the review
  skill's dispatcher "Verify each against the tree before relaying or acting on it", the planning
  skill's "Check each finding against the tree, or against the transcript", the record search's
  verdicts "checked as it checks its own claims", per
  `design@agent-skills@session-acts-on-the-verdicts`, and the audit's verifiers with the session's
  verification after them, per `design@agent-skills@one-run-pass` and
  `design@agent-skills@verifier-in-the-run`; the record searcher's own "You check your conclusions
  as the session that sent you checks its own claims"
  (`path@agent-skills@content/agents/record-searcher.md`); and the intake, whose session
  "re-checks every verdict a proposal rests on" (`skill@knowledge-architect-retrospective-intake@intake-subagents`).
- The retrospective skill's standing questions ask nothing about dispatch.
- The owner's real-use report (round 2): the main session nearly never dispatched where it would
  have helped, and spent context the design discussion needed; in the session that drafted the goal,
  the leads were applied, and the session ended with spare context and the work carried well; since,
  dispatch happens occasionally, not reliably.
- Measured in Claude Code, this session, one run each: a general-purpose subagent holds `Agent` and
  can dispatch a nested agent, and holds no `Workflow`; the primer reaches a subagent of a subagent,
  which quoted the first sentence of `primer@room-to-judge` verbatim without reading a file.

Outside the work:

- how parallel work is partitioned among agents: a shared tree or a worktree per agent, who commits,
  how the parts are merged. A later design discussion, behind the `deferred` issue this work opens,
  on the owner's word (round 3);
- report files that an agent writes as it works and a verifier annotates: a `deferred` issue this
  work opens, since no real session has shown the gap;
- the owner's user-level instructions, outside the project, which the owner aligns with this
  section after the work (round 2);
- the cost a review adds, which `issue@agent-skills@the-retrospective-counts-no-review-cost` holds;
  the source issue's lead on whether checking raised the total cost is refused, under "Losing
  alternatives";
- how often the work is reviewed, which the source issue's leads leave to judgement where no
  absolute criterion applies: the development skill's checkpoints and the review before a merge
  already state it, per `design@agent-skills@review-cadence`, and the section's judgement covers
  the rest.

## What is already decided

The decisions the design rests on and does not argue again:

- `goal@only-needed-instructions`, `goal@session-orchestrates-the-work`,
  `goal@the-workflow-improves-through-real-use`.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@statement-names-an-act`,
  `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@primer-limit`,
  `design@agent-skills@verification-in-primer`, `design@agent-skills@a-scratch-directory-per-subagent`,
  `design@agent-skills@step-is-no-commit`, `design@agent-skills@plan-written-in-the-converging-session`,
  `design@agent-skills@restatement-size-test`.

The recorded decisions the work rewrites, each with the texts that `cargo klarch show` lists as
referencing it, judged at the harvest (step 3):

- `design@agent-skills@session-acts-on-the-verdicts`: its rule on checking a verdict becomes an
  instance of the new verification head, which it cites instead of stating the rule; its title and its
  other clauses stand. Referenced by `path@agent-skills@docs/design/record-search.md`,
  `path@agent-skills@docs/design/reviews.md` and
  `issue@agent-skills@a-lighter-pass-for-untouched-components`, each for a clause that stands; the
  installed searcher's description restates the rule, and its body gains a `%%` line citing the
  verification head.
- `design@agent-skills@verification-in-primer`: gains the new section as the place where the check
  of a returned conclusion lives. Referenced by `path@agent-skills@docs/design/project-configuration.md`
  alone.
- `design@agent-skills@primer-limit`: gains the new section as a member. Referenced by ten files,
  which `cargo klarch show` lists, each for the head's rule, which stands.
- `design@agent-skills@premortem-as-watch-points`: its list of the standing questions gains the three
  new questions, each naming what it watches. Referenced by seven files, the retrospective skill's
  `%%` line among them, each for the rule, which stands.
- `design@agent-skills@a-scratch-directory-per-subagent`: names the primer section among the texts
  that state its rule. Referenced by
  `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`, for the
  rule, which stands.
- `design@agent-skills@admission-rules-home`: its one section gains the sentence on fixed dispatches.
  Referenced by thirteen files, its own home included, each for the rule that the section is the one
  home, which stands.
- `design@agent-skills@one-run-pass`, `design@agent-skills@verifier-in-the-run`,
  `design@agent-skills@workflow-axis-lenses`, `design@agent-skills@repair-proposals-audited` and
  `design@agent-skills@verdicts-reach-the-conformance-review`: each gains a citation of the check
  head where it states its verification; its rule stands.

Step 3 reads each referencing text named above and updates any that restates a changed part.

The record search ran on the draft of this spec, 18 agents, every report passing
`cargo klarch worklist --verify`. The verdicts that shaped the design, by outcome, in the table;
every verdict other than `unrelated`, with the searcher's reason and its outcome, in the list after
it:

| entry | verdict, action | outcome |
| --- | --- | --- |
| `design@agent-skills@verification-in-primer` | conflicts, check-reversal | answered: the verification paragraph says one independent verification stands for the dispatching session's own verification, which resolves the two acts "Re-verify a conclusion before acting on it" gives on a verifier's report; the head is not reversed, and gains the section as the place of that rule |
| `design@agent-skills@session-acts-on-the-verdicts` | conflicts, check-reversal | rewritten at the harvest to cite the verification head |
| `design@agent-skills@one-run-pass`, `design@agent-skills@verifier-in-the-run`, `design@agent-skills@workflow-axis-lenses`, `design@agent-skills@repair-proposals-audited`, `design@agent-skills@verdicts-reach-the-conformance-review` | conflicts, check-reversal | answered: the check is a minimum, "one check is enough", which forbids no second check a skill asks for; none is reversed |
| `rejected@agent-skills@repair-check-left-to-session`, `rejected@agent-skills@repair-defects-left-to-next-pass` | brings-back, new-argument-needed | answered: the minimum drops no check step, so neither comes back |
| `design@agent-skills@premortem-as-watch-points` | conflicts, check-reversal | the questions widen its list within its argument: each names what it watches; the compaction question is dropped, on the argument under "Losing alternatives" |
| `the-installed-text-has-the-main-session-carry-work-it-could-dispatch` | closes, close-with-the-work | closed by the commit that adds this spec: each departure is built or refused on its argument, the cost lead refused under "Losing alternatives" |
| `rejected@agent-skills@head-rules-whole-in-primer` | constrains, respect | the primer "can dilute in a long one", a long session: answered by the pointer sentence at point 3 of the planning skill's procedure, where implementation starts |
| `design@agent-skills@a-goal-reading-is-reported` | constrains, respect | the reading of `goal@session-orchestrates-the-work` is named under "Decided" (`thread@dispatch-judgement@dispatch-judgement-in-primer`) |
| `design@agent-skills@one-decision-per-head`, `rejected@agent-skills@head-split-by-independent-reversal` | constrains, respect | the harvest writes one head per decision; the exception of `thread@dispatch-judgement@absolute-dispatch-conditions` stays in the judgement's head |
| `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@new-or-reshaped-head-needs-design` | constrains, respect | the widening of the verification rule was argued in this discussion, rounds 4 to 7 |
| `design@agent-skills@no-two-installed-instructions-contradict` | constrains, respect | the minimum form leaves every named chain of verifications standing |
| `issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness` | touches, respect | the judgement head states the section as a weighing beside the harness's default, not a precedence over it, so it does not depend on that issue |
| `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies` | touches, respect | the judgement head states where a fixed dispatch is admitted; the issue stays open |
| `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review` | touches | the two parked threads take `deferred` issues with a trigger |
| `issue@agent-skills@the-retrospective-counts-no-review-cost` | touches | the cost question is left to it |
| `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up` | touches | it leaves to a design discussion a question the owner's consideration places in the retrospective; the new questions take none of its subject, so either can land first |
| `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`, `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` | touches | cited by the partition issue |
| `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`, `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own` | touches | cited by the report-file issue |
| `issue@agent-skills@agent-skills-heads-owing-an-argument-on-one-branch` | touches, respect | through its member on the harness, answered as above |
| `design@knowledge-architect@changelog-entries`, `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@the-branch-writes-its-changelog-entries` | constrains, respect | the branch writes a Workflow entry in `Next release`, a patch |
| `design@query@worklist-bound-is-40000-words` | constrains, respect | the owner's earlier report of reliable agents up to about 200,000 to 250,000 tokens stays the search's bound's ground; no number goes in the primer |
| `rejected@query@one-search-agent-per-anchor` | constrains, respect | its measured start of about 36,000 tokens per agent is the ground of "a fresh agent first reloads its instructions" |
| `design@model@section-homes-carry-slugs`, `design@model@installed-entities-from-the-tree` | constrains, respect | the section's heading carries `{{slug:dispatching-work}}` in the source |
| `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@shipped-text-line-comments`, `design@agent-skills@installed-text-works-with-what-an-installing-project-holds` | constrains, respect | the reasons go in `%%` lines; the shipped text cites no entry |
| `design@check@orphan-rejected-entry` | constrains, respect | each `rejected` entry the harvest writes is cited by its head |
| `rejected@mutate@worktree-holds-the-mutation`, `design@knowledge-architect@no-operation-loses-content`, `design@knowledge-architect@scratch-worktree-outside-git-rules`, `design@knowledge-architect@git-flow` | constrains, respect | carried into the partition issue |
| `rejected@agent-skills@ledger-written-during-discussion`, `design@agent-skills@refused-report-returned-as-text` | constrains, respect | answered in the report-file issue, as its harvest row says |
| `design@agent-skills@admission-rules-home`, `rejected@agent-skills@admission-standard-in-primer` | constrains, respect | the rule on fixed dispatches goes in the admission standard, its one home; the primer holds no rule on what enters agent-facing text |
| `design@agent-skills@term-defined-at-first-load` | constrains, respect | "carrier" is defined in the section itself, so it takes no line in `primer@coined-terms` |
| `design@query@worklist-writes-and-verifies-the-reports`, `design@query@worklist-check-judges-one-report` | constrains, respect | the record search's fixed dispatch rests on an output whose form a command verifies and whose claims the session checks; a command is no check of a claim |
| `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument` | touches, respect | each member the harvest adds is within its head's argument, as the rows say |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | touches, respect | the causes on how sessions dispatch are watched by the retrospective questions, not by tripwires |

The 188 lines of `generated-lines.md` that the search wrote are carried in the message of the commit
that adds this spec. Every verdict other than `unrelated`, as the searcher graded it; its outcome is
the table's where the table names the entry, and otherwise that the decided design respects it:

- `design@query@worklist-writes-and-verifies-the-reports`: constrains, respect. The work admits an absolute dispatch only for a step needing a separate point of view or whose output a check verifies; the record search's absolute dispatch qualifies through this check, so the condition's wording must keep reading `--verify` (coverage, sets, quotations) as such a check, and the agent-skills head may cite this decision as an instance.
- `design@query@worklist-check-judges-one-report`: constrains, respect. The draft rule says a returned conclusion is checked once and a check is not checked again, while here the searcher runs `--check` on its report and the session then runs `--verify` over the same report; the rule's wording must confine itself to the session's judgement of conclusions, or exempt a command's mechanical check, so it does not read as forbidding this pair.
- `design@query@occasion-shaped-output`: constrains, none. It serves the goal the work serves by giving a session one verdict file per subject to put in an agent's brief; handing a subject to an agent falls under the work's dispatch judgement, and nothing in the work limits or changes this output.
- `design@query@clusters-packed-under-a-word-bound`: constrains, respect. The work rejects a size threshold for the session's dispatch judgement, while this head fixes a word bound per search agent by command; the losing alternative's argument (size varies without bound, a threshold per interaction) must stay scoped to the session's judgement and the named dispatches, so it does not read as an argument against this bound.
- `design@query@worklist-bound-is-40000-words`: constrains, respect. This records the owner's earlier report that reliability holds to 200,000 to 250,000 tokens and token use grows from about 250,000, the measure being cost, with a re-take command over each subagent's transcript; the work's hint of about 300k per agent differs from it, so any figure or question the retrospective carries on a piece too large must agree with this report or say why it departs, and can reuse its measurement command.
- `rejected@query@one-search-agent-per-anchor`: constrains, respect. Its measurement, an agent's fixed start of about 36,000 tokens over 40 searchers' transcripts, is the ground of the work's carry clause that a brief can cost about as much as the work since a fresh agent first reloads its instructions; the work does not bring back one agent per anchor, and its head can cite this measurement rather than assert the cost.
- `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`: touches, respect. Its subject is how a dispatched agent writes its report file, which the parked issue on report files written as the agent works also covers, so that new issue must state how it differs from this one or cite it; and its instances show a session writing a part of a dispatched piece, the report, when the harness refuses the agent's edit, which the rule that the session takes over no part once dispatched must admit or name as the agent's failure to carry it.
- `design@transcript@subagent-file-attached-to-its-dispatcher`: constrains, none. The work leads to more dispatches, nested ones included (a dispatched developer sends its checkpoint axes itself); the rule already attaches a subagent file by the call found in the top-level file or another subagent file, so nothing in the work is limited by it.
- `rejected@mutate@worktree-holds-the-mutation`: constrains, respect. The issue the work opens on partitioning parallel work names worktrees per agent as a shape; this measured cold-build cost of a fresh worktree limits that shape, and the issue should carry it, without bringing back a worktree for a mutation.
- `design@agent-skills@primer-limit`: constrains, respect. Is a seed; the new primer section is a member the head must admit: the draft argues that every session that can dispatch meets the judgement (subagents included, measured to hold `Agent`) and no skill loads at that moment, and the head's member list gains it.
- `design@agent-skills@verification-in-primer`: conflicts, check-reversal. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@primer-limit`; the work's "a check is not checked again" lets the session act on a second agent's check of a returned conclusion with no verification it observed, which narrows the primer's re-verification and "establish it from the implementation, or from a run you observe" for a dispatched check, so the draft must argue that narrowing against this head.
- `design@agent-skills@code-change-reads-its-record`: constrains, respect. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@primer-limit`; a dispatched developer proposes and makes code changes, so it reads the record of that code itself, the brief defining the piece and never replacing that reading.
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: constrains, none. Cites seed: `design@agent-skills@primer-limit`; nothing in the work acts on a ruling of the owner past its premise or on an answer about a reading.
- `design@agent-skills@met-outside-the-task-outcomes`: constrains, respect. The new section on dispatch must keep this route: a dispatched agent reports what it met outside its piece, and an agent that carries a whole piece and edits, as a dispatched developer, routes its items by the primer's table itself.
- `design@agent-skills@decisions-reported-to-the-owner`: constrains, respect. A returned conclusion checked once under the work still carries the subagent's Decided lines into the session's own, and the check of a conclusion leaves that carrying in place.
- `design@agent-skills@a-goal-reading-is-reported`: constrains, respect. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`; `goal@session-orchestrates-the-work` says it does not fix when a dispatch happens nor for which kind or size of work, and the work closes that question by judgement with no threshold, so its Decided line names that reading and the rival threshold it defeats.
- `design@agent-skills@one-home-for-head-rules`: constrains, respect. Cites seed: `design@agent-skills@primer-limit`; the work delivers the dispatch judgement by the primer alone, met at many moments of long sessions, so the draft's argument against guidance per skill must answer this mechanism, which this head measured for another primer rule.
- `design@agent-skills@restatement-size-test`: constrains, respect. The primer's "checked once, as the session checks its own claims" generalises the rule that the record search and the review texts state; each such skill keeps at most one sentence of it with its pointer, and the primer's mention of the skills' named dispatches restates none of them.
- `design@agent-skills@context-cost-of-agent-text`: constrains, respect. Cites seed: `design@agent-skills@primer-limit`; the new primer section and the retrospective questions are weighed by this rule: the section costs only a session that never weighs a dispatch, and a pointer from a skill would cost the lookup the rule counts.
- `design@agent-skills@no-routing-table`: constrains, none. Cites seed: `design@agent-skills@capabilities-not-structure`; the work adds no table mapping skills, nor a mapping of kinds of work to dispatches.
- `design@agent-skills@installed-skill-takes-precedence`: constrains, none. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`; the work weighs the harness's Agent tool default, not a skill from outside the installed set, and changes no skill loading.
- `design@agent-skills@staged-check-before-each-commit`: constrains, none. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@primer-limit`; a dispatched agent that commits holds the primer and this rule as the session does, and nothing in the work changes it.
- `design@agent-skills@staged-forms-for-a-partial-commit`: constrains, none. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`; the work changes neither the issue-tracking nor the planning skill's commands.
- `rejected@agent-skills@head-rules-whole-in-primer`: constrains, respect. The work puts the dispatch judgement whole in the primer, applied at moments deep in long sessions; it does not bring back this design about heads, but its reason limits a primer-only delivery, which the draft argues against per-skill guidance without answering.
- `design@agent-skills@audit-method`: constrains, respect. The primer's check-once rule will reach the audit's dispatches too, and must keep this rule that the session reads what an agent proposes before it is applied.
- `design@agent-skills@one-run-pass`: conflicts, check-reversal. The session's verification checks findings the run's verifiers already checked, and samples their refuted drafts, so it checks a check; the draft primer sentence "a check is not checked again" contradicts it unless its scope excludes the dispatches a skill names, which the draft limits only for size and kind.
- `design@agent-skills@refused-report-returned-as-text`: constrains, respect. The issue the work opens on report files written as an agent works must reckon with the harness's refusal of a subagent's report file, and with the fallback to text this head and the record searcher take.
- `design@agent-skills@apply-stays-with-the-session`: constrains, respect. This is a built case of agents applying parts on disjoint files with the session reading the diff before each commit; the carried-whole rule must not forbid it (the draft's losing alternative already keeps it) and the parallel-partition issue starts from it.
- `design@agent-skills@audit-axis-runs-a-workflow`: constrains, respect. An audit axis dispatches absolutely through its saved workflow; the absolute-dispatch condition, a point of view apart or an output a check verifies, must keep admitting these dispatches, and the work's measurement that a subagent holds no Workflow means only the main session runs them.
- `design@agent-skills@audit-outcomes`: constrains, none. Cites seed `design@agent-skills@behaviour-additions-need-real-use`; it routes an audit's findings, and the work is no audit, so nothing in the work meets it.
- `design@agent-skills@verifier-in-the-run`: conflicts, check-reversal. A verifier's check of each draft is followed by the session's own verification, a second check of a checked conclusion, which the draft's "a check is not checked again" contradicts unless the sentence leaves out a skill's named dispatches; its three tripwires watch exactly that second check.
- `design@agent-skills@audit-triggers`: constrains, none. Cites seed `design@agent-skills@behaviour-additions-need-real-use`; it fixes the occasions of an audit, and the work adds no audit occasion.
- `design@agent-skills@audit-agent-per-axis`: constrains, respect. Its argument, a standard behind a read the brief must ask for comes out uneven, bears on a dispatched developer that is to load the development skill; the shape holds only where the primer's skill list, which reaches every subagent, triggers the load and not the brief alone.
- `rejected@agent-skills@session-rereads-every-draft`: constrains, respect. The check-once rule must not lead the session to read again every conclusion one checker already judged; the work does not bring this design back, and its reason supports the draft's single check.
- `rejected@agent-skills@three-runs-per-pass`: constrains, respect. Its measurement is the context cost the dispatch judgement weighs; a guideline that has the main session read a dispatched step's intermediate outputs brings back the cost this entry measured.
- `design@agent-skills@no-installed-rule-on-when-an-intermediate-component-exists`: constrains, none. Cites seed `design@agent-skills@behaviour-additions-need-real-use`; it keeps intermediate Components out of the installed text, which the work does not touch.
- `design@agent-skills@enclosing-design-read-at-grounding`: constrains, respect. Cites seed `design@agent-skills@primer-limit`; it is the argument a primer member owes against a clause in each skill, the same rival the work's losing alternative "guidance in each activity's skill" names, and the new section joins the members `design@agent-skills@primer-limit` lists.
- `design@agent-skills@settled-means-recorded`: constrains, none. Cites seed `design@agent-skills@primer-limit`; the work, the owner's proposal of dispatch guidance, already goes through the design skill's grounding and record search, so nothing in it meets the trigger's exceptions.
- `design@agent-skills@structure-the-flow`: constrains, none. Cites seed `design@agent-skills@capabilities-not-structure`; the head bounds the design skill's discussion flow, which the work does not edit; its token-bound hint is per dispatched agent, not on a discussion's size.
- `design@agent-skills@new-or-reshaped-head-needs-design`: constrains, respect. The work creates an agent-skills head and generalises `design@agent-skills@session-acts-on-the-verdicts` to every returned conclusion, a member beyond that head's argument; both must be argued in this discussion, not recorded directly at the harvest, and the added retrospective question is routed by `design@agent-skills@premortem-as-watch-points`' ground.
- `design@agent-skills@title-states-the-rule`: constrains, respect. The new head's title must state the rule (absolute dispatch only for a separate point of view, a checked output or an anticipable occasion), not the list of today's named dispatches, and no wider than its argument.
- `design@agent-skills@one-decision-per-head`: constrains, respect. The draft plans one head for the dispatch judgement, the absolute-dispatch condition, the piece carried whole and the single check, which lose to different rivals listed in Losing alternatives (no text, thresholds, split authorship, a chain of verifiers): each part passing an entry test takes a head of its own, the exception of absolute dispatches staying with its rule.
- `design@agent-skills@extension-follows-the-ground`: constrains, respect. The work adds a member to `design@agent-skills@primer-limit`'s list, a standing question to `design@agent-skills@premortem-as-watch-points`, and widens `design@agent-skills@session-acts-on-the-verdicts` to every returned conclusion; each is routed by its head's ground, and the widening, beyond that head's record-search argument, is a change argued here.
- `design@agent-skills@existing-heads-on-touch`: constrains, respect. The work's harvest edits `design@agent-skills@primer-limit`, `design@agent-skills@premortem-as-watch-points` and `design@agent-skills@session-acts-on-the-verdicts`, so each touched head is brought to the rules on heads in the same change.
- `design@agent-skills@design-home-write-loads-recording`: constrains, respect. The work writes a new agent-skills head and edits the primer and the retrospective skill, texts heads describe, so the decision-recording skill is loaded before those writes.
- `rejected@agent-skills@head-split-by-independent-reversal`: constrains, respect. When the dispatch head is split per `design@agent-skills@one-decision-per-head`, the absolute-dispatch exception stays with the `spec@plans@dispatch-judgement` rule; splitting by independent reversal would separate them.
- `rejected@agent-skills@title-states-built-instance`: constrains, respect. The absolute-dispatch head must not title itself with today's dispatches (record search, review axes, development checkpoint, audit, goal reviewer, transcript extraction) but with the property that admits them.
- `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`: touches, respect. The work adds members to `design@agent-skills@primer-limit` and `design@agent-skills@premortem-as-watch-points` and widens `design@agent-skills@session-acts-on-the-verdicts`; each addition is the act this issue records going wrong, so each is checked against its head's argument and the widening argued, not recorded directly.
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: touches, respect. Whether sessions dispatch as the new primer section asks is agent behaviour seen only in sessions; the work's retrospective standing questions are the channel this issue names, so the premortem's behaviour causes go there, not into tripwires whose evidence no reading occasion holds.
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: touches, respect. The work opens two issues, `thread@dispatch-judgement@dispatch-report-file` pending real-use evidence and `thread@dispatch-judgement@parallel-work-partition` for a later discussion; one waiting on evidence takes kind deferred with a Trigger the standing-state reviewer reads, since a re-entry point on another kind is read by no review.
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: touches, respect. Cites seed design@agent-skills@capabilities-not-structure; the work adds to the same retrospective a question whether checking returned conclusions raised the total cost, which the issue's counts of review rounds would measure, so the two questions are written to fit together, without closing the issue.
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: constrains, respect. Each decision the work records (primer dispatch judgement, absolute-dispatch condition, piece carried whole, single check) needs its entry test named: test 3 on the harness's Agent-tool default, or test 2 for a rule several installed texts respect; one that passes none stays a %% comment at the primer line.
- `design@agent-skills@losing-alternatives-filter`: constrains, respect. Of the six losing alternatives, only those refuted by costly evidence or with a remaining doubt earn rejected entries; the primer token bound, carried by the owner's hint, is the likeliest candidate on a doubt, the rest refuted by reasoning stay in the spec.
- `design@agent-skills@losing-shape-test`: constrains, respect. The superseded `thread@dispatch-judgement@design-investigation-dispatch` thread is judged by the filter only if its distinct shape (guidance in the design skill) lost on an argument; the parked `thread@dispatch-judgement@dispatch-report-file` and `thread@dispatch-judgement@parallel-work-partition` shapes stay in the spec, their questions going to issues.
- `design@agent-skills@rejected-citation-direction`: constrains, respect. Any rejected entry the work records (a primer token bound, a chain of verifiers, guidance per skill) is cited by the new head whose argument covers it, and the checker's orphan check fails one cited by none.
- `design@agent-skills@thread-slug-is-entry-id`: constrains, respect. Each head the work records takes its thread's slug, checked for a collision first, unless the slug misdescribes the approved decision, and in discussion prose a thread is written with a hash sign.
- `rejected@agent-skills@entry-names-the-decision-it-lost-to`: constrains, respect. A rejected entry the work writes for a losing alternative states its reason and names no head it lost to, as the draft's phrasing lost to the real-use report could invite.
- `rejected@agent-skills@every-losing-thread-recorded`: constrains, respect. The draft lists six losing alternatives; recording each as a rejected entry would bring this back, so only those the filter admits are recorded.
- `design@agent-skills@development-skill`: constrains, respect. A dispatched developer loads this skill and carries its whole span, the commit and the checkpoint review axes included, which is why `thread@dispatch-judgement@development-dispatch` edits no text and why the session takes over no part of the piece.
- `design@agent-skills@goal-one-meaning-any-breadth`: constrains, respect. Goal@session-orchestrates-the-work says neither when nor for which work a dispatch happens; the work closes that open question in a design head of agent-skills, so the head must argue it against its rivals (no text, per-skill guidance, thresholds) and not by narrowing the goal.
- `design@agent-skills@what-shows-a-goal-met-is-design`: constrains, respect. The retrospective dispatch questions are the old measure of `goal@session-orchestrates-the-work` this head names; the work states them in the retrospective skill and in the agent-skills head citing the goal, never back in the goal.
- `design@agent-skills@goal-reviewer-agent`: constrains, respect. An absolute dispatch an installed skill names before every ruling; the work's condition for absolute dispatches (a point of view apart from the session's) must keep admitting it, and it does.
- `design@agent-skills@whole-set-rounds`: constrains, respect. An absolute dispatch of fresh reviewers per set that the work's absolute-dispatch condition must keep covering; and it cuts work for focus, not by a size bound, which the work's refusal of a token number keeps.
- `rejected@agent-skills@goal-measure-in-what-the-project-builds`: constrains, respect. The retrospective questions were part of the goal's old measure; the work places them in a skill and a head, which does not bring this back, and its reason keeps them out of the goal.
- `rejected@agent-skills@goal-measured-by-evidence-from-use`: constrains, respect. The retrospective questions are evidence from use of whether the dispatch judgement serves the goal; the work states them in a head and the retrospective skill, which this entry admits, not as a measure in the goal.
- `rejected@agent-skills@ledger-written-during-discussion`: constrains, respect. The parked report file written as an agent works is an artifact updated as it goes; it is no discussion ledger, so it does not bring this back, but the issue the work opens must answer this reason: the verifier's annotation is the outside corroboration the entry names.
- `issue@agent-skills@no-review-compares-the-readings-heads-take-of-one-goal`: constrains, none. Cites seed: design@agent-skills@behaviour-additions-need-real-use; nothing in the work meets it: the work adds one head reading goal@session-orchestrates-the-work and builds no comparison of readings.
- `rejected@agent-skills@one-agent-reads-every-standing-entry`: constrains, respect. The work's retrospective question on a piece too large for an agent asks this doubt in use; its reason limits the primer's guidance to pieces an agent's context holds, with no number.
- `rejected@agent-skills@subject-groups-bounded-at-sixty`: constrains, respect. The work drops the owner's 300k token hint from the primer and defines the piece by its brief, not by a size; a number in the primer that cut a coherent piece would bring this back.
- `issue@agent-skills@agent-skills-heads-owing-an-argument-on-one-branch`: touches, respect. Its member issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness covers the precedence of an installed instruction over a harness instruction and reliance on harness mechanisms; the work argues the primer section against the Agent tool's "When in doubt, don't spawn" default and relies on dispatch, a harness mechanism, so its head must state that ground or cite the member.
- `the-installed-text-has-the-main-session-carry-work-it-could-dispatch`: closes, close-with-the-work. The work builds the dispatch judgement in the primer, the report standard and the retrospective questions, and refuses on argument the dispatch of implementation in the development skill and design-skill text (the primer judgement covers both); parallel partition and the report file go to new issues.
- `design@agent-skills@design-record-axis-dispatch`: constrains, respect. An installed dispatch with a size bound per agent; the draft primer text exempts only "the dispatches an installed skill names", and this bound sits in a saved workflow the audit skill names, so the wording must keep it covered and the size refusal must not reach it.
- `design@agent-skills@calibration-in-the-run`: constrains, respect. The calibration agent's proposed reading is a returned conclusion the session checks once, as the work's report standard says; the standard's "a check is not checked again" must not read the session's confirmation as a forbidden second check.
- `design@agent-skills@created-heads-checked-in-the-run`: constrains, respect. An absolute dispatch inside the audit run whose output the session verifies; the work's condition (a checked output or a separate point of view) admits it.
- `design@agent-skills@application-judged-by-diff`: constrains, respect. An absolute dispatch that judges the session's own application, a point of view apart; its argument on one context's reach is the same the work's primer guidance gives, and the work's condition must keep admitting it.
- `design@agent-skills@one-record-searcher`: constrains, respect. The record search is an absolute dispatch the new rule must keep admitting (its output is checked by `worklist --check`), and its rule that the dispatcher never repairs a failed report is an instance the "carried whole" thread must stay consistent with; cites the seed `design@agent-skills@session-acts-on-the-verdicts`.
- `design@agent-skills@verdict-report`: constrains, respect. The primer's "conclusion checked once" rule generalises the acting on returned conclusions; checking once without redoing the work rests on a report carrying its evidence, which this head argues and the new rule must not undercut; cites the seed `design@agent-skills@session-acts-on-the-verdicts`.
- `design@agent-skills@a-seed-and-its-citers-are-never-unrelated`: constrains, none. Cites the seed `design@agent-skills@session-acts-on-the-verdicts`; the work changes neither seeds nor generated lines, so nothing in it meets this head.
- `design@agent-skills@verdict-states-a-conclusion`: constrains, respect. The generalised rule on returned conclusions covers the searcher's; it must keep a dispatched conclusion unconditional with owner-reserved questions carried back, the parity argument that also underlies checking a conclusion once.
- `design@agent-skills@session-acts-on-the-verdicts`: conflicts, check-reversal. The draft generalises this rule to every returned conclusion in the primer and adds that the check is made once, by the session or one agent whose brief carries the original brief, which narrows this head's "hands them to agents it dispatches, at its judgement"; the head is rewritten to cite or match the primer rule, not left as a second statement.
- `design@agent-skills@record-searched-before-the-work`: constrains, respect. The same argument bounds the new primer section: it must stay a judgement (capability form), not a primer line sending every session to dispatch, and its real-use evidence must meet `design@agent-skills@behaviour-additions-need-real-use`; the search's dispatch at design occasions is an absolute dispatch the new condition must still admit.
- `rejected@agent-skills@search-work-carries-the-deliberation`: constrains, respect. The dispatch rule's "a brief can carry what it needs of the session's context" must not be read as passing the discussion's transcript, which this entry rejected on cost; and the record already holds a per-searcher bound of 250,000 tokens that the size-hint thread (about 300k) meets.
- `issue@agent-skills@a-lighter-pass-for-untouched-components`: constrains, none. Cites the seed `design@agent-skills@session-acts-on-the-verdicts`; the work changes no clustering, packing or writing of the search, so its re-entry is not met.
- `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session`: touches, respect. The primer's dispatch judgement reaches the intake session's choice to hand reading to agents, and the issue the work opens on partitioning parallel work overlaps this one's "partitions its work among agents"; the new issue cites or is scoped apart from it.
- `design@agent-skills@shipped-text-line-comments`: constrains, respect. The reasons of the new primer section and retrospective questions (harness default, owner's report) and the citation of the new head go in `%%` lines beside them, never in the installed text.
- `design@agent-skills@shipped-text-cites-no-entry`: constrains, respect. The new primer section and retrospective questions are shipped text: they cite only installed skills, agents and primer sections, no entry and no harness or repository particular.
- `design@agent-skills@installed-text-works-with-what-an-installing-project-holds`: constrains, respect. The dispatch judgement is followed in every installing project, so its criteria must read complete without this repository's owner report or the harness quotation that argue it here.
- `design@agent-skills@one-skill-per-activity`: constrains, respect. A dispatched developer carries development out from the development skill, loaded whole, with no edit; and the judgement met in every activity goes in the primer rather than in each activity's skill, as the losing alternative states.
- `design@agent-skills@a-reference-claims-a-revisit`: constrains, respect. The new design head names the goals it derives from and the `rejected` entries its argument covers, and the two issues the work opens name the decision they strain and the goal they threaten.
- `design@agent-skills@plain-text-is-no-repair`: constrains, respect. The plan document and commit messages name the two issues the work is still to open by their bare slugs in backticks, not as unbacked references.
- `design@agent-skills@standing-state-review-before-every-merge`: constrains, respect. An absolute dispatch at a merge; the work's rule on absolute dispatches must keep admitting it, and the draft's "an occasion most work meets (a merge...)" does.
- `design@agent-skills@plan-read-against-the-record`: constrains, respect. An absolute dispatch at a plan document's review moments, outside a merge or a checkpoint; the work's conditions admit it only through "a point of view apart from the session's", which the wording must keep.
- `design@agent-skills@verdicts-reach-the-conformance-review`: conflicts, check-reversal. The head (and its seed `design@agent-skills@session-acts-on-the-verdicts`) has a searcher's verdicts, already checked by the session, read again by the conformance reviewer; the draft's "checked once ... a check is not checked again", generalised to every returned conclusion, contradicts that second reading unless worded to leave a review of the work outside it.
- `design@agent-skills@transcript-review-last-before-merge`: constrains, respect. An absolute merge-time dispatch the conditions must admit, and a reader of whether the other axes' findings were acted on, which the "checked once" rule must not read as a forbidden second check.
- `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`: constrains, respect. The work's "a check is not checked again" must stay compatible with the review's re-review of a head-changing repair, which reviews new work, not a returned conclusion.
- `design@agent-skills@a-scratch-directory-per-subagent`: constrains, respect. The head's rule is stated only in the review skill's invariants; the primer's general dispatch of agents together, and the parked partition design, must keep it, by a pointer or in the primer.
- `design@agent-skills@reviewer-writes-its-report`: constrains, respect. A seed; the work's report standard and the parked report-file issue build on a reviewer's report written whole to a named path, which they must not contradict.
- `design@agent-skills@report-written-through-the-shell`: constrains, respect. The parked issue on report files written as an agent works must state the harness's refusal of a report file written with `Write`, and the shell write this head relies on.
- `design@agent-skills@reviewer-returns-its-report-whole`: constrains, respect. A returned conclusion the work's standard checks must still reach the transcript as text; a report-file design that returns only a path would contradict this head.
- `design@agent-skills@reviews-judge-by-the-writer-s-criteria`: constrains, none. Cites the seed `design@agent-skills@behaviour-additions-need-real-use`; nothing in the work changes the criteria a review judges by.
- `design@agent-skills@review-judges-additions`: constrains, none. The work's primer section and retrospective questions are agent-facing text, so this axis is owed at its review; it limits no shape of the work.
- `design@agent-skills@goals-axis-in-the-review-before-a-merge`: constrains, respect. A conditional absolute dispatch at a merge, which the work's conditions must keep admitting, as the merge occasion does.
- `design@agent-skills@code-shape-axis`: constrains, respect. An absolute dispatch argued on a reader apart from the writing session, the first condition the work names; its wording must keep admitting it.
- `design@agent-skills@spec-first-axis`: constrains, respect. A checkpoint dispatch the conditions admit by the development-checkpoint occasion, and which a dispatched developer sends itself under the `thread@dispatch-judgement@development-dispatch` thread.
- `design@agent-skills@adversarial-scoping`: constrains, respect. Under `thread@dispatch-judgement@development-dispatch` the dispatcher of the adversarial reviewers becomes the dispatched developer, which then owes their scoping; the head leaves to a retrospective whether scoped reviewers run unbounded, which the new question on a piece too large must not turn into a cap.
- `design@agent-skills@review-cadence`: constrains, respect. Fixes which axes run at each checkpoint and which once per branch; the work's checkpoint occasion and the developer sending its own checkpoint axes must keep this cadence.
- `design@agent-skills@code-axes-rows-home`: constrains, respect. The checkpoint axes live in the development skill, which is why a dispatched developer that loads it sends them itself with no edit to that skill, as `thread@dispatch-judgement@development-dispatch` assumes.
- `rejected@agent-skills@one-generic-auditor-agent`: constrains, respect. The work's checking agent, briefed with the original brief, is a general-purpose agent whose brief is its only standard; the reason limits it to ad hoc checks, never a step with a fixed standard, which keeps an installed agent.
- `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript`: touches, respect. The work states how a returned conclusion is checked and parks report files; a conclusion returned through a workflow reaches the transcript only as the script's return, which the parked issue should cite.
- `design@agent-skills@workflow-axis-corpus`: constrains, respect. The head already names the evidence of a piece too large for one agent; the retrospective question on a piece too large, absorbing the size hint, must not put a bound in its place.
- `design@agent-skills@workflow-axis-lenses`: conflicts, check-reversal. An auditor's finding is checked by its verifier, then by the session; the draft's "checked once ... a check is not checked again", stated for every returned conclusion, contradicts that chain unless the named dispatches of an installed skill stay outside it, as the draft's first paragraph exempts them from the size rule only.
- `design@agent-skills@workflow-axis-finding-standard`: constrains, none. Cites the seed `design@agent-skills@behaviour-additions-need-real-use`; the work changes no finding standard of the audit.
- `design@agent-skills@workflow-auditor-agent`: constrains, respect. The audit's verify and check briefs and the session's decision are a named sequence the work's "checked once" must leave standing.
- `design@agent-skills@root-cause-repair`: constrains, respect. The audit's cluster, merge and repair agents are named dispatches whose output the session verifies; the work's conditions must keep admitting them through "an output a check verifies".
- `design@agent-skills@repair-proposals-audited`: conflicts, check-reversal. A repair proposal is checked by a check agent, then by the session's verification; the draft's "a check is not checked again" contradicts this sequence unless it is worded to exempt a skill's named steps.
- `design@agent-skills@repairs-read-against-the-record`: constrains, none. The record check binds every writer of agent-facing text, and this record search meets it for the work's primer and retrospective text; it limits no shape of the work.
- `rejected@agent-skills@one-repair-per-finding`: constrains, none. Cites the seed `design@agent-skills@capabilities-not-structure`; the work proposes no repair method.
- `rejected@agent-skills@repair-check-left-to-session`: brings-back, new-argument-needed. "checked once, by the session or by one agent", applied to an audit's repair proposals, lets the session drop the check step and check at application, the design this entry rejects.
- `rejected@agent-skills@repair-defects-left-to-next-pass`: brings-back, new-argument-needed. If "a check is not checked again" removes the check step before the session's verification, the defects of a repair's text are left to the next pass, which cost a whole pass each.
- `rejected@agent-skills@two-readings-left-to-judgement`: constrains, none. Cites the seed `design@agent-skills@capabilities-not-structure`; the work leaves nothing read two ways to judgement.
- `rejected@agent-skills@workflow-corpus-shared-out`: constrains, respect. Limits a partition of a reading among agents: a piece whose question spans its parts is not split, which the parked partition design and the primer's guidance on cutting pieces must respect.
- `design@agent-skills@instruction-record-is-minimal`: constrains, respect. The work records a design head on installed text; it must pass an entry test (the harness's default, external behaviour; absolute dispatches as a reason at several skills), and the wording of the primer section's sentences takes %% comments, not the head.
- `design@agent-skills@behaviour-additions-need-real-use`: constrains, respect. Is a seed; the owner's report and the harness-default mechanism ground the dispatch judgement, but the draft names no real-use instance for the carried-whole and checked-once paragraphs nor for the retrospective questions beyond the first.
- `design@agent-skills@no-size-budget`: constrains, none. The retrospective skill gains questions, which no size budget limits; the edit is judged on the behaviour it changes, which the work already argues.
- `design@agent-skills@synthetic-evidence-not-built`: constrains, respect. The draft's one run (a subagent holds Agent, not Workflow) is admitted harness mechanics; no run may be built to see whether sessions dispatch more under the new section or to set the token bound, so that evidence comes from retrospectives, as the work plans.
- `design@agent-skills@capabilities-not-structure`: constrains, respect. Is a seed; the section gives a capability and its judgement with no size or kind threshold, absolute dispatches limited to named needs, and the carried-whole rule kept narrower than a fixed mapping so a partition into non-committing agents stays possible.
- `design@agent-skills@expectation-set-bounds-scope`: constrains, none. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`; the dispatch questions bear on the session's behaviour, not the owner's, so no expectation set scopes them.
- `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect. Cites seed: `design@agent-skills@capabilities-not-structure`; the section's exception sentence covers the skills' named dispatches, but 'a check is not checked again' meets the audit axis where a verifier and then the session read every finding and a re-check reads again (`design@agent-skills@workflow-axis-lenses`): the section needs a stated precedence there too.
- `design@agent-skills@statement-names-an-act`: constrains, respect. Cites seed: `design@agent-skills@capabilities-not-structure`; each sentence of the new primer section and each retrospective question must name an act that differs without it, a reason only where the act needs it.
- `design@agent-skills@term-defined-at-first-load`: constrains, respect. The section uses 'brief' and 'piece', and 'dispatch'; a term coined elsewhere that the primer now uses gets a `primer@coined-terms` line, one the section coins is defined in it.
- `design@agent-skills@admission-rules-home`: constrains, respect. The condition on when an installed text states an absolute dispatch is a rule on what enters agent-facing text; if it binds a project's own skills it goes in the admission standard, if the installed text's maintainer alone, in the new head and a %% line, not as a writing rule in the primer.
- `rejected@agent-skills@admission-standard-in-primer`: constrains, respect. Cites seed: `design@agent-skills@primer-limit`; the primer section's dispatch judgement is every session's act, but a rule to writers of installed text on absolute dispatches placed in the primer would bring part of this design back.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: constrains, none. Cites seed: design@agent-skills@behaviour-additions-need-real-use; the work changes neither the record search's scope nor the pending-file directories.
- `issue@agent-skills@expectation-sets-for-the-installed-skills`: constrains, none. Cites seed: design@agent-skills@behaviour-additions-need-real-use; the retrospective questions on dispatch concern the session's behaviour and add no expectation set.
- `issue@agent-skills@no-head-argues-an-outside-instruction-s-precedence-or-the-reliance-on-the-harness`: touches, respect. Cites seed: design@agent-skills@behaviour-additions-need-real-use; the section relies on the harness's Agent mechanism and is argued against the harness's own instruction 'When in doubt, don't spawn', the two statements this issue says no head argues: the new head must state its relation to that instruction (a weighing it adds, not a precedence) or argue the precedence.
- `issue@agent-skills@no-head-argues-where-the-rule-against-unneeded-structure-applies`: touches, respect. Cites seeds: design@agent-skills@behaviour-additions-need-real-use and design@agent-skills@capabilities-not-structure; the work states where a fixed dispatch is admitted, a case of where the structure rule applies, and adds retrospective questions of the kind statement 4 names; its head must not contradict what that issue's closing discussion would argue.
- `tripwire@agent-skills@agent-text-written-without-the-standard`: constrains, none. Cites seed: `design@agent-skills@behaviour-additions-need-real-use`; the work is designed under the design skill and its text written with the agent-configuration skill, and no such addition is reported.
- `design@agent-skills@ruled-items-labelled`: constrains, none. Cites seed: `design@agent-skills@capabilities-not-structure`; the draft puts no labelled item to the owner and changes no prefix.
- `design@agent-skills@design-home-is-built-intent`: constrains, respect. The new agent-skills head lands with the primer section and retrospective questions; until then the plan document is its only home.
- `design@agent-skills@head-prevails-over-diverging-code`: constrains, respect. The new head limits absolute dispatches to three grounds; each existing one (record search, review axes, development checkpoint, audit, goal reviewer, transcript extraction at a plan's assembly) is checked against them in the work, else the head lands diverging from the installed text.
- `design@agent-skills@local-intent-binds`: constrains, respect. Re-judging an existing absolute dispatch under the new conditions reads its %% comment and introducing commit first, whose reason binds as a presumption.
- `design@agent-skills@standing-argument-in-head`: constrains, respect. The new head carries its premises: the goal and its constraint, the harness's tool description read (with how it is re-read), the owner's real-use report, and the fact defeating each rival.
- `design@agent-skills@heads-stand-on-argument`: constrains, respect. The owner's real-use report lands as an attributed premise, the owner's proposal is recorded on its argument, and the owner's token hint is a fact or a threshold, not a ground by itself.
- `design@agent-skills@attribution-replaces-quotation`: constrains, respect. The head and the commit attribute the owner's real-use report and size hint with their hedges ('nearly never', 'occasionally, not reliably', 'about'), never quote them.
- `design@agent-skills@unargued-approval-is-argued`: constrains, respect. The retrospective questions are listed with no argument or rival each; an approval of them in a batch is argued before any record states them.
- `design@agent-skills@owner-rules-at-a-fork`: constrains, respect. The threads, the owner's proposal and size hint included, close only after this search, on their argument or as a fork.
- `rejected@agent-skills@head-cites-every-owner-ruling`: constrains, respect. The draft marks the primer thread as the owner's proposal; the head grounds on the argument and the owner's real-use report, not on the owner's ruling, which does not bring this design back.
- `rejected@agent-skills@owner-words-quoted-in-head`: constrains, respect. The head records the owner's real-use report by attribution; quoting it would bring this design back, while quoting the harness's tool description is a source, not the owner.
- `issue@agent-skills@a-head-s-measurement-says-nothing-of-its-magnitude`: constrains, none. Cites seed: design@agent-skills@primer-limit; the work writes no measured magnitude into a head nor the head-ground fragment, the token figure staying out of the primer.
- `design@agent-skills@document-vocabulary`: constrains, none. Cites seed `design@agent-skills@step-is-no-commit`; its subject is the plan documents' words (spec, milestone, slice, step), and the work coins no plan-document word, so nothing in it is limited.
- `design@agent-skills@step-is-no-commit`: constrains, respect. Is a seed; the thread that a dispatched piece is carried whole, its commit included, must not turn into a mapping of a dispatched piece or step to a commit count, which this head leaves to the implementing session's judgement.
- `design@agent-skills@no-untested-snippet-is-authority`: constrains, respect. The spec will carry the primer section's text as a block, which must stay labelled an illustration of a shape, as the draft does; and a dispatched implementer given the plan is the reader this head guards against taking snippets as absolute truth.
- `design@agent-skills@plan-written-in-the-converging-session`: constrains, respect. The primer's carry-or-dispatch guidance must not let a session hand the writing of a plan document to a later session; it agrees with the draft's sentence that the main session's context holds the exchange with the owner, which no brief rebuilds.
- `design@agent-skills@ledger-from-transcript`: constrains, respect. The planning skill's subagent assembly is one of the absolute dispatches the work lists: it fits the condition of an output a check verifies, and the rule that a returned conclusion is checked once must count the transcript reviewer's check as that one check, or state how the two relate, so the assembly is not checked twice.
- `design@agent-skills@undesigned-work-is-an-issue`: constrains, respect. The two parked threads, the partition of parallel work and report files written as an agent works, are known undesigned work, so each lands as an issue entry in the owning anchor, agent-skills, with its leads, as the draft says.
- `rejected@agent-skills@spec-plus-detailed-plan`: constrains, respect. The primer's guidance on dispatching implementation, its brief weighed against what it leaves out, must not lead a session to write a dispatched developer a second, detailed step-by-step plan with code; the draft's sentence that a plan document carries most of what an implementer needs keeps clear of it.
- `design@agent-skills@unrecognised-entries-reach-upstream`: constrains, respect. A standing question costs every retrospective its loading, so each dispatch question must watch an occasion most sessions meet; the work argues every session judges dispatch, which meets it, and a question on a check or a piece too large must not be one most sessions never meet.
- `design@agent-skills@premortem-as-watch-points`: conflicts, check-reversal. The work adds standing questions to the list this head enumerates, so it widens the head: each must name the decision or goal it watches (the new dispatch head, `goal@session-orchestrates-the-work`) and the head's body gains them; and the head records that a session sees a subagent's brief and report, not its context, so the question whether a dispatched agent's context compacted is answerable only through its transcript.
- `design@agent-skills@intake-shares-the-admission-standard`: constrains, none. Cites seed `design@agent-skills@behaviour-additions-need-real-use`; it governs how the intake judges a fix, and the work changes no intake step, so nothing in it is limited.
- `rejected@agent-skills@retrospective-asks-for-unrecognised-lines`: constrains, respect. The work proposes no question on unrecognised lines, so it brings nothing back; its reason, the cost of a standing question in every retrospective, limits the dispatch questions to ones most sessions can answer.
- `design@check@nothing-of-a-project-is-compiled-in`: constrains, none. Cites seed: `design@agent-skills@primer-limit`; the work adds a primer member but changes nothing init writes or the tool compiles in, so nothing in the work is limited by it.
- `design@check@installed-files-committed`: constrains, none. The work edits the shipped primer and retrospective skill, so its installed copies are reinstalled and committed in the same change; this limits no shape of the work.
- `design@check@installed-files-compared-by-bytes`: constrains, none. The edited content/PRIMER.md and retrospective SKILL.md make the installed copies differ until reinstalled; a procedure the change follows, limiting no shape of the work.
- `design@check@primer-import-line`: constrains, none. Dispatch-judgement-in-primer places the guidance in the primer because this import delivers it to every session; the work rests on the delivery and changes nothing of it.
- `design@check@orphan-rejected-entry`: constrains, respect. The work records a design head in agent-skills with losing alternatives; any of them it records as a rejected entry must be cited from inside that head's section, or the check fails.
- `issue@knowledge-architect@the-retrospective-asks-for-no-tool-a-session-set-up`: touches, none. The work adds dispatch standing questions to the retrospective skill, the document this issue's owner consideration names as the moment for its own question; both edit that skill's standing questions, and neither limits the other.
- `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand`: touches, none. The issue the work opens on partitioning parallel work (a shared tree or worktrees per agent) bears on the per-agent worktrees and scratch directories this issue would give a command; the opened issue can cite it.
- `design@knowledge-architect@versioning-policy`: constrains, respect. The work changes only installed text, the primer and the retrospective skill, so its changelog entry is classed patch by this table.
- `design@knowledge-architect@changelog-entries`: constrains, respect. A primer section that makes sessions dispatch where they carried, and new retrospective questions put to the owner, are observable changes, so the branch owes a Workflow entry with surface agent-skills.
- `design@knowledge-architect@the-branch-writes-its-changelog-entries`: constrains, respect. The branch implementing the primer section and the retrospective questions writes its own Workflow entry in Next release.
- `design@knowledge-architect@git-flow`: constrains, respect. The issue the work opens on who commits and how parallel agents' parts merge must keep each commit of the branch passing the tip checker, and every part landing on a branch with main's history linear.
- `design@knowledge-architect@no-operation-loses-content`: constrains, respect. A dispatched agent that carries its own commit, or parallel agents sharing one tree, act in a checkout holding another author's uncommitted work, where an amend, rebase or restore by one loses the other's content; the opened partition issue and the carried-whole rule must keep that out.
- `design@knowledge-architect@scratch-worktree-outside-git-rules`: constrains, respect. A worktree per agent that holds the agent's durable part of the work is no scratch worktree, so the parked partition design cannot place parallel agents' work under this exception.
- `design@model@section-homes-carry-slugs`: constrains, respect. The new primer section Dispatching work is a level-two heading of a section home, so it owes a slug (the draft names `primer@dispatching-work`), and the slug becomes an interface consumers may cite.
- `design@model@installed-entities-from-the-tree`: constrains, respect. The new primer section is written in content/PRIMER.md, so its slug is the build placeholder {{slug:dispatching-work}} (as every primer heading there is, measured with grep), and the section is defined only once the installed copy is reinstalled.
- `goal@serves-one-kind-of-project`: constrains, respect. The primer section is argued on the claude harness default against dispatching; the text serves that harness and adds no mode for another.
- `goal@every-target-project-can-adopt-it`: constrains, respect. The primer text and the retrospective question must hold nothing particular to this project, which is one ground for keeping the owner's 300k figure out of the primer.
- `goal@design-is-recorded-with-its-arguments`: constrains, respect. The work records a new agent-skills design head for the dispatch judgement and the absolute-dispatch rule, which must carry its argument and the losing alternatives the draft lists.
- `goal@records-reach-their-reader`: constrains, respect. The work opens issues for the parallel partition and the report files, and a head, each to cite the entries it bears on (the dispatch issue and the goal) so later sessions meet them.
- `goal@the-owner-decides`: constrains, respect. A dispatched developer or other agent that decides something must have its decision reach the owner through the session; the report standard and the carried-whole rule must keep that path.
- `goal@the-owner-s-word-is-verified`: constrains, respect. The owner's token hint and real-use report are words the work acts on; the draft checks the hint (window sizes differ, meaning unestablished) before absorbing it into a retrospective question.
- `goal@design-with-agents-as-counterparts`: constrains, respect. The dispatch judgement argues that the main session's context holds the exchange with the owner; dispatched grounding must still reach the owner established before the owner rules.
- `goal@agents-work-without-drift`: constrains, respect. Dispatched agents build too: the dispatched developer loads the development skill and sends its checkpoint axes, and a returned conclusion is checked once, so dispatched work stays within the record.
- `goal@generic-tools-and-procedures-are-shipped`: constrains, respect. The dispatch judgement, today in the owner's user-level instructions, is taken into the shipped primer on the owner's proposal; only the generic part ships.
- `goal@agents-get-a-complete-workflow`: constrains, respect. Dispatch guidance now lives in the owner's user-level instructions, outside the workflow; the primer section brings it inside so the workflow is complete without them.
- `goal@only-needed-instructions`: constrains, respect. The primer section is admitted only because the real-use report shows judgement does not handle dispatch well enough; each retrospective question and the parked report file must pass the same test, and the token bound stays out.
- `goal@the-workflow-improves-through-real-use`: constrains, respect. The primer section rests on the owner's real-use report, the retrospective questions gather further real-use evidence, and the report file waits for it.
- `goal@session-orchestrates-the-work`: constrains, respect. The work is the workflow serving this goal; it leaves when to dispatch to judgement with no size or kind threshold, as the goal leaves open, and keeps the main session on the design.

## Criteria

### The main session orchestrates the agents that carry the work, and the goal fixes no when, kind or size `##orchestration-goal`

Binding, from `goal@session-orchestrates-the-work`. Met by `thread@dispatch-judgement@dispatch-judgement-in-primer`
and `thread@dispatch-judgement@dispatched-piece-carried-whole`.

### No instruction for a case judgement handles well enough `##only-needed-instructions-goal`

Binding, from `goal@only-needed-instructions`. Met: the section states a judgement and no
threshold, and each further paragraph repairs a reading that allows two acts or rests on the owner's
real-use report.

### A capability and the judgement to use it, no structure the work does not need `##no-imposed-structure`

Binding, from `design@agent-skills@capabilities-not-structure`. Met by
`thread@dispatch-judgement@absolute-dispatch-conditions`; the division of a piece is left to its
brief.

### A behaviour addition rests on real-use evidence, or repairs a defect provable by reading `##real-use-evidence`

Binding, from `design@agent-skills@behaviour-additions-need-real-use`. Met: the owner's report
(round 2) for the judgement; a reading that allows two acts for the carried-whole and the verification
paragraphs (round 5).

### Every statement names an act it changes `##every-statement-an-act`

Binding, from `design@agent-skills@statement-names-an-act`. Met: each sentence changes a dispatch, a
carry, a verification or an answer.

### Verifying dispatched work does not raise the total cost much `##check-cost-weighed`

Weighed, a lead of the source issue. Met: one independent verification is enough, and no rule of the
primer asks for a second.

## Threads

### The primer states how a session divides the work between itself and the agents it dispatches, built on two relations between pieces, as a judgement `##dispatch-judgement-in-primer`

Proposed by the owner, round 1: guidelines in the primer, judgement decides, no absolute condition
of size or kind (`argument@dispatch-judgement@arg-1`, `argument@dispatch-judgement@arg-2`). The
session sharpened it: the evidence rule (`argument@dispatch-judgement@arg-8`), met by the owner's
report (`argument@dispatch-judgement@arg-15`, `argument@dispatch-judgement@arg-18`,
`argument@dispatch-judgement@arg-25`); the harness's default as the mechanism
(`argument@dispatch-judgement@arg-9`, `argument@dispatch-judgement@arg-19`); the owner's three
conditions (`argument@dispatch-judgement@arg-21`, `argument@dispatch-judgement@arg-22`) worked into
(A and B) or C (`argument@dispatch-judgement@arg-27`); the plan document as the brief
(`argument@dispatch-judgement@arg-28`); the asymmetry of the two contexts
(`argument@dispatch-judgement@arg-29`); the measured reload (`argument@dispatch-judgement@arg-30`,
`argument@dispatch-judgement@arg-47`); dilution answered by a pointer
(`argument@dispatch-judgement@arg-45`); a weighing, not a precedence
(`argument@dispatch-judgement@arg-46`); scratch directories (`argument@dispatch-judgement@arg-48`);
the goal's reading (`argument@dispatch-judgement@arg-56`); one carrier for verifying and repairing
(`argument@dispatch-judgement@arg-68`, `argument@dispatch-judgement@arg-73`); and, on the owner's
point that pieces relate to each other too (`argument@dispatch-judgement@arg-77`), the rebuild on two
relations (`argument@dispatch-judgement@arg-79`, `argument@dispatch-judgement@arg-82`). Its rival, no
text, lost to the owner's report and the harness's default; guidance in each skill lost to
`argument@dispatch-judgement@arg-7`. Final state: approved. The owner, round 8: "Good, I have
nothing more to add or contest." Shape: "Decided design", the section. Harvest: the head
`design@agent-skills@work-divided-by-judgement` in the agent-skills design home.

### An installed text fixes a dispatch only for a step that needs a separate point of view, whose output is verified before it is used, or at an occasion most work meets `##absolute-dispatch-conditions`

Proposed by the owner, round 1 (`argument@dispatch-judgement@arg-3`,
`argument@dispatch-judgement@arg-4`), sharpened by the session into a test on the step's property
(`argument@dispatch-judgement@arg-13`). Every existing absolute dispatch fits a ground, and the rule
lives in a head, not in the admission standard, since it binds what the installed text states
(`argument@dispatch-judgement@arg-49`). Final state: argued, round 4; confirmed in the owner's word
of round 8. Reopened at the review of this spec by a material finding:
`design@agent-skills@admission-rules-home` gives every rule on what enters agent-facing text one
home, the admission standard, which defeats the placement "in a head, not the admission
standard"; closed again argued, the rule in the admission standard, which binds a project's own
skills too. Shape: "Decided design", the admission sentence. Harvest: inside the head
`design@agent-skills@work-divided-by-judgement`, the exception staying with its rule.

### A dispatched piece is what its brief gives the agent, and the agent carries all of it while it is dispatched `##dispatched-piece-carried-whole`

Proposed by the session, round 1, from the issue's lead and the owner's user-level instructions
(`argument@dispatch-judgement@arg-10`). The owner, round 3, feared it forbade the thaum process
(`argument@dispatch-judgement@arg-35`); the session reworded it so the brief defines the piece
(`argument@dispatch-judgement@arg-37`). Kept on reading, not on real use: the section's own Carry
bullet, read at the size of one step, produces the takeover (`argument@dispatch-judgement@arg-58`,
`argument@dispatch-judgement@arg-62`). The premortem added that what comes back unfinished is a new
piece (`argument@dispatch-judgement@arg-64`), and the rebuild made "writing a change and its
commit" the example of one carrier (`argument@dispatch-judgement@arg-80`). Final state: approved,
round 8. Shape: the section's carried-whole paragraph. Harvest: none in a design home, since one
text states it; its reason in a `%%` line at the paragraph.

### The design skill gets no text of its own on dispatching `##design-investigation-dispatch`

Proposed by the session, round 1, as a capability sentence in the design skill's loop. Superseded by
`thread@dispatch-judgement@dispatch-judgement-in-primer`, whose dispatch bullet covers a search for
another shape (`argument@dispatch-judgement@arg-34`). Final state: superseded, round 4. Harvest:
none.

### The development skill gets no edit: a dispatched developer loads it and sends its own checkpoint reviews `##development-dispatch`

Proposed by the session, round 1 (`argument@dispatch-judgement@arg-11`), on the measured nested
dispatch and the primer's list of installed skills (`argument@dispatch-judgement@arg-52`). Final
state: argued, round 4. Harvest: inside `design@agent-skills@work-divided-by-judgement`, as the rival "guidance in each
activity's skill", the development skill's included.

### One independent verification, by a session other than the claim's author, against the evidence its report carries, is sufficient for a claim the work acts on `##dispatch-report-standard`

Proposed by the session, round 1, as a sentence on the dispatcher's side
(`argument@dispatch-judgement@arg-12`); the owner gave its principle, round 2: equal trust, one
verification is enough (`argument@dispatch-judgement@arg-23`). The record search found the wording "a
check is not checked again" in conflict with the named check chains
(`argument@dispatch-judgement@arg-42`, `argument@dispatch-judgement@arg-43`). The owner, round 5,
asked for a definition instead of a workaround (`argument@dispatch-judgement@arg-57`); the session
found the record holds the principle as a minimum (`argument@dispatch-judgement@arg-59`,
`argument@dispatch-judgement@arg-60`, `argument@dispatch-judgement@arg-61`). The owner, round 6,
made it a general principle with self-verification's lapses as its ground
(`argument@dispatch-judgement@arg-66`, `argument@dispatch-judgement@arg-67`), measured in this
session (`argument@dispatch-judgement@arg-70`). The boundary on a claim nothing acts on was put to
the owner as a fork (`argument@dispatch-judgement@arg-72`); the owner, round 7: "2. Agreed. This
matters most when the claims is acted on." and "Both cases are pretty much the same here actually."
(`argument@dispatch-judgement@arg-76`, `argument@dispatch-judgement@arg-78`). Final state: approved,
round 8; the boundary is the session's choice at a fork, which no goal ranks, approved by the
owner. Shape: the section's verification paragraph. Harvest: the head `design@agent-skills@at-least-one-independent-verification`, with
the instances that cite it, as the owner asked, round 6: "with individual instances citing it".

### No token bound goes in the primer; a piece too large is asked about in the retrospective `##dispatch-size-hint`

Proposed by the owner, round 2: at most a hint of a bound, maybe 300k tokens
(`argument@dispatch-judgement@arg-17`). The session argued against a number
(`argument@dispatch-judgement@arg-31`), and the record holds the owner's earlier figure as the
search's ground, not a dispatch guideline (`argument@dispatch-judgement@arg-50`). Final state:
superseded by `thread@dispatch-judgement@retrospective-dispatch-questions`, round 4. Harvest: a
`rejected` entry if `skill@knowledge-architect-decision-recording` admits it.

### The retrospective asks whether the session carried work an agent could have carried, whether an agent came back unfinished or with an unusable report, and whether a returned conclusion proved wrong after its independent verification or was verified again where no skill asked `##retrospective-dispatch-questions`

Proposed by the session, round 1 (`argument@dispatch-judgement@arg-14`); reshaped after the search,
each question naming what it watches, answerable from the session's side, and met by most sessions
(`argument@dispatch-judgement@arg-51`). Final state: approved, round 8; the compaction question's drop is re-argued after the review of this
spec found its first reason wrong. Shape: "Decided design", the questions. Harvest: the three questions in `design@agent-skills@premortem-as-watch-points`.

### Report files written as an agent works, annotated by a verifier, wait for evidence `##dispatch-report-file`

Proposed by the owner, round 2 (`argument@dispatch-judgement@arg-24`); the session found it predicts
a behaviour (`argument@dispatch-judgement@arg-33`). Final state: parked. Cause: a real session whose
returned report could not be acted on without redoing the work. Re-entry: the trigger of the
`deferred` issue `issue@agent-skills@dispatched-agents-write-their-report-as-they-work`, a retrospective answer to the
second dispatch question naming such a report (`argument@dispatch-judgement@arg-53`).

### How parallel work is partitioned among agents is decided at design and planning, in a later discussion `##parallel-work-partition`

Proposed by the owner, round 3 (`argument@dispatch-judgement@arg-35`,
`argument@dispatch-judgement@arg-36`). The owner, round 3: "We can probably keep the guidelines such
as in the draft for now, and keep that idea for a later design discussion." Final state: parked.
Cause: a piece of work whose plan wants parallel agents. Re-entry: the trigger of the `deferred`
issue `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`, which carries the facts of
`argument@dispatch-judgement@arg-38`, `argument@dispatch-judgement@arg-39` and
`argument@dispatch-judgement@arg-54`.

### The owner's user-level instructions are aligned with the section after the work `##user-level-alignment`

Proposed by the owner, round 2 (`argument@dispatch-judgement@arg-20`). Final state: approved, round
8, as "brought into line" after the spec, with no actor named. Who does it and when is the owner's:
D1. Outside the project: no harvest.

## Arguments

### Absolute conditions on when or what to dispatch would fail or grow without bound, size being the most varied measure `##arg-1`

Round 1, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "we should not give absolute conditions for when to dispatch subagents, or even for which kind of work"; "these tasks can have very varied shapes, so absolute instructions would either fail, or need to become very large to handle every case (especially for determining whether it is worth dispatching or not by size of work, which is the most varied metric that can be faced)".

### Guidelines in the primer reach every agent, and judgement decides `##arg-2`

Round 1, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "it is better to provide guidelines, in the primer where they reach all agents, and to rely on agent judgement for deciding."

### A mandated step an agent does better, or a review that needs another point of view, is the exception on what to dispatch `##arg-3`

Round 1, owner. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "The exception to "which tasks to dispatch" are steps that the workflow mandates where an agent can do the work better (and in such cases we try to provide a shipped subagent definition, such as record search), or reviews, where a subagent is clearly better, because it can take a different point of view."

### A mandated step, or an occasion most work meets, is the exception on when to dispatch `##arg-4`

Round 1, owner. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "The exception to "when to dispatch" has the same shape: a step the workflow mandates (such as record search, plan writing...), or situations and tasks which can be anticipated, because they happen for most work, if not all (such as a merge of a branch)"

### A case that judgement handles gets no instruction `##arg-5`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: `goal@only-needed-instructions`: "A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve."

### A size threshold is a rule per interaction, which the structure head's measurement argues against `##arg-6`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: `design@agent-skills@capabilities-not-structure`: "Its measurement is the argument against size thresholds: in one revision of designing-together, 34 edits produced 10 interaction defects, nearly all of them between rules that scripted one exchange. A size threshold for dispatching is that kind of rule. Every activity would meet it, and each would need its own carve-out."

### No skill loads at the moment a dispatch is judged, so the primer is the home `##arg-7`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: `design@agent-skills@primer-limit`: "A session meets the question of whether to dispatch in every activity, and no skill loads at that moment. So by its own test, the primer is the home."

### A behaviour addition needs real-use evidence, and a guideline that changes no act is not written `##arg-8`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@design-investigation-dispatch`. Key words: "The evidence rule applies to this addition."; "A guideline for a case where sessions already dispatch changes no act. It fails `design@agent-skills@statement-names-an-act`, so it is not written."; "A guideline for implementation or design-time investigation needs your observation".

### The harness's default leans against dispatching, and no installed text weighs on the other side `##arg-9`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*the harness's default tells a session to do the work itself when in doubt, and no installed text weighs on the other side, so a session carries a coherent change itself.*"; ""Do the work yourself when it is a handful of tool calls … When in doubt, don't spawn." Under test 3 of what earns a head, that external fact earns the decision a head."

### One coherent change is never split between two authors `##arg-10`

Round 1, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "Read together, both say *do not split one coherent change between two authors*: either the main session carries it whole, or one agent carries it whole, commits included."; "If it said "keep authorship", the session would read it as "keep the work"."

### A dispatched developer can send its own checkpoint reviews; a saved workflow stays with the main session `##arg-11`

Round 1, session. Bears on `thread@dispatch-judgement@development-dispatch`. Key words: "A dispatched developer can itself send the checkpoint review axes that `skill@knowledge-architect-development` names. So dispatching the implementation takes no reviews away from it."; "A saved workflow, such as an audit axis's run, stays with the main session."; "the guidelines cover `Agent` dispatches only."

### The primer binds a subagent's own verification; the dispatcher's side is unstated `##arg-12`

Round 1, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The primer's `primer@verify-before-relying` already binds every subagent"; "What the primer does not state is the dispatcher's side: *a conclusion that comes back is a claim, checked like one's own before acting on it.*"

### An absolute dispatch is admitted only on a property of the step that holds whatever the task's size `##arg-13`

Round 1, session. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "**an installed text states an absolute dispatch only where its ground is a property of the step that holds whatever the task's size.** The two such properties are a fresh point of view (reviews, the goal reviewer) and an output whose shape a check verifies (the record search, the audits). Any other "always dispatch X" is a size claim in disguise"; "Without this test, the two classes are a list. A later writer would extend the list by analogy, and the same threshold creep would come back."

### The retrospective questions produce the evidence that corrects judgement-based guidelines `##arg-14`

Round 1, session. Bears on `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "these are what produces the real-use evidence that judgement-based guidelines are later corrected by"

### The main session nearly never dispatched, and spent context the design needed `##arg-15`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "I nearly never saw the main session dispatch subagents for tasks where I believe it could have been useful, especially for saving context that was not well spent and should probably have been kept for design discussions."

### As far as the owner knows, a session with a large context costs more usage than one with a small context `##arg-16`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "There is also the cost of a session whose context gets high, which costs more usage quota than a session with nearly empty context AFAIK. This was discussed at the last redesign of the record search."

### Numbers do not belong in guidelines, at most a hint of a token bound per agent `##arg-17`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-size-hint`. Key words: "I'm not sure we should include numbers in guidelines though. At most, a hint that "if dispatched agents regularly exceed <bound> tokens, they might not be sufficiently fine grained" with bound at maybe 300k."

### In the goal-drafting session the leads were applied and worked; since, dispatch is occasional `##arg-18`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "In the session that drafted the orchestration goal and wrote the issue, the session applied my leads, and from my observation it went very well, with the main session ending the task with enough spare context"; "In sessions since that, I have seen that happen occasionally, I guess due to sessions reading the orchestration goal, but definitely not reliable."

### As far as the owner knows, the harness keeps dispatching opt in because users might not like it, early users preferring to watch the work `##arg-19`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the claude code harness tries to discourage dispatch by default, keeping that feature opt in, because users might not like it (especially at the beginning, where trusting agents is harder, and early users prefer keeping a close watch on agent work)."

### The owner's user-level instructions are more likely the flawed text, and are to be aligned `##arg-20`

Round 2, owner. Bears on `thread@dispatch-judgement@user-level-alignment`. Key words: "I would like to bring my root CLAUDE.md to align with the guidelines we settle on in this session. The current text was written without a deeper discussion and brainstorming, so it is more likely to be the flawed one, at least on the wording aspect."

### Three conditions make a dispatch clearly worth it `##arg-21`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the current session's full context is not useful to the task (a brief can carry what the agent will need)"; "the process of the work is not useful to the current session, only the result and/or report is"; "A fresh point of view is needed."; "(I am unsure if they should be "or" or "and")".

### Implementation sits where the conditions are in doubt `##arg-22`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The cases were there is doubt about these is when it is harder to decide, and this includes implementation I believe, where the current context is actually useful, and the process itself can be useful, but the cost of doing it itself might be too big for the session."

### A verifier's check weighs as much as the main session's, and one check is enough `##arg-23`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "there is no reason to put more trust in the main session than in a subagent"; "one round of verifier is enough"; "One verifier verifying the claims of a review is just as much weight, and just as trustable, as the main session verifying the claims of a review. The point is that one verification was done."

### The completeness of a subagent's report, not its judgement, is the weak point `##arg-24`

Round 2, owner. Bears on `thread@dispatch-judgement@dispatch-report-file`. Key words: "The hard point this leaves is not on trusting the judgement of the verifier or of the initial reviewer, but more on trusting the completeness of the subagent reports. Because those are often written one shot in the last messages"; "Having the main session carry the verification as the advantage that it has high chances to fill any gap in the report. I think that having file reports that agents write and  a verifier can edit might help reduce this gap."

### The owner's report meets the evidence rule `##arg-25`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "Your report meets `design@agent-skills@behaviour-additions-need-real-use`"; "**What it lacked:** context left for the design discussion."; "Your reason for the default (users who want to watch the work closely) is your belief, recorded as such."

### Each turn costs more as the context grows `##arg-26`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the API resends the whole context on every turn, and cached tokens are billed per token at a reduced rate. So each turn of a session costs more as its context grows. This is my reading of the pricing structure. I have not checked it this session."

### A fresh view implies a briefable task; briefable with only the result needed is the clear case `##arg-27`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**C implies A, and makes it a requirement.**"; "**A and B together** is the clear case"; "**A without B:** … Carry it."; "**B without A, or either one partly true,** is your doubt zone, implementation among it."; "**(A and B) or C** is the clear dispatch. Everything else is weighed."

### For implementation, the plan document is the brief `##arg-28`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@development-dispatch`. Key words: "**For implementation, the plan document is the brief.**"; "The process that still matters to the main session comes back through the agent's "Decided" and "Met outside the task" sections".

### The main session's context is worth more than an agent's, since no brief rebuilds it `##arg-29`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**The two contexts are not worth the same.** The main session's context holds the exchange with you, and no brief rebuilds it. An agent's context is rebuilt by sending it again. The harness's generic "when in doubt, don't spawn" ignores this asymmetry."

### A fresh agent pays a fixed reload `##arg-30`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**a fresh agent pays a fixed reload.**"; "a dispatch whose brief costs about as much as the work is not worth it."

### A token bound in the primer measures the wrong figure, varies by model, and serves the next session `##arg-31`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-size-hint`, `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "The one figure a session sees per agent, `subagent_tokens`, has an unestablished meaning"; "Context windows differ by model. Under 300k, a 200k-window agent never trips the bound."; "The act the hint changes is cutting pieces finer *next time*. That is a lesson from a finished session, so its occasion is the retrospective, not the moment of dispatch."

### The record already checks search verdicts as the session's own claims, which ends the chain `##arg-32`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Your point, that one check carries the same weight whoever makes it and a check is not checked again, already stands in the record for the search. Its verdicts are checked as the session "checks its own claims, no more and no less". The primer would generalise that, and it ends the chain of verifiers."

### Report files written as an agent works, annotated by a verifier, predict a behaviour without evidence `##arg-33`

Round 2, session. Bears on `thread@dispatch-judgement@dispatch-report-file`. Key words: "Reviewers already write their reports to files"; "dispatchers saved condensed copies, or none, in 7 of 12 sessions"; "a verifier that **annotates** each finding rather than rewriting it"; "you report not having read such reports, so this predicts a behaviour without evidence"; "the verifier's brief carries the original brief, so it can see what was asked and is not answered."

### The design skill needs no text, since the primer's dispatch bullet covers a search for another shape `##arg-34`

Round 2, session. Bears on `thread@dispatch-judgement@design-investigation-dispatch`. Key words: "superseded by #dispatch-judgement-in-primer: "a search for another shape" is in the dispatch bullet, so the design skill needs no text"

### Parallel work is another reason to dispatch, and the carried-whole rule must not forbid the thaum process `##arg-35`

Round 3, owner. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "dispatching agent has another condition where it can be beneficial, in addition to the three I mentionned before: parallelizing work."; "Here, more than context, the goal is to also save time."; "In particular, **A dispatched piece is carried whole by its agent** no longer necessarily holds. As in the thaum split example."; "We should just make sure that our guidelines do not rule out a process such as wht happened in thaum with parallel subagents."

### How parallel work is partitioned is decided at design and planning, not during a task `##arg-36`

Round 3, owner. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "such kind of procedures are significant decisions about how the work is carried, and those are hard to come with "on the move", while carrying a task."; "maybe taking these kind of decisions about how the work is carried could be left to the design and planning steps."

### The brief, not a rule about commits, defines what belongs to a dispatched piece `##arg-37`

Round 3, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "The lead behind the clause is narrower than its wording. It says the session must not *interrupt* a dispatched agent to take over a task that belongs to the dispatched work, such as its commit, just because that task is small. What makes the commit "part of the dispatched work" is the brief, not a rule about commits."

### Under a partition between agents, which session judges the commit count is ambiguous `##arg-38`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "`design@agent-skills@step-is-no-commit` gives "how many commits it takes" to "the implementing session". Under a plan that partitions the work between agents, it is ambiguous which session that is."

### A shared tree is safe only while partitions touch disjoint files `##arg-39`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "That is safe only while their partitions touch disjoint files. A worktree per agent removes the condition, at the cost of a merge."

### Parallelism gets no sentence for lack of evidence, and nothing in the draft forbids it `##arg-40`

Round 3, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "It does not show a session that failed to parallelize when it should have. So a sentence would predict a behaviour"; "the bullets say when to dispatch, not "only when""; "Thaum's split also met the dispatch bullet on its own terms: a mechanical change, briefable, where only the result matters."

### A deferral needs a home a later session reads `##arg-41`

Round 3, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "The deferral needs a home that a later session reads, because a thread parked in this conversation dies with it."

### Checked once, as written, collides with the check chains the installed text builds `##arg-42`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Read as written, "a conclusion that comes back is checked once … a check is not checked again" reaches every check chain the installed text already builds on purpose"; "it would also bring back two `rejected` designs of the audit"; "scope the sentence to dispatches that the session judges, and leave every dispatch an installed skill names under its own head's argument. Those chains have measured reasons, such as verifiers that wave findings through". (Proposed first by searcher agent-4, L311.)

### A session acting on another's check has not observed the run, so the report must carry the evidence `##arg-43`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "A session that acts on a verifier agent's check has not observed the run."; "Its condition is that **the report carries the evidence**"; "Extending that head to every returned conclusion changes it, so the change is argued here and recorded as a change".

### Two paragraphs lack real-use evidence and would be deferred `##arg-44`

Round 4, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "two parts of the draft lack real-use evidence"; "**Default if you have no instance:** both parts leave the primer and become `deferred` issues". (Q1, withdrawn in round 5.)

### The primer dilutes in a long session, answered by one pointer at the moment implementation starts `##arg-45`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*Dilution.* `rejected@agent-skills@head-rules-whole-in-primer` records that the primer "can dilute in a long session". The case you saw fail, implementation, starts at a known moment"; "**one sentence there, with its pointer to the primer section**". (Raised by searcher agent-3, L251.)

### The section is a weighing beside the harness's default, not a precedence over it `##arg-46`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The head states the section as a weighing beside the harness's default, not a precedence over it."

### The reload is measured at about 36,000 tokens `##arg-47`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "A fresh agent's fixed start is about 36,000 tokens"; "The head cites it as the ground of "carry when the brief costs as much as the work"."

### Agents dispatched together need distinct scratch directories, shown in real use `##arg-48`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "*Scratch directories.* `design@agent-skills@a-scratch-directory-per-subagent` states a general rule, but only the review skill and the searcher deliver it."; "Its evidence is real use: parallel searchers overwrote each other's files."

### Every existing absolute dispatch fits a ground, and the rule lives in a head `##arg-49`

Round 4, session. Bears on `thread@dispatch-judgement@absolute-dispatch-conditions`. Key words: "Every existing absolute dispatch still fits"; "if it bound a project's own skills, it would belong in the admission standard; if it binds only this repository's maintainers, in a head and a `%%` line. My default is **the head only**."

### The 250k figure grounds the search's bound, not a dispatch guideline `##arg-50`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-size-hint`. Key words: "The 250k is an argument the search's bound stands on, not a guideline a dispatcher can apply. So no number goes in the primer".

### A standing question must name what it watches, be answerable from the session, and concern most sessions `##arg-51`

Round 4, session. Bears on `thread@dispatch-judgement@retrospective-dispatch-questions`. Key words: "Adding questions widens it, and its argument covers each question only if the question names what it watches."; "So my compaction question is unanswerable as written."; "every project's retrospective answers each standing question, so a question must concern what most sessions meet."

### A dispatched developer loads the development skill through the primer and reads its own code's record `##arg-52`

Round 4, session. Bears on `thread@dispatch-judgement@development-dispatch`. Key words: "A dispatched developer loads `skill@knowledge-architect-development` through the primer's list of installed skills, not through its brief, as `design@agent-skills@audit-agent-per-axis` requires. It reads its own code's record, per `design@agent-skills@code-change-reads-its-record`."

### A parked thread takes a deferred issue, since only a trigger is read by a review `##arg-53`

Round 4, session. Bears on `thread@dispatch-judgement@dispatch-report-file`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "Both are opened as `deferred` issues with a trigger, since only a trigger is read by a review."

### The partition issue carries the facts the search found `##arg-54`

Round 4, session. Bears on `thread@dispatch-judgement@parallel-work-partition`. Key words: "the cold build of a fresh worktree"; "parallel target directories that filled a shared tmpfs"; "the content-loss risk of a shared tree"; "that a durable per-agent worktree is not a scratch worktree"; "that `skill@knowledge-architect-review@review-invariants` gives a worktree only to a subagent that judges a commit." (From searchers agent-2, agent-13, agent-15, agent-6.)

### The source issue closes, a changelog entry is owed, and the harvest splits the heads `##arg-55`

Round 4, session. Bears on no thread: bookkeeping. Key words: "The source issue closes with the work: its clause accepts "refused on its argument"."; "The branch writes a Workflow entry in the changelog, classed as a patch."; "The harvest splits the decision into one head per decision, per `design@agent-skills@one-decision-per-head`. A part stated by one text alone earns no head and keeps its reason in a `%%` line."

### The goal is read as settled by judgement with named exceptions `##arg-56`

Round 4, session, in its "Decided" section. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "the goal leaves when, and for what size of work, a dispatch happens to design. I read it as settled by judgement with named exceptions. The rival reading, a threshold, is defeated by `design@agent-skills@capabilities-not-structure`."

### A check is another agent than the reporter verifying the claim, as applied until now `##arg-57`

Round 5, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "I don't think we should work around this in our additions. Instead, we should make it clear what a "check" means. To me, what was meant by a check, is another agent than the one reporting an information verifies the claim. This can be the main session, or a verifier. For review repairs, this can be a dispatched repairer, tasked with both verifying findings, and applying repairs"; "I think that this is pretty much the general principle that had been applied until now, no ?"

### A defect provable by reading is admitted without real-use evidence `##arg-58`

Round 5, owner. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Those obviously did not happen, because the instructions that could trigger them did not exist yet. A defect that is provable by reading the instructions is admitted without real use evidence."; "If we defer those, we have to make sure the text is not making those happen."

### The ceiling wording caused the collisions, and the definition removes the exemption `##arg-59`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The clause that did the harm was "that check is not checked again". It reads as a ceiling, so it collided with every skill that asks for more"; "Your definition removes the need for an exemption."

### The record holds the principle as a minimum `##arg-60`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**Review:** … That is one check, by an agent other than the reporter."; "`--check` and `--verify` are commands that check the report's form and quotations. They are no check in your sense."; "That sample checks the verifier itself, and three tripwires guard it."; "So the record holds your principle as a minimum. Where it goes beyond one check, a skill states its reason, and no skill builds an open-ended chain. The rule therefore needs a minimum, not a maximum".

### Each clause of the check paragraph closes a two-act reading `##arg-61`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"One check is enough", not "a check is not checked again".** It ends the chain, because no rule asks for a second check. It forbids nothing"; "**"It stands for the session's own verification".** It answers `design@agent-skills@verification-in-primer`."; "**"A command that checks a report's form is no such check".** Without it, a session could take `worklist --verify` as the one check and skip reading the verdicts."

### The Carry bullet, read at the size of one step, produces the takeover `##arg-62`

Round 5, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "The draft's own **Carry** bullet says to carry a piece "when the brief would cost about as much as the work". Applied to the commit step of a dispatched change, taken alone, the bullet says carry it."; "So our addition, read at the size of one step, produces the takeover. The paragraph fixes the unit at the brief's piece. It is admitted on reading."

### Re-verify before acting gives two acts on a verifier's report `##arg-63`

Round 5, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"Checked once".** This is the two-act reading of "re-verify before acting" shown above. It is admitted on reading too."

### The premortem's causes and outcomes `##arg-64`

Round 5, session, at the premortem. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@dispatch-report-standard`, `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition`. Key words: Cause 1 "sessions still carry the implementation themselves: the primer dilutes, and the harness default wins" (watched by retrospective question 1); cause 2 "sessions dispatch lookups, and each pays about 36,000 tokens to reload" ("dropped: only you see usage, which fails condition 2"); cause 3 "a verifier refutes a true finding, and one check lets it pass unseen"; cause 4 "an agent returns its piece unfinished, and "the session does not take over a part of it" is read as forbidding the session to finish it" ("**changed in the design**: one clause, "what an agent returns unfinished is a new piece, for the session to judge""); cause 5 "agents dispatched together edit one tree and lose each other's uncommitted work".

### No acceptance criterion, since the work has no built code to observe `##arg-65`

Round 5, session. Bears on no thread: acceptance criteria. Key words: "**Acceptance criteria: none.** The work is text, and nothing in it is built code whose observation could show a decision wrong."

### The principle becomes a general head, a definition of sufficient verification `##arg-66`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "The principle I gave should be made a general design principle, with individual instances citing it. The bound on checks is a minimum, a definition of what is "sufficient verification"."

### Self-verification lapses often, hence a separate verification `##arg-67`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "An agent is still asked to verify its own claims before writing them, by the primer instructions, whether it is the main session or not. But this instruction is not sufficiently reliable, because it is the kind of instruction were a lapse is very common. Hence, we ask for a separate verification from a session that did not write the claims."

### Verification may be dispatched, but where its process serves the repair, one carrier takes both `##arg-68`

Round 6, owner. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Dispatching for the verification is fully admitable, mostly because in most cases the verification process is not useful (such as code defect findings). However, for some cases, such as review findings, it is likely that the verification process is actually useful for the repair."; "either the session carries both verification and repairs, or it dispatches an agent that does both verification and repairs, if the scope is too large (which reports what it acted on or not and why, and this report gets a verification of its claims too)."

### The check principle earns a head by test 2 `##arg-69`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "It earns a head by test 2: the same reason is respected at several sites, namely the review's check of findings, the record search's verdicts, the audit's verifiers, the plan reviews, and now the primer section."

### The lapse rate of self-verification is measured in this session `##arg-70`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "Every one of the 18 reported that its first `--check` failed"; "Two limits on this evidence: the failures are about exact quotation, not about the truth of a claim, and an agent may have relied on the cheap check on purpose rather than lapsed."; "dispatchers saved condensed copies, or none, in 7 of 12 sessions against an explicit instruction"; the transcript reviewer "exists to catch that expected rate".

Corrected at assembly: 16 reports state that the first check failed; for agent-4 and agent-8 a first failure is inferred from their re-runs.

### The chain ends: each claim gets one check, and the main session's claims are checked at the merge `##arg-71`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**The chain still ends.** Each claim gets one check by a session other than its author. A repairer's report holds *new* claims"; "That is not a check of a check. The main session's own claims are checked by the review before the merge."

### The minimum binds a claim the work acts on; a refutation may be sampled `##arg-72`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "`design@agent-skills@one-run-pass` has the session read "a sample of the refuted drafts", not each one."; "**A wrong confirmation** produces a wrong edit, which lands."; "**A wrong refutation** loses a finding, which nothing applies."; "**the minimum binds a claim the work acts on. A claim whose effect is that nothing is done, such as a refuted finding, may be sampled where a skill says so, with a watch on what is lost.**"; put to the owner as a fork.

### Verifying and repairing go to one carrier, as a bullet of the judgement `##arg-73`

Round 6, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "Applied to review findings, that sends verification to one agent and the repair to another, so the repairer reads everything again. The **Carry** bullet covers only the session, not two agents."; the **Keep together** bullet; "The bullet is admitted on reading as a two-act repair".

### The refinements fall within subjects already searched `##arg-74`

Round 6, session. Bears on no thread: process. Key words: "These are refinements of #dispatch-report-standard and #dispatch-judgement-in-primer, and both subjects were searched."; "So no new search is owed."

### Lapses are normal, and the separate check catches what remains `##arg-75`

Round 7, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "In practice, this instruction has a lot of lapses, and this is totally normal to me"; "Overall I think it helps reduce the amount of wrong claims, and the separate verification takes care of what's left."

### A repairer's reason for not acting may go unchecked, and both reasons are alike `##arg-76`

Round 7, owner. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "This matters most when the claims is acted on. Though this might leave a loophole in review findings, where a repaired that did not apply a repair to a claim, might not have its reason checked."; "If the reason is deciding the finding is real but not worth acting on, maybe that deserves verification. Or maybe not. Both cases are pretty much the same here actually."

### The guidelines considered pieces against the main session only, not against each other `##arg-77`

Round 7, owner, in the second part of the message. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The problem was that our draft guidelines were only considering dispatched tasks relative to the main session, but not dispatched tasks relative to each other. Maybe we should cover this case a bit more largely ?"

### A refutation keeps its reproduction, and a not-worth decision reaches the owner under Decided `##arg-78`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-report-standard`. Key words: "**"The finding is wrong."** This is a refutation."; "**"Real, but not worth acting on."** This is a decision, one the owner could make. The primer's `primer@decided-section` makes a subagent report it under "Decided""; "It is not unseen. It is merely unchecked by a third session, and that is all the boundary allows."

### Two relations between pieces govern any pair: one carrier, or separate carriers `##arg-79`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "The check principle is the same kind of question, about two pieces that must *not* share a carrier"; "**One carrier** when they reason over the same details"; "**Separate carriers** when one judges the other, as a review or a check judges what it reads; or when they are independent and the split saves context."; "The session-or-agent question then becomes the special case of the first piece the session holds."

### Writing a change and its commit is the example of one carrier `##arg-80`

Round 7, session. Bears on `thread@dispatch-judgement@dispatched-piece-carried-whole`. Key words: "**"Writing a change and its commit"** is now the example of one carrier. That makes the "carried whole" rule follow from the first bullet. The "carried whole" paragraph stays, to fix the unit at the brief's piece."

### Independence is a condition for a split, not time a reason `##arg-81`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`, `thread@dispatch-judgement@parallel-work-partition`. Key words: "**Independence is stated as a condition for a split, not time as a reason.** Your thaum case meets it."

### A relation, not a list of cases, judges a pairing nobody anticipated `##arg-82`

Round 7, session. Bears on `thread@dispatch-judgement@dispatch-judgement-in-primer`. Key words: "**The section is built on a relation, not a list of cases.** So a pairing nobody anticipated, such as an investigation and the fix it leads to, is judged by the same two bullets with no new text. That is `goal@only-needed-instructions`."


## New names, in one place

```text
`primer@dispatching-work`                        the primer section, content/PRIMER.md
`design@agent-skills@work-divided-by-judgement` a design head, the agent-skills design home
`design@agent-skills@at-least-one-independent-verification` a design head, the agent-skills design home
`issue@agent-skills@dispatched-agents-write-their-report-as-they-work` a deferred issue, crates/agent-skills/docs/open-issues/
`issue@agent-skills@how-parallel-work-is-partitioned-among-agents` a deferred issue, crates/agent-skills/docs/open-issues/
dispatching.md                                   a subdocument of the agent-skills design home, holding the two heads
```

## Decided design

### `thread@dispatch-judgement@dispatch-judgement-in-primer`, with `thread@dispatch-judgement@dispatched-piece-carried-whole` and `thread@dispatch-judgement@dispatch-report-standard`

A new primer section, placed after `primer@decided-section`. An illustration of its shape, its
wording the step's own; the `%%` lines carry the reasons:

```markdown
## Dispatching work {{slug:dispatching-work}}

**A session that can dispatch agents judges how the work is divided between itself and the agents
it dispatches.** No instruction fixes a size or a kind of work for this, except the dispatches an
installed text names.

- **One carrier**, the session or one agent, takes two pieces that reason over the same details, so
  that none reads again what another read: verifying findings and repairing them, or writing a
  change and its commit.
- **Separate carriers** take two pieces when one judges the other, as a review or an independent
  verification judges what it reads, even where the two reason over the same details; or when
  neither needs what the other reads, and splitting them saves context.
- **The session dispatches** a piece when a brief can carry what it needs of the session's context
  and the session needs only its result, not the reading and the trials behind it, as an
  investigation, a survey or a mechanical change.
- **The session carries** a piece when it will reason later over the details the piece reads, or
  when the brief would cost about as much as the work: a fresh agent first reloads its
  instructions.
- **Between the two**, as for implementing a design, it weighs what the brief leaves out against
  the context carrying the piece spends. The main session's context holds the exchange with the
  owner, which no brief rebuilds, while an agent's is rebuilt by dispatching it again; and every
  later turn costs more as the context grows. A plan document carries most of what an implementer
  needs.

**A dispatched piece is what its brief gives the agent, and the agent carries all of it**: the
session does not take over a part of it while it is dispatched, such as its commit because that
part alone is small. What an agent returns unfinished is a new piece. An agent reports what it acted
on and what it did not, with why. Agents dispatched together each get a scratch directory of their
own.

**A conclusion that comes back gets an independent verification before the session acts on it**:
a session other than the one that reported it, the dispatching session or one it dispatches,
verifies its claims against the evidence the report carries; a command that checks a report's form
is none. One independent verification is enough: it stands for the dispatching session's own
verification of that conclusion, per `primer@verify-before-relying`. A verifier's verdict on a claim
is no new conclusion; the claims a verifier adds of its own, such as a repairer's account of what it
did and did not repair, are.
```

The words: in this section, as in the primer, a **session** is the main session or any agent; "the
session" alone is the one that dispatches. The bullets are named here by their bold words: the
**One carrier**, **Separate carriers**, **The session dispatches**, **The session carries** and
**Between the two** bullets.

The argument, by part:

- **A judgement, not a threshold.** Size varies without bound, and a threshold is a rule per
  interaction, against `design@agent-skills@capabilities-not-structure`, whose measurement is 34
  edits producing 10 interaction defects in one revision of designing-together. A relation between
  pieces, not a list of cases, judges a pairing nobody anticipated. The nearest rival, no text and
  `primer@room-to-judge` alone, lost to the owner's real-use report and to the harness's default,
  which leans against dispatching; the section is a weighing beside that default, not a precedence
  over it. "Every later turn costs more as the context grows" rests on the Messages API being
  stateless: each request carries, and is billed for, the whole conversation, cached or not.
- **The primer as its home.** A session meets the question in every activity, and no skill loads at
  that moment, per `design@agent-skills@primer-limit`. The primer "can dilute in a long one", a
  long session, per `rejected@agent-skills@head-rules-whole-in-primer`: the case the owner saw
  fail, implementation, starts at point 3 of the planning skill's procedure, which gets one
  sentence and its pointer to the section, per `design@agent-skills@restatement-size-test`. The
  rival, guidance in each activity's skill, the development skill's included, restates the
  judgement in each and reaches only the sessions that load it.
- **The dispatches an installed text fixes.** An installed text fixes a dispatch only on a property
  of the step that holds whatever the task's size, today one of three: a separate point of view
  (the reviews, the goal reviewer), an output whose form a command verifies and whose claims a session
  then verifies (the record search, through `worklist --verify` and the session's reading of the
  verdicts; the audits, through their verifiers; the transcript extraction, through the transcript
  reviewer), or an occasion most work meets (a merge, a development checkpoint). Any other "always
  dispatch X" is a size claim in disguise. The rule binds what enters installed text, so its home
  is the admission standard, `skill@knowledge-architect-agent-configuration@admission-standard`, per
  `design@agent-skills@admission-rules-home`; the primer says only that the dispatches an installed
  text names are not judged.
- **Carried whole.** The **The session carries** bullet, read at the size of one step, says the
  session carries a commit alone; the harness's "Do the work yourself when it is a handful of tool
  calls" says the same. The paragraph fixes the unit at the brief's piece. It does not rule out the
  thaum process, defined under Names: its briefs said not to commit, so the commit was never part of
  an agent's piece, and the bullets are a judgement in which parts that need none of each other's details weigh.
- **The independent verification.** The primer asks every session to verify its own claims before writing them, and
  the instruction lapses often: of the 18 record searchers of this discussion, each told to quote
  verbatim, 16 reported that their first `worklist --check` failed on their quotations, and agent-4
  and agent-8 re-ran theirs after repairs, a first failure inferred. The failures are of exact
  quotation, not of a claim's truth, and an agent may have leaned on the cheap check on purpose;
  the figure is kept in the message of the commit that adds this spec. Dispatchers saved condensed
  copies of reports, or none, in 7 of 12 sessions, per `design@agent-skills@reviewer-writes-its-report`.
  So a claim the work acts on gets one independent verification. One is a minimum, not a
  ceiling: it forbids no second verification a skill asks for, such as the audit's sample of
  a verifier's refutations. A claim whose effect is that nothing is done, a refuted finding, may be
  sampled where a skill says so: the session chose this boundary over verifying every refutation,
  which no goal ranks, and the owner approved it, adding that it "matters most when the
  claims is acted on", and that a finding judged real and not worth acting on "maybe that deserves
  verification. Or maybe not." Such a finding is a decision, which `primer@decided-section` brings
  to the owner. Each claim gets its one independent verification, a verdict is no new claim, and the
  main session's own claims are verified by the review before the merge, so no chain is open.

### `thread@dispatch-judgement@absolute-dispatch-conditions`

One sentence in the admission standard,
`skill@knowledge-architect-agent-configuration@admission-standard`, an illustration of its shape:
"An instruction fixes a dispatch only on a property of the step that holds whatever the task's
size, such as a point of view apart from the session's, an output verified before it is used, or an occasion
most work meets; any other fixed dispatch is judged by the primer's section on dispatching work."
It binds a project's own skills as it binds the installed text.

### `thread@dispatch-judgement@retrospective-dispatch-questions`

Three standing questions, after the existing ones in the retrospective skill. An illustration of
their shape:

```markdown
- Did this session carry work an agent could have carried, spending context its design or its
  exchange with the owner needed? For each: the work, and what it cost.
- Did a dispatched agent return without finishing its piece, or with a report the session could not
  act on without redoing the work? For each: the brief, and what was missing.
- Did this session act on a returned conclusion that proved wrong after its independent
  verification, or verify a conclusion again where no skill asked for it? For each: the conclusion,
  and who verified it.
```

Each watches a decision whose failure shows in real sessions first, per
`design@agent-skills@premortem-as-watch-points`: the first and the second watch
`design@agent-skills@work-divided-by-judgement`, the second also bringing the evidence the issue
`issue@agent-skills@dispatched-agents-write-their-report-as-they-work` waits for; the third watches
`design@agent-skills@at-least-one-independent-verification`. The questions are watches that head argues, not instructions on
how a working session behaves, so `design@agent-skills@behaviour-additions-need-real-use` asks
nothing more of them.

### The pointer at implementation's start

One sentence at point 3 of `skill@knowledge-architect-planning@working-a-slice`, an illustration:
"Who carries the implementation, the session or agents it dispatches, is judged as the primer's
section on dispatching work says." Step 1 writes the pointer as the primer reference of the new
section, `primer@dispatching-work`, once the install defines it.

## Mapping tables

The work needs none.

## Losing alternatives

- **No text, relying on `primer@room-to-judge`**, lost to `thread@dispatch-judgement@dispatch-judgement-in-primer`:
  the owner's real-use report shows the judgement failing, and the harness's default leans against
  dispatching.
- **Guidance in each activity's skill**, the development skill's included, lost to the same thread:
  the judgement is met in every activity, and no skill loads at that moment.
- **Absolute thresholds of size or kind**, lost to the same thread: size varies without bound, per
  `design@agent-skills@capabilities-not-structure`.
- **The rule on fixed dispatches in a head and `%%` lines alone**, the session's first placement in
  `thread@dispatch-judgement@absolute-dispatch-conditions`, lost to
  `design@agent-skills@admission-rules-home`, found by the review of this spec: every rule on what
  enters agent-facing text has one home.
- **A token bound in the primer**, from `thread@dispatch-judgement@dispatch-size-hint`, lost to
  `thread@dispatch-judgement@retrospective-dispatch-questions`: the figure a session sees per agent
  has an unestablished meaning, context windows differ by model, and the lesson serves the next
  session.
- **"Carried whole, commits included, never split between two authors"**, the first wording of
  `thread@dispatch-judgement@dispatched-piece-carried-whole`, lost to its rewording: it forbade the
  thaum process.
- **"A check is not checked again", scoped to dispatches no skill names**, the wording of rounds 2
  and 4 of `thread@dispatch-judgement@dispatch-report-standard`, lost to the minimum: as a ceiling
  it collided with seven heads and needed an exemption.
- **Every refutation verified**, lost at a fork to the boundary of
  `thread@dispatch-judgement@dispatch-report-standard`: the session chose the boundary, which no
  goal ranks, and the owner approved it. A wrong refutation loses a finding nothing applies, which
  the audit's next pass and `tripwire@agent-skills@verifier-refutes-a-true-finding` watch, where a
  wrong confirmation lands an edit.
- **A capability sentence in the design skill**, `thread@dispatch-judgement@design-investigation-dispatch`,
  superseded: the section's **The session dispatches** bullet covers it.
- **A compaction question in the retrospective**, lost: a compaction that harmed the work shows in
  the second question, as a piece returned unfinished or a report the session could not act on, and
  one that did not harm it is no failure; asking it would have every retrospective read every
  subagent's transcript, which `klarch transcript show --subagents` prints, for an event that is no
  failure on its own. The first reason given, that a session cannot see an agent's context, was
  wrong: `design@agent-skills@premortem-as-watch-points` adds "unless it reads the subagent's
  transcript".
- **A question on whether checking raised the total cost**, the source issue's lead, refused: no
  session holds the cost the work would have had without the verification, so no session can answer it.
  The counts a session can take of a review's cost are
  `issue@agent-skills@the-retrospective-counts-no-review-cost`'s.

## Readings

The work reads no external specification. It reads the behaviour of external tools:

- Claude Code 2.1.322's Agent tool description, "Do the work yourself when it is a handful of tool
  calls … When in doubt, don't spawn.", read in this session's tool list; recorded in
  `design@agent-skills@work-divided-by-judgement`, with the version;
- in Claude Code 2.1.322, a general-purpose subagent holds `Agent` and no `Workflow`, and the primer
  reaches a nested agent: measured once each in this session; recorded in
  `design@agent-skills@work-divided-by-judgement`, with the probe that takes each again: one general-purpose agent asked
  to list its tools and to dispatch one nested agent that quotes a sentence of the primer from its
  initial context;
- the Messages API is stateless, each request carrying the whole conversation: the ground of "every
  later turn costs more as the context grows".

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| sessions still carry the implementation themselves: the primer dilutes, and the harness's default wins | `thread@dispatch-judgement@dispatch-judgement-in-primer` | converted: the pointer sentence at point 3 of the planning skill's procedure; and watched by the first of the new retrospective questions |
| sessions dispatch lookups, each paying about 36,000 tokens to reload | `thread@dispatch-judgement@dispatch-judgement-in-primer` | dropped: only the owner sees usage, which fails condition 2 of the tripwire standard, `skill@knowledge-architect-planning@acceptance-criteria`, and the owner would see the failure |
| a verifier refutes a true finding, and one verification lets it pass unseen | `thread@dispatch-judgement@dispatch-report-standard` | in the audit, guarded by `tripwire@agent-skills@verifier-refutes-a-true-finding`; elsewhere, no session learns of it, so condition 2 fails; named for the harvest to judge, likely dropped |
| an agent returns its piece unfinished, and "does not take over a part of it" is read as forbidding the session to finish it | `thread@dispatch-judgement@dispatched-piece-carried-whole` | converted into a clause: "What an agent returns unfinished is a new piece." |
| agents dispatched together edit one tree and lose each other's uncommitted work | `thread@dispatch-judgement@dispatched-piece-carried-whole`, `thread@dispatch-judgement@parallel-work-partition` | carried into the issue `issue@agent-skills@how-parallel-work-is-partitioned-among-agents` |

## Acceptance criteria

None: the work is text, and nothing in it is built code whose observation could show a decision
wrong.

## Implementation sequence

The three steps land in one commit or in several, the implementing session's to judge, per
`design@agent-skills@step-is-no-commit`; the `%%` lines that cite the new heads are written in the
commit that writes the heads, since a reference to a head no tree holds fails the check.

### Step 1: the primer section, the pointer and the admission sentence

Builds the section `primer@dispatching-work` in the primer, after `primer@decided-section`; the pointer
sentence at point 3 of `skill@knowledge-architect-planning@working-a-slice`; the sentence of the
admission standard; `%%` lines at each site that states a verification rule, naming the verification head
once it exists: the review skill's "Verify each against the tree before relaying or acting on it",
the planning skill's "Check each finding against the tree, or against the transcript", the record
searcher's "You check your conclusions as the session that sent you checks its own claims", and the
intake's "re-checks every verdict a proposal rests on"; the install. "Carrier" gets no line in
`primer@coined-terms`: the primer's section defines it, per
`design@agent-skills@term-defined-at-first-load`.

Claims, each tested by reading, a phrase search being a lead only; a test is shown to discriminate
by naming the site it would fail on had the text been left as it was:

- every fixed dispatch the installed text states fits a property of the step that holds whatever the
  task's size: tested by reading every installed skill and agent for a fixed dispatch;
- no installed instruction and the section leave a session without a move that obeys both: tested
  by reading each verification site named above, the audit's verify-then-sample of
  `design@agent-skills@one-run-pass` included, against the verification paragraph;
- `cargo klarch check` passes on the installed copies.

### Step 2: the retrospective questions

Builds the three standing questions in the retrospective skill, after the existing ones; the
install.

Claims: each question is answerable from what a session holds, its own transcript and its
subagents' through `klarch transcript show --subagents`, the briefs it sent and the reports it
received: tested by reading each against `design@agent-skills@premortem-as-watch-points`.

### Step 3: the harvest

Writes every row of the Harvest below, then the `%%` lines of step 1 that cite the new heads.

Claims:

- each head cites what its argument stands on, and each instance named in the Harvest cites
  `design@agent-skills@at-least-one-independent-verification`: tested by `cargo klarch show` of the head, which lists every
  reference to it;
- `cargo klarch check` passes;
- the message of the harvest's commit states every cause of the premortem with its outcome, and
  the report on the acceptance criteria, none.

## Order rationale

- Step 1 before step 2: the questions watch the decisions the section states.
- Step 2 before step 3: the harvest records the questions as members of the watch head.

## Defaults awaiting the owner

- **D1**, on `thread@dispatch-judgement@user-level-alignment`, ruled: the owner, after the review of
  this spec: "D1 agreed". The session aligns the owner's
  user-level instructions, `~/.claude/CLAUDE.md`, section "Using subagents", with the primer
  section, after step 3, reading `~/.utils/README.md` first since the file is a tracked dotfile, and
  puts the diff to the owner before committing it there.

## Harvest

| what | where | when |
| --- | --- | --- |
| the head `design@agent-skills@work-divided-by-judgement` (pair: `thread@dispatch-judgement@dispatch-judgement-in-primer` with `thread@dispatch-judgement@absolute-dispatch-conditions` → `design@agent-skills@work-divided-by-judgement`): its title states the rule, the work divided by a judgement on the relations of its pieces, an installed text fixing a dispatch only on a property of the step that holds whatever the task's size; its body names the three properties of today, the readings with the Claude Code version, and the rivals no text, thresholds, and guidance in each skill; test 3 on the harness's default, test 2 on the fixed dispatches at several sites | a new subdocument, `path@agent-skills@docs/design/dispatching.md`, linked from the README of `register@agent-skills@design` | step 3 |
| the head `design@agent-skills@at-least-one-independent-verification` (pair: `thread@dispatch-judgement@dispatch-report-standard` → `design@agent-skills@at-least-one-independent-verification`), test 2: its body names the self-check measurement, kept in the message of the commit that adds this spec, which it names by its subject; the boundary in the weak form, the session's choice approved by the owner, the owner's hedge kept; and it cites `design@agent-skills@verification-in-primer` | the same subdocument | step 3 |
| the instances cite `design@agent-skills@at-least-one-independent-verification`: `design@agent-skills@session-acts-on-the-verdicts`, its verification rule rewritten to cite it and its title kept; `design@agent-skills@one-run-pass`, `design@agent-skills@verifier-in-the-run`, `design@agent-skills@workflow-axis-lenses`, `design@agent-skills@repair-proposals-audited` and `design@agent-skills@verdicts-reach-the-conformance-review`, each gaining a citation where it states its verification | their homes | step 3 |
| `design@agent-skills@verification-in-primer` gains the section as the place where a returned conclusion's independent verification lives; `design@agent-skills@primer-limit` gains the section as a member, with where the judgement lives; `design@agent-skills@premortem-as-watch-points` gains the three questions, each with what it watches; `design@agent-skills@a-scratch-directory-per-subagent` names the primer section among the texts that state its rule; `design@agent-skills@admission-rules-home` names the admission sentence on fixed dispatches | their homes | step 3 |
| the losing alternatives, each judged by `skill@knowledge-architect-decision-recording` for a `rejected` entry | `register@agent-skills@rejected` | step 3 |
| the premortem's third cause, judged by the tripwire standard | `register@agent-skills@tripwire`, or dropped with its reason in the harvest's commit | step 3 |
| the issue `issue@agent-skills@dispatched-agents-write-their-report-as-they-work`, kind `deferred`, trigger: a retrospective answer to the second dispatch question naming a report the session could not act on. Its body: the owner's proposal of report files an agent writes as it works and a verifier annotates, to fill the gap of reports written in one shot at the end; why it waits, no real session having shown the gap; how it differs from `issue@agent-skills@a-review-run-by-a-workflow-keeps-no-report-in-the-transcript` and `issue@query@a-searcher-fills-its-report-by-a-script-of-its-own`, which it cites; that the harness refuses a subagent's `Write` of a `report.md`, per `design@agent-skills@report-written-through-the-shell` and `design@agent-skills@refused-report-returned-as-text`; and how the shape answers `rejected@agent-skills@ledger-written-during-discussion`, a file an agent writes for its own piece rather than the discussion's ledger | `register@agent-skills@issue` | step 3 |
| the issue `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`, kind `deferred`, trigger: a plan document whose work wants parallel agents. Its body: the owner's report of the thaum split, agents in parallel per Component, no commits during the work, the session committing each part after; the owner's motive, time more than context; the shape the owner named, parts carried in worktrees of their own, committed, then rebase-merged; and the facts: the cold build of a fresh worktree (`rejected@mutate@worktree-holds-the-mutation`), parallel target directories filling a shared tmpfs (`issue@mutate@an-environment-failure-reads-as-caught`), the content-loss risk of a shared tree (`design@knowledge-architect@no-operation-loses-content`), that a durable per-agent worktree is not a scratch one (`design@knowledge-architect@scratch-worktree-outside-git-rules`), that `skill@knowledge-architect-review@review-invariants` gives a worktree to a subagent that judges a commit and runs the checker, a test, a mutation or a built program, the ambiguity `design@agent-skills@step-is-no-commit` meets under a partition, and `issue@check@a-staged-check-compares-installed-files-with-unstaged-shipped-text`; it cites `issue@agent-skills@the-intake-s-search-lands-a-batch-s-whole-neighbourhood-on-one-session` and `issue@knowledge-architect@the-review-s-worktrees-and-scratch-directories-are-made-by-hand` | `register@agent-skills@issue` | step 3 |
| a Workflow entry, a patch, in `Next release`, then `cargo x changelog` | CHANGELOG.md | step 3 |

## Later consequences

- The later discussion on partitioning parallel work starts from
  `issue@agent-skills@how-parallel-work-is-partitioned-among-agents`.
- The alignment of the owner's user-level instructions, per D1.
