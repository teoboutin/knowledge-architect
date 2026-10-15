# The primer carries what every adopting project's root CLAUDE.md would say alike: the checker's model, verification and working prose

## Status and audience

This spec is the plan of the work that moves the general instructions of this repository's root
CLAUDE.md into the installed primer, adds a primer section on the checker's model, and adds a
command that lists a project's anchors. The work does five things:

- the core gains a command, `klarch anchors`, which prints each anchor of the project with its
  kind, its path, its registers and the home of each register as the tree holds it;
- the primer gains three sections: `checker-model`, what a reader needs to read and cite the
  project's records and to run the checker; `verify-before-relying`, the verification rules; and
  `working-prose`, the language rules;
- the design and planning skills point to those sections, the agent-configuration skill names
  `working-prose`, and the setup skill drops its staged-check recommendation, which the primer
  now states;
- this repository's root CLAUDE.md keeps only what is its own;
- the design heads on the primer are rewritten, one head per new section that passes an entry
  test.

The spec is written for a session that did not witness the design discussion that produced it. It
leaves the repository in the commit that completes its harvest.

- Where this spec and a design home disagree on a point "What is already decided" does not list
  as reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says
  what waits: a default listed under "Defaults awaiting the owner" is built as written, and the
  branch merges only once the owner has ruled on it.
- It was assembled from one transcript,
  `path@elsewhere@~/.claude/projects/-home-catA-tb266682-Documents-code-knowledge-architect/1fa1d2a3-9239-487b-bf8e-ab380e3191f8.jsonl`.
  The discussion begins at the owner's message that opens "I would like to discuss the repartition
  of instructions between this project's root CLAUDE.md." It holds 3 owner messages, called rounds 1
  to 3, and the agent's reply to each. Find the file by that opening message, not by its name.

## How the work is done

Per `skill@knowledge-architect-planning@working-a-slice`, the work of a spec. Step 1 changes Rust
source and follows `skill@klarch-development`. Steps 2 and 3 edit agent-facing text and follow
`skill@knowledge-architect-agent-configuration`, and step 2 follows `path@agent-skills@CLAUDE.md`,
"Editing an installed skill or agent", read whole.

## Names

- **the primer**: the shipped source `path@agent-skills@content/PRIMER.md`, installed as
  `path@agent-config@knowledge-architect/PRIMER.md` by `cargo klarch install-agent-skills`. Edits
  go to the source; the install writes the copy.
- **the root CLAUDE.md**: `path@knowledge-architect@CLAUDE.md`, which imports the primer with the
  line holding the primer's installed path after an `@`, checked at `path@core@src/check/agents.rs`.
- **the rarer rules**: the checker rules that `instructions@mechanical-validation` states, that
  a writer needs only when writing a reference of an uncommon shape, and that the primer does not
  state today: the candidate rule, the two retired forms, ignored targets, the path to anchor, a
  register's home cited by its path, the scope of a plan item's citation, and a slug written
  outside a definition site.
- **the bundle issue**: `issue@agent-skills@primer-content-bundles-each-directive-s-decision`.
- **F23, F69, F121**: the owner's rulings on the agentic-workflow audit, recorded in
  `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`: the rarer checker
  rules are delivered by their finding messages, not by the installed text, and an unstated checker
  rule is a finding only where its finding message does not name its repair.
- **V1 to V13**: the directives of `instructions@verify-before-relying`, numbered by the overlap
  map of round 2, which called them D1 to D13, `argument@primer-common-root-part@a30`:
  V1 names and prose are leads, V2 follow the whole path, V3 one case is no convention, V4 report
  the evidence and label assumptions, V5 re-verify before acting, V6 a transfer names its property,
  V7 evidence is textual and cheap, V8 a checkable claim is checked before it is written, V9 a
  commit message is a document whose claims are listed and checked, V10 the checks apply to
  answers to the owner, V11 check whether it is already recorded, V12 precedent is not authority,
  V13 a document can describe the code as committed, and confidence is not correlated with
  correctness.
- **the overlap maps**: the two read-only inventories of round 2, of the installed text against
  V1 to V13 and against the language rules.
- **`{{command}}`**: the placeholder the install fills with the project's declared command, in
  `shipped_from` of `path@core@src/agents.rs`, per `design@core@declared-command`; the shipped
  text writes the checker's command with it.

## What the work is

### What exists today

- **The primer** has 12 level-two sections, from `primer@goals-bind` to `primer@installed-skills`.
  `primer@intent-and-claims` holds four bullets: built intent, a reason recorded at the code, a
  claim about the code goes stale (bullet 3), and the check before diagnosing (bullet 4).
  `primer@where-knowledge-goes` holds the knowledge table and, among its prose paragraphs, the rules
  on when to write a reference and how a `path` reference is shaped, and how a finding is repaired.
  No section lists the reference kinds, the anchor kinds or the register shapes, or says how to run
  the checker.
- **The setup skill** states the layout a Component carries, in
  `skill@knowledge-architect-setup@components-and-locations` and
  `skill@knowledge-architect-setup@component-documents`, and loads only when a project adopts the
  workflow or moves its pin. `skill@knowledge-architect-setup@setup-gates` recommends that the root
  CLAUDE.md state `check --staged` before each commit, per
  `design@agent-skills@staged-check-before-each-commit`.
- **The design skill** holds the language rules in
  `skill@knowledge-architect-design@discussion-language`, whose jurisdiction is all working prose,
  commit messages and code comments included; no installed text cites that section. Its Overview
  holds the metaphor rule. Its loop step 4 holds a provenance rule for a factual claim doing closing
  work, and step 5 the evidence built when a discussion stalls.
- **The planning skill**, in `skill@knowledge-architect-planning@spec-contents`, says a statement
  about existing code "is checked against the tree before it is written, exactly as in a design
  head"; no sentence of `primer@design-heads` states that check.
- **The root CLAUDE.md** holds `instructions@language-and-style` (an "Always" list restating the
  design skill's language rules, with an em-dash bullet of its own),
  `instructions@mechanical-validation` (118 lines restating the core's model and the rarer rules,
  and this repository's manifest), a "Plan documents" subsection of
  `instructions@where-knowledge-goes`, `instructions@verify-before-relying` (92 lines) and
  `instructions@verify-mechanically` (76 lines).
- **The core's CLI**, `path@core@src/cli/mod.rs`, has eight commands: `check`, `show`, `issues`,
  `tripwires`, `index`, `model`, `commits`, `install-agent-skills`. None lists the anchors.
  `Anchor` in `path@core@src/entity.rs` holds each anchor's name, path, home base, the registers
  it carries (`path` aside), whether it is a Component, and which constructed kind it is;
  `Anchor::home_of` computes a register's home, and `Anchors::of` builds every anchor from the
  manifest and the tree's listing.
- **thaum's root CLAUDE.md** restates much of the same text, by hand, in another repository: its
  section on the checker and its anchors, its verification and language sections. This work does
  not edit thaum; the changelog's Migration entry tells it what it can delete, per
  `acceptance@primer-common-root-part@migration-names-the-sections`.

### Outside the work

- **The rarer rules are not shipped.** They reach a session through their finding messages, per
  F23, F69 and F121; `thread@primer-common-root-part@checker-model-in-primer`.
- **"Run `commits` after each commit" is not shipped.** It waits for real-use evidence, per
  `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text`.
- **The `ls docs/audit-reports/` line is not shipped.** It waits for evidence, per
  `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`.
- **`cargo x gates` and the CI command stay this repository's**, per
  `design@agent-skills@gates-convention`.
- **The root's "When presenting content to the user" and "When writing into durable files"
  subsections stay this repository's.** The discussion did not argue them.
- **The em dashes of the existing installed text are not swept.** A text this work does not edit
  is not rewritten for style; a passage it edits follows `working-prose`.
- **The setup skill's layout tables stay.**
  `skill@knowledge-architect-setup@components-and-locations` and
  `skill@knowledge-architect-setup@component-documents` state what a setup writes, the act of that
  moment; the work does not edit them.
- **The narrower homes of V1 to V13 stay**, as "The section `verify-before-relying`" lists.
- **No general rule of defined vocabulary is added.** The overlap map found it in three scoped
  forms, each serving its own document, and no act was found that lacks a general one.
- **"Current reality only" stays where it is**, in the agent-configuration skill and in the root
  CLAUDE.md; the discussion did not move it.
- **No review axis is added to judge the language rules.** The reviewer agents say "Do not report
  style preferences", and the discussion did not argue a change.
- **thaum's own files** are edited by thaum, at its next pin move.

## What is already decided

### The decisions this design rests on

- `goal@agent-skills@installed-text-works-anywhere`, `goal@knowledge-architect@adoption-is-easy`,
  `goal@knowledge-architect@the-workflow-improves-through-real-use`.
- `design@agent-skills@shipped-text-cites-no-entry`: the moved text cites only installed sections.
- `design@agent-skills@behaviour-additions-need-real-use` and `design@agent-skills@need-test`.
- `design@agent-skills@restatement-size-test` and `design@agent-skills@knowledge-table-home`.
- `design@agent-skills@plans-directory-in-primer`.
- `design@agent-skills@primer-reread-before-recording`, the pattern the design skill's re-read
  of `verify-before-relying` follows.
- `design@agent-skills@one-home-for-head-rules`.
- `design@core@declared-command`, `design@core@exit-code-ladder`,
  `design@core@the-core-cli-is-a-library-module`.
- `design@knowledge-architect@version-lockstep`: the primer and the binary ship at one version.

### The decisions this work rewrites

| decision | the change | texts that cite it, and what updates each |
| --- | --- | --- |
| `design@agent-skills@primer-limit` | its title and body state the owner's framing, the primer as the part every root CLAUDE.md would say alike | `path@agent-skills@CLAUDE.md` line 47 (restates it: step 4); `path@agent-skills@content/PRIMER.md` lines 3 and 4, a `%%` comment (step 2); `path@agent-skills@docs/rejected-alternatives.md` line 391 and three lines of `register@agent-skills@design` (re-read at step 4); the bundle issue (closes at step 4) |
| `design@agent-skills@primer-content` | leaves the design home, its member list moving to `primer-limit`'s body and each directive it argues recorded as "The rewrite of `primer-limit`, and the end of `primer-content`" says, per D3 | the bundle issue, which closes at step 4 |
| `design@agent-skills@staged-check-before-each-commit` | the primer states the staged check; the setup skill no longer asks the root CLAUDE.md to | the root CLAUDE.md line 368 (step 3); `path@agent-config@skills/klarch-release/SKILL.md` line 46 (re-read at step 3); `register@agent-skills@design` line 408 (step 4); `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` line 31 (re-read at step 4) |

The sections of the root CLAUDE.md that this work shortens keep their slugs. These texts cite
them, or cite the bullets of `primer@intent-and-claims` that move:

| cited | at | what it points to | updated at |
| --- | --- | --- | --- |
| `instructions@verify-before-relying` | `path@agent-config@skills/klarch-development/SKILL.md` line 41 | the subsection "Check whether it is already known", which moves | step 3: points to the primer's section, and to the root's two listing lines |
| `instructions@verify-before-relying` | `path@agent-config@open-issues/a-pending-retrospective-analysis-is-reached-by-no-standing-search.md` line 21 | the `ls docs/retrospective-reports/` line, which stays | step 3: re-read |
| `instructions@verify-before-relying` | `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` line 40 | the `ls docs/audit-reports/` line, which stays | step 3: re-read |
| `instructions@verify-before-relying` | `issue@knowledge-architect@a-measurement-from-another-project-has-two-homes` line 23 | the thaum figure sentence, which stays | step 3: re-read |
| `primer@intent-and-claims` | `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` line 28 | "its last bullet", which moves | step 3: re-pointed |
| `primer@intent-and-claims` | the root CLAUDE.md lines 215 and 271 | bullets 3 and 4 | step 3: rewritten |
| `primer@intent-and-claims` | `path@agent-config@skills/klarch-development/SKILL.md` line 44 | the section, read whole, for reading documents against code | step 3: re-read, and the primer's `verify-before-relying` named beside it |
| `instructions@verify-mechanically` | `path@agent-config@skills/klarch-development/SKILL.md` line 184, `path@agent-config@skills/klarch-release/SKILL.md` line 46 | the gates and the checks before and after a commit | step 3: re-read |
| `instructions@verify-mechanically` | `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text` line 46 | the rule that a failing staged check is repaired before the commit, which moves | step 3: re-pointed to the primer's section |
| `skill@knowledge-architect-design@discussion-language` | the root CLAUDE.md line 25 | the language rules, which move | step 3: rewritten |
| `skill@knowledge-architect-design@discussion-overview` | the root CLAUDE.md line 27 | the metaphor rule, which moves | step 3: deleted with the "Always" list |

### The standing entries the search returned

Three searchers read the 141 issues and tripwires at the grounding. The entries they returned, with
their outcome:

| entry | outcome |
| --- | --- |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | holds F23, F69 and F121: the rarer rules are not shipped |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` | closed at step 4, by its own closing condition, per D3 |
| `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` | stays open; its addition is not shipped |
| `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` | stays open; its line 28 is re-pointed |
| `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` | stays open; its line stays in the root |
| `issue@knowledge-architect@command-output-is-not-declared-a-contract` | the primer states only the exit codes, which `path@core@README.md` documents, and the verdict line, which ends the output whose order that README states as a contract; the `anchors` output is for reading, not a contract |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | bears: the move removes root restatements; it stays open |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | the primer states no rule on it, so it takes no side |
| `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text` | its line 46 is re-pointed. The defect occurs only in a project that builds the checker from its own tree, as its summary says, so a primer that states the staged check brings it to no adopting project. Step 2 commits the primer source and its installed copy together |
| `issue@core@branch-sha-citations-are-judged-within-the-range-only` | the root keeps its `commits origin/main..HEAD` line and the pointer |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | the moved text is read at review for any entry it cites |
| `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | the new primer section holds no TOML or CI block |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton` | the primer states the documents a Component carries today; it stays open |
| `issue@agent-skills@a-skill-for-creating-a-component` | bears: `checker-model` states what a Component carries, not how to create one; it stays open |
| `issue@core@installed-file-findings-belong-in-phase-four` | the primer names the four phases without naming which phase reports an installed file |
| `issue@core@no-checked-form-cites-a-point-inside-a-section` | bears: the planning skill's pointer names a subsection of `verify-before-relying` in words beside the section's reference; it stays open |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing`, `issue@core@references-are-read-in-markdown-and-rust-only`, `issue@core@cross-project-references`, `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, `issue@core@a-component-states-at-least-one-goal`, `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | bear on nothing the work changes: it states no plan-item rule, deletes no slug, and moves no row of the knowledge table |
| `issue@agent-skills@patching-an-installed-skill`, `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`, `issue@agent-skills@a-criterion-derived-from-a-decision-binds-until-the-owner-says-otherwise`, `issue@agent-skills@a-saved-workflow-s-comments-ship-to-every-project`, `issue@agent-skills@the-primer-and-its-head-name-a-guard-s-issue-under-two-conditions`, `issue@agent-skills@the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head`, `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on` | reached through the seeds; the work edits none of the text they are about |
| `tripwire@agent-skills@pointer-not-followed` | read at review: the design skill's steps 4 and 5 become pointers |
| `tripwire@agent-skills@plain-text-pointer-found` | read at review: the moved text loses its entry references, and none becomes plain text |
| `tripwire@agent-skills@real-use-refusal-overruled` | not fired: the real-use criterion is met, `criterion@primer-common-root-part@real-use` |
| `tripwire@agent-skills@structure-rejection-overruled` | not fired: no rejection citing it was made |
| `tripwire@agent-skills@agent-text-written-without-the-standard`, `tripwire@agent-skills@agent-text-findings-need-nothing` | read at review: the agent-text axis runs on the diff |
| `tripwire@agent-skills@head-created-without-deliberation`, `tripwire@agent-skills@member-beyond-the-argument`, `tripwire@agent-skills@rule-title-wider-than-its-argument`, `tripwire@agent-skills@split-rule-from-exception`, `tripwire@agent-skills@owner-intent-stripped` | read at the harvest, which writes and rewrites heads with this spec as their deliberation |
| `tripwire@core@candidate-rule-silence` | bears on nothing: the candidate rule is not shipped |
| `tripwire@agent-skills@argued-closure-reopened` | not fired: one thread closed `argued` and the owner confirmed it |

Every other entry was read whole and judged unrelated. The searchers' lists are in the transcript.

## Criteria

### An installed text works in any project that installs it `##works-anywhere`

Binding, from `goal@agent-skills@installed-text-works-anywhere`. Met: the moved text names no
path of this repository, writes the command as `{{command}}`, and assumes no Rust or cargo; the
project-specific parts stay in the root CLAUDE.md, per
`thread@primer-common-root-part@verification-in-primer` and
`thread@primer-common-root-part@checker-cli-in-primer`.

### A reader finds what to do where it already looks `##adoption-easy`

Binding, from `goal@knowledge-architect@adoption-is-easy`. Met by
`thread@primer-common-root-part@checker-model-in-primer` and
`thread@primer-common-root-part@anchors-listing-command`.

### The shipped text cites no entry of this repository `##cites-no-entry`

Binding as a presumption, from `design@agent-skills@shipped-text-cites-no-entry`. Met: the moved
text cites only primer and skill sections.

### The primer holds what every session needs and no skill delivers `##primer-limit-test`

Binding as a presumption, from `design@agent-skills@primer-limit`. Met, with the head rewritten
under `thread@primer-common-root-part@primer-extends-every-root`.

### A behaviour instruction is added on evidence from real use `##real-use`

Binding as a presumption, from `design@agent-skills@behaviour-additions-need-real-use`. Met: the
checker's facts are a provable gap, a fact the checker fixes; the staged check before a commit is
the exception `design@agent-skills@staged-check-before-each-commit` already argues; the verification
rules rest on the owner's account of real sessions in several projects,
`argument@primer-common-root-part@a21` and `argument@primer-common-root-part@a22`; the language
rules on a contradiction provable by reading, `argument@primer-common-root-part@a16`.

### The rarer checker rules reach a session through their findings `##rarer-rules-by-findings`

Binding, the owner's rulings F23, F69 and F121. Met: `checker-model` holds the model and the rules
every reference follows, and no rarer rule; the owner confirmed in round 2,
`argument@primer-common-root-part@a19`.

### A restatement is no longer than a pointer `##restatement-size`

Binding as a presumption, from `design@agent-skills@restatement-size-test`, with
`design@agent-skills@need-test`. Met: the root CLAUDE.md keeps no copy of a directive the primer
states, the primer and the root being one text, per "Mapping tables"; and every restatement this
work writes or edits in a skill is one sentence with its pointer. A restatement this work does not
edit is converted when a later change edits it.

## Threads

### The primer holds the checker's model a reader needs, and not the rarer rules `##checker-model-in-primer`

Proposed by the owner in round 1, `argument@primer-common-root-part@a3`; the agent added the
exclusion of the rarer rules in its reply to round 1, `argument@primer-common-root-part@a4` and
`argument@primer-common-root-part@a5`. State: `approved`. Arguments:
`argument@primer-common-root-part@a3`, `argument@primer-common-root-part@a5`,
`argument@primer-common-root-part@a8`, `argument@primer-common-root-part@a9`,
`argument@primer-common-root-part@a10`, `argument@primer-common-root-part@a19`. Shape: "Decided
design", "The section `checker-model`". Harvest: a head of agent-skills, by test 2 at no site, a
decision about an absence, the rarer rules not shipped, its ground for that absence attributed to
the owner's rulings F23, F69 and F121. The owner, round 2: "About
instructions@mechanical-validation: yes, I agree, the goal is to facilitate \"reading\", and the
detailed findings do not need to be shipped in the primer." and "checker-model-in-primer: agreed on
that shape, in the primer." Round 3: "Agreed on all threads here, you can proceed".

### A core command lists the project's anchors, their registers and their homes `##anchors-listing-command`

Proposed by the agent in its reply to round 1, `argument@primer-common-root-part@a11`. State:
`approved`. Arguments: `argument@primer-common-root-part@a11`,
`argument@primer-common-root-part@a20`. Shape: "Decided design", "The command `anchors`", and the
deletion of the root's hand-written list in "Mapping tables". Harvest: a head of core, by test 1, a
command line. The owner, round 2: "anchors-listing-command: good idea. This is a good way to deliver
easily, without needing manual updates in each project, which often go stale." The round-2 delta the
owner approved in round 3 read "root and thaum lists deleted after". Round 3: "Agreed on all threads
here, you can proceed".

### The primer holds the verification rules, and the skills point to it `##verification-in-primer`

Proposed by the owner in round 1, `argument@primer-common-root-part@a1`; the agent raised the
real-use obstacle in its reply to round 1, `argument@primer-common-root-part@a12`; the owner
answered it and asked for the overlap check in round 2, `argument@primer-common-root-part@a21` to
`argument@primer-common-root-part@a24`; the agent gave the shape in its reply to round 2,
`argument@primer-common-root-part@a30` to `argument@primer-common-root-part@a35`. State: `approved`.
Shape: "Decided design", "The section `verify-before-relying`". Harvest: a head of agent-skills, by
test 2: the primer and the 7 agents' "Establish the state of the tree yourself" state its
instruction; its evidence of real use is attributed to the owner. The owner, round 2:
"verification-in-primer: I doubt we'll find evidence. However, you can trust my word that adding
this section in thaum did its fair share of work. LLM tend to believe things will easily work out,
and that the project they work on and the tools they use have intuitive, known behavior. This is
often not the case, sometimes by a lot." and "The primer might become the primary location (every
session might face these problems IMO), with the design skill pointing at it (and maybe restating
part of it, or asking the agent to reread it)." Round 3: "Agreed on all threads here, you can
proceed".

### The primer states the checker's general CLI facts, and the root keeps the project's gates `##checker-cli-in-primer`

Proposed by the owner in round 1, `argument@primer-common-root-part@a1` ("Part of
##verify-mechanically could be too.", the backticks dropped so that the slug defines nothing here);
shaped by the agent in its reply to round 1, `argument@primer-common-root-part@a13` and
`argument@primer-common-root-part@a14`. State: `approved`. Shape: "Decided design", "The section
`checker-model`", its last part. Harvest: a member named in the body of
`design@agent-skills@primer-limit`, and the rewrite of
`design@agent-skills@staged-check-before-each-commit`; no head of its own. The owner, round 2:
"checker-cli-in-primer: agreed. This would be simpler than having the setup skill ask for it to be
added to root CLAUDE.md." Round 3: "Agreed on all threads here, you can proceed".

### The root CLAUDE.md's "Plan documents" subsection is deleted `##root-plans-subsection`

Proposed by the owner in round 1, `argument@primer-common-root-part@a2`. Closed `argued` by the
agent in its reply to round 1, `argument@primer-common-root-part@a15`: a project row that only
repeats a primer row is removed, per `design@agent-skills@knowledge-table-home` and
`design@agent-skills@plans-directory-in-primer`. The rival, keeping it as a pointer, loses to that
rule. Shape: step 3. Harvest: none, a member of `design@agent-skills@knowledge-table-home`'s rule.
The owner, round 2: "root-plans-subsection: agreed."

### The primer holds the working-prose rules, merged from the design skill and the root `##working-prose-in-primer`

Proposed by the owner in round 1, `argument@primer-common-root-part@a1` ("And the Always subsection
of ##language-and-style.", the backticks dropped); widened by the agent to the design skill's
section in its reply to round 1, `argument@primer-common-root-part@a16` and
`argument@primer-common-root-part@a17`; the owner ruled the merge and the two wordings in round 2,
`argument@primer-common-root-part@a26` and `argument@primer-common-root-part@a27`; the agent gave
the draft and the scope default in its reply to round 2, `argument@primer-common-root-part@a36` and
`argument@primer-common-root-part@a37`. State: `approved`. Shape: "Decided design", "The section
`working-prose`". Harvest: no head by test 2, since the primer alone will state the rules and their
reasons sit beside them; a head by test 4 only on the owner's answer to Q1. The owner, round 2:
"working-prose-in-primer: Both version from design skill and CLAUDE.md should be merged. The
em-dash/semicolon can be removed I guess. Though I believe LLM tend to use those in excess, so I
lean toward keeping it. \"Readers are non-native speakers\" can become \"Readers might be non-native
speakers\". This applies generally I believe. No project is ever contributed to by only native
english speakers. Maybe the sentence should be completed, the goal is that LLM prefer to use simpler
sentences and words whenever possible." Round 3: "Agreed on all threads here, you can proceed".

### Each new primer section gets a head when it passes an entry test `##primer-heads-per-directive`

Proposed by the agent in its reply to round 1, `argument@primer-common-root-part@a18`; heads named
in the reply to round 2, `argument@primer-common-root-part@a40`. State: `approved`. Shape:
"Harvest", and "The rewrite of `primer-limit`, and the end of `primer-content`" for the directives
the old head bundles, per D3, since the owner's words name the new sections only. The owner, round
2: "primer-heads-per-directive: this is a good idea, one head per section whenever it passes the
entry test." Round 3: "Agreed on all threads here, you can proceed".

### The primer is the part of every adopting project's root CLAUDE.md that is the same in every project `##primer-extends-every-root`

Proposed by the owner in round 2, `argument@primer-common-root-part@a25`; placed in
`design@agent-skills@primer-limit` by the agent in its reply to round 2,
`argument@primer-common-root-part@a38` and `argument@primer-common-root-part@a39`. State:
`approved`. Shape: "Decided design", "The rewrite of `primer-limit`, and the end of
`primer-content`". Harvest: the rewrite of
`design@agent-skills@primer-limit`, the pair #primer-extends-every-root →
`primer-limit`: the thread folds into an existing head, which keeps
its slug because it still names the same decision, `argument@primer-common-root-part@a38`; the pair
is stated so `git log --grep` finds the thread's name. The owner, round 2: "The primer can be
considered as an extension of the root CLAUDE.md of every consumer project. This way to put it
should be recorded, so any future session understand how it is expected to be shaped." Round 3:
"Agreed on all threads here, you can proceed".

## Arguments

The arguments are numbered from the transcript extraction, which a subagent made at the assembly;
its files were the scratch files `owner-words.md`, `deltas.md` and `arguments.md`, outside the
repository. Round n is the owner's n-th message and the agent's reply to it.

### The owner's request and its ground `##a1`

Round 1, the owner. Threads: all. "I believe a good part of it should be relocated to the primer
and shipped." "The main ground for that would be that thaum restates most of those itself ... and
I believe they are all sound instructions that should apply to any project."

### The plans subsection is fixed by the checker `##a2`

Round 1, the owner. Thread: `thread@primer-common-root-part@root-plans-subsection`.
"`### Plan documents` which is fixed by the checker IIRC."

### The primer explains nothing of the checker a reader meets `##a3`

Round 1, the owner. Threads: `thread@primer-common-root-part@checker-model-in-primer`,
`thread@primer-common-root-part@checker-cli-in-primer`. "the primer does not carry a section that
explains clearly things related to the checker, such as the minimal file layout, what components
are, register shapes and how items are declared, and how to cite those items, as well as basic usage
of klarch cli." "necessary in most if not all sessions ... from the very begining, even if only to
understand what they are reading and how to navigate the project."

### Shipping the whole section contradicts F23 `##a4`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-model-in-primer`.
"Moving all of `instructions@mechanical-validation` into the primer contradicts an earlier ruling of
yours." F23: "the rarer checker rules are delivered by their finding messages rather than by the
installed text"; narrowed by F69, kept by F121.

### Reading needs the model; writing an uncommon shape meets a finding `##a5`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-model-in-primer`.
"Every session needs the model to *read* the project, before any finding can help. The rarer rules
are only needed when *writing* a reference, and a finding then names the repair."

### thaum and this repository share one origin text `##a6`

Reply to round 1, the agent, measured with `git log -S` on thaum's root CLAUDE.md. Threads:
`thread@primer-common-root-part@verification-in-primer`,
`thread@primer-common-root-part@working-prose-in-primer`. The directives appear first in thaum's first
commit, 72c58617, of 2026-08-13; this repository imported them in d2f0ce44. "they are **not two
independent observations**". Only "Never chain a command on a verdict" came later, from 85ebaeba,
a thaum review finding.

### The sizes of the root's sections `##a7`

Reply to round 1, the agent, measured with `grep -n '^## '` on the root CLAUDE.md. Threads: all.
`mechanical-validation` 118 lines, `verify-before-relying` 92, `verify-mechanically` 76. Three
searchers read 141 standing entries, 84 issues and 57 tripwires.

### The checker's model is a provable gap `##a8`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-model-in-primer`. The
real-use rule admits "an input that an act needs, that no text supplies and that judgement cannot
derive, … such as a format the checker enforces". The setup skill "loads only when a project adopts
the workflow"; the primer "lists no kinds and no register shapes".

### Keeping the current state leaves reading unserved `##a9`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-model-in-primer`. "A
finding arrives only after something is written. Reading comes first". "Both projects added 30 to
118 lines to fill this gap by hand." The review re-measured: thaum's section on the checker is 87
lines and this repository's 118, and the lower bound 30 was not reproduced. thaum's section was
written "so a writer meets them before the finding".

### A command reaches only the session that runs it `##a10`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-model-in-primer`,
against rival B, the binary printing the model. It "loses on delivery: the harness puts the primer
in every session's context, including subagents." "A command reaches a session only if the session
thinks of running it." Drift stays bounded by `design@knowledge-architect@version-lockstep`.

### A hand-written list of anchors goes stale `##a11`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@anchors-listing-command`. "A
hand-written list goes stale when the manifest changes, and no check catches it. The command cannot
go stale." thaum's "What thaum declares" is 15 lines.

### The verification rules are behaviour instructions `##a12`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@verification-in-primer`. "The
obstacle is `design@agent-skills@behaviour-additions-need-real-use`." "Two months of use in two
projects shows they caused no harm. It does not show a session would lack them."

### The checker's CLI facts are already contracts `##a13`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@checker-cli-in-primer`. The
exit code, the verdict line, the pipe rule, the four phases and `--staged` "are facts about the
checker, and `path@core@README.md` already documents the exit codes and the verdict line as
contracts." `cargo x gates` stays, per `design@agent-skills@gates-convention`; `commits` after each
commit stays out, per its issue.

### The staged check's delivery changes `##a14`

Reply to round 1, the agent. Threads: `thread@primer-common-root-part@checker-cli-in-primer`,
`thread@primer-common-root-part@primer-extends-every-root`.
`design@agent-skills@staged-check-before-each-commit` has the "*setup skill recommend*" that the
root states the staged check; "Stating it in the primer instead changes that head's delivery".

### A project row that repeats a primer row is removed `##a15`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@root-plans-subsection`. Per
`design@agent-skills@knowledge-table-home` and `design@agent-skills@plans-directory-in-primer`.

### The language rules' jurisdiction is wider than their delivery `##a16`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@working-prose-in-primer`.
`discussion-language` governs "commit messages and code comments" but "is delivered only by the
design skill". "This contradiction is provable by reading, so no real-use evidence is needed."

### The em-dash bullet and the non-native premise are the owner's weighing `##a17`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@working-prose-in-primer`. They
"are your weighing, not argument"; "a fork for you".

### `primer-content` already bundles six decisions `##a18`

Reply to round 1, the agent. Thread: `thread@primer-common-root-part@primer-heads-per-directive`.
Adding members "makes that worse"; one head per section that passes an entry test closes the bundle
issue.

### The owner: the goal is reading `##a19`

Round 2, the owner. Thread: `thread@primer-common-root-part@checker-model-in-primer`. "yes, I agree,
the goal is to facilitate \"reading\", and the detailed findings do not need to be shipped in the
primer."

### The owner: a command delivers with no manual update `##a20`

Round 2, the owner. Thread: `thread@primer-common-root-part@anchors-listing-command`. "a good way to
deliver easily, without needing manual updates in each project, which often go stale."

### The owner's account of real use `##a21`

Round 2, the owner. Thread: `thread@primer-common-root-part@verification-in-primer`. "I doubt we'll
find evidence. However, you can trust my word that adding this section in thaum did its fair share
of work." "I have used this text, worded more or less similarly, in several projects already, and it
has definitively allowed to prevent lots of difficulties. It is text I built slowly over working
with LLM in multiple projects."

### The owner's mechanism `##a22`

Round 2, the owner. Thread: `thread@primer-common-root-part@verification-in-primer`. "LLM tend to
believe things will easily work out, and that the project they work on and the tools they use have
intuitive, known behavior. This is often not the case, sometimes by a lot."

### The owner: the project's purpose `##a23`

Round 2, the owner. Thread: `thread@primer-common-root-part@verification-in-primer`. "The whole
point of the knowledge-architect project, to me, is to bring all of these crumbs together and
deliver them to all of my other projects directly."

### The owner: check the overlap first `##a24`

Round 2, the owner. Thread: `thread@primer-common-root-part@verification-in-primer`. "rather than a
simple relocation, we should check how much it overlaps with other agent facing files." "The primer
might become the primary location (every session might face these problems IMO), with the design
skill pointing at it (and maybe restating part of it, or asking the agent to reread it)."

### The owner: the primer extends every root CLAUDE.md `##a25`

Round 2, the owner. Threads: `thread@primer-common-root-part@checker-cli-in-primer`,
`thread@primer-common-root-part@primer-extends-every-root`. "This would be simpler than having the
setup skill ask for it to be added to root CLAUDE.md. The primer can be considered as an extension
of the root CLAUDE.md of every consumer project. This way to put it should be recorded".

### The owner: merge the two style texts, and a lean toward keeping the em-dash rule `##a26`

Round 2, the owner. Thread: `thread@primer-common-root-part@working-prose-in-primer`. "Both version
from design skill and CLAUDE.md should be merged." "The em-dash/semicolon can be removed I guess.
Though I believe LLM tend to use those in excess, so I lean toward keeping it."

### The owner: readers might be non-native speakers `##a27`

Round 2, the owner. Thread: `thread@primer-common-root-part@working-prose-in-primer`. "\"Readers
might be non-native speakers\". This applies generally I believe. No project is ever contributed to
by only native english speakers." "the goal is that LLM prefer to use simpler sentences and words
whenever possible."

### The owner: one head per section `##a28`

Round 2, the owner. Thread: `thread@primer-common-root-part@primer-heads-per-directive`. "this is a
good idea, one head per section whenever it passes the entry test."

### The checker still requires the import line `##a29`

Reply to round 2, the agent, read in the code. Threads:
`thread@primer-common-root-part@checker-model-in-primer`,
`thread@primer-common-root-part@primer-extends-every-root`. `path@core@src/check/agents.rs` line 85 reports a root CLAUDE.md that
lacks it, with the repair "add a line holding exactly" the import line;
the tests at `path@core@src/agents.rs` lines 676 to 689 cover the placements it refuses.

### The overlap of V1 to V13 with the installed text `##a30`

Reply to round 2, the agent, from the overlap map. Thread:
`thread@primer-common-root-part@verification-in-primer`. No home at all: V2, V6, V9, V12, and the
second half of V13. A narrower home in one skill or agent: V1 (7 reviewer agents, the design and
planning skills), V3, V4, V5, V7, V8, V10. Already in the primer: V11, and the first half of V13.

### The planning skill compares to a check the primer does not state `##a31`

Reply to round 2, the agent, read. Thread: `thread@primer-common-root-part@verification-in-primer`.
`skill@knowledge-architect-planning@spec-contents` says a statement about existing code "is
checked against the tree before it is written, exactly as in a design head". "No sentence of
`primer@design-heads` requires that."

### The language rules are cited by nothing installed `##a32`

Reply to round 2, the agent, measured. Thread:
`thread@primer-common-root-part@working-prose-in-primer`. "No installed text cites
`skill@knowledge-architect-design@discussion-language`." "The design skill contains 74 em dashes;
the primer contains 0", counted with `grep -o` over the shipped sources, `%%` lines excluded. The
root's "Always" list holds "for every text a session writes"; the design skill excludes end-user
text.

### The owner's account meets the real-use rule `##a33`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@verification-in-primer`. "Your
account meets the real-use rule; it does not overrule it." Behaviour in the owner's other projects
is "a fact only you hold". The mechanism: "An agent assumes that the project and its tools behave as
their names, their documents and its own expectations suggest, and acts on that assumption without
checking it."

### The shape of `verify-before-relying` `##a34`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@verification-in-primer`. A new
section after `primer@intent-and-claims`; bullets 3 and 4 move into it; the design skill's steps 4
and 5 point to it and step 1 re-reads it, after
`design@agent-skills@primer-reread-before-recording`; the 7 agents keep "a brief is a lead"; the
root keeps the thaum figure, the two listing lines and the ground-first pointer. "Roughly 50 lines."

### A pointer hub has nothing to point to `##a35`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@verification-in-primer`,
against the primer as a pointer hub with each skill keeping its narrower rule. It "loses on the five
directives that have no home anywhere: a pointer hub has nothing to point to for them."

### The merged draft and the end-user scope `##a36`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@working-prose-in-primer`. The
draft "Readers might be non-native speakers: where a simpler word or a shorter sentence says the
same thing, use it." The default scope is the design skill's: "end-user text can have literary value
as a deliberate goal"; the root adds one sentence binding end-user text in this repository.

### The em dashes of existing text are not swept `##a37`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@working-prose-in-primer`. "The
74 em dashes in the design skill are not swept", per `design@agent-skills@restatement-size-test`.

### The owner's framing belongs in `primer-limit` `##a38`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@primer-extends-every-root`. It
"belongs in that head, not in a new one": the two answer one question and lose to one rival. The
title: "The primer is the part of every adopting project's root CLAUDE.md that is the same in every
project: what every session needs and no installed skill delivers."

### The framing's test, and its one case today `##a39`

Reply to round 2, the agent, read. Threads:
`thread@primer-common-root-part@primer-extends-every-root`,
`thread@primer-common-root-part@checker-cli-in-primer`. A statement the setup skill would ask every
root to write alike belongs in the primer. "Today exactly one such case exists, the staged-check
recommendation (`skill@knowledge-architect-setup@setup-gates`, line 187)." Line 187 is the installed
copy's; in the source, which step 2 edits, it is line 192. The import line is also identical in
every root, and the primer cannot carry it. The limit: it "does not admit project-specific content
into the primer."

### The heads the new sections earn `##a40`

Reply to round 2, the agent. Thread: `thread@primer-common-root-part@primer-heads-per-directive`.
The reply's table, by section: "`checker-model-in-primer` | agent-skills | test 1",
"`verify-before-relying` | agent-skills | test 4", "`working-prose` | agent-skills | test 2",
"`anchors-listing-command` | core | test 1". Each head takes its thread's slug, per
`primer@design-heads`. The review of this spec found the tests misassigned; the threads' harvest
lines give the corrected tests.

### The owner closes every thread `##a41`

Round 3, the owner. Threads: all. "Agreed on all threads here, you can proceed"

## New names, in one place

```
klarch anchors                     the command; `cargo klarch anchors` here
Command::Anchors                   path@core@src/cli/mod.rs, a variant of the existing enum
Anchor, Home, Constructed          path@core@src/entity.rs, existing
Anchors::of, Anchor::home_of       path@core@src/entity.rs, existing
## The project's documents and how they are checked {{slug:checker-model}}
                                   the primer source, a new level-two section
## Verify before relying {{slug:verify-before-relying}}
                                   the primer source, a new level-two section
## Working prose {{slug:working-prose}}
                                   the primer source, a new level-two section
checker-model-in-primer            a head of register@agent-skills@design
verification-in-primer             a head of register@agent-skills@design
working-prose-in-primer            a head of register@agent-skills@design, on Q1 only
anchors-listing-command            a head of register@core@design
rarer-rule-in-the-model            T2, a tripwire of register@agent-skills@tripwire
```

The heads of the bundled directives that pass test 2, if any, take slugs minted at the harvest.
The three section slugs are written as the build's slug placeholder in the source; installed,
each becomes a backticked `##` slug.

## Decided design

### The section `checker-model`

A new level-two section of the primer, placed before `primer@where-knowledge-goes`; an estimate
of its size is 50 lines. It holds what a session needs to read the project's records and cite
them, and the checker's general CLI facts. It holds no rarer rule. Its parts, in order:

1. **The manifest**, knowledge-architect.toml at the project's root: it declares the Components,
   the locations and the registers. `{{command}} anchors` lists what it resolves to.
2. **The anchors**: a Component, which carries the required documents and every component
   register; a location, which carries only the registers it declares, their homes directly under
   its path; and the anchors the checker constructs: `plans` at docs/plans/, one per milestone
   directory, one per spec file. The anchor words `*`, `elsewhere` and `plans` are reserved.
3. **What every Component carries**: `path@*@README.md`, `path@*@CLAUDE.md`,
   `path@*@docs/rejected-alternatives.md`, and the home of each component register:
   `register@*@goal`, `register@*@design`, `register@*@tripwire`, `register@*@issue`.
4. **The registers and their entries**. A heading register's home is one file, or a directory
   behind a `README.md` that links every subdocument; an entry is a heading at the register's level
   ending with its slug: level 3 for `design`, level 2 for `goal` and `tripwire`. A file register's
   home is a directory with one file per entry, a hand-written `README.md` and a generated
   `index.md`; an entry's id is its file's basename. A plan document is an entry of `spec` or of
   `milestone`, and a plan's items, of `thread`, `argument`, `criterion` and `acceptance`, are
   level-three headings under the section of their kind. An id matches `[a-z0-9]+(-[a-z0-9]+)*`.
5. **A reference**: one backticked span `<kind>@<anchor>@<id>`, where the kind is a register's
   name (each of the ten above, and any a project declares) and names one entry of it; or
   `register`, which names a register's home; or `path`, which names a file or a directory under
   the anchor; or `planned`, which names, in a plan document, a path its work will create. The
   harness kinds take no anchor: `skill@<name>`, `agent@<name>`, each with an optional section
   slug after a second `@`, and `primer@<slug>` and `instructions@<slug>`, the root CLAUDE.md's
   sections. When to write a reference, and the rules of a `path` reference, stay in
   `primer@where-knowledge-goes`, which this part points to.
6. **Reading**: `{{command}} show <reference>` prints the entry whole and every reference to it;
   `{{command}} issues` and `{{command}} tripwires` list the standing entries.
7. **Running the checker**: `{{command}} check` reads every document in four phases and stops at
   the first phase that finds anything; its exit code is the verdict, 0 for no finding, 1 for
   findings, 2 when it could not run, and its last line states it, `PASSED: no findings` or
   `FAILED: n findings above`. A finding names its repair. `{{command}} check --staged` judges the
   tree a commit will record, and is run after staging and before each commit; a failing staged
   check is repaired before the commit. `{{command}} commits <range>` judges each commit of a range,
   its message and its tree: a commit message is a document, and every reference in it resolves.
   Never chain a command on a verdict that went through a pipe: the shell takes the exit status of
   the pipe's last command.

The exit codes are those `path@core@README.md` documents under `design@core@exit-code-ladder`, and
the order of `check`'s output, verdict line last, is one of the two output contracts that README
states, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

The rarer rules, which this section does not state: the candidate rule, the two retired forms,
ignored targets, the path to anchor, a register's home cited by its path, the scope of a plan
item's citation, and a slug written outside a definition site. Each reaches a session through its
finding, per F23, F69 and F121. Two rules a writer of every `path` reference needs, the trailing
`/` and the deepest anchor, and the rule on a bare skill or agent name, are in
`primer@where-knowledge-goes` today and stay there.

The argument: `argument@primer-common-root-part@a3`, `argument@primer-common-root-part@a5`,
`argument@primer-common-root-part@a8`, `argument@primer-common-root-part@a9`. The nearest rival,
rival B, the binary printing the model, loses on delivery, `argument@primer-common-root-part@a10`,
and is a losing alternative. The rival "keep the current state" loses on
`argument@primer-common-root-part@a9`.

### The command `anchors`

`klarch anchors` prints every anchor the project has, the constructed ones included, built as
`check` builds them, with `Anchors::of` over the manifest and the tree's listing, in
`path@core@src/entity.rs`. For each: its name, its kind (component, location, plans, milestone,
spec) and its path. For a Component, a location and `plans`, it prints each register the anchor
carries with its home as the tree holds it, computed with `Anchor::home_of`: the file, the
directory, both where both exist, or `missing`. A milestone and a spec print no register: their
items' homes are sections of the plan document. The output is for a reader and is not a contract,
per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

It exits 0 when every home it prints exists, 1 when one is `missing`, a negative answer about its
subject, and 2 where it could not run, per `design@core@exit-code-ladder`. It is a command of the
core's CLI module, so a binary that registers extensions runs it unchanged, per
`design@core@the-core-cli-is-a-library-module`. `path@core@README.md` lists it with the other
commands and names it in the cells of the three rows of its exit-code table; the root README's
short command list, `path@knowledge-architect@README.md`, gains it too.

Its tests run the binary over the mock projects of `path@core@tests/projects/`: over `planted`, for
every anchor kind printed with exit 0; over `unsound`, whose manifest's comment names a missing
heading home, for the `missing` mark and exit 1. No new defect is planted, per
`path@core@CLAUDE.md`.

An illustration of a shape, not authority, cut to three anchors:

```
$ cargo klarch anchors
knowledge-architect  component  .
  goal      docs/goals.md
  design    docs/design.md
  tripwire  docs/tripwires.md
  issue     docs/open-issues/
agent-config  location  .claude
  issue     .claude/open-issues/
plans  plans  docs/plans
  spec       docs/plans/specs/
  milestone  docs/plans/milestones/
```

The argument: `argument@primer-common-root-part@a11` and `argument@primer-common-root-part@a20`.
The nearest rival, the hand-written list in each root CLAUDE.md, goes stale with no check,
`argument@primer-common-root-part@a11`.

### The section `verify-before-relying`

A new level-two section of the primer, placed right after `primer@intent-and-claims`; an estimate
of its size is 50 lines. Bullets 3 and 4 of `primer@intent-and-claims` move into it; bullets 1 and
2 stay. Outline:

- the lead: a document can describe the code as committed rather than as it is; confidence is not
  correlated with correctness (V13);
- before relying on how anything behaves, an API, a tool, a file format, an issue's diagnosis:
  names, doc comments and prose are leads, never evidence, and a claim is established from the
  implementation or from a run observed (V1 and the first half of V13, the old bullet 3); follow
  the whole path; one case is no convention; report the evidence with the conclusion and label an
  assumption; re-verify before acting; a transfer from another project names the property it
  depends on and checks it here (V2 to V6);
- evidence is textual and cheap: build it rather than trade intuitions (V7);
- a level-three subsection, "Verify a claim before writing it" (V8 to V10): a number comes from a
  measurement with its machine named, a count is re-counted, a search is run across line breaks
  against the revision the sentence describes, "X says this" is opened, a slug or a path is
  resolved, "this is fixed" is diffed; a commit message is a document: before each commit, list
  its factual claims and check each; the same holds for an answer to the owner;
- a level-three subsection, "Check whether it is already known" (V11, the old bullet 4): before
  diagnosing anything as a problem, or reporting one met while doing something else, the listing
  commands with `{{command}}`;
- a level-three subsection, "Precedent is not authority" (V12). It bars copying an existing
  defect into new text; `primer@where-knowledge-goes`' rule that an existing longer restatement is
  converted when a change edits it is about length, not defects, and the two do not conflict.

Every other text keeps what it states, and these gain pointers:

- the design skill's loop step 1 gains one sentence asking the session to read the primer's
  `verify-before-relying` again, whole, before grounding;
- the design skill's step 4 keeps its provenance sentence, which is the discussion's own, a claim
  doing closing work and what its assumptions become, and adds "as the primer's
  `verify-before-relying` asks of every claim" after "or assumed";
- the design skill's step 5 keeps its text, and its first sentence points to the primer for why
  evidence is built rather than argued;
- `skill@knowledge-architect-planning@spec-contents`' "exactly as in a design head" becomes "as
  the primer's `verify-before-relying` asks". Its subsection is named in words beside the section's
  reference, since no checked form cites a point inside a section, per
  `issue@core@no-checked-form-cites-a-point-inside-a-section`.

The narrower homes the overlap map found are not edited: the 7 agents' "Establish the state of the
tree yourself" sentence, the design skill's and the planning skill's "never act on an entry from
the reason the search gave", the planning skill's phrase-search sentence, its finding check and its
readiness check on numbers, the issue-tracking skill's cold-reader labels, the review skill's
"Reproduce before claiming" and its check of findings, and the decision-record reviewer's
predicates. Each states the act its own moment needs, narrower than the primer's directive, and
none is a copy of it.

The root CLAUDE.md keeps, under its slug, the thaum figure sentence, the two `ls docs/...-reports/`
lines and the ground-first pointer to `skill@klarch-development`; see "Mapping tables".

The argument: `argument@primer-common-root-part@a21`, `argument@primer-common-root-part@a22`,
`argument@primer-common-root-part@a24`, `argument@primer-common-root-part@a30`,
`argument@primer-common-root-part@a33`. The nearest rival, the primer as a pointer hub, loses on
`argument@primer-common-root-part@a35`.

### The section `working-prose`

A new level-two section of the primer, placed after `verify-before-relying`. The draft the owner
approved, written in the primer as it stands here:

```
These rules govern working prose: every text the project's contributors read, such as a
discussion with the owner, a document of the project, a commit message and a code comment.
Text the product's end users read follows the product's own style contract. The test is
who reads the text, not where it is stored.

- Write plain, direct technical English, in short sentences. Readers might be non-native
  speakers: where a simpler word or a shorter sentence says the same thing, use it.
- Use precise technical terms. No idioms, colloquialisms or aphorisms: they read
  differently to different readers, and they carry a qualitative judgement past argument.
- No undefined metaphor. A metaphor that names a defined concept is vocabulary once defined.
- Use em dashes and semicolons sparingly. They usually join two sentences that read
  better apart.
- Prefer explicit quantities, units and invariants over qualitative wording. A qualitative
  claim may state a goal or an observation. Offered as an argument, it is replaced by its
  checkable form: a count, a bound, an invariant, a named failure it prevents, or a test
  either party could run. The test: delete the sentence; if the argument falls, the
  sentence was the argument. An owner's ruling needs no such form.
```

The design skill's `discussion-language` section keeps only what is the discussion's own: a
qualitative claim opens a thread and never closes one, and the one-sentence compression; it points
to the primer's `working-prose`. The Overview's metaphor sentence becomes that pointer.
`skill@knowledge-architect-agent-configuration@admission-standard`'s line "Wording, style and
placement across files are judged by other rules" names the primer's `working-prose` for wording and
style. `path@agent-skills@README.md` names the primer at its line 67, the design skill's list, and
at line 330, "the skill's Language rule"; its lines 390 and 406 describe the skill's mechanisms and
their origin, and stay. The root CLAUDE.md's "Always" list is deleted; see "Mapping tables".

The argument: `argument@primer-common-root-part@a16`, `argument@primer-common-root-part@a26`,
`argument@primer-common-root-part@a27`, `argument@primer-common-root-part@a36`. The nearest rival,
the rules left in the design skill, loses on `argument@primer-common-root-part@a16`.

### The rewrite of `primer-limit`, and the end of `primer-content`

`design@agent-skills@primer-limit` takes the title "The primer is the part of every adopting
project's root CLAUDE.md that is the same in every project: what every session needs and no
installed skill delivers". Its body states the test that follows from it: a statement the setup
skill would ask every root CLAUDE.md to write alike belongs in the primer, the import line aside,
which the primer cannot carry; and the limit: what one project's root alone would say does not. It
attributes the framing to the owner, `argument@primer-common-root-part@a25`. Its body names the
primer's members, as `issue@agent-skills@primer-content-bundles-each-directive-s-decision` asks.

`design@agent-skills@primer-content`, whose title restates that rule, leaves the design home, per
D3. Each directive its body argues is recorded as the table below says, the entry tests of
`primer@design-heads` deciding at the harvest.

| directive argued in `primer-content` | where its decision is recorded after the work |
| --- | --- |
| the goals bind, and decisions as a presumption | a head of its own if test 2 holds, the primer, the goal-setting skill and the design skill's criteria stating it; else the reason beside the primer's instruction |
| a question goes to its authority | `design@agent-skills@questions-go-to-their-authority`, existing |
| the owner's word holds as far as its premise | `design@agent-skills@owner-word-holds-as-far-as-its-premise`, existing |
| room to judge | `goal@agent-skills@installed-text-leaves-room-to-judge` states it; the primer's sentence cites the goal, and no head |
| intent and claims: built intent, a reason at the code | `design@agent-skills@design-home-is-built-intent` and `design@agent-skills@local-intent-binds`, existing |
| a claim about the code goes stale, and the check before diagnosing | the head `verification-in-primer`, which absorbs them |
| something met outside the task | `design@agent-skills@met-outside-the-task-outcomes`, existing |
| the "Decided" section | `design@agent-skills@decisions-reported-to-the-owner`, existing |
| the labels and an answer's authority | `design@agent-skills@ruled-items-labelled` and `design@agent-skills@answer-keeps-its-authority`, existing |
| every commit of a branch judged at its tip | a head of its own if test 2 holds, the primer, the planning skill and the review skill stating it; else the reason beside the primer's instruction |

`design@agent-skills@staged-check-before-each-commit` is rewritten: the primer states the staged
check, and the setup skill's recommendation in `skill@knowledge-architect-setup@setup-gates` is
removed. The head keeps its argument, an exception to
`design@agent-skills@behaviour-additions-need-real-use` argued on delivery, and its rival,
dropping the recommendation until a real session shows the lack. The incumbent delivery, the setup
skill asking each root CLAUDE.md to state the check, lost to
`thread@primer-common-root-part@primer-extends-every-root` and is a losing alternative.

## Mapping tables

One table, the fate of every part of the five sections of the root CLAUDE.md the work changes:

| section, part | fate |
| --- | --- |
| `instructions@language-and-style`, "Always" | deleted; one sentence takes its place: "Here the primer's `working-prose` rules also bind text end users read", with the reference |
| `instructions@language-and-style`, "When presenting content to the user", "When writing into durable files" | kept |
| `instructions@mechanical-validation`, its opening restatement line | deleted |
| `instructions@mechanical-validation`, the manifest bullet | its general part moves to `checker-model` part 1; kept: one sentence, the pin `"self"` per `design@core@checked-sentinel-values` |
| `instructions@mechanical-validation`, the built-in registers, the anchors, the required documents, the register shapes, the entry shape, the kinds of reference and their resolution, the harness kinds, the register home's reference | moved to `checker-model` parts 2 to 5; deleted from the root |
| `instructions@mechanical-validation`, "One location is declared: `agent-config`" | deleted; `cargo klarch anchors` lists it, per the round-2 delta the owner approved in round 3, `argument@primer-common-root-part@a41` |
| `instructions@mechanical-validation`, the rarer rules: the candidate rule, the retired forms, the path to anchor, ignored targets, plan-item scope, a slug elsewhere, the bare skill name | deleted, per D4 |
| `instructions@mechanical-validation`, "A commit message is a document" | moved to `checker-model` part 7; deleted from the root |
| `instructions@where-knowledge-goes`, "Plan documents" | deleted, per `thread@primer-common-root-part@root-plans-subsection` |
| `instructions@verify-before-relying`, V1 to V13 and its three subsections | moved to the primer's `verify-before-relying`; deleted from the root |
| `instructions@verify-before-relying`, the thaum figure sentence, the ground-first pointer, the two `ls docs/...-reports/` lines | kept, under a one-sentence pointer to the primer's section |
| `instructions@verify-mechanically`, `cargo x gates`, its gates, its pipe refusal, the CI command, the gate list's home, the single gates run by hand | kept |
| `instructions@verify-mechanically`, "Each fails by its exit code", the verdict line, "Never chain a command on a verdict", the four phases and the table of the three checks, "What this project declares conformant" | moved to `checker-model` part 7, the table of checks deleted; deleted from the root |
| `instructions@verify-mechanically`, "Which gates you owe", the staged check before each commit | the staged check moves to `checker-model` part 7; kept: which gates a change owes, `cargo klarch commits origin/main..HEAD` after each commit with its range and `issue@core@branch-sha-citations-are-judged-within-the-range-only`, and the branch's `cargo x gates --require-rebased` |

The root keeps no copy of a directive the primer states: the primer and the root CLAUDE.md are one
text, and a duplicate inside one text fails `design@agent-skills@need-test`.

## Losing alternatives

- **The binary prints the model, and the primer points to the command** (rival B), lost to
  `thread@primer-common-root-part@checker-model-in-primer`: a command reaches a session only if it
  runs it, while the harness loads the primer into every session,
  `argument@primer-common-root-part@a10`.
- **Keep the current state**: the setup skill, the findings and `show`, lost to
  `thread@primer-common-root-part@checker-model-in-primer`: reading comes before any finding,
  `argument@primer-common-root-part@a9`.
- **Ship the rarer rules too**, lost to `thread@primer-common-root-part@checker-model-in-primer`:
  F23, F69 and F121, and the owner's round 2, `argument@primer-common-root-part@a19`.
- **A hand-written list of anchors in each root**, lost to
  `thread@primer-common-root-part@anchors-listing-command`, `argument@primer-common-root-part@a11`;
  a core decision, so its home, if the recording tests keep it, is
  `path@core@docs/rejected-alternatives.md`.
- **The setup skill asks each root CLAUDE.md to state the staged check**, the incumbent of
  `design@agent-skills@staged-check-before-each-commit`, lost to
  `thread@primer-common-root-part@primer-extends-every-root`: the primer is the part every root
  would say alike, `argument@primer-common-root-part@a25`.
- **The primer as a pointer hub, each skill keeping its narrower rule**, lost to
  `thread@primer-common-root-part@verification-in-primer`, `argument@primer-common-root-part@a35`.
- **Search the histories for real-use evidence**, option (a) of the reply to round 1, superseded by
  the owner's account, `argument@primer-common-root-part@a21`, which the agent judged meets the
  rule, `argument@primer-common-root-part@a33`.
- **The language rules left in the design skill**, lost to
  `thread@primer-common-root-part@working-prose-in-primer`, `argument@primer-common-root-part@a16`.
- **The em-dash bullet kept in the root only**, the agent's default of the reply to round 1. The
  owner leaned toward keeping the bullet in round 2, `argument@primer-common-root-part@a26`, and
  approved in round 3 the draft that ships it, `argument@primer-common-root-part@a36` and
  `argument@primer-common-root-part@a41`.
- **Keep the plans subsection as a pointer**, lost to
  `thread@primer-common-root-part@root-plans-subsection`, `argument@primer-common-root-part@a15`.
- **A head of its own for the owner's framing**, lost to
  `thread@primer-common-root-part@primer-extends-every-root`: one question, one rival,
  `argument@primer-common-root-part@a38`.

## Readings

The work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| The primer grows from 454 to about 600 lines, and in long sessions the new sections are not applied. | `thread@primer-common-root-part@verification-in-primer`, `thread@primer-common-root-part@primer-extends-every-root` | proposed as T1, a tripwire; declined by the owner, D2 |
| The model section drifts from the checker: a kind, an anchor word or a register shape changes in the core, and the primer is not updated. | `thread@primer-common-root-part@checker-model-in-primer` | AC1, `acceptance@primer-common-root-part@model-lists-every-kind` |
| Adopting projects keep their own root copies of the moved text, which drift from the primer. | `thread@primer-common-root-part@primer-extends-every-root` | AC2, `acceptance@primer-common-root-part@migration-names-the-sections` |
| The model section gains rarer rules over time, one per confusion, against F23. | `thread@primer-common-root-part@checker-model-in-primer` | T2, a tripwire, `rarer-rule-in-the-model`, recorded on the owner's word, D2 |

The tripwires as put to the owner. The owner ruled D2 to record T2 and not T1:

- **T1.** Fires when a retrospective, in any project, reports a session that acted against a
  directive of `verify-before-relying` or `working-prose` that its primer stated at the time.
  Response: add re-read instructions at the moments of use, as
  `design@agent-skills@primer-reread-before-recording` did. Re-entry: that retrospective's intake.
- **T2.** Fires when a review or an audit finds a rule in `checker-model` that a finding message
  already delivers with its repair. Response: remove it, and reopen F23's scope with the owner.
  Re-entry: that review.

## Acceptance criteria

### AC1: a core test holds every reference kind and reserved anchor word to appear in the primer's model section `##model-lists-every-kind`

Applied on the owner's word, D2, on this text as put to the owner: "A core test asserts that every
reference kind the checker accepts, and the reserved anchor words, appear in the shipped primer's
`checker-model` section. Judged at the step that writes the section." Guards
`thread@primer-common-root-part@checker-model-in-primer`. Judged at step 2. Observable: the test,
in the core's suite, reads the shipped primer as the install renders it and asserts each kind and
each reserved word appears between the section's heading and the next level-two heading; the kinds
are the constants of `path@core@src/entity.rs`, `PATH_KIND`, `PLANNED_KIND`, `REGISTER_KIND` and
`HARNESS_KINDS`, and the built-in register names of `path@core@src/manifest.rs`. Fires when the
test cannot be written against a fixed list, because the kinds are not enumerable. Response:
reopen how the section is delivered, against rival B.

### AC2: the changelog's Migration entry names each root section an adopting project can delete `##migration-names-the-sections`

Applied on the owner's word, D2, on this text as put to the owner: "The changelog's Migration entry
names each root section an adopting project can now delete: the checker model, the verification
rules, the working-prose rules, the checker's CLI facts. Judged at the step that writes the
changelog." Guards `thread@primer-common-root-part@primer-extends-every-root`. Judged at step 3.
Observable: the `Next release` section of CHANGELOG.md, which step 3 creates above the newest
released section, holds that entry. It is a Migration entry under
`design@knowledge-architect@changelog-entries` because the change gives a consumer a thing to
change: once the primer states a directive, a root CLAUDE.md's copy of it is a duplicate inside one
text, which `design@agent-skills@need-test` refuses. Fires when the entry cannot name them as
sections, because the content was rewritten beyond recognition. Response: a one-time audit pass of
the agentic-workflow axis on each adopting project.

## Implementation sequence

1. **The command `anchors`.** `Command::Anchors` in `path@core@src/cli/mod.rs`, its output from
   `Anchors::of`, its line in `path@core@README.md`'s command list and in the cells of its
   exit-code table, its line in `path@knowledge-architect@README.md`, and its tests over `planted`
   and `unsound`, as "The command `anchors`" says; a changelog entry, `cli`, minor. Fails alone on:
   the command's tests.
2. **The shipped text.** The primer's three sections and the two moved bullets; the design skill's
   steps 1, 4 and 5, its `discussion-language` section and its Overview sentence; the setup skill's
   `setup-gates` recommendation removed; the planning skill's pointer; the agent-configuration
   skill's line on wording and style; `path@agent-skills@README.md` lines 67 and 330; the `%%`
   comment of the primer's lines 3 and 4; AC1's test; `cargo klarch install-agent-skills` for the
   installed copies, in the same commit as the sources. Fails alone on: AC1's test and the
   shipped-set tests of `path@core@src/agents.rs`.
3. **This repository's own text.** The root CLAUDE.md's five sections, as "Mapping tables" says; the
   re-pointed texts of "What is already decided"; the CHANGELOG.md entries of `Next release`, AC2's
   Migration entry among them, then `cargo x changelog`. Fails alone on: `cargo klarch check` over
   the re-pointed references, and AC2.
4. **The harvest**, as the next section says, then the report on AC1 and AC2, and the spec's
   deletion.

## Order rationale

- Step 1 before step 2: the primer names `{{command}} anchors`, which must exist when the text
  ships.
- Step 2 before step 3: the root's pointers name primer sections that step 2 creates.
- Step 3 before step 4: the heads describe the delivery as built, the root's included.

## Defaults awaiting the owner

- **D1**, ruled: one spec on one branch, in the steps above, rather than a milestone of two
  slices. Bears on every thread. The ground: no design session is needed between the command and
  the text. The owner: "D1 agreed."
- **D2**, ruled: the premortem's verdicts, proposed as T1 and T2 recorded as tripwires, AC1 and
  AC2 applied. The owner: "D2: Keep only T2, AC1 and AC2". T1 is not recorded.
- **D3**, awaiting: `design@agent-skills@primer-content` leaves the design home, its member list
  moving into `primer-limit`'s body and each directive it argues recorded as the table of "The
  rewrite of `primer-limit`, and the end of `primer-content`" says. Bears on
  `thread@primer-common-root-part@primer-heads-per-directive`, whose owner's words name the new
  sections only: the bundled directives are a widening, raised by the review. The ground: the
  bundle issue's own closing condition.
- **D4**, awaiting: the rarer rules are deleted from the root CLAUDE.md, not only left unshipped.
  Bears on `thread@primer-common-root-part@checker-model-in-primer`. The agent's default in its
  reply to round 1 read "The rarer rules leave the root CLAUDE.md and are not shipped"; the owner's
  answer, "the detailed findings do not need to be shipped in the primer", names the primer only.
- **Q1**, awaiting, entry test 4 of `primer@design-heads` for
  `thread@primer-common-root-part@working-prose-in-primer`: do the owner's words "I lean toward
  keeping it" (the em-dash bullet) and "\"Readers might be non-native speakers\". This applies
  generally I believe." record the owner's intent, or were they an answer to the agent's proposal,
  or a hedged statement? Default: no head, the reasons staying beside the instructions in the
  primer.

## Harvest

| what | where | when |
| --- | --- | --- |
| `thread@primer-common-root-part@checker-model-in-primer`, a head | `register@agent-skills@design`, under "The configuration a project holds" | step 4 |
| `thread@primer-common-root-part@verification-in-primer`, a head by test 2, its evidence attributed to the owner | `register@agent-skills@design` | step 4 |
| `thread@primer-common-root-part@working-prose-in-primer`, a head by test 4 only on Q1 | `register@agent-skills@design` | step 4 |
| #primer-extends-every-root → `primer-limit`, rewritten | `register@agent-skills@design` | step 4 |
| `primer-content` leaves; its member list moves to `primer-limit`; its bundled directives as the table of "Decided design" says, per D3 | `register@agent-skills@design` | step 4 |
| `thread@primer-common-root-part@checker-cli-in-primer`: `staged-check-before-each-commit` rewritten | `register@agent-skills@design` | step 4 |
| `thread@primer-common-root-part@anchors-listing-command`, a head | `register@core@design` | step 4 |
| rival B, the setup skill's staged-check delivery, and the other losing alternatives of agent-skills the recording tests keep | `path@agent-skills@docs/rejected-alternatives.md` | step 4 |
| the hand-written list of anchors, if the recording tests keep it | `path@core@docs/rejected-alternatives.md` | step 4 |
| T2, `rarer-rule-in-the-model`, on the owner's word, D2 | `register@agent-skills@tripwire` | step 4 |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` closes | its file is deleted | step 4 |
| `path@agent-skills@CLAUDE.md`'s restatement of `primer-limit` | rewritten | step 4 |
| `thread@primer-common-root-part@root-plans-subsection` | no head: a member of `design@agent-skills@knowledge-table-home` | — |

## Later consequences

- thaum deletes its restatements at its next pin move, guided by AC2's Migration entry.
- `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` and
  `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`, when real use
  shows their need, add a line to `verify-before-relying` or to `checker-model` instead of to the
  setup skill.
- A future reference kind or anchor word fails AC1's test until the primer names it.
