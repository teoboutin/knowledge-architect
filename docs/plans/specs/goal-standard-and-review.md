# A goal meets a standard of its own, a fresh reviewer reads every goal draft before the owner rules, and the whole goal set is reviewed inside goal-setting

## Status and audience

This spec plans the work that makes a goal follow a standard, and makes that standard checked.

- **A standard**: what a proper goal is, in eight rules, written once as a fragment that the
  goal-setting skill and a new reviewer agent both host.
- **A reviewer**: a fresh installed agent, `knowledge-architect-goal-reviewer`, which reads every
  version of a goal draft before the owner rules on it, given every relevant statement of the
  owner, and which also serves a row of the review table and a review of the whole goal set.
- **A draft the owner can judge by intent**: each drafted goal is shown with its boundary block,
  the cases it rules out and admits and the sibling goals it touches.
- **A goal wording is ruled on alone**, never inside a batch approval of other items.
- **A review of the whole goal set** is an occasion of the goal-setting skill, not a fourth axis
  of the project audit.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- One default awaits the owner, D1. It is built as written until the owner rules, and the branch
  merges once the owner has, per `skill@knowledge-architect-planning@working-a-slice`.
- The spec and its work share one branch: no step changes what
  `cargo klarch commits` checks in a tree, so `primer@branch-commits` does not ask the spec to merge
  first.

The discussion spans one session. It begins at the owner's message "I want to discuss the goal
setting skill, and a potential goal reviewer and audit." The assembly read that session's
extraction by `cargo klarch transcript show`, found by those words.

## How the work is done

`skill@knowledge-architect-planning`, its section `skill@knowledge-architect-planning@working-a-slice`,
read whole, as it says the work of a spec takes it. The text under
`path@agent-skills@content/` is written under `skill@knowledge-architect-agent-configuration`, read
whole, as `path@agent-skills@CLAUDE.md` says in "Editing an installed skill or agent".

## Names

| name | what it names |
| --- | --- |
| goal-setting | `skill@knowledge-architect-goal-setting`, source `path@agent-skills@content/skills/goal-setting/SKILL.md` |
| the review skill | `skill@knowledge-architect-review`, source `path@agent-skills@content/skills/review/SKILL.md` |
| the planning skill | `skill@knowledge-architect-planning`, source `path@agent-skills@content/skills/planning/SKILL.md` |
| the transcript reviewer | `agent@knowledge-architect-transcript-reviewer`, source `path@agent-skills@content/agents/transcript-reviewer.md` |
| `FRAGMENT_HOSTS` | the table of each fragment and its hosts' install paths in `path@agent-skills@src/lib.rs`, held by the tests `each_fragment_has_its_hosts` and `no_fragment_line_ships_outside_its_hosts` |
| the goals design home | `path@agent-skills@docs/design/goals.md`, the subdocument of `register@agent-skills@design` on goals |
| R1 to R9 | the rewordings of rootpoint's goals, in its repository at /home/teo/dev/rootpoint, listed in "What the work is" |
| D5, D8 | two defaults of thaum's milestone driven-testing, in its repository at /home/teo/dev/games/thaum: D5 reworded a goal and was approved with the milestone's other defaults (thaum commit a62e2296); D8 repaired the conflict D5 had made with another goal (thaum commit 1186dac0) |
| boundary block | per drafted goal, the cases it rules out and the cases it admits, from the project's existing or planned work, and the sibling goals it touches, each with its relation; shown to the owner, never written to a goals home |

## What the work is

**The evidence.** Of rootpoint's 10 goal changes over 8 commits, read in its history with
`git log --follow -p` on each goals home:

| # | goal | phrase at fault → replacement | defect | caught by |
| --- | --- | --- | --- | --- |
| R1 | `minimal-content-set` | "every feature the editor has" → "every feature of slide content … the features other goals require … are not counted against it" | too wide; conflicts with `presenter-mode`, `readable-save-file`, `text-command-protocol` | the milestone's review |
| R2 | `minimal-content-set` | "Collaborative editing is not in scope." → "Editing that would reconcile two changes made at once to the same part …" | too wide, two intents in one goal; conflicts with `live-agent-edits` | the owner |
| R4 | `readable-save-file` | "one human-readable text file" → "a text file, with its images … as files beside it" | too wide | a design discussion |
| R7 | `commands-drive-everything` | "every view it shows" → "every view of the presentation …" | two readings | the design-conformance review |
| R9 | `no-language-no-build` | "no … language … is involved" → "… is involved in authoring through the editor" | too wide; conflicts with `text-command-protocol` | the owner |

The other changes added intent missing from the first drafts, or repaired a measure that pointed at
a plan document. The goal-setting step caught none of the 10. In thaum, D5 reworded the fuzzing
goal as a milestone default, approved among the others, and its dropped qualifier set it against
the goal `coverage-before-speed`, which D8 repaired; thaum commit 85f2b26c replaced "learning agent",
a term the project used in another sense.

**What exists today at each site:**

- goal-setting has four sections: `skill@knowledge-architect-goal-setting@goal-or-not` (outcome, never mechanism, and the lifecycle),
  `skill@knowledge-architect-goal-setting@goal-entry-shape`, `skill@knowledge-architect-goal-setting@drawing-out-intent` (five steps: ask, refine, draft with source marks, the
  owner rules on each goal by its slug, write), and `skill@knowledge-architect-goal-setting@when-it-runs-again`. No step dispatches a
  reviewer, and nothing states what a proper goal is beyond outcome and shape.
- `path@agent-skills@fragments/` holds 20 fragments; none concerns goals. `FRAGMENT_HOSTS` lists
  their hosts.
- `path@agent-skills@content/agents/` holds 12 agents; none reads goals but
  `agent@knowledge-architect-design-conformance-reviewer`, which reads a plan document against them.
- The review skill's table, `skill@knowledge-architect-review@review-axes`, has no row for a diff
  that changes a goals home.
- Two installed texts state that a refinement within a ruling's direction, "stricter", is no
  finding: the transcript reviewer, its list of what each ruling is checked for, and the planning
  skill, `skill@knowledge-architect-planning@plan-reviews`, its paragraph "What their findings
  become". The fragment on a head's shape, `path@agent-skills@fragments/head-shape.md`, says the same of a
  head's title and body; it governs heads only, and this work leaves it unchanged.

**Outside the work:**

- The goals of rootpoint and thaum. Their repair is each project's goal-setting, run as the
  whole-set review once a release ships this work.
- This repository's own goals, including `goal@knowledge-architect@rust-projects-come-first`, whose
  "every check and tool" is a universal under rule 3. A whole-set review after the landing is the
  owner's to ask for.
- The goal-coverage axis of `issue@agent-skills@audit-axes-beyond-the-design-record`, which reads
  each goal against the heads that serve it: another question than a goal's own quality.
- An expectation set for goal-setting, `issue@agent-skills@expectation-sets-for-the-installed-skills`:
  the owner's answer Q1, that the owner approved in a rush, is behaviour the reviewer and the
  boundary block compensate for, so the work does not need one.
- How an agent applies a goal that reads two ways: no instruction, per #goal-reading-in-use.

## What is already decided

**Decisions the design rests on and does not argue again:**

- `design@agent-skills@goals-bind`, `design@agent-skills@goal-lifecycle`,
  `design@agent-skills@heads-stand-on-argument`, `design@agent-skills@questions-go-to-their-authority`,
  `design@agent-skills@owner-word-holds-as-far-as-its-premise`.
- `design@agent-skills@shared-text-fragment`: the standard is a fragment, and its host list is the
  owner's; the owner ruled it in round 2.
- `design@agent-skills@additions-review-agent`: a review with a fixed standard rides in an
  installed agent.
- `design@agent-skills@behaviour-additions-need-real-use` and
  `design@agent-skills@synthetic-evidence-not-built`: no run is built to test the reviewer.
- `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@no-external-handoff`,
  `design@core@owned-namespace-check`, `design@core@harness-entity-names`,
  `design@core@unusable-harness-name-is-a-finding`, `design@core@section-homes-carry-slugs`: the
  new agent's name and sections.

**Decisions the work rewrites**, each with every text `cargo klarch show` lists as citing it:

| decision | the change | its citers, and who updates them |
| --- | --- | --- |
| `design@agent-skills@goal-entry-shape` | its "never states a mechanism" test becomes rule 1 of the standard; the head keeps the entry's shape and points to the standard's head | cited by nothing; the harvest |
| `design@agent-skills@goals-change-through-goal-setting` | its list of occasions gains the whole-set review, and the reviewer runs before every ruling | `path@core@docs/design/run-model.md` line 24 cites it for "no goal, which is the owner's to state"; unaffected; the harvest reads it |
| `design@agent-skills@audit-is-an-activity` | a review of the whole goal set runs inside goal-setting: its argument, two standards in one text, does not reach one standard serving a draft and a whole set | `issue@agent-skills@audit-axes-beyond-the-design-record`, line 9; unaffected, since its goal-coverage axis stays open; the harvest |
| `design@knowledge-architect@changelog-entries` | a Migration entry for a change of the rules recorded content must meet cites the section that brings that content to them, an audit axis's or the goal-setting skill's whole-set review | `agent@klarch-changelog-reviewer`, lines 23 and 42, `path@knowledge-architect@CHANGELOG.md` line 5, `path@knowledge-architect@CLAUDE.md` line 202, `path@agent-skills@docs/design/audit-method.md` line 328, `path@agent-skills@docs/design/audit-record-axes.md` line 50, `path@core@docs/design/reference-lints.md` line 161, `register@knowledge-architect@design`, lines 154, 172 and 243, two issues of `register@knowledge-architect@issue`, `register@xtask@rejected`; step 5 reads the changelog reviewer's two lines against the new wording, the harvest the rest |
| `design@agent-skills@transcript-reviewer-agent` | "the same reading applies wherever fidelity to an approval is judged" gains its exception: a goal's wording wider than the owner's words is a finding | eight sites in `register@agent-skills@design` and two in `register@agent-skills@tripwire` (lines 396 and 405); the harvest reads each against the exception |

**Entries the discussion's search graded `bears`**, 94, each with its outcome:

| entries | outcome |
| --- | --- |
| the goals `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@no-contradiction-in-workflow`, `goal@agent-skills@only-needed-instructions`, `goal@agent-skills@one-skill-per-activity`, `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@any-project-can-adopt-it`, `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@the-owner-s-word-is-verified`, `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@documentation-stays-consistent`, `goal@knowledge-architect@design-with-agents-as-counterparts`, `goal@knowledge-architect@the-workflow-improves-through-real-use` | criteria of the discussion, each met as "Criteria" says |
| `design@agent-skills@goals-bind`, `design@agent-skills@goal-lifecycle`, `design@agent-skills@heads-stand-on-argument`, `design@agent-skills@questions-go-to-their-authority`, `design@agent-skills@owner-word-holds-as-far-as-its-premise`, `design@agent-skills@intent-through-goals-not-overrule`, `design@agent-skills@fork-choice-in-weak-form`, `design@agent-skills@default-ruled-before-merge` | rested on; the reviewer's findings (a) and (b) are the contradiction case of `primer@owner-word-premise`; #goal-set-review's choice is the session's default the owner approved, recorded in the weak form at the harvest |
| `design@agent-skills@goal-entry-shape`, `design@agent-skills@goals-change-through-goal-setting`, `design@agent-skills@audit-is-an-activity`, `design@knowledge-architect@changelog-entries`, `design@agent-skills@transcript-reviewer-agent` | rewritten, as the table above says |
| `design@agent-skills@shared-text-fragment`, `design@agent-skills@restatement-size-test`, `design@agent-skills@context-cost-of-agent-text`, `design@agent-skills@primer-limit`, `rejected@agent-skills@directive-restated-everywhere` | constrain the delivery: the standard is one fragment, the primer hosts nothing of it since only a drafter and a judge apply it, and planning and review restate #goal-edit-ruled-alone in one sentence each with its pointer |
| `design@agent-skills@content-mirrors-the-install-layout`, `design@agent-skills@shipped-text-line-comments`, `design@agent-skills@shipped-text-cites-no-entry`, `design@agent-skills@no-external-handoff`, `design@agent-skills@saved-workflow-name-is-its-file-stem`, `design@core@owned-namespace-check`, `design@core@harness-entity-names`, `design@core@unusable-harness-name-is-a-finding`, `design@core@section-homes-carry-slugs`, `design@core@an-entry-is-a-heading-at-the-register-level` | constrain steps 1 and 2; the work adds no saved workflow, so the stem rule meets nothing |
| `design@agent-skills@audit-method`, `design@agent-skills@audit-passes-repeat-while-the-count-falls`, `design@agent-skills@convergence-count-scope`, `design@agent-skills@one-run-pass`, `design@agent-skills@apply-stays-with-the-session`, `design@agent-skills@audit-axis-runs-a-workflow`, `design@agent-skills@committed-audit-report`, `design@agent-skills@audit-outcomes`, `design@agent-skills@verifier-in-the-run`, `design@agent-skills@audit-triggers`, `design@agent-skills@audit-agent-per-axis`, `design@agent-skills@audit-repairs-the-whole-record` | bear on shape B of #goal-set-review only, which the owner ruled out; shape A adds no axis, so none applies; `design@agent-skills@audit-triggers` is met by the Migration entry naming the whole-set review |
| `rejected@agent-skills@audit-without-calibration-or-recheck`, `rejected@agent-skills@commit-messages-as-audit-record`, `rejected@agent-skills@head-repaired-only-when-touched`, `rejected@agent-skills@one-generic-auditor-agent` | not brought back: each finding of a whole-set review becomes a draft the reviewer reads again, which is the re-check; one agent reads the whole set, so no calibration across agents arises; with no passes, no evidence is needed to classify findings across them; the whole-set review reaches goals no change touches; the reviewer agent carries the standard |
| `issue@agent-skills@audit-axes-beyond-the-design-record` | touched: its goal-coverage axis stays open, as "What the work is" says |
| `issue@agent-skills@no-check-compares-an-audit-pass-s-owner-list-count-with-the-previous-pass-s`, `issue@agent-skills@two-audit-workflows-can-collide-group-labels-and-list-a-sample-entry-twice`, `issue@agent-skills@whether-a-saved-workflow-run-needs-a-permission-rule` | bear on a new saved workflow only; the work adds none |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | worsened: its fourth case, an agent type a crossed version adds, now has a second instance, the goal reviewer at a pin move's whole-set review; the harvest adds that instance to the entry |
| `design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@title-states-the-rule`, `design@agent-skills@one-decision-per-head`, `design@agent-skills@existing-heads-on-touch`, `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@a-head-is-owed-by-an-entry-test`, `design@agent-skills@instruction-record-is-minimal` | constrain the harvest, which loads `skill@knowledge-architect-decision-recording` |
| `rejected@agent-skills@head-split-by-independent-reversal` | not brought back: rule 4 tests whether each part means something without the other, the head standard's own exception rule, not independent reversal |
| `rejected@agent-skills@owner-intent-entry-test` | not brought back; its doubt, goals homes swelling with local preferences, is premortem cause 2, and #exclusion-names-its-rival answers part of it |
| `design@agent-skills@outcome-over-display`, `design@agent-skills@capabilities-not-structure`, `design@agent-skills@statement-names-an-act`, `design@agent-skills@admission-rules-home`, `design@agent-skills@expectation-set-bounds-scope` | constrain the text of steps 1 to 4; the boundary block changes what the owner rules on, so it is outcome, not display |
| `issue@agent-skills@expectation-sets-for-the-installed-skills` | touched, not closed, as "What the work is" says |
| `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use` | bears on nothing: no census shaped the standard |
| `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see` | bears on the premortem: a cause on how agents draft or review is admitted only where its evidence lands in a retrospective file or the history, judged at the harvest |
| `issue@agent-skills@a-skill-for-creating-a-component` | touched: rule 7 answers how a new Component's goal relates to the root's; the issue stays open |
| `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes` | touched: the heads the harvest writes name rootpoint and thaum as the source of their counts, per `instructions@verify-before-relying`; the issue stays open |
| `issue@agent-skills@this-repository-s-heads-await-the-design-record-axis` | bears on nothing the work changes |
| the issue `what-a-proper-goal-is-has-no-design` | closed by the commit that adds this spec |
| `design@knowledge-architect@versioning-policy`, `design@knowledge-architect@the-branch-writes-its-changelog-entries` | constrain step 5: skill changes are a patch, with a Migration entry |
| `design@agent-skills@ruled-items-labelled` | constrains step 3: each goal carries its slug, so a boundary block takes no label; a question of intent left in the conversation takes `Q` |
| `design@agent-skills@plan-read-against-the-record`, `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`, `design@agent-skills@review-judges-additions`, `design@agent-skills@additions-review-agent`, `design@agent-skills@misstated-ruling-rated-by-what-it-changes`, `design@agent-skills@workflow-axis-lenses` | constrain step 3; a goal edit inside a review repair goes through goal-setting, which dispatches the reviewer, so the re-review head is unchanged; L8 of the agentic-workflow axis meets one fragment for the reviewer's three modes, so no two standards diverge |

**Entries graded `may-bear` that the grounding read**, each with its outcome:

- `design@agent-skills@one-home-for-head-rules` and `rejected@agent-skills@head-rules-in-decision-recording`,
  `rejected@agent-skills@head-rules-one-fragment`: the goal standard is its own fragment and
  restates no rule on heads; the primer hosts none of it, since reading a goal needs no rule of it.
- `design@agent-skills@attribution-replaces-quotation`: the commit that writes a goal attributes the
  ruling, as today; the owner's statements reach the reviewer in the brief, never a durable record.
- `design@agent-skills@derived-decision-proceeds-on-default`: unchanged, per #goal-reading-in-use.
- `design@agent-skills@losing-shape-test`: a head of absence and an exclusion goal answer different
  questions; rule 8 keeps an exclusion a goal only where it names the design it excludes.
- `design@agent-skills@created-heads-checked-in-the-run`: its parallel for goals is the reviewer's
  run on every version before the ruling.
- `design@agent-skills@search-apart-from-the-standing-state-review`: its argument is two standards
  in one agent; the goal reviewer's three modes apply one fragment, so it does not reach them.
- `design@agent-skills@answer-keeps-its-authority`: a goal's wording is a ruling on intent, never a
  reading.
- `design@agent-skills@tripwire-audit-axis`: its Migration precedent is met by the whole-set review
  section, per the rewritten `design@knowledge-architect@changelog-entries`.

**Entries graded `may-bear` that the grounding did not read**, each with the searcher's reason:

- `design@agent-skills@knowledge-table-home`: shape 5 (when an exclusion statement is a goal, and where it goes when it is not) may need a primer row; no conflict established
- `design@agent-skills@goals-required`: the reviewer on every goal draft adds a step to setup's goal step and contradicts nothing in it
- `goal@knowledge-architect@adoption-is-easy`: a review step at the first goal draft could add a step an adopter follows; no conflict established
- `goal@knowledge-architect@rust-projects-come-first`: its "every check and tool" is an unqualified universal of the kind rule 3 refuses; the work edits no goal
- `rejected@agent-skills@head-rules-whole-in-primer`: placing the standard whole in the primer is the same shape for goals; bears by analogy only
- `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`: a goal in the wrong Component is a goal to change and so the owner's; the issue names heads only
- `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: the work adds a standard applied whole by several texts; the issue's sweep covers existing pointers
- `issue@agent-skills@the-agentic-workflow-audit-s-repair-agents-read-the-admission-standard-by-a-pointer`: a goals audit as a saved workflow would reach its agents by a pointer; no saved workflow is added
- `issue@agent-skills@the-material-finding-duty-has-no-head`: a goal's ambiguity found after its ruling is close to the material-finding duty; no relation established
- `issue@core@a-component-states-at-least-one-goal`: the reviewer adds cost to setup's goal step; its trigger is not met
- `tripwire@agent-skills@open-fork-classed-as-settled`: a goal read on its widest reading can appear to rank shapes it does not; the work makes no such decision
- `tripwire@agent-skills@pointer-not-followed`: counts only if the standard reaches a host by a pointer; it reaches each by the fragment
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: more goal questions reach the owner; a two-reading goal answers neither reading, so no firing is established
- `rejected@agent-skills@manual-fallback-file-per-axis`, `rejected@agent-skills@plain-confirmed-findings-count`, `rejected@agent-skills@workflow-script-inline-in-skill`: bear on a goals axis only
- `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`: a goals axis could join a move's closing audit; none is added
- `tripwire@agent-skills@axis-not-run-at-a-pin-move`, `tripwire@agent-skills@verifier-refutes-a-true-finding`, `tripwire@agent-skills@verifiers-wave-findings-through`: count passes of an axis; none is added
- `design@core@orphan-rejected-entry`: an exclusion goal and a head of absence for a design lost to a goal; no bearing established
- `goal@core@declared-instructions-are-checked`: parts of the standard could be checks; the work proposes skill text and a reviewer only
- `goal@knowledge-architect@structure-and-workflow-work-together`: a whole-set reading may ask for a listing command; no need established
- `rejected@core@entry-reachable-by-citing-a-goal`, `rejected@core@goal-citing-a-rejected-entry`: an exclusion goal could cite the rejected designs it excludes; not proposed
- `issue@agent-skills@the-design-record-auditor-s-return-names-two-verdict-vocabularies`: a reviewer modelled on that auditor could copy its two vocabularies; no return format is stated yet
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`: with several contributors, whose words a goal must not exceed is that issue's question
- `design@agent-skills@bounded-path-in-design`, `design@agent-skills@extension-follows-the-ground`: cite a seed; no element changes them
- `rejected@agent-skills@title-states-built-instance`: narrowing a universal could push a goal toward listing instances; about head titles
- `issue@agent-skills@the-retrospective-counts-no-review-cost`: the reviewer adds dispatches nobody counts
- `tripwire@agent-skills@rule-title-wider-than-its-argument`: the harvest's heads count toward it; fires only on a finding no review has made
- `design@core@nothing-of-a-project-is-compiled-in`: cites a seed for what `init` writes; the work changes no `init`
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`: its mechanism bears on where an instruction applied mid-session is delivered
- `design@agent-skills@a-reference-claims-a-revisit`: no row for a goal citing the goal it refines or ranks against; no shape says goals cite each other beyond rule 7's existing reference
- `design@agent-skills@term-defined-at-first-load`, `tripwire@agent-skills@coined-term-without-an-entry`: only if the primer uses a term the standard coins; it does not
- `rejected@agent-skills@admission-standard-in-primer`: the standard is not placed in the primer
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: bears on audit report files; shape A writes none
- `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`: bears on a new saved workflow; none is added
- `design@core@installed-entities-from-the-tree`: a reference to the new agent resolves against its installed copy, which step 2's install writes
- `design@agent-skills@design-record-axis`, `design@agent-skills@design-record-axis-dispatch`, `design@agent-skills@calibration-in-the-run`: models for a goals axis; none is added
- `design@agent-skills@tripwire-recording-authority`, `design@agent-skills@tripwire-stands-on-the-standard`, `design@agent-skills@tripwire-admission-test`, `design@agent-skills@tripwire-horizon`: the harvest's judgement of the premortem's causes
- `design@agent-skills@plan-reviewed-as-a-commit`: the reviewer reads a draft before the ruling, uncommitted; whether it is a review-skill reviewer is not established
- `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`: the reviewer reads the owner's words, which the brief carries
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: gathering every goal may be several steps; not established
- `design@core@backticked-bare-id`, `design@core@plain-word-lint`: bear on the names chosen for slugs and the agent
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: the standard's numbered rules cited in words are checked by nothing
- `design@agent-skills@premortem-as-watch-points`: the work could add a standing retrospective question; none proposed
- `design@agent-skills@standing-argument-in-head`, `design@agent-skills@unargued-approval-is-argued`, `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`, `tripwire@agent-skills@owner-fact-recorded-unattributed`: cite a seed; no element meets them
- `design@agent-skills@searcher-grades-goals`, `design@agent-skills@search-relevance-grade`: the boundary block stays out of the goal entries, so the goals homes' size is unchanged
- `design@agent-skills@a-scratch-directory-per-subagent`: only if several reviewers are dispatched together
- `design@agent-skills@transcript-extraction-is-the-checker-s`, `design@agent-skills@transcript-found-by-listing`: only if the reviewer extracts the owner's words from a transcript; the session passes them in the brief
- `rejected@agent-skills@transcript-reviewer-reports-every-clause`: only if the check of reach went to the transcript reviewer; a separate reviewer carries it
- `design@agent-skills@workflow-axis-corpus`, `design@agent-skills@workflow-axis-finding-standard`: goals are outside that axis's corpus
- `design@agent-skills@workflow-auditor-agent`: records the owner's worry about publishing too many agents, for that axis only
- `rejected@agent-skills@two-readings-left-to-judgement`: rule 2 asks one reading of a goal, close to how that entry lost

## Criteria

### The owner rules on every goal wording, and no reviewer edits a goal `##c-owner-rules-goals`

Binding, from `goal@knowledge-architect@the-owner-decides` and `design@agent-skills@goal-lifecycle`.
Met by #goal-reviewer-agent, which reports and never edits, and #goal-set-review, whose findings
each become a draft the owner rules on.

### The reach of a draft is checked before the owner rules `##c-reach-checked-before-ruling`

Binding, from `goal@knowledge-architect@the-owner-s-word-is-verified` and `primer@owner-word-premise`:
a premise the drafter supplied, the reach of its wording included, is checked before the ruling.
Met by #goal-reviewer-agent and #goal-draft-shows-reach.

### The reviewer and the whole-set review judge by the standard the drafter meets `##c-one-standard`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Met by #goal-standard, one fragment
hosted by goal-setting and the reviewer, whose three modes all apply it.

### Each new instruction names an act, and no structure the work does not need `##c-only-needed`

Binding, from `goal@agent-skills@only-needed-instructions`. Met: one agent, one fragment, no audit
axis, no instruction for #goal-reading-in-use, no rule for splitting a large goal set.

### No mock goal set is built to test the reviewer `##c-real-use`

Binding, from `goal@knowledge-architect@the-workflow-improves-through-real-use`. Met: the work's
checks are the build, the fragment tests and the gates; the reviewer's first run is a real goal
change.

### The installed text names no project `##c-installed-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met: the rules' examples are
written with placeholders; rootpoint and thaum appear only in `%%` comments and the record.

### The owner's reading time per round of goal rulings `##c-owner-attention`

Weighed. Unmet and accepted: the boundary block costs about three lines per goal. The owner's
answer to Q1 argues the cost buys the attention that was missing; the owner accepted the table that
named this criterion, in round 3.

### A goals home stays short `##c-goals-stay-short`

Binding as a presumption, from `design@agent-skills@goal-entry-shape`. Met: the boundary block never
enters a goals home, and rule 2's qualifiers add about one line to a goal, as R2 and R9 did.

## Threads

### A proper goal meets eight rules, written once as a fragment hosted by goal-setting and the reviewer `##goal-standard`

Proposed by the session in round 1 with seven rules; rule 8 is #exclusion-names-its-rival's, joined
to the fragment at assembly since the reviewer applies it. Approved in round 2, rule 3 reopened by
the session's material finding in round 2 on the owner's argument #arg-34, and closed in round 3.
Arguments: #arg-2, #arg-4, #arg-5, #arg-6, #arg-14, #arg-15, #arg-16, #arg-17, #arg-18, #arg-34,
#arg-41. Its shape is "Decided design", the standard; its harvest is a head in the goals design home.

The owner, round 2: "Agreed on goal-standard and the delivery method as a fragment in skill and
reviewer." The owner, round 3: "After these refinements,  agree with the whole table, you can
proceed to the spec".

### Each drafted goal is shown with its boundary block, and the owner's statements go to the reviewer, not into the display `##goal-draft-shows-reach`

Proposed by the session in round 1 with the owner's words shown beside each goal; the owner objected
in round 2 (#arg-34, #arg-35); the session withdrew the quotation from the display and kept the
block (#arg-42, #arg-43). Closed in round 3. Arguments: #arg-12, #arg-19, #arg-20, #arg-34, #arg-35,
#arg-42, #arg-43. Its shape is "Decided design", the draft; its harvest is a head in the goals
design home.

The owner, round 2: "goal-draft-shows-reach: I'm not completely in favor of having a verbatim
recorded in goals. Most of the time, the intent is hidden behind several statements of mine, so a
short verbatim would not transmit much." … "I understood your proposal as recording a verbatim in
the goal, but I might have misunderstood." The owner, round 3: "After these refinements,  agree
with the whole table, you can proceed to the spec".

### A fresh installed reviewer reads every version of a goal draft before the owner rules `##goal-reviewer-agent`

Proposed by the session in round 1; approved in round 2 with the owner's two additions, the
owner's statements that can be wrong or conflict, and several goals at once (#arg-36, #arg-37),
which the session built into the reviewer's findings (a) and (b) and the whole-batch run
(#arg-44, #arg-45). Arguments: #arg-13, #arg-21, #arg-22, #arg-23, #arg-24, #arg-36, #arg-37,
#arg-44, #arg-45. Its shape is "Decided design", the reviewer; its harvest is a head in the goals
design home, and the rewrite of `design@agent-skills@transcript-reviewer-agent`.

The owner, round 2: "goal-reviewer-agent: agreed on the dispatch before showing the draft. And on
having the agent receive every relevant verbatim of mine, this is important I believe. It is
important to keep in the loop the idea that my verbatim statements can be wrong or
contradict/conflict an existing goal, and such a case must not be brushed aside. The case were
multiple goals are being added or reworded at once should also be handled."

### A diff that changes a goals home owes the goal reviewer at the review before the merge `##goal-review-before-merge`

Proposed by the session in round 1. Approved in round 2. Arguments: #arg-25. Its shape is
"Decided design", the review row; its harvest is a head in `path@agent-skills@docs/design/reviews.md`.

The owner, round 2: "goal-review-before-merge agreed."

### A goal wording is ruled on alone, never inside a batch approval of other items `##goal-edit-ruled-alone`

Proposed by the session in round 1. Approved in round 2. Arguments: #arg-26. Its shape is "Decided
design", the ruling; its harvest is a head in the goals design home.

The owner, round 2: "goal-edit-ruled-alone agreed"

### The whole goal set is reviewed inside goal-setting, by the goal reviewer, not by an audit axis `##goal-set-review`

Proposed by the session in round 1 as a fork, shape A its default and shape B a fourth audit axis.
The owner chose shape A in round 2, the session's default. Arguments: #arg-7, #arg-8, #arg-27,
#arg-28, #arg-29, #arg-39, #arg-46. Its shape is "Decided design", the whole-set review; its harvest
is a head in the goals design home, recorded in the weak form as the session's choice the owner
approved, per `design@agent-skills@fork-choice-in-weak-form`, and the rewrites of
`design@agent-skills@audit-is-an-activity` and `design@knowledge-architect@changelog-entries`.

The owner, round 2: "goal-set-review: agreed on shape A here." On Q2: "I expect that root goals
that refine no other goals would stay in the tens. When factoring refining goals, I think this can
get larger. Anyway, I think a single agent should be able to read at least 100 goals, or probably
more, as long as it does only that."

### A statement of what the project is not for is a goal only when it names the design it excludes `##exclusion-names-its-rival`

Proposed by the session in round 1, from the owner's remark #arg-9. Approved in round 2. Arguments:
#arg-9, #arg-30, #arg-31. Its shape is rule 8 of the standard; its harvest is the standard's head.

The owner, round 2: "exclusion-names-its-rival agreed"

### An agent that meets a goal reading two ways gets no new instruction `##goal-reading-in-use`

Proposed by the session in round 1. Accepted in round 2. Arguments: #arg-32. Its shape is the
absence of an instruction; its harvest is a sentence of the standard's head, a decision about an
absence, which entry test 2 admits only if the harvest finds a reason that must be respected at no
site; otherwise the commit message.

The owner, round 2: "goal-reading-in-use accepted"

## Arguments

### The drafted wording reaches wider than the intent, or reads two ways, and neither party notices `##arg-1`

Round 1, the owner; bears on #goal-standard, #goal-draft-shows-reach, #goal-reviewer-agent. "my
intent is correctly understood, but the proposed wording reaches wider than I intended, or has
multiple readings, and it can be very hard to notice at first( for me as well as for the drafting
agent)".

### A goal admits one reading only `##arg-2`

Round 1, the owner; bears on #goal-standard. "A goal must have no ambiguity in the wording, it must
not admit multiple readings. This is probably one of the most important point."

### rootpoint is the first project started on the workflow from its first commit `##arg-3`

Round 1, the owner; bears on #goal-standard. "Rootpoint is the first project that starts from
scratch using knowledge-architect from the beginning".

### Goals do not conflict with each other `##arg-4`

Round 1, the owner; bears on #goal-standard, #goal-set-review. "Goals also  should not contradict
or conflict with each others."

### One goal holds one intent `##arg-5`

Round 1, the owner; bears on #goal-standard. "A single goal should not conflate  two intents , even
if they are similar or share the subject."

### The goal standard is close to the head standard `##arg-6`

Round 1, the owner; bears on #goal-standard. "goals must adhere to a standard that is auite close
to the design heads standard, with just a few differences."

### A standard, a reviewer at every edit, and perhaps an audit `##arg-7`

Round 1, the owner; bears on #goal-standard, #goal-reviewer-agent, #goal-set-review. "with a
reviewer dispatched everytime a goal edit is done, and maybe an audit methodology can be proposed
(though goals are usually small, so maybe just dispatching the reviewer manually is enough)."

### The goals' consistency together matters `##arg-8`

Round 1, the owner; bears on #goal-set-review, #goal-standard. "Consistency of goals together is
also very important."

### Minimal content is no goal in itself `##arg-9`

Round 1, the owner; bears on #exclusion-names-its-rival. "Minimal content is not a goal in itself,
no content will get implemented without me asking for it anyway."

### The aim is intent transmitted so that agents work autonomously in intended directions `##arg-10`

Round 1, the owner; bears on #goal-standard, #goal-draft-shows-reach. "so that agents can work
autonomously and propose designs that follow intended directions, based on real usage."

### More of the owner's intent now lands in goals `##arg-11`

Round 1, the owner; bears on #goal-standard. "the lst two PR, with the addition of the
design-with-agents-as-counterparts goal, have shifted a good part of the owner intent toward goals
rather than decision record."

### The mechanism: a bounded phrase becomes an unqualified universal, read for intent and applied at its widest `##arg-12`

Round 1, the session; bears on #goal-standard, #goal-draft-shows-reach. "The drafter turns a phrase
of yours, which was bounded by its context, into a universal with no qualifier ("every feature",
"not in scope", "no language", "one file"). You then read the draft to see whether it says what you
mean, not to see what else it rules out. And since goals bind outright, every later agent applies
the widest literal reading."

### Excess reach showed as sibling conflicts, and goal-setting caught none of the 10 defects `##arg-13`

Round 1, the session; bears on #goal-standard, #goal-reviewer-agent. "In 3 of the 4 cases of excess
reach (R1, R2, R9), the excess was a conflict with a sibling goal." "The goal-setting step caught
none of the 10 defects. Reviews caught 4, you caught 3."

### The rules, each with the case that needs it `##arg-14`

Round 1, the session; bears on #goal-standard. The seven rules as drafted, each followed by its
case from rootpoint or thaum.

### The head split test is not reused for goals `##arg-15`

Round 1, the session; bears on #goal-standard. "The rival test rejected for heads, "would reversing
one part leave the other standing", is not reused, since it splits a rule from its exception".

### A goal differs from a head: the owner's word, the owner's wording, binding outright, a measure, sibling agreement `##arg-16`

Round 1, the session; bears on #goal-standard. "it stands on the owner's word; it has no argument
and no rival"; "its wording is the owner's ruling"; "it binds outright"; "it has a measure (rule
6), and must agree with every sibling goal (rule 5)".

### The standard is a fragment, and the primer hosts none of it `##arg-17`

Round 1, the session; bears on #goal-standard. "The primer does not host it: only a drafter and a
judge of a goal apply it."

### The standard's cost is a longer goal, mildly `##arg-18`

Round 1, the session; bears on #goal-standard. "A goal drafted under the standard may be longer,
since rules 2 and 3 add qualifiers. That weighs against c-goals-stay-short, mildly: the R2 and R9
rewords each added about one line."

### The boundary block turns reach into cases the owner judges by intent `##arg-19`

Round 1, the session; bears on #goal-draft-shows-reach. "You read for intent, and the boundary
block turns the reach into cases you can judge by intent."

### The reviewer alone loses: whether a case was meant is a question of intent `##arg-20`

Round 1, the session; bears on #goal-draft-shows-reach, #goal-reviewer-agent. "whether "every
feature" was meant to include presenter mode is a question of intent, which only you answer. The
reviewer can find the case. It cannot rule on it."

### The reviewer runs before every ruling, the owner's rewordings included `##arg-21`

Round 1, the session; bears on #goal-reviewer-agent. "goal-setting dispatches it **before every
ruling**: on the first draft, and again on every version you reword, since step 4 shows a reworded
goal again, and today nobody checks your own rewording."

### A fresh agent, since the drafter is anchored on its reading `##arg-22`

Round 1, the session; bears on #goal-reviewer-agent. "the drafter is anchored on its own reading,
which is your report"; "`design@agent-skills@additions-review-agent` already holds that a fixed
standard rides in an agent's definition."

### The reviewer's first run is real use `##arg-23`

Round 1, the session; bears on #goal-reviewer-agent. "its first run is a real one, this
repository's or rootpoint's next goal change."

### The stricter-refinement reading stops at goals `##arg-24`

Round 1, the session; bears on #goal-reviewer-agent. "a goal binds outright, so a stricter goal
rules out work, where a stricter acceptance criterion only adds a check."

### A goal edit can be made inside another activity, as thaum's D5 was `##arg-25`

Round 1, the session; bears on #goal-review-before-merge. "thaum's D5 was a goal rewording put as a
milestone default and approved with the others, outside the step that rules on each goal by its
slug, and it produced the D8 conflict."

### The review row alone catches a batched goal edit only after the approval `##arg-26`

Round 1, the session; bears on #goal-edit-ruled-alone. "the review row alone catches the defect,
but only after your approval, which then costs a second ruling, as D8 did."

### Every goal edit is the owner's, a goal set fits one agent, and per-draft review re-checks each repair `##arg-27`

Round 1, the session; bears on #goal-set-review. "every goal edit is yours anyway, so an audit's
split between edits applied in the branch and items put to you has nothing to split. A goal set
fits one agent's context. And per-draft review already re-checks each repair."

### Shape A narrows two heads whose arguments rest on two standards `##arg-28`

Round 1, the session; bears on #goal-set-review. "Both arguments rest on the case of two standards,
review and audit. Here one standard serves both modes, so the arguments do not reach this case, but
both heads' texts change."

### Shape B touches no head, at the cost of a listed workflow and machinery sized for hundreds of entries `##arg-29`

Round 1, the session; bears on #goal-set-review. "it touches no head, and it fits the existing
Migration path."; "machinery for corpora of hundreds of entries, applied to about 30 goals. And it
would bypass per-goal ruling, or re-implement it."

### An exclusion names the design another goal would argue for; smallness alone is no goal `##arg-30`

Round 1, the session; bears on #exclusion-names-its-rival. "earns a goal only when it rules out a
design that another goal would otherwise argue for, and it names that design."; "either it is your
scope control, which you already exercise by what you ask for; or it is a quality argument a design
head makes against its own rival."

### In rootpoint, the concurrency sentence carried weight and the smallness part did not `##arg-31`

Round 1, the session; bears on #exclusion-names-its-rival. "The concurrency sentence carries the
weight in `one-controller-at-a-time` and `wider-lock`. That is the part that names a rival."; "The
smallness part was used against two rivals whose own heads already argue their loss."; "It forced
`assets-tab` to argue that a feature falls outside the goal."

### Today's routing covers a goal reading two ways in use, and no real case shows otherwise `##arg-32`

Round 1, the session; bears on #goal-reading-in-use. "Today's routing already covers it, since a
conflict with a goal goes to you (`primer@goals-bind`), and R7 shows it working"; "I found no real
case where a wide reading silently blocked a shape."

### Whether the owner read for intent or for reach decides the block's weight `##arg-33`

Round 1, the session's question Q1; bears on #goal-draft-shows-reach. "If you already read for
reach and still missed it, the boundary block helps less, and the reviewer carries more weight."

### Goal-setting distils a larger intent from several statements, and the distilled form is the record `##arg-34`

Round 2, the owner; bears on #goal-draft-shows-reach, #goal-standard. "The value of the goal
setting skill is that it is supposed to help the agent distill the statements I made to extract the
larger, long term intent behind them. Though it often goes through a round where I state the intent
more directly. But the record value lay more in the distilled form IMO."

### A short quotation transmits little, and may misstate the intent `##arg-35`

Round 2, the owner; bears on #goal-draft-shows-reach. "Most of the time, the intent is hidden
behind several statements of mine, so a short verbatim would not transmit much."; "Especially since
the draft review might show my wording to mistate the intent I believe."

### The owner's statements can be wrong or conflict with a goal, and must not be brushed aside `##arg-36`

Round 2, the owner; bears on #goal-reviewer-agent. "my verbatim statements can be wrong or
contradict/conflict an existing goal, and such a case must not be brushed aside."

### Several goals added or reworded at once `##arg-37`

Round 2, the owner; bears on #goal-reviewer-agent. "The case were multiple goals are being added or
reworded at once should also be handled."

### The owner approved rootpoint's goals in a rush, trusting well-aligned proposals `##arg-38`

Round 2, the owner, answering Q1; bears on #goal-draft-shows-reach, #goal-reviewer-agent. "I
approved the goals of rootpoint in a rush, without thinking too much of it. The exact reach was a
bit hard to determine too, I think. In the design discussion, the session had seemed to have really
gotten my intent, because its proposals were truly well aligned. So I did not expect the goals to
be misalligned or overstated, and did not read them seriously enough."

### Root goals stay in the tens, and one agent reads at least 100 goals `##arg-39`

Round 2, the owner, answering Q2, a report the owner holds; bears on #goal-set-review. "I expect
that root goals that refine no other goals would stay in the tens. When factoring refining goals, I
think this can get larger. Anyway, I think a single agent should be able to read at least 100
goals, or probably more, as long as it does only that. goals are relatively short in most cases."

### The quotation in the display is withdrawn; the owner's statements go to the reviewer `##arg-40`

Round 2, the session; bears on #goal-draft-shows-reach. "Your words go to the reviewer only, all the
relevant ones. You agreed on that. Nothing of them reaches the draft display, the goals home or the
commit."

### Rule 3 as drafted forbids the distillation goal-setting is for `##arg-41`

Round 2, the session's material finding; bears on #goal-standard. "As I drafted it, "It reaches no
wider than the owner's statement", the rule forbids the very thing you say goal-setting is for".
"The defect in R1, R2, R4 and R9 was not that the goal generalised. It was that the
generalisation, and what it ruled out, never reached you as such."

### The owner's rushed reading makes the block weigh more `##arg-42`

Round 2, the session; bears on #goal-draft-shows-reach. "The block puts the reach in front of you
as cases. A line like `sibling goals touched: presenter-mode … CONFLICT` is hard to skim past. A
prose goal is easy to skim past."

### The display was never meant for the goals home `##arg-43`

Round 2, the session; bears on #goal-draft-shows-reach. "I did not propose writing your words into
a goal. The draft said "per goal and never written to the goals home"."

### The reviewer reports the owner's statements in conflict, and a part dropped unshown, never resolved silently `##arg-44`

Round 2, the session; bears on #goal-reviewer-agent. "(a) a statement of yours that conflicts with
an existing goal, or with another statement of yours; (b) a draft that drops, or turns around, part
of what your statements say, without showing it."; "The drafter does not pick a side, and no goal
resting on it is written before your answer."

### The reviewer always takes the whole batch, and runs again on any rewording `##arg-45`

Round 2, the session; bears on #goal-reviewer-agent. "It checks each new or reworded goal against
every other new one, and against every existing one. When you reword any goal of the batch, the
reviewer runs again on the whole batch, since one rewording can change a sibling relation."

### No rule for splitting a large goal set before evidence asks for one `##arg-46`

Round 2, the session; bears on #goal-set-review. "No rule for splitting the set is added before
evidence shows it is needed (`goal@agent-skills@only-needed-instructions`)."

## New names, in one place

| name | file | status |
| --- | --- | --- |
| `planned@agent-skills@fragments/goal-standard.md` | the standard's fragment | new |
| `planned@agent-skills@content/agents/goal-reviewer.md` | the reviewer agent's source, installed as the agent `knowledge-architect-goal-reviewer` | new |
| the section `whole-set-review` | goal-setting's source | new |
| `skill@knowledge-architect-goal-setting@goal-or-not` | goal-setting's source | existing; hosts the fragment |
| `skill@knowledge-architect-goal-setting@drawing-out-intent` | goal-setting's source | existing; gains the dispatch and the block |
| `skill@knowledge-architect-goal-setting@when-it-runs-again` | goal-setting's source | existing; names the whole-set review |
| `FRAGMENT_HOSTS` | `path@agent-skills@src/lib.rs` | existing; gains one row |

The agent's sections, each a level-two heading with its slug placeholder, per
`design@core@section-homes-carry-slugs`: its section the-goal-standard (the fragment), `the-modes`,
`what-to-report`. The names are the session's default and may change at step 2.

## Decided design

### The standard

The fragment holds the eight rules below, and goal-setting hosts it in its section
`skill@knowledge-architect-goal-setting@goal-or-not`, where its first rule replaces today's outcome test, so the test has one home. The
lifecycle sentences of `skill@knowledge-architect-goal-setting@goal-or-not` stay in the skill. The wording below is the decided content;
the step may reword it, keeping each rule's meaning, and the reviewers judge the fidelity.

> **A goal is proper when all of these hold.**
> 1. **It states an outcome**: what the Component is for, and for whom. It never states a
>    mechanism, a tool, a check or a derivation: the argument that a design follows from it goes in
>    that design's head, which cites it.
> 2. **It has one reading.** Every term has one meaning in the project; a word the project already
>    uses in another sense is defined or replaced. Every "every", "no", "all" and "not in scope"
>    names the set it ranges over.
> 3. **It rules out nothing the owner has not ruled out knowingly.** Where it generalises the
>    owner's statements, the generalisation and the cases it rules out are shown to the owner
>    before the ruling.
> 4. **It holds one intent.** Two statements are two goals when each means something without the
>    other; a qualifier, a domain or a measure belongs to the goal it qualifies.
> 5. **It contradicts no other goal of the project.** No state of the project meets one goal only
>    by failing another. Where two outcomes compete, a goal states which ranks first.
> 6. **What shows it met stays checkable**: an observable of the project as it stands, never a plan
>    document or anything that leaves, and never the tool that checks it.
> 7. **A refinement neither widens nor contradicts the goal it refines.**
> 8. **A statement of what the Component is not for is a goal only when it names the design it
>    excludes**, one that another goal would otherwise argue for. A property of smallness with no
>    named design is no goal: it is the owner's scope control, exercised by what the owner asks
>    for, or a quality argument a design head makes against its own rival.

Rule 4's test is the head standard's rule on an exception, not the independent-reversal test
`rejected@agent-skills@head-split-by-independent-reversal` records. Each rule's case, R1 to R9, D5
and D8, goes in a `%%` comment beside it, naming the project, so the installed text names none, per
`goal@agent-skills@installed-text-works-anywhere`.

**The nearest rival** of the fragment is the standard written in goal-setting alone, with the
reviewer briefed to read it: it puts the reviewer's standard behind a read the brief must ask for,
which `rejected@agent-skills@one-generic-auditor-agent` records as uneven. The rival rule 3, "no
wider than the owner's statement", lost to #arg-41.

### The draft

Step 3 of `skill@knowledge-architect-goal-setting@drawing-out-intent` writes, per goal, beside its source mark:

- **its boundary block**: for each universal or exclusion, at least one case it rules out and one
  it admits, from the project's existing or planned work; each sibling goal it touches, with the
  relation, a conflict named as such;
- **the reviewer's findings that are questions of intent**, each with the case on which the
  readings differ.

The owner's statements are not shown in the draft, and nothing of the block or the statements
reaches the goals home or the commit, whose message attributes each ruling, as today. An
illustration of the shape, not authority:

```text
<slug>: "<the goal's sentence>"
  rules out:   <case>, <case>
  admits:      <case>
  sibling goals touched:  <slug> (refines), <slug> CONFLICT: <the case where both cannot hold>
  question of intent:  does "<term>" include <case>?
```

**The nearest rival**, the owner shown the repaired draft alone, lost to #arg-20: whether a case
was meant is a question of intent the reviewer cannot rule on. The rival that shows the owner's
statements beside each goal lost to #arg-34 and #arg-35.

### The reviewer

`knowledge-architect-goal-reviewer`, read-only, hosts the fragment, and runs in three modes, each
named by its brief:

- **draft**: dispatched by `skill@knowledge-architect-goal-setting@drawing-out-intent` before the draft is shown, and again on every
  version before a ruling, the owner's own rewordings included. Its brief: the whole batch of new
  and reworded goals, every statement of the owner the session drew them from, verbatim, and every
  goals home of the project. It checks each goal of the batch against the standard, against every
  other goal of the batch and against every existing goal, and reports per goal: each reading with
  the case on which two differ; each generalisation, or case ruled out, the draft does not show;
  each pair of intents; each conflict with another goal; a measure or a refinement that fails its
  rule; the material of the boundary block. It also reports, as contradictions the drafter never
  resolves alone: **(a)** a statement of the owner that conflicts with an existing goal or with
  another statement of the owner; **(b)** a draft that drops or turns around part of what the
  owner's statements say, without showing it.
- **whole set**: dispatched by the section `whole-set-review`. Its brief: every goals home. It
  reports every rule a goal fails and every conflict between two goals.
- **merge**: dispatched by the review row. Its brief: the branch's range. It reports every goal the
  diff adds or changes that fails a rule it can judge without the owner's statements, rules 1, 2
  and 4 to 8, or that conflicts with a goal of the tree, and every goal edit whose ruling no commit
  message of the branch records.

It reports and never edits; each finding the standard settles is the drafter's to repair, and each
question of intent and each contradiction goes to the owner in the draft. A contradiction holds the
goal resting on it, which is not written before the owner's answer, per `primer@owner-word-premise`.
The three modes apply one fragment, so the argument of
`design@agent-skills@search-apart-from-the-standing-state-review`, two standards in one agent, does
not reach them.

The owner's report recorded as a premise: root goals that refine no other goal stay in the tens,
and one agent reads at least 100 goals when it does only that (#arg-39). No rule splits a larger
set (#arg-46).

**The nearest rival**, the drafter checking its own draft against the standard, lost to #arg-22:
the drafter is anchored on its own reading, which the owner reports of both parties.

### The review row

A row of `skill@knowledge-architect-review@review-axes`: "goals | `knowledge-architect-goal-reviewer`,
in its merge mode | the diff changes a goals home". **The nearest rival**, no row, lost to #arg-25.

### The ruling alone

A sentence of `skill@knowledge-architect-goal-setting@drawing-out-intent`: a goal's wording proposed in another activity, a plan's
default, a review's repair or an audit's item, is drafted here as its own item, with its boundary
block, and the owner's word on a batch of other items never covers it. The planning skill, in
`skill@knowledge-architect-planning@plan-reviews`' paragraph "What their findings become", and the review skill, in
`skill@knowledge-architect-review@what-review-leaves`, each restate it in one sentence with the pointer to `skill@knowledge-architect-goal-setting@drawing-out-intent`, per
`design@agent-skills@restatement-size-test`. **The nearest rival**, the review row alone, lost to
#arg-26.

### The whole-set review

A section `whole-set-review` of goal-setting. Its occasions: the owner's request, and a pin move
across a version whose Migration entry names the section. Setup is no occasion, per D1. The session
dispatches the reviewer in its whole-set mode, and each finding becomes a draft that
`skill@knowledge-architect-goal-setting@drawing-out-intent` takes, its draft-mode review included, which is the re-check.
`skill@knowledge-architect-goal-setting@when-it-runs-again` names the section among the occasions the skill runs on.

**The nearest rival**, shape B, a fourth axis of the project audit with its saved workflow, lost to
#arg-27 and #arg-29, on the owner's choice of the session's default.

### The stricter-refinement exception

The transcript reviewer's list of what each ruling is checked for, and the planning skill's
paragraph "What their findings become", each gain one clause: a goal's wording wider than what the
owner ruled out knowingly is a finding, since a goal binds outright and rules out work, and it goes
to `skill@knowledge-architect-goal-setting@drawing-out-intent`. The fragment on a head's shape is unchanged: it governs a head's title and body,
never a goal.

## Mapping tables

Empty: the work needs no total function.

## Losing alternatives

- **The owner's words shown beside each goal in the draft** (#goal-draft-shows-reach), withdrawn
  by the session in round 2: lost to #arg-34 and #arg-35, a short quotation carries little of an
  intent spread over several statements.
- **The repaired draft shown alone, with the reviewer as the only reader of reach**
  (#goal-draft-shows-reach): lost to #arg-20.
- **Rule 3 as "no wider than the owner's statement"** (#goal-standard): lost to #arg-41.
- **The head split test, "would reversing one part leave the other standing", for rule 4**
  (#goal-standard): lost to #arg-15, as `rejected@agent-skills@head-split-by-independent-reversal`
  records for heads.
- **The standard hosted by the primer** (#goal-standard): lost to #arg-17.
- **No review row** (#goal-review-before-merge): lost to #arg-25.
- **The review row alone, with no rule on batched rulings** (#goal-edit-ruled-alone): lost to
  #arg-26.
- **Shape B, a fourth audit axis** (#goal-set-review): lost to #arg-27 and #arg-29, the owner
  choosing shape A, the session's default.
- **Smallness goals kept as written** (#exclusion-names-its-rival): lost to #arg-31.
- **An instruction for an agent applying a goal that reads two ways** (#goal-reading-in-use): lost
  to #arg-32.

## Readings

Empty: the work reads no external specification.

## Premortem

Each cause is named for the harvest to judge by the standard of
`skill@knowledge-architect-decision-recording@ruled-tripwires`, on its wording then:

1. **The reviewer always finds something**, the blocks grow long, the owner skims them, and excess
   reach still passes. Stresses #goal-reviewer-agent and #goal-draft-shows-reach. Its evidence: a
   goal written or reworded under the reviewer and later reworded for a defect a rule of the
   standard names, in the history of a goals home of this repository or in a retrospective file the
   intake receives.
2. **The reviewer pulls distilled goals back down to the owner's statements**, until the goals homes
   hold local preferences, the doubt `rejected@agent-skills@owner-intent-entry-test` keeps.
   Stresses #goal-standard's rule 3 and #goal-reviewer-agent. The owner would see it in a draft.
3. **Rule 4 splits goals until the set passes what one agent reads.** Stresses #goal-standard and
   #goal-set-review. Its basis is the owner's estimate of 100 goals, #arg-39; the count is in the
   tree.
4. **The review row reopens rulings at every merge**, each finding on wording the owner already
   ruled costing a second round. Stresses #goal-review-before-merge. The owner would see it.
5. **An exclusion's named design goes stale**: the goal that argued for it is reworded, and the
   exclusion still rules designs out. Stresses #exclusion-names-its-rival.
6. **Scope control by what the owner asks for fails inside requested work**: an agent over-builds a
   requested feature, and no goal argues against it. Stresses #exclusion-names-its-rival.

## Acceptance criteria

None. Every observable of these decisions is how an agent drafts or reviews, which
`design@agent-skills@synthetic-evidence-not-built` keeps out of an acceptance test. A whole-set
review of this repository's goals after the landing is real work the owner may ask for, not a test.

## Implementation sequence

### Step 1: the fragment, hosted by goal-setting

Builds `planned@agent-skills@fragments/goal-standard.md` with the eight rules and their `%%` cases;
goal-setting's `skill@knowledge-architect-goal-setting@goal-or-not` hosts it, its outcome test removed as the fragment's rule 1;
`FRAGMENT_HOSTS` gains the row naming the installed goal-setting skill;
`cargo klarch install-agent-skills`.

Claims:

- The fragment ships in goal-setting and nowhere else: `each_fragment_has_its_hosts` and
  `no_fragment_line_ships_outside_its_hosts` pass; shown to fail by adding a second host to the row
  with no placeholder line.
- No sentence of the outcome test ships twice in goal-setting's installed copy: a read of the copy.

Fails alone on: the fragment-host tests and the build.

### Step 2: the reviewer agent

Builds `planned@agent-skills@content/agents/goal-reviewer.md`: its frontmatter, read-only rules, the
fragment hosted in its section the-goal-standard, its three modes and its report; `FRAGMENT_HOSTS` gains the
agent's install path; install. The spec's bare name of the agent becomes `knowledge-architect-goal-reviewer`
in the commit that lands it, and the spec's planned references to it become path references.

Claims:

- The agent installs under the owned namespace and its name passes the harness rules:
  `cargo klarch check` passes; shown to fail by a frontmatter `name` that differs from the file.
- One dispatch of the new agent sees its tools granted, the harness mechanics
  `design@agent-skills@synthetic-evidence-not-built` admits; no judgement of its report.

Fails alone on: the check's installed-file and harness-name findings.

### Step 3: the procedure

Builds, in goal-setting: the dispatch and the block in `skill@knowledge-architect-goal-setting@drawing-out-intent`, the re-run on any
rewording and on the whole batch, the contradictions (a) and (b) held for the owner, the ruling
alone; the section `whole-set-review`, named in `skill@knowledge-architect-goal-setting@when-it-runs-again`. In the review skill: the row
of `skill@knowledge-architect-review@review-axes` and the sentence in `skill@knowledge-architect-review@what-review-leaves`. In the planning skill: the sentence in
`skill@knowledge-architect-planning@plan-reviews`. Install.

Claims:

- Every reference the new text writes resolves: `cargo klarch check`.
- Each restatement of #goal-edit-ruled-alone is one sentence with its pointer: a read of both copies.

Fails alone on: the check, and the agent-facing-text review.

### Step 4: the stricter-refinement exception

Builds the clause in the transcript reviewer and in the planning skill's paragraph "What their
findings become". Install.

Claims: the fragment on a head's shape is unchanged, `git diff -- crates/agent-skills/fragments/head-shape.md`
empty; each clause names goals only, a read of both.

Fails alone on: the agent-facing-text review of two passages.

### Step 5: the changelog

Builds the `Next release` entries of `CHANGELOG.md`, per `design@knowledge-architect@changelog-entries`
read whole, as rewritten by the harvest: under Workflow, the reviewer before every goal ruling, the
boundary block, the review row, the ruling alone and the whole-set review; under Migration, the
standard as a change of the rules recorded goals must meet, citing
the new section `whole-set-review`; each a patch on the surface
`agent-skills`. Then `cargo x changelog`, and the changelog reviewer's two citing lines read against
the rewritten head.

Claims: the copies match, by the test `cargo x changelog` keeps; `cargo x gates` exits 0.

Fails alone on: the changelog copy test.

## Order rationale

- The fragment comes first so that the agent of step 2 hosts a fragment that exists.
- The agent comes before the procedure, so that the references step 3 writes to it resolve.
- The procedure comes before the exception, since the exception routes a finding to the procedure.
- The changelog comes last, since it names what steps 1 to 4 built.

## Defaults awaiting the owner

- **D1** (#goal-set-review): setup is no occasion of the whole-set review. The owner approved shape A
  with "after setup" among its occasions. At setup each Component's batch is reviewed in draft mode
  against every goals home written before it, so every pair of goals is checked by the end of setup,
  and a whole-set review then adds a dispatch that checks nothing new. Default: dropped. Dropping it
  relaxes what the owner approved, so it waits for the owner's word.

## Harvest

At the landing, under `skill@knowledge-architect-decision-recording`, read whole:

| thread | home | what lands |
| --- | --- | --- |
| #goal-standard | the goals design home | a head on the standard, its fragment and its hosts; `design@agent-skills@goal-entry-shape` rewritten to point to it |
| #goal-draft-shows-reach | the goals design home | a head on the draft's boundary block, the owner's statements kept out of it |
| #goal-reviewer-agent | the goals design home; `path@agent-skills@docs/design/reviews.md` | a head on the reviewer, its modes and its findings (a) and (b); `design@agent-skills@transcript-reviewer-agent` rewritten with the exception for goals |
| #goal-review-before-merge | `path@agent-skills@docs/design/reviews.md` | a head on the row |
| #goal-edit-ruled-alone | the goals design home | a head on the ruling alone |
| #goal-set-review | the goals design home; `path@agent-skills@docs/design/audit-method.md`; `register@knowledge-architect@design` | a head in the weak form; `design@agent-skills@audit-is-an-activity` and `design@knowledge-architect@changelog-entries` rewritten; `design@agent-skills@goals-change-through-goal-setting` rewritten with the occasion |
| #exclusion-names-its-rival | the standard's head | rule 8 and its argument |
| #goal-reading-in-use | the standard's head, or the commit message | the absence, by entry test 2 if a reason must be respected at no site |
| the losing alternatives | `register@agent-skills@rejected`, where `skill@knowledge-architect-decision-recording` admits each | each with the head that cites it |
| the premortem's causes | `register@agent-skills@tripwire`, where the standard admits each | each with its horizon |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | its own file | the goal reviewer as a second instance of its fourth case |

## Later consequences

- rootpoint and thaum each run the whole-set review when they move their pin across the release, per
  its Migration entry; rootpoint's `minimal-content-set` is expected to split under rules 4 and 8.
- This repository's goals are reviewed on the owner's request, `goal@knowledge-architect@rust-projects-come-first` among the
  first findings expected.
- A set past what one agent reads, if premortem cause 3 fires, reopens #goal-set-review.
