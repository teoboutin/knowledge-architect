# Review of the whole goal set: the goal standard, 2026-10-31

The record of the review of this repository's whole goal set, run under
`skill@knowledge-architect-goal-setting@whole-set-review`. It leaves with the commit that writes the
goals the owner rules last.

## The rounds so far

These rounds ran on the branch `goal-standard`, before the review ran in rounds the owner answers
between: the session took its own default on every `intent` and `contradiction` finding, and the
owner judged the result at the end. Batch 1 was ruled goal by goal and committed by "[goals] Step 6,
batch 1: the owner and agents cluster of the root's goals meets the goal standard", before the
rounds below. Counts are the findings that are not `minor`, every reviewer of the round
together. The briefs changed at lockstep round 3, which from then on asked each reviewer not to
report again a question already open, so the counts before and after that round are not comparable.

| round | reviewers | findings | not minor |
| --- | --- | --- | --- |
| lockstep 1 | one per batch, 5 | 91 | 74 |
| lockstep 2 | one per batch, 5 | 88 | 72 |
| lockstep 3 | one per batch, 5 | 52 | 38 |
| lockstep 4 | one per batch, 5 | 45 | 28 |
| whole set 1 | one, whole-set mode | 14 | 11 |
| whole set 2 | one, whole-set mode | 8 | 5 |

The batches: batch 1, the owner and agents cluster of the root (7 goals); batch 2, the workflow
cluster of the root and agent-skills (9); batch 3, adoption (6); batch 4, core (6); batch 5, gates
and xtask (3).

## The owner's answers

On the goals the rounds above left, with every default the session took and the 25 questions it
had left open, the owner, 2026-10-31:

> good, I agree with the defaults and the current goals, but wording has one problem, see below.
>
> The goal setting skill says "in two to four sentences". This is a hard cap, and a low one at that,
> and I do not like this at all. The new standard clearly requires more than four sentences to
> properly describe everything importtant, and I think you have been going to great lengths to meet
> this cap by writing convoluted sentences. This probably also makes it harder to serve the rules of
> no contradictions and no ambiguity. And I think leaving a bit of freedom in the form could help for
> clarity. I'm fairly certain some goals would be more readable with some things described in a
> bullet list.

On Q49, the record committed under docs/, Q50, the stop at a fifth of the first round's count, and
Q51, the skill's change first and the goals' new layout after it: "defaults for Q49 to Q51, go
ahead".

## Layout round 1

The first round under the round procedure: the goals laid out again without the sentence bound, one
reviewer per set in draft mode and one in whole-set mode, each brief naming this record. The count of
the stop rule is the findings that are not `minor`, every reviewer together: 41, so the review stops
on that rule once a round counts 8 or fewer.

| reviewer | findings | not minor | contradiction |
| --- | --- | --- | --- |
| set 1, the owner and agents | 5 | 5 | 0 |
| set 2, the workflow | 8 | 6 | 1 (b) |
| set 3, adoption | 10 | 6 | 0 |
| set 4, core | 10 | 9 | 0 |
| set 5, gates and xtask | 6 | 5 | 0 |
| whole set | 10 | 10 | 0 |
| total | 49 | 41 | 1 |

The reports, as the session condensed them, every finding kept:

Set 1, layout round 1: 5 findings (5 not minor): rule 2 5; settled 5, intent 0, contradiction 0. Two on a layout difference. Conforming: `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@owner-attention-on-what-is-theirs`, `goal@knowledge-architect@the-workflow-improves-through-real-use`.
drift: B1 settled r2 (layout) "and none of its rejected designs is brought back" as a list item under "as it stands until a change reverses": keep three nouns, rejected designs in a sentence of its own. B2 settled r2 "that record" includes the goals and leaves out tripwires and issues, against `goal@knowledge-architect@the-owner-decides`' definition: "the recorded design".
mechanical: B3 settled r2 (layout) "may ask" reads as a permission: keep the condition "Where the installed text asks".
word-verified: B4 settled r2 "the record" used outside its definition's reach (this goal does not cite `goal@knowledge-architect@the-owner-decides`).
counterparts: B5 settled r2 same as B4.
Met outside: HEAD's goals homes predate the rounds, so a diff against HEAD is not layout only (for the commit message).

Set 2, layout round 1: 8 findings (6 not minor): rule 2 5, rule 3 2, rule 6 2 (minor); contradiction (b) 1, intent 5, settled 2 (minor). Conforming: `goal@knowledge-architect@agents-get-a-complete-workflow`, `records-live-beside-the-code`, `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@only-needed-instructions`.
design-is-recorded: L1 intent r2 "and with the alternatives" fixes the third-item reading of an apposition (measure the same). L2 intent r2/r3 "the evidence that would flip a recorded decision" names no set, vs the tripwire standard (a premortem cause a check reports); approved text has the same words.
structure: L3 intent r2 "Neither the structure nor the workflow" fixes the reading (the approved "Neither" right after the two lists).
`goal@knowledge-architect@documentation-stays-consistent`: L4 (b) r3 the bullet split drops "in the texts the checker reads" from the rules guarantee: restore. L5 minor r6 the listing commands unmeasured (same gap approved).
no-contradiction: L6 intent r2 "the session takes the move" states the session's act; approved "leaves the session the move" a property of the text; "between two instructions" vs "between the two": restore.
reviews-judge: L7 intent r2 the bullet narrows "they make binding" to the goals ... issues; in the run-on sentence "they" could take in the installed text and CLAUDE.md files. L8 minor r6 agree sentence unmeasured.

Set 3, layout round 1: 10 findings (6 not minor): rule 2 6, rule 6 4 (minor); settled 5 (4 minor), intent 5, contradiction 0. Layout changed 1 reading (D1).
any-project: A1 intent r2 "comes first" over what (rank of known work vs target only; `goal@knowledge-architect@the-owner-decides` keeps the order). A2 minor r6 three statements unmeasured.
rust-first: R1 intent r2 "built for Rust first": time order or rank. R2 intent r2 "a check, a tool or a library API": exhaustive or examples (shipped snippets). R3 minor r6 "not given up" unmeasured.
adoption: D1 intent r2 (layout) "each of them does so in plain words and with examples" attaches to all four; approved attached to the last two or all four; the body supports all four.
setup: S1 minor r6 open set vs the four measured.
installed-anywhere: W1 settled r2 "the project's command": the checker's or the gates command: "the command by which the project runs the checker, as its manifest declares it".
environment: E1 intent r2 "depends on nothing outside": any dependency (gh, git) or skills and instructions. E2 minor r6 the first clause unmeasured.
Met outside: retrospective and intake skills depend on gh and GitHub; a defect only under E1's wide reading.

Set 4, layout round 1: 10 findings (9 not minor): rule 1 3, rule 2 6, rule 6 1 (minor); settled 7 (1 minor), intent 3, contradiction 0. Layout dropped nothing; findings C1 and C10 are splits that read less clearly. Conforming: `goal@core@documents-stay-current`, `goal@core@records-reach-their-reader`. Introduction unchanged.
declared: C1 settled r2 (layout) "These stand beside ... / conforms to all of it": name the set. C2 settled r2 the slug's "instructions" (cited from 9 heads and issues); approved text; owner to see.
relocation: C3 settled r1 "Every reference ... names its anchor ... So:" is the mechanism (anchors.md:49, `design@core@every-path-names-its-anchor`). C4 intent r2/r3 references of the form path@<parent-anchor>@<dir>/, and a path to a home that outgrows one file (paths-and-links.md:160-162): inside "no document that cites what moved" or not; none to a Component directory in the tree.
render: C5 intent r1 mdbook outcome or mechanism (the issue reads it decided). C6 settled r1 the argument sentence to a head. C7 settled r2 "once it is released" (crates published). C8 intent r2 whose documents (the issue: a project's). C9 minor r6 hosting.
projects-add: C10 settled r2 (layout) "Its checker binary" after a sentence ending on "its contract": "The project's checker binary".
Met outside: the render issue's wording follows the ruling on C8 and C7.

Set 5, layout round 1: 6 findings (5 not minor): rule 2 4, rule 6 1 (minor), rule 7 1; settled 4 (1 minor), intent 2, contradiction 0. Layout changed 3 readings.
gates: G1 settled r2 heading "writing only its list" vs body "and the call that runs it" (same split approved). G2 intent r7 "runs" as a universal vs the parent's "is led to" (known).
one-command: O1 settled r2 (layout) the rebase check made a member of the list: "every gate of the list" has an exception; state it in the universal. O2 intent r2 (layout) "By default" now scopes the run-all clause only; the verdict holds under every flag (wider than one reading; matches the tool). O3 minor r6 run-all and verdict unmeasured.
repeated: R1 settled r2 (layout) the list after "when it:" can read as any one condition: mark it "all of these".
Met outside: the reword, the fixup fold and the merge predicate have no xtask command and no issue (searched).

Whole set, layout round 1: 10 findings (10 not minor): rule 2 9, rule 7 1; settled 5, intent 5, contradiction 0. Five from the new layout (W1, W3, W4, W9's layout part none; W1, W3, W4 and the rebase membership).
W1 settled r2 (layout) drift's fourth bullet "and none of its rejected designs is brought back". W3 settled r2 (layout) "may ask". W4 settled r2 (layout) the rules bullet lost "in the texts the checker reads". W5 settled r2 "the record": its definition in `goal@knowledge-architect@the-owner-decides` reaches goals that use it otherwise (drift's "that record" includes goals) and misses goals that use it so (word-verified, counterparts); doc-consistent "the whole record"; the intro "the record of design". W6 intent r2 "the owner" and "the project" in `goal@knowledge-architect@the-owner-decides`: an adopting project's owner (met clause) or knowledge-architect's (the citing goals). W7 intent r2 "once it is released" (versions released; or leaving 0.x); bears on `goal@knowledge-architect@adoption-is-easy`'s site. W8 intent r2 "every reference" of the render pass: whose documents. W9 settled r2 "agents" for agent definitions and for sessions (one-skill; installed-anywhere and only-needed write "agents"; no-contradiction "the agent definitions"). W10 intent r7 `goal@gates@gates-from-a-list`'s universal "runs", made stronger by the split "It writes no gate logic". Unnumbered intent r2: the rebase check made a member of the list (matches gates.rs:61).

### Outcomes

Repaired, the layout's own changes of reading:

- `goal@knowledge-architect@agents-work-without-drift`: the list holds three nouns, and "What they build brings back none of
  its rejected designs" stands after it; "that recorded design" in place of "that record".
- `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`: the condition "Where the installed text asks" restored in place of
  "may ask".
- `goal@knowledge-architect@documentation-stays-consistent`: "in the texts the checker reads" restored on the rules guarantee,
  the (b) finding.
- `goal@agent-skills@no-contradiction-in-workflow`: the approved "A precedence an installed text states between the two
  leaves the session the move the prevailing one asks" restored.
- `goal@core@declared-instructions-are-checked`: "conforms to what its manifest states and to the rules the core
  compiles in" in place of "all of it".
- `goal@core@projects-add-their-own-checks`: "The project's checker binary".
- `goal@gates@gates-from-a-list`: one sentence again, "writing only that list and the call that runs it, which
  holds no gate logic".
- `goal@xtask@repeated-tasks-are-automated`: "when it meets all of these".
- `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@installed-text-works-anywhere`, `goal@agent-skills@only-needed-instructions`: "agent
  definitions" for the installed agents, "agent" kept for a session.

Defaults put to the owner, Q52 to Q64, in the session's message of this round. The owner's answer,
2026-10-31: "I agree with those new defaults".

- Q52, `goal@knowledge-architect@the-owner-decides`: "In a project that follows the workflow, this one included, its owner
  states ..."; the definition of the record reaches "this goal and the other goals of this home".
- Q53, `goal@knowledge-architect@rust-projects-come-first`: "built for Rust first" read as a rank, Rust winning where serving
  two languages competes; text unchanged.
- Q54, `goal@knowledge-architect@rust-projects-come-first`: "such as a check, a tool or a library API", so the list gives
  examples.
- Q55, `goal@knowledge-architect@adoption-is-easy`: "each of them does so in plain words and with examples" covers all four
  places, as the body's sentence says.
- Q56, `goal@knowledge-architect@any-project-can-adopt-it`: "is the primary target" in place of "comes first", as the title
  says; the order of known work stays the owner's.
- Q57, `goal@knowledge-architect@design-is-recorded-with-its-arguments`: the lost alternatives are a third item of what a head
  records; "the evidence that would flip a recorded decision" read as the evidence a tripwire
  records under the installed standard; text unchanged on the second.
- Q58, `goal@knowledge-architect@structure-and-workflow-work-together`: "Neither the structure nor the workflow is designed
  without the other".
- Q59, `reviews-judge-by-the-writer-s-criteria`: "the external specifications that any of these make
  binding", the binding texts included.
- Q60, `goal@xtask@one-command-runs-every-gate`: the rebase check is a member of the list, run only when asked;
  "By default" governs the run-all clause only, the verdict holding under every flag, as the tool does.
- Q61, `installed-text-works-in-any-environment`: "depends on no skill or instruction outside the
  installed set and the project; it may rely on the harness's own mechanisms", so a tool such as `gh`
  is no dependency the goal counts.
- Q62, `goal@agent-skills@installed-text-works-anywhere`: "the command by which the project runs the checker, as its
  manifest declares it", in place of the approved "the project's command as the project declares it".
- Q63, `goal@core@relocation-is-one-manifest-edit`: a reference of the primer's form for an anchor's own
  directory, cited from the anchor above, is covered by the goal; the form is the defect, kept for the
  workflow's file; text unchanged.
- Q64, `goal@core@declared-instructions-are-checked`: the slug becomes `document-rules-are-checked`, with every
  reference re-pointed, in the commit that writes the ruled goals.

Already answered, and not put again: the questions the owner closed with "I agree with the defaults
and the current goals", among them the render goal's mechanisms, its argument sentence, "once it is
released" read as met and its documents being knowledge-architect's; the anchor sentence of
`goal@core@relocation-is-one-manifest-edit`; and whether "runs" binds every adopting Rust project.

Minor findings left as they stand: the measures that do not cover a sentence of their body (rule 6),
and the goals that hold more than one intent (rule 4).

## Layout round 2

| reviewer | findings | not minor | contradiction |
| --- | --- | --- | --- |
| set 1, the owner and agents | 3 | 3 | 1 (b) |
| set 2, the workflow | 3 | 3 | 0 |
| set 3, adoption | 4 | 4 | 0 |
| set 4, core | 2 | 2 | 0 |
| set 5, gates and xtask | 2 | 2 | 0 |
| whole set | 6 | 6 | 0 |
| total | 20 | 20 | 1 |

The count falls from 41 to 20; the stop rule asks 8 or fewer.

The reports, as the session condensed them, every finding kept:

Set 1, layout round 2: 3 findings (3 not minor): rule 2 2, rule 3 1; settled 2, contradiction (b) 1. Six goals conform.
drift: 1 (b) r3 (round 1's repair) "What they build brings back none of its rejected designs" lost the reversal qualifier (a rejected design reopened through the design skill, chosen and built). 2 settled r2 "that recorded design" now leaves the rejected designs out of what a departure is. 3 settled r2 "Its goals" right after "before it merges": "That project's goals".

Set 2, layout round 2: 3 findings (3 not minor): rule 2 3; settled 2, intent 1. Six goals conform.
records-live: F1 settled r2 "a record of the project" vs the record's definition, which since Q52 reaches this home (an intake's analysis is none of its members): "a file of the project it runs in". F2 settled r2 (layout) "the history of its commits" after the split: "the repository's commits".
reviews-judge: F3 intent r2/r3 "the reasons recorded beside the code": does it include a reason only in the message of the commit that introduced the code (the primer's reason recorded at the code)? Default: the wide reading, the commit message named.

Set 3, layout round 2: 4 findings (4 not minor): rule 2 4; settled 2, intent 2. Conforming: `goal@knowledge-architect@rust-projects-come-first`, `goal@knowledge-architect@setup-brings-quality-tools`.
any-project: settled r2 (layout) the split gives two targets with no relation: "Among them, a new project ... is the primary target".
adoption: intent r2 "the installed skills, taken together, and the rendered site state the whole": each alone, or jointly (same words approved).
installed-anywhere: settled r2 the Q62 wording misses the default command `klarch` where the manifest declares none (`design@core@declared-command`): "or `klarch` where it declares none".
environment: intent r2 a harness's own built-in instruction: "an instruction the environment adds" or a mechanism the text may rely on (same words approved).

Set 4, layout round 2: 2 findings (2 not minor): rule 2 2; settled 2. Conforming: the introduction and 5 goals.
declared: F1 settled r2 "No project can hold part of it back": round 1's repair removed the antecedent of "it"; name the set. F2 settled r2 the reference grammar: a third member of the measure with no exception, unnamed in the body: name it in the body as the measure does.
Met outside: the render issue's summary still says "a project's documents" (row 2, with the goals commit).

Set 5, layout round 2: 2 findings (2 not minor): rule 2 2; settled 2. Conforming: `goal@xtask@one-command-runs-every-gate`, `goal@xtask@repeated-tasks-are-automated`.
gates: 1 settled r2 round 1's G1, not repaired: the heading "writing only its list of gates" vs the body's list and call. 2 settled r2 (round 1's repair) "which holds no gate logic": the call or the list.
Remark, not counted: one-command states the rebase exception twice (first sentence and sixth bullet).
Met outside: the reword, the fixup fold and the merge predicate have no command and no issue entry, still unrouted.

Whole set, layout round 2: 6 findings (6 not minor): rule 2 6; settled 3, intent 3.
drift: intent r2 "by a check or by a review step": "check" in three senses across homes (the checker's run, a checker rule, an agent's verification step); the primer's "Read the record of code before changing it" counts or not.
`goal@knowledge-architect@documentation-stays-consistent`: intent r2 "an audit that reads the whole record on one aspect": since Q52 the record excludes goals, READMEs, code and agent-facing text, while the audit reads one aspect of the whole project (the agentic-workflow axis reads the installed text).
any-project: settled r2 "Every record is cited in one readable grammar": the countable "record" against the defined "the record" (goals outside it): "every entry".
declared: settled r2 "No project can hold part of it back": the only singular antecedent is now the manifest's statements; name both sets.
repeated: settled r2 "its klarch- skills and agents": "agent definitions". intent r2 "mechanical task" here lacks the root's "result follows from the tree" (the merge predicate depends on GitHub's state).
Met outside: after the layout commit, `goal@knowledge-architect@the-owner-s-word-is-verified` counts only the premises stated by the commits recording a ruling on the current text; the goals commit names the earlier rulings.

### Outcomes

Repaired:

- `goal@knowledge-architect@agents-work-without-drift`: the rejected designs sentence keeps the reversal qualifier, "unless a
  change reopens one through the workflow's procedure", the (b) finding, which round 1's own repair
  had made; a departure is "from that recorded design, or a rejected design brought back without
  that procedure"; "That project's goals".
- records-live-beside-the-code: "a document of the project it runs in", since "record" has a defined
  sense in this home since Q52; "the history of the repository's commits".
- `goal@knowledge-architect@any-project-can-adopt-it`: "Every entry is cited in one readable grammar"; "Among them, a new
  project ... is the primary target".
- `goal@agent-skills@installed-text-works-anywhere`: "or `klarch` where it declares none", the default command
  `design@core@declared-command` gives, which Q62's wording missed.
- `goal@core@declared-instructions-are-checked`: the body names the reference grammar beside the manifest's
  statements and the core's rules, and "No project can hold back any part of these three", the
  exceptions following.
- `goal@gates@gates-from-a-list`: "that call holds no gate logic".
- `goal@xtask@one-command-runs-every-gate`: the rebase exception stated once.
- `goal@xtask@repeated-tasks-are-automated`: "agent definitions".

Defaults put to the owner, Q65 to Q71, in the session's message of this round:

- Q65, `goal@gates@gates-from-a-list`: the heading becomes "A Rust project gets its merge gates by writing only
  its list of gates and the call that runs them", matching the body; the heading is the owner's
  wording.
- Q66, `goal@knowledge-architect@adoption-is-easy`: the installed skills taken together state the whole of adopting it, and so
  does the rendered site, each on its own.
- Q67, installed-text-works-in-any-environment: an instruction the harness itself gives counts among
  those the environment adds, "the harness's own included", so an installed instruction prevails
  over it for an activity the set covers.
- Q68, reviews-judge-by-the-writer-s-criteria: the reasons include those "in the messages of the
  commits that made them", the primer's reason recorded at the code.
- Q69, `goal@knowledge-architect@agents-work-without-drift`: "by the checker or by a step the installed text owes an agent or a
  reviewer", so an agent's own reading of the record before a change counts.
- Q70, `goal@knowledge-architect@documentation-stays-consistent`: the offered audit "reads the whole project on one aspect",
  since "the whole record" no longer reaches the installed text under the definition of Q52.
- Q71, `goal@xtask@repeated-tasks-are-automated`: a task whose result depends on state outside the tree, such as
  the merge predicate, is covered as written; text unchanged.

## Open questions

- The new layout of every goal, which the owner rules on by its slug, each goal with its boundary
  block, once the rounds stop.

## The goals as the owner approved their content

The goals homes as the working tree held them at the owner's answer above, under the bound of two to
four sentences. Each home's text is quoted whole.

### docs/goals.md

````markdown
# Goals — knowledge-architect

knowledge-architect is a tool that projects use to keep the quality of their documentation, with a
focus on the record of design: what was decided, and the intent and the arguments behind it. It is
built for agentic work. It gives AI agents a complete workflow, which aims to raise the quality and
the efficiency of their work, in particular in projects developed mostly by agents. Its main
purpose is to let agents work autonomously without drifting from the design recorded in their
project, per `goal@knowledge-architect@agents-work-without-drift`. The
documentation structure and the workflow are designed to work together.

A goal is met or unmet. A decision about how the project is built is won or lost, and lives in the
design home. **A goal stays in this document while it is met.** A goal removed when it is achieved
stops being checked, and can stop being met without anyone noticing. A goal leaves only when it is
abandoned, on the owner's word, per `design@agent-skills@goal-lifecycle`. The goals are the
owner's intent: an agent may propose wording, and the owner decides it.

## Agents work productively and autonomously without drifting from the design recorded in their project `##agents-work-without-drift`

The project as a whole, the workflow and the checker, lets AI agents carry more of a project's work
with less supervision, while what they build stays within the design recorded in that project, as it
stands until a change reverses part of it through the workflow's procedure: its goals, its design
heads and the reasons recorded beside its code and its agent-facing text, bringing back none of its
rejected designs. It ranks first among knowledge-architect's goals, every Component's included,
where two compete, a goal that refines it taking no rank from it, but never over a goal's exclusion
that names the design it excludes, nor over the owner's authority, exchange and attention that
`goal@knowledge-architect@the-owner-decides`,
`goal@knowledge-architect@design-with-agents-as-counterparts` and
`goal@knowledge-architect@owner-attention-on-what-is-theirs` give. The supervision it saves is
measured by `goal@knowledge-architect@the-owner-decides`. It is met while the workflow and the
checker are built so that, in a project that adopts them, a departure from that record is caught
before it merges: its goals, its design heads, those reasons and its rejected designs, with the
tripwires and the issues that bear on the work, are read against the work by a check or by a review
step the installed text owes. A departure that reaches a project's main branch shows the goal unmet
in practice.

## The documentation records the design, with the intent and the arguments behind it `##design-is-recorded-with-its-arguments`

In a project that adopts the workflow, each decision that earns a design head is recorded with its
intent and its argument, the alternatives that lost and why they lost; a decision that earns no head
has its reason in the message of the commit that made it, and beside the code or the agent-facing
text it shapes; and what is outstanding is recorded, as is the evidence that would flip a recorded
decision. A later session can then tell what it may change, and what a change costs, without
deriving the argument again or reversing a decision blind. It is met while the installed text asks
for each of these to be recorded in the home the installed text gives it: a head carrying its
standing argument, naming its rivals in its body or in the rejected designs it cites, a reason in
its commit message, and beside the code or the agent-facing text, for a decision that earns no head,
what is outstanding as an issue, and that evidence as a tripwire.

## AI agents get a complete workflow that raises the quality and the efficiency of their work `##agents-get-a-complete-workflow`

The workflow covers setting up a project, setting its goals, discussing designs, planning, recording
decisions, tracking open issues, reviewing, auditing a project on one aspect by the project audit's
method, and the retrospective, among the activities `goal@agent-skills@one-skill-per-activity`
lists, a list that grows with the workflow. It targets projects where agents do most of the
development. No installed skill covers writing the code itself, which is left to each project. Each
installed instruction says what it asks plainly enough to follow without guessing what it means, and
no two instructions are ones a session cannot both obey, as
`goal@agent-skills@no-contradiction-in-workflow` states; what no instruction covers is left to the
agent's judgement on purpose, per `goal@agent-skills@only-needed-instructions`. It is met while the
installed text holds a skill for each of these activities and each that goal names, each instruction
with one reading, and no two instructions a session cannot both obey, as
`goal@agent-skills@no-contradiction-in-workflow` measures. A review round, an argument derived
again, or a decision reversed by accident, that the workflow would have spared, shows the goal unmet
in practice.

## A mechanical task the workflow asks of a session is done by the checker `##mechanical-tasks-are-the-tool-s`

Where the installed text asks a session for a task whose result follows from the tree and the
workflow's rules, with no judgement, and that takes several steps or a script the session would
write, the published checker offers a command that does it, so every session gets the same result
without building it again. When such a command is built follows the order of known work, which is
the owner's, per `goal@knowledge-architect@the-owner-decides`. It is met while each such task the
installed text asks of a session has a command of the checker. A task found in real use with none
shows the goal unmet in practice. It refines
`goal@knowledge-architect@agents-get-a-complete-workflow`.

## The documentation structure and the workflow work together `##structure-and-workflow-work-together`

The workflow writes into the structure, and the structure gives the workflow what it needs: one
place, the home the knowledge table gives, to find each decision that earns a design head with its
argument, and one such home each, in the anchor it belongs to, for what is outstanding and for the
evidence that would flip a recorded decision. The checker computes two lists the workflow asks a
session to read: what a reversed decision touches, and which issues and tripwires a change must read
again. Neither is designed without the other. It is met while each kind of record the installed text
asks a session to write has a home the installed text gives it, the manifest declaring each such
home that is a register's, and those two lists are computed by the checker.

## The project's records live in its repository, beside the code `##records-live-beside-the-code`

In a project that adopts the workflow, the project's own decisions, what it has outstanding and the
evidence that would flip a decision, in the homes the knowledge table gives them, the project's own
rows included, live in its repository beside the code, as files an agent reads and edits like any
other and the history of its commits, rather than in an external tracker it would need a separate
tool to reach. The workflow's file of a retrospective, which a project sends to the maintainers of
the workflow, and a retrospective's project files, are no record of the project that wrote them; an
intake makes their analysis a record of the project it runs in. It is met while every home the
knowledge table gives a decision, an outstanding item or a tripwire of the project is a file or a
directory of the repository, or its commit history.

## The documentation stays consistent with the code and with itself `##documentation-stays-consistent`

In a project that adopts the workflow, its reviews and its audits keep the documentation consistent
with the code and with itself; in a project that uses the checker, the checker supports them, and
makes the same guarantees where it is used alone: its check guarantees that, in the texts the
checker reads, nothing cited is missing and every rule the checker holds, compiled in or declared in
the manifest, is met, save those a declaration of the manifest switches off, and its commands list
and display the entries a review must read. It is met while the installed text owes, before every
merge into the main branch, a review that reads the documents the branch touches, and those that
describe the code it touches, against the code and against each other, offers an audit that reads
the whole record on one aspect, and the check reports every reference that resolves to nothing in
the texts the checker reads, and every breach of such a rule. Development driven by agents changes
code faster than review alone can follow, so the reviews need these tools to keep up.

## The owner decides what the argument from the goals, the record, the code and the sources leaves open, and every action that cannot be undone, and is informed of every decision the agents make that earns durable text or a line of a turn's Decided section `##the-owner-decides`

The owner states the project's intent in its goals and brings the facts only the owner holds. The
record, in this goal and the goals that cite it, is what the project keeps beside its goals: its
design heads, its rejected designs, its tripwires, its issues and the reasons recorded beside its
code and its agent-facing text. Agents decide what the goals, the record, the code and its
measurements, and the sources the project follows settle, by argument, and report each such decision
that earns durable text, as below. The workflow sends to the owner for a ruling or an approval
these, and nothing else: a choice between shapes that all survive the argument, where choosing
weighs a cost or a scope that no goal or decision ranks, such as the order of known work; a
contradiction that no source or argument resolves; a goal to set or change; a fact only the owner
holds; and, whatever the argument settles, an action that cannot be undone, such as a publication. A
project may reserve further actions to its owner. A proposal on what the argument settles, which the
agent puts before the owner for challenge inside an exchange, as
`goal@knowledge-architect@design-with-agents-as-counterparts` defines one, is not sent for a ruling.
The owner changes a decision the argument settled through a goal, a fact or an argument, which the
agents weigh like any other, and never by a word that gives none; a choice at a fork is the owner's
to make by a bare pick. Each decision the agents make that earns durable text, such as a design
head, a `rejected` entry, an issue, a tripwire, a commit message or a comment, or that earns a line
of a turn's "Decided" section, is reported in the record, a commit message included, or in that
section, and the owner can open the exchange on it; the two halves are one intent, since agents
decide what the argument settles only while the owner can see each such decision. It is met while
the installed text sends each question to the authority that answers it, sends to the owner for a
ruling or an approval these and no other, and has the session report each such decision so. A
question put to the owner for a ruling that the goals, the record, the code and its measurements, or
the sources answered, or such a decision reported nowhere, shows the goal unmet in practice.

## Agents verify the owner's word before acting on it, and build nothing on a contradiction `##the-owner-s-word-is-verified`

An agent following the workflow takes no word of the owner as granted. Before acting on a word other
than a goal, the agent checks its premises against the goals, the record, the code and its
measurements, and the sources the project follows. A goal's premises, those its text and the commit
messages that record a ruling on its current text state, a rewrap or a re-pointed reference
recording none, are checked when the goal is written or reworded, and again when work meets evidence
against one. A contradiction it finds that no source or argument resolves goes to the owner, and the
part of the work that rests on it waits for the owner's answer; the rest goes on, and a goal whose
premise proved false still binds the rest. It is met while the installed text asks for that check
before an act on a word of the owner in every activity of the workflow, asks for a goal's premises
to be checked when the goal is written or reworded and when work meets evidence against one, and
holds the work that rests on a contradiction. Work built on a word whose premise a check would have
shown false shows the goal unmet in practice.

## The owner designs with agents as counterparts `##design-with-agents-as-counterparts`

The owner and the agent both propose, and each challenges and refines the other's proposals on their
failures, their costs and their benefits, grounded in the record and backed by the measurements the
agent takes. The exchange is open and argued: the owner meets each proposal of the exchange with its
consequences, and its rival where one stands, before it reaches the main branch, and can challenge
any proposal, the agent's own and its refinements of the owner's included, add arguments or facts,
or change the direction of the design, reversing, with a goal, a fact or an argument, what a goal,
the record or the argument settled; a pick between shapes that all survive the argument is no change
of direction. A design discussion, a goal-setting draft and a question put with a built default that
the owner rules on before the merge are such exchanges; a report alone is not. It is met while the
installed text has the agent ground and challenge every proposal of the owner that decides something
before it is built; puts before the owner, in such an exchange and with its consequences and its
rival where one stands, the agent's own proposals on a choice or a goal that the goals, the record,
the code and its measurements, and the sources leave open, and every proposal it makes inside an
exchange the owner is already in; and brings every such choice and goal to the owner before it
reaches the main branch.

## The owner's attention is spent on what is the owner's to read or answer `##owner-attention-on-what-is-theirs`

The agent carries the investigation, the measurements and the tracking of what is outstanding, so
that the owner's attention goes to what is the owner's to read or answer: the proposals put before
the owner, their arguments, the questions only the owner answers, and the reports of what agents
decided. It is met while the installed text gives the agent each of those tasks, and puts a proposal
before the owner with its grounding and its consequences already established.

## Any project can adopt it, and a new project is the primary target `##any-project-can-adopt-it`

Nothing particular to one adopting project is compiled into the published crates; the tree the
checker is built from is no adopting project. A project declares in its manifest the structure
particular to it, and the installed text names no path or convention particular to one adopting
project. Every record is cited in one readable grammar, the same in every project, so what an agent
learns in one project applies in the next. A project pins the version it uses, and moves to a new
one when it chooses. The target is projects developed mostly by AI agents, and a new project, which
adopts the checker and the workflow from its first commit, comes first. An existing project can
adopt them too, by bringing its documents into conformance; what that costs it is never an argument
against a check the workflow needs to work well. A project without agents can still use the checker
alone. A project in a language other than Rust adopts it in the reduced mode of
`goal@knowledge-architect@rust-projects-come-first`. It is met while the checker and the installed
text hold no path or convention particular to one adopting project, the checker enforces one
reference grammar in every project, a project's pin moves only by an explicit edit, and the setup
skill leads a new project and an existing one to a passing check.

## A Rust project gets every feature, and a project in another language a reduced mode `##rust-projects-come-first`

Rust is the language knowledge-architect serves first. A feature that depends on an adopting
project's language is built for Rust first, and a feature is not given up because it can serve only
Rust projects. A project in another language can still adopt the checker and the workflow, without
what depends on Rust. It is met while a Rust project gets every feature the published crates offer,
a check, a tool or a library API, and the core's README and the setup skill state what a project in
another language adopts.

## Adopting the project is easy, with documentation that is clear and reaches its reader `##adoption-is-easy`

A person or an agent adopting knowledge-architect finds what to do in its documentation, where they
already look for it: the README of each published package, the API documentation of each library,
the installed skills, and the rendered site of knowledge-architect's documents. The documentation
says what to do in plain words, with examples. It is met while each published package's README and
each library's API documentation state what an adopter of that package does, the installed skills
taken together and the rendered site state the whole of adopting it, each in plain words and with
examples, and the setup skill leads an adopter from the first install of the skills to a passing
check. An adopter who had to read the source or ask the authors shows the goal unmet in practice.

## The workflow improves through real use `##the-workflow-improves-through-real-use`

Real use is the test of how an agent behaves under the workflow: a session that applies it inside a
real project that already follows it, as part of a contribution submitted to that project, under the
supervision of a person who contributes to the project or owns it. No mock task and no mock project
is built to apply the workflow to and observe how an agent behaves, whether to originate an edit, to
choose between shapes, to accept a piece of work, or to check that a text runs as written. The
retrospective, and the owner's observations of real sessions, collect what was unclear, missing or
wrong; what changes is decided per `goal@knowledge-architect@the-owner-decides`. It is met while the
installed text admits an instruction about how an agent behaves only on evidence from real use or on
a defect provable by reading, has no session build a mock task or a mock project to apply the
workflow to and observe how an agent behaves, and has the retrospective collect the evidence of real
use.

## Adopting knowledge-architect sets up quality tools proven to work with it `##setup-brings-quality-tools`

A project that adopts knowledge-architect is led to set up the quality-assurance tools that have
proven useful alongside it in the projects that follow the workflow, knowledge-architect's own and
thaum's among them, in recommended shapes that work well with the checker, among them one command
running every gate owed before a merge, continuous integration running that command, commit messages
checked as documents, and the gates of the project's language. It is met while the setup skill gives
each of the four tools listed in its recommended shape for a Rust project, and for a project in
another language the one command, its continuous integration and its commit-message check in shapes
that depend on no language, with a place for the gates of its language, so that a Rust project set
up by following it designs none of the four itself, and a project in another language designs only
the gates of its language.
````

### crates/core/docs/goals.md

````markdown
# Goals — the core checker

What the core checker is for, as its share of the project's goals,
`register@knowledge-architect@goal`. The checker exists mostly for
`goal@knowledge-architect@agents-work-without-drift`: it reports where the documents and the code
drift from the recorded design, and it makes keeping that design current cheap. A goal is met or
unmet, where a decision about how the checker is built is won or lost and lives in
`register@core@design`. A goal stays here while it is met, and leaves only when the owner abandons
it.

## In a project whose gates run the checker on each commit, references, register entries and generated listings stay correct at every commit `##documents-stay-current`

In a project whose gates run the checker on each commit, every reference in the texts the checker
reads resolves, every register entry has the shape its register declares, and every generated
listing matches what regenerating it writes, on the main branch and on every commit of a branch that
merges. It is met while the checker's verdict fails on a tree where any of these fails, and on a
range for each commit where any fails. It refines
`goal@knowledge-architect@documentation-stays-consistent`.

## The rules a project's documents must meet, compiled in or declared in its manifest, are enforced by the checker `##declared-instructions-are-checked`

A project states in its manifest what it records, where each kind of record lives, and the kinds and
the anchors by which one record points at another, in the reference grammar every project shares,
beside the rules the core compiles in. A tree that passes the checker conforms to all of it in the
texts the checker reads, and no project can hold part of it back, beyond the paths its manifest
exempts and the rules a declaration of it switches off, as an empty list of harnesses switches off
the checks of the installed text. Conformance is established by running the checker, not by a
reviewer remembering the rules. It is met while a tree and each commit of a range that the checker
passes conform, in the texts the checker reads, to every rule the core compiles in that no
declaration switches off, every rule the manifest declares and the reference grammar. It refines
`goal@knowledge-architect@documentation-stays-consistent`.

## Relocating or refactoring what exists costs at most one manifest edit, and no edit to a document that cites it `##relocation-is-one-manifest-edit`

Every reference to what the manifest places names its anchor, and each anchor's directory is
declared once, in the manifest, so moving a Component or a location changes the manifest,
refactoring a register's home from one file to a directory changes no file outside the home and no
manifest line, and neither changes a document that cites what moved. Renaming a Component's
directory renames its anchor, which its basename names, and is no move. It is met while such a move
or change of shape leaves no reference to repair. It refines
`goal@knowledge-architect@documentation-stays-consistent`.

## A recorded decision or an outstanding issue is delivered to whoever needs it, through checked references `##records-reach-their-reader`

For any entry, the checker lists every text of the tree it reads that references it, and it lists
every standing issue and tripwire of the project, so a session reading a decision meets what stands
against it, and a reversal comes with the list of what it touches. It is met while the first listing
holds every reference in the texts of the tree the checker reads, and the second every standing
issue and tripwire, both computed from the tree, never kept by hand. It refines
`goal@knowledge-architect@structure-and-workflow-work-together`.

## The documents render as one linked site `##documents-render-as-a-linked-site`

A preprocessing pass rewrites every reference into a link and an mdbook build renders the result,
hosted with the project once it is released. Dense cross-referencing is what makes the rendered site
worth reading, so every citeable thing having one reference form serves this goal directly. It is
met while a build renders knowledge-architect's Markdown documents that the checker reads into one
hosted site, each reference that resolves rendered as a link to its target.

## A project adds checks of its own through a documented extension API `##projects-add-their-own-checks`

A project whose conventions the built-in checks do not cover writes its own checks as an extension,
against a library API documented with its contract and its examples, and its checker binary runs
them in the same subcommands, with the same report, as the built-in ones. It is met while an
extension needs no change to the core and uses only its public, documented API. It refines
`goal@knowledge-architect@agents-work-without-drift`: a project's own checks guard its own design
intent.
````

### crates/agent-skills/docs/goals.md

````markdown
# Goals — agent skills

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@agent-skills@design`. A goal stays here while it is met, and leaves only
when the owner abandons it. This component is published, so its goals are encouraged to refine a
goal of the project as a whole, `register@knowledge-architect@goal`.

## Every activity of the workflow has one installed skill `##one-skill-per-activity`

An activity is the scope over which a complete set of procedures makes sense, a sub-activity
included, as `skill@knowledge-architect-agent-configuration@where-text-goes` uses the term; two
tasks whose methodologies differ, their steps taken as a whole, are two activities, even where they
share part of a name, and are not bundled in one skill; where the two tests disagree, the
methodologies decide. The installed set holds one skill for each activity the workflow asks an agent
to carry out in every project that installs it, writing the code itself excepted, and the agents
those skills dispatch, and an agent carries out each activity from its skill, reading another
installed skill only at a step that points to it, a moment of the primer's list of installed skills
included; the two are one intent, since a skill per activity serves only where the activity is
carried out from it. The activities include setting up a project, setting its goals, discussing
designs, planning, recording decisions, tracking open issues, reviewing, auditing a project on one
aspect by the project audit's method, maintaining the agent configuration, the retrospective, and
the intake of a retrospective's findings; the list grows with the workflow. It is met while the
installed set holds a skill for each such activity and no skill for two of them, and each skill
states every step its activity needs, leaving to judgement what judgement handles, and points to
another installed skill only at a step that needs it. It refines
`goal@knowledge-architect@agents-get-a-complete-workflow`.

## No two installed instructions contradict `##no-contradiction-in-workflow`

The installed texts, the skills, the agent definitions, the saved workflows and the primer, give no
two instructions a session cannot both obey; a precedence an installed text states between the two
leaves the session the move the prevailing one asks, which counts as obeying both. It is met while
every pair of installed instructions that meet at one moment leaves a session a move that obeys
both, the move a precedence names counting as one. A pair found in use that leaves none shows the
goal unmet in practice. It refines `goal@knowledge-architect@agents-get-a-complete-workflow`.

## A review or an audit judges a text by the criteria its writer is asked to meet `##reviews-judge-by-the-writer-s-criteria`

A review or an audit the installed text owes or offers judges a text written under a skill,
installed or the project's own, by the criteria that bind its writer at the time of the review, and
by no other: the installed text, the project's root `CLAUDE.md`, its skills and the scoped
`CLAUDE.md` beside what is edited, the reasons recorded beside the code and the agent-facing text,
and the goals, the design homes, the rejected designs, the tripwires, the issues and the external
specifications they make binding. The criteria of the reviews and of the audits agree with each
other. It is met while each criterion a review or an audit the installed text owes or offers applies
to such a text is one that binds its writer at that time.

## An installed text works unchanged in any project that installs it `##installed-text-works-anywhere`

The installed skills, agents, saved workflows and primer name no path or convention particular to
one adopting project, and write the project's command as the project declares it. It is met while
they name none, and write that command, so that a project of any language and of any layout its
manifest declares can follow them without editing them. It refines
`goal@knowledge-architect@any-project-can-adopt-it`.

## An installed text works in any environment that loads it `##installed-text-works-in-any-environment`

The installed text depends on nothing outside the installed set, the project and the harness's own
mechanisms, and on no skill or instruction outside the first two: in any environment that loads it,
a session with no user-level instruction and no plugin reaches each installed skill at its moment,
and a skill or an instruction the environment adds, outside the installed set and the project, never
takes the place of an installed skill or instruction for an activity the set covers. It is met while
the installed text routes a session to each installed skill at its moment, and no installed text
lets a skill or an instruction the environment adds take the place of an installed one for the same
activity. A session routed otherwise because of its environment's own instructions shows the goal
unmet in practice.

## The installed text holds only the instructions the workflow needs, and leaves to the session's judgement the cases it handles well enough `##only-needed-instructions`

The installed text, its skills, agents, saved workflows and primer, states what an agent needs to
carry out the workflow as intended, with the reason for each instruction. It does not try to cover
every case: a case the judgement of the session following it handles well enough gets no
instruction, since each instruction added for a specific case is read by every session that loads
the text and can conflict with the instructions already there. Nor does it impose structure the work
does not need: a fixed sequence, count or mapping between a unit of the work and a unit of its
record that another instruction needs to vary. It is met while each installed instruction names an
act the workflow needs, and the installed text imposes no fixed sequence, count or mapping between a
unit of the work and a unit of its record that another instruction needs to vary. A review that
finds an instruction for a case judgement handles, or a structure another instruction needs to vary,
shows the goal unmet in practice, and is answered by removing it rather than by adding a rule. It
refines `goal@knowledge-architect@agents-work-without-drift`.
````

### crates/gates/docs/goals.md

````markdown
# Goals — gates

What this component is for. A goal is met or unmet; a decision about how the component is built
lives in `register@gates@design`. A goal stays here while it is met, and leaves only when the
owner abandons it. This component is published, so its goals are encouraged to refine a goal of
the project as a whole, `register@knowledge-architect@goal`.

## A Rust project gets its merge gates by writing only its list of gates `##gates-from-a-list`

A Rust project that adopts knowledge-architect runs every gate of its list through this library,
writing no gate logic beyond that list and the call that runs it: the runner, the complete logs, the
distilled failures and the verdict come with it. It is met while the library provides each of these,
so that a maintenance binary using it holds no gate logic beyond its list and the call that runs it:
no runner, no log, no distiller and no verdict of its own. It refines
`goal@knowledge-architect@setup-brings-quality-tools`.
````

### tools/xtask/docs/goals.md

````markdown
# Goals — xtask

What this tool is for. A goal is met or unmet, where a decision about how the tool is built is
won or lost and lives in `register@xtask@design`. A goal stays here while it is met, and leaves
only when the owner abandons it. This tool serves the project rather than its consumers, so its
goals need not refine a goal of `register@knowledge-architect@goal`.

## Every gate the project owes before a merge runs from one command `##one-command-runs-every-gate`

One command runs every gate of the project's list, formatting, the document check, the commit
messages, the linter and the tests among them, and the rebase check, which it runs when asked. By
default it runs them all when one fails, and it gives one verdict as its exit code, on the working
tree and the branch's commits, which says whether the gates let the branch merge. It is met while
the command's list holds each of these gates, and the continuous integration runs the same command.

## A mechanical task this repository's own instructions ask repeatedly is a command of the tool `##repeated-tasks-are-automated`

A task with no judgement that takes several steps, local to this repository, one its own
instructions ask or the procedure they fix for one the installed text asks, at a recurring moment,
such as each commit, each reword or each merge, becomes a command of the tool, so it runs the same
way each time. A task the installed text asks of every project that adopts the workflow, with no
procedure of this repository's own, is the published checker's, per
`goal@knowledge-architect@mechanical-tasks-are-the-tool-s`. It is met while each such task that this
repository's root and scoped `CLAUDE.md` files and its `klarch-` skills and agents ask, or whose
procedure they fix, has a command of the tool. Such a task left to be done by hand shows the goal
unmet in practice.
````
