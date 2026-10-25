# Standards read whole at several places are delivered as fragments, the rules on design heads are split by the act that applies them, and a text's size cost is counted in the context of the sessions that load it

## Status and audience

This spec is the plan of one branch and one pull request, on the owner's word, round 5: "This work
should be a single spec and PR, done at once". The work does four things:

- it migrates ten blocks of the installed text, Q1 to Q11 but Q4, now delivered by a pointer read
  whole, to fragments the build copies into each text that applies them, per
  `design@agent-skills@shared-text-fragment`;
- it splits the primer's section on design heads, `primer@design-heads`, by the act that applies
  each part: what every session applies stays in the primer, and the rules only a writer or a judge
  of a head applies leave it, each part a fragment hosted where its act happens;
- it adds a rule on how a size cost of agent-facing text is weighed, as a fragment hosted in the
  primer and beside the admission standard's shape of a repair;
- it rewords one sentence of the rules on heads, closing
  `the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head`.

The spec is written for a session that did not witness the discussion. It leaves the repository in
a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point the section "What is already decided" does
  not list as reversed or rewritten, the design home wins, and the disagreement is a defect of this
  spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. No default awaits the owner at the writing of this spec.
- It was assembled from one transcript, this session's. The discussion begins at the owner's
  message that opens "the last branch introduced "instruction fragments", meant to improve delivery
  of instructions or standards that are large and form one coherent whole." and ends at the owner's
  message "agreed on ac 1 and 2, proceed." It holds 5 owner messages, called rounds 1 to 5 below.
  Find the file by that opening message, not by its name. The reviews of this spec raised three
  defaults the owner rules on, D1 to D3, and step 4 a fourth, D4, under "Defaults awaiting the
  owner".
- The work starts in the session where the discussion converged, on the branch that holds this
  spec, so it takes no design audit, per `skill@knowledge-architect-planning@working-a-slice`. A
  session other than that one takes the design audit of its point 2 first, with the decisions of
  "What is already decided" as its seeds.
- Between step 1 and step 6, the design homes describe the delivery the earlier steps replace; step
  6 brings them to the tree, before the review before the merge.

## How the work is done

`skill@knowledge-architect-planning@working-a-slice`, its paragraph on the work of a spec.

## Names

- **fragment**: a file of `path@agent-skills@fragments/`, which the build copies whole, indented to
  its placeholder line `{{fragment:<file>}}`, into each content file that names it, per
  `design@agent-skills@shared-text-fragment`. The pass is `render_fragments` in
  `path@agent-skills@src/render.rs`.
- **host**: a content file that holds a fragment's placeholder line. A text is a host only where its
  act applies the whole fragment.
- **the hosts test**: `each_fragment_has_its_hosts` in `path@agent-skills@src/lib.rs`, which lists
  each fragment's installed hosts and fails when the shipped set differs.
- **the installed copy**: the file `cargo klarch install-agent-skills` writes under .claude from a
  content file. A review of a fragment reads every host's installed copy, per
  `path@agent-skills@CLAUDE.md`.
- **Q1 to Q12**: the labels of the candidate table of round 1's reply, each a block of the
  installed text pointed to with an instruction to read it whole, listed under "What the work is".
- **the admission standard**: `skill@knowledge-architect-agent-configuration@admission-standard`.
  Its five level-three subsections: "The need test", "Evidence and kind", "The shape of a repair",
  "The record check", "The outcomes of a need-test failure".
- **the three parts of the rules on heads**: the three level-three subsections of
  `primer@design-heads` at this spec's commit: "What earns a head", "What a head holds", "How a head
  is shaped".
- **shape C**: the split of the rules on heads that `thread@standards-delivered-as-fragments@head-rules-delivery`
  approved.
- **the section on what a head holds and how it is shaped**: `skill@knowledge-architect-decision-recording@head-shape`, titled "What a head holds and how it is shaped",
  which step 1 creates.
- **a fragment file** is named by its file name, as `head-shape.md`; its path is under "New names,
  in one place".
- **AC1, AC2**: the acceptance criteria the owner ruled applied, round 5.
- **a word count** of a section is `wc -w` over its content file, from its heading line to the next
  heading of the same or a higher level, the heading included.

## What the work is

### What exists today

Each site below was read at this spec's parent commit, in `path@agent-skills@content/`, and the
spec's commit changes no file there. Line numbers name that tree.

**The fragments directory** holds two fragments, `path@agent-skills@fragments/tripwire-standard.md`
and `path@agent-skills@fragments/tripwire-lifecycle.md`. The hosts test lists their hosts.

**Q1, the admission standard** (`path@agent-skills@content/skills/agent-configuration/SKILL.md`, the section at line 19,
1565 words by `wc -w`). Pointers read whole, outside its file:

- `path@agent-skills@content/agents/agent-text-reviewer.md`, line 14: "Read that section whole, from the installed skill's file in
  the tree you review, before judging anything";
- `path@agent-skills@content/agents/workflow-auditor.md`, lines 76-78 (the draft kind "an unneeded statement", the need test), line 126
  (lens L9, the need test), line 200 (the check of a proposal, "against the record, as … says");
- `path@agent-skills@content/skills/project-audit/SKILL.md`, line 265 ("The repair by cause, as … says, read whole"), line 270
  (the record check, made by the workflow auditor), line 296 (net growth) and line 304 (the outcome
  row);
- `path@agent-skills@content/skills/retrospective/SKILL.md`, line 111 (its proposals "shaped as … says, read whole");
- `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, lines 105-106 (the lapse rule), line 134 (the fix "shaped, and read
  against the record, as … says, read whole").

**Q2, `skill@knowledge-architect-retrospective@two-files`** (374 words): pointed to from
`path@agent-skills@content/skills/retrospective-intake/SKILL.md`, line 30, "read it whole before reading a file".

**Q3, the paragraph of `skill@knowledge-architect-retrospective@what-it-examines`** that defines
unclear, missing and wrong (`path@agent-skills@content/skills/retrospective/SKILL.md`, the paragraph at lines 57-64, whose
other sentences are the retrospective's own acts): pointed
to from `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, line 104, "defines the three, read whole".

**Q4, `skill@knowledge-architect-retrospective@expectation-sets`**: kept as a pointer, per
`thread@standards-delivered-as-fragments@fragment-migrations`.

**Q5, `skill@knowledge-architect-decision-recording@losing-alternatives`** (948 words): pointed to
from `path@agent-skills@content/agents/design-record-auditor.md`, line 44 (its list "Before your first entry, read whole") and
line 88; restated in words in `path@agent-skills@content/agents/decision-record-reviewer.md`, lines 87-98 ("Judge it by the skill's
tests for a rejected design"); named as the rule of the axis in `path@agent-skills@content/skills/project-audit/SKILL.md`, line 151.

**Q6, `skill@knowledge-architect-decision-recording@owning-component`** (169 words): pointed to from
`path@agent-skills@content/agents/routing-reviewer.md`, line 92 ("read whole") and `path@agent-skills@content/agents/design-record-auditor.md`, lines 42 and 84;
named as the rule of the axis in `path@agent-skills@content/skills/project-audit/SKILL.md`, line 150.

**Q7, `skill@knowledge-architect-decision-recording@three-homes`** (293 words): pointed to from
`path@agent-skills@content/agents/design-record-auditor.md`, line 45.

**Q8, `skill@knowledge-architect-decision-recording@reversal-check`** (554 words): pointed to from
`path@agent-skills@content/agents/decision-record-reviewer.md`, lines 100-102 ("judge it against every point of … read whole").

**Q9, `skill@knowledge-architect-project-audit@finding-outcomes`** (206 words): pointed to from
`path@agent-skills@content/agents/design-record-auditor.md`, lines 47 and 106, `path@agent-skills@content/agents/tripwire-auditor.md`, lines 45 and 139,
`path@agent-skills@content/agents/workflow-auditor.md`, line 138.

**Q10, the paragraph "A skill is self-sufficient" of
`skill@knowledge-architect-agent-configuration@shaping-a-skill`** (the per-pointer test): pointed to
from `path@agent-skills@content/agents/routing-reviewer.md`, lines 87-89 and `path@agent-skills@content/agents/workflow-auditor.md`, lines 73-75.

**Q11, `skill@knowledge-architect-issue-tracking@trigger-test`** (225 words): pointed to from
`path@agent-skills@content/agents/standing-state-reviewer.md`, lines 117-118, "Read it whole", and line 87.

**Q12, `primer@design-heads`** (`path@agent-skills@content/PRIMER.md`, lines 417-616, 2608 words; its parts: "What earns a head"
907, "What a head holds" 699, "How a head is shaped" 949, measured with `wc -w` over each
subsection). Its paragraphs by opening words, with their lines:

| line | paragraph | part |
| --- | --- | --- |
| 419 | the section's introduction | none |
| 426 | "Most implementation choices earn no head." to the end of the four entry tests | What earns a head |
| 460 | "A choice that turns on what the specification means" | What earns a head |
| 464 | "Otherwise it belongs in an **inline comment**" | What earns a head |
| 469 | "**The backstop.**" | What earns a head |
| 482 | "**A new member of a head is routed by the head's ground.**" | What earns a head |
| 494 | "**The standing argument**" | What a head holds |
| 503 | "**A head stands on its argument, and cites the owner only for what came from the owner.**" | What a head holds |
| 514 | "**A durable record attributes a decision to the owner; it does not quote the owner.**" | What a head holds |
| 521 | "**What the ground means to a later session.**" | What a head holds |
| 528 | "**The deliberation is not copied into the head.**" | What a head holds |
| 531 | "A head is written **as if the design had always been so**." | What a head holds |
| 535 | "**A head carries intent, shape and the standing argument, not implementation.**" | What a head holds |
| 540 | "Never list what cites a head" | What a head holds |
| 543 | "A decision that relies on the checker of this workflow" | What a head holds |
| 549-616 | every paragraph of "How a head is shaped" | How a head is shaped |

The instructions to read it again, whole:

- `path@agent-skills@content/skills/decision-recording/SKILL.md`, line 8, the skill's first line: "Read `primer@design-heads`
  again, whole, before writing or judging a head."; and `:145`, "What a head holds and how it is
  shaped is `primer@design-heads`: read it whole before writing a head";
- `path@agent-skills@content/skills/design/SKILL.md`, line 141, the in-change path: "Read `primer@design-heads` again, whole, before
  grounding";
- `path@agent-skills@content/skills/planning/SKILL.md`, line 454, the harvest: "Read `primer@design-heads` again, whole, before
  writing a head";
- `path@agent-skills@content/agents/decision-record-reviewer.md`, lines 15-18: "The standard is the
  primer's section `primer@design-heads` … Read both in full before anything else"; and lines 68-69:
  "Judge each against the whole section";
- `path@agent-skills@content/agents/design-record-auditor.md`, line 41: in its list read whole before the first entry;
- `path@agent-skills@content/agents/routing-reviewer.md`, line 106: "Judge it by `primer@design-heads`, read whole", for present
  tense alone;
- `path@agent-skills@content/agents/tripwire-auditor.md`, line 43: "its part "What a head holds"", for attribution alone.

Two citations in words of a rule that leaves the primer, which a search for the reference does not
return: `path@agent-skills@content/skills/decision-recording/SKILL.md`, line 151, the template's
"Then the standing argument, per the primer's section on design heads"; and
`register@core@design`, line 6, "`primer@design-heads` owns the shape". Found with
`git grep -n "section on design heads\|owns the shape"`.

The positional phrases in the blocks to be cut, found with
`git grep -n -E "below|above|next section|this section" -- crates/agent-skills/content` and read:
`path@agent-skills@content/PRIMER.md`, lines 457 ("as the next section says"), 475 ("by the test of
one decision per head below") and 563 ("by the alignment rule below");
`path@agent-skills@content/skills/agent-configuration/SKILL.md`, line 32 ("the table below");
`path@agent-skills@content/skills/decision-recording/SKILL.md`, lines 42, 171, 175 and 176; and
`path@agent-skills@content/skills/retrospective/SKILL.md`, line 59.

`git grep -n 'primer@design-heads' -- crates/agent-skills/content | wc -l` gives 38 lines in 13
files at this spec's parent commit. Outside content/, .claude and the plans directory, this
repository's Markdown holds 46 such lines in 19 files
(`git grep -n 'primer@design-heads' -- '*.md' ':!.claude' ':!crates/agent-skills/content' ':!docs/plans'`),
and no file of another type holds one.

**The sizes**, measured at this spec's parent commit on the installed set of this repository:
`cat .claude/knowledge-architect/PRIMER.md .claude/skills/knowledge-architect-*/SKILL.md .claude/agents/knowledge-architect-*.md | wc -w`
gives 73,830; `wc -w .claude/knowledge-architect/PRIMER.md` gives 8,412.

### What is outside the work

- **The saved workflows**, `path@agent-skills@content/workflows/`: their prompts are single-quoted
  JavaScript strings joined with `+`, where a placeholder line cannot stand. A prompt that names an
  agent type gets the fragments its definition hosts. The general-purpose agents of
  `path@agent-skills@content/workflows/agentic-workflow-audit.js`, its cluster, merge and repair
  agents (lines 188, 201, 216 and 264), get none, and their prompt's `KINDS` (lines 90-93) points
  them to the admission standard for how a repair is shaped. That pointer stays; step 6 opens a
  `todo` issue for it.
- **The search that found the candidates** read the lines holding "whole" or "again" and the
  pointers beside them. A pointer such as "as X says" that applies a whole standard, with no such
  word near it, was not found. Step 6 opens a `todo` issue for a search of those.
- **This repository's own skills and its root `CLAUDE.md`**: the fragment pass renders content/
  only. Their pointers stay.
- **The candidates the round-1 table excluded**: the pointer to
  `skill@knowledge-architect-planning@working-a-slice` written into a project's milestone document;
  `agent@knowledge-architect-transcript-reviewer@extract-transcripts`, applied by a subagent the
  brief names; the "read whole with `show`" lines, which point at a project's own entries;
  `skill@knowledge-architect-review@review-invariants` in the auditors' descriptions.
- **`issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`**:
  its rule moves with `head-ground.md` unchanged, and its question stays open for the discussion it
  asks for.
- **The shape of an issue entry**, `skill@knowledge-architect-issue-tracking@issue-entry`: the checker
  asserts it, so `path@agent-skills@content/agents/standing-state-reviewer.md`, line 110 keeps its pointer, per Q11.
- **A pointer in a text that hosts no fragment of its block**, as
  `path@agent-skills@content/skills/project-audit/SKILL.md`, lines 150-151, 270, 296 and 304, which
  name rules another actor applies: unchanged.

## What is already decided

### The decisions the work rests on

- `design@agent-skills@shared-text-fragment`: the mechanism, the rule that a host applies the whole
  fragment, and that a fragment holds no heading slug, no other fragment and no placeholder but
  `{{command}}`.
- `design@agent-skills@content-mirrors-the-install-layout`: a level-two heading under content/ ends
  with a slug placeholder, so a section's slug stays in its host, never in a fragment.
- `design@agent-skills@restatement-size-test`: a directive longer than one sentence is delivered by
  a pointer read whole or by a fragment; one sentence may be restated with its pointer.
- `design@agent-skills@shipped-text-cites-no-entry`: a fragment cites only the shipped set.
- `design@core@installed-entities-from-the-tree` and `design@core@section-homes-carry-slugs`: a
  `primer@<slug>` or `skill@<name>@<slug>` reference resolves against the installed copies; a slug
  of the installed text is an interface, whose rename owes a Migration entry.
- `design@agent-skills@audit-agent-per-axis`: an axis's standard reaches its subagent from the
  first token of the agent's definition.
- `design@agent-skills@root-cause-repair` and `design@agent-skills@admission-rules-home`, their
  centralization: one home per moment, which one fragment file keeps.
- `design@agent-skills@behaviour-additions-need-real-use`: the evidence for the moves is the
  owner's request naming what a session lacked, rounds 2 and 3, and
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`.
- `design@knowledge-architect@changelog-entries` and `design@knowledge-architect@versioning-policy`:
  a skill change is a patch; a Migration entry names what a consumer changes in its own files.

### The decisions the work reverses or rewrites

Each with every text `cargo klarch show` lists as referencing it at this spec's commit, and the step
that updates it.

| decision | what changes | referencing texts, and who updates them |
| --- | --- | --- |
| `design@agent-skills@one-home-for-head-rules` | its location part is reversed: the rules leave the primer's single section for fragments hosted by act; its centralization part stands. Title rewritten, slug kept | `register@agent-skills@design`, lines 278, 2978, 3236: step 6; the issue on a project's purpose closes in step 1 |
| `primer-reread-before-recording` | leaves: its instruction is removed | `path@agent-skills@content/skills/design/SKILL.md`, line 132, in the `%%` block of lines 129-134, which step 1 deletes with the reread it argues; `register@agent-skills@design`, lines 390, 3018: step 6 |
| `design@agent-skills@new-or-reshaped-head-needs-design` | its paragraph "Three texts deliver it" is rewritten: the backstop is delivered in the primer and, at the act, in decision-recording | `register@agent-skills@design`, lines 343, 1684; `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`, lines 20, 48; `register@agent-skills@tripwire`, lines 48, 64: step 6 reads each |
| `design@agent-skills@design-home-write-loads-recording` | its reread argument is rewritten: the skill delivers the tests itself | `register@agent-skills@design`, lines 279, 3251; `register@agent-skills@tripwire`, line 52: step 6 |
| `design@agent-skills@verification-in-primer` | its clause "as … does for the heads" is reworded | `register@agent-skills@design`, line 2971: step 6 |
| `design@agent-skills@admission-rules-home` | "each judging actor's text points to it" becomes: each judging actor hosts the subsections it applies, as fragments | `register@agent-skills@design`, lines 463, 2098, 2115, 2126, 2622, 2655, 2755, 2844, 3622; `issue@agent-skills@patching-an-installed-skill`, line 77; `register@agent-skills@tripwire`, lines 391, 402: step 6 reads each |
| `design@agent-skills@additions-review-agent` | "holds no copy of it" becomes: holds it as fragments | `register@agent-skills@tripwire`, line 408: step 6 |
| `design@agent-skills@intake-shares-the-admission-standard` | "a pointer read whole at its step that shapes a fix" becomes: the fragments at that step | `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, line 136 (a `%%` line), step 3 |
| `design@agent-skills@shared-text-fragment` | its sentence "the other such blocks migrated later" is rewritten to what is migrated; it names which host lists are the owner's, and lists no host, since the hosts test holds the lists | `path@agent-skills@CLAUDE.md`, line 52; `register@agent-skills@design`, lines 21, 3220; `a-pointer-delivers-its-directive-only-when-the-session-fetches-it`, which closes, `path@agent-skills@src/lib.rs`, line 132: steps 1 to 6 |
| `design@agent-skills@primer-limit` | its member "the rules on design heads" narrows to recognizing a decision and reading a head's ground; it gains the context-cost rule | `path@agent-skills@CLAUDE.md`, line 59; `path@agent-skills@content/PRIMER.md`, line 4 (a `%%` line); `register@agent-skills@design`, lines 1340, 3027, 3131, 3208, 3353; `register@agent-skills@rejected`, line 433; `register@core@design`, line 40; `path@core@src/cli/create.rs`, line 473: step 6 reads each |
| `design@agent-skills@design-record-axis` | "`primer@design-heads` for a head" cites both homes of the rules | `issue@agent-skills@a-head-in-the-wrong-component-goes-to-the-owner-though-the-record-settles-the-component`, line 25: step 6 |
| `rejected@agent-skills@head-rules-in-decision-recording` | stays `live`, its reason narrowed to the parts that stay in the primer | `register@agent-skills@design`, line 3211: step 6 |
| `tripwire@agent-skills@head-created-without-deliberation` | its premise names the new delivery; its cause is unchanged | `register@agent-skills@design`, line 281; `register@agent-skills@rejected`, line 133: step 6 |
| `design@agent-skills@intake-skill-home` | "The intake reads the report's format by pointers to the retrospective's sections on the two files, on what a finding examines and on the expectation sets" becomes: by the fragments of the two files and of the kinds of a defect, and by a pointer to the expectation sets | `path@agent-skills@content/skills/retrospective/SKILL.md`, line 155, and `path@agent-skills@content/skills/retrospective-intake/SKILL.md`, line 8, both `%%` lines: step 4; `register@agent-skills@design`, line 3491: step 6 |
| `design@agent-skills@behaviour-additions-need-real-use` | its exception "the reread of the primer on the in-change path" leaves, with the reread; its rule stands | the citing texts cite its rule, which stands; the `%%` block of `path@agent-skills@content/skills/design/SKILL.md`, line 134, leaves in step 1 |

The other heads that cite a part leaving the primer are re-pointed in step 6, per the mapping table
"Citations of `primer@design-heads`": `design@agent-skills@thread-slug-is-entry-id`,
`design@core@plain-word-lint`, `design@agent-skills@only-a-head-changing-repair-is-reviewed-again`,
`design@agent-skills@existing-heads-on-touch`, `design@agent-skills@created-heads-checked-in-the-run`;
and the issues `issue@core@core-heads-bundle-decisions-with-many-citing-sites`,
`issue@gates@the-one-spawn-helper-has-no-recorded-argument`,
`issue@knowledge-architect@the-released-changelog-section-rule-has-no-argument`,
`issue@core@cross-project-references`, `issue@core@a-plain-hash-item-name-is-read-by-nothing`.

### The standing entries and graded entries the grounding returned

The discussion's searches returned the entries below as bearing. Each with its outcome.

- `a-pointer-delivers-its-directive-only-when-the-session-fetches-it`: closes in
  step 6; this work judges its three candidates, Q1, Q2 and Q12, and nine more, with the limit of
  the search stated under "What is outside the work".
- `the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head`: closes
  in step 1, by the rewording of `thread@standards-delivered-as-fragments@head-cites-the-goal-it-serves`.
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`:
  evidence for the work's argument; it concerns `primer@verify-before-relying`, which the work does
  not move, and stays open.
- `issue@agent-skills@patching-an-installed-skill`: thaum's override of "entry test 2 of
  `primer@design-heads`" still resolves, since the entry tests stay in the primer. No change.
- `issue@agent-skills@a-member-recorded-directly-lies-beyond-its-head-s-argument`: the member
  routing rule moves into `head-entry-tests.md`, delivered at decision-recording too. Its question
  stays open.
- `issue@agent-skills@an-approved-default-is-attributed-to-the-owner-and-the-agent-records-no-decision-of-its-own`,
  `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`:
  their rules move unchanged into `head-ground.md`; both stay open.
- `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`:
  its text moves unchanged into `losing-alternatives.md`; it stays open, and its closing edit will
  edit the fragment.
- `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: cites the backstop,
  which stays in the primer. No change.
- `issue@agent-skills@the-design-record-auditor-s-return-names-two-verdict-vocabularies`: the
  auditor's draft names a rule "by the section of the rule", which after step 1 is a section of the
  auditor's own definition. Its question, the vocabulary of `<verdict>`, is untouched; it stays
  open.
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`: a project's root restatement of a
  shape rule would point to the primer after the move; the Migration entry of step 6 names it.
- `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: a fragment cites no entry, as
  content/ does; the reviews judge it.
- `issue@core@no-checked-form-cites-a-point-inside-a-section`: "test 4 included" and similar
  citations in words still name a test the primer holds. No change.
- `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`: each step stages
  every edited content file and its installed copy together.
- `issue@core@installed-file-findings-belong-in-phase-four`,
  `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`: bear on how the work
  is checked, not on its design. No change.
- `tripwire@agent-skills@pointer-not-followed`: its premise is the one this work stops relying on
  for the migrated blocks; it does not fire, since no recorded break meets its clause.
- `tripwire@agent-skills@head-created-without-deliberation`: reworded in step 6.
- `tripwire@agent-skills@rarer-rule-in-the-model`: its horizon counts commits that change
  `path@agent-skills@content/PRIMER.md`; this work's commits count toward it.
- `tripwire@agent-skills@agent-text-written-without-the-standard`: the context-cost rule reaches
  sessions that load no agent-configuration skill through the primer; no change.
- `tripwire@agent-skills@a-shortcut-decision-earns-a-head`, `tripwire@agent-skills@rule-title-wider-than-its-argument`,
  `tripwire@agent-skills@split-rule-from-exception`, `tripwire@agent-skills@owner-intent-stripped`,
  `tripwire@agent-skills@attribution-loses-the-scope`, and the audit-axis tripwires the search
  returned: their rules move unchanged; their decisions stand. No change.
- `rejected@agent-skills@directive-restated-everywhere`: not brought back, since a fragment's copy
  is written by the build; its doubt stays with `tripwire@agent-skills@pointer-not-followed`.
- `rejected@agent-skills@admission-standard-in-primer`: not brought back, and stays `live`. The
  context-cost rule is not the admission standard: it weighs where a text is delivered, which
  sessions that write no agent-facing text weigh too, as a mapping task and a design discussion did
  (`argument@standards-delivered-as-fragments@arg-33`). The entry's reason, that writing
  agent-facing text is not every session's work, still holds for the standard, and its doubt stays
  with `tripwire@agent-skills@agent-text-written-without-the-standard`.
- `rejected@agent-skills@one-generic-auditor-agent`: the move puts more of each standard in the
  agent's definition, away from this rival.
- `rejected@agent-skills@every-decision-through-design-skill`: unchanged.
- `design@agent-skills@workflow-axis-corpus`: the corpus grows by the fragments' copies in their
  hosts, which the workflow auditor reads as it reads every text, since each copy is a text a
  session receives. Its signal for a corpus too large, an auditor's return that it did not read a
  file whole, stands. No change.
- `design@agent-skills@design-record-axis-dispatch`: its bound of 60 entries per group rests on
  what an auditor reads per entry; the fragments add a fixed text to the auditor's definition, read
  once per auditor, not per entry. No change.
- `design@agent-skills@root-cause-repair`: its repair "the homes merged into one and the others
  pointing to it" stays true, since a fragment is one home and its hosts are no homes. No change.
- `design@agent-skills@intake-skill-home` and `design@agent-skills@behaviour-additions-need-real-use`:
  rewritten, under "The decisions the work reverses or rewrites".
- The head-rule entries filed under the subject "The shipped set" of
  `register@agent-skills@design`, met by a search: dropped, since no rule ties a design entry to a
  subject, and the design-entry searcher reads its groups whole, so a subject's placement hides no
  entry.
- `goal@agent-skills@no-contradiction-in-workflow`, `goal@agent-skills@installed-text-works-anywhere`,
  `goal@agent-skills@one-skill-per-activity`, `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`,
  `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@core@documents-stay-current`:
  the criteria derive from them.
- The heads graded as citing a part of the section, listed in the mapping table "Citations of
  `primer@design-heads`", are re-pointed in step 6 where their part leaves.

## Criteria

### No two installed instructions on heads that a session cannot both obey `##no-contradiction`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Satisfied by
`thread@standards-delivered-as-fragments@head-rules-delivery`, once
`acceptance@standards-delivered-as-fragments@host-copies-true-in-their-host` has been judged.

### Each rule has one home, with no hand-written partial copy `##one-home-per-rule`

Binding, from the centralization part of `design@agent-skills@one-home-for-head-rules`, the
owner's words. Satisfied: one fragment file per rule.

### Each rule is delivered in the text of the act that applies it `##delivered-at-the-act`

Binding, from the owner's argument of round 2. Satisfied by
`thread@standards-delivered-as-fragments@head-rules-delivery`,
`thread@standards-delivered-as-fragments@fragment-migrations` and
`thread@standards-delivered-as-fragments@context-cost-of-agent-text`.

### Every session can tell during any task whether a decision needs a head, and can read the ground of a head it meets `##recognizing-in-every-session`

Binding until the owner says otherwise, from the owner's arguments recorded in
`design@agent-skills@one-home-for-head-rules`. Satisfied: `head-entry-tests.md` and
`head-ground.md` stay hosted in the primer.

### A fragment host applies the whole fragment `##host-applies-whole`

Binding, from `design@agent-skills@shared-text-fragment`. Satisfied: the texts that apply one rule
of a fragment get one restated sentence with its pointer, not a host.

### `primer@design-heads` keeps resolving `##design-heads-slug-resolves`

Weighed, from the released 0.6.0 section of CHANGELOG.md, thaum's root instructions and
`design@knowledge-architect@capability-cost-before-migration`. Satisfied: the slug stays; the
citations of a part that leaves are re-pointed, per
`acceptance@standards-delivered-as-fragments@citations-name-what-the-section-holds`.

## Threads

### Ten blocks read whole at several places become fragments; the expectation sets stay a pointer `##fragment-migrations`

Proposed by the agent, round 1's reply, as the candidate table Q1 to Q12. Final state: approved,
for Q1 to Q11, Q4 as a pointer. The host lists the table showed are the owner's; the ones this spec
narrows are D2. The arguments: `argument@standards-delivered-as-fragments@arg-1` to
`argument@standards-delivered-as-fragments@arg-5`. Its shape: "Decided design", "The migrated
blocks". Harvest: `design@agent-skills@shared-text-fragment`, its migrated blocks, and the hosts
test. The owner's words, round 2: "I agree with your recommendation table overall, including your
leans. The only exception is the primer, in particular design heads." The leans were Q7, Q8 and
the trigger test of Q11. Q5's condition, that the whole section is rules for writing and judging
an entry, holds: `argument@standards-delivered-as-fragments@arg-41`. Q12 left this thread for
`thread@standards-delivered-as-fragments@head-rules-delivery`.

### The rules on design heads are split by the act that applies them, each part a fragment hosted where its act happens `##head-rules-delivery`

Proposed by the owner, round 2, against the agent's Q12 of round 1; shape C proposed by the agent,
round 2's reply. Final state: approved. The arguments:
`argument@standards-delivered-as-fragments@arg-6` to `argument@standards-delivered-as-fragments@arg-27`.
Its shape: "Decided design", "The rules on heads". Harvest: `design@agent-skills@one-home-for-head-rules`,
rewritten; whether it attributes the split to the owner's intent is D3; where "The standing
argument" goes is D1. The owner's words, round 3: "head-rules-delivery: I agree with your C split, I was going
to propose the same split and distribution." The three host defaults of round 3's reply, the
routing reviewer, the tripwire auditor and the planning harvest, stood presumed-settled and closed
with the displayed checkpoint table of round 4's reply, on the owner's word of round 5, "agreed on ac
1 and 2, proceed.", read as the batch confirmation of that table, per
`skill@knowledge-architect-design@threads-and-states`.

### The size cost of agent-facing text is counted in the context of the sessions that load it `##context-cost-of-agent-text`

Proposed by the owner, round 3; its wording proposed by the agent, round 3's reply. Final state:
approved. The arguments: `argument@standards-delivered-as-fragments@arg-28` to
`argument@standards-delivered-as-fragments@arg-36`. Its shape: "Decided design", "The context-cost
rule". Harvest: a new head of `register@agent-skills@design`, under the thread's slug. The owner's words,
round 3: "I would like to define a clear instruction about how to weight costs related to size of
agent text. It is about the size of the context of a session using the text, not the size of the
text itself, and the argument on delivery I gave in my last message should bear on it too. A
session that needs to resolve a reference, search for location, then reach it, is always wasting
context (and time too, btw) compared to one that receives the instruction directly." Round 4:
"context-cost-of-agent-text: agreed on your wording and the two locations".

### A head cites the goal its arrangement serves, and a project's purpose is its goals home's `##head-cites-the-goal-it-serves`

Proposed by the agent, round 4's reply, with a default. Final state: approved, by the owner's word
of round 5, "proceed.", given against the displayed checkpoint table that held it as new with its
default, read as the batch confirmation of that table. The arguments:
`argument@standards-delivered-as-fragments@arg-37` and `argument@standards-delivered-as-fragments@arg-38`.
Its shape: "Decided design", "The rules on heads", the rewording. Harvest: none beyond the fragment's
text; the issue on a project's purpose closes in step 1.

## Arguments

### A text gets a fragment only where its act applies the whole section `##arg-1`

Round 1's reply, the agent; bears on `thread@standards-delivered-as-fragments@fragment-migrations`.
"a text gets the fragment only if it applies the whole section", the test of
`design@agent-skills@shared-text-fragment`.

### The admission standard's hosts apply different subsections `##arg-2`

Round 1's reply, the agent; bears on Q1. "Only the agent-text reviewer applies all five
subsections. The others each use one or two: the need test, the lapse rule, the shape of a repair,
or the record check."

### The intake runs in a session that never loaded the retrospective skill `##arg-3`

Round 1's reply, the agent; bears on Q2. "The intake runs in a session that never loaded the
retrospective skill."

### The intake needs one skill's expectation set, which varies per finding `##arg-4`

Round 1's reply, the agent; bears on Q4, kept as a pointer. "The intake needs only the set of the
skill a finding is about, and that changes with each finding."

### The decision-record reviewer restates the rejected-design rules in its own words `##arg-5`

Round 1's reply, the agent; bears on Q5. "The reviewer currently restates it in its own words ("the
skill's tests for a rejected design"), which can drift from the skill." For Q11's issue entry: "no,
because the checker already enforces the shape."

### The primer is in every context, so a fragment would load it twice (withdrawn) `##arg-6`

Round 1's reply, the agent; bears on Q12. "Every session and every subagent already has the primer
in its context … a fragment would load up to 2600 words a second time in each host." Withdrawn in
round 3's reply, on `argument@standards-delivered-as-fragments@arg-29`.

### Fragments get longer in each host (withdrawn) `##arg-7`

Round 1's reply, the agent. "Fragments get longer. Q1 and Q5 copy 1000–1500 words into each host."
Withdrawn in round 3's reply, on the same ground.

### The primer is loaded once, and delivery belongs right before the act `##arg-8`

Round 2, the owner; bears on `thread@standards-delivered-as-fragments@head-rules-delivery`. "context
can get large, and the primer is loaded only at the very begining of a session. It can get
significantly diluted, memory is far from enough." and "The point though, is that instructions are
delivered right before the action they shape, as often as possible."

### The installed text is small next to what an autonomous session needs `##arg-9`

Round 2, the owner. "all of those texts are only a very small amount of context compared to what
working in a project autonomously needs (to write code, tests and documentation). TBH, I believe
loading the full agentic workflow at once would not put much strain on context size compared to a
typical session lenght."

### The primer placement predates fragments, and some parts are not needed everywhere `##arg-10`

Round 2, the owner. "it was chosen as the only location carrying the "design heads" section
because we did not have the fragment idea back then, but I believe some subsections of it might
not be required everywhere/all of the time."

### The decision-recording skill asks for the reread `##arg-11`

Round 2's reply, the agent, answering the owner's question of round 2, "I believe the decision
recording skill asks to reread it, doesn't it ?". Its first line: "Read `primer@design-heads` again,
whole, before writing or judging a head. The primer was loaded when the session started, and a
recording comes late in it."; the design skill's in-change path and the planning harvest say the
same; `primer-reread-before-recording` records it.

### The decision reversed has two parts, and only its location part changes `##arg-12`

Round 2's reply, the agent. "Centralization: one home, no partial restatements. A fragment keeps
this, since it is one file and no copy is written by hand." and the location part rests on the
owner's three arguments, "nearly baseline", interpreting recorded design, telling in advance what
may enter a design home.

### Real use shows a primer rule read at the start lapses at its act `##arg-13`

Round 2's reply, the agent, from
`issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`:
"11 of the 12 workflow files from the 2026-10-24 intake, across three projects, report a primer
rule acted on as if absent." Its limit: "That issue concerns `primer@verify-before-relying`, not
the head rules. I found no recorded case of the head rules being skipped at the moment of writing."

### The owner's request is the evidence the real-use rule asks `##arg-14`

Round 2's reply, the agent. `design@agent-skills@behaviour-additions-need-real-use` accepts "asked
for by the owner mid-session, with the owner naming what the session would have lacked", and the
owner named it: the rules at the moment of writing or judging.

### The installed workflow measures 73,830 words `##arg-15`

Round 2's reply, the agent, measured with the commands of "What the work is": the installed set
73,830 words and the primer 8,412, over the installed copies; the section 2,608, over its content
file ("What earns a head" 907, "What a head holds"
699, "How a head is shaped" 949).

### The incumbent fails delivery at the act `##arg-16`

Round 2's reply, the agent, on shape A, the section whole in the primer with rereads: "This fails
C3: the reread is a pointer the session may skip." C3 was the label, in that reply, of
`criterion@standards-delivered-as-fragments@delivered-at-the-act`.

### One whole fragment keeps rules in the primer that no act there applies `##arg-17`

Round 2's reply, the agent, on shape B: "each primer reader keeps 949 words of heading and slug
rules that only a writer applies. That goes against your point that some subsections are not
needed everywhere."

### The primer keeps what every session applies in any task `##arg-18`

Round 2's reply, the agent, on shape C: "The primer keeps what every session needs in any task:
recognizing a decision, and reading a head's ground. The writing and judging rules leave the
primer." Attribution "applies to commit messages, issues and tripwires".

### The slug must keep resolving `##arg-19`

The agent, during the grounding and in round 2's reply: "the released 0.6.0 changelog cites
`primer@design-heads`, so that slug has to stay in the primer whichever shape we choose", and
"thaum's root instructions cite "entry test 2 of `primer@design-heads`"".

### An auditor's standard belongs in its definition from its first token `##arg-20`

The agent, during the grounding, from `design@agent-skills@audit-agent-per-axis`: "That supports
putting a fragment in the design-record auditor."

### Several heads cite a part of the section by name `##arg-21`

The agent, during the grounding: heads cite "entry test 2 of `primer@design-heads`" and similar
parts, which constrain how the section is cut; the mapping table "Citations of
`primer@design-heads`" lists them.

### The reviewers' growth, counted as a cost (withdrawn) `##arg-22`

Round 2's reply, the agent: "The two reviewers and the auditor each grow by about 2,600 words, in a
context that already holds the primer's ~1,250 remaining words of it." Withdrawn in round 3's
reply, on `argument@standards-delivered-as-fragments@arg-28`.

### The owner would have proposed the same split and distribution `##arg-23`

Round 3, the owner: "I agree with your C split, I was going to propose the same split and
distribution." Whether these words record the owner's own intent, a ground for the harvested head,
or an approval of the agent's proposal, which is no ground, is the owner's to say: D3.

### The hosts already read the section whole `##arg-24`

Round 3's reply, the agent: "`agent@knowledge-architect-decision-record-reviewer` already judges
"against the whole section", and `agent@knowledge-architect-design-record-auditor` reads
`primer@design-heads` whole before its first entry … So C adds nothing to their context. It saves
them the lookup."

### Three partial readers get one sentence each `##arg-25`

Round 3's reply, the agent: "The routing reviewer gets one sentence on present tense, with its
pointer. The tripwire auditor gets one sentence on attribution, with its pointer. A fragment host
must apply the whole fragment, and neither agent does. The planning harvest loads decision-recording
at that moment anyway, so pointing to it costs no extra read."

### The citations of a part that stays keep their reference `##arg-26`

Round 4's reply, the agent: "Citations of an entry test, the backstop, the ground or attribution
stay valid. That includes thaum's "entry test 2" and the three in the released 0.6.0 changelog,
which cite the section as a whole."

### The rejected design stays live, narrowed `##arg-27`

Round 4's reply, the agent: `rejected@agent-skills@head-rules-in-decision-recording`'s "reason, that
a head is read at every grounding with no skill loaded, still holds for the parts that stay in the
primer, so its text is narrowed to them."

### The reviewers' growth is a misattributed cost `##arg-28`

Round 3, the owner: "growth of auditors and reviewers here is not a cost, because they were already
required to read these instructions to their context anyway. I think this is a common
misattribution of costs that you tend to do in this project."

### A size cost is the context of the session using the text, and a pointer costs context and time `##arg-29`

Round 3, the owner: "It is about the size of the context of a session using the text, not the size
of the text itself, and the argument on delivery I gave in my last message should bear on it too. A
session that needs to resolve a reference, search for location, then reach it, is always wasting
context (and time too, btw) compared to one that receives the instruction directly."

### Two misattributions in this session `##arg-30`

Round 3's reply, the agent: "I made the same mistake twice in this session": the first Q12
recommendation, and the growth listed under shape C's costs.

### Where a size cost exists `##arg-31`

Round 3's reply, the agent, the rule's wording: "A size cost exists only for a session that loads a
statement its act does not apply".

### The rule goes beside the restatement rule `##arg-32`

Round 3's reply, the agent: "Every session that decides between a pointer and a restatement reads
that rule, and this paragraph is the cost that choice weighs."

### Neither misattribution had the agent-configuration skill loaded `##arg-33`

Round 3's reply, the agent, against the rival "the admission standard alone": "Neither of my two
misattributions happened with `skill@knowledge-architect-agent-configuration` loaded. One was a
mapping task, the other this design discussion. Both weighed where installed text goes, and only
the primer reaches those sessions."

### The weighing is every installing project's `##arg-34`

Round 3's reply, the agent, against the rival "this repository's root `CLAUDE.md`": "the same
weighing is done in every project that installs the workflow, by its audits, retrospectives and
repairs."

### The head earns test 2, and the head on a skill's size stays apart `##arg-35`

Round 3's reply, the agent: "Test 2: the same reason is respected at every site that weighs a
placement. Test 4: it records your intent." No question under a label put test 4 to the owner, so
the head stands on test 2 alone, per the harvest. And "`design@agent-skills@no-size-budget` stays
separate. It answers a different question, whether length makes a session drift, on its own
evidence."

### No premortem for the context-cost rule `##arg-36`

Round 3's reply, the agent: "The change touches none of the four things that require one: no stored
data, no interface others consume, no behaviour users adapted to, and no decided thread it would
reopen." The work's premortem, under "Premortem", covers the rule as part of the whole design.

### The record routes a purpose to the goals home `##arg-37`

Round 4's reply, the agent: "The knowledge table of `primer@where-knowledge-goes` routes "what the
project, or one Component, is for" to the goals home." and the section asks a head to cite "the
goal it derives a constraint from".

### The sentence as it stands misrouted a goal in real use `##arg-38`

Round 4's reply, the agent, against the rival "leave the sentence": "That keeps a sentence the issue
shows sent a goal into a head in one real session (`klarch init`)."

### A fragment answers the rival the reread head beat `##arg-39`

Round 4's reply, the agent, on `design@agent-skills@design-home-write-loads-recording`: "Its rival,
"restating the backstop's tests at each site", lost because hand copies drift. A fragment answers
that, so the head is rewritten, not contradicted." A design-entry searcher of the grounding had
graded it "may change" for the same reason: "a fragment delivered at each site is close to that
rival".

### The words that leave the primer, and the words the rule adds, measured `##arg-40`

The agent, round 4's reply and round 3's reply, gave "about 1,350 words" out of the primer for
shape C and "about 110 words" in for the context-cost rule. Measured at this spec's parent commit:
`sed -n '494,502p;528,546p;549,616p' crates/agent-skills/content/PRIMER.md | wc -w` gives 1,287
words for shape C as approved; under D1, `sed -n '528,546p;549,616p'` gives 1,147; the context-cost
paragraph of "The context-cost rule" gives 124 words by `wc -w`.

### The rules on rejected designs are all applied by their writer and their judges `##arg-41`

The assembly, on Q5's condition. `skill@knowledge-architect-decision-recording@losing-alternatives`
holds which threads are candidates, the four recording tests, how an entry is written, its
markers, its citation from a head and when it leaves. A writer applies each; the decision-record
reviewer judges each ("Does each entry that exists carry what it owes?"); the design-record auditor
re-applies each to every entry. The template in it is shown to a writer and checked by a judge.

### "The standing argument" holds a rule a reader of a head applies `##arg-42`

The design-conformance review of this spec: its paragraph, `path@agent-skills@content/PRIMER.md`,
lines 494-502, holds "A constraint from a goal binds outright, where one from a decision is a
presumption, and the reference is what tells a reader which", a rule for reading a head, which
every grounding applies. `rejected@agent-skills@head-rules-in-decision-recording` lost on the
reason that a head is read at every grounding with no skill loaded. Moving that paragraph out of
the primer meets that reason again for this paragraph: D1.

## New names, in one place

```text
crates/agent-skills/fragments/head-entry-tests.md        new
crates/agent-skills/fragments/head-ground.md             new
crates/agent-skills/fragments/head-shape.md              new
crates/agent-skills/fragments/context-cost.md            new
crates/agent-skills/fragments/admission-need-test.md     new
crates/agent-skills/fragments/admission-evidence.md      new
crates/agent-skills/fragments/admission-repair-shape.md  new
crates/agent-skills/fragments/admission-record-check.md  new
crates/agent-skills/fragments/retrospective-two-files.md new
crates/agent-skills/fragments/retrospective-defect-kinds.md new
crates/agent-skills/fragments/losing-alternatives.md     new
crates/agent-skills/fragments/owning-component.md        new
crates/agent-skills/fragments/three-homes.md             new
crates/agent-skills/fragments/reversal-check.md          new
crates/agent-skills/fragments/finding-outcomes.md        new
crates/agent-skills/fragments/skill-self-sufficient.md   new
crates/agent-skills/fragments/trigger-test.md            new
crates/agent-skills/src/lib.rs   each_fragment_has_its_hosts    existing, gains a row per fragment
crates/agent-skills/src/lib.rs   no_fragment_line_ships_outside_its_hosts   new test
`skill@knowledge-architect-decision-recording@what-earns-a-head`   new section
`skill@knowledge-architect-decision-recording@head-shape`          new section
`skill@knowledge-architect-design@what-earns-a-head`               new section
`agent@knowledge-architect-decision-record-reviewer@design-head-rules`   new section
`agent@knowledge-architect-design-record-auditor@design-head-rules`      new section
agent-skills design home   new head, its slug that of `thread@standards-delivered-as-fragments@context-cost-of-agent-text`
```

The fragment paths are listed here and named elsewhere by file name. Each new section is cited by its reference.

## Decided design

### The rules on heads

**The primer's section** keeps its heading and slug, `primer@design-heads`. Its introduction is
rewritten to say: this section holds what every session applies, to recognize a decision that may
need a head and to read a head; the rest of what a head holds, and how a head is shaped, is
`skill@knowledge-architect-decision-recording@head-shape`, which the decision-record
reviewer and the design-record auditor host too. Its subsections become:

```markdown
### What earns a head
{{fragment:head-entry-tests.md}}

### What a head stands on
{{fragment:head-ground.md}}
```

This is an illustration of the shape, not authority for its wording.

**The three fragments**, cut along the table of "What exists today", every paragraph moved
unchanged except where named. Under D1, as built:

- `head-entry-tests.md`: lines 426-491, "What earns a head", whole;
- `head-ground.md`: lines 494-527, "The standing argument" and the paragraphs "A head stands on its
  argument…", "A durable record attributes…" and "What the ground means to a later session.";
- `head-shape.md`: lines 528-546, then lines 549-616, "How a head is shaped", whole. Its sentence
  at line 535 is reworded by `thread@standards-delivered-as-fragments@head-cites-the-goal-it-serves`:
  "**A head carries the intent it serves, its shape and the standing argument, not
  implementation.** How the project is arranged to reach its goals belongs here, each head citing
  the goal it derives a constraint from; what the project is for is its goals home's." followed by
  the rest of that paragraph unchanged. A `%%` line above it states the reason: the knowledge table
  routes a project's purpose to its goals home, and the former sentence sent a goal into a head in
  real use, per that thread.

D1 ruled otherwise, `head-ground.md` takes lines 503-527 and `head-shape.md` takes lines 494-502
first.

**Positional phrases.** Each phrase of a fragment that names a place by position, "below", "above",
"the next section", "this section" and the like, is reworded to name its target, since each host
places the fragment under another heading. The known ones are listed under "What exists today".
Where the target is in a fragment the host does not hold, as the backstop's "by the test of one
decision per head below" in the primer, the phrase names `skill@knowledge-architect-decision-recording@head-shape`.

**The hosts**, the mapping table "Fragment hosts". **In every host, each fragment it hosts sits
under a heading of the host's own, and every other pointer of that host to the same block becomes
a reference to the host's own section that holds it**; the placeholder takes the place of the
pointer that asked for the block to be read whole, and the host's sentence that held the pointer is
rewritten to name its own section. A pointer inside a sentence that names another target too is
rewritten the same way. So:

- decision-recording: `## What earns a head {{slug:what-earns-a-head}}`, after the section on the
  reversal check, hosting `head-entry-tests.md`; and `## What a head holds and how it is shaped
  {{slug:head-shape}}`, before "The current design", hosting `head-ground.md` then `head-shape.md`.
  The skill's first line, the reread, leaves. "The current design" keeps its location paragraph, its
  template and its paragraph on renaming; its sentence "What a head holds and how it is shaped is
  `primer@design-heads`: read it whole before writing a head" and its template's "per the primer's
  section on design heads" point to `skill@knowledge-architect-decision-recording@head-shape`. The reversal check's "carry on to the
  entry tests of `primer@design-heads`" points to the skill's own section. The description's clause
  "what earns a head and how a head is shaped are the primer's section on design heads" is
  rewritten to say the skill holds both;
- design: `## What earns a head {{slug:what-earns-a-head}}`, placed after "Bounded work", hosting
  `head-entry-tests.md`, then a level-three heading "What a head stands on" hosting
  `head-ground.md`, which the in-change path applies when it attributes a ruling in the commit
  message. The in-change path's reread is replaced by a pointer to that section; the `%%` block of
  its lines 129-134, which argues the reread, is deleted; bounded work's "the entry tests of
  `primer@design-heads`" points to it;
- decision-record-reviewer and design-record-auditor: `## The rules on design heads
  {{slug:design-head-rules}}`, with three level-three headings "What earns a head", "What a head stands
  on", "What a head holds and how it is shaped", each hosting its fragment. Every pointer of theirs
  to `primer@design-heads` or to `skill@knowledge-architect-decision-recording@head-shape` points to that section, the reviewer's
  lines 15-18 and 68-69 included. Each opens the section with one sentence of its own host text:
  the rules are written for the session that records a head, and the agent judges the record
  against them; where one asks the owner a question or asks for an edit, the agent reports the case
  in its finding or its draft instead.

**The one-sentence restatements**, per `thread@standards-delivered-as-fragments@head-rules-delivery`:

- `path@agent-skills@content/agents/routing-reviewer.md`, line 106: "A head is written as if the
  design had always been so: present tense, no dates, no account of the change, per"
  `skill@knowledge-architect-decision-recording@head-shape`.
- `path@agent-skills@content/agents/tripwire-auditor.md`, line 43: "A durable record attributes a
  decision to the owner and never quotes the owner, keeping the scope and the hedges of the owner's
  words, per `primer@design-heads`.";
- `path@agent-skills@content/skills/planning/SKILL.md`, line 454: the reread becomes a sentence that
  the rules on what earns a head and how it is shaped are delivered by
  `skill@knowledge-architect-decision-recording`, which the harvest loads.

**The citations**, per the mapping table "Citations of `primer@design-heads`".

The argument: `thread@standards-delivered-as-fragments@head-rules-delivery`. The nearest rival,
shape B, one fragment of the whole section hosted in the primer and every writer and judge, lost
on `argument@standards-delivered-as-fragments@arg-17`: it keeps in every session's primer the shape
rules no act there applies, 1,147 words by `argument@standards-delivered-as-fragments@arg-40`,
which is the size cost `thread@standards-delivered-as-fragments@context-cost-of-agent-text` counts.
Shape A, the incumbent, lost on `argument@standards-delivered-as-fragments@arg-16`.
`design@agent-skills@design-home-write-loads-recording` is rewritten, not contradicted, on
`argument@standards-delivered-as-fragments@arg-39`.

### The context-cost rule

`context-cost.md` holds the words the owner approved in round 4, wrapped at 100 columns as the
installed text is:

> **The size cost of agent-facing text is counted in the context of the sessions that load it, never in the text's length.** A statement a session applies to its act costs that session nothing when the text it loads delivers it, since the session holds it either way. A pointer the session must follow costs more: the reference resolved, the location found and the target read, each in context and in time, and the risk that the session acts from memory instead. A size cost exists only for a session that loads a statement its act does not apply: the primer's text, in every session that never meets its occasion, or a fragment's copy, in a host whose act does not apply it whole.

Its hosts: `primer@where-knowledge-goes`, beside the restatement rule, and each host of
`admission-repair-shape.md`, its placeholder right after that fragment's, since a fragment holds no
other fragment. The nearest rival, the admission standard alone, lost on
`argument@standards-delivered-as-fragments@arg-33`.

### The migrated blocks

Each block's home keeps its section heading and slug and hosts its fragment in place of its body.
Each other host follows the rule on hosts of "The rules on heads": the fragment under a heading of
its own, the host's other pointers to the block made references to that section. The fragments and
their hosts are the mapping table "Fragment hosts". A fragment holds no heading; each host writes
the headings it needs.

- **Q1, the admission standard**, cut into four fragments, one per subsection, "The outcomes of a
  need-test failure" joined to the need test, since the outcomes are the need test's:
  `admission-need-test.md`, `admission-evidence.md`, `admission-repair-shape.md`,
  `admission-record-check.md`. In the home skill, the subsections take the order need test,
  outcomes, evidence and kind, shape of a repair, record check, each under its level-three heading.
  The standard's opening paragraph, on what it binds, stays host text in the skill. The record
  check's last paragraph, "A project's skills and agents are walked by … Run it after editing
  one.", stays host text in the skill, since no judging host performs that act. The agent-text
  reviewer hosts all four, and its instruction to read the section "from the installed skill's file
  in the tree you review" leaves; a branch that changes a fragment has the reviewer read its own
  definition from disk, per `skill@knowledge-architect-review@review-invariants`. The intake hosts
  all four, since it judges a fix by the whole standard, per
  `design@agent-skills@intake-shares-the-admission-standard`, its two pointers, lines 105-106 and
  134, made references to its own section.
- **Q2**: `retrospective-two-files.md`, under D4: the format the intake reads, the table of the two
  files, what `<subject>` names and where an interaction finding goes, with the `%%` lines on the
  subject. The rest of `skill@knowledge-architect-retrospective@two-files`, proposing the subject,
  checking the names, the second file kept free of the project, and where the files go, are the
  retrospective session's own acts, false in the intake, and stay host text in the retrospective.
- **Q3**: `retrospective-defect-kinds.md`, the definitions alone, reworded into one sentence of
  definitions: an item is unclear when it could be read two ways, missing when the session could
  not follow the workflow, or could not produce a document the checker accepts, without an
  instruction that does not exist, and wrong when following it produced a defect or a correction;
  and the sentence that a decision taken by judgement where no instruction covers it is not
  missing, with its `primer@room-to-judge`. The retrospective's own acts, reading back over the
  session, listing per subject, quoting the instruction, stay host text in the retrospective.
- **Q5**: `losing-alternatives.md`, the body of
  `skill@knowledge-architect-decision-recording@losing-alternatives`. In the decision-record
  reviewer, its lines 87-98 keep their two bold questions and the sentence "apply them to every
  thread whose shape lost to an argument: a ruled-out thread, a thread withdrawn with a defeating
  reason, a superseded thread whose distinct shape lost", and point to the reviewer's section that
  hosts the fragment; the rest of the two paragraphs, which restates the skill, leaves.
- **Q6**: `owning-component.md`. **Q7**: `three-homes.md`, whose sentence "The standing argument,
  and what stays out of the head, are `primer@design-heads`" names `primer@design-heads` for the
  standing argument, under D1, and `skill@knowledge-architect-decision-recording@head-shape` for what stays out of the head. In the
  design-record auditor, which hosts both, the pointer stays in the fragment as written, since a
  fragment cannot name its host's own section. **Q8**: `reversal-check.md`.
- **Q9**: `finding-outcomes.md`, whose three mentions of "the axis's section" are reworded to name
  the axis's section of `skill@knowledge-architect-project-audit`.
- **Q10**: `skill-self-sufficient.md`, the paragraph "A skill is self-sufficient", its test per
  pointer included.
- **Q11**: `trigger-test.md`, the body of `skill@knowledge-architect-issue-tracking@trigger-test`.

The argument: `thread@standards-delivered-as-fragments@fragment-migrations`. The nearest rival, the
pointer read whole, lost on `argument@standards-delivered-as-fragments@arg-29`: a session that must
resolve a pointer and read its target spends more context than one that receives the text.

## Mapping tables

### Fragment hosts

The hosts test holds this table. A host is named by its content file under content/. Under D2, as
built.

| fragment | hosts |
| --- | --- |
| `head-entry-tests.md` | PRIMER.md, skills/decision-recording, skills/design, agents/decision-record-reviewer, agents/design-record-auditor |
| `head-ground.md` | PRIMER.md, skills/decision-recording, skills/design, agents/decision-record-reviewer, agents/design-record-auditor |
| `head-shape.md` | skills/decision-recording, agents/decision-record-reviewer, agents/design-record-auditor |
| `context-cost.md` | PRIMER.md, skills/agent-configuration, agents/agent-text-reviewer, skills/project-audit, skills/retrospective, skills/retrospective-intake |
| `admission-need-test.md` | skills/agent-configuration, agents/agent-text-reviewer, agents/workflow-auditor, skills/retrospective-intake |
| `admission-evidence.md` | skills/agent-configuration, agents/agent-text-reviewer, skills/retrospective-intake |
| `admission-repair-shape.md` | skills/agent-configuration, agents/agent-text-reviewer, skills/project-audit, skills/retrospective, skills/retrospective-intake |
| `admission-record-check.md` | skills/agent-configuration, agents/agent-text-reviewer, agents/workflow-auditor, skills/retrospective-intake |
| `retrospective-two-files.md` | skills/retrospective, skills/retrospective-intake |
| `retrospective-defect-kinds.md` | skills/retrospective, skills/retrospective-intake |
| `losing-alternatives.md` | skills/decision-recording, agents/decision-record-reviewer, agents/design-record-auditor |
| `owning-component.md` | skills/decision-recording, agents/routing-reviewer, agents/design-record-auditor |
| `three-homes.md` | skills/decision-recording, agents/design-record-auditor |
| `reversal-check.md` | skills/decision-recording, agents/decision-record-reviewer |
| `finding-outcomes.md` | skills/project-audit, agents/design-record-auditor, agents/tripwire-auditor, agents/workflow-auditor |
| `skill-self-sufficient.md` | skills/agent-configuration, agents/routing-reviewer, agents/workflow-auditor |
| `trigger-test.md` | skills/issue-tracking, agents/standing-state-reviewer |

### Citations of `primer@design-heads`

Each citation, and each citation in words of the same rules, is mapped by the rule it cites and
by whether its text hosts the fragment that holds that rule.

| the citation cites | in a text that hosts the fragment | elsewhere |
| --- | --- | --- |
| the section as a whole, for reading a head or telling whether a change bears on one | the host's own section | unchanged |
| an entry test, the backstop, the routing of a new member, "a choice that turns on what the specification means" | the host's own section | unchanged |
| the standing argument, the ground, attribution, what the ground means to a later session | the host's own section | unchanged |
| the deliberation not copied, present tense, intent and not implementation, the list of citing texts, reliance on the checker, any rule of "How a head is shaped" | the host's own section | `skill@knowledge-architect-decision-recording@head-shape` |
| every rule on heads | the host's own section | both `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape` |
| a released section of CHANGELOG.md | none | unchanged: the released sections cite the section as a whole |

A fragment cites the primer and `skill@knowledge-architect-decision-recording@head-shape` by these rules, as "elsewhere", since it
cannot name the section of the host it lands in.

## Losing alternatives

- **Shape A, the section whole in the primer with its rereads**, lost to
  `thread@standards-delivered-as-fragments@head-rules-delivery` on
  `argument@standards-delivered-as-fragments@arg-16`. It is the incumbent, recorded by
  `design@agent-skills@one-home-for-head-rules` and `primer-reread-before-recording`.
- **Shape B, one fragment of the whole section, hosted in the primer and every writer and judge**,
  lost to the same thread on `argument@standards-delivered-as-fragments@arg-17`.
- **Q12 as first recommended, no fragment of the primer's sections**, withdrawn by the agent on
  `argument@standards-delivered-as-fragments@arg-29`.
- **The context-cost rule in the admission standard alone**, lost to
  `thread@standards-delivered-as-fragments@context-cost-of-agent-text` on
  `argument@standards-delivered-as-fragments@arg-33`; **in this repository's root `CLAUDE.md`**,
  lost on `argument@standards-delivered-as-fragments@arg-34`.
- **The sentence on a project's purpose left as it is**, lost to
  `thread@standards-delivered-as-fragments@head-cites-the-goal-it-serves` on
  `argument@standards-delivered-as-fragments@arg-38`.

Judged by the four tests of `skill@knowledge-architect-decision-recording@losing-alternatives`, as
the skill's text stands:

- **Shapes A and B pass test 1.** The work makes `skill@knowledge-architect-decision-recording@head-shape` a slug of the installed text,
  an interface whose consumers the Migration entry sends there, and either shape would remove it.
  The harvest writes one `rejected` entry for each, in `register@agent-skills@rejected`, cited by
  the rewritten `design@agent-skills@one-home-for-head-rules`; shape A also moves there as the
  reversed decision, per `skill@knowledge-architect-decision-recording@reversal-check`. Whether
  test 1 should admit every rival of an interface head is
  `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`,
  which stays open; the harvest follows the text as it stands.
- **The others pass none.** None changes an interface, reads a specification or lost to evidence
  that cost work; each is refuted by an argument derived again in one round. Test 4: the doubts the
  premortem keeps, causes 3 and 5, bear on the kept design and on shape A alike, so no rival is the
  candidate they would reopen.

## Readings

The work reads no external specification.

## Premortem

Assume the work shipped and failed.

1. **A fragment sentence is false in one host**, as entry test 4's "ask the owner … `Q<n>`" in a
   reviewer that cannot ask the owner. Stresses `thread@standards-delivered-as-fragments@head-rules-delivery`
   and `thread@standards-delivered-as-fragments@fragment-migrations`. Verdict: an acceptance
   criterion on the owner's word, AC2; and a clause of the design, the host sentence that opens each
   agent's section on the rules on heads.
2. **A text that hosts no `head-shape.md` needs a shape rule whole** and must look it up, which
   would show the boundary of the split wrong. Stresses
   `thread@standards-delivered-as-fragments@head-rules-delivery`. Verdict: an acceptance criterion
   on the owner's word, AC1.
3. **An adopting project's own text cites `primer@design-heads` for a shape rule**, and after its
   pin move resolves to a section that no longer holds the rule; the checker cannot see it. Stresses
   `thread@standards-delivered-as-fragments@head-rules-delivery`. Verdict: survives into the
   Migration entry of step 6; named as a cause for the harvest to judge as a tripwire.
4. **The rule is read as "size never costs"**, and a later fragment gets a host that applies only
   part of it. Stresses `thread@standards-delivered-as-fragments@context-cost-of-agent-text`.
   Verdict: named as a cause for the harvest to judge as a tripwire. The discussion's reading,
   round 4's reply: it "is caught by the agent-text review and by L9 of the workflow audit", so
   condition 1 of the tripwire standard, a silent failure, would exclude it.
5. **A session that writes no head misses the backstop mid-task**, the primer copy being read at
   the start only. Stresses `thread@standards-delivered-as-fragments@head-rules-delivery`. Verdict:
   survives into `tripwire@agent-skills@head-created-without-deliberation`, reworded in step 6.

## Acceptance criteria

### Every citation of `primer@design-heads` names a rule the section still holds, or is re-pointed `##citations-name-what-the-section-holds`

AC1. Guards `thread@standards-delivered-as-fragments@head-rules-delivery`, its split boundary.
Judged in steps 1 and 6, at their commits, by reading each citation that
`git grep -n 'primer@design-heads'` returns, and each citation in words that
`git grep -n -i "section on design heads\|owns the shape"` returns: at step 1 over content/ and
fragments/, at step 6 over every Markdown file outside .claude and the plans directory; against the
mapping table "Citations of `primer@design-heads`". It fires on a
citation, in a text that hosts no `head-shape.md`, whose act applies a rule of `head-shape.md`
whole. Response: reopen `thread@standards-delivered-as-fragments@head-rules-delivery` on where the
split falls, in a design session before the merge.

### Each host's installed copy of each new fragment holds no sentence false in that host, and none that asks an act its agent cannot perform `##host-copies-true-in-their-host`

AC2, as the owner ruled it, round 5, on the text of round 4's reply: "each host's installed copy of
each new fragment holds no sentence that is false there, and none that asks an act its agent cannot
perform. It is checked by the agent-text axis, briefed with every host copy, as
`path@agent-skills@CLAUDE.md` asks. If it fails, #head-rules-delivery's host list is reopened."
Guards `thread@standards-delivered-as-fragments@head-rules-delivery`, its host list. Judged at the
review of the work. It fires on any such sentence in any host's copy. Response: reopen the host
list of `thread@standards-delivered-as-fragments@head-rules-delivery`.

## Implementation sequence

Every step ends with `cargo klarch install-agent-skills`, `cargo klarch check`, `cargo test -p
knowledge-architect-agent-skills`, and stages each edited content file with its installed copy, per
`issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`.

### Step 1: the rules on heads

Builds: the three fragments `head-entry-tests.md`, `head-ground.md`, `head-shape.md`, the
rewording of `thread@standards-delivered-as-fragments@head-cites-the-goal-it-serves`, the primer's
section rewritten, the new sections of decision-recording, design, the decision-record reviewer and
the design-record auditor, the three one-sentence restatements, the removed rereads, and every
citation of the installed text re-pointed per the mapping table. Closes
`the-primer-s-design-head-section-sends-a-project-s-purpose-to-a-head`.

Claims:

- each fragment ships in exactly the hosts the mapping table lists: three rows of the hosts test.
  Shown to fail by adding `{{fragment:head-shape.md}}` to PRIMER.md in a scratch worktree, which the
  test reports;
- no shipped file outside a fragment's hosts holds a line of it: a new test,
  `no_fragment_line_ships_outside_its_hosts` in `path@agent-skills@src/lib.rs`, which reads every
  line of every fragment longer than 60 characters, trimmed, and fails where a shipped file outside
  the fragment's hosts holds it. Shown to fail by leaving one paragraph of `head-shape.md` in
  PRIMER.md in a scratch worktree. The hosts test alone cannot show it, since it finds a host by the
  fragment's first line;
- AC1 over content/ and fragments/: the reading of each citation, its outcome listed in the commit
  message;
- the build still refuses a fragment no placeholder names, and
  `every_host_ships_each_fragment_it_names` in `path@agent-skills@src/lib.rs` still passes: both
  run at every step.

Fails alone on: a host list that differs from the table, or the build refusing a fragment.

### Step 2: the decision-recording blocks

Builds: `losing-alternatives.md`, `owning-component.md`, `three-homes.md`, `reversal-check.md` and
their hosts, Q5 to Q8, with the decision-record reviewer's restatement of the rejected-design rules
replaced.

Claims: four rows of the hosts test, each shown to fail by removing one host's placeholder in a
scratch worktree.

Fails alone on: a row of those four.

### Step 3: the admission standard and the context-cost rule

Builds: the four admission fragments, `context-cost.md`, and their hosts, Q1 and
`thread@standards-delivered-as-fragments@context-cost-of-agent-text`; the agent-text reviewer's
instruction to read the skill's file leaves; the `%%` line of the intake that cites
`design@agent-skills@intake-shares-the-admission-standard` is reworded.

Claims: five rows of the hosts test, each shown to fail as above; the context-cost paragraph holds
the words of round 4 in their order, checked by comparing the fragment's words, line breaks
removed, with "The context-cost rule".

Fails alone on: a row of those five.

### Step 4: the retrospective blocks

Builds: `retrospective-two-files.md`, `retrospective-defect-kinds.md` and their hosts, Q2 and Q3.

Claims: two rows of the hosts test, each shown to fail as above.

Fails alone on: a row of those two.

### Step 5: the audit, configuration and trigger blocks

Builds: `finding-outcomes.md`, `skill-self-sufficient.md`, `trigger-test.md` and their hosts, Q9 to
Q11.

Claims: three rows of the hosts test, each shown to fail as above.

Fails alone on: a row of those three.

### Step 6: the record

Builds: the harvest rows below, the re-pointed citations of this repository's documents, AC1 over
them, the two `todo` issues of "What is outside the work", and the changelog entries, then
`cargo x changelog`.

Claims: AC1 over this repository's documents, by reading, its outcome in the commit message;
`cargo klarch check` passes; the changelog copies match, by the test that fails while a copy
differs.

Fails alone on: a dangling reference, or a head left asserting the old delivery.

## Order rationale

- Step 1 before step 2: both edit the decision-recording skill and the two record agents, and the
  sections step 1 adds are where step 2's fragments sit beside them.
- Step 2 before step 3: step 3 edits other files; the order only keeps the record agents' edits
  together.
- Step 3 before step 4: the retrospective skill hosts `admission-repair-shape.md` from step 3, and
  step 4 adds its own fragments beside it.
- Step 4 before step 5: independent; the order follows the round-1 table.
- Step 5 before step 6: the record states the delivery the earlier steps built.

## Defaults awaiting the owner

Each is built as written until the owner rules, and the branch merges once the owner has, per
`primer@question-authority`.

- **D1**, ruled: accepted by the owner. On `thread@standards-delivered-as-fragments@head-rules-delivery`: "The standing argument",
  `path@agent-skills@content/PRIMER.md`, lines 494-502, stays in the primer, in `head-ground.md`,
  where shape C as approved put it in `head-shape.md`. Ground:
  `argument@standards-delivered-as-fragments@arg-42`. It narrows what leaves the primer, a ruling
  of the owner, so it is the owner's. Default: it stays.
- **D2**, ruled: accepted by the owner. On `thread@standards-delivered-as-fragments@fragment-migrations`: three hosts of the
  round-1 table are dropped. `skill@knowledge-architect-project-audit` from Q5 and Q6: its lines
  150-151 name the rules the design-record auditor applies, and the session applies none of them.
  `skill@knowledge-architect-design` from Q7: round 1 placed its use in "the design skill's
  in-change path", which was wrong; its one pointer to the block, line 399, uses one sentence, where
  a deliberation is kept. Each would host a fragment its act does not apply whole, against
  `criterion@standards-delivered-as-fragments@host-applies-whole`. It narrows the owner's host
  lists. Default: dropped.
- **D3**, ruled: the owner declined it; the head does not attribute the split to the owner and stands on its argument. On `thread@standards-delivered-as-fragments@head-rules-delivery`: the owner's words of
  round 3, "I was going to propose the same split and distribution", record the owner's own intent,
  so the rewritten `design@agent-skills@one-home-for-head-rules` attributes the split by act to the
  owner, as test 4 of `primer@design-heads` asks the owner to confirm. Default: attributed to the
  owner, the head standing also on its argument.
- **D4**, ruled: accepted by the owner. On `thread@standards-delivered-as-fragments@fragment-migrations`: Q2's fragment holds the
  format of `skill@knowledge-architect-retrospective@two-files` alone, not its whole body, found at
  step 4: its other paragraphs ask acts only the retrospective session performs, which AC2 reports
  as false in the intake. It narrows the owner's ruling on Q2. Default: the format alone.

## Harvest

| what | where | when |
| --- | --- | --- |
| `thread@standards-delivered-as-fragments@head-rules-delivery` | `design@agent-skills@one-home-for-head-rules`, rewritten in place, slug kept: its centralization stated as one file per rule, attributed to the owner, with no quotation of the owner left, per the rules on a head a change touches; its ground per D3. `primer-reread-before-recording` leaves. `design@agent-skills@new-or-reshaped-head-needs-design`, `design@agent-skills@design-home-write-loads-recording`, `design@agent-skills@verification-in-primer`, `design@agent-skills@primer-limit`, `design@agent-skills@design-record-axis`, `design@agent-skills@behaviour-additions-need-real-use` rewritten where they state the delivery. `rejected@agent-skills@head-rules-in-decision-recording` narrowed. Two new `rejected` entries, shapes A and B, per "Losing alternatives" | step 6 |
| `thread@standards-delivered-as-fragments@fragment-migrations` | `design@agent-skills@shared-text-fragment`: the migrated blocks, and which host lists are the owner's, with no host listed; `design@agent-skills@admission-rules-home`, `design@agent-skills@additions-review-agent`, `design@agent-skills@intake-shares-the-admission-standard`, `design@agent-skills@intake-skill-home` rewritten where they state the delivery | step 6 |
| `thread@standards-delivered-as-fragments@context-cost-of-agent-text` | a new head of `register@agent-skills@design`, under the thread's slug, admitted by entry test 2; its rule attributed to the owner, who proposed it, and its wording and its two locations standing on their arguments, `argument@standards-delivered-as-fragments@arg-31` to `argument@standards-delivered-as-fragments@arg-34`, approved by the owner; citing `design@agent-skills@restatement-size-test` and `design@agent-skills@primer-limit` | step 6 |
| `thread@standards-delivered-as-fragments@head-cites-the-goal-it-serves` | the fragment's text, and the `%%` line above it that carries its reason; no head | step 1 |
| the premortem's five causes | every cause with its outcome in the harvest's commit message: causes 1 and 2 by AC2 and AC1, reported there; cause 5 by `tripwire@agent-skills@head-created-without-deliberation`; causes 3 and 4 judged by the tripwire standard, written or dropped with the condition each fails | step 6 |
| `tripwire@agent-skills@head-created-without-deliberation` | its premise reworded to the new delivery | step 6 |
| `a-pointer-delivers-its-directive-only-when-the-session-fetches-it` | closes | step 6 |
| the two limits of "What is outside the work" | two `todo` issues of `register@agent-skills@issue`: the workflow's general-purpose repair agents read the admission standard by a pointer; a pointer that applies a whole standard with no "read whole" near it was not searched | step 6 |
| CHANGELOG.md, `Next release` | Workflow: the rereads of the primer's section are removed, and the rules on heads and the migrated standards are delivered in the texts that apply them; Migration: a project's text that cites `primer@design-heads` for a rule on a head's shape cites `skill@knowledge-architect-decision-recording@head-shape` instead | step 6 |

## Later consequences

- A later edit of a migrated standard edits its fragment, and its review reads every host's
  installed copy, per `path@agent-skills@CLAUDE.md`.
- `issue@agent-skills@plan-documents-keep-the-owner-s-words-verbatim-where-other-records-attribute-them`
  and `issue@agent-skills@the-first-test-for-a-losing-alternative-admits-every-rival-of-an-interface-head`
  stay open; their closing edits now land in `head-ground.md` and `losing-alternatives.md`.
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`
  stays open; the delivery argument of this work bears on its discussion.
