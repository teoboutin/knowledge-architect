# The primer defines the terms it uses, and sends a session about to change code to the record of that code

## Status and audience

This spec is the plan of one branch's work on the installed text. The work does three things:

- the primer gains a section that defines, in one sentence each with a pointer to its home, every
  term the workflow coins that the primer uses, and every word the workflow uses in two senses; the
  admission standard gains the rule that keeps coined terms defined that way;
- the primer's `primer@verify-before-relying` gains a paragraph that sends a session about to propose
  a fix, or any other change to code, to the record of that code: the entries its comments cite, the
  entries that cite its file, and the heads on its subject in its Component's design and `rejected`
  homes;
- the README of the agent-skills crate gains a diagram of the activities and their hand-offs, for
  human readers.

It is written for a session that did not witness the design discussion that produced it. It leaves
the repository in a commit of its own, after its harvest and the review before the merge.

- Where this spec and a design home disagree on a point "What is already decided" does not list as
  reversed or rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined in it, under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits: D1 is built as written, and the branch merges once the owner has ruled on it.
- It was assembled from one transcript, extracted with `cargo klarch transcript show`. The
  discussion begins at the owner's message that opens "I was thinking about adding a glossary in
  the primer about the terms knowledge-architect uses", and ends at the owner's message "Agreed on
  this table with AC1 and AC2, proceed to  spec and implementation if spec review finds nothing
  critical." It holds 3 owner messages, called rounds 1 to 3, and 2 agent replies that carry the
  ledger: "the reply to round 1", headed "Glossary and workflow outline: first round", and "the
  reply to round 2", headed "Glossary and workflow outline: second round". Find the session by that
  opening message, not by its file's name.

## How the work is done

By `skill@knowledge-architect-planning`, its section on the work of a spec.

## Names

- **the installed copies**: the files `cargo klarch install-agent-skills` writes under .claude: the
  primer at .claude/knowledge-architect/PRIMER.md, and the skills, agents and saved workflows whose
  names begin with `knowledge-architect-`. The source they are rendered from is
  `path@agent-skills@content/`, with the fragments of `path@agent-skills@fragments/` copied in by
  the build.
- **the primer**: `path@agent-skills@content/PRIMER.md`, installed as
  .claude/knowledge-architect/PRIMER.md.
- **the need-test fragment**: `path@agent-skills@fragments/admission-need-test.md`, hosted by
  `skill@knowledge-architect-agent-configuration@admission-standard` and by the agents that judge
  agent-facing text; the hosts test `each_fragment_has_its_hosts` of `path@agent-skills@src/lib.rs`
  holds the list.
- **a coined term**: a word or phrase the installed text gives a meaning of its own, which a reader
  who knows only common developer usage would not give it, such as "slice" or "bounded work".
- **the census**: the reading of step 1, which lists each coined term the primer uses with whether
  the primer defines it. Its lead is a phrase search over the primer, as the commands in the
  message of the commit that lands step 1 show; a phrase search misses a phrase split over a line
  break, so the census reads the primer whole as well.
- **off-path session**: a session whose task enters no installed skill's trigger that grounds it in
  the record, such as an investigation of a reported bug.

## What the work is

What exists at each site the work touches, at the commit this spec is written on:

- **The primer** has 15 level-two sections, from `primer@goals-bind` to `primer@installed-skills`,
  and 5927 words, counted with `wc -w crates/agent-skills/content/PRIMER.md`. It holds no section
  of definitions. `primer@checker-model` defines register, home, entry, anchor, location,
  Component, reference, kind (a reference's), id, plan document and item. The primer uses, without
  defining them: "installed text" (line 81 and lines 331 and 332), "slice", "full path" (line 446),
  "bounded work" (line 442), "thread" (line 233), "premortem" (line 235), "pin" (lines 412, 452 and
  457) and "axis" (line 457). Line numbers are of the source file.
- **`primer@verify-before-relying`**, its subsection "Check whether it is already known", names
  `issues`, `tripwires` and `show` as what a session reads before diagnosing. No other text a
  session outside the design skill loads names a design home as a place to read before a fix:
  `skill@knowledge-architect-issue-tracking@read-before-diagnosing` names the issues and the
  tripwires.
- **The need-test fragment** holds the bullet "**A definition** passes when an instruction anywhere
  in the workflow uses the term."
- **`path@agent-skills@README.md`** has 439 lines and the sections "Extending an installed skill",
  "The design skill: what it changes", "What it expects of you", "What the setup skill expects of
  you", "Model requirements", "Grounding in the literature" and "Origin". It holds no map of the
  skills.
- **The record search**, `agent@knowledge-architect-record-searcher`, is dispatched by the design
  skill, the planning skill's design audit and the retrospective intake only, per
  `design@agent-skills@record-searched-before-the-work`.

Outside the work:

- **Grounding for development work as a whole.** The owner, round 2, plans a goal for it, "but not
  right now". It is set under `skill@knowledge-architect-goal-setting` when the owner states it.
- **Sending bug fixes through the design skill's grounding**, which dispatches the record search.
  Parked: `thread@terms-and-fix-grounding@off-path-grounding`, Losing alternatives.
- **Rewriting every existing installed text so that it points to a term's home at first use.** The
  rule of `thread@terms-and-fix-grounding@term-defined-at-first-load` applies to new text and to
  text a change edits, as `design@agent-skills@restatement-size-test` applies its own.
- **How the workflow names the people who rule**: `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`.
  The terms section defines "the owner" as the installed text uses it today.
- **A skill loaded long ago in a long session.** The owner, round 2: "this happens rarely, so we
  can put it aside."

## What is already decided

The decisions the design rests on and does not argue again:

- `design@agent-skills@primer-limit`: the primer holds what every session needs and no installed
  skill delivers. Both primer additions are new members; the harvest adds them to its list.
- `design@agent-skills@context-cost-of-agent-text`: the size cost of the new text is counted in the
  sessions that load it and do not apply it.
- `design@agent-skills@restatement-size-test`: a one-sentence restatement carries its pointer.
- `design@agent-skills@one-home-for-head-rules`: a statement goes where the act that applies it
  happens; the split of terms by the texts that use them follows it.
- `design@agent-skills@behaviour-additions-need-real-use`: the paragraph of
  `thread@terms-and-fix-grounding@off-path-grounding` is a behaviour addition, admitted on the
  owner's real-use evidence, `argument@terms-and-fix-grounding@arg-6`.
- `design@agent-skills@statement-names-an-act`: a definition passes when an instruction uses the
  term; a duplicate inside one text fails, so a term `primer@checker-model` defines takes no entry.
- `design@agent-skills@record-searched-before-the-work`: no search before other work. Kept: the
  paragraph dispatches no searcher.
- `design@agent-skills@no-routing-table` and `rejected@agent-skills@root-routing-table`: no table
  of skill hand-offs in agent-facing text; the outline goes to a README.
- `design@agent-skills@shipped-text-cites-no-entry`: the new installed text cites only the shipped
  set's skills, agents and primer sections.
- `design@agent-skills@shared-text-fragment`: the need-test fragment is edited once and read in
  each host.

No recorded decision is reversed or rewritten.

The entries the discussion's search graded `bears`, each with its outcome:

- **Constrain the work and are met as the Criteria say:** `goal@knowledge-architect@agents-work-without-drift`,
  `goal@agent-skills@installed-text-leaves-room-to-judge`, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@installed-text-works-anywhere`, `goal@knowledge-architect@any-project-can-adopt-it`,
  `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@the-workflow-improves-through-real-use`,
  `design@agent-skills@primer-limit`, `design@agent-skills@context-cost-of-agent-text`,
  `design@agent-skills@behaviour-additions-need-real-use`, `design@agent-skills@shipped-text-cites-no-entry`,
  `design@agent-skills@statement-names-an-act`.
- **Served by the work:** `goal@knowledge-architect@adoption-is-easy`, by the README outline;
  `goal@knowledge-architect@structure-and-workflow-work-together` and `goal@core@records-reach-their-reader`,
  by the paragraph, which sends a session to `show` and to the homes rather than listing entries by hand.
- **Constrain how the terms section and the outline are written, and are respected:**
  `design@agent-skills@restatement-size-test` (one sentence with its pointer, checked by
  `acceptance@terms-and-fix-grounding@terms-entries-restate-nothing-new`),
  `goal@knowledge-architect@documentation-stays-consistent` (same check),
  `design@agent-skills@one-home-for-head-rules`, `design@agent-skills@capabilities-not-structure`
  and `goal@agent-skills@one-skill-per-activity` (the outline is in a README, not agent-facing text),
  `design@agent-skills@root-cause-repair` (the terms section restates no procedure),
  `design@agent-skills@no-routing-table` and `rejected@agent-skills@root-routing-table` (no
  hand-off table in agent-facing text), `design@agent-skills@no-external-handoff` (the README names
  only installed skills), `design@agent-skills@checker-model-in-primer` (the terms section restates
  none of its definitions), `rejected@agent-skills@checker-model-printed-by-binary` (the terms are
  in the primer, not printed by a command), `design@agent-skills@record-searched-before-the-work`
  (kept, as above), `design@agent-skills@shared-text-fragment` (the fragment's hosts are read).
- **Define a term the terms section points to, and each pointer must agree with it:**
  `design@agent-skills@document-vocabulary`, `design@agent-skills@step-is-no-commit` and
  `design@agent-skills@spec-and-milestone` (spec, milestone, slice, step);
  `design@agent-skills@bounded-path-in-design` (bounded work); `design@agent-skills@in-change-path`
  and `design@agent-skills@design-hands-off-to-planning` (in-change path, full path);
  `design@core@owned-namespace-check` (installed text); `design@core@harness-kinds` and
  `design@core@harness-kinds-cited-without-anchor` (the citation forms the pointers use).
- **Define terms `primer@checker-model` already holds, so the terms section adds no entry for them:**
  `design@core@rejected-register`, `design@core@registers-are-declared`,
  `design@core@heading-register-two-shapes`, `design@core@a-file-register-is-a-directory-of-entries`,
  `design@core@declarable-directory-register`, `design@core@components-carry-the-same-documents`,
  `design@core@anchors-are-components-and-locations`, `design@core@a-component-carries-what-a-project-attaches`,
  `design@core@register-home-reference`, `design@core@plans-is-a-reserved-anchor`,
  `design@core@one-entity-table`, `design@core@an-entry-is-a-heading-at-the-register-level`,
  `design@core@an-entity-belongs-to-its-anchor`, `design@core@plan-register`,
  `design@core@plans-dir-fixed`, `design@core@plan-document-kinds`, `design@core@plan-items-by-section`,
  `design@core@milestone-entries-are-directories`, `design@core@anchors-listing-command`.
- **Fix how the new text is written:** `design@core@declared-command` (commands as `{{command}}`
  in the source); `design@core@section-homes-carry-slugs` (the new level-two section owes a slug,
  which becomes an interface consuming projects cite); `design@agent-skills@design-home-write-loads-recording`
  (the harvest loads the decision-recording skill); `design@agent-skills@existing-heads-on-touch`
  (a head the harvest edits is brought to the rules); `design@agent-skills@instruction-record-is-minimal`
  (only the decisions of the harvest rows are recorded).
- **Describe hand-offs the README outline draws, and the outline agrees with each:**
  `design@agent-skills@retro-trigger`, `design@agent-skills@retro-offered-once`,
  `design@agent-skills@project-file-goes-to-an-intake`, `design@agent-skills@intake-skill-home`,
  `design@agent-skills@audit-triggers`, `design@agent-skills@plan-written-in-the-converging-session`,
  `design@agent-skills@undesigned-work-is-an-issue`, `design@agent-skills@roadmap-orders-issues`,
  `design@agent-skills@design-home-is-built-intent`, `design@agent-skills@new-or-reshaped-head-needs-design`.
- **Judge the work's own review:** `design@agent-skills@review-judges-additions` and
  `design@agent-skills@additions-review-agent` (the agent-facing-text axis is owed);
  `design@agent-skills@synthetic-evidence-not-built` (no run is built to see whether sessions use the
  new text; its evidence is real use).
- **Issues the work touches:**
  - `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`: the terms section defines "the
    owner" as the text uses it today; the issue stays open.
  - `issue@agent-skills@a-pending-ruled-file-is-reached-by-no-installed-search`: the paragraph
    names no directory of pending files; the issue stays open, unchanged.
  - `issue@agent-skills@pointers-written-bare-to-clear-a-finding-reach-no-check`: the new text names
    no project root directory in plain text; the paragraph names homes by their kind.
  - `issue@core@no-checked-form-cites-a-point-inside-a-section`: the pointers of the terms section
    name level-two sections only.
  - `issue@agent-skills@the-word-plan-is-banned-alone-and-used-alone`: the terms section points to
    `skill@knowledge-architect-planning@planning-terms` and does not settle the issue.
  - `issue@agent-skills@shipped-text-citing-an-entry-is-unchecked`: the review of the agent-facing
    text checks by hand that the new text cites no entry.
  - `issue@core@a-home-for-developer-contracts-outside-agent-configuration`: the terms section
    defines "agent configuration" as the agent-configuration skill states its scope today.
  - `issue@agent-skills@the-closing-audit-of-a-move-reaches-no-planning-text`: the README outline
    names the audit's occasions as `skill@knowledge-architect-project-audit` states them, and adds
    no hand-off; the issue stays open.

The entries the search graded `may-bear` that the grounding read:

- `rejected@agent-skills@head-rules-whole-in-primer`: its argument, a statement read once at the
  start of a session dilutes in a long one, is cause P1 of the Premortem.
- `rejected@agent-skills@directive-restated-everywhere` and `tripwire@agent-skills@pointer-not-followed`:
  the terms section's pointers replace no restatement; the tripwire's clause is not met by this work.
- `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`:
  the same mechanism as P1.
- `issue@agent-skills@an-issue-s-re-entry-point-is-read-by-no-review`: bears on nothing in the work;
  the paragraph sends a session to the issues no differently than today.
- `rejected@agent-skills@separate-bounded-problem-skill`: not brought back; no skill is added.
- `design@knowledge-architect@binary-bundles-workflow`: uses "installed workflow"; the terms section
  defines "installed text", which that head's subject is.

The entries the search graded `may-bear` that the grounding did not read, each with the searcher's
reason, for the design-conformance review:

- `design@agent-skills@knowledge-table-home`: not read. The searcher's reason: a glossary entry for "home", "plans", "spec" or "roadmap" and an outline of where records go overlap the primer's knowledge-table rows, which this head says ship in the primer and are refined only by project rows; no contradiction established until the glossary's text exists
- `design@agent-skills@verification-in-primer`: not read. The searcher's reason: the work's aim, a fresh session unsure what to search, meets the primer's "Check whether it is already known" (`cargo klarch issues`, `tripwires`, `show`) this head argues; an outline of where to search could restate or widen it; cites seed `design@agent-skills@primer-limit`
- `design@agent-skills@questions-go-to-their-authority`: not read. The searcher's reason: an outline of how a plain question or a bug report is handled could restate the primer's routing of a question by authority; a restated routing that differs would conflict with "The section prevails over an installed instruction"; no such text exists yet
- `design@agent-skills@owner-word-holds-as-far-as-its-premise`: not read. The searcher's reason: cites seed `design@agent-skills@primer-limit` as the content test that put this directive in the primer ("a word of the owner can arrive in any session"); the work applies the same test to its additions, with no element of the change meeting this rule
- `design@agent-skills@met-outside-the-task-outcomes`: not read. The searcher's reason: an outline of skill hand-offs for a bug investigation would describe routing to issue-tracking, which the primer's met-outside table already gives; a hand-off line that differs from "the first outcome that applies" would conflict; not established
- `design@agent-skills@plans-directory-in-primer`: not read. The searcher's reason: glossary entries for plans, spec and milestone would name the plans directory the primer already names per this head; a definition naming another path or a declared one would contradict it; not established
- `design@agent-skills@staged-check-before-each-commit`: not read. The searcher's reason: cites seed `design@agent-skills@primer-limit`; it also shows a primer addition argued as an exception to `design@agent-skills@behaviour-additions-need-real-use`, which the glossary and outline face too unless the owner's report of fresh sessions unsure what to search counts as the named lack
- `rejected@agent-skills@head-rules-one-fragment`: not read. The searcher's reason: cites seed `design@agent-skills@context-cost-of-agent-text`; one shape of the glossary, one fragment hosted by the primer and every skill using the terms, mirrors this rejected shape, refused for keeping in every session text no act there applies
- `issue@agent-skills@a-pointer-that-applies-a-whole-standard-without-asking-to-read-it-whole-was-not-searched`: not read. The searcher's reason: cites seed `design@agent-skills@context-cost-of-agent-text`; skills pointing to a primer glossary "as the glossary defines" would add pointers that apply a whole block with no instruction to read it, the class this issue's sweep looks for / it cites the seed `design@agent-skills@context-cost-of-agent-text`; the outline adds pointers from the primer to skills, and this issue's sweep judges pointers that apply a whole section; an outline's pointers ask for no section applied whole, so no bearing is established
- `issue@agent-skills@a-skill-for-creating-a-component`: not read. The searcher's reason: an outline of the installed skills and their hand-offs would place the creation of a Component; this issue records that "no installed skill says how a new Component is created" and that the owner has not converged on it, so an outline that presents setup as covering a later Component would state a hand-off that does not exist
- `issue@agent-skills@an-upgrade-re-reads-no-root-restatement`: not read. The searcher's reason: its third case, a session keeping its old copy of the primer after an install, and its first, root restatements unread after an upgrade, both apply to a primer glossary that projects restate or that changes across versions; no element of the work worsens it established / a primer glossary and outline are text a session that moves a pin keeps in its old copy (third case: "the session that moves the pin keeps its own copies of the primer ... from before the install"); a larger primer that restates skills' definitions widens what that copy can hold stale; the issue also records a delivered "routing table" row older than disk, the shape `design@agent-skills@no-routing-table` (a seed) removed; no relation established beyond this
- `issue@agent-skills@the-agentic-workflow-audit-s-repair-agents-read-the-admission-standard-by-a-pointer`: not read. The searcher's reason: cites seed `design@agent-skills@context-cost-of-agent-text`; no element of the glossary or outline meets the repair agents' pointer / cites the seed `design@agent-skills@context-cost-of-agent-text`; the issue is about saved-workflow agents reading the standard by a pointer, which no element of a primer glossary or outline meets / cites the seed `design@agent-skills@context-cost-of-agent-text`, which counts a pointer as a cost; the work weighs primer text against that seed, but the issue's subject, the saved workflow's general-purpose agents' prompts, is untouched by a primer glossary or outline
- `issue@agent-skills@the-material-finding-duty-has-no-head`: not read. The searcher's reason: it names the planning skill's vocabulary table row **material** as a restatement of the design skill's duty; a glossary centralizing coined terms would define "material" and relocate or duplicate that row / centralizing coined terms would move or restate "material", which the entry says the planning skill "restates ... in its vocabulary table, row **material**" from the design skill's duty; the work does not record the missing head, so no relation is established
- `issue@core@tooling-for-project-skills`: not read. The searcher's reason: cites seed `design@agent-skills@no-routing-table` (descriptions with MUST and symptom deliver a skill); no element of the work meets the prefix or description checks / cites the seed `design@agent-skills@no-routing-table` for "begin each skill's description with MUST and its symptom ... since that description is how the skill reaches a session"; an outline of skill hand-offs in the primer is a routing text that seed bears on, but this entry asks for checks of project skills, which the work does not change / cites the seed `design@agent-skills@no-routing-table`: "that description is how the skill reaches a session"; the outline maps installed skills, not a project's, so it meets no element of this issue
- `tripwire@agent-skills@rarer-rule-in-the-model`: not read. The searcher's reason: a glossary that defines checker terms in or beside `primer@checker-model` could carry a rule a finding message already delivers, its firing clause; and each commit changing PRIMER.md counts toward its horizon
- `tripwire@agent-skills@record-answered-questions-reach-the-owner`: not read. The searcher's reason: its premise is that routing a question by its authority is "stated in the primer every session reads"; an outline written for a session that starts with "a plain question" may restate or reshape where questions go in the primer, which would bear on that premise; the firing clause (two retrospectives finding record-answered questions put to the owner) is not met by the work
- `design@knowledge-architect@versioning-policy`: not read. The searcher's reason: a primer addition is a "skill change", a patch in the table; the branch building it classes it under this head, with no conflict established
- `design@knowledge-architect@changelog-entries`: not read. The searcher's reason: whether a glossary and an outline owe a Workflow entry turns on "a change ... a person watching agent sessions would observe"; "A rewording or a clarification is not one" may exclude it; not settled
- `design@agent-skills@tripwire-admission-test`: not read. The searcher's reason: a glossary entry defining "tripwire" would restate part of the fragment `tripwire-standard.md`, which is "one text"; a definition narrower or wider than its four conditions would conflict; not established
- `issue@agent-skills@a-tripwire-on-agent-behaviour-fires-where-this-project-cannot-see`: not read. The searcher's reason: the work's premise, that fresh sessions in adopting projects search better with a glossary, is agent behaviour in consumer sessions; a tripwire from its premortem would meet this issue's gap, though condition 2 of the admission test now excludes a transcript-only tripwire / a premortem of the primer addition could name a cause on how fresh sessions behave; the issue says "a tripwire whose evidence lives only in a session's transcript is no longer written"; whether the discussion records such a tripwire is not established / the work's success is a change in how fresh sessions behave; a premortem cause or acceptance criterion on that behaviour would have evidence only in session transcripts, the class this issue describes; whether the design writes one is not known / a premortem of the primer addition could propose a tripwire on whether fresh sessions now find what to search; the issue states such a tripwire's evidence "happens mostly in the sessions of consumer projects" and is read nowhere; no tripwire is proposed yet, so no bearing established
- `design@agent-skills@audit-is-an-activity`: not read. The searcher's reason: the outline of how the installed skills hand off would describe the project-audit skill beside the review skill; the head says a review "stays the reading of a diff" and an audit reads the whole record on one axis, so a one-line outline that merges them, or presents an audit as a kind of review, would contradict it; no such wording is proposed yet
- `design@agent-skills@audit-method`: not read. The searcher's reason: the glossary defines "every term the workflow coins"; this head coins the audit's "owner list" labelled `F<n>`, which a glossary entry would restate and could narrow; whether audit vocabulary is in the glossary's scope is not settled
- `design@agent-skills@audit-passes-repeat-while-the-count-falls`: not read. The searcher's reason: it defines "a violation left" (a re-check finding a reading confirms, on no kept list, recorded by no issue); a glossary that defines that term or "kept list" restates this definition and could drift from it; scope of the glossary over audit terms not settled
- `design@agent-skills@convergence-count-scope`: not read. The searcher's reason: it defines "old text" and "new text" ("a move, a reflow or a rename leaves text old"); a glossary entry for these coined terms would restate a definition that the primer cannot hold more than one sentence of, per the restatement rule; scope not settled
- `design@agent-skills@one-run-pass`: not read. The searcher's reason: it defines a "pass" of an axis as one run of its saved workflow; a glossary defining "pass" or "axis" restates it; scope not settled
- `design@agent-skills@audit-axis-runs-a-workflow`: not read. The searcher's reason: the work defines "agent workflow / installed workflow"; this head uses "saved workflow" for a harness file of .claude/workflows/, a third sense of "workflow"; a glossary that sets one meaning per word must place this sense or collide with it; it also states a context cost ("each saved workflow is listed in every session ... one that never audits included"), the same cost a primer addition has in every session
- `design@core@tripwire-guards-no-rejected-entry`: not read. The searcher's reason: the glossary defines "tripwire"; this head states "A tripwire guards a decision: a `rejected` entry records a design not kept, which is no decision"; a glossary definition of tripwire as guarding any entry would contradict it; no wording proposed
- `design@core@home-at-the-carrier-path`: not read. The searcher's reason: the glossary defines "home" and "location"; this head makes a location declared with `dir = "."` its register's home, so a glossary definition of "home" as a path under an anchor's `docs/` would be false for that case
- `design@core@instance-options-beside-the-instance`: not read. The searcher's reason: "register instance" is a coined term the installed routing reviewer uses ("one register instance"), which a systematic glossary would have to define; this head is where an instance's meaning shows (per-anchor options in register.toml); whether the glossary takes this term is not settled
- `design@core@groups-are-subdirectories`: not read. The searcher's reason: "group" is a coined term of the issue register a systematic glossary would define; this head fixes it as a declared subdirectory that is not part of an entry's id; whether the glossary covers it is not settled
- `design@core@issue-kind-list-closed`: not read. The searcher's reason: the work lists "kind" among the terms to define, meaning a reference's kind (a register's name); this head uses "kind" for an issue's kind (`todo`, `defect`, ...), a second meaning of the same word; a glossary aiming at one meaning per word, as `design@agent-skills@document-vocabulary` (a seed) does for plan words, must split or place both
- `design@core@entry-anchor-carries`: not read. The searcher's reason: a glossary definition of "home" must hold where an entry's README is both the entry's document and the head of the carried home; no definition is proposed yet
- `design@core@entry-anchor-path-kind`: not read. The searcher's reason: a glossary definition of the reference "kind" `path` must hold for an entry anchor, where a file other than the README is named only by `path@<entry>@<file>`; no wording proposed
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: not read. The searcher's reason: the work aims at a fresh session finding the record before it re-derives; the goal's "a later session can then tell what it may change ... without re-deriving the argument" may be served by the outline, but the work only adds orientation text and records no decision / the outline's aim, a session finding where decisions live, might serve 'A later session can then tell what it may change ... without re-deriving the argument'; the work changes how the record is reached, not how it is recorded, so no bearing is established
- `design@agent-skills@extension-follows-the-ground`: not read. The searcher's reason: the glossary might be recorded as a new member of a seed head, such as `design@agent-skills@document-vocabulary` or `design@agent-skills@primer-limit`; this head routes a member by whether the head's argument covers it, and whether those seeds' arguments cover a primer glossary is not established from this cluster
- `tripwire@agent-skills@deferred-trigger-met-by-undesigned-work`: not read. The searcher's reason: an outline that tells a session handling a bug report or a plain question where to search would add a search before work that goes through neither the design skill nor an intake, which the guarded decision `design@agent-skills@record-searched-before-the-work` chose not to send and which this tripwire's response proposes; the work records no second instance, so its firing clause is not met / it watches work that bypasses the design skill, the population the work targets (a bug report investigated outside the design path), and its Response is to 'propose a search before such work, with its host'; the outline approaches that response, but the firing clause needs a second recorded instance of a deferred trigger found only at review, which the discussion does not supply
- `tripwire@agent-skills@head-created-without-deliberation`: not read. The searcher's reason: its premise is that "the backstop, which the primer hosts", reaches a session before it writes a head; a primer edit that moves or shortens text to fit a glossary and an outline under `design@agent-skills@primer-limit` could move the backstop, but no such move is in the work as briefed
- `rejected@core@session-start-hook-primer`: not read. The searcher's reason: if the work proposes delivering the glossary or the outline to a fresh session by a channel other than the primer's import, such as a hook, it brings back this design; the entry's remaining doubt, "another context or harness might not follow it", bears on whether the primer reaches the sessions the work targets; the work as briefed puts both in the primer
- `issue@core@configuration-for-several-agent-providers`: not read. The searcher's reason: the glossary defines "agent configuration"; this entry's open work gives the agent configuration other providers' files and layouts, so a definition tied to CLAUDE.md and `.claude/` would need changing then; no conflict with the work itself is established / the glossary defines 'agent configuration'; a definition naming CLAUDE.md and .claude/ would have to change when the issue adds harnesses ('Projects often keep their agent instructions in a generic AGENTS.md'); no bearing is established until the definition's wording is chosen
- `design@core@nothing-of-a-project-is-compiled-in`: not read. The searcher's reason: cites the seed `design@agent-skills@primer-limit` for "the root CLAUDE.md is its title and the primer's import line, since what every root would say alike is the primer's"; it also states "What the word _component_ means", which a glossary entry for Component would restate; the work adds to the primer and changes neither the init text nor the compiled-in meaning, so no conflict is established
- `issue@core@a-reference-with-a-line-or-fragment-suffix-names-a-wrong-repair`: not read. The searcher's reason: it records the owner's ruling that rarer checker rules are delivered by finding messages "rather than by the installed text"; a glossary meant to be "systematic so no coined term stays undefined" could move checker rules into the primer against that ruling; not established that the glossary states rules
- `design@agent-skills@a-head-is-owed-by-an-entry-test`: not read. The searcher's reason: the workflow outline would describe when a design decision is recorded, and the discussion's own outcome, a decision about installed text, is judged by these tests ("for agent-facing text, each text that states the instruction is one site"); a glossary entry for "head" or "entry test" restating them would be a second site; no conflict established until the glossary's wording exists
- `design@agent-skills@losing-alternatives-filter`: not read. The searcher's reason: a glossary entry for a `rejected` entry would state what earns one; the head says "A design not kept earns an entry ... only if it passes the tests of the installed decision-recording skill", and a definition reading "a design not kept" alone would contradict it; not established
- `design@agent-skills@losing-shape-test`: not read. The searcher's reason: "thread" is a listed term; the head names thread states ("ruled-out", "withdrawn", "superseded") and which reach the `rejected` home; a glossary definition of a thread and its states must agree; no conflict established
- `design@agent-skills@rejected-citation-direction`: not read. The searcher's reason: a glossary definition of a `rejected` entry would state its link to heads; the head says "A `rejected` entry names no winner" and the head cites the entry; a definition such as "the design a decision lost to" could bring back the framing "the owner judged ... wrong"; not established
- `design@agent-skills@subject-split-standard`: not read. The searcher's reason: a systematic glossary would define "subject" and "cluster", coined here ("a level-two subject holds the heads a piece of work reads together"), whose standard is a shared fragment of named hosts; a primer definition is a further site that could diverge; not established
- `design@agent-skills@thread-slug-is-entry-id`: not read. The searcher's reason: "thread" is a listed term; the head defines how a thread is named ("a slug minted in the entry-id grammar", written plain with a hash sign) and its item form in the plan document; a glossary definition must agree; no conflict established
- `issue@agent-skills@the-void-marker-s-reason-reads-two-ways`: not read. The searcher's reason: the `live` and `void` markers are coined terms a systematic glossary would define; the entry asks which act the marker prevents, and a definition stating its purpose would pick one reading; not established that the definition goes that far
- `issue@core@a-staged-check-compares-installed-files-with-unstaged-shipped-text`: not read. The searcher's reason: building the work edits the primer's source under content/ and its installed copy; the entry reports `check --staged` failing when "another edited skill and its copy stay unstaged"; it bears only if the branch commits several edited installed texts one at a time, not established
- `design@core@backticked-bare-id`: not read. The searcher's reason: the glossary lists coined terms, likely in backticks; "A lint of the last phase reports such a span when the entity table defines its id", a section slug or skill name included, so a hyphenated term (such as "in-change path" or "re-entry point") that equals a defined id would be a finding; whether any coined term equals an id is not checked
- `design@core@plain-word-lint`: not read. The searcher's reason: the glossary writes hyphenated coined terms as plain words; "A plain word that is exactly a defined id is a finding", and "such a slug met there as an ordinary word is reported upstream", so a term that equals a slug of the installed text clashes with this head; no term's clash is established
- `issue@core@a-plain-hash-item-name-is-read-by-nothing`: not read. The searcher's reason: a glossary entry for "thread" or a plan item may state how an item is cited; the issue's "The planning skill prescribes a plain `#<id>` to name a plan item from outside its plan" is the form nothing reads; a definition that restates it would add a site, not established / a glossary entry for "thread" may say how an item is named outside its plan, restating the plain `#<id>` form this issue finds unchecked and asks a design discussion to settle; not established that the entry says so
- `design@agent-skills@premortem-as-watch-points`: not read. The searcher's reason: cites the seed `design@agent-skills@no-routing-table`, and its standing question "whether the session ... acted as if a rule of the primer were absent" is the channel that would carry evidence of the gap the work names; the outline changes none of the questions, so no bearing is established
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: not read. The searcher's reason: 'systematic so no coined term stays undefined' might become a task the workflow asks of a session (checking each new term against the glossary), which the goal says the checker does where it needs no judgement; whether a term is coined needs judgement, so no bearing is established / if the outline gives a fresh session a several-step search of the record (issues, tripwires, plans, design homes), that search is the kind of mechanical task the goal gives the checker as a command; not established that the search follows from the tree with no judgement / "what to search and where" for a fresh session may be a task whose result follows from the tree, which this goal gives to a checker command rather than to text; a rival shape to the outline, not established that the search is mechanical
- `design@agent-skills@milestone-is-a-directory`: not read. The searcher's reason: a glossary definition of milestone and slice could state their shape ("its head a README, each slice a spec", cited `spec@<milestone>@<slice>`); a definition that differs would contradict it, but no definition is drafted yet
- `design@agent-skills@plan-document-leaves-at-landing`: not read. The searcher's reason: the outline of hand-offs (planning, review, landing) could state when a plan document leaves; the head fixes it "in a commit of its own after the review before the merge"; whether the outline says anything on leaving is not decided
- `design@agent-skills@ledger-from-transcript`: not read. The searcher's reason: the brief names "ledger" as a term the glossary defines; this head and its rejected rival use the word for the discussion's record assembled from the transcript; a definition of ledger as a file the agent keeps would conflict, with no wording yet to judge
- `design@agent-skills@acceptance-criteria-in-the-document`: not read. The searcher's reason: acceptance criterion is a coined term of the plan documents the glossary would define; the head defines it as "a check on a recorded decision that only the work's built code can apply", which a glossary definition must not contradict
- `design@agent-skills@plan-reviewed-as-a-commit`: not read. The searcher's reason: the outline's planning-to-review hand-off could state when the reviews run; the head requires the plan document committed before its reviews; no bearing until the outline's wording exists
- `design@agent-skills@plan-landing-is-not-tied-to-its-work`: not read. The searcher's reason: a glossary definition of spec and milestone could tie the document's landing to its work; the head says "What 'spec' and 'milestone' measure is the work ... not where the document lands"
- `design@agent-skills@plan-records-the-exchange`: not read. The searcher's reason: the glossary defines thread; this head says what a plan document records of each thread (proposer, round, final state, arguments, rulings); a definition of thread could narrow it, and no definition is drafted
- `design@agent-skills@arguments-as-items`: not read. The searcher's reason: the glossary would define the plan items (thread, argument, criterion, acceptance); this head makes an argument an item "without a state", which a definition could contradict
- `rejected@agent-skills@ledger-written-during-discussion`: not read. The searcher's reason: "ledger" is a term the glossary defines; a definition presenting the ledger as a file kept during the discussion would restate this rejected design; no definition is drafted yet
- `design@agent-skills@a-reference-claims-a-revisit`: not read. The searcher's reason: a glossary entry for "reference" (or for kind and anchor) could restate the head's definition, "a reference is therefore a claim of dependence"; the head also lists restatements under `design@agent-skills@restatement-size-test`, which a glossary of definitions is; no wording yet
- `rejected@core@component-register-list-fixed`: not read. The searcher's reason: a glossary definition of Component as carrying the component registers and nothing else would restate this rejected design ("a Component's register list fixed by the word _component_"); no definition is drafted yet
- `design@agent-skills@no-size-budget`: not read. The searcher's reason: the glossary and outline add length to the primer; the head says 'An installed skill's text carries no size budget', which is scoped to a skill, while the primer's limit is the seed `design@agent-skills@primer-limit`, so no bearing is established for the primer
- `rejected@agent-skills@admission-standard-in-primer`: not read. The searcher's reason: cites the seed `design@agent-skills@primer-limit`: it lost because writing agent-facing text is not every session's work and a skill trigger delivers it; the glossary is not the admission standard so no bring-back, but the same argument applies to each glossary term only some sessions use
- `issue@agent-skills@whether-a-census-of-real-text-under-a-planned-instruction-is-real-use`: not read. The searcher's reason: making the glossary 'systematic so no coined term stays undefined' might be done by a census of the installed text with subagents, the case the issue leaves open ('Which is it?'); a census by grep would be mechanical and outside it, so no bearing is established until the discussion picks a method
- `tripwire@agent-skills@agent-text-written-without-the-standard`: not read. The searcher's reason: the work adds to the primer, and the tripwire's Response reopens `design@agent-skills@admission-rules-home` on 'a one-sentence pointer in the primer to the standard'; its firing clause, an unneeded addition made in a session that did not load the agent-configuration skill, is not met by a design discussion that loads it
- `design@agent-skills@one-record-searcher`: not read. The searcher's reason: an outline of how the design skill grounds its work would describe the record searcher's dispatch, which the head places in the agent's description ('a dispatcher reads an agent's description and never its body'); a restatement in the primer would be a second site, but no bearing is established unless the outline names the dispatch
- `design@core@every-path-names-its-anchor`: not read. The searcher's reason: a glossary entry for "anchor" or "path reference" would restate "the deepest anchor wins" and the `path@<anchor>@<path>` grammar; no bearing unless the glossary defines reference forms, which the brief's term list does not name
- `design@core@reserved-anchors`: not read. The searcher's reason: a glossary defining "anchor" may cover the words `elsewhere` and `*`; the head says each "is no anchor" and names what kinds each serves, so a definition calling them anchors would contradict it; not established that the glossary covers them
- `design@core@path-to-a-home-is-reported`: not read. The searcher's reason: a glossary definition of "home" would have to match this head's list of home targets (a heading register's file or directory, a file register's directory; an entry's directory is no home); not established that the glossary goes to that detail
- `design@core@planned-path-form`: not read. The searcher's reason: a glossary entry for "kind" may list `planned`; this head limits it to the plans directory, which a definition must not widen; not established that the glossary lists kinds beyond registers
- `design@core@links-are-navigation-rows`: not read. The searcher's reason: a glossary or outline written with relative markdown links from each term to its home would be reported, since the head admits links in README.md and index.md only and PRIMER.md is neither; not established that the design uses links
- `rejected@core@component-placeholder-anchor`: not read. The searcher's reason: the glossary's definition of the anchor `*` could propose `<component>` as the clearer spelling for a reader, which this entry rejects while keeping the owner's doubt that `*` is "a bit confusing"; no proposal of it is established
- `rejected@core@home-kind-word`: not read. The searcher's reason: a glossary entry for "home" that calls the register kind's reference a home reference would revive the word this entry rejects for the kind; not established
- `design@core@two-segment-ids`: not read. The searcher's reason: a glossary entry for "id" or "slug" would restate `[a-z0-9]+(-[a-z0-9]+)+`; not established that those terms are in the glossary
- `design@core@a-slug-elsewhere-defines-nothing`: not read. The searcher's reason: if each glossary term is given a citable slug, a slug in a table cell or at a level-3 heading of the primer "defines nothing and is reported as a misplaced definition"; the glossary's shape is not decided
- `design@agent-skills@content-mirrors-the-install-layout`: not read. The searcher's reason: if the glossary or outline ships as its own file rather than inside PRIMER.md, "a file the layout does not map fails the build", so the layout would change; the placement is not decided
- `design@agent-skills@naming-rule`: not read. The searcher's reason: the brief's glossary distinguishes "agent configuration" from "installed workflow"; this head notes that agent-configuration is the one skill "named like the configuration it edits", so a glossary definition of the artifact meets the skill's name; no conflict established
- `design@core@plans-split-dirs`: not read. The searcher's reason: a glossary definition of "plans" may describe the directory's content; this head fixes it as a README and two homes, specs/ and milestones/, "and nothing else"; not established that the definition goes to that detail
- `design@core@spec-file-is-an-anchor`: not read. The searcher's reason: a glossary definition of "anchor" must include each spec file and each milestone directory as anchors; not established that the definition lists the constructed anchors
- `design@core@plan-document-sections-by-kind`: not read. The searcher's reason: it says "the two lists are the planning skill's, which owns the shape of a plan document"; a glossary entry for spec or slice that describes a plan's sections would be a second statement of that shape
- `design@agent-skills@standing-state-review-before-every-merge`: not read. The searcher's reason: the outline of how the skills hand off would describe when the record is read; the head states "the search before the work runs only for work that goes through the design skill, a plan or a retrospective intake", and the standing-state review is the one reader for every change; an outline written for a session outside the design path could restate or misstate that split, no bearing established until the outline's text exists
- `design@agent-skills@ruled-items-labelled`: not read. The searcher's reason: a glossary of every coined term could define labels and their prefixes (`Q`, `D`, `W`, `AC`, `F`); the head says "Every distinct prefix is listed here" and the numbering "is stated once, in `primer@labelled-items`"; a glossary row restating them would be a second list, not established that the glossary includes them
- `design@agent-skills@standing-argument-in-head`: not read. The searcher's reason: a glossary entry for "head", "standing argument" or "deliberation" would define what this head decides a head carries versus what history carries; whether the glossary defines those terms is not settled
- `design@agent-skills@owner-rules-at-a-fork`: not read. The searcher's reason: "thread" is in the work's term list; this head states how a thread closes ("argued", "ruled-out") and what a fork is; a glossary definition of thread or fork could restate or diverge from `skill@knowledge-architect-design@threads-and-states` and the primer's fork sentence; no definition written yet
- `design@agent-skills@derived-decision-proceeds-on-default`: not read. The searcher's reason: the outline for a session starting from a bug report would likely describe the bounded path; the head states "The bounded path's one message names its default, which the session builds"; an outline that describes that hand-off could misstate it, not established
- `issue@agent-skills@the-planning-skill-answers-every-material-finding-with-a-default`: not read. The searcher's reason: a centralized glossary would take over or restate the skills' vocabulary rows; this issue's subject includes the planning skill's vocabulary row **material**, and its close has that row "stop restating" the design skill; if the glossary centralizes vocabulary rows, it touches that row, not established
- `issue@agent-skills@the-primer-s-divergence-paragraph-reads-two-ways-on-work-that-goes-on`: not read. The searcher's reason: the work edits the primer, and its outline addresses a session investigating a bug report, which can meet a divergence between a head and the code; an outline sentence on that case would restate the ambiguous `primer@intent-and-claims` bullet; the work does not edit that bullet as described
- `design@core@record-is-every-register-entry`: not read. The searcher's reason: a glossary of coined terms could define "the record"; this head defines it as every entry of every register but the plan registers, agent-facing text excluded; a glossary definition differing from it would contradict it; not established that the glossary defines the term
- `design@core@cluster-rooted-at-a-subject`: not read. The searcher's reason: "cluster", "unit" and "subject" are coined terms of the record search; a glossary that defines "every term the workflow coins" could define them, and this head fixes their meaning; whether the glossary reaches the record searcher's terms is not settled
- `design@agent-skills@workflow-axis-corpus`: not read. The searcher's reason: the primer is in the corpus, and the head "rests on one agent being able to read it whole" (measured by `wc -w` over CLAUDE.md and .claude); the glossary and outline grow that corpus, though no figure shows the growth reaching the limit; also the glossary's definition of "agent configuration" meets the head's corpus of "the project's agent configuration directory", which excludes scoped CLAUDE.md files, with no conflict established until the definition is written
- `design@agent-skills@workflow-axis-lenses`: not read. The searcher's reason: an outline of how the skills hand off restates the skills' moments, and L5 "reads each restatement of a directive against its home"; whether the outline is a restatement or pointers is not yet decided, so no bearing is established
- `design@agent-skills@workflow-axis-finding-standard`: not read. The searcher's reason: the work is argued from a fresh session being "unsure what to search and where"; the head says "a gap that only predicts what an agent would do leads to no edit", deriving from `design@agent-skills@behaviour-additions-need-real-use`; the head's scope is the audit axis, not a design discussion, so it is a lead to that derived head rather than a bearing
- `design@agent-skills@audit-net-growth`: not read. The searcher's reason: the outline's hand-off sentences may add instructions to the primer; the head sends to the owner "a set of repairs that adds more instructions than it removes", but its scope is an audit's repairs, and a glossary of definitions adds no instruction by its definition of one, so no bearing is established
- `rejected@agent-skills@workflow-corpus-shared-out`: not read. The searcher's reason: it is the candidate "when an auditor's return says it did not read every file whole", since "the corpus grows with every skill and every repair"; the glossary and outline grow the primer, which every auditor reads, but no figure shows the corpus outgrowing one agent's reading
- `design@core@grammars-not-prefixes`: not read. The searcher's reason: a glossary meant to define every coined term may define "prose"; this head defines what bytes are prose for the checker ("A Rust file is prose only where its grammar says so"), while `primer@working-prose` uses the word for the text contributors read; no glossary entry is drafted yet, so no conflict is established
- `design@core@installed-binary-version-check`: not read. The searcher's reason: the primer speaks of a project moving "its pin of the checker", a coined term the glossary would define; this head defines the pin as `[project] checker-version`, the exact version every dispatching binary refuses to run against; no definition is drafted, so no conflict is established

## Criteria

### A session outside the design path finds what the record already answers about the code it changes `##off-path-finds-the-record`

Binding. Derived in the reply to round 1 from `goal@knowledge-architect@agents-work-without-drift`.
Met by `thread@terms-and-fix-grounding@off-path-grounding` and
`thread@terms-and-fix-grounding@term-defined-at-first-load`.

### No fixed sequence or mapping where judgement serves `##no-fixed-sequence`

Binding. Derived from `goal@agent-skills@installed-text-leaves-room-to-judge`. Met: the outline of
`thread@terms-and-fix-grounding@readme-workflow-outline` is in a README, not in agent-facing text.

### No two installed texts define a term differently `##one-definition-per-term`

Binding. Derived from `goal@agent-skills@no-contradiction-in-workflow`. Met by
`thread@terms-and-fix-grounding@term-defined-at-first-load`, checked by
`acceptance@terms-and-fix-grounding@terms-entries-restate-nothing-new`.

### The primer holds only what every session needs and no skill delivers `##primer-holds-what-every-session-needs`

Binding as a presumption. Derived from `design@agent-skills@primer-limit`. Met: the paragraph
applies to every session that changes code, and no skill delivers it outside the design path; the
terms section holds only terms the primer uses and words with two senses, per D1.

### An instruction about how an agent behaves has real-use evidence and a mechanism `##behaviour-needs-real-use`

Binding as a presumption. Derived from `design@agent-skills@behaviour-additions-need-real-use`. Met
by the owner's instances in thaum and rootpoint, `argument@terms-and-fix-grounding@arg-6`, and the
mechanism of `argument@terms-and-fix-grounding@arg-8`. The terms section is no behaviour addition:
a term used and not defined is a defect provable by reading.

### The installed text names no project's paths and cites no entry `##no-project-paths`

Binding as a presumption. Derived from `design@agent-skills@shipped-text-cites-no-entry` and
`goal@agent-skills@installed-text-works-anywhere`. Met: both drafts cite only primer and skill
sections.

### Words loaded by sessions whose work does not use them `##context-cost-weighed`

Weighed. Derived from `design@agent-skills@context-cost-of-agent-text`. Met: about 80 words for the
paragraph, which most sessions that change code apply, and the terms section, bounded by D1 and
checked by `acceptance@terms-and-fix-grounding@terms-census-holds`.

## Threads

### A coined term is defined once, and the primer's terms section defines each one the primer uses `##term-defined-at-first-load`

Proposed by the agent in the reply to round 1, refining the owner's proposal of round 1. Approved.
Arguments: `argument@terms-and-fix-grounding@arg-1`, `argument@terms-and-fix-grounding@arg-2`,
`argument@terms-and-fix-grounding@arg-11`. Shape: Decided design, "The terms section and the rule".
Harvest: a design head of `register@agent-skills@design`, and a member added to
`design@agent-skills@primer-limit`. The owner's words, round 2: "I think term-defined-at-first-load
is a good refinement to my initial proposal"; round 3: "Agreed on this table with AC1 and AC2". The
census of step 1 narrowed which terms the section holds, D1, awaiting the owner.

### A central glossary in the primer of every term the workflow coins `##primer-glossary`

Proposed by the owner, round 1. Superseded by `thread@terms-and-fix-grounding@term-defined-at-first-load`,
which the owner approved by name, round 2. Arguments: `argument@terms-and-fix-grounding@arg-1`,
`argument@terms-and-fix-grounding@arg-2`. The owner's words, round 1: "I was thinking about adding a
glossary in the primer about the terms knowledge-architect uses." / "The practice would certainly
be better if it was centralized and systematic, so that no term coined by ourselves stays
undefined." Harvest: judged as a losing alternative.

### The words with two senses, "workflow" and "kind", are defined apart `##workflow-word-senses`

Proposed by the agent in the reply to round 1. Superseded by
`thread@terms-and-fix-grounding@term-defined-at-first-load`, which carries it. Argument:
`argument@terms-and-fix-grounding@arg-2`. Harvest: none of its own.

### An outline in the primer of how the skills hand off to each other `##workflow-outline`

Proposed by the owner, round 1: "Maybe this should also come with a rough outline of the full
workflow:  a mapping of how the main skills interract and hand off to each other." Withdrawn by the
owner, round 2: "as you said, I am now realizing that the trigger instructions are already supposed
to cover this." Arguments: `argument@terms-and-fix-grounding@arg-3`,
`argument@terms-and-fix-grounding@arg-4`, `argument@terms-and-fix-grounding@arg-7`. Harvest: judged
as a losing alternative.

### A diagram of the activities and their hand-offs in the agent-skills README, for human readers `##readme-workflow-outline`

Proposed by the agent in the reply to round 1, as the rival of
`thread@terms-and-fix-grounding@workflow-outline`. The owner's words, round 2: "But this
information is lacking in human facing docs. Adding it to a readme would not hurt."; held
`presumed-settled` in the reply to round 2, and approved with the table, round 3: "Agreed on this
table". Argument: `argument@terms-and-fix-grounding@arg-7`. Shape: Decided design, "The outline".
Harvest: none; it earns no head, as the Harvest says.

### Before proposing a fix or any other change to code, a session reads the record of that code `##off-path-grounding`

Proposed by the agent in the reply to round 1; its shape in the reply to round 2. Closed `argued` in
the reply to round 2, and approved with the table, round 3. Arguments:
`argument@terms-and-fix-grounding@arg-4`, `argument@terms-and-fix-grounding@arg-5`,
`argument@terms-and-fix-grounding@arg-6`, `argument@terms-and-fix-grounding@arg-8`,
`argument@terms-and-fix-grounding@arg-9`. The argument that closed it,
`argument@terms-and-fix-grounding@arg-9`: of the rivals, the same sentence in
`skill@knowledge-architect-issue-tracking@read-before-diagnosing` is delivered at the diagnosis and
reaches no change that is not a bug, and the design skill's grounding costs the record search,
outside the owner's "minimal fix". Shape: Decided design, "The paragraph". Harvest: a design head of
`register@agent-skills@design`, and a member added to `design@agent-skills@primer-limit`.

### The knowledge table also answers where to look `##knowledge-table-lookup`

Proposed by the agent in the reply to round 1, as shape (a) of
`thread@terms-and-fix-grounding@off-path-grounding`. Withdrawn by the agent in the reply to round 2:
it names no moment and no mechanical route, and the paragraph carries its useful part. Argument:
`argument@terms-and-fix-grounding@arg-10`. Harvest: none.

## Arguments

### Terms one skill uses cost every other session, and a second definition drifts `##arg-1`

The reply to round 1, the agent. Bears on `thread@terms-and-fix-grounding@primer-glossary` and
`thread@terms-and-fix-grounding@term-defined-at-first-load`. "Centralized, so no coined term stays
undefined" would bring in terms only one skill uses: ledger, delta, bucket, owner list, pass, lens,
cluster, violation left. "A session using those terms has already loaded the skill that defines
them. In the primer they cost every other session the words, and they add a second definition that
can drift." `design@agent-skills@one-home-for-head-rules` made the same split.

### The primer uses coined terms it does not define, and two words have two senses `##arg-2`

The reply to round 1, the agent. Bears on `thread@terms-and-fix-grounding@term-defined-at-first-load`,
`thread@terms-and-fix-grounding@primer-glossary` and `thread@terms-and-fix-grounding@workflow-word-senses`.
The primer uses "slice", "bounded work", "full path" and "installed text" without defining them;
"workflow" names the method, the installed text and a saved workflow, and "kind" names a
reference's kind and an issue's. "These are defects you can prove by reading", which need no
real-use evidence; a definition passes the need test when an instruction uses the term.

### An outline restates edges whose home is the skills `##arg-3`

The reply to round 1, the agent. Bears on `thread@terms-and-fix-grounding@workflow-outline`. "Each
hand-off is a directive owned by a skill"; `design@agent-skills@root-cause-repair` names "two homes
for one moment" as a cause of findings; the argument that removed
`rejected@agent-skills@root-routing-table` applies: the harness lists every skill's description,
and the primer's `primer@installed-skills` gives each skill's triggers.

### The outline does not reach the symptom `##arg-4`

The reply to round 1, the agent. Bears on `thread@terms-and-fix-grounding@workflow-outline` and
`thread@terms-and-fix-grounding@off-path-grounding`. "A session holding a bug report needs to know
where the answer lives. The edges between design, planning and review don't tell it that."

### A design-path session is told what to read, an off-path one is not `##arg-5`

The reply to round 1, the agent. Bears on `thread@terms-and-fix-grounding@off-path-grounding`. The
design skill's loop step 1 names the goals, design and rejected homes, the READMEs and the record
search; an off-path session has only "Check whether it is already known", whose `issues`,
`tripwires` and `show` "read an entry you already know about". The searcher's description scopes it
to the design discussion, the intake and the design audit.

### The owner's instances: bug investigations do not look for known design `##arg-6`

Round 2, the owner. Bears on `thread@terms-and-fix-grounding@off-path-grounding`. "from memory,I
faced this mostly during bug investigation, in thaum and in rootpoint" / "The exact problem is that
a bug investigation session is not really informed of where to look for known design (it looks for
issues, but skims them mostly). It matters little for investigation, but it starts mattering for
proposing a fix." / "For now, I'd be happy with a minimal fix that helps a fresh session not going
through the workflow find informations relatively reliably."

### The triggers route between skills; the human-facing docs lack the map `##arg-7`

Round 2, the owner. Bears on `thread@terms-and-fix-grounding@workflow-outline` and
`thread@terms-and-fix-grounding@readme-workflow-outline`. "I am now realizing that the trigger
instructions are already supposed to cover this. But this information is lacking in human facing
docs. Adding it to a readme would not hurt. I have never had a wrong or missing skill load, except
sometimes during long session".

### The mechanism, checked against the text `##arg-8`

The reply to round 2, the agent. Bears on `thread@terms-and-fix-grounding@off-path-grounding`. A bug
session meets two texts that tell it what to read, `primer@verify-before-relying` "Check whether it
is already known" and `skill@knowledge-architect-issue-tracking@read-before-diagnosing`, and neither
names a design home. "the only grounding texts a session outside the design skill meets name the
issue and tripwire listings, and nothing it loads sends it to the design record before it proposes
a fix."

### The rivals of the paragraph lose on their moment and on the owner's scope `##arg-9`

The reply to round 2, the agent. Bears on `thread@terms-and-fix-grounding@off-path-grounding`. The
same sentence in `skill@knowledge-architect-issue-tracking@read-before-diagnosing` loads at the
diagnosis of a "new problem", never reaches a change that is not a bug, and governs the diagnosis,
not the fix. Sending fixes through the design skill's grounding dispatches the record search: this
session's took 10 agents and about 1.07M subagent tokens, summed from the 10 agents' completion
reports, outside the "minimal" scope the owner set and inside the development-work goal the owner
will add.

### The knowledge table names no moment `##arg-10`

The reply to round 2, the agent. Bears on `thread@terms-and-fix-grounding@knowledge-table-lookup`.
"A generic 'the table also says where to look' names no moment and no mechanical route, and the
paragraph above carries its useful part."

### The census admits more terms than a two-text count can bound `##arg-11`

The assembly, the agent, after the convergence. Bears on
`thread@terms-and-fix-grounding@term-defined-at-first-load`. A census of the 27 installed copies, the
primer, 11 skills, 12 agents and 3 saved workflows, finds more than 30 coined terms used by two or more
of them, among them "owner list" in 10, "brief" in 20 and "axis" in 23; the reply to round 2
estimated 17 members. A section of every such term grows past the size the reply to round 2 put to
the owner, which cause P4 names. The rule that bounds it, D1: the section holds each coined term the
primer uses and each word with two senses, and any other text points to a term's home at its first
use.

## New names, in one place

```text
crates/agent-skills/content/PRIMER.md
  ## Terms the workflow coins {{slug:coined-terms}}     new level-two section, last in the file;
                                                        cited by the primer kind with the slug coined-terms
  the paragraph of the Decided design, "The paragraph"  new, in `primer@verify-before-relying`,
                                                        after "Check whether it is already known"
crates/agent-skills/fragments/admission-need-test.md
  the bullet "**A coined term is defined once**"         new, after the bullet on a definition
crates/agent-skills/README.md
  ## How the skills hand off to each other               new section, after "Extending an installed skill"
```

## Decided design

### The terms section and the rule

A level-two section, last in the primer, titled "Terms the workflow coins", with the slug
`coined-terms`. One bullet per term: the term in bold, one sentence that defines it, and a pointer
to its home where the home is another text. Its members are each coined term the primer uses and
does not define elsewhere in itself, and each word the workflow uses in two senses, per D1. The
census of step 1 fixes the list; the expected members, from the reading of "What the work is":

| term | home the pointer names |
| --- | --- |
| installed text | `skill@knowledge-architect-agent-configuration@installed-files-never-edited` |
| agent configuration | `skill@knowledge-architect-agent-configuration` |
| agent-facing text | `skill@knowledge-architect-agent-configuration@admission-standard` |
| workflow, saved workflow | none: the sentence separates the senses |
| kind | `primer@checker-model` for a reference's kind; `skill@knowledge-architect-issue-tracking@issue-entry` for an issue's |
| the owner | `primer@question-authority` |
| spec, milestone, slice, step | `skill@knowledge-architect-planning@planning-terms` |
| bounded work | `skill@knowledge-architect-design@bounded-work` |
| in-change path, full path | `skill@knowledge-architect-design@decision-authority` |
| thread | `skill@knowledge-architect-design@threads-and-states` |
| premortem | `skill@knowledge-architect-design@discussion-loop` |
| pin | `skill@knowledge-architect-setup` |
| axis | `skill@knowledge-architect-review@review-axes` for a review's; `skill@knowledge-architect-project-audit` for an audit's |

An illustration of the shape, not authority:

```markdown
- **installed text**: every file `{{command}} install-agent-skills` writes, this primer and the
  skills, agents and saved workflows named `knowledge-architect-*`, never edited by hand
  (`skill@knowledge-architect-agent-configuration@installed-files-never-edited`).
```

The rule, a bullet of the need-test fragment after the bullet on a definition. It states: a coined
term is defined once, in the text of the activity that coins it; the primer's section coined-terms holds one
sentence with its pointer for each coined term the primer uses and each word used in two senses;
another text that uses a term coined elsewhere points to the term's home at its first use; text a
change does not edit is brought to the rule when a change edits it.

**The argument.** A term defined where the act that uses it is delivered reaches every reader of
that act: the primer reaches every session and every subagent, and a skill reaches the sessions of
its activity, per `design@agent-skills@one-home-for-head-rules`. A term the primer uses is met by
every session, so its definition belongs in the primer; a term one activity uses is met only by
sessions that load that activity's text. **The nearest rival**, the owner's central glossary of
every coined term, `thread@terms-and-fix-grounding@primer-glossary`, loses on
`argument@terms-and-fix-grounding@arg-1` and, at the census, on `argument@terms-and-fix-grounding@arg-11`:
more than 30 terms, each a second definition in every session's load.

### The paragraph

In `primer@verify-before-relying`, after the subsection "Check whether it is already known", the
paragraph the owner approved in the reply to round 2, an illustration of the wording:

```markdown
**Before proposing a fix or any other change to code, read what the project decided about that
code**: each entry its comments cite; every entry that cites its file, which
`{{command}} show path@<anchor>@<file>` lists after the file's text; and the heads on its subject in
the design and `rejected` homes of the Component that holds it, where each head's title states its
decision. A fix that contradicts a head is a divergence, routed as `primer@intent-and-claims` says.
```

**The argument.** `argument@terms-and-fix-grounding@arg-6` and `argument@terms-and-fix-grounding@arg-8`.
The paragraph names the moment the owner found, proposing a fix, and gives two mechanical lookups
and one by reading. **The nearest rival**, the same sentence in
`skill@knowledge-architect-issue-tracking@read-before-diagnosing`, loses on its delivery moment,
`argument@terms-and-fix-grounding@arg-9`.

### The outline

A section of `path@agent-skills@README.md`, "How the skills hand off to each other", after
"Extending an installed skill": a text diagram of the activities and their hand-offs, each node
naming its installed skill, then one line per skill that loads at a moment rather than in sequence
(issue tracking, decision recording, project audit, agent configuration, setup, goal setting). An
illustration of the shape:

```text
goal-setting ─▶ design ──(full path)──▶ planning ─▶ work ─▶ review ─▶ merge ─▶ retrospective (offered once)
                  │                                    ▲                              │
                  └──(in-change / bounded)─────────────┘          on the owner's word: retrospective-intake
```

**The argument.** `argument@terms-and-fix-grounding@arg-7`: the skill descriptions route agents,
and human readers have no map. It is in a README because no agent session loads it, so it imposes
no sequence on agents, per `goal@agent-skills@installed-text-leaves-room-to-judge`. **The nearest
rival**, the outline in the primer, `thread@terms-and-fix-grounding@workflow-outline`, loses on
`argument@terms-and-fix-grounding@arg-3`.

## Mapping tables

The work needs none.

## Losing alternatives

- `thread@terms-and-fix-grounding@primer-glossary`, superseded by
  `thread@terms-and-fix-grounding@term-defined-at-first-load`: every coined term in the primer costs
  every session the terms only one activity uses, `argument@terms-and-fix-grounding@arg-1`, and the
  census counts more than 30, `argument@terms-and-fix-grounding@arg-11`.
- `thread@terms-and-fix-grounding@workflow-word-senses`, superseded by
  `thread@terms-and-fix-grounding@term-defined-at-first-load`, which carries it.
- `thread@terms-and-fix-grounding@workflow-outline`, withdrawn by the owner: the triggers already
  route between skills, `argument@terms-and-fix-grounding@arg-7`; lost to
  `thread@terms-and-fix-grounding@readme-workflow-outline`.
- `thread@terms-and-fix-grounding@knowledge-table-lookup`, withdrawn by the agent:
  `argument@terms-and-fix-grounding@arg-10`; absorbed by
  `thread@terms-and-fix-grounding@off-path-grounding`.
- Inside `thread@terms-and-fix-grounding@off-path-grounding`: the sentence in
  `skill@knowledge-architect-issue-tracking@read-before-diagnosing`, ruled out on its delivery
  moment; bug fixes sent through the design skill's grounding, parked, its re-entry the goal on
  development work the owner will add, `argument@terms-and-fix-grounding@arg-9`.

## Readings

The work reads no external specification.

## Premortem

Assume the work shipped and failed.

- **P1**, stresses `thread@terms-and-fix-grounding@off-path-grounding`: the paragraph is read at the
  session's start and needed at the fix, often much later, the mechanism of
  `issue@agent-skills@the-claim-check-of-a-commit-message-is-read-at-the-start-and-needed-at-each-commit`.
  Verdict: named as a cause for the harvest to judge as a tripwire.
- **P2**, stresses `thread@terms-and-fix-grounding@off-path-grounding`: in a project whose code
  cites no head, the lookup falls back to the titles, and a long design home is skimmed as the
  issues were. Verdict: named as a cause for the harvest to judge as a tripwire.
- **P3**, stresses `thread@terms-and-fix-grounding@term-defined-at-first-load`: an entry of the terms
  section drifts from its home's definition. Verdict: an acceptance criterion, AC2, on the owner's
  word.
- **P4**, stresses `thread@terms-and-fix-grounding@term-defined-at-first-load`: "used by two texts"
  admits dozens of terms, and the section grows past what sessions read. Verdict: an acceptance
  criterion, AC1, on the owner's word; the census confirmed the risk,
  `argument@terms-and-fix-grounding@arg-11`, converted into the bound of D1.

## Acceptance criteria

### AC1: at landing, every coined term the primer uses is defined in the primer, and each entry of its terms section is used by two or more installed texts `##terms-census-holds`

Guards `thread@terms-and-fix-grounding@term-defined-at-first-load`. Judged at step 1 and again on the
installed copies at the review before the merge. Observable: a census of the installed primer that
finds a coined term it uses with no definition in it, or an entry of the terms section that only
one installed text uses, the primer counting as one. Response: the entry is added or removed in the
branch; a census that cannot settle what counts as coined reopens the thread. Ruled applied by the
owner, round 3.

### AC2: at landing, each entry of the terms section is one sentence with its pointer, and says nothing its home does not `##terms-entries-restate-nothing-new`

Guards `thread@terms-and-fix-grounding@term-defined-at-first-load`. Judged by the routing review axis
at the review before the merge, which reads each entry against its home. Observable: an entry
longer than one sentence beside its pointer, or one that states what its home does not. Response:
the entry is cut to its home's statement in the branch; a home that cannot be stated in one sentence
reopens the thread. Ruled applied by the owner, round 3.

## Implementation sequence

1. **The terms section.** The census of the primer; the section, last in the primer, with the
   members the census finds; `cargo klarch install-agent-skills`. Fails alone on: a member missing
   or extra in the census, or a pointer the shipped set does not resolve. Claims: every coined term
   the primer uses is defined in it, tested by the census, which is shown to fail by deleting one
   entry and finding the census lists its term; every pointer resolves within the shipped set,
   tested by `cargo test -p knowledge-architect` and `cargo klarch check`, shown to fail by a pointer
   to a section that does not exist.
2. **The rule.** The bullet in the need-test fragment; the install. Fails alone on: a host whose
   installed copy does not carry the bullet, or the hosts test failing. Claim: every host of the
   fragment carries the bullet, tested by `cargo test -p knowledge-architect-agent-skills` and by a
   search of the installed copies for the bullet's first words, shown to fail before the install
   runs.
3. **The paragraph.** The paragraph in `primer@verify-before-relying`; the install. Fails alone on:
   the command form `show path@<anchor>@<file>` printing no reference list. Claim: the command the
   paragraph names lists the entries citing a file after its text, tested by running
   `cargo klarch show path@core@src/check/tree.rs`, whose output ends with its "referenced at" list.
4. **The outline.** The README section. Fails alone on: a hand-off the diagram draws that a skill
   does not state. Claim: every edge the diagram draws is stated in the skill it starts from, tested
   by reading each edge against that skill's text.
5. **The changelog.** The `Next release` entry, per `design@knowledge-architect@changelog-entries`
   and `design@knowledge-architect@versioning-policy`, read whole first; `cargo x changelog`.

The gates, `cargo x gates`, run before the branch is pushed.

## Order rationale

- 1 before 2: the rule's bullet points to the section, which must exist for the shipped set's
  references to resolve.
- 2 before 3: the rule is the mechanism of the section; the paragraph is independent of both, and
  comes after so that a failure of the install in step 3 is the paragraph's alone.
- 3 before 4: the outline names no primer text; it is last of the text because it reaches no agent.
- 4 before 5: the changelog entry describes what the steps built.

## Defaults awaiting the owner

- **D1**, bears on `thread@terms-and-fix-grounding@term-defined-at-first-load`: the terms section
  holds each coined term the primer uses and each word with two senses, rather than every term two
  or more installed texts use; a text that uses a term coined elsewhere points to its home at its
  first use, for new text and text a change edits. The census found more than 30 terms used by two or
  more texts, `argument@terms-and-fix-grounding@arg-11`, against the 17 the reply to round 2 put to
  the owner. It narrows the rule the owner approved, so it is a scope change; it is built as
  written, and the branch merges once the owner has ruled.

## Harvest

| what | where | when |
| --- | --- | --- |
| `thread@terms-and-fix-grounding@term-defined-at-first-load`: the rule and the terms section | a design head of `register@agent-skills@design`, in its subdocument on how the installed text changes, citing `design@agent-skills@statement-names-an-act` and `design@agent-skills@one-home-for-head-rules`; its slug and the thread's pair stated in the harvest's commit | the landing |
| `thread@terms-and-fix-grounding@off-path-grounding`: the paragraph | a design head of `register@agent-skills@design`, in its subdocument on the project's configuration, citing `design@agent-skills@primer-limit`, `design@agent-skills@behaviour-additions-need-real-use` and `design@agent-skills@record-searched-before-the-work` | the landing |
| the two new primer members | the member list of `design@agent-skills@primer-limit` | the landing |
| `thread@terms-and-fix-grounding@primer-glossary` and `thread@terms-and-fix-grounding@workflow-outline` | judged by the tests for a losing alternative; an entry of `register@agent-skills@rejected` for each that passes, cited by the head that argues against it | the landing |
| `thread@terms-and-fix-grounding@readme-workflow-outline` | no head: a choice of human documentation at one site fails the entry tests; the commit message carries its argument | the landing |
| P1 and P2 | judged by the tripwire standard; a tripwire of `register@agent-skills@tripwire` for each the standard admits | the landing |
| the changelog entry | CHANGELOG.md, `Next release` | step 5 |

## Later consequences

- The goal on development work the owner will add may design a grounding for all development work,
  which would absorb or replace the paragraph.
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner`, when it is designed, changes the
  terms section's line on the owner.
- Each installed text a later change edits is brought to the rule: a term coined elsewhere gets a
  pointer to its home at its first use.
- The parked rival, the design skill's grounding for fixes, re-enters with the goal on development
  work.
