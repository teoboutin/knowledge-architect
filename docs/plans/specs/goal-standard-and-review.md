# A goal meets a standard of its own, a fresh reviewer reads every goal draft before the owner rules, and the whole goal set is reviewed inside goal-setting

## Status and audience

This spec plans the work that makes a goal follow a standard, and makes that standard checked.

- **A standard**: what a proper goal is, in eight rules, written once as a fragment that the
  goal-setting skill and a new reviewer agent both host.
- **A reviewer**: a fresh installed agent, `agent@knowledge-architect-goal-reviewer`, which reads every
  version of a goal draft before the owner rules on it, given every relevant statement of the
  owner, and which also serves a row of the review table and a review of the whole goal set.
- **A draft the owner can judge by intent**: each drafted goal is shown with its boundary block,
  the cases it rules out and admits and the sibling goals it touches.
- **A goal wording is ruled on alone**, never inside a batch approval of other items.
- **A review of the whole goal set** is an occasion of the goal-setting skill, not a fourth axis
  of the project audit.
- **This repository's goals brought into conformance** with the standard, on this branch, by
  step 6, on the owner's word.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point its section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- No default awaits the owner: D1, D2 and D3 are ruled.
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
| R1 to R9 | the changes of an existing goal in rootpoint's history, in its repository at /home/teo/dev/rootpoint, listed in "What the work is" |
| Q1 to Q8 | the questions the session put to the owner, in the conversation: Q5 is D2, Q6 is D3, Q7 whether this repository's goals are brought into conformance on this branch (#arg-50), Q8 how a whole-set review checks rule 3, ruled for every goal's block; and Q1, what the owner read rootpoint's goal drafts for, intent or reach (#arg-33); Q2, whether a project's goal set stays in the tens of goals (#arg-39); Q3, whether the owner accepts by name the cost #c-owner-attention weighs; Q4, the outcome of `goal@knowledge-architect@rust-projects-come-first` |
| D5, D8 | two defaults of thaum's milestone driven-testing, in its repository at /home/teo/dev/games/thaum: D5 reworded a goal and was approved with the milestone's other defaults (thaum commit a62e2296); D8 restored a qualifier whose absence D5's wording carried, which set the goal against another (thaum commit 1186dac0); the qualifier had been dropped earlier by a goal-setting draft the owner approved (thaum commit 04bed4a6) |
| boundary block | per drafted goal, the cases it rules out and the cases it admits, from the project's existing or planned work or the uses the owner's statements describe, and the sibling goals it touches, each with its relation; shown to the owner, never written to a goals home |

## What the work is

**The evidence.** rootpoint's history holds 9 changes of an existing goal over 7 commits, read with
`git log --follow -p` on each goals home, a goal added after the first draft not counted:

| # | goal | phrase at fault → replacement | defect | caught by |
| --- | --- | --- | --- | --- |
| R1 | `minimal-content-set` | "every feature the editor has" → "every feature of slide content … the features other goals require … are not counted against it" | too wide; conflicts with `presenter-mode`, `readable-save-file`, `text-command-protocol` | the milestone's review |
| R2 | `minimal-content-set` | "Collaborative editing is not in scope." → "Editing that would reconcile two changes made at once to the same part …" | too wide, two intents in one goal; conflicts with `live-agent-edits` | the owner |
| R3 | `live-agent-edits` | the window shared with an agent that makes the changes the author asks for, added | intent missing from the first draft | the owner |
| R4 | `readable-save-file` | "one human-readable text file" → "a human-readable text file, with its images and other binary assets as files beside it" | too wide | a design discussion |
| R5 | `runs-in-browser` | "served as a static website", and a packed file opened, added | intent missing | a design discussion |
| R6 | `direct-manipulation` | a measure naming "every UI action of the first prototype's scope" → the list of actions | a measure pointing at a plan document | a review |
| R7 | `commands-drive-everything` | "every view it shows" → "every view of the presentation …" | two readings | the design-conformance review |
| R8 | `runs-in-browser` | "opened and presented" → "opened, presented and saved again" | intent missing | the session's report at a milestone's end |
| R9 | `no-language-no-build` | "no … language … is involved" → "… is involved in authoring through the editor" | too wide; conflicts with `text-command-protocol` | the owner |

Excess reach is R1, R2, R4 and R9, and three of them, R1, R2 and R9, also conflict with a sibling
goal. Reviews or the session caught 4 (R1, R6, R7, R8), the owner 3 (R2, R3, R9), and a design
discussion 2 (R4, R5). The goal-setting step caught none. In thaum, a goal-setting draft the owner
approved dropped a qualifier of the fuzzing goal; D5 then reworded that goal as a milestone default,
approved among the others, still without the qualifier, which set it against the goal
`coverage-before-speed`, and D8 restored it; thaum commit 85f2b26c replaced "learning agent",
a term the project used in another sense.

**What exists today at each site:**

- goal-setting has four sections: `skill@knowledge-architect-goal-setting@goal-or-not` (outcome, never mechanism, and the lifecycle),
  `skill@knowledge-architect-goal-setting@goal-entry-shape`, `skill@knowledge-architect-goal-setting@drawing-out-intent` (five steps: ask, refine, draft with source marks, the
  owner rules on each goal by its slug, write), and `skill@knowledge-architect-goal-setting@when-it-runs-again`. No step dispatches a
  reviewer, and nothing states what a proper goal is beyond outcome and shape.
- `path@agent-skills@fragments/` holds 20 fragments; none concerns goals. `FRAGMENT_HOSTS` lists
  their hosts.
- `path@agent-skills@content/agents/` holds 12 agents. Several read goals as part of the record a
  work is judged against: `agent@knowledge-architect-design-conformance-reviewer`, which reads a
  plan document against them, `agent@knowledge-architect-record-searcher`, which grades them, and
  `agent@knowledge-architect-agent-text-reviewer` and `agent@knowledge-architect-workflow-auditor`, which
  read a proposed repair against every goals home. None judges a goal's own wording.
- The review skill's table, `skill@knowledge-architect-review@review-axes`, has no row for a diff
  that changes a goals home.
- Three installed texts state that a refinement within a ruling's direction, "stricter", is no
  finding, of the texts that judge fidelity to an approval of any kind: the transcript reviewer, its
  list of what each ruling is checked for and its scope paragraph; the planning skill,
  `skill@knowledge-architect-planning@plan-reviews`, its paragraph "What their findings become"; and
  `skill@knowledge-architect-retrospective`, its paragraph on a ruling recorded a little relaxed.
  The decision-record reviewer and the design-record auditor apply the same reading through the
  fragment on a head's shape, to heads only. The fragment on a head's shape, `path@agent-skills@fragments/head-shape.md`, says the same of a
  head's title and body; it governs heads only, and this work leaves it unchanged.
- `skill@knowledge-architect-setup@moving-the-pin`, step 2, lists only the Migration entries that cite
  an axis's section of `skill@knowledge-architect-project-audit`, and its step 5 runs those audits.
- Setup ships two goal wordings for a maintenance Component, proposed under goal-setting for the
  owner's ruling like any draft, so the reviewer reads them when one is drafted.
- The design skill sets a goal mid-discussion through goal-setting, in its grounding step of
  `skill@knowledge-architect-design@discussion-loop`, and goal-setting's frontmatter
  description and the primer's line for goal-setting under `primer@installed-skills` name the
  occasions the skill loads on.

**Inside the work, on the owner's word to Q7 (#arg-50):** this repository's own goals are brought
into conformance with the standard on this branch, by step 6, a whole-set review whose every
finding is drafted and ruled goal by goal, rewordings, retitlings, splits, removals and additions
included. Two findings are known in advance: `goal@knowledge-architect@rust-projects-come-first`,
whose "every check and tool" is a universal under rule 3, which
`issue@knowledge-architect@the-goal-that-rust-projects-come-first-states-a-universal-the-owner-has-not-ruled-on-knowingly`
records and step 6 closes; and `goal@agent-skills@one-skill-per-activity`, whose "activity" reads
two ways under rule 2 and which holds two intents under rule 4, for which the owner's argument
#arg-48 is among the statements step 6 gives the reviewer.

**Outside the work:**

- The goals of rootpoint and thaum. Their repair is each project's goal-setting, run as the
  whole-set review once a release ships this work.
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
| `design@knowledge-architect@changelog-entries` | a Migration entry for a change of the rules recorded content must meet cites the section that brings that content to them, an audit axis's or the goal-setting skill's whole-set review | `agent@klarch-changelog-reviewer`, lines 23 and 42, `path@knowledge-architect@CHANGELOG.md` line 5, `path@knowledge-architect@CLAUDE.md` line 202, `path@agent-skills@docs/design/audit-method.md` line 328, `path@agent-skills@docs/design/audit-record-axes.md` line 50, `path@core@docs/design/reference-lints.md` line 161, `register@knowledge-architect@design`, lines 154, 172 and 243, `issue@knowledge-architect@a-mechanical-changelog-check` line 9, `issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument` line 26, `register@xtask@rejected`; step 5 writes the Migration entry from the wording in this table and reads the changelog reviewer's two lines against it, the harvest rewrites the head to the same wording and reads the rest |
| `design@agent-skills@audit-triggers` | a version that changes the rules recorded content must meet names, in its Migration entry, the section that brings that content to them, an audit axis's or the whole-set review, and `skill@knowledge-architect-setup@moving-the-pin` runs either in a branch of its own once the pin move has merged | `tripwire@agent-skills@axis-not-run-at-a-pin-move` at `register@agent-skills@tripwire` lines 113, 123 and 124; `path@agent-skills@docs/design/audit-method.md` line 349, `path@agent-skills@docs/design/audit-record-axes.md` line 51, `path@agent-skills@docs/design/design-skill-and-heads.md` line 229, `path@agent-skills@docs/design/premortem-and-tripwires.md` line 133, `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text` line 34 and `register@knowledge-architect@design` line 196, each read by the harvest against the rewritten head; step 3 edits setup's pin-move steps, the harvest widens the tripwire's clause to the whole-set review |
| `design@agent-skills@misstated-ruling-rated-by-what-it-changes` | "a refinement within the ruling's direction is none at all" gains the exception for a goal, and the scale rates it: a goal recorded stricter than the owner ruled out knowingly changes what is built, since a goal rules work out, so it is major | `path@agent-skills@docs/design/reviews.md` line 186, and `register@agent-skills@tripwire` lines 29 and 244, read by the harvest against the rewritten head |
| `design@agent-skills@transcript-reviewer-agent` | "the same reading applies wherever fidelity to an approval is judged" gains its exception: a goal's wording that rules out what the owner has not ruled out knowingly is a finding, since a goal binds outright; the case is new against `rejected@agent-skills@transcript-reviewer-reports-every-clause` on that fact, #arg-24, and the owner approved the narrowing in round 3, by the word given against the table of the session's round-2 reply, which listed it under #goal-reviewer-agent | six lines of `register@agent-skills@design`: `path@agent-skills@docs/design/audit-method.md` line 250, `path@agent-skills@docs/design/design-skill-and-heads.md` line 219, `path@agent-skills@docs/design/owner-and-intent.md` lines 200 and 215, `path@agent-skills@docs/design/project-configuration.md` line 344, `path@agent-skills@docs/design/reviews.md` line 247; and two of `register@agent-skills@tripwire`, lines 396 and 405; the harvest reads each against the exception |

**Entries the discussion's search graded `bears`**, 94, each with its outcome:

| entries | outcome |
| --- | --- |
| the goals `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@no-contradiction-in-workflow`, `goal@agent-skills@only-needed-instructions`, `goal@agent-skills@one-skill-per-activity`, `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@any-project-can-adopt-it`, `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@the-owner-s-word-is-verified`, `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@documentation-stays-consistent`, `goal@knowledge-architect@design-with-agents-as-counterparts`, `goal@knowledge-architect@the-workflow-improves-through-real-use` | constrain the work. Six are criteria, met as "Criteria" says; `goal@agent-skills@one-skill-per-activity` is D3; the other six, `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@any-project-can-adopt-it`, `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@documentation-stays-consistent` and `goal@knowledge-architect@design-with-agents-as-counterparts`, are served by the work as #arg-12 and #arg-13 argue, and no shape contradicts one |
| `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@synthetic-evidence-not-built` | rested on: the work's evidence is real use, rootpoint's and thaum's histories, and no run is built to test the reviewer; step 2's one dispatch is the harness mechanics the second admits |
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

Three entries listed as not read in the spec's first commit were found by its design-conformance
review to bear, which by `design@agent-skills@may-bear-read-on-touch` reopens that decision, as an
issue entry records for its design discussion; the
session read each whole:

- `tripwire@agent-skills@axis-not-run-at-a-pin-move`: the pin move now runs the whole-set review
  too; the harvest widens its clause, as the table of rewritten decisions says.
- `rejected@agent-skills@transcript-reviewer-reports-every-clause`: step 4's exception reports, for
  a goal only, a clause inside a ruling; the case is new on #arg-24, a goal binding outright, which
  which the rejected entry's ruling did not weigh. The
  harvest's rewrite of `design@agent-skills@transcript-reviewer-agent` cites it.
- `issue@agent-skills@a-built-default-awaiting-the-owner-enters-a-head-only-once-the-owner-rules`:
  D1 and D2 are its case, each a default relaxing an approval; the owner ruled both before any
  work, so no head is written ahead of a ruling.

**Entries graded `may-bear` that the grounding did not read**, each with the searcher's reason:

- `design@agent-skills@knowledge-table-home`: #exclusion-names-its-rival (when an exclusion statement is a goal, and where it goes when it is not) may need a primer row; no conflict established
- `design@agent-skills@goals-required`: the reviewer on every goal draft adds a step to setup's goal step and contradicts nothing in it — read whole after the second design-conformance review: it binds step 6, which keeps every Component with at least one goal
- `goal@knowledge-architect@adoption-is-easy`: a review step at the first goal draft could add a step an adopter follows; no conflict established — read whole after the second design-conformance review found it borne on by step 6: a goal step 6 reviews, so any change to it goes through goal-setting at that step, per `design@agent-skills@search-relevance-grade`
- `goal@knowledge-architect@rust-projects-come-first`: its "every check and tool" is an unqualified universal of the kind rule 3 refuses; the searcher's reason, that the work edits no goal, no longer holds since step 6 reviews this goal, and "What the work is" names it
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
- `tripwire@agent-skills@verifier-refutes-a-true-finding`, `tripwire@agent-skills@verifiers-wave-findings-through`: count passes of an axis; none is added
- `design@core@orphan-rejected-entry`: an exclusion goal and a head of absence for a design lost to a goal; no bearing established
- `goal@core@declared-instructions-are-checked`: parts of the standard could be checks; the work proposes skill text and a reviewer only — read whole after the second design-conformance review found it borne on by step 6: a goal step 6 reviews, so any change to it goes through goal-setting at that step, per `design@agent-skills@search-relevance-grade`
- `goal@knowledge-architect@structure-and-workflow-work-together`: a whole-set reading may ask for a listing command; no need established — read whole after the second design-conformance review found it borne on by step 6: a goal step 6 reviews, so any change to it goes through goal-setting at that step, per `design@agent-skills@search-relevance-grade`
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
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: gathering every goal may be several steps; not established — read whole after the second design-conformance review found it borne on by step 6: a goal step 6 reviews, so any change to it goes through goal-setting at that step, per `design@agent-skills@search-relevance-grade`
- `design@core@backticked-bare-id`, `design@core@plain-word-lint`: bear on the names chosen for slugs and the agent
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: the standard's numbered rules cited in words are checked by nothing — read whole after the second design-conformance review: worsened, since the reviewer's modes cite rules by number; the harvest adds that instance to the issue
- `design@agent-skills@premortem-as-watch-points`: the work could add a standing retrospective question; none proposed
- `design@agent-skills@standing-argument-in-head`, `design@agent-skills@unargued-approval-is-argued`, `tripwire@agent-skills@owner-fact-recorded-unattributed`: cite a seed; no element meets them
- `design@agent-skills@searcher-grades-goals`, `design@agent-skills@search-relevance-grade`: the boundary block stays out of the goal entries, so the goals homes' size is unchanged
- `design@agent-skills@a-scratch-directory-per-subagent`: only if several reviewers are dispatched together
- `design@agent-skills@transcript-extraction-is-the-checker-s`, `design@agent-skills@transcript-found-by-listing`: only if the reviewer extracts the owner's words from a transcript; the session passes them in the brief
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

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met: the rules hold no example,
the draft's illustration uses placeholders, and rootpoint and thaum appear only in `%%` comments and
the record.

### The owner's reading time per round of goal rulings `##c-owner-attention`

Weighed. Unmet and accepted, on the owner's word naming it, given to Q3: "Q3 Q4 accepted". The
boundary block adds one line per case and per sibling goal touched to each drafted goal, as the
illustration under "The draft" shows. The owner's answer to Q1, #arg-38, argues the cost buys the
attention that was missing.

### A goals home stays short `##c-goals-stay-short`

Binding as a presumption, from `design@agent-skills@goal-entry-shape`. Met: the boundary block never
enters a goals home, and rule 2's qualifiers add one or two lines to a goal: R2 added one, R9 two.

### Each activity is carried out from its own skill `##c-one-skill-per-activity`

Binding, from `goal@agent-skills@one-skill-per-activity`. Met, on the owner's ruling of D3 and the
owner's argument #arg-48: an activity is its methodology, so the whole-set review, which follows
goal-setting's, a fresh reviewer and a draft ruled goal by goal, and none of the audit's, passes,
verifiers and report files, is part of setting goals.

## Threads

### A proper goal meets eight rules, written once as a fragment hosted by goal-setting and the reviewer `##goal-standard`

Proposed by the session in round 1 with seven rules; rule 8 is #exclusion-names-its-rival's, joined
to the fragment at assembly since the reviewer applies it. Approved in round 2, rule 3 reopened by
the session's material finding in round 2 on the owner's argument #arg-34, and closed in round 3.
State: `approved`, rules 1 and 6 by D2. Arguments: #arg-1, #arg-2, #arg-3, #arg-4, #arg-5, #arg-6, #arg-10, #arg-11, #arg-14, #arg-15, #arg-16, #arg-17, #arg-18, #arg-34,
#arg-41, #arg-47, #arg-49, #arg-51, #arg-52. Its shape is "Decided design", the standard; its harvest is a head in the goals design home.

The owner, round 2: "Agreed on goal-standard and the delivery method as a fragment in skill and
reviewer." The owner, round 3: "After these refinements,  agree with the whole table, you can
proceed to the spec".

### Each drafted goal is shown with its boundary block, and the owner's statements go to the reviewer, not into the display `##goal-draft-shows-reach`

Proposed by the session in round 1 with the owner's words shown beside each goal; the owner objected
in round 2 (#arg-34, #arg-35); the session withdrew the quotation from the display and kept the
block (#arg-40, #arg-42, #arg-43). State: `approved`, in round 3. Arguments: #arg-10, #arg-12, #arg-19, #arg-20, #arg-33, #arg-34, #arg-35, #arg-38, #arg-40,
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
(#arg-44, #arg-45). State: `approved`. Arguments: #arg-1, #arg-13, #arg-21, #arg-22, #arg-23, #arg-24, #arg-36, #arg-37,
#arg-44, #arg-45. Its shape is "Decided design", the reviewer; its harvest is a head in the goals
design home, and the rewrite of `design@agent-skills@transcript-reviewer-agent`.

The owner, round 2: "goal-reviewer-agent: agreed on the dispatch before showing the draft. And on
having the agent receive every relevant verbatim of mine, this is important I believe. It is
important to keep in the loop the idea that my verbatim statements can be wrong or
contradict/conflict an existing goal, and such a case must not be brushed aside. The case were
multiple goals are being added or reworded at once should also be handled."

### A diff that changes a goals home owes the goal reviewer at the review before the merge `##goal-review-before-merge`

Proposed by the session in round 1. State: `approved`, in round 2. Arguments: #arg-25. Its shape is
"Decided design", the review row; its harvest is a head in `path@agent-skills@docs/design/reviews.md`.

The owner, round 2: "goal-review-before-merge agreed."

### A goal wording is ruled on alone, never inside a batch approval of other items `##goal-edit-ruled-alone`

Proposed by the session in round 1. State: `approved`, in round 2. Arguments: #arg-26. Its shape is "Decided
design", the ruling; its harvest is a head in the goals design home.

The owner, round 2: "goal-edit-ruled-alone agreed"

### The whole goal set is reviewed inside goal-setting, by the goal reviewer, not by an audit axis `##goal-set-review`

Proposed by the session in round 1 as a fork, shape A its default and shape B a fourth audit axis.
The session chose shape A, its default, and the owner approved it in round 2; state `approved`. Arguments: #arg-7, #arg-8, #arg-27,
#arg-28, #arg-29, #arg-39, #arg-46, #arg-48, #arg-53. Its shape is "Decided design", the whole-set review; its harvest
is a head in the goals design home, recorded in the weak form as the session's choice the owner
approved, per `design@agent-skills@fork-choice-in-weak-form`, and the rewrites of
`design@agent-skills@audit-is-an-activity` and `design@knowledge-architect@changelog-entries`.

The owner, round 2: "goal-set-review: agreed on shape A here." On Q2: "Q2 I'm unsure about this. I
believe it should be the case. At the very least, I expect that root goals
that refine no other goals would stay in the tens. When factoring refining goals, I think this can
get larger. Anyway, I think a single agent should be able to read at least 100 goals, or probably
more, as long as it does only that."

### A statement of what the project is not for is a goal only when it names the design it excludes `##exclusion-names-its-rival`

Proposed by the session in round 1, from the owner's remark #arg-9. State: `approved`, in round 2. Arguments:
#arg-9, #arg-30, #arg-31. Its shape is rule 8 of the standard; its harvest is the standard's head.

The owner, round 2: "exclusion-names-its-rival agreed"

### An agent that meets a goal reading two ways gets no new instruction `##goal-reading-in-use`

Proposed by the session in round 1. State: `approved`, in round 2. Arguments: #arg-32. Its shape is the
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
none of the 10 defects. Reviews caught 4, you caught 3." Corrected at assembly against rootpoint's
history: 9 changes over 7 commits, reviews or the session catching 4, the owner 3 and a design
discussion 2, the 4 cases of excess reach unchanged.

### The rules, each with the case that needs it `##arg-14`

Round 1, the session; bears on #goal-standard. The seven rules as drafted, each followed by its
case from rootpoint or thaum: "1. **It states an outcome.**" "2. **It has one reading.**" "3.
**It reaches no wider than the owner's statement.**" "4. **It holds one intent.**" "5. **It
contradicts no other goal of the project.**" "6. **What shows it met stays checkable.**" "7. **A
refinement neither widens nor contradicts the goal it refines.**"

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
rewords each added about one line." Measured at assembly: R2 adds one line, R9 two.

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
rules out work, where a stricter acceptance criterion only adds a check." It is the fact that makes
the case new against `rejected@agent-skills@transcript-reviewer-reports-every-clause`, whose ruling
did not weigh a statement that binds outright.

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
fits one agent's context. And per-draft review already re-checks each repair." The counts it
rested on, rootpoint 28, thaum 8 and this repository 27 goals, recounted at assembly where round 1 had said 26 and 23, are re-taken by counting the
level-two headings of every goals home `cargo klarch anchors` lists in each project.

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

### The owner believes, unsure, that root goals stay in the tens, and that one agent reads at least 100 goals `##arg-39`

Round 2, the owner, answering Q2, a report the owner holds; bears on #goal-set-review. "Q2 I'm
unsure about this. I believe it should be the case. At the very least, I expect
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

### A published Component's features are mechanisms, their outcome is probably a goal, and a goal stays general `##arg-47`

Round 6, the owner, on Q5 (D2); bears on #goal-standard. "Whenever the project is published, the
features provided by the public API are always mechanisms, but the outcome (in a general way) they
provide is probably a goal. From the other side, goals are not meant to state the exact API, so the
described outcome has to stay "vague". Are you sure that your proposed default holds in all these
sort of cases ?"

### An activity is its methodology, not the scope of what it reads `##arg-48`

Round 6, the owner, on Q6 (D3); bears on #goal-set-review. "for audits, the activity is the whole
methodology, not the fact that it is a project wide audit. If two activities do not follow the same
methodologies, they are not the same, even if they share part of the name, and should not be
bundled in the same skill."

### No wording of rule 1 decides every case, since the level of a goal is the owner's intent `##arg-49`

Round 6, the session; bears on #goal-standard. The first default of D2 admitted, by its clause on a
tool provided, `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`'s "the published checker
offers a command" as an outcome without asking whether a command, rather than another kind, is what
the session needs. Rule 1 keeps the test alone, applied at the user's level: the kind of thing
provided is part of the outcome only where another kind would not reach the user's end, as a check
and a reviewer's memory do not in `goal@core@declared-instructions-are-checked`; an exact form never
is; and where the test leaves the level open, the reviewer asks the owner.

### This repository's goals are brought into conformance on this branch, since other work would have a hard time without it `##arg-50`

Round 7, the owner, on Q7; bears on #goal-set-review, #goal-standard. "since we are changing the
goal standard, I think that once the work is done, we should apply the standard to bring the goals
into conformance, on the same branch. This includes any rewording, retitling, split, removal, or
addition that is needed to pass the standard. I think that any work in the project would have a
hard time getting done properly without bringing these goals into conformance." On Q6, the same
round: "my argument should either be recorded, or used to reword the goal".

### The goals are not assumed right, and a kind of interface the owner intends is a goal `##arg-51`

Round 8, the owner, on D2; bears on #goal-standard. "is it a defect of the standard, or of the
current goals ? We should not assume the goals are right, since we are planning to bring them to
conformance after the work. Is the shape of a public API truly not something that would belong in a
tool ? If a CLI API is what I intend to provide, then it should be stated as a goal"

### The defect is the standard's: its list contradicted the outcome test, which is read against the owner's intent `##arg-52`

Round 8, the session; bears on #goal-standard. Rule 1 as approved, "never states a mechanism, a
tool, a check or a derivation", contradicted the outcome test of goal-setting, "would the Component
be for something else (a goal), or would it reach the same end another way (a decision, …)", whose "end" is what the owner intends:
an owner who intends a command line makes a Component without one for something else. The second
default kept the contradiction by judging the end from the user's side. The goals that met the
first rule are evidence that the rule bites, not that they are right; step 6 judges them. The exact
form of an interface fails the test, since renaming a command leaves the Component for what the
owner intends, and its home is a design head, by the entry test on an interface others consume.

### Rule 3 is judged by the owner on every goal's boundary block, at a review of the whole set `##arg-53`

Round 9, the session, on Q8; bears on #goal-set-review. Whole-set mode holds no statement of the
owner, so it cannot judge rule 3, and a goal whose one defect is its reach, as rootpoint's "one
human-readable text file" was, would pass as conforming. Every goal is shown with its block, a cost
of one block per goal, 27 in this repository. The owner agreed on that shape.

## New names, in one place

| name | file | status |
| --- | --- | --- |
| `path@agent-skills@fragments/goal-standard.md` | the standard's fragment | new |
| `path@agent-skills@content/agents/goal-reviewer.md` | the reviewer agent's source, installed as the agent `agent@knowledge-architect-goal-reviewer` | new |
| the section `skill@knowledge-architect-goal-setting@whole-set-review` | goal-setting's source | new |
| `skill@knowledge-architect-goal-setting@goal-or-not` | goal-setting's source | existing; hosts the fragment |
| `skill@knowledge-architect-goal-setting@drawing-out-intent` | goal-setting's source | existing; gains the dispatch and the block |
| `skill@knowledge-architect-goal-setting@when-it-runs-again` | goal-setting's source | existing; names the whole-set review |
| `FRAGMENT_HOSTS` | `path@agent-skills@src/lib.rs` | existing; gains one row |

The agent's sections, each a level-two heading with its slug placeholder, per
`design@core@section-homes-carry-slugs`: its section `agent@knowledge-architect-goal-reviewer@the-goal-standard` (the fragment), `agent@knowledge-architect-goal-reviewer@the-modes`,
`agent@knowledge-architect-goal-reviewer@what-to-report`. The names are the session's default and may change at step 2.

## Decided design

### The standard

The fragment holds the eight rules below, and goal-setting hosts it in its section
`skill@knowledge-architect-goal-setting@goal-or-not`, where its first rule carries today's outcome test, so the test has one home. The
section's opening, "A goal is met or unmet. A decision is won or lost.", and its lifecycle sentences stay in the skill. The wording below is the decided content;
the step may reword it, keeping each rule's meaning, and the reviewers judge the fidelity.

> **A goal is proper when all of these hold.**
> 1. **It states an outcome the owner intends**: what the Component is for, and for whom. A
>    sentence is a mechanism when, changed, the Component would still be for what the owner intends,
>    and an outcome when the change would make it for something else. So the kind of interface the
>    Component provides, a command line, a library API or a check, is an outcome where the owner
>    intends to provide that kind, and the drafter asks the owner where the statements do not say.
>    Its exact form, a command's name, a flag or a signature, is a decision: an interface others
>    consume earns a design head. The argument that a design follows from a goal goes in that
>    design's head, which cites it.
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
>    document or anything that leaves.
> 7. **A refinement neither widens nor contradicts the goal it refines.**
> 8. **A statement of what the Component is not for is a goal only when it names the design it
>    excludes**, one that another goal would otherwise argue for. A property of smallness with no
>    named design is no goal: it is the owner's scope control, exercised by what the owner asks
>    for, or a quality argument a design head makes against its own rival.

Rule 4's test is the head standard's rule on an exception, not the independent-reversal test
`rejected@agent-skills@head-split-by-independent-reversal` records. Each rule's cases go in a `%%` comment beside it, as "Mapping tables" lists them, naming the
project, so the installed text names none, per `goal@agent-skills@installed-text-works-anywhere`.
Rules 1 and 6 are written as D2, which the owner ruled. Rule 1's test is the outcome test
goal-setting states today, read against the owner's intent, per #arg-52: where the statements do not
say whether a kind of interface is intended, the reviewer reports an `intent` finding. Whether the
goals this repository holds today pass it is step 6's question, never an argument for the rule.

**The nearest rival** of the fragment is the standard written in goal-setting alone, with the
reviewer briefed to read it: it puts the reviewer's standard behind a read the brief must ask for,
which `rejected@agent-skills@one-generic-auditor-agent` records as uneven. The rival rule 3, "no
wider than the owner's statement", lost to #arg-41.

### The draft

Step 3 of `skill@knowledge-architect-goal-setting@drawing-out-intent` writes, per goal, beside its source mark:

- **its boundary block**: for each universal or exclusion, at least one case it rules out and one
  it admits, from the project's existing or planned work, or, where none exists yet, from the uses the owner's
  statements describe; each sibling goal it touches, with the
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

`agent@knowledge-architect-goal-reviewer`, read-only, hosts the fragment, and runs in three modes, each
named by its brief:

- **draft**: dispatched by `skill@knowledge-architect-goal-setting@drawing-out-intent` before the draft is shown, and again on every
  version before a ruling, the owner's own rewordings included. Its brief: the whole batch of new
  and reworded goals, every relevant statement of the owner, verbatim, whether or not the drafter
  drew on it, and every goals home of the project. It checks each goal of the batch against the standard, against every
  other goal of the batch and against every existing goal, and reports per goal: each reading with
  the case on which two differ; each generalisation, or case ruled out, the draft does not show;
  each pair of intents; each conflict with another goal; a measure or a refinement that fails its
  rule; the material of the boundary block. It also reports, as contradictions the drafter never
  resolves alone: **(a)** a statement of the owner that conflicts with an existing goal or with
  another statement of the owner; **(b)** a draft that drops or turns around part of what the
  owner's statements say, without showing it.
- **whole set**: dispatched by the section `skill@knowledge-architect-goal-setting@whole-set-review`. Its brief: every goals home. It
  reports every goal that fails a rule it can judge without the owner's statements, rules 1, 2 and
  4 to 8, and every conflict between two goals, and gives every goal's boundary block material, on
  which the owner judges rule 3, per Q8.
- **merge**: dispatched by the review row. Its brief: the branch's range. It reports every goal the
  diff adds or changes that fails a rule it can judge without the owner's statements, rules 1, 2
  and 4 to 8, or that conflicts with a goal of the tree, and every goal edit whose ruling no commit
  message of the branch records.

It reports and never edits; each finding the standard settles is the drafter's to repair, the
repaired batch reviewed again so that the version shown is a version reviewed, and each
question of intent and each contradiction goes to the owner in the draft. A contradiction holds the
goal resting on it, which is not written before the owner's answer, per `primer@owner-word-premise`.
The three modes apply one fragment. Each mode's own checks, (a) and (b) in draft mode and the
ruling record in merge mode, fire only on the input that mode's brief carries: a reviewer given no
statement of the owner cannot apply (a) or (b), and one given no range cannot read commit messages.
So the argument of `design@agent-skills@search-apart-from-the-standing-state-review`, that a second
mode lets a session apply the wrong standard, does not reach them: the brief's input selects the
check.

**Its report**, per goal of the batch or of the set, under the goal's slug: each finding tagged
`settled`, which the standard decides and the drafter repairs, `intent`, a question only the owner
answers, with the case on which the readings differ, or `contradiction`, (a) or (b), held for the
owner; then, in draft mode, the boundary block's material. A goal with no finding is listed as
conforming. **Its frontmatter**: the description names the three modes and their dispatchers; its
tools are `Read, Grep, Glob, Bash`, as every read-only reviewer's, with its read-only rule in its
body.

**Its dispatch.** In draft and whole-set mode no commit is judged: the reviewer reads the live
tree's goals homes with commands that read, and the draft in its brief, so it needs no worktree, and
the review skill's invariants on a commit range and a worktree do not apply; it is fresh, never a
fork, and returns its report as text. In merge mode it is a reviewer of the review skill, under
every invariant of `skill@knowledge-architect-review@review-invariants`.

The owner's report recorded as a premise, with its doubt: the owner is unsure, and believes that
root goals that refine no other goal stay in the tens, refining goals perhaps more, and that one
agent reads at least 100 goals when it does only that (#arg-39). No rule splits a larger
set (#arg-46).

**The nearest rival**, the drafter checking its own draft against the standard, lost to #arg-22:
the drafter is anchored on its own reading, which the owner reports of both parties.

### The review row

A row of `skill@knowledge-architect-review@review-axes`: "goals | the goal reviewer, by its
agent reference once step 2 lands it, in its merge mode | the diff changes a
goals home". **The nearest rival**, no row, lost to #arg-25.

### The ruling alone

A sentence of `skill@knowledge-architect-goal-setting@drawing-out-intent`: a goal's wording proposed in another activity, a plan's
default, a review's repair or an audit's item, is drafted here as its own item, with its boundary
block, and the owner's word on a batch of other items never covers it. The planning skill, in
`skill@knowledge-architect-planning@plan-reviews`' paragraph "What their findings become", the review skill, in
`skill@knowledge-architect-review@what-review-leaves`, and the design skill, where its checkpoint's batch confirmation is stated in `skill@knowledge-architect-design@threads-and-states`, each restate it in one sentence with the pointer to `skill@knowledge-architect-goal-setting@drawing-out-intent`, per
`design@agent-skills@restatement-size-test`. **The nearest rival**, the review row alone, lost to
#arg-26.

### The whole-set review

A section of goal-setting titled "Reviewing the whole goal set", its slug `skill@knowledge-architect-goal-setting@whole-set-review`. Its occasions: the owner's request, and a pin move
across a version whose Migration entry names the section. Setup is no occasion, per D1. The session
dispatches the reviewer in its whole-set mode, and each finding becomes a draft that
`skill@knowledge-architect-goal-setting@drawing-out-intent` takes, its draft-mode review included, which is the re-check. On the owner's word to Q8, every
goal is put to the owner with its boundary block, a goal with no other finding included, for a
ruling: kept, or reworded; only the owner judges rule 3 (#arg-53).
`skill@knowledge-architect-goal-setting@when-it-runs-again` names the section among the occasions the skill runs on.
The occasion reaches the sessions that meet it by three edits: goal-setting's frontmatter
description and the primer's line for goal-setting under `primer@installed-skills` each gain "the
owner asks to review the whole goal set, or a pin move crosses a version whose Migration entry
names it"; and `skill@knowledge-architect-setup@moving-the-pin` lists, in its step 2, each Migration
entry that names the whole-set review beside those that cite an audit axis, and runs it in its step
5, after the pin's merge, as it runs an axis.

**The nearest rival**, shape B, a fourth axis of the project audit with its saved workflow, lost to
#arg-27 and #arg-29: the session chose shape A, and the owner approved it.

### No instruction for a goal read two ways in use

#goal-reading-in-use adds no text: a conflict between a proposal and a goal already goes to the
owner, per `primer@goals-bind`, which R7 showed working.

### The stricter-refinement exception

Three texts gain the clause, at four passages: the transcript reviewer, in its list of what each ruling is
checked for and in its scope paragraph; the planning skill's paragraph "What their findings
become"; and the retrospective's paragraph on a ruling recorded a little relaxed. The clause: a
goal's wording that rules out what the owner has not ruled out knowingly is a finding, since a goal binds outright and rules out work, and it goes
to `skill@knowledge-architect-goal-setting@drawing-out-intent`. The fragment on a head's shape is unchanged: it governs a head's title and body,
never a goal.

## Mapping tables

The `%%` comment beside each rule of the fragment, its cases:

| rule | cases |
| --- | --- |
| 1 outcome | thaum's library goal, "The stream shape that meets this is …"; thaum's fuzzing goal, "whole games as a lower-rate second arm", removed by 0c8c25bd, and its mechanisms, a62e2296 |
| 2 one reading | R7, "every view it shows"; thaum 85f2b26c, "learning agent" |
| 3 ruled out knowingly | R1, R4, R9 |
| 4 one intent | R2, the concurrency sentence inside rootpoint's `minimal-content-set` |
| 5 no contradiction | R1, R2, R9; thaum's fuzzing goal against `coverage-before-speed`, its qualifier dropped by thaum 04bed4a6, kept absent by D5 and restored by D8 |
| 6 checkable measure | R6, a measure naming a milestone's list |
| 7 refinement | no case observed; the rule follows from #arg-4 |
| 8 exclusion | rootpoint's `minimal-content-set`, whose concurrency sentence named its design and whose smallness part did not |

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
- **Shape B, a fourth audit axis** (#goal-set-review): lost to #arg-27 and #arg-29; the session
  chose shape A, and the owner approved it.
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
review of this repository's goals is step 6's real work, on the owner's word to Q7, not a test.

## Implementation sequence

### Step 1: the fragment, hosted by goal-setting

Builds `path@agent-skills@fragments/goal-standard.md` with the eight rules and their `%%` cases;
goal-setting's `skill@knowledge-architect-goal-setting@goal-or-not` hosts it, its outcome test, the paragraph "A goal states an outcome … It never states a mechanism. The test …", moved into the fragment's rule 1;
`FRAGMENT_HOSTS` gains the row naming the installed goal-setting skill; the fragment and its
placeholder land in one commit, since the build refuses a fragment no host names;
`cargo klarch install-agent-skills`.

Claims:

- The fragment ships in goal-setting and nowhere else: `each_fragment_has_its_hosts` and
  `no_fragment_line_ships_outside_its_hosts` pass; shown to fail by adding a second host to the row
  with no placeholder line.
- No sentence of the outcome test ships twice in goal-setting's installed copy: a read of the copy.

Fails alone on: the fragment-host tests and the build.

### Step 2: the reviewer agent

Builds `path@agent-skills@content/agents/goal-reviewer.md`: its frontmatter, read-only rules, the
fragment hosted in its section `agent@knowledge-architect-goal-reviewer@the-goal-standard`, its three modes, its dispatch and its report, as
"The reviewer" says; `FRAGMENT_HOSTS`' row gains the agent's install path, inserted before the
skill's, since `each_fragment_has_its_hosts` compares the row with a sorted list; install. In the
commit that lands it, the spec's bare backticked name of the agent becomes its agent reference, and
the spec's two planned references become path references.

Claims:

- The agent installs under the owned namespace and its name passes the harness rules:
  `cargo klarch check` passes, every reference to the agent resolving against its installed copy;
  shown to fail by the agent's absence, under which the check reports each reference to it.
- One dispatch of the new agent sees its tools granted, the harness mechanics
  `design@agent-skills@synthetic-evidence-not-built` admits; no judgement of its report. The harness
  listed the agent as a type in the session that installed it, before step 4, so the dispatch may
  run there.

Fails alone on: the check's installed-file and harness-name findings.

### Step 3: the procedure

Builds, in goal-setting: the dispatch and the block in `skill@knowledge-architect-goal-setting@drawing-out-intent`, the re-run on any
rewording and on the whole batch, the contradictions (a) and (b) held for the owner, the ruling
alone; the section `skill@knowledge-architect-goal-setting@whole-set-review`, named in `skill@knowledge-architect-goal-setting@when-it-runs-again`, and the frontmatter description's new
occasion. In the review skill: the row of `skill@knowledge-architect-review@review-axes` and the sentence in
`skill@knowledge-architect-review@what-review-leaves`. In the planning skill: the sentence in
`skill@knowledge-architect-planning@plan-reviews`. In the design skill: the sentence in
`skill@knowledge-architect-design@threads-and-states`. In setup: steps 2 and 5 of
`skill@knowledge-architect-setup@moving-the-pin`. In the primer: the goal-setting line of
`primer@installed-skills`. Install. Once the section exists, the spec's bare `skill@knowledge-architect-goal-setting@whole-set-review`
becomes a reference to that section of goal-setting.

Claims:

- Every reference the new text writes resolves: `cargo klarch check`.
- Each restatement of #goal-edit-ruled-alone is one sentence with its pointer: a read of the three
  copies.
- A Migration entry naming the whole-set review reaches a step that runs it: a read of
  `skill@knowledge-architect-setup@moving-the-pin`'s steps 2 and 5.

Fails alone on: the check, and the agent-facing-text review.

### Step 4: the stricter-refinement exception

Builds the clause in the transcript reviewer, its list and its scope paragraph, in the planning
skill's paragraph "What their findings become", and in the retrospective's paragraph on a ruling
recorded a little relaxed. Install.

Claims: the fragment on a head's shape is unchanged, `git diff -- crates/agent-skills/fragments/head-shape.md`
empty; each clause names goals only, a read of the four passages.

Fails alone on: the agent-facing-text review of four passages.

### Step 5: the changelog

Builds the `Next release` entries of `CHANGELOG.md`, per `design@knowledge-architect@changelog-entries`
read whole, in the wording the table of rewritten decisions gives it, which the harvest then writes
into the head: under Workflow, the reviewer before every goal ruling, the
boundary block, the review row, the ruling alone and the whole-set review; under Migration, the
standard as a change of the rules recorded goals must meet, citing
`skill@knowledge-architect-goal-setting@whole-set-review`; each a patch on the surface
`agent-skills`. Then `cargo x changelog`, and the changelog reviewer's two citing lines read against
that wording.

Claims: the copies match, by the test `cargo x changelog` keeps; `cargo x gates` exits 0.

Fails alone on: the changelog copy test.

### Step 6: this repository's goals brought into conformance

Builds, under `skill@knowledge-architect-goal-setting`, read whole as installed by step 3, the
whole-set review of every goals home of this repository: the reviewer in its whole-set mode, then
each finding drafted through the skill's drawing out of intent, the reviewer in its draft mode on
each version, with every relevant statement of the owner, #arg-48 among them, and each goal ruled by
the owner by its slug. Every edit the rulings call for is made: a rewording, a retitling, a split, a
removal, an addition, every Component keeping at least one goal, per `design@agent-skills@goals-required`.
Each removed or renamed goal's references are repaired in the same commit, as
`skill@knowledge-architect-goal-setting@when-it-runs-again` says of a removal and the check reports of either; a head whose ground a reworded goal
changes is read against it. It runs where the reviewer is a registered agent type: the harness of the session that installed it
listed it as an agent type before step 4, so it may run in that session. Where step 6 runs in another session, the session that builds steps 1 to 5 pushes them and tells
the owner, the owner starts the session of step 6, which grounds on this spec and the branch's
commit messages, and the review before the merge names both sessions' transcripts, each by its
opening message. Step 6's
edits are this repository's own goals, a change internal to this repository, so they owe no
changelog entry, per `design@knowledge-architect@versioning-policy`.

Claims:

- Every goal of this repository carries the owner's ruling at this review, kept or reworded, on its
  boundary block and findings: the reviewer's last report on the set, with each goal ruled.
- `issue@knowledge-architect@the-goal-that-rust-projects-come-first-states-a-universal-the-owner-has-not-ruled-on-knowingly`
  closes with the owner's ruling on that goal.
- `cargo klarch check` passes after every goal removal or rename.

Fails alone on: the reviewer's report, the owner's rulings and the check.

## Order rationale

- The fragment comes first so that the agent of step 2 hosts a fragment that exists.
- The agent comes before the procedure, so that the references step 3 writes to it resolve.
- The procedure comes before the exception, since the exception routes a finding to the procedure.
- The changelog follows the procedure, since it names what steps 1 to 4 built.
- The goals are brought into conformance last, since they are reviewed by the installed procedure,
  the reviewer and the standard the earlier steps land.

## Defaults awaiting the owner

- **D1** (#goal-set-review): setup is no occasion of the whole-set review. The owner approved shape A
  with "after setup" among its occasions. At setup each Component's batch is reviewed in draft mode
  against every goals home written before it, so every pair of goals is checked by the end of setup,
  and a whole-set review then adds a dispatch that checks nothing new. Default: dropped. **Ruled:**
  the owner, after the spec's first commit: "D1 approved, drop it".
- **D2** (#goal-standard): rules 1 and 6 as "The standard" writes them, rule 1 in its third
  default. The first, "a tool, a check or a command is an outcome where it is what the Component
  provides", admitted a goal naming any command, the exact form #arg-47 keeps out. The second
  judged the kind of interface by whether another kind would reach the user's end, which #arg-51
  defeats: the owner's intent to provide a kind of interface makes it an outcome, whatever another
  kind could do. The third reads the outcome test against the owner's intent, #arg-52. **Ruled:** the
  owner, on the third default: "D2 approved, go ahead". As approved, rule 1 said a
  goal "never states a mechanism, a tool, a check or a derivation", and rule 6 "never the tool that
  checks it". Read so, they rule out three goals of this repository the owner adopted,
  `goal@core@declared-instructions-are-checked`, `goal@core@documents-stay-current` and
  `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`, for which the tool is what the
  Component provides, and rule 1 had dropped the outcome test those goals pass. As ruled, rule 1
  is written as "The standard" gives it, and rule 6 drops its last clause.
- **D3** (#goal-set-review): `goal@agent-skills@one-skill-per-activity` is met with the whole-set
  review inside goal-setting, on the reading #c-one-skill-per-activity states. A conflict with a
  goal goes to the owner, so the reading waits for the owner's word. **Ruled:** the owner, after the
  spec's repairs: "Q6 no conflict here." with #arg-48.

## Harvest

At the landing, under `skill@knowledge-architect-decision-recording`, read whole:

| thread | home | what lands |
| --- | --- | --- |
| #goal-standard | the goals design home | a head on the standard, its fragment and its hosts; `design@agent-skills@goal-entry-shape` rewritten to point to it |
| #goal-draft-shows-reach | the goals design home | a head on the draft's boundary block, the owner's statements kept out of it |
| #goal-reviewer-agent | the goals design home; `path@agent-skills@docs/design/reviews.md` | a head on the reviewer, its modes and its findings (a) and (b); `design@agent-skills@transcript-reviewer-agent` rewritten with the exception for goals |
| #goal-review-before-merge | `path@agent-skills@docs/design/reviews.md` | a head on the row |
| #goal-edit-ruled-alone | the goals design home | a head on the ruling alone |
| #goal-set-review | the goals design home; `path@agent-skills@docs/design/audit-method.md`; `register@knowledge-architect@design` | a head in the weak form, carrying the owner's argument #arg-48 that an activity is its methodology, whatever step 6 rules on the goal that states it; `design@agent-skills@audit-is-an-activity` and `design@knowledge-architect@changelog-entries` rewritten; `design@agent-skills@goals-change-through-goal-setting` rewritten with the occasion |
| #exclusion-names-its-rival | the standard's head | rule 8 and its argument |
| #goal-reading-in-use | the standard's head, or the commit message | the absence, by entry test 2 if a reason must be respected at no site |
| the losing alternatives | `register@agent-skills@rejected`, where `skill@knowledge-architect-decision-recording` admits each | each with the head that cites it |
| step 6's goal edits | the goals homes, under goal-setting | written at step 6, each in the commit of its round of rulings; no harvest row |
| the premortem's causes | `register@agent-skills@tripwire`, where the standard admits each | each with its horizon |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | its own file | the goal reviewer as a second instance of its fourth case |
| #goal-set-review | `path@agent-skills@docs/design/audit-method.md`; `register@agent-skills@tripwire` | `design@agent-skills@audit-triggers` rewritten to name the whole-set review; `tripwire@agent-skills@axis-not-run-at-a-pin-move` widened to it |
| #goal-reviewer-agent | `path@agent-skills@docs/design/reviews.md`; `register@core@issue` | `design@agent-skills@misstated-ruling-rated-by-what-it-changes` rewritten with the exception and its rating; `issue@core@no-checked-form-cites-a-point-inside-a-section` gains the reviewer's citations of rules by number as an instance |
| #goal-standard | `path@agent-skills@docs/design/shipped-set.md` | the new fragment added to the list of fragments in `design@agent-skills@shared-text-fragment`, and its host list as the owner ruled it |

## Later consequences

- rootpoint and thaum each run the whole-set review when they move their pin across the release, per
  its Migration entry; rootpoint's `minimal-content-set` is expected to split under rules 4 and 8.
- A design head whose ground cites a goal that step 6 rewords, splits or removes is read again at
  that step; one that needs more than a re-pointed reference is a decision of its own, routed to
  `skill@knowledge-architect-design`.
- A set past what one agent reads, if premortem cause 3 fires, reopens #goal-set-review.
