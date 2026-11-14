# 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow

- **Received file**: 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow.md, the workflow file of a retrospective of this repository.
- **Version used**: "knowledge-architect built from source: `v0.9.0-379-gf9137c97`, the main commit the session's tree held at its start; the session worked on main, then on one branch, with no uncommitted changes at its start."
- **Analysed against**: main at `v0.10.0-59-g605d1538`.
- **The search's verdicts on the sections this file holds**:

  - `design@agent-skills@primer-limit`: constrains, respect.
  - `design@agent-skills@rarer-checker-rules-by-findings`: constrains, respect.
  - `design@agent-skills@shipped-text-cites-no-entry`: constrains, respect.
  - `design@agent-skills@no-two-installed-instructions-contradict`: constrains, respect.
  - `rejected@agent-skills@two-readings-left-to-judgement`: constrains, respect.
  - `design@agent-skills@plain-text-is-no-repair`: conflicts, check-reversal.
  - `issue@check@cross-project-references`: touches, respect.
  - `design@check@every-path-names-its-anchor`: constrains, respect.
  - `design@check@unanchored-path-first-segment-here`: constrains, respect.
  - `design@knowledge-architect@workflow-findings-stay-here`: constrains, respect.
  - `design@knowledge-architect@committed-findings-analysis`: constrains, respect.
  - `design@model@escape-anchor-elsewhere`: constrains, respect.
  - `design@model@foreign-file-names-its-project`: constrains, respect.
  - `goal@documentation-has-a-structure-and-a-model`: constrains, respect.
  - `design@agent-skills@in-change-path`: constrains, respect.
  - `design@agent-skills@outcome-over-display`: constrains, respect.
  - `design@agent-skills@instruction-record-is-minimal`: constrains, respect.

## premortem-reversal-reads-two-ways: "the premortem runs when reversal touches any of the four" reads as this change's reversal or a later one

Members: 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow W2.

- **a. What it says.** "Which reversal the four nouns of the premortem test": two searchers read a
  premortem as owed because the change reversed a head, the session read a later reversal of the
  decision, and its message called the four "the test of the in-change path", which the paragraph
  denies. The blamed text, `skill@knowledge-architect-design@decision-authority`: "On the in-change
  path, the premortem runs when reversal touches any of the four", the fourth being "a decided thread
  that would have to be REOPENED if this one changed, as opposed to one that merely reads it."
  Proposed: "the premortem runs when a later reversal of this decision would touch any of the four".
- **b. Whether it still applies.** Yes. The paragraph reads so at `f9137c97` (the file's version) and
  at origin/main, `v0.10.0-59-g605d1538`, lines 185 to 187 of
  `path@agent-skills@content/skills/design/SKILL.md`: "On the in-change path, the premortem runs when
  reversal touches any of the four; the four never decide which path is taken. To decide, state what
  reversal touches, in nouns." The head `design@agent-skills@in-change-path` has "The premortem runs
  when reversal touches any of the four things the skill names", line 193 of
  `path@agent-skills@docs/design/plan-documents.md`.
- **c. Whether it is already recorded.** No entry records the misreading. The reading is settled by
  the text's own nouns: each names a cost of changing the decision now being made ("stored data that
  would have to be migrated", "an interface other code or other people already consume", "a decided
  thread that would have to be REOPENED if this one changed"). The record searcher graded
  `design@agent-skills@in-change-path` `constrains` with the same reading: "the premortem's reversal
  is a later reversal of the decision recorded, since the four name costs of reversing it".
- **d. Its kind.** A defect of the text: unclear, an instruction whose words allow two acts,
  confirmed by three readers in one session. Provable by reading; `design@agent-skills@no-two-installed-instructions-contradict`
  has a reading confirmed two ways repaired "even where every act it allows is acceptable". The
  session's naming of the four as the path's test is a lapse against "the four never decide which
  path is taken".
- **e. Its scope.** The installed design skill and the head that states the same clause, in
  `agent-skills`. The rewording reverses nothing: it states the reading the four nouns already fix.
- **f. A better fix.** Nearest rival: no change, since the fourth noun's "if this one changed"
  already shows the reading. It loses because two searchers and the session read it otherwise in real
  use, and the rewording adds no instruction.
- **g. Its route.** A fix in the installed text, at its source under `path@agent-skills@content/`,
  as `path@agent-skills@CLAUDE.md`, section "Editing an installed skill or agent", says; the head's
  clause is rewritten in place under `skill@knowledge-architect-decision-recording`, changing no
  decision.
- **h. The proposal.** Handle now. In `path@agent-skills@content/skills/design/SKILL.md`, the two
  sentences become:

  ```text
  On the in-change path, the premortem runs when a later reversal of this decision would touch any of the four; the four never
  decide which path is taken.
  To decide, state what that reversal would touch, in nouns. The four: stored
  ```

  and in `design@agent-skills@in-change-path`, "The premortem runs when reversal touches any of the
  four things the skill names" becomes "The premortem runs when a later reversal of the decision
  would touch any of the four things the skill names". Instructions added: 0; removed: 0; two
  clauses reworded in the skill, one in the head. Nearest rival: no change, which loses as item f
  says. The argument goes in the commit message, per
  `design@agent-skills@instruction-record-is-minimal`.

Default: handle now, with the rewording above.
Outcome: decided by the session: handle now, with the rewording above. Ground: three readers in one
session took two readings; the four nouns fix the reading the rewording states, so no decision
changes and no instruction is added.

## other-project-path-form-unclear: which form a file of another project takes

Members: 2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow C3.

- **a. What it says.** The session wrote paths of files of another project's repository, read as
  sources, in backticks; "the checker reported each as a path that names no anchor. The session
  wrote them as `path@elsewhere@<path>`, which the check accepts, but the primer leaves open whether
  a file of another project is 'a path the tree does not hold' or text outside the checker's syntax,
  like 'a commit of another project'". It blames `primer@where-knowledge-goes`, "`path@elsewhere@<path>`
  for a path the tree does not hold", and "Text that is not in the checker's syntax, such as a commit
  named by its subject, a commit of another project or a description in words, is outside this
  rule." Proposal: state in that section which form a file of another project takes (0 instructions
  added).
- **b. Whether it still applies.** Yes. Both sentences stand in `path@agent-skills@content/PRIMER.md`
  at `v0.9.0-379-gf9137c97` and at `v0.10.0-59-g605d1538`. The finding the session met, in
  `path@check@src/check/references.rs`, "is shaped like a path and names no anchor", names as its
  repair "`path@elsewhere@<path>` for a path this tree does not hold", and no form for a file of
  another project. The session's paths are on main: the eight spans at the escape anchor in
  `path@knowledge-architect@docs/grounding/crates-io-publishing.md`, each a path of the crates.io
  repository with no project segment, such as its rate limiter's source file under src/.
- **c. Whether it is already recorded.** The answer is recorded:
  `design@model@foreign-file-names-its-project`, present at the file's version: "**A file of another
  project is written with that project's name as its first segment**, as
  `path@elsewhere@<project>/<path>`, and the refusal's repair names that form." The refusal it means
  is the escape anchor's on a path this tree holds, whose repair in `path@model@src/resolver.rs`
  says "for a file of another project, begin the path with that project's name". The session met the
  other finding, whose repair does not name the form, and wrote a form the head does not give,
  which passes because its paths collide with no file here. `design@agent-skills@rarer-checker-rules-by-findings`
  (`constrains`) places such a rule in the finding, not the primer: "The decision stands on the
  premise that a rule a lint reads has a finding that names its repair ... such a rule whose finding
  does not name it reopens the decision for that rule." Here the finding a writer of a foreign path
  meets first does not name it.
- **d. Its kind.** A defect of the checker's text: the finding of an unanchored path omits the form
  `design@model@foreign-file-names-its-project` gives, so the primer's sentence reads two ways to a
  writer who meets it. The grounding file's eight spans are a divergence from that head on main.
- **e. Its scope.** The checker, a finding's repair line; `goal@documentation-has-a-structure-and-a-model`,
  "it says what it names, with one reading". The fix reverses no head: it carries out what
  `design@model@foreign-file-names-its-project` and `design@agent-skills@rarer-checker-rules-by-findings`
  say.
- **f. A better fix.** The file's proposal, a clause of the primer, is the rival
  `design@agent-skills@rarer-checker-rules-by-findings` defeats: it costs every session that loads
  the primer a statement only a writer of a foreign path applies, where the finding delivers it at
  the moment it is needed. A second rival, no change, leaves the head unmet by its own finding,
  which the eight spans on main show. The fix is the finding's repair line.
- **g. Its route.** A change to the checker's text: `skill@klarch-development`, with its tests. It
  owes no changelog entry: a reworded repair line is no migration, no new feature and no change to
  the installed skills, the three tests of `design@knowledge-architect@changelog-entries`. The eight
  spans are a text edit that changes no behaviour.
- **h. The proposal.** Handle now, in two commits on the intake's branch:
  - in `path@check@src/check/references.rs`, the repair of the unanchored-path finding, and its two
    pinned copies, in the tests `an_unanchored_path_shape_s_repair_offers_no_unchecked_form` and
    `an_unanchored_path_in_a_plan_document_is_offered_the_planned_form` of the same file, become:

    ```text
    write `path@<anchor>@<path>`, `register@<anchor>@<register>` for a register's home,
    `path@elsewhere@<path>` for a path this tree does not hold, with the project's name as its
    first segment for a file of another project, as `path@elsewhere@<project>/<path>`, or
    `path@*@<path>` for every component's own copy
    ```

    One clause added to a finding's repair, no instruction removed. Nearest rival: the primer's
    clause, which the head on rarer rules defeats, as item f says.
  - in `path@knowledge-architect@docs/grounding/crates-io-publishing.md`, each of the eight spans
    at the escape anchor takes the project's name, crates.io, as its first segment, as
    `path@elsewhere@crates.io/src/rate_limiter.rs`. Two such spans, one of a file and one of a
    directory, appended to a copy of the minimal mock project, pass `check` under the 0.10.0 release
    binary of this checkout with "PASSED: no findings".
    No instruction added or removed.

Default: handle now, as above.
Outcome: decided by the session: handle now. Ground: `design@model@foreign-file-names-its-project`
gives the form and says the refusal's repair names it; `design@agent-skills@rarer-checker-rules-by-findings`
delivers a rarer rule by its finding and reopens for a rule whose finding does not name it, which
is this finding; the change carries out both heads and reverses none.

## standing-2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow: the standing answers

Members: the section "Standing questions" of
2026-11-12-knowledge-architect-new-crate-placeholder-klarch-workflow, lines 78 to 108. The file
predates the questions on misses, review cost and tools; its counts are in the lines for
`issue@agent-skills@the-review-cost-counts-are-not-compared-yet`.

Carried by a subject, named only:

- the claims written unverified, "a registry's address", "a template holds no code", a file "held by
  no tree", a tool's version, a dependency pin, a measurement of another case, which "caused most of
  its four rounds" (lines 82 to 85): `message-claims-written-unchecked`;
- the two premises put to the owner unchecked, a policy presented as forbidding a rival and a rate
  limit reported per account without its exemption (lines 86 to 90): `owner-premise-unchecked`;
- the rewords of C2, the quotation repairs of C1, and the reviewers' worktrees and scratch
  directories made by hand (lines 93 to 95): `no-command-rewords-branch-messages`,
  `quotation-check-refuses-cut-punctuation` and `reviewer-worktrees-by-hand`;
- "Several agents returned their report as text, since a write with their file tool was refused ...
  the triage brief had not asked for a shell redirect" (lines 102 to 104): `report-file-write-refused`.

Analysed here:

- **The research brief lacked the question that mattered** (lines 99 to 102): it "asked for the
  limits and not for which publishes they count; the agent returned them as per account, and the
  exemption of the publishes at stake surfaced only at the review. The brief, not the agent, lacked
  the question." Kind: a lapse of the session's brief, with its premise then put to the owner
  (carried above). It is no report the session "could not act on without redoing the work": the
  session acted on it, wrongly, and the review corrected the premise. So the trigger of
  `issue@agent-skills@dispatched-agents-write-their-report-as-they-work` is not met.
- **Work an agent could have carried** (lines 97 and 98): the repairs of each round, "no cost
  found". **A conclusion acted on and proved wrong** (lines 105 to 108): a workaround's count of
  re-runs, corrected from the dependency graph before reporting. **Installed file changed**: no.
  **Project skill addition missed**: no. **Backticks** (line 92): none, C3 being a form whose fit was
  unclear, `other-project-path-form-unclear`. **Code rules**: no code. Observations.

Default: no change for the answers analysed here.
Outcome: decided by the session: no change. Ground: the brief's gap is a lapse whose consequence, a
premise put to the owner, is counted under `owner-premise-unchecked`; the deferred trigger on
reports is not met, since the report was acted on without redoing; the rest are observations.

## quotation-check-refuses-cut-punctuation: the report check refuses a quotation cut from its punctuation, and every searcher scripts the repair

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `quotation-check-refuses-cut-punctuation`, which gives the outcome.

## thread-shape-change-owes-no-search: a part of the work that changes after the search, with no new thread, gets no search

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-searcher-check-reviewer-reports-klarch-workflow.md`, in its section on `thread-shape-change-owes-no-search`, which gives the outcome.

## no-command-rewords-branch-messages: the reword of an older commit message, and the move of a record between messages, run by a hand-written script each time

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files.md`, in its section on `no-command-rewords-branch-messages`, which gives the outcome.

## report-file-write-refused: the harness refuses a subagent's write of a report file by its name

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-first-slice-klarch-workflow.md`, in its section on `report-file-write-refused`, which gives the outcome.

## reviewer-worktrees-by-hand: each reviewer's worktree and scratch directory made by hand

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-11-knowledge-architect-goals-second-slice-klarch-workflow.md`, in its section on `reviewer-worktrees-by-hand`, which gives the outcome.

## message-claims-written-unchecked: claims written from memory in commit messages, heads, documents and answers to the owner

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-07-knowledge-architect-intake-forty-files-klarch-workflow.md`, in its section on `message-claims-written-unchecked`, which gives the outcome.

## owner-premise-unchecked: questions put to the owner on premises the session had not checked

Analysed in `path@knowledge-architect@docs/retrospective-reports/2026-11-08-knowledge-architect-quick-fixes-klarch-workflow.md`, in its section on `owner-premise-unchecked`, which gives the outcome.
