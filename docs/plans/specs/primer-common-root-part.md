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
- the design, setup and planning skills point to those sections where they restated or
  narrowed them;
- this repository's root CLAUDE.md keeps only what is its own;
- the agent-skills design heads on the primer are rewritten, one head per new section.

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
  The discussion begins at the owner's message that opens "I would like to discuss the
  repartition of instructions between this project's root CLAUDE.md." It holds 3 owner messages,
  called rounds 1 to 3, and the agent's reply to each. Find the file by that opening message, not
  by its name.

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
- **the rarer rules**: the checker rules that `instructions@mechanical-validation` states and that
  a writer needs only when writing a reference of an uncommon shape: the candidate rule, the two
  retired forms, the trailing `/` claim, ignored targets, the path to anchor, the bare skill or
  agent name, and the `planned` form.
- **F23, F69, F121**: the owner's rulings on the agentic-workflow audit, recorded in
  `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`: the rarer checker
  rules are delivered by their finding messages, not by the installed text, and an unstated checker
  rule is a finding only where its finding message does not name its repair.
- **V1 to V13**: the directives of `instructions@verify-before-relying`, numbered by the overlap
  map of round 2, which called them D1 to D13, #a30:
  V1 names and prose are leads, V2 follow the whole path, V3 one case is no convention, V4 report
  the evidence and label assumptions, V5 re-verify before acting, V6 a transfer names its property,
  V7 evidence is textual and cheap, V8 a checkable claim is checked before it is written, V9 a
  commit message is a document whose claims are listed and checked, V10 the checks apply to
  answers to the owner, V11 check whether it is already recorded, V12 precedent is not authority,
  V13 a document can describe the code as committed, and confidence is not correlated with
  correctness.
- **the overlap maps**: the two read-only inventories of round 2, of the installed text against
  V1 to V13 and against the language rules.
- **`{{command}}`**: the placeholder the build fills with the project's declared command, per
  `design@core@declared-command`; the shipped text writes the checker's command with it.

## What the work is

### What exists today

- **The primer** has 12 level-two sections, from `primer@goals-bind` to
  `primer@installed-skills`. `primer@intent-and-claims` holds four bullets: built intent, a reason
  recorded at the code, a claim about the code goes stale (bullet 3), and the check before
  diagnosing (bullet 4). `primer@where-knowledge-goes` holds the knowledge table and one paragraph
  on the reference grammar. No section lists the reference kinds, the anchor kinds or the register
  shapes, or says how to run the checker.
- **The setup skill** states the layout a Component carries, in
  `skill@knowledge-architect-setup@components-and-locations` and
  `skill@knowledge-architect-setup@component-documents`, and loads only when a project adopts the
  workflow or moves its pin. `skill@knowledge-architect-setup@setup-gates` recommends that the root
  CLAUDE.md state `check --staged` before each commit, per
  `design@agent-skills@staged-check-before-each-commit`.
- **The design skill** holds the language rules in `skill@knowledge-architect-design@discussion-language`,
  whose jurisdiction is all working prose, commit messages and code comments included; no
  installed text cites that section. Its Overview holds the metaphor rule. Its loop step 4 holds a
  provenance rule for a factual claim doing closing work, and step 5 the evidence built when a
  discussion stalls.
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
  `Anchor` in `path@core@src/entity.rs` holds each anchor's name, path, registers, whether it is a
  Component, and which constructed kind it is.
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
- **The em dashes of the existing installed text are not swept.** Under
  `design@agent-skills@restatement-size-test`, existing text is converted when a change edits it.
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
| `design@agent-skills@primer-content` | keeps the list of the primer's members; each bundled decision with a rival of its own gets a head, per `thread@primer-common-root-part@primer-heads-per-directive` | `issue@agent-skills@primer-content-bundles-each-directive-s-decision`, which closes at step 4 |
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
| `instructions@verify-mechanically` | `path@agent-config@skills/klarch-development/SKILL.md` line 184, `path@agent-config@skills/klarch-release/SKILL.md` line 46 | the gates and the checks before and after a commit | step 3: re-read |
| `instructions@verify-mechanically` | `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text` line 46 | the rule that a failing staged check is repaired before the commit, which moves | step 3: re-pointed to the primer's section |
| `skill@knowledge-architect-design@discussion-language` | the root CLAUDE.md line 25 | the language rules, which move | step 3: rewritten |

### The standing entries the search returned

Three searchers read the 141 issues and tripwires at the grounding. The entries they returned, with
their outcome:

| entry | outcome |
| --- | --- |
| `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair` | holds F23, F69 and F121: the rarer rules are not shipped |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` | closed at step 4 |
| `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` | stays open; its addition is not shipped |
| `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search` | stays open; its line 28 is re-pointed |
| `issue@agent-config@a-pending-retrospective-analysis-is-reached-by-no-standing-search` | stays open; its line stays in the root |
| `issue@knowledge-architect@command-output-is-not-declared-a-contract` | the primer states only the exit codes and the verdict line, which `path@core@README.md` already states as contracts; the `anchors` output is for reading, not a contract |
| `issue@agent-skills@an-upgrade-re-reads-no-root-restatement` | bears: the move removes root restatements; it stays open |
| `issue@core@a-span-with-an-empty-head-is-malformed-against-its-head` | the primer states no rule on it, so it takes no side |
| `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text` | its line 46 is re-pointed; step 2 commits the primer source and its installed copy together |
| `issue@core@branch-sha-citations-are-judged-within-the-range-only` | the root keeps its `commits origin/main..HEAD` line and the pointer |
| `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` | the moved text is read at review for any entry it cites |
| `issue@agent-skills@the-setup-section-s-toml-and-ci-blocks-are-unchecked` | the new primer section holds no TOML or CI block |
| `issue@core@a-checker-only-project-carries-the-workflow-skeleton` | the primer states the documents a Component carries today; it stays open |
| `issue@agent-skills@a-skill-for-creating-a-component` | bears: `checker-model` states what a Component carries, not how to create one; it stays open |
| `issue@core@installed-file-findings-belong-in-phase-four` | the primer names the four phases without naming which phase reports an installed file |
| `issue@core@a-plain-hash-item-name-is-read-by-nothing`, `issue@core@no-checked-form-cites-a-point-inside-a-section`, `issue@core@references-are-read-in-markdown-and-rust-only`, `issue@core@cross-project-references`, `issue@core@four-heads-bundle-decisions-with-many-citing-sites`, `issue@core@a-bare-mention-of-a-deleted-entry-is-never-reported`, `issue@core@a-component-states-at-least-one-goal`, `issue@core@a-home-for-developer-contracts-outside-agent-configuration` | bear on nothing the work changes: it states no plan-item rule, deletes no slug, and moves no row of the knowledge table |
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

Binding as a presumption, from `design@agent-skills@behaviour-additions-need-real-use`. Met:
the checker's facts are a provable gap, a fact the checker fixes; the verification rules rest on
the owner's account of real sessions in several projects, #a21 and #a22; the language rules on a
contradiction provable by reading, #a16.

### The rarer checker rules reach a session through their findings `##rarer-rules-by-findings`

Binding, the owner's rulings F23, F69 and F121. Met: `checker-model` holds the model and the rules
every reference follows, and no rarer rule; the owner confirmed in round 2, #a19.

### A restatement is no longer than a pointer `##restatement-size`

Binding as a presumption, from `design@agent-skills@restatement-size-test`. Met: every
restatement left in the root CLAUDE.md or in a skill is one sentence with its pointer.

## Threads

### The primer holds the checker's model a reader needs, and not the rarer rules `##checker-model-in-primer`

Proposed by the owner in round 1, #a3; the agent added the exclusion of the rarer rules in its
reply to round 1, #a4 and #a5. State: `approved`. Arguments: #a3, #a5, #a8, #a9, #a10, #a19.
Shape: "Decided design", "The section `checker-model`". Harvest: a head of agent-skills.
The owner, round 2: "About instructions@mechanical-validation: yes, I agree, the goal is to
facilitate \"reading\", and the detailed findings do not need to be shipped in the primer." and
"checker-model-in-primer: agreed on that shape, in the primer." Round 3: "Agreed on all threads
here, you can proceed".

### A core command lists the project's anchors, their registers and their homes `##anchors-listing-command`

Proposed by the agent in its reply to round 1, #a11. State: `approved`. Arguments: #a11, #a20.
Shape: "Decided design", "The command `anchors`". Harvest: a head of core. The owner, round 2:
"anchors-listing-command: good idea. This is a good way to deliver easily, without needing manual
updates in each project, which often go stale." Round 3: "Agreed on all threads here, you can
proceed".

### The primer holds the verification rules, and the skills point to it `##verification-in-primer`

Proposed by the owner in round 1, #a1; the agent raised the real-use obstacle in its reply to
round 1, #a12; the owner answered it and asked for the overlap check in round 2, #a21 to #a24; the
agent gave the shape in its reply to round 2, #a30 to #a35. State: `approved`. Shape: "Decided
design", "The section `verify-before-relying`". Harvest: a head of agent-skills. The owner, round
2: "verification-in-primer: I doubt we'll find evidence. However, you can trust my word that
adding this section in thaum did its fair share of work. LLM tend to believe things will easily
work out, and that the project they work on and the tools they use have intuitive, known behavior.
This is often not the case, sometimes by a lot." and "The primer might become the primary location
(every session might face these problems IMO), with the design skill pointing at it (and maybe
restating part of it, or asking the agent to reread it)." Round 3: "Agreed on all threads here,
you can proceed".

### The primer states the checker's general CLI facts, and the root keeps the project's gates `##checker-cli-in-primer`

Proposed by the owner in round 1, #a1 ("Part of ##verify-mechanically could be too.", the backticks dropped so that the slug defines nothing here); shaped by
the agent in its reply to round 1, #a13 and #a14. State: `approved`. Shape: "Decided design", "The
section `checker-model`", its last part. Harvest: a member of `design@agent-skills@primer-content`,
and the rewrite of `design@agent-skills@staged-check-before-each-commit`; no head of its own. The
owner, round 2: "checker-cli-in-primer: agreed. This would be simpler than having the setup skill
ask for it to be added to root CLAUDE.md." Round 3: "Agreed on all threads here, you can proceed".

### The root CLAUDE.md's "Plan documents" subsection is deleted `##root-plans-subsection`

Proposed by the owner in round 1, #a2. Closed `argued` by the agent in its reply to round 1, #a15:
a project row that only repeats a primer row is removed, per
`design@agent-skills@knowledge-table-home` and `design@agent-skills@plans-directory-in-primer`.
The rival, keeping it as a pointer, loses to that rule. Shape: step 3. Harvest: none, a member of
`design@agent-skills@knowledge-table-home`'s rule. The owner, round 2: "root-plans-subsection:
agreed."

### The primer holds the working-prose rules, merged from the design skill and the root `##working-prose-in-primer`

Proposed by the owner in round 1, #a1 ("And the Always subsection of ##language-and-style.", the backticks dropped);
widened by the agent to the design skill's section in its reply to round 1, #a16 and #a17; the
owner ruled the merge and the two wordings in round 2, #a26 and #a27; the agent gave the draft and
the scope default in its reply to round 2, #a36 and #a37. State: `approved`. Shape: "Decided
design", "The section `working-prose`". Harvest: a head of agent-skills. The owner, round 2:
"working-prose-in-primer: Both version from design skill and CLAUDE.md should be merged. The
em-dash/semicolon can be removed I guess. Though I believe LLM tend to use those in excess, so I
lean toward keeping it.  \"Readers are non-native speakers\" can become  \"Readers might be
non-native speakers\". This applies generally I believe. No project is ever contributed to by only
native english speakers. Maybe the sentence should be completed, the goal is that LLM prefer to use
simpler sentences and words whenever possible." Round 3: "Agreed on all threads here, you can
proceed".

### Each new primer section gets a head when it passes an entry test `##primer-heads-per-directive`

Proposed by the agent in its reply to round 1, #a18; heads named in the reply to round 2, #a40.
State: `approved`. Shape: "Harvest". The owner, round 2: "primer-heads-per-directive: this is a
good idea, one head per section whenever it passes the entry test." Round 3: "Agreed on all
threads here, you can proceed".

### The primer is the part of every adopting project's root CLAUDE.md that is the same in every project `##primer-extends-every-root`

Proposed by the owner in round 2, #a25; placed in `design@agent-skills@primer-limit` by the agent
in its reply to round 2, #a38 and #a39. State: `approved`. Shape: "Decided design", "The rewrite
of `primer-limit`". Harvest: the rewrite of `design@agent-skills@primer-limit`, the pair
#primer-extends-every-root → `primer-limit`, since the thread's name describes the head's new
statement but the head keeps its slug. The owner, round 2: "The primer can be considered as an
extension of the root CLAUDE.md of every consumer project. This way to put it should be recorded,
so any future session understand how it is expected to be shaped." Round 3: "Agreed on all
threads here, you can proceed".

## Arguments

The arguments are numbered from the transcript extraction, which a subagent made at the assembly;
its files were the scratch files `owner-words.md`, `deltas.md` and `arguments.md`, outside the
repository. Round n is the owner's n-th message and the agent's reply to it.

### The owner's request and its ground `##a1`

Round 1, the owner. Threads: all. "I believe a good part of it should be relocated to the primer
and shipped." "The main ground for that would be that thaum restates most of those itself ... and
I believe they are all sound instructions that should apply to any project."

### The plans subsection is fixed by the checker `##a2`

Round 1, the owner. Thread: #root-plans-subsection. "`### Plan documents` which is fixed by the
checker IIRC."

### The primer explains nothing of the checker a reader meets `##a3`

Round 1, the owner. Threads: #checker-model-in-primer, #checker-cli-in-primer. "the primer does not
carry a section that explains clearly things related to the checker, such as the minimal file
layout, what components are, register shapes and how items are declared, and how to cite those
items, as well as basic usage of klarch cli." "necessary in most if not all sessions ... from the
very begining, even if only to understand what they are reading and how to navigate the project."

### Shipping the whole section contradicts F23 `##a4`

Reply to round 1, the agent. Thread: #checker-model-in-primer. "Moving all of
`instructions@mechanical-validation` into the primer contradicts an earlier ruling of yours." F23:
"the rarer checker rules are delivered by their finding messages rather than by the installed
text"; narrowed by F69, kept by F121.

### Reading needs the model; writing an uncommon shape meets a finding `##a5`

Reply to round 1, the agent. Thread: #checker-model-in-primer. "Every session needs the model to
*read* the project, before any finding can help. The rarer rules are only needed when *writing* a
reference, and a finding then names the repair."

### thaum and this repository share one origin text `##a6`

Reply to round 1, the agent, measured with `git log -S` on thaum's root CLAUDE.md. Threads:
#verification-in-primer, #working-prose-in-primer. The directives appear first in thaum's first
commit, 72c58617, of 2026-08-13; this repository imported them in d2f0ce44. "they are **not two
independent observations**". Only "Never chain a command on a verdict" came later, from 85ebaeba,
a thaum review finding.

### The sizes of the root's sections `##a7`

Reply to round 1, the agent, measured with `grep -n '^## '` on the root CLAUDE.md. Threads: all.
`mechanical-validation` 118 lines, `verify-before-relying` 92, `verify-mechanically` 76. Three
searchers read 141 standing entries, 84 issues and 57 tripwires.

### The checker's model is a provable gap `##a8`

Reply to round 1, the agent. Thread: #checker-model-in-primer. The real-use rule admits "an input
that an act needs, that no text supplies and that judgement cannot derive, … such as a format the
checker enforces". The setup skill "loads only when a project adopts the workflow"; the primer
"lists no kinds and no register shapes".

### Keeping the current state leaves reading unserved `##a9`

Reply to round 1, the agent. Thread: #checker-model-in-primer. "A finding arrives only after
something is written. Reading comes first". "Both projects added 30 to 118 lines to fill this gap
by hand." thaum's section was written "so a writer meets them before the finding".

### A command reaches only the session that runs it `##a10`

Reply to round 1, the agent. Thread: #checker-model-in-primer, against rival B, the binary
printing the model. It "loses on delivery: the harness puts the primer in every session's context,
including subagents." "A command reaches a session only if the session thinks of running it."
Drift stays bounded by `design@knowledge-architect@version-lockstep`.

### A hand-written list of anchors goes stale `##a11`

Reply to round 1, the agent. Thread: #anchors-listing-command. "A hand-written list goes stale when
the manifest changes, and no check catches it. The command cannot go stale." thaum's "What thaum
declares" is 15 lines.

### The verification rules are behaviour instructions `##a12`

Reply to round 1, the agent. Thread: #verification-in-primer. "The obstacle is
`design@agent-skills@behaviour-additions-need-real-use`." "Two months of use in two projects shows
they caused no harm. It does not show a session would lack them."

### The checker's CLI facts are already contracts `##a13`

Reply to round 1, the agent. Thread: #checker-cli-in-primer. The exit code, the verdict line, the
pipe rule, the four phases and `--staged` "are facts about the checker, and `path@core@README.md`
already documents the exit codes and the verdict line as contracts." `cargo x gates` stays, per
`design@agent-skills@gates-convention`; `commits` after each commit stays out, per its issue.

### The staged check's delivery changes `##a14`

Reply to round 1, the agent. Threads: #checker-cli-in-primer, #primer-extends-every-root.
`design@agent-skills@staged-check-before-each-commit` has the "*setup skill recommend*" that the
root states the staged check; "Stating it in the primer instead changes that head's delivery".

### A project row that repeats a primer row is removed `##a15`

Reply to round 1, the agent. Thread: #root-plans-subsection. Per
`design@agent-skills@knowledge-table-home` and `design@agent-skills@plans-directory-in-primer`.

### The language rules' jurisdiction is wider than their delivery `##a16`

Reply to round 1, the agent. Thread: #working-prose-in-primer. `discussion-language` governs
"commit messages and code comments" but "is delivered only by the design skill". "This
contradiction is provable by reading, so no real-use evidence is needed."

### The em-dash bullet and the non-native premise are the owner's weighing `##a17`

Reply to round 1, the agent. Thread: #working-prose-in-primer. They "are your weighing, not
argument"; "a fork for you".

### `primer-content` already bundles six decisions `##a18`

Reply to round 1, the agent. Thread: #primer-heads-per-directive. Adding members "makes that
worse"; one head per section that passes an entry test closes the bundle issue.

### The owner: the goal is reading `##a19`

Round 2, the owner. Thread: #checker-model-in-primer. "yes, I agree, the goal is to facilitate
\"reading\", and the detailed findings do not need to be shipped in the primer."

### The owner: a command delivers with no manual update `##a20`

Round 2, the owner. Thread: #anchors-listing-command. "a good way to deliver easily, without
needing manual updates in each project, which often go stale."

### The owner's account of real use `##a21`

Round 2, the owner. Thread: #verification-in-primer. "I doubt we'll find evidence. However, you can
trust my word that adding this section in thaum did its fair share of work." "I have used this
text, worded more or less similarly, in several projects already, and it has definitively allowed
to prevent lots of difficulties. It is text I built slowly over working with LLM in multiple
projects."

### The owner's mechanism `##a22`

Round 2, the owner. Thread: #verification-in-primer. "LLM tend to believe things will easily work
out, and that the project they work on and the tools they use have intuitive, known behavior. This
is often not the case, sometimes by a lot."

### The owner: the project's purpose `##a23`

Round 2, the owner. Thread: #verification-in-primer. "The whole point of the knowledge-architect
project, to me, is to bring all of these crumbs together and deliver them to all of my other
projects directly."

### The owner: check the overlap first `##a24`

Round 2, the owner. Thread: #verification-in-primer. "rather than a simple relocation, we should
check how much it overlaps with other agent facing files." "The primer might become the primary
location (every session might face these problems IMO), with the design skill pointing at it (and
maybe restating part of it, or asking the agent to reread it)."

### The owner: the primer extends every root CLAUDE.md `##a25`

Round 2, the owner. Threads: #checker-cli-in-primer, #primer-extends-every-root. "This would be
simpler than having the setup skill ask for it to be added to root CLAUDE.md. The primer can be
considered as an extension of the root CLAUDE.md of every consumer project. This way to put it
should be recorded".

### The owner: merge the two style texts, keep the em-dash rule `##a26`

Round 2, the owner. Thread: #working-prose-in-primer. "Both version from design skill and
CLAUDE.md should be merged." "The em-dash/semicolon can be removed I guess. Though I believe LLM
tend to use those in excess, so I lean toward keeping it."

### The owner: readers might be non-native speakers `##a27`

Round 2, the owner. Thread: #working-prose-in-primer. "\"Readers might be non-native speakers\".
This applies generally I believe. No project is ever contributed to by only native english
speakers." "the goal is that LLM prefer to use simpler sentences and words whenever possible."

### The owner: one head per section `##a28`

Round 2, the owner. Thread: #primer-heads-per-directive. "this is a good idea, one head per
section whenever it passes the entry test."

### The checker still requires the import line `##a29`

Reply to round 2, the agent, read in the code. Threads: #checker-model-in-primer,
#primer-extends-every-root. `path@core@src/check/agents.rs` line 85 reports a root CLAUDE.md that
lacks it, with the repair "add a line holding exactly" the import line;
the tests at `path@core@src/agents.rs` lines 676 to 689 cover the placements it refuses.

### The overlap of V1 to V13 with the installed text `##a30`

Reply to round 2, the agent, from the overlap map. Thread: #verification-in-primer. No home at
all: V2, V6, V9, V12, and the second half of V13. A narrower home in one skill or agent: V1 (7
reviewer agents, the design and planning skills), V3, V4, V5, V7, V8, V10. Already in the primer:
V11, and the first half of V13.

### The planning skill compares to a check the primer does not state `##a31`

Reply to round 2, the agent, read. Thread: #verification-in-primer.
`skill@knowledge-architect-planning@spec-contents` says a statement about existing code "is
checked against the tree before it is written, exactly as in a design head". "No sentence of
`primer@design-heads` requires that."

### The language rules are cited by nothing installed `##a32`

Reply to round 2, the agent, measured. Thread: #working-prose-in-primer. "No installed text cites
`skill@knowledge-architect-design@discussion-language`." "The design skill contains 74 em dashes;
the primer contains 0", counted with `grep -o` over the shipped sources, `%%` lines excluded. The
root's "Always" list holds "for every text a session writes"; the design skill excludes end-user
text.

### The owner's account meets the real-use rule `##a33`

Reply to round 2, the agent. Thread: #verification-in-primer. "Your account meets the real-use
rule; it does not overrule it." Behaviour in the owner's other projects is "a fact only you hold".
The mechanism: "An agent assumes that the project and its tools behave as their names, their
documents and its own expectations suggest, and acts on that assumption without checking it."

### The shape of `verify-before-relying` `##a34`

Reply to round 2, the agent. Thread: #verification-in-primer. A new section after
`primer@intent-and-claims`; bullets 3 and 4 move into it; the design skill's steps 4 and 5 point to
it and step 1 re-reads it, after `design@agent-skills@primer-reread-before-recording`; the 7 agents
keep "a brief is a lead"; the root keeps the thaum figure, the two listing lines and the
ground-first pointer. "Roughly 50 lines."

### A pointer hub has nothing to point to `##a35`

Reply to round 2, the agent. Thread: #verification-in-primer, against the primer as a pointer hub
with each skill keeping its narrower rule. It "loses on the five directives that have no home
anywhere: a pointer hub has nothing to point to for them."

### The merged draft and the end-user scope `##a36`

Reply to round 2, the agent. Thread: #working-prose-in-primer. The draft "Readers might be
non-native speakers: where a simpler word or a shorter sentence says the same thing, use it." The
default scope is the design skill's: "end-user text can have literary value as a deliberate goal";
the root adds one sentence binding end-user text in this repository.

### The em dashes of existing text are not swept `##a37`

Reply to round 2, the agent. Thread: #working-prose-in-primer. "The 74 em dashes in the design
skill are not swept", per `design@agent-skills@restatement-size-test`.

### The owner's framing belongs in `primer-limit` `##a38`

Reply to round 2, the agent. Thread: #primer-extends-every-root. It "belongs in that head, not in
a new one": the two answer one question and lose to one rival. The title: "The primer is the part
of every adopting project's root CLAUDE.md that is the same in every project: what every session
needs and no installed skill delivers."

### The framing's test, and its one case today `##a39`

Reply to round 2, the agent, read. Threads: #primer-extends-every-root, #checker-cli-in-primer. A
statement the setup skill would ask every root to write alike belongs in the primer. "Today exactly
one such case exists, the staged-check recommendation (`skill@knowledge-architect-setup@setup-gates`,
line 187)." The limit: it "does not admit project-specific content into the primer."

### The heads the new sections earn `##a40`

Reply to round 2, the agent. Thread: #primer-heads-per-directive. `checker-model-in-primer`
(agent-skills, test 1), `verification-in-primer` (agent-skills, test 4), `working-prose-in-primer`
(agent-skills, test 2), `anchors-listing-command` (core, test 1).

### The owner closes every thread `##a41`

Round 3, the owner. Threads: all. "Agreed on all threads here, you can proceed"

## New names, in one place

```
klarch anchors                     the command; `cargo klarch anchors` here
Command::Anchors                   path@core@src/cli/mod.rs, a variant of the existing enum
Anchor, Home, Constructed          path@core@src/entity.rs, existing
## The project's documents and how they are checked {{slug:checker-model}}
                                   the primer source, a new level-two section
## Verify before relying {{slug:verify-before-relying}}
                                   the primer source, a new level-two section
## Working prose {{slug:working-prose}}
                                   the primer source, a new level-two section
checker-model-in-primer            a head of register@agent-skills@design
verification-in-primer             a head of register@agent-skills@design
working-prose-in-primer            a head of register@agent-skills@design
anchors-listing-command            a head of register@core@design
```

The three section slugs are written as the build's slug placeholder in the source; installed,
each becomes a backticked `##` slug.

## Decided design

### The section `checker-model`

A new level-two section of the primer, placed before `primer@where-knowledge-goes`, of about 50
lines. It holds what a session needs to read the project's records and cite them, and the
checker's general CLI facts. It holds no rarer rule. Its parts, in order:

1. **The manifest**, knowledge-architect.toml at the project's root: it declares the Components,
   the locations and the registers. `{{command}} anchors` lists what it resolves to.
2. **The anchors**: a Component, which carries the required documents and every component
   register; a location, which carries only the registers it declares, their homes directly under
   its path; and the anchors the checker constructs: `plans` at docs/plans/, one per milestone
   directory, one per spec file. The anchor words `*` and `elsewhere` are reserved.
3. **What every Component carries**: `path@*@README.md`, `path@*@CLAUDE.md`,
   `path@*@docs/rejected-alternatives.md`, and the home of each component register:
   `register@*@goal`, `register@*@design`, `register@*@tripwire`, `register@*@issue`.
4. **The two register shapes**. A heading register's home is one file, or a directory behind a
   `README.md` that links every subdocument; an entry is a heading at the register's level ending
   with its slug: level 3 for `design`, level 2 for `goal` and `tripwire`. A file register's home is
   a directory with one file per entry, a hand-written `README.md` and a generated `index.md`; an
   entry's id is its file's basename. A plan's items are level-three headings under the section of
   their kind.
5. **A reference**: one backticked span `<kind>@<anchor>@<id>`. Each kind and what it names: a
   register's name names one entry of that register; `register` names a register's home;
   `path` names a file or a directory under the anchor; `planned` names a path a plan's work will
   create. The harness kinds take no anchor: `skill@<name>`, `agent@<name>`, `primer@<slug>`,
   `instructions@<slug>`, each with an optional section slug.
6. **Reading**: `{{command}} show <reference>` prints the entry whole and every reference to it;
   `{{command}} issues` and `{{command}} tripwires` list the standing entries.
7. **Running the checker**: `{{command}} check` reads every document in four phases and stops at
   the first phase that finds anything; its exit code is the verdict, and its last line states it,
   `PASSED: no findings` or `FAILED: n findings above`. `{{command}} check --staged` judges the
   tree a commit will record, and is run after staging and before each commit; a failing staged
   check is repaired before the commit. Never chain a command on a verdict that went through a
   pipe: the shell takes the exit status of the pipe's last command. A finding names its repair.

The CLI facts are those `path@core@README.md` states as contracts, the exit codes and the verdict
line, per `issue@knowledge-architect@command-output-is-not-declared-a-contract`.

The argument: #a3, #a5, #a8, #a9. The nearest rival, rival B, the binary printing the model, loses
on delivery, #a10, and is a losing alternative. The rival "keep the current state" loses on #a9.

### The command `anchors`

`klarch anchors` prints every anchor the manifest resolves to, the constructed ones included, with
its kind (component, location, plans, milestone, spec), its path, and for each register it carries
the home as the tree holds it: the file, the directory, or a mark that it is missing. The output is
for a reader. It is not a contract, per
`issue@knowledge-architect@command-output-is-not-declared-a-contract`. It exits 0 on every run
that could start, and 2 where the project cannot be read, as `model` does under
`design@core@exit-code-ladder`. It is a command of the core's CLI module, so a binary that
registers extensions runs it unchanged, per `design@core@the-core-cli-is-a-library-module`.
`path@core@README.md` lists it with the other commands, and its exit-code table gains its row.

An illustration of a shape, not authority:

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

The argument: #a11 and #a20. The nearest rival, the hand-written list in each root CLAUDE.md,
goes stale with no check, #a11.

### The section `verify-before-relying`

A new level-two section of the primer, placed right after `primer@intent-and-claims`, of about 50
lines. Bullets 3 and 4 of `primer@intent-and-claims` move into it; bullets 1 and 2 stay. Outline:

- the lead: a document can describe the code as committed rather than as it is; confidence is not
  correlated with correctness (V13);
- before relying on how anything behaves, an API, a tool, a file format, an issue's diagnosis:
  names, doc comments and prose are leads, never evidence (V1, the old bullet 3); follow the whole
  path; one case is no convention; report the evidence with the conclusion and label an
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
- a level-three subsection, "Precedent is not authority" (V12).

The design skill then changes:

- loop step 1 gains one sentence asking the session to read the primer's `verify-before-relying` again,
  whole, before grounding;
- step 4's provenance sentence keeps what is the discussion's own, a claim doing closing work and
  the tripwire or acceptance criterion an assumption becomes, and points to the primer for the
  rest;
- step 5 keeps the pre-commitment and the tie, and points to the primer for building evidence.

`skill@knowledge-architect-planning@spec-contents`' "exactly as in a design head" points to the
primer's section instead. The 7 agents' "a brief is a lead" sentence stays: one sentence, the act
each brief needs. The root CLAUDE.md keeps, under its slug, the thaum figure sentence, the two
`ls docs/...-reports/` lines and the ground-first pointer to `skill@klarch-development`.

The argument: #a21, #a22, #a24, #a30, #a33. The nearest rival, the primer as a pointer hub, loses
on #a35.

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
`path@agent-skills@README.md`, which attributes the language rule to the design skill, names the
primer. The root CLAUDE.md's "Always" list is deleted, and one sentence takes its place: "Here the
primer's `working-prose` rules also bind text end users read." written with its reference.

The argument: #a16, #a26, #a27, #a36. The nearest rival, the rules left in the design skill, loses
on #a16.

### The rewrite of `primer-limit`

`design@agent-skills@primer-limit` takes the title "The primer is the part of every adopting
project's root CLAUDE.md that is the same in every project: what every session needs and no
installed skill delivers". Its body states the test that follows from it: a statement the setup
skill would ask every root CLAUDE.md to write alike belongs in the primer; and the limit: what one
project's root alone would say does not. It attributes the framing to the owner, #a25.
`design@agent-skills@primer-content` keeps the list of the primer's members, the three new
sections and the CLI facts among them. `design@agent-skills@staged-check-before-each-commit` is
rewritten: the primer states the staged check, and the setup skill's recommendation in
`skill@knowledge-architect-setup@setup-gates` is removed.

## Mapping tables

The work needs none.

## Losing alternatives

- **The binary prints the model, and the primer points to the command** (rival B), lost to
  `thread@primer-common-root-part@checker-model-in-primer`: a command reaches a session only if it
  runs it, while the harness loads the primer into every session, #a10.
- **Keep the current state**: the setup skill, the findings and `show`, lost to
  `thread@primer-common-root-part@checker-model-in-primer`: reading comes before any finding, #a9.
- **Ship the rarer rules too**, lost to `thread@primer-common-root-part@checker-model-in-primer`:
  F23, F69 and F121, and the owner's round 2, #a19.
- **A hand-written list of anchors in each root**, lost to
  `thread@primer-common-root-part@anchors-listing-command`, #a11.
- **The primer as a pointer hub, each skill keeping its narrower rule**, lost to
  `thread@primer-common-root-part@verification-in-primer`, #a35.
- **Search the histories for real-use evidence**, option (a) of the reply to round 1, superseded by
  the owner's account, #a21, which the agent judged meets the rule, #a33.
- **The language rules left in the design skill**, lost to
  `thread@primer-common-root-part@working-prose-in-primer`, #a16.
- **The em-dash bullet kept in the root only**, the agent's default of the reply to round 1, lost
  to the owner's lean in round 2, #a26.
- **Keep the plans subsection as a pointer**, lost to
  `thread@primer-common-root-part@root-plans-subsection`, #a15.
- **A head of its own for the owner's framing**, lost to
  `thread@primer-common-root-part@primer-extends-every-root`: one question, one rival, #a38.

## Readings

The work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| The primer grows from 454 to about 600 lines, and in long sessions the new sections are not applied. | #verification-in-primer, #primer-extends-every-root | T1, a tripwire, on the owner's word |
| The model section drifts from the checker: a kind, an anchor word or a register shape changes in the core, and the primer is not updated. | #checker-model-in-primer | AC1, `acceptance@primer-common-root-part@model-lists-every-kind` |
| Adopting projects keep their own root copies of the moved text, which drift from the primer. | #primer-extends-every-root | AC2, `acceptance@primer-common-root-part@migration-names-the-sections` |
| The model section gains rarer rules over time, one per confusion, against F23. | #checker-model-in-primer | T2, a tripwire, on the owner's word |

The tripwires as put to the owner:

- **T1.** Fires when a retrospective, in any project, reports a session that acted against a
  directive of `verify-before-relying` or `working-prose` that its primer stated at the time.
  Response: add re-read instructions at the moments of use, as
  `design@agent-skills@primer-reread-before-recording` did. Re-entry: that retrospective's intake.
- **T2.** Fires when a review or an audit finds a rule in `checker-model` that a finding message
  already delivers with its repair. Response: remove it, and reopen F23's scope with the owner.
  Re-entry: that review.

## Acceptance criteria

### AC1: a core test holds every reference kind and reserved anchor word to appear in the primer's model section `##model-lists-every-kind`

Awaiting the owner, D2. Guards `thread@primer-common-root-part@checker-model-in-primer`. Judged at
step 2. Observable: a test in the core's suite asserts that every compiled-in kind, the register
names and the harness kinds of `path@core@src/entity.rs`, and the reserved anchor words `*`,
`elsewhere` and `plans`, appear in the shipped primer's `checker-model` section. Fires when the test
cannot be written against a fixed list, because the kinds are not enumerable. Response: reopen the
thread on its delivery, against rival B.

### AC2: the changelog's Migration entry names each root section an adopting project can delete `##migration-names-the-sections`

Awaiting the owner, D2. Guards `thread@primer-common-root-part@primer-extends-every-root`. Judged at
step 3. Observable: the `Next release` section of CHANGELOG.md holds a Migration entry naming the
subjects an adopting project's root CLAUDE.md can now delete: the checker's model, the verification
rules, the working-prose rules, the checker's CLI facts and the staged check. Fires when the entry
cannot name them as subjects, because the primer's sections were rewritten past recognition.
Response: a pass of the agentic-workflow axis on each adopting project at its pin move.

## Implementation sequence

1. **The command `anchors`.** `Command::Anchors` in `path@core@src/cli/mod.rs`, its output from
   the resolved anchors, its row in `path@core@README.md`'s command list and exit-code table, and
   its tests over a mock project: every anchor kind printed, and a missing home marked. Fails alone
   on: the command's test.
2. **The shipped text.** The primer's three sections and the two moved bullets; the design
   skill's steps 1, 4 and 5, its `discussion-language` section and its Overview sentence; the setup
   skill's `setup-gates` recommendation removed; the planning skill's pointer;
   `path@agent-skills@README.md`; AC1's test; `cargo klarch install-agent-skills` for the installed
   copies, in the same commit as the sources. Fails alone on: AC1's test and the shipped-set tests
   of `path@core@src/agents.rs`.
3. **This repository's own text.** The root CLAUDE.md's five sections shortened or deleted as
   "Decided design" says; the re-pointed texts of "What is already decided"; the CHANGELOG.md
   entries of `Next release`, AC2's Migration entry among them, then `cargo x changelog`. Fails
   alone on: `cargo klarch check` over the re-pointed references, and AC2.
4. **The harvest**, as the next section says, then the report on AC1 and AC2, and the spec's
   deletion.

## Order rationale

- Step 1 before step 2: the primer names `{{command}} anchors`, which must exist when the text
  ships.
- Step 2 before step 3: the root's pointers name primer sections that step 2 creates.
- Step 3 before step 4: the heads describe the delivery as built, the root's included.

## Defaults awaiting the owner

- **D1**: one spec on one branch, in the steps above, rather than a milestone of two slices. Bears
  on every thread. The ground: no design session is needed between the command and the text.
- **D2**: the premortem's verdicts as defaults: T1 and T2 recorded as tripwires, AC1 and AC2
  applied.

## Harvest

| what | where | when |
| --- | --- | --- |
| #checker-model-in-primer, a head | `register@agent-skills@design`, under "The configuration a project holds" | step 4 |
| #verification-in-primer, a head, its evidence attributed to the owner | `register@agent-skills@design` | step 4 |
| #working-prose-in-primer, a head | `register@agent-skills@design` | step 4 |
| #primer-extends-every-root → `primer-limit`, rewritten | `register@agent-skills@design` | step 4 |
| `primer-content` keeps its list; each of its bundled decisions with a rival of its own takes a head, per #primer-heads-per-directive | `register@agent-skills@design` | step 4 |
| #checker-cli-in-primer: `staged-check-before-each-commit` rewritten | `register@agent-skills@design` | step 4 |
| #anchors-listing-command, a head | `register@core@design` | step 4 |
| rival B, and the losing alternatives the recording tests keep | `path@agent-skills@docs/rejected-alternatives.md` | step 4 |
| T1 and T2, on the owner's word | `register@agent-skills@tripwire` | step 4 |
| `issue@agent-skills@primer-content-bundles-each-directive-s-decision` closes | its file is deleted | step 4 |
| `path@agent-skills@CLAUDE.md`'s restatement of `primer-limit` | rewritten | step 4 |
| #root-plans-subsection | no head: a member of `design@agent-skills@knowledge-table-home` | — |

## Later consequences

- thaum deletes its restatements at its next pin move, guided by AC2's Migration entry.
- `issue@agent-skills@the-message-check-after-a-commit-is-delivered-by-no-installed-text` and
  `issue@agent-skills@a-pending-audit-report-is-reached-by-no-installed-search`, when real use
  shows their need, add a line to `verify-before-relying` or to `checker-model` instead of to the
  setup skill.
- A future reference kind or anchor word fails AC1's test until the primer names it.
