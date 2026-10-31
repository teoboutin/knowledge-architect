# Goal review of the goal-standard branch: defaults the session took, and finding counts, per batch

Each batch ran 5 draft-mode review rounds of `agent@knowledge-architect-goal-reviewer`; between
rounds the session repaired every `settled` finding and took its own default on every `intent` and
`contradiction` finding. None of these defaults is ruled by the owner: the owner judges them here
and in the diff of the goals homes. Counts are as each reviewer reported them; "non-minor" excludes
rule 4 and rule 6 findings, tagged `minor` (D4 of the spec). The per-batch statements files, which
held the owner's statements the reviewer was given, and each round's condensed review, were in the
session's scratch space and are not kept.

Batch 1 was ruled by the owner goal by goal and committed in "[goals] Step 6, batch 1: the owner and
agents cluster of the root's goals meets the goal standard"; it is not repeated here.

This file is the session's report of batches 2 to 5 and of the lockstep and whole-set rounds that
followed, as the owner read it before answering "good, I agree with the defaults and the current
goals" to its list of questions; the goals it bears on were ruled since, so the commit that deletes
`spec@plans@goal-standard-and-review` deletes it too. A slug that names an entry of the tree is written as
a reference to it; a slug renamed since stays as the report wrote it.

## Batch 2: the workflow cluster

Goals: docs/goals.md `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `goal@knowledge-architect@agents-get-a-complete-workflow`,
`goal@knowledge-architect@structure-and-workflow-work-together`, `goal@knowledge-architect@records-live-beside-the-code` (new, split from
structure-and-workflow), `goal@knowledge-architect@documentation-stays-consistent`; crates/agent-skills/docs/goals.md
`goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@no-contradiction-in-workflow`, `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`
(new, split from no-contradiction), `goal@agent-skills@only-needed-instructions`.

### Counts

| round | total | non-minor | rule 1 | rule 2 | rule 3 | rule 4 | rule 5 | rule 6 | rule 7 | rule 8 | (b) | (a) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 32 | 25 | 2 | 12 | 4 | 2 | 3 | 5 | 0 | 0 | 2 | 2 |
| 2 | 35 | 30 | 3 | 17 | 6 | 2 | 0 | 3 | 1 | 0 | 0 | 3 |
| 3 | 32 | 27 | 2 | 14 | 8 | 3 | 1 | 2 | 1 | 1 | 2 | 0 |
| 4 | 28 | 20 | 3 | 10 | 2 | 4 | 0 | 4 | 2 | 0 | 1 | 2 |
| 5 | 38 | 26 | 2 | 15 | 9 | 5 | 0 | 7 | 0 | 0 | 3 | 2 |

The (a) contradictions of every round are the same three goals of batch 3 measured by use
(`goal@knowledge-architect@adoption-is-easy`, `goal@knowledge-architect@setup-brings-quality-tools`, `goal@agent-skills@installed-text-works-anywhere`), carried to batch 3.

### Defaults, by goal (the state they reached; earlier defaults the session withdrew are noted)

- **`goal@knowledge-architect@design-is-recorded-with-its-arguments`**
  - Narrows "each decision is recorded with ... the alternatives that lost" to the decisions that earn
    a design head; a decision that shapes code or a text and earns no head has its reason beside it
    and in its commit message. Raised as (b)/intent at every round; kept for the owner to rule,
    since it matches the decision record's own rules.
  - The goal is of a project that adopts the workflow; the installed text asks for the recording.
  - One term, "what is outstanding" (the primer's), for what the owner's goals called "open".
    (Round 2 had added "issues and tripwires"; withdrawn at round 3.)
- **`goal@knowledge-architect@agents-get-a-complete-workflow`**
  - The owner's list of activities kept, as a minimum that grows with the workflow, with two added
    to match `goal@agent-skills@one-skill-per-activity`: maintaining the agent configuration, the intake; "setting up a
    project" and "setting its goals" are two items.
  - The owner's "It targets projects where agents do most of the development" kept. (A range
    "the activities it asks" and a release exclusion were tried and withdrawn.)
  - The aims "fewer review rounds, less re-derivation, fewer decisions reversed by accident" become
    what shows the goal unmet in practice, "that the workflow would have spared" (rule 6).
  - "No two instructions a session cannot both obey", a precedence an installed text states counting
    as a move that obeys both; the pointer to `goal@agent-skills@only-needed-instructions` kept.
- **`goal@knowledge-architect@structure-and-workflow-work-together`**
  - The owner's "one place ... and one place for what is open" kept, read as "the home the
    knowledge table gives"; the manifest declares each such home that is a register's.
  - The checker computes the two lists the owner named; no kind of interface named. The per-change
    list is unmet today, as `issue@core@no-command-lists-the-standing-entries-citing-a-set-of-entries`
    records.
  - Its exclusion "rather than in an external tracker" moves into a new goal.
- **`goal@knowledge-architect@records-live-beside-the-code`** (new)
  - Split from structure-and-workflow, on rule 4 as it stood before D4; the owner's "decisions" and
    "what is open" ("Both live in the repository") kept, plans not added.
  - The commit history counts as the repository's; what a project sends to the workflow's
    maintainers is theirs; a retrospective's files are no record of the project until an intake
    writes their analysis into it.
- **`goal@knowledge-architect@documentation-stays-consistent`**
  - "In a project that uses the checker or the workflow"; one set, "the texts the checker reads";
    "before every merge into the main branch"; the reviews read what a branch touches, and an
    audit, offered, reads the whole record.
  - The sentence "Development driven by agents changes code faster than review alone can follow, so
    the reviews need these tools to keep up" is an argument (rule 1); it stays in the goal until a
    design head citing the goal holds it. (Round 1 had planned core's goals home; changed at round 3.)
- **`goal@agent-skills@one-skill-per-activity`**
  - "Activity" takes the agent-configuration skill's term (the scope over which a complete set of
    procedures makes sense, sub-activities included), with the owner's test of #arg-48 beside it: two
    tasks that follow different methodologies are two activities. (Rounds 1 to 3 tried "a
    methodology" and "procedures"; a clause "the axes of one audit share its procedure" was dropped as
    a design statement.)
  - Coverage and carrying out an activity from its skill stay one intent; a moment of the primer's
    list of skills counts as a step that points to another skill; "every step its activity needs,
    leaving to judgement what judgement handles".
- **`goal@agent-skills@no-contradiction-in-workflow`**
  - Title "No two installed instructions contradict"; a precedence an installed text states counts
    as a move that obeys both; a pair found in use shows the goal unmet.
- **`goal@agent-skills@reviews-judge-by-the-writer-s-criteria`** (new)
  - Split from no-contradiction (rule 4 before D4). Covers a text written under a skill, installed or
    the project's own; the criteria are those that bind its writer: the installed text, the root and
    scoped CLAUDE.md, its skills, and the goals and design record they make binding. Widens the
    owner's "the skill of its activity" (raised as intent; kept for the owner to rule).
  - The owner's "the criteria of the reviews and of the audits agree" kept whole; refines no root goal.
- **`goal@agent-skills@only-needed-instructions`**
  - The set is the whole installed text, as the title says; the owner's "handles well enough" left
    unmeasured (a definition by real use was tried twice and withdrawn); the owner's measure "names
    an act the workflow needs" and test "another instruction needs to vary" kept; the owner's removal
    clause kept as what shows the goal unmet; parent stays `goal@knowledge-architect@agents-work-without-drift`, approved as
    drafted in 27d2aae9.

### Session corrections recorded during the batch

- The statements file had called "decisions and open state" the owner's words, and the parent of
  `goal@agent-skills@only-needed-instructions` the owner's choice; both corrected at round 2.

### Defaults taken after round 5 (version 6, applied to the tree)

- design-is-recorded: "what is still open" in place of "what is outstanding", which the primer
  defines without tripwires; the home the knowledge table gives moves from the outcome into the
  measure (rule 1).
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the list of activities leaves, replaced by a pointer to
  `goal@agent-skills@one-skill-per-activity`, which lists them (one list, two goals).
- structure-and-workflow: "each decision that earns a design head", matching design-is-recorded.
- `goal@knowledge-architect@documentation-stays-consistent`: the workflow's half and the checker's half named by the project
  that uses each; "nothing cited is missing" restored from the owner's text.
- `goal@agent-skills@one-skill-per-activity`: the owner's "should not be bundled" restored as "are not bundled in one
  skill".
- no-contradiction: a precedence leaves the move the prevailing instruction asks, which counts as
  obeying both.
- reviews-judge: "owes or offers", "at the time of the review".

## Lockstep procedure (from here on)

On the owner's word, after batch 2's fifth round, every batch's current version was applied to the
tree, uncommitted, and rounds run in lockstep: one draft-mode reviewer per batch, all five reading
the same composed tree; the session applies every repair and default in the tree only once all five
have returned. Batches 3 to 5 below ran their round 1 against a tree that did not yet hold the other
batches.

## Batch 3: adoption

Goals: docs/goals.md `goal@knowledge-architect@any-project-can-adopt-it`, `goal@knowledge-architect@rust-projects-come-first`, `goal@knowledge-architect@adoption-is-easy`,
`goal@knowledge-architect@setup-brings-quality-tools`; crates/agent-skills/docs/goals.md `goal@agent-skills@installed-text-works-anywhere`,
`goal@agent-skills@installed-text-works-in-any-environment` (new, split from `goal@agent-skills@installed-text-works-anywhere`, which
widened its parent).

### Round 1 counts

Batch 3 round 1 (v1): 25 findings (20 non-minor): rule 1 3, rule 2 12, rule 3 4, rule 4 1 (minor), rule 6 4 (minor), (a) 1; settled 15 (5 minor), intent 9.

### Defaults

Session's defaults taken to write version 1 (on the whole-set review's findings and batch 2's (a) items):
- `goal@knowledge-architect@any-project-can-adopt-it`: a measure on what is built; "a check an installed skill relies on" bounds the checks the cost of conformance never argues against; the grammar is kind, anchor and id, the manifest declaring which kinds and anchors a project has.
- `goal@knowledge-architect@rust-projects-come-first`: unchanged; its universal is the owner's to rule, through the open issue.
- `goal@knowledge-architect@adoption-is-easy`: "the rendered site of knowledge-architect's documents"; the measure is on the documentation built, and an adopter who had to read the source shows the goal unmet.
- `goal@knowledge-architect@setup-brings-quality-tools`: "proven useful ... in the projects that follow the workflow"; the measure is on the setup skill.
- `goal@agent-skills@installed-text-works-anywhere`: its environment half becomes its own goal, `goal@agent-skills@installed-text-works-in-any-environment`, since it widened its parent; "any environment" is one of a harness the checker supports.

Session's defaults taken after round 1, for version 2:
- any-project: the grammar's form (kind, anchor, id) leaves the goal as a design's; the owner's "a check the workflow needs to work well" restored; "no path or name particular to one project".
- rust-first: "knowledge-architect" and "an adopting project" named apart; the measure on use becomes "the documentation states what a project in another language adopts"; the universal stays the owner's to rule.
- adoption-easy: the site is knowledge-architect's own, and the measure names it (unmet while it is not built); "the install of the checker".
- setup-quality: the owner's "proven useful alongside it" restored; the list is a minimum; a project in another language gets the shapes its reduced mode admits.
- installed-anywhere: "write the command the project declares"; "any layout its manifest declares".
- installed-in-any-environment: "outside the installed set and the project"; no instruction or skill the install does not write; the measure covers instructions; "the installed text routes".

## Batch 4: the core checker

Goals: crates/core/docs/goals.md, its introduction and its 6 goals.

### Round 1 counts

Batch 4 round 1 (v1): 16 findings (14 non-minor): rule 1 2, rule 2 9, rule 3 3, rule 6 2 (minor); settled 9 (2 minor), intent 4, (b) 3.

### Defaults

Session's defaults taken to write version 1 (on the whole-set review's findings):
- `goal@core@documents-stay-current`: "stale" means references, shapes and generated listings, as its body says; the title says so; the slug stays.
- declared-instructions-are-checked: "no project can hold part of it back" reads as no exemption from what the manifest declares; the reference grammar is shared by every project and not declared, agreeing with `goal@knowledge-architect@any-project-can-adopt-it`; an explicit measure.
- `goal@core@relocation-is-one-manifest-edit`: a move to another parent directory, not a rename; the title narrows to the body; the slug stays.
- `goal@core@records-reach-their-reader`: no finding; unchanged.
- `goal@core@documents-render-as-a-linked-site`: the documents are knowledge-architect's own; the mechanisms (a preprocessing pass, mdbook) and the argument "Dense cross-referencing is what makes the rendered site worth reading, so every citeable thing having one reference form serves this goal directly" leave the goal; the time condition "once it is released" leaves; a measure on what is built.
- `goal@core@projects-add-their-own-checks`: conforming; unchanged.
- The introduction gains the checker sentence, per the corrected Q15.

Session's defaults taken after round 1, for version 2:
- `goal@core@documents-stay-current`: any project that runs the checker; the measure on what the checker reports, `check` for a tree and `commits` for a range.
- declared-instructions: the owner's "no project can hold part of it back" restored, over the rules compiled in and declared; "exempt" withdrawn; the commands are check and commits; title and body name the same set.
- relocation: the owner's title "Relocating or refactoring what exists" restored, covering a change of shape of a register's home; the rename sentence limited to a Component.
- records-reach: "every text it reads".
- documents-render: the owner's text "stays as written" (7dfe8fd5) restored, the pass, mdbook and the argument, "once it is released" dropped as met; the documents are knowledge-architect's own (default), for its adopters and contributors; the measure covers the hosting and names the documents the checker reads.

## Batch 5: gates and xtask

Goals: crates/gates/docs/goals.md `goal@gates@gates-from-a-list`; tools/xtask/docs/goals.md
`goal@xtask@one-command-runs-every-gate`, `goal@xtask@repeated-tasks-are-automated`.

### Round 1 counts

Batch 5 round 1 (v1): 13 findings (11 non-minor): rule 1 2 (intent), rule 2 6 (5 settled, 1 intent), rule 3 3 (intent), rule 6 2 (minor); (a) 0, (b) 0.

### Defaults

Session's defaults taken to write version 1 (on the whole-set review's findings):
- `goal@gates@gates-from-a-list`: "can run", so the goal covers a project that chooses the library, not every adopting Rust project; the title follows; a measure on what the library provides.
- `goal@xtask@one-command-runs-every-gate`: the closed list of gates gives way to "the gates its list names", the list being a design head's (`design@xtask@gates-list-primary-home`); "by default it runs them all", since CI runs it with --fail-fast.
- `goal@xtask@repeated-tasks-are-automated`: "None exists today." leaves, a state claim that goes stale and is false (cargo x changelog, cargo x skill); a measure on what is built.

Session's defaults taken after round 1, for version 2:
- `goal@gates@gates-from-a-list`: "is led to run", between the owner's "runs" and the session's "can run", since the library cannot oblige an adopter; "as a command"; "nothing beyond its list of gates and the call that runs it" in place of "gate logic"; the title keeps the owner's "runs".
- `goal@xtask@one-command-runs-every-gate`: the owner's list restored as a minimum, with the rebase check behind its flag; "runs them all when one fails, unless asked to stop at the first"; "whether a branch passes the checks".
- `goal@xtask@repeated-tasks-are-automated`: "with no judgement" in the body, matching the title's "mechanical"; the measure ranges over the tasks this repository's instructions or skills ask; the owner's "no such task is left to be done by hand" kept as what shows the goal unmet. The round-1 ground for dropping "None exists today." was false (it said no such task is left by hand); the sentence leaves as a state claim, on rule 6.

## Lockstep round 1 (composed tree)

| batch | total | non-minor | r1 | r2 | r3 | r4 | r5 | r6 | (b) | (a) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 9 | 9 | 0 | 7 | 2 | 0 | 0 | 0 | 1 | 0 |
| 2 | 26 | 16 | 1 | 11 | 4 | 5 | 0 | 5 | 2 | 0 |
| 3 | 17 | 15 | 2 | 11 | 2 | 1 | 0 | 1 | 0 | 0 |
| 4 | 24 | 21 | 4 | 11 | 6 | 0 | 0 | 3 | 3 | 0 |
| 5 | 15 | 13 | 2 | 6 | 2 | 0 | 2 | 2 | 0 | 0 |

Minor findings (rules 4 and 6) were left as they stand unless a repair of another finding covered them.

### Defaults, by goal

Batch 1 (approved by the owner and committed; each change below rewords approved text):
- `goal@knowledge-architect@agents-work-without-drift`: "the design recorded in that project" names its registers (goals, design heads, rejected designs, tripwires, issues); the measure reads "each of those registers". The (b) finding on Q40's "what another goal gives the owner", narrowed to three named goals: no change, since the owner approved the seven goals as drafted, the three names included.
- mechanical-tasks: "several steps" becomes "more than one command or a script" (so `index --staged` then `check --staged` is a case the goal covers, unmet today).
- `goal@knowledge-architect@the-owner-decides`: title "is informed of each decision the agents record or report"; "sends to the owner for a ruling these, and nothing else" in body and measure; "can see" in place of "sees".
- `goal@knowledge-architect@the-owner-s-word-is-verified`: a goal's premises include those of every commit recording a ruling on a sentence the current text still holds (the wider reading).
- `goal@knowledge-architect@design-with-agents-as-counterparts`: "each proposal of the exchange".
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: "a real project that follows it or is adopting it", so a setup session's retrospective counts.

Batch 2:
- design-is-recorded: "what is outstanding" (issues) and "the evidence that would flip a recorded decision" (tripwires) named apart. The (b) narrowing of the lost alternatives to head-earning decisions stays for the owner.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the owner's list of activities restored as a minimum ("among the activities one-skill-per-activity lists"), so a rewording of the child cannot drop one; "How a project writes and tests its code is left to it"; the precedence clause leaves for a pointer to `goal@agent-skills@no-contradiction-in-workflow`.
- structure-and-workflow: "one such place for what is outstanding".
- records-live: the measure reads the installed primer's table only, so a project's own row for an external tracker is its own choice; the workflow's file of a retrospective is "no record of the project".
- `goal@knowledge-architect@documentation-stays-consistent`: the review reads also the documents that describe the code a branch touches; "every rule the checker holds, compiled in or declared in the manifest" in place of "the declared structure"; the checker makes the same guarantees where it is used alone. The argument sentence stays until its design head exists.
- `goal@agent-skills@one-skill-per-activity`: "where the two tests disagree, the methodologies decide".
- no-contradiction: "the move a precedence names counting as one".
- reviews-judge: the time "at the time of the review" moves into the statement. The (b) widening from "the skill of its activity" stays for the owner.
- `goal@agent-skills@only-needed-instructions`: the reason may be "in the installed text or in its source's comment", in the statement as in the measure. "Well enough" stays the owner's to define.

Batch 3:
- any-project: "particular to one adopting project" in body and measure, so the tool's own crate directories are outside it; "path or convention" in both.
- `goal@knowledge-architect@rust-projects-come-first`: title "every check and tool"; the set is what "the published crates offer"; the documentation is "the core's README and the setup skill". The universal stays the owner's, through its issue.
- `goal@knowledge-architect@adoption-is-easy`: "each published package's README"; the installed skills "taken together"; "from the first install of the skills".
- `goal@knowledge-architect@setup-brings-quality-tools`: the measure covers the four listed tools; another language gets each shape that depends on no language, with a place for its language's checks; "a project that adopts the workflow", so a checker-only project is outside.
- `goal@agent-skills@installed-text-works-anywhere`: "name the command by which the project runs the checker".
- `goal@agent-skills@installed-text-works-in-any-environment`: one set, "outside the installed set and the project"; "in any environment that loads it" in place of "a harness the checker supports"; the measure is that no installed text lets an environment's addition take an installed one's place.

Batch 4:
- Introduction: kept as framing, no goal.
- `goal@core@documents-stay-current`: "in the texts it reads"; the owner's "on the main branch and on every commit of a branch that merges" restored; the measure "a tree where any of these fails fails `check`", agreeing with the phase gate.
- declared-instructions: the manifest declares "the kinds and the anchors by which one record points at another, in the reference grammar every project shares" (the reading that keeps the owner's words); "beyond the paths its manifest exempts"; "the rules the core compiles in"; "in the texts the checker reads".
- relocation: the title kept; "refactoring a register's home, from one file to a directory or the reverse"; "no document that cites what moved". The rename exclusion and the anchor sentence (rule 1; the owner's wording) are kept for the owner.
- records-reach: the owner's "every text that references it" restored; the measure holds every reference in the texts the checker reads, and a reference elsewhere shows the goal unmet; title "reaches the session working on what it bears on".
- documents-render: "once it is released" restored; "for its adopters and its contributors" dropped. The mechanisms and the argument stay, on the owner's "stays as written".
- `goal@core@projects-add-their-own-checks`: "any extension a project writes" (so the register-view issue makes it unmet today); "its checker binary runs them in the same subcommands".

Batch 5:
- `goal@gates@gates-from-a-list`: the owner's title restored ("gets its merge gates by writing only its list of gates"); the owner's verb "runs" restored; "every gate of its list"; "as a command" dropped; the measure "no gate logic beyond its list: no runner, no log, no distiller and no verdict of its own".
- `goal@xtask@one-command-runs-every-gate`: "every gate of the project's list", the owner's list among them, the rebase check when asked; "by default it runs them all"; the verdict "says whether the branch passes its gates"; the measure on the tree, the command's list and CI.
- `goal@xtask@repeated-tasks-are-automated`: the task is one this repository's own instructions ask at a recurring moment; the instructions are the root and scoped `CLAUDE.md` files and the `klarch-` skills and agents.

### Met outside the task in this round

- The root CLAUDE.md asks two repeated mechanical tasks with no `cargo x` command: deleting the backup refs after a reword, and the merge predicate. Under `goal@xtask@repeated-tasks-are-automated` they are unmet cases: an issue of xtask once the goals land.
- Two issues quote goal wording the drafts change (the reduced-mode issue quotes `goal@knowledge-architect@adoption-is-easy`'s old measure; the rust-first issue quotes "the project offers"); the render issue says "a project's documents". Row 2 fixes once the goals land.
- `goal@knowledge-architect@documentation-stays-consistent`'s measure had the phase-gate problem too; fixed with "a tree where any fails fails check" in its child only. The parent's "reports every reference" stays as is.

## Lockstep round 2

| batch | total | non-minor | r1 | r2 | r3 | r4 | r5 | r6 | r7 | (b) | (a) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 12 | 10 | 0 | 6 | 4 | 1 | 0 | 1 | 0 | 3 | 0 |
| 2 | 24 | 16 | 1 | 7 | 7 | 5 | 0 | 3 | 1 | 4 | 1 |
| 3 | 20 | 16 | 2 | 11 | 2 | 2 | 0 | 2 | 1 | 0 | 0 |
| 4 | 21 | 20 | 4 | 9 | 7 | 0 | 0 | 1 | 0 | 2 | 0 |
| 5 | 11 | 10 | 1 | 6 | 2 | 0 | 0 | 1 | 1 | 1 | 0 |

Several (b) findings of this round were the session's own round-1 defaults departing from the owner's
approved words; each such default is withdrawn and the owner's words restored, and the question the
round-1 finding raised is left for the owner.

### Defaults, by goal

Batch 1:
- `goal@knowledge-architect@agents-work-without-drift`: "the design recorded in that project: its goals and its design heads, bringing back none of its rejected designs"; tripwires and issues are read as what bears on them, not as part of the design (round 1 had counted them in, which Q26 did not rule); "ranks first among knowledge-architect's goals". The (b) on Q40's "what another goal gives the owner", narrowed to three goals in the text the owner approved, stays for the owner.
- mechanical-tasks: the approved "several steps" restored (round 1's "more than one command" widened the owner's hedged words); what "several" bounds stays the owner's.
- `goal@knowledge-architect@the-owner-decides`: title "every decision the agents make that earns durable text or a line of a turn's Decided section", Q27's set; "exchange" cites `goal@knowledge-architect@design-with-agents-as-counterparts` for its definition.
- `goal@knowledge-architect@the-owner-s-word-is-verified`: the approved text restored (round 1's wider reading dropped Q41's second clause); which commits' premises count, for a sentence kept across rulings, stays the owner's.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: the owner reverses what the record or the argument settled with a goal, a fact or an argument, and what a goal settled by changing the goal (goals bind outright).
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: the approved "already follows it" restored; whether an adopting session's retrospective counts stays the owner's.
Batch 2:
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the child's names for the activities; "auditing a project on one aspect by the project audit's method"; the owner's "Writing the code itself is left to each project" restored; the statement points to `goal@agent-skills@no-contradiction-in-workflow` for the no-contradiction clause.
- structure-and-workflow: one place each "for what is outstanding and for the evidence that would flip a recorded decision", so the owner's "what is open" keeps tripwires. The head-earning narrowing follows design-is-recorded's, held for the owner.
- records-live: the statement reads the installed primer's table, as the measure does; the workflow's file and the project files are both no record until an intake writes their analysis.
- `goal@agent-skills@one-skill-per-activity`: "auditing a project on one aspect by the project audit's method", so goal-setting's whole-set review is goal-setting's.
- reviews-judge: the criteria include "the external specifications they make binding".
- `goal@agent-skills@only-needed-instructions`: the reason is in the installed text (the owner's "state ... with the reason"); the %% comments hold citations, not the reason; the structure test is "that another instruction needs to vary" in the statement as in the measure.
- Held for the owner, unchanged: the (b) narrowing in design-is-recorded, the (b) widening in reviews-judge, "well enough", the precedence clause of `goal@agent-skills@no-contradiction-in-workflow`.
Batch 3:
- any-project: "one adopting project" in the second sentence too; "declares in its manifest the structure particular to it". The manifest as the interface and "a check the workflow needs" stay as the owner approved.
- `goal@knowledge-architect@rust-projects-come-first`: "built for Rust first", on the owner's "for now".
- `goal@knowledge-architect@adoption-is-easy`: each package's README and API documentation state what an adopter of that package does; the skills taken together and the site state the whole adoption.
- `goal@knowledge-architect@setup-brings-quality-tools`: "proven useful alongside it in the projects that follow the workflow, knowledge-architect's own and thaum's among them"; the owner's "adopts knowledge-architect" restored; another language gets the one command, CI and the commit-message check in shapes that depend on no language.
- `goal@agent-skills@installed-text-works-anywhere`: "one adopting project", matching its parent; the approved "write the project's command as the project declares it" restored (round 1 had moved it towards the outcome; whether the command itself is meant stays the owner's).
- `goal@agent-skills@installed-text-works-in-any-environment`: title "loads it"; it may rely on "the harness's own mechanics"; never takes the place "of an installed skill or instruction". "Any environment that loads it" stays: the harness question is the owner's.
Batch 4:
- `goal@core@documents-stay-current`: "in a project whose gates run the checker on each commit"; the measure is the checker's verdict, with no command names.
- declared-instructions: "the rules" in place of "the instructions"; "every rule the core compiles in" in body and measure; "by running the checker", neither one command nor several. The exemption clause stays, unbounded, for the owner.
- relocation: title "costs at most one manifest edit, and no edit to a document that cites it"; a move changes the manifest, a refactor of a home from one file to a directory changes nothing else; "or the reverse" dropped, since it breaks path references to subdocuments. The rename exclusion and the anchor sentence stay for the owner.
- records-reach: the owner's title restored ("is delivered to whoever needs it"); "every text it reads", one measure. Narrowing the owner's "every text" to the texts the checker reads is held for the owner, since the wider reading is unmet by construction.
- documents-render: unchanged; "once it is released" read as met by the crates' publication.
- `goal@core@projects-add-their-own-checks`: the owner's "an extension needs no change to the core" restored; whether it means every extension stays the owner's, with the register-view issue.
Batch 5:
- `goal@gates@gates-from-a-list`: "writing no gate logic beyond that list and the call that runs it"; "from exit codes" dropped (rule 1; the owner's words, shown here). The owner's "runs" stays; whether it binds every adopting Rust project is the owner's.
- `goal@xtask@one-command-runs-every-gate`: title "Every gate the project owes"; the verdict is "on the working tree and the branch's commits" and says "whether the gates let the branch merge", keeping the owner's "may merge" within the gates.
- `goal@xtask@repeated-tasks-are-automated`: title "A mechanical task this repository's own instructions ask repeatedly"; "that takes several steps", matching its root sibling; local covers "the procedure they fix for one the installed text asks".

### Met outside the task in this round

- Five sites cite `goal@agent-skills@installed-text-works-anywhere` for the environment half the split moved to the new
  goal (crates/agent-skills/content/PRIMER.md:459 in a %% comment;
  crates/agent-skills/docs/design/design-skill-and-heads.md:55; project-configuration.md:477;
  shipped-set.md:62; audit-method.md:196-199): re-pointed with the citer repairs once the owner rules.
- The slug declared-instructions-are-checked no longer matches its title, which covers compiled-in
  rules too: kept, to rename with the citer repairs if the owner wants.

## Lockstep round 3

| batch | total | non-minor | r1 | r2 | r3 | r4 | r5 | r6 | (b) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 8 | 8 | 0 | 6 | 1 | 0 | 1 | 0 | 1 |
| 2 | 15 | 8 | 0 | 5 | 2 | 4 | 1 | 3 | 2 |
| 3 | 9 | 5 | 0 | 4 | 1 | 2 | 0 | 2 | 0 |
| 4 | 14 | 12 | 0 | 9 | 3 | 0 | 0 | 2 | 2 |
| 5 | 6 | 5 | 1 | 3 | 1 | 0 | 0 | 1 | 1 |

From this round on, the briefs ask the reviewers not to report again a question already left for the
owner unless the tree's text changed it, so the counts fall partly for that reason.

### Defaults, by goal

Batch 1:
- `goal@knowledge-architect@agents-work-without-drift`: the recorded design includes "the reasons recorded at its code", which the primer calls intent at the scale of that code, in the statement and the measure.
- `goal@knowledge-architect@the-owner-decides`: one set, Q27's: "earns durable text", the list as its examples ("such as a design head, ..."); "agents decide what the argument settles". Whether a sub-point the record settles, raised inside a ruled question, counts as put before the owner or as ruled stays the owner's.
- `goal@knowledge-architect@design-with-agents-as-counterparts`: round 2's "what a goal settled by changing the goal" withdrawn (it narrowed Q11 and set the goal against `goal@knowledge-architect@the-owner-decides`); the approved wording restored, reordered as "reversing, with a goal, a fact or an argument, what a goal, the record or the argument settled".
- Left for the owner, approved text unchanged: what evidence can make a goal's premise false (`goal@knowledge-architect@the-owner-s-word-is-verified`); whether the real-use exclusion binds builders outside a session (`goal@knowledge-architect@the-workflow-improves-through-real-use`).
Batch 2:
- design-is-recorded: "the home the installed text gives it", since a commit message and a reason beside an instruction are given by `primer@design-heads`, not the table; "code or an agent-facing text".
- structure-and-workflow: "one such home each, in the anchor it belongs to", which this tree's six issue homes meet.
- records-live: tripwires added ("and the evidence that would flip a decision", "or a tripwire"); the round-1 carve-out for a project's own table rows withdrawn: "the knowledge table ..., the project's own rows included", as the owner's "rather than in an external tracker" reads.
- `goal@agent-skills@only-needed-instructions`: the owner's approved text restored: the statement keeps "with the reason for each instruction", the measure has no reason clause. Round 2's premise, that the %% comments hold citations and not reasons, was false (root CLAUDE.md: they "carry the reasons and the citations"). Whether the owner's "state ... with the reason" admits a reason only in a %% comment, as `design@agent-skills@shipped-text-line-comments` argues, is held for the owner. The measure takes the statement's qualifier, "between a unit of the work and a unit of its record".
- Left for the owner: whether a review round the workflow would have spared outranks `goal@agent-skills@only-needed-instructions` where the two compete.
Batch 3:
- any-project: "the tree the checker is built from is no adopting project", so the compiled-in package names and the "self" pin are outside it.
- `goal@knowledge-architect@setup-brings-quality-tools`: "a Rust project set up by following it designs none of them itself, and a project in another language designs only the checks of its language". The other-language shapes and the title's set stay for the owner.
- `goal@agent-skills@installed-text-works-in-any-environment`: "the harness's own mechanisms, which hold no skill and no instruction", so a skill or an instruction the harness ships counts as one the environment adds.
Batch 4:
- `goal@core@documents-stay-current`: the title names the body's set, "a project whose gates run the checker on each commit". The narrowing "in the texts it reads" of the owner's "every reference" is held for the owner, one answer with declared-instructions, records-reach and `goal@knowledge-architect@documentation-stays-consistent`.
- declared-instructions: the measure carries the body's bounds, "in the texts the checker reads and outside the paths the manifest exempts".
- relocation: "changes no file outside the home and no manifest line". Whether "what exists" covers moving an entry between Components, and whether an extension's code counts, stay for the owner.
- records-reach: "every text of the tree it reads", since show lists no commit message.
- documents-render: "the Markdown documents the checker reads". Whether "every reference" includes one that resolves to nothing in the tree stays the owner's.
Batch 5:
- `goal@gates@gates-from-a-list`: the measure "no gate logic beyond its list and the call that runs it", matching the body; the owner's title kept.
- `goal@xtask@one-command-runs-every-gate`: "and the rebase check", its flag left to its design head. The (b) on the owner's "every check a branch must pass" and "may merge" stays for the owner.
- `goal@xtask@repeated-tasks-are-automated`: a task the installed text asks "with no procedure of this repository's own" is the checker's, and the measure covers a task whose procedure this repository's files fix. Whether a reword, one judged input and a mechanical rest, is a task with no judgement stays the owner's.

### Session correction

- Round 2's default on `goal@agent-skills@only-needed-instructions` rested on a false premise, that the %% comments hold
  citations and not reasons; withdrawn at round 3.
- A round-3 reviewer cited a design head `goal-placement` as a rule; `cargo klarch show` reports that it resolves to nothing, so the lead was dropped.

## Lockstep round 4

| batch | total | non-minor | r2 | r3 | r4 | r6 | r7 | (b) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 5 | 5 | 5 | 0 | 0 | 0 | 0 | 0 |
| 2 | 15 | 7 | 5 | 2 | 3 | 5 | 0 | 0 |
| 3 | 9 | 5 | 5 | 0 | 2 | 2 | 0 | 0 |
| 4 | 12 | 8 | 8 | 0 | 1 | 3 | 0 | 0 |
| 5 | 4 | 3 | 2 | 0 | 0 | 1 | 1 | 0 |

No rule 1, 5 or 8 finding and no contradiction this round. With round 4, batches 3, 4 and 5 have had
five rounds each (one alone, four in lockstep).

### Defaults, by goal

Batch 1:
- `goal@knowledge-architect@agents-work-without-drift`: the recorded design is the record "as it stands until a change reverses part of it through the workflow's procedure", so a reversal argued through the design skill is no departure, and a head rewritten to match drifted code without that procedure is one; the reasons are those "recorded beside its code and its agent-facing text", as design-is-recorded records them; the tripwires and issues read are those "that bear on the work" (Q26; structure-and-workflow's per-change list).
- `goal@knowledge-architect@the-owner-decides`: the title adds "and every action that cannot be undone", matching the body's list; "for a ruling or an approval", so asking the owner to approve what the argument settles is outside the closed list (the intake's typo case, issue `issue@agent-skills@texts-send-the-owner-more-than-the-closed-list`).
Batch 2:
- design-is-recorded: every decision that earns no head has its reason in its commit message, and beside the code or the agent-facing text it shapes (the owner's "Each decision" keeps its scope); the head names its rivals "in its body or in the rejected designs it cites".
- `goal@knowledge-architect@agents-get-a-complete-workflow`: the narrow reading, "No installed skill covers writing the code itself", so the installed instructions about code stay in scope.
- structure-and-workflow: "a home the installed text gives it", design-is-recorded's term.
- records-live: the retrospective's files are no record "of the project that wrote them; an intake makes their analysis a record of the project it runs in".
- `goal@knowledge-architect@documentation-stays-consistent`: the check's guarantees range over "the texts the checker reads outside the paths the manifest exempts".
- reviews-judge: "the design record" is named as "the design homes, the rejected designs, the tripwires, the issues"; the statement names the reviews and audits "the installed text owes or offers", as the measure does.
Batch 3:
- `goal@knowledge-architect@rust-projects-come-first`: one term, "feature", in title, body and measure, the measure naming it as "a check, a tool or a library API" the published crates offer.
- `goal@knowledge-architect@adoption-is-easy`: "the README of each published package" in the body too.
- `goal@knowledge-architect@setup-brings-quality-tools`: "none of the four"; the title follows the owner's body, "Adopting knowledge-architect".
- `goal@agent-skills@installed-text-works-in-any-environment`: the clause made restrictive, "those of the harness's own mechanisms that hold no skill and no instruction".
Batch 4:
- `goal@core@documents-stay-current`: "the texts the checker reads".
- declared-instructions: a project holds nothing back "beyond the paths its manifest exempts and the rules a declaration of it switches off, as an empty list of harnesses switches off the checks of the installed text". The slug keeps "instructions" as a name only; renaming it is left for the citer repairs if the owner wants.
- records-reach: one measure per listing; the references "in the texts of the tree the checker reads". The range of the owner's title "whoever needs it" stays the owner's.
- documents-render: the measure names "knowledge-architect's Markdown documents that the checker reads", and "rendered as a link".
Batch 5:
- `goal@xtask@one-command-runs-every-gate`: "and the rebase check, which it runs when asked", agreeing with its design head without naming the flag.

## Whole-set round 1 (one reviewer, every goals home)

14 findings (11 non-minor): rule 2 7, rule 4 2 (minor), rule 5 2, rule 6 1 (minor), rule 7 2; settled 10, intent 4, contradiction 0. The two rule 5 findings were conflicts across homes the batch reviews had not seen: `goal@knowledge-architect@any-project-can-adopt-it` against `goal@core@projects-add-their-own-checks`, and `goal@agent-skills@reviews-judge-by-the-writer-s-criteria` against `goal@knowledge-architect@agents-work-without-drift`.

### Defaults

- `goal@knowledge-architect@agents-work-without-drift`: the ranking never overrides "a goal's exclusion that names the design it excludes" (rule 8's form), so only-needed's structure clause and "No installed skill covers writing the code itself" count; "a goal that refines it taking no rank from it", so `goal@agent-skills@only-needed-instructions` does not outrank `goal@knowledge-architect@agents-get-a-complete-workflow` through its parent (the held ranking question between those two stays the owner's).
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: title "is done by the checker" (xtask's home calls xtask "the tool"); the slug stays.
- `goal@knowledge-architect@documentation-stays-consistent` and declared-instructions-are-checked: "outside the paths the manifest exempts" dropped from both measures, since the walk skips those paths; the exception for "the rules a declaration switches off" carried into both measures.
- `goal@knowledge-architect@any-project-can-adopt-it`: "compiled into the published crates", so an extension's binary, which compiles in one project's checks, meets it with `goal@core@projects-add-their-own-checks`.
- `goal@core@records-reach-their-reader`: "standing issue and tripwire", the primer's term.
- `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: the criteria include "the reasons recorded beside the code and the agent-facing text", which `goal@knowledge-architect@agents-work-without-drift` has a review read.
- `goal@agent-skills@installed-text-works-in-any-environment`: "depends on nothing outside the installed set, the project and the harness's own mechanisms, and on no skill or instruction outside the first two".
- Left for the owner: whether `goal@core@relocation-is-one-manifest-edit`, a cost bound, refines `goal@knowledge-architect@documentation-stays-consistent` or another goal; whether `goal@core@records-reach-their-reader`'s whole-project list refines the parent's per-change list.

### Met outside the task

- The primer's form `path@<parent-anchor>@<dir>/` for an anchor's own directory needs a repair when a Component moves, against `goal@core@relocation-is-one-manifest-edit`; no such reference to a Component directory exists today. The fix lies in the installed text or the checker: kept for the workflow's file, per `design@knowledge-architect@workflow-findings-stay-here`.

## Whole-set round 2

8 findings (5 non-minor): rule 2 3, rule 4 1 (minor), rule 6 2 (minor), rule 7 2; settled 7, intent 1, contradiction 0.

### Defaults

- `goal@knowledge-architect@the-owner-decides`: "the record" defined, for it and the goals that cite it, as what the project keeps beside its goals: design heads, rejected designs, tripwires, issues, and the reasons recorded beside the code and the agent-facing text.
- `goal@knowledge-architect@setup-brings-quality-tools`: "gate" in place of "check" for the merge gates, as its child and xtask say; "check" stays the word for a rule of the checker.
- `goal@agent-skills@one-skill-per-activity`: "writing the code itself excepted", carrying its parent's exclusion.
- `goal@xtask@repeated-tasks-are-automated`: "the installed text", the primer's term, in place of "the installed agent text".
- Minor findings left as they stand: `goal@knowledge-architect@agents-get-a-complete-workflow` (clarity and coverage in one goal), `goal@knowledge-architect@adoption-is-easy` and `goal@core@documents-render-as-a-linked-site` (a rendered or hosted site is not in the tree).

### Met outside the task

- "installed agent text" also appears in the setup skill's source (crates/agent-skills/content/skills/setup/SKILL.md:276), which is installed text: kept for the workflow's file; and in crates/core/docs/design/transcripts.md:19: a row 2 rewording with the goals' commit.

## Where the rounds stopped

The session stopped after whole-set round 2: its 5 non-minor findings were wordings, now repaired, and
one known question with a new ground. The tree passes `cargo klarch check`; nothing is committed.

## Questions left for the owner (one list)

Batch 1 (approved goals):
1. `goal@knowledge-architect@agents-work-without-drift`: the ranking excepts three named goals; Q40's default said "what another goal gives the owner" (the narrowing is in the text the owner approved).
2. `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: what "several steps" bounds.
3. `goal@knowledge-architect@the-owner-s-word-is-verified`: which commits' premises count for a sentence kept across rulings; what evidence can make a goal's premise false.
4. `goal@knowledge-architect@the-owner-decides`: whether a sub-point the record settles, raised inside a ruled question, counts as put before the owner or as ruled.
5. `goal@knowledge-architect@design-with-agents-as-counterparts`: whether the owner reverses what a goal settled while the goal stands, or by changing it.
6. `goal@knowledge-architect@the-workflow-improves-through-real-use`: whether a session in a project still adopting the workflow counts as real use; whether the exclusion binds builders outside a session.

Batch 2:
7. design-is-recorded and structure-and-workflow: the lost alternatives are recorded only for decisions that earn a head, a narrowing of the owner's "Each decision".
8. `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: the criteria widened from "the skill of its activity" to every text that binds the writer.
9. `goal@agent-skills@only-needed-instructions`: what "well enough" means; whether "state ... with the reason" admits a reason kept only in a %% comment (`design@agent-skills@shipped-text-line-comments`).
10. `goal@agent-skills@no-contradiction-in-workflow`: the precedence clause (a stated precedence counts as obeying both).
11. `goal@knowledge-architect@agents-get-a-complete-workflow` against `goal@agent-skills@only-needed-instructions`: which ranks first where a review round would be spared by a case-specific instruction.
12. `goal@knowledge-architect@documentation-stays-consistent`: its argument sentence goes to a design head, not yet written.

Batch 3:
13. `goal@knowledge-architect@rust-projects-come-first`: the universal, through its issue.
14. `goal@knowledge-architect@setup-brings-quality-tools`: the shapes owed to a project in another language; whether a project that uses the checker alone is in scope.
15. `goal@agent-skills@installed-text-works-anywhere`: whether "the project's command" means the command itself or where it is declared.
16. `goal@agent-skills@installed-text-works-in-any-environment`: whether "any environment" means any configuration of one harness or any harness.

Batch 4:
17. One answer for four goals (`goal@knowledge-architect@documentation-stays-consistent`, `goal@core@documents-stay-current`, declared-instructions-are-checked, `goal@core@records-reach-their-reader`): the owner's "every reference" / "every text" narrowed to the texts the checker reads, since the wider reading is unmet by construction.
18. declared-instructions-are-checked: how far the manifest's exemptions may reach.
19. `goal@core@relocation-is-one-manifest-edit`: the rename exclusion; the anchor sentence (a mechanism, in the owner's wording); whether "what exists" covers moving an entry between Components; whether an extension's code counts; which goal it refines.
20. `goal@core@records-reach-their-reader`: the range of "whoever needs it"; whether its whole-project list refines the parent's per-change list.
21. `goal@core@documents-render-as-a-linked-site`: the mechanisms and the argument kept on the owner's "stays as written"; whether a reference that resolves to nothing is linked.
22. `goal@core@projects-add-their-own-checks`: whether "an extension" means every extension (the register-view issue makes it unmet).

Batch 5:
23. `goal@gates@gates-from-a-list`: whether "runs" binds every adopting Rust project, against its parent's "is led to"; the owner's title against the body's "the call that runs it".
24. `goal@xtask@one-command-runs-every-gate`: the owner's "every check a branch must pass" and "may merge" narrowed to the list's gates.
25. `goal@xtask@repeated-tasks-are-automated`: whether a reword, one judged input and a mechanical rest, is a task with no judgement.

Work owed after the ruling, not done: the citer repairs of the reworded goals (`goal@agent-skills@installed-text-works-anywhere` has citers in 11 files outside the goals homes; the batch 3 reviewer found five that cite it for the half split off into `goal@agent-skills@installed-text-works-in-any-environment`, not checked one by one by the session); the stale quotations in the rust-first, reduced-mode and render issues; an xtask issue for the fold, the merge predicate and the reword steps; the workflow-file items (the primer's path form against relocation; "design record" and "installed agent text" in the installed text).
