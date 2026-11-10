# The review of the whole goal set after the rewrite under the standard of three kinds

The record of the whole-set review that `spec@goals-at-the-root@standard-and-rewrite` runs over
the 21 goals of `register@knowledge-architect@goal`, per
`skill@knowledge-architect-goal-setting@whole-set-review`. It leaves in the commit that writes the
goals the owner ruled last, or a later commit of the same branch.

The owner's statements and rulings on these goals are in the analysis beside this file,
`path@knowledge-architect@docs/goal-reviews/2026-11-10-goals-at-the-root-analysis.md`, in its
section "The owner's rulings", verbatim, and in `milestone@plans@goals-at-the-root`, in its
sections "Threads" and "Arguments". The analysis also holds, for each of the 54 goals that the
rewrite started from, one row per sentence with where it went: a reviewer reads every row of the
old goals its file's goals come from, not only the rows sent nowhere.

## The sets

One set per file of the goals home: the documentation model; what is built on the model; the
workflow; the two together; the owner and the agents; whom the project serves, and adoption.

## Source marks

Each goal is drafted from the owner's statements where the analysis quotes some, and otherwise
proposed from the old goal of the same slug as the owner had ruled it under the former standard.
The new first goal, `goal@knowledge-architect@documentation-has-a-structure-and-a-model`, is from
the owner's statements alone. The head of the goals home is the owner's paragraph, tightened for
form.

## Questions open

After round 1, the 17 questions R1 to R17 of the consolidation below, each put to the owner with
its default. The goals in the tree keep their wording on each of them until the owner answers.

## Rounds

### Round 1

Seven goal reviewers, each fresh and in a worktree detached at the commit of the review's record:
one per file in draft mode, and one over the whole set. Their reports follow as returned, the
boundary material of each goal included.

#### The consolidation of round 1, and the outcome of each finding

One agent merged the seven reports: 78 findings as reported, 55 after merging. It checked every
quotation against the goals and the rulings. Its text follows. The outcomes:

- the settled repairs S1 to S6, S8 to S10, S12, S14 to S16 and S18 to S22 are written in the goals
  home by the commit that writes this section;
- S13 is not applied: it changes the words of a row the analysis keeps, "Real use is the test",
  and goes to the owner with R8;
- S7, S11, S17 and S23 take no edit, for the reason each gives;
- the 17 questions R1 to R17 are open, and the goals keep their wording on each;
- the 11 findings dropped are judged to need nothing, each on the ruling or the rule quoted.

Read whole: docs/goals/ (README.md and six files), the section "A goal, and what is not one" of the goal-setting skill, "## The owner's rulings" of the analysis (every subsection), and "### Round 1" of the review record (lines 35 to 1618). 78 findings as reported; 55 after merging.

Verification. Every quotation of a goal matches docs/goals/. Every quotation of the rulings section matches it. No finding is dropped for a wrong quotation. Two citations of the analysis are inexact, and one "no finding" is wrong:
- Whole-set 18a says old row 4 of `goal@knowledge-architect@design-with-agents-as-counterparts` "stays in the goal" whole. The row keeps it only 'for "with its consequences, and its rival where one stands"'. So "of the exchange" is not a kept clause (see R13).
- The adoption reviewer says the drop of "which is knowledge-architect's owner's" is settled by the triage of Q51. Q51 itself rests on the words being kept: 'row 29d of `goal@knowledge-architect@serves-one-kind-of-project` keeps "the order of known work, which is knowledge-architect's owner's"'. The whole-set reviewer's 19a is right (S25).
- Statements of the owner quoted from the milestone document (#arg-3, #arg-27, #arg-31, #arg-57, #arg-58, D3) were checked only where the analysis quotes them. All those matched.

Reviewer keys below: M = model, B = built on the model, Wf = workflow, T = together, O = owner, A = adoption, W = whole set.


##### Part 1: Settled repairs

"Moved from intent" marks a finding a reviewer routed `intent` that I place here because a ruling, a kept row or a sibling goal fixes the answer. Check each of those before applying.

The head (docs/goals/README.md)

S1. Head, definition of the record. T-H1 and O-H-1 (rule 3, asks; moved from intent). "its goals included" reads two ways. Three goals list the goals apart from the record (`goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@the-owner-s-word-is-verified`), so the reading is fixed.
- Current: "Nothing else is part of it, its goals included;"
- Replacement: "Its goals are no part of it, and neither is anything else;"

S2. Head, shared words with no definition. W-H3 (rule 3, minor, settled) and B tasks-3 for "a tool" (rule 3, asks). The triage of Q61 sent a definition of "a tool" to the head; the head holds none. The analysis's draft read: "A tool is something knowledge-architect publishes for a person or an agent working in a target project to run, other than the text it installs."
- Add to the list of definitions: "- **a tool**: something a person or an agent working in a target project runs, other than the installed text;" and "- **a session**: one run of an AI agent on a project's work;"
- Current: "such as citations, design entries and issues" Replacement: "such as references, design entries and issues"
- Consequence to check: this definition answers B tasks-3. An installed agent or a saved workflow is then no tool of `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`.

S3. `goal@knowledge-architect@records-reach-their-reader` and `goal@knowledge-architect@documentation-has-a-structure-and-a-model`: the definition of a reference is taken by a pointer. M-8 (rule 8, asks, intent) and W-3a (rule 8, asks, settled). Four goals use the word.
- Add to the head: "- **a reference**: a pointer by which one text names a recorded item;" (its width waits for R2).
- model.md, current: "A reference is a pointer by which one text names a recorded item: it is written in one readable form, the same in every project, in any text the checker reads, and it says what it names, with one reading." Replacement: "A reference is written in one readable form, the same in every project, in any text the checker reads, and it says what it names, with one reading."
- model.md, delete from `goal@knowledge-architect@records-reach-their-reader`: ", a reference being what `goal@knowledge-architect@documentation-has-a-structure-and-a-model` says it is" (full sentence in S6).
- The other shape: the skill says "A definition that is the outcome of one goal is stated in that goal instead." If the definition is read as the first goal's outcome, it stays there and only the pointer leaves.

docs/goals/model.md

S4. `goal@knowledge-architect@documentation-has-a-structure-and-a-model`: the declaring reads as an obligation over every kind. M-3 (rule 4, asks; moved from intent). The rulings send out of the goals "the sentence that a project declares to the checker what it records, which `design@check@nothing-of-a-project-is-compiled-in` argues". The owner's words are a capability: "allow declaring various types of entries and citing them".
- Current: "and a project declares the kinds of entries it keeps under it, those of its own included."
- Replacement: "and a project can declare kinds of entries of its own under it."

S5. Same goal: "judges" has two objects in two goals. M-2 and W-1b (rule 3, asks; moved from intent). The old goal fixed the sense: "To judge what a document says is to report that it breaks a rule on its content". The rulings put "the reading that judges nothing" in this goal.
- Current: "The model judges none of what the documents say."
- Replacement: "The model reports no breach of a rule on what a document says."
- Not settled by this edit: which side reports a reference that names nothing, or a slug defined twice. That is a boundary between two parts, so a head closes it.

S6. `goal@knowledge-architect@records-reach-their-reader`: two statements, one home and the citing. M-9 (rule 5, minor, settled).
- Current: "In a target project, each recorded item has one home. It is cited at each place where it bears on other work and where a project may write a reference the checker checks, a reference being what `goal@knowledge-architect@documentation-has-a-structure-and-a-model` says it is."
- Replacement: "In a target project, each recorded item is stated in one home, and is cited, never copied, at each place where it bears on other work and where a project may write a reference the checker checks."

S7. `goal@knowledge-architect@design-is-recorded-with-its-arguments`: "What is outstanding is recorded" is a second statement. M-7 (rule 5, minor, settled). No edit proposed: the only repair is a goal of its own, which changes the list of 21. See R17.

docs/goals/built-on-the-model.md

S8. `goal@knowledge-architect@projects-add-their-own-checks`: "publishes" reads as the published library API. B-1 and W-5a (rule 3, asks; moved from intent). Ruling 2 of the eight decides: "whether a published library API is part of the purpose; default, no, the API is design", answered "2. agreed on default".
- Current: "It adds them with what knowledge-architect publishes, and with no change to knowledge-architect."
- Replacement: "It adds them with what knowledge-architect provides, and with no change to knowledge-architect."

S9. `goal@knowledge-architect@documents-render-as-a-linked-site`: the body reads as one site for all projects. W-6b (rule 3, minor, settled).
- Current: "The documents of every target project render as one site, in which each reference is a link to what it names, so a reader follows a reference by a link."
- Replacement: "The documents of each target project render as one site of that project, in which each reference is a link to what it names, so a reader follows a reference by a link."

S10. `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`: "a task of the workflow" bounds less than the owner's framing. B tasks-4 (rule 3, asks; moved from intent), with the reviewer's note on "its owner" in the title. The agreed list title is "a task a tool does at a lower cost, or better, is the tool's", "widened", and the owner's framing has no bound: "tools are built upon the model (or outside of it) to make agents more efficient at searching, updating, and verifying knowledge".
- Title, current: "A task of the workflow that a tool does at a lower cost, or with a better result, is done by a tool knowledge-architect provides, once its owner has taken the task in". Replacement: "A task that a tool does at a lower cost, or with a better result, is done by a tool knowledge-architect provides, once knowledge-architect's owner has taken the task in".
- Body, current: "The workflow asks tasks of the agents of a target project." Replacement: "The agents of a target project carry out tasks, those the workflow asks among them."

S11. Same goal: the condition "that takes several steps or a script the session would write" is gone. B tasks-1 and W-7a ((b), blocks). No edit. The list row says "widened" and the owner agreed the list. The triage of Q59a calls the condition "a threshold" that "Rule 3 of the standard leaves ... to a head", and asks only that the wider reach be shown. It is shown in Part 3.

docs/goals/workflow.md

S12. `goal@knowledge-architect@only-needed-instructions`: "its activities" in the title. Wf-3 (rule 3, asks; moved from intent). `goal@knowledge-architect@agents-get-a-complete-workflow` limits the workflow to activities taken in, so one reading stands.
- Title, current: "The workflow states what an agent needs to carry out its activities, and leaves every other case to the agent's judgement". Replacement: "The workflow states what an agent needs to carry out the activities it covers, and leaves every other case to the agent's judgement".

S13. `goal@knowledge-architect@the-workflow-improves-through-real-use`: "the test" reads as the only admitted evidence. Wf-8 and W-12a (rule 3, asks; moved from intent). The triage of Q29 is "ANSWERED": the rule on mock runs is its head's, so the goal must not carry it outright.
- Current: "It is the test of how an agent behaves under the workflow."
- Replacement: "It shows how an agent behaves under the workflow."
- Caution: row 1 of the old goal keeps the words "Real use is the test". The edit changes a kept row's wording to fit the ruled fate of row 2.

S14. `goal@knowledge-architect@session-orchestrates-the-work`: "acting on what comes back" is dropped. Wf-10 ((b), blocks). Row 6 reads "stays in the goal". The other side: the owner's statement holds no such clause and ends "Nothing more detailed", and the whole-set reviewer finds the goal conforming.
- Current: "It orchestrates agents it dispatches to carry the work, so that it can focus on deciding the design with that person, and on reporting each decision taken."
- Replacement: "It orchestrates agents it dispatches to carry the work, and acts on what comes back, so that it can focus on deciding the design with that person, and on reporting each decision taken."

docs/goals/together.md

S15. `goal@knowledge-architect@agents-work-without-drift`: the path of change covers "a recorded decision" only. T-D2 ((b), blocks). Row 2, kept, gave the path for every part of the recorded design. For the goals, `goal@knowledge-architect@the-owner-decides` holds it. For a reason beside code or agent-facing text, no goal does.
- Current: "A recorded decision changes only by a change that reverses it on purpose and argues the reversal."
- Replacement: "A recorded decision, or a reason recorded beside code or agent-facing text, changes only by a change that reverses it on purpose and argues the reversal."

S16. `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` and the head: the clause "it serves each target project that carries out its activity, or is meant to" is in neither. T-G2 and W-H2 ((b), blocks). Row 2 reads "stays in the goal".
- Current: "so that every target project gets it."
- Replacement: "so that every target project that has a use for it, or is meant to, gets it."

S17. Same file, `goal@knowledge-architect@agents-work-without-drift`: the ranking is a second intent. T-D1 and W-14a (rule 5, minor, settled). No edit; a ruling answers it. See the dropped list.

docs/goals/owner.md

S18. `goal@knowledge-architect@the-owner-decides`, first paragraph. Four findings, one replacement:
- O-D-7 (rule 3; moved from intent): "a project that follows the workflow" is defined nowhere, and `goal@knowledge-architect@serves-one-kind-of-project` says "Every other goal that speaks of a project ... means a target project".
- O-D-6 (rule 3; moved from intent): row 7 keeps the member "a goal to set or change"; the draft names goals only as an authority. I did not re-read row 7.
- O-D-8 and O-V-2 ((b), blocks): "and its measurements" dropped; row 2 reads "stays in the goal".
- O-D-3 (rule 3; moved from intent): a reading of a goal that no argument ranks. The owner agreed the proposal "where no argument ranks two readings that move in different directions, it goes to the owner" with "1 agreed on the reporting of a goal reading".
- Also the two-word alignment the owner reviewer names: old row 6 is "a contradiction that no source or argument resolves".
- Current: "In a project that follows the workflow, the owner states what the project is for and brings the facts only the owner holds. Agents decide, by argument, what the goals, the record, the code and the sources the project follows settle. The owner decides the rest: a choice between shapes that all survive the argument, and a contradiction no argument resolves."
- Replacement: "In a target project, the owner sets and changes the goals, which state what the project is for, and brings the facts only the owner holds. Agents decide, by argument, what the goals, the record, the code and its measurements, and the sources the project follows settle. The owner decides the rest: a choice between shapes, or between readings of a goal, that all survive the argument, and a contradiction no source or argument resolves."

S19. `goal@knowledge-architect@the-owner-decides`, third paragraph: "durable text" has two senses in the project. O-D-1 (rule 3, blocks, settled). The head's "record" holds the reasons beside code and in commit messages, which is the old goal's sense.
- Current: "Each decision the agents make that earns durable text, and each choice they make that the owner could have ruled, reaches the owner"
- Replacement: "Each decision the agents make that enters the record, and each choice they make that the owner could have ruled, reaches the owner"
- R11 bears on the second half of the same sentence.

S20. `goal@knowledge-architect@the-owner-s-word-is-verified`: "and its measurements" dropped. O-V-2 ((b), blocks), same drop as in S18.
- Current: "it checks what the word assumes against the project's goals, its record, its code and the sources it follows."
- Replacement: "it checks what the word assumes against the project's goals, its record, its code and its measurements, and the sources it follows."

docs/goals/adoption.md

S21. `goal@knowledge-architect@serves-one-kind-of-project`: "targets" has a second sense, and "glue" has none. A-1 (rule 3, blocks, settled).
- Current: "the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need."
- Replacement: "the code it builds and ships is Rust, built by Cargo, apart from the code in another language that a platform it is built for requires."
- Wording, same item (A-2's note): "Files the workflow installs" has instructions install files. Replacement, if R14 keeps the clause: "Files knowledge-architect installs".

S22. Same goal: "which is knowledge-architect's owner's" is dropped. W-19a ((b), blocks). Row 29d reads "stays in the goal", and the triage of Q51 rests on it. Without it no goal of the 21 says whose the order of known work is.
- Current: "and for the order of known work."
- Replacement: "and for the order of known work, which is knowledge-architect's owner's."

S23. `goal@knowledge-architect@every-target-project-can-adopt-it` (A-7) and `goal@knowledge-architect@adoption-is-easy` (A-9), rule 5, minor, settled. No edit proposed: A-7 goes to R17 and A-9 follows R16.


##### Part 2: Questions for the owner (17)

The head

R1. The scanner (W-H1).
(1) The head says the starting point is "a documentation structure that it defines, a model that represents that structure, and a scanner". Is a scanner part of what the project is for, or the means of building the model, as "in Rust" was ("design under the scope goal")?
(2) A model built from a project's documents by another means than a scan of the tree, such as through git's object store.
(3) Default: kept, the words are yours. If it is a means, "and a scanner" leaves the head.

model.md

R2. `goal@knowledge-architect@documentation-has-a-structure-and-a-model`: what a reference is (M-1, W-1a).
(1) In the goals, is a reference a pointer to a recorded item only, or also to a file, a directory or a section of a text?
(2) A pointer to a file, or to a section of a skill. Under the draft it is no reference: the one form "with one reading", the model's "what each reference names", the site's links and the citing of `goal@knowledge-architect@records-reach-their-reader` do not reach it.
(3) Default proposed: the wide one, "a pointer by which one text names a recorded item, a file, a directory or a section of a text". That is the old definition, which the analysis sent to a head. If narrow: the text stays.

R3. `goal@knowledge-architect@design-is-recorded-with-its-arguments`: the commit message (M-5, W-2a).
(1) Is "and in the message of its commit" part of the outcome, or a means? The goal says the reason "is recorded beside that code or that text and in the message of its commit".
(2) A decision whose reason stands in a comment at its code, with a commit message silent on it. With "and", it fails the goal.
(3) Default: kept (row 2 of the old goal). If a means: the sentence ends at "beside that code or that text", and a head says where else.

built-on-the-model.md

R4. `goal@knowledge-architect@documentation-stays-consistent` and `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`: may a shipped tool judge meaning? (B consistency-1, W-4a, B tasks-2)
(1) The first goal says the checker and the listing tools "as knowledge-architect ships them ... judge no meaning". The analysis found that "no word of the owner in the milestone document states it". Is that an outcome you hold, or a decision an argument may reverse?
(2) A shipped check that compares what a design head states with the code, by a model's reading; or a shipped tool that grades whether a document agrees with the code, taken in because it "reaches a better result".
(3) Default: kept as a scope of that goal. Then the tasks goal says so too, since "where the judgement stays the agent's" covers only its second class. If a decision: the sentence leaves for a head.

R5. `goal@knowledge-architect@documents-render-as-a-linked-site`: hosting (B site-1, W-6a, B site-2).
(1) The old goal said the site is "hosted with the project once it is released". The draft says "A project can host that site with what it releases." Is hosting part of the outcome, or is the ability enough? Your ruling 1 widened whose documents render and says nothing on hosting.
(2) knowledge-architect's own site at a release; a private target project that releases nothing.
(3) Default: "can host", reworded to one reading: "A project can host that site when it releases." If hosting is owed: "The site is hosted with the project once it is released."

workflow.md

R6. `goal@knowledge-architect@agents-get-a-complete-workflow`: what "outside" means (Wf-1).
(1) The goal says an agent "needs no instruction from outside" the workflow and what its project adds. Does that rule out an installed instruction that sends the agent to a text knowledge-architect publishes and does not install, and one that defers by choice to an outside instruction?
(2) The Migration entries of the changelog at a pin move; an installed instruction that defers to a harness or plugin instruction (the old goal admitted it, and that row went to a design issue).
(3) Default proposed: "from outside what knowledge-architect provides and what its project adds", which admits the first case and rules out the second. If both are ruled out, the text stays.

R7. `goal@knowledge-architect@the-workflow-leads-to-better-code` and `goal@knowledge-architect@projects-are-split-into-modular-units`: "learned from use" (Wf-6, W-10a).
(1) The code goal says "what makes code better is learned from use". The analysis sent the two sentences behind it to heads, and kept only "this goal defines no quality". The split goal keeps the parallel clause by its row 7. Does the clause stay in both, or leave both?
(2) No case differs in what an agent does; the point is whether a goal states how the project learns, which rule 7 gives to design.
(3) Default: kept in both. If it leaves: "This goal fixes no definition of quality." and "so this goal fixes no rule of splitting" stand without the clause.

R8. `goal@knowledge-architect@the-workflow-improves-through-real-use`: the last sentence (Wf-9, W boundary note).
(1) "This goal forbids no change that is argued to lead to a better result in what a target project implements or records." It comes from no row; the analysis routed your D3 words to a head. Does the goal admit a change on argument alone against that head, or is it only silent?
(2) An instruction on behaviour, argued, with no evidence from use, against `design@agent-skills@behaviour-additions-need-real-use`. One agent reads the head as yielding to the goal. Another reads the head as still binding.
(3) Default proposed: delete the sentence; with S13 the goal no longer reads as a bar. If it is a licence: it stays, and the head is rewritten under it.

owner.md

R9. `goal@knowledge-architect@the-owner-decides`: actions a project reserves (O-D-2).
(1) "The owner decides the rest:" names two members and reads closed. The sentence "A project may reserve further actions to its owner" went to a head. Does the goal admit what a project reserves?
(2) A project whose instructions reserve the merge to its owner, where the argument settles that the branch is ready. One agent merges; another waits.
(3) Default: no change, read as admitted, since the project's own instruction is among what the argument reads (the whole-set reviewer reads it so). If you want it stated: one clause, "and what the project reserves to its owner".

R10. `goal@knowledge-architect@the-owner-decides`: what counts as an ask (O-D-4).
(1) knowledge-architect's owner decides "each addition, tightening or removal of what knowledge-architect asks of every target project", "Whatever the argument settles". The bounds of "asks" left for a design issue. Does it reach a new rule of the checker, a new or tightened instruction of the workflow, and a gate?
(2) An agent adds a rule the checker holds for every project, on a settled argument. Under the wide reading it waits for you; under the narrow one it is reported.
(3) Default proposed: narrow, the conditions and asks of `goal@knowledge-architect@serves-one-kind-of-project`; the old text said "a procedure of the installed workflow is none". Wide: the text stays and each such change waits.

R11. `goal@knowledge-architect@the-owner-decides`: implementation choices (O-D-5, W-16a).
(1) "each choice they make that the owner could have ruled, reaches the owner". The old exclusion, "other than an implementation choice that a commit message records", went to a head. Does the goal reach every implementation choice?
(2) An agent picks one of two data structures and records why in the commit message. Under the wide reading it owes a report, and the head's exclusion contradicts the goal.
(3) Default proposed: narrow, "each choice they make on a question that is the owner's". Wide: the text stays and the head's exclusion goes.

R12. `goal@knowledge-architect@design-with-agents-as-counterparts`: changing the direction (O-C-1).
(1) The owner can "challenge it, add an argument or a fact, or change the direction of the design". The means, "with a goal, a fact or an argument", went to `design@agent-skills@intent-through-goals-not-overrule`. Can you change the direction by a bare word?
(2) You say "take the other shape" on a point the argument settled, with no reason. One agent complies. Another asks for the goal, the fact or the argument. Your words: "If I judge a cost too large, i can overrule a decision without needing a goal."
(3) Default: kept, read as through a goal, a fact or an argument, a cost you weigh being an argument. If a bare word suffices: the head is reversed, and `goal@knowledge-architect@the-owner-decides` gains a member.

R13. `goal@knowledge-architect@design-with-agents-as-counterparts`: "each proposal" (O-C-2, W-18a).
(1) "The owner meets each proposal with its grounding and its consequences established". The old text said "each proposal of the exchange". Does the goal reach a design an agent closes on its argument during autonomous work?
(2) A design settled by argument, with no fork, built and reported under "Decided". Wide: you are owed it with its rival while you can still challenge it. Your words: "If there is nothing else I can give other than approval, there is little value in bring the question to me."
(3) Default proposed: restore "each proposal of the exchange". Wide: the text stays.

adoption.md

R14. `goal@knowledge-architect@serves-one-kind-of-project`: two clauses that settle border cases (A-2, A-3, W-19b).
(1) "Files the workflow installs, the configuration of a continuous integration and scripts kept beside the code are no code it ships" and "An edit that a person makes now and then leaves it a target project". Do they stay in the scope, or does a head hold them?
(2) A project whose maintenance scripts are Python; a project whose owner writes one commit in twenty by hand. With the clauses the goal settles both; without them an argument may reopen each.
(3) Default: kept, since they bound the kind. If they leave: both sentences are deleted.

R15. `goal@knowledge-architect@serves-one-kind-of-project`: "a state that follows none" (A-4).
(1) The goal "builds nothing only to serve a state that follows none". Is that a state that follows no ask at all, or a state that does not follow one of the asks? The old words were "a state that does not follow an ask". You took the default of question 7 in these same words.
(2) A target project under git and `claude` that keeps its own gate runner beside the published gates library. Under "no ask at all", support built only for that runner is not ruled out.
(3) Default proposed: "builds nothing only to serve a state that does not follow an ask". If "none at all": the text stays.

R16. `goal@knowledge-architect@adoption-is-easy`: "Adopting asks little work of them." (A-8, A-9, W-21a)
(1) The sentence comes from no row and no statement of yours; the old body spoke of documentation only. Is little work an outcome you hold?
(2) An existing project with many documents to move. If yes, the goal asks for means that carry the move, each ask of the scope goal weighs against it, and nothing ranks it against what adoption asks beyond checks. If no, the goal asks only that the move be explained.
(3) Default proposed: delete the sentence; the goal is then one statement. If kept: it needs a ranking against `goal@knowledge-architect@every-target-project-can-adopt-it`.

Several files

R17. Rule 5 and the list of 21 (M-7, A-7, W-16b).
(1) Three goals hold a statement that means something without the rest: "What is outstanding is recorded" in `goal@knowledge-architect@design-is-recorded-with-its-arguments`; "Each target project moves to a new version of knowledge-architect when it chooses" in `goal@knowledge-architect@every-target-project-can-adopt-it`; the paragraph on knowledge-architect's own owner in `goal@knowledge-architect@the-owner-decides`. Do they stay where the list of 21 has them?
(2) No case differs in what an agent does. A split makes up to 24 goals.
(3) Default: they stay, as the fates were ruled, and no text changes.


##### Part 3: The reach of each goal

The head
- Requires: a defined documentation structure and a model of it.
- Admits: "tools are built upon the model, or outside of it".
- Rules out: nothing by its own words.
- May not intend: "a recorded item" no longer excludes a plan document, so a plan's items take a name and one home; "a scanner" read as bound (R1).

`goal@knowledge-architect@documentation-has-a-structure-and-a-model`
- Requires: each recorded item, a goal included, takes a name; a reference has one form and one reading in any text the checker reads; a tool learns whether its read is complete and right.
- Admits: a program other than knowledge-architect's own reading the model; a model in any language as far as this goal goes; a tool that reads past the model.
- Rules out: a reference form particular to one project; a reference with two readings; a model that reports a breach of a rule on content.
- May not intend: pointers to files and sections left outside "reference" (R2).

`goal@knowledge-architect@design-is-recorded-with-its-arguments`
- Requires: a decision recorded in an entry names the alternatives that lost and why; a reason the code does not show is beside the code and in the commit message; what is outstanding is recorded.
- Admits: any threshold between an entry and a comment; a record kept in an external tracker, as far as this goal goes.
- Rules out: a decision recorded in an entry with its conclusion alone.
- May not intend (M-6): a project that records no decision in an entry meets the first sentence; "any other decision" reaches every implementation choice whose code does not show its reason; a decision with no code and no entry is asked nothing; an entry owes losing alternatives even where none was weighed.

`goal@knowledge-architect@records-reach-their-reader`
- Requires: one home per recorded item; a citation wherever it bears and a checked reference can stand; a change to an item can list the texts that cite it.
- Admits: no citation where no checked reference can be written, such as installed text for a project's entry; any means of listing.
- Rules out: an item copied in a second home; an item not cited where it bears and a reference may stand.
- May not intend: a goal is a recorded item, so each goal is owed a citation wherever it bears, code comments included.

`goal@knowledge-architect@documentation-stays-consistent`
- Requires: a tree with a reference to a missing entry fails the checker; a breach of a rule the project declares fails it.
- Admits: a mechanical rule over content; a project's own check that judges meaning; a breach in a text the checker does not read; a rule switched off by a declaration.
- Rules out: a shipped check or listing tool that judges meaning (R4); conformance taken from a reviewer's memory.
- May not intend: whether a check a project adds counts among "every rule the project declares" is not said; the goal says nothing of when the checker runs.

`goal@knowledge-architect@projects-add-their-own-checks`
- Requires: a project's check runs and reports with the checker's, with no change to knowledge-architect.
- Admits: a check only one project uses; any form of the means, after S8.
- Rules out: a check that needs a patched knowledge-architect; a check that reports apart from the checker's run.
- May not intend: a check that needs the project's build or tests to run is still owed a place in the checker's run.

`goal@knowledge-architect@documents-render-as-a-linked-site`
- Requires: in every target project, each reference renders as a link to what it names. Nothing of it exists today.
- Admits: any rendering tool; a project that never hosts its site, under "can".
- Rules out: a render for knowledge-architect's repository only; a resolved reference rendered as plain text.
- May not intend: "the documents" has no bound, so plan documents, open issues and installed skills render and, where hosted, are published; a private project is owed the same site.

`goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`
- Requires: for a task taken in, a tool that does it, so each session gets the same result.
- Admits: any form of the tool; a tool that reads history or past sessions; a task that meets the test and is never taken in.
- Rules out: a task counted as taken in because the workflow asks it; installed text that has each session write its own script for a task taken in.
- May not intend: a task of one step is now a candidate, since "several steps or a script" is gone (S11); searching "what its past sessions said" is in reach; a tool that itself judges (R4).

`goal@knowledge-architect@agents-get-a-complete-workflow`
- Requires: instructions for each listed activity. "working on a branch under a linear history" and "releasing" have no installed skill today.
- Admits: releasing instructions installed in a project that never releases; the harness's own mechanisms; "the list grows".
- Rules out: an activity carried out only with a plugin's skill or a user-level instruction.
- May not intend: a pointer to the changelog or the README counted as "outside" (R6); the brief a main session writes for a dispatched agent read as an outside instruction; the retrospective and its intake held at a goal's weight.

`goal@knowledge-architect@only-needed-instructions`
- Requires: nothing built; it bounds.
- Admits: an instruction for a case judgement handles poorly; a rewording that removes an ambiguity and adds no instruction.
- Rules out: an instruction added because one session lapsed once; a pass that adds a sentence to every instruction with two readings.
- May not intend: it bounds instructions that serve the owner goals and the drift goal too ("whichever goal the instruction would serve"); a fixed sequence or count is now refused only by a head.

`goal@knowledge-architect@the-workflow-leads-to-better-code`
- Requires: that what the workflow says about code and tests aims at quality, tests and maintenance tools included.
- Admits: any shape of guidance; code that departs from a default shape with its reason.
- Rules out: a fixed definition of quality.
- May not intend: without "Rust", the aim reaches glue, scripts and CI configuration; guidance on Rust idiom is excluded only by a head.

`goal@knowledge-architect@projects-are-split-into-modular-units`
- Requires: that the workflow leads to units of one purpose for the project's parts, the code of one part, and its design documents by subject.
- Admits: a project of one purpose kept whole; any form of unit.
- Rules out: a rule of splitting that a unit fails alone.
- May not intend: the three layers read as a closed set, or as examples that reach tests, plan documents and agent-facing text; the goal now grounds heads on code inside one crate, where the old one bound Components only.

`goal@knowledge-architect@the-workflow-improves-through-real-use`
- Requires: a route from a session in any target project, one you do not own included, to knowledge-architect.
- Admits: any means of collecting; a change made on argument alone (last sentence, R8).
- Rules out: on one reading of "the test", evidence other than real use (S13).
- May not intend: a session with no supervising person is no real use, while the drift goal moves toward "less supervision"; every lapse found read as "missing".

`goal@knowledge-architect@session-orchestrates-the-work`
- Requires: work carried by dispatched agents; the main session's focus on the design and on reporting decisions. The installed text departs today.
- Admits: the main session reading one known file itself; any threshold of kind or size.
- Rules out: a main session that carries all the work as the standing shape.
- May not intend: a harness with a far larger context still owes the dispatch; "that person" is whoever supervises, not only the owner.

`goal@knowledge-architect@agents-work-without-drift`
- Requires: a change that contradicts a recorded decision reverses it on purpose with an argument, or is not built; a design not kept returns only by a reopening on purpose.
- Admits: any means of catching a departure; any amount of supervision left; an agent reversing a decision on argument.
- Rules out: a silent reversal; a gain in autonomy that takes a decision from the project's owner, or skips the exchange or the check of the owner's word.
- May not intend: it ranks before every other purpose of the 21, not one; a reason held only in a commit message is not in its list; the ranking names the project's owner, not knowledge-architect's owner on asks and taking in.

`goal@knowledge-architect@generic-tools-and-procedures-are-shipped`
- Requires: a generic part the owner took in ships for target projects; what is particular stays in its project.
- Admits: a generic part never taken in; any form of shipping; any delay; a project that keeps its own copy.
- Rules out: shipping a part because it is generic; shipping what is particular to one project.
- May not intend: "every target project gets it" read as delivered into projects with no use for it (S16); a reviewer or a publishing method owed by this goal and by the workflow goal at once.

`goal@knowledge-architect@the-owner-decides`
- Requires: a fork and an unresolved contradiction go to the owner; an action that cannot be undone waits for the project's owner; a new ask or a taking in waits for knowledge-architect's owner; each recorded decision of an agent reaches the owner.
- Admits: any form and moment of the report; the owner contesting after the merge.
- Rules out: an agent decision the owner cannot see; an irreversible action on argument alone.
- May not intend: a reserved merge (R9); each checker rule and workflow instruction as an ask (R10); every implementation choice reported (R11); the taking in of an activity or a task is not named.

`goal@knowledge-architect@the-owner-s-word-is-verified`
- Requires: the agent reads the code before acting on a word that assumes something about it; nothing is built on an unresolved contradiction.
- Admits: the rest of the work going on; any means of the check; any occasion for checking a goal's premises.
- Rules out: acting on an unchecked word; building on an unresolved contradiction.
- May not intend: the goal asks no check of what a goal assumes, at any moment ("other than a goal"); the check reaches a bare pick at a fork; the installed text asks less today.

`goal@knowledge-architect@design-with-agents-as-counterparts`
- Requires: the agent challenges the owner's design on its failures and costs; its own proposal comes with grounding, consequences, measurements and its rival.
- Admits: any moment and form of the exchange; the owner picking at a fork with no argument.
- Rules out: a proposal whose consequences the owner must work out; an owner's proposal built unchallenged.
- May not intend: a bare change of direction (R12); every design closed by argument owed to the owner with its rival (R13); it names the owner alone where two goals name a supervising person.

`goal@knowledge-architect@serves-one-kind-of-project`
- Requires: work judged against the four conditions, not against the projects that exist.
- Admits: a web target's glue; a person's occasional edit; a project not yet a git repository; use elsewhere with no guarantee; removing an instruction because no current project needs it.
- Rules out: a mode for another language; merge commits; instructions for a project persons drive; a checker mode for another harness; narrowing the kind on what today's projects need.
- May not intend: any step toward a second harness is ruled out until the ask changes; a defect where an ask is not followed is covered only by inference; where the .git sits is a head's reading; a Rust project whose owner has not decided is no target, so nothing is built to win it.

`goal@knowledge-architect@every-target-project-can-adopt-it`
- Requires: nothing particular to one project in what is provided; a new project served first; an existing one can adopt.
- Admits: adoption over several branches; a project that stays on an old version.
- Rules out: a forced version move; softening a check the workflow needs on an existing project's cost.
- May not intend: no exception is left for knowledge-architect's own tree (the pin value "self"); a security fix cannot be forced; "served first" has no bound.

`goal@knowledge-architect@adoption-is-easy`
- Requires: each step of adopting written in documentation, where an adopter looks, in plain words with examples.
- Admits: several places of documentation; an adopter who reads the source by choice; any number of commands.
- Rules out: a step stated only in the source; a step learned only by asking knowledge-architect's owner.
- May not intend: "little work" as an outcome of its own (R16); "its documentation" reaches the installed setup skill, which `goal@knowledge-architect@only-needed-instructions` bounds.


##### Part 4: Counts

As the reports give them

| reviewer | total | by rule | by grade | by route |
| --- | --- | --- | --- | --- |
| model | 9 | r2 1, r3 2, r4 2, r5 2, r8 1, (b) 1 | blocks 1, asks 6, minor 2 | intent 6, settled 2, contradiction 1 |
| built on the model | 9 | r2 1, r3 6, (b) 2 | blocks 2, asks 7, minor 0 | intent 7, settled 0, contradiction 2 |
| workflow | 10 | r2 1, r3 4, r4 2, r6 1, (b) 2 | blocks 2, asks 8, minor 0 | intent 8, settled 0, contradiction 2 |
| together | 5 | r3 1, r5 1, (b) 3 | blocks 3, asks 1, minor 1 | intent 1, settled 1, contradiction 3 |
| owner | 13 | r3 8 (one with a conditional r6), r4 3, (b) 2 | blocks 3, asks 10, minor 0 | intent 10, settled 1, contradiction 2 |
| adoption | 9 | r3 4, r4 1, r5 2, (b) 2 | blocks 3, asks 4, minor 2 | intent 4, settled 3, contradiction 2 |
| whole set | 23 | r2 3, r3 10, r5 2, r6 2, r8 1, (b) 5 | blocks 5, asks 14, minor 4 | intent 13, settled 5, contradiction 5 |
| sum | 78 | | blocks 19, asks 50, minor 9 | intent 49, settled 12, contradiction 17 |

After deduplication: 55 findings (23 merges absorbed).
- By highest grade given: blocks 14, asks 34, minor 7.
- Part 1: 23 findings. 21 with an edit (S1 to S6, S8 to S10, S12 to S16, S18 to S22; S18 holds four), 2 with no edit because a ruling decides for the draft (S11, S17). 10 of the 21 are moved from intent.
- Part 2: 21 findings in 17 questions.
- Dropped: 11.

Dropped, with the reason

1. M-4, "a model ... which its tools read" as every tool through the model. A ruling answers: "tools are built upon the model (or outside of it)".
2. M-6, the reach of the two tiers. No question: its open point is the threshold between an entry and a comment, a question of how much the goal leaves open. Its four cases are shown in Part 3.
3. B consistency-2 and W-4c, "save those the project switches off by a declaration". The clause admits a switch-off and requires none; cutting it would close the question the other way. A question of how much, left open. `design@check@the-regime-has-no-opt-out` holds it.
4. Wf-2, the retrospective and its intake in the list. A ruling answers: the goal "takes in the list of activities of `design@agent-skills@one-skill-per-activity`", and the owner: "1. I think this is right."
5. Wf-4 and W-9a, `goal@knowledge-architect@only-needed-instructions` against `goal@knowledge-architect@the-workflow-improves-through-real-use`. A ruling answers: the scope "bounds the installed text whichever goal an added instruction would serve"; the owner: "1. I think this is right.", and "The point is that the installed workflow should not grow without bound." A scope bounds a purpose and owes no ranking.
6. Wf-5, "better code" without "Rust". The agreed list title is "the workflow leads agents to write better code": "2. agreed on those 6 files and 21 goals in the list." The reach to glue and scripts is in Part 3.
7. Wf-7, the three layers as a set or as examples. Both readings move the same way and differ in how far: a question of how much, left open.
8. T-D1 and W-14a, the ranking inside `goal@knowledge-architect@agents-work-without-drift`. A ruling answers: the list row is "`goal@knowledge-architect@agents-work-without-drift`, with its ranking", and "2. agreed on those 6 files and 21 goals in the list."
9. T-G1, "declared or configured beside what knowledge-architect ships". Where the particular part is declared is a question of by what means. The reviewer says so itself. The clause was a kept row, so name the drop when the goal is shown.
10. O-V-1, "other than a goal". The old text said when a goal's premises are checked; that went to design. The draft leaves when open, and its first sentence still covers every word. Shown in Part 3.
11. A-5, where the .git sits, and A-6, the defect where an ask is not followed (two findings). Ruling 8: "the `claude` harness and git; default, both are scope, named in the scope goal", "8. default": git is named, and where its directory sits is a question of how much. Ruling 7: "7. default", on a default sentence that holds the same words as the draft and no clause on a defect (analysis, "After the rulings": "it guarantees its working only as far as a project follows what it asks, and builds nothing only to serve a state that follows none, other than what helps a project into conformance").

##### Decided

- Ten findings routed `intent` are placed in Part 1: S1, S4, S5, S8, S10, S12, S13 and three of S18. Ground, each stated at its item: a ruling, a kept row of the analysis, or a sibling goal fixes the answer.
- Two (b) findings graded `blocks` take no edit (S11) or are dropped (A-5, A-6, T-G1). Ground: a ruling decides for the draft, or the dropped clause closes a question of when, how much or by what means.
- Rule 5 findings whose only repair is a split go to one question, R17, not to Part 1. Ground: a split changes the list of 21 the owner agreed.
- B tasks-2 and the consistency goal's rule 2 finding are asked once, as R4. Ground: both turn on whether a shipped tool may judge meaning.

##### Met outside the task

- `design@git@repository-variables-dropped` states a reading of the scope goal on where the .git sits. Whether it was reported under "Decided" as a goal reading is for the session to check (adoption reviewer).
- `design@agent-skills@the-workflow-teaches-no-rust-idiom` quotes `goal@knowledge-architect@only-needed-instructions` and `goal@knowledge-architect@the-workflow-leads-to-better-code`; it is re-read if either changes (workflow reviewer).
- A line of the agent-skills design, project-configuration.md line 335, uses "durable text" in the sense S19 removes (owner reviewer). Not opened by me.
- The supervising person who is not the owner, named three ways across the goals, belongs to `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` (owner and whole-set reviewers). Not opened by me.
- The holders reading's question 6, which goal holds the form of a citation, has no ruling; the draft answers it by placing the sentence in the first goal (model reviewer).
- The brief given to the reviewers said a goal states "no list of cases" under the standard. The words are in the section "The shape of a goal entry", not in the eight rules; two reviewers noted the mismatch.

#### The reviewer of: The documentation model

Tree read: worktree `/home/teo/dev/knowledge-architect/.claude/worktrees/r1-model`, HEAD the commit under review. Nothing edited, no file written.

What I established from the tree:

- `cargo klarch anchors` lists 13 goals homes. `register@knowledge-architect@goal` holds the head and six files with 21 goals (3, 4, 6, 2, 3, 3). The 12 Component homes hold no `## ` entry (`grep -c '^## '` gives 0 for each).
- Read whole: the agent definition on disk, `register@knowledge-architect@goal` and the six files, the review record, the section "The owner's rulings" of the analysis (lines 17 to 261), and the rows of the old goals this file comes from: `goal@knowledge-architect@design-is-recorded-with-its-arguments`, `structure-and-workflow-work-together`, `records-live-beside-the-code`, `goal@knowledge-architect@records-reach-their-reader`, `a-tool-reads-the-documents-with-no-check`, `a-tool-knows-the-read-is-right`, `recorded-items-can-be-referenced`, with row 1 of `documents-stay-current` and of `document-rules-are-checked`, and the post-ruling sections "Rows that name a goal as their holder" and the triage of Q1 to Q17.
- Milestone document: I read the thread titles and searched "Threads" and "Arguments" for the owner's words on the model, citing and the record. I did not read those two sections line by line (assumption: no other statement of the owner there bears on these three goals; the only one found is #arg-31, the "documentation model and checker" example, which the analysis quotes).
- The review record holds no open question and no round, so nothing is a repeat.

##### `goal@knowledge-architect@documentation-has-a-structure-and-a-model`

**Finding 1. Contradiction (b), no rule, `blocks`, `contradiction`.** The draft holds a definition that the analysis sent to a design head, and narrows it.

- Draft: "A reference is a pointer by which one text names a recorded item".
- Old goal `documents-stay-current`, row 1: "A reference is a pointer by which one text names a recorded item, a file, a directory or a section of a text." Destination: "existing head: `design@model@one-entity-table`".
- The rulings: "every fate is ruled as proposed but those the next two rulings change". The ruled list gives the first goal "the naming and citing of a recorded item", and names no definition.
- The other side of the record: the unruled post-ruling table "Rows for the new first goal" says of this row "the definition of a reference, or the home's head", and the theme's "Texts above the first goal" says "A definition of 'a reference' may join them" (the head's definitions).
- Cases the narrowing changes: a a path reference reference to a file or a directory, a a register reference reference, a a skill reference or a primer reference reference. Under the draft none is "a reference". So "one readable form, the same in every project", "with one reading" and "what each reference names" in this goal do not reach them. Nor do "each reference is a link to what it names" in `goal@knowledge-architect@documents-render-as-a-linked-site` and the citing of `goal@knowledge-architect@records-reach-their-reader`.
- For the owner: is a reference, in the goals, a pointer to a recorded item only, or also to a file, a directory or a section?

**Finding 2. Rule 3, `asks`, `intent`.** "The model judges none of what the documents say" reads two ways beside the next sentence, "A tool can tell whether what it read is complete and right, and what is wrong where it is not."

- The old goal drew the line with a definition: "To judge what a document says is to report that it breaks a rule on its content, such as a section an entry owes" (`a-tool-reads-the-documents-with-no-check`, row 3). It went to `design@knowledge-architect@model-crate`.
- Case on which readings differ: one slug defined twice, or a declared home that is missing. The old `a-tool-knows-the-read-is-right`, row 1, counts both as the read being wrong. One agent keeps their detection in the model as "what is wrong". Another moves it out as a judgement on what the documents say.
- Sibling collision: `goal@knowledge-architect@documentation-stays-consistent` says the checker and the listing tools "judge no meaning", and "A mechanical rule ... judges none". If "judges none of what the documents say" is read as "judges no meaning", the sentence no longer separates the model from the checker.

**Finding 3. Rule 4, `asks`, `intent`.** "a project declares the kinds of entries it keeps under it, those of its own included" can require a project to declare every kind it keeps, the workflow's included.

- Owner, #arg-31: "allow declaring various types of entries and citing them", a capability.
- Case: the kinds every project gets without declaring them (decisions, designs not kept, goals, tripwires, issues). `path@model@docs/design/anchors.md` line 21 says "the five built-in component registers in `manifest.rs` belong to no project".
- For the owner: does the goal ask that a project declare those too, or only admit that it declares its own?

**Finding 4. Rule 3, `asks`, `intent`.** "a model of that documentation, which its tools read" can be read as every tool of knowledge-architect reading the documents through the model.

- The head, the owner's paragraph: "tools are built upon the model, or outside of it".
- Old `a-tool-reads-the-documents-with-no-check`: "can read it through one library", and row 7, "Admits, for instance: a tool that scans a document's parse again for a subject of its own, or that reads a file past the library", sent nowhere as repeating the statement.
- Case: a tool of knowledge-architect that parses a project's documents itself, past the model. Is it ruled out or admitted?

**No finding:**

- Rule 1: a purpose. The owner: "The starting point is that we define a documentation structure, a model representation in Rust and a scanner."
- Rule 2: the goal names no built part. "In Rust", "one library" and "or why it names nothing" are absent, as ruled ("design under the scope goal"; `design@model@resolver-outcome`).
- Rule 5: structure, naming and citing, the model, the reading that judges nothing and the tool that can tell the read is right were each put in this one goal by the ruled list and by the default of question 4 of the eight.
- Rules 6, 7 and 8: no conflict, no measure, no citation.
- For information: the owner's "and a scanner" is in the head and not in this goal. The list title the owner agreed to holds no scanner.

**Boundary material:**

- Asks of future work: keep a defined documentation structure with kinds a project declares, one reference form for all projects, and a model of the documentation that knowledge-architect's tools read, which reports whether the read can be trusted and reports no breach of a rule on content.
- Requires: every recorded item, a goal included, takes a name. A reference has one form and one reading in any text the checker reads, a code comment and a commit message included. The model gives what is declared, what the documents hold, the items defined and what each reference names. A tool learns that a read is incomplete or wrong, and what is wrong.
- Admits: a program other than knowledge-architect's own reading the model (question 4: "other programs being admitted and not served"); a model in any language, as far as this goal goes (Rust follows from the scope goal); a tool that shows one entry without asking whether the read is right.
- Rules out: a reference with two readings; a reference form particular to one project; a model that reports a breach of a rule on content.
- Cases the owner may not intend: findings 1, 3 and 4.
- Siblings:
  - `goal@knowledge-architect@records-reach-their-reader` takes the definition of a reference from this goal by citation.
  - `goal@knowledge-architect@documents-render-as-a-linked-site` and `goal@knowledge-architect@documentation-stays-consistent` use "reference" and "cited" in this goal's sense with no definition of their own.
  - `goal@knowledge-architect@every-target-project-can-adopt-it`: its analysis draft held "A recorded item is cited in one readable form, the same in every project". The tree holds that sentence here and not there. The holders reading's question 6, "which one holds the form of a citation?", was not among the eight put to the owner.
  - `goal@knowledge-architect@projects-add-their-own-checks` and `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` are built on the model. No conflict.
  - `goal@knowledge-architect@serves-one-kind-of-project` gives "project" its meaning and the language.

##### `goal@knowledge-architect@design-is-recorded-with-its-arguments`

**Finding 5. Rule 2, `asks`, `intent`.** "is recorded beside that code or that text and in the message of its commit" states where a reason is written.

- Under the test of rule 2: were the reason kept beside the code only, would the project still be for what the owner intends?
- Case: a decision whose reason stands in a comment at its code, with a commit message silent on it. The goal's "and" makes that a failure.
- The sentence comes from the old goal (row 2, "stays in the goal"), ruled under the former standard. The owner has no statement on it under the new one.

**Finding 6. Rule 4, `asks`, `intent`.** The reach of the two tiers, which the triage of Q3a left to this round ("The reach of the two-tier draft is shown in the reviewer's rounds").

- Draft: "Each decision the project records in an entry of its own carries its intent, its argument, and the alternatives that lost with why they lost. The reason of any other decision, where its code or its agent-facing text does not show it, is recorded beside that code or that text and in the message of its commit."
- (a) The goal does not say which decision gets an entry. A project that records no decision in an entry meets the first sentence.
- (b) "any other decision" reaches every implementation choice whose code does not show its reason, each owing a comment and a commit message.
- (c) A decision with no code and no text, such as a decision about an absence or a policy, that has no entry: the goal asks nothing of it.
- (d) An entry owes "the alternatives that lost with why they lost" even where no alternative was weighed.
- The owner has no statement on the threshold ("None on the threshold between an entry and a comment", the analysis says).

**Finding 7. Rule 5, `minor`, `settled`.** "What is outstanding is recorded" means something without the rest.

- The title and the first sentence state the design recorded with its arguments. A defect or missing work outstanding is no decision and no argument.
- The closing sentence, "what it may change, and what a change costs", does not need it.
- Repair implied: a goal of its own, or a statement and title that this intent qualifies. Which one is the owner's.
- "the evidence that would flip a recorded decision" qualifies the decisions and is no second intent.

**No finding:**

- Rule 1: a purpose.
- Rule 3: one direction.
- Rule 6: no conflict.
- Rule 7: the measure rows 6 to 10 are gone.
- Rule 8: no citation.
- Kind (b): rows 1 to 5 are kept, "with why they lost" included. The sentence of `records-live-beside-the-code` ("never in an external tracker") is absent, as the rulings send it to "a new design head of the root's design home".

**Boundary material:**

- Asks of future work: lead a target project to record what it decided and why, what is outstanding, and the evidence that would flip a decision, so a later session neither derives an argument again nor reverses a decision blind.
- Requires: a recorded decision names the alternative that lost and why; a choice whose reason the code does not show has it in a comment and in the commit message; an outstanding defect is recorded.
- Admits: a decision and the designs it did not keep in separate documents; any threshold between an entry and a comment; the record kept anywhere, an external tracker included, as far as this goal goes.
- Rules out: a decision recorded in an entry with its conclusion alone.
- Cases the owner may not intend: finding 6, (a) to (d).
- Siblings:
  - `goal@knowledge-architect@agents-work-without-drift` relies on what this goal has recorded. It names "the reasons recorded beside its code and its agent-facing text" and no commit message.
  - `goal@knowledge-architect@the-owner-decides` ("each decision the agents make that earns durable text") shares the aspect. No conflict.
  - The head's definition of "the record" lists the same parts and adds "the designs it did not keep". This goal does not use the term.
  - `goal@knowledge-architect@only-needed-instructions` bounds the installed text, not the record. No conflict.

##### `goal@knowledge-architect@records-reach-their-reader`

**Finding 8. Rule 8, `asks`, `intent`.** The goal takes a term from a sibling goal by citation.

- Draft: "a reference being what `goal@knowledge-architect@documentation-has-a-structure-and-a-model` says it is".
- The citation is one rule 8 allows, a goal of its own Component. Rule 8 also says "A goal holds what it needs in its own words."
- The goal's intent holds without the other goal once the word is defined, so it is no refinement.
- Case for the owner: should the definition of "a reference" stand in the head of the home with the other shared words? The analysis proposed that, and three goals of two files use the word. Or does this goal state it itself? It turns on finding 1.

**Finding 9. Rule 5, `minor`, `settled`.** "each recorded item has one home" and "It is cited at each place where it bears on other work" each mean something without the other.

- One home is no duplication. Citing is reach.
- The old goal held both in one sentence (row 1, "stays in the goal"), and the holders reading kept "has one home" here on purpose.
- Repair implied: two goals, or a wording in which one home is the qualifier of the citing (cited, never copied). Which one is the owner's.

**No finding:**

- Rule 1: a purpose, the citing half of the owner's "declaring various types of entries and citing them".
- Rule 2: "a reference the checker checks" names what the head defines.
- Rule 3: "where it bears on other work" is broad in one direction.
- Rule 4: the bound "and where a project may write a reference the checker checks" is the old goal's.
- Rules 6 and 7: no conflict, no measure. The reading and the two review questions (rows 7 to 9 and 12) left for `issue@agent-skills@the-review-does-not-ask-whether-items-are-cited-where-they-bear`, as the triage of Q9 says.
- Kind (b): row 1 is carried whole. Row 3, naming and the reference's form, is in the first goal, as the list rules. Row 17 is in `goal@knowledge-architect@documentation-stays-consistent`.

**Boundary material:**

- Asks of future work: each recorded item is stated once and cited wherever it bears and a checked reference can stand, so a session working there meets it and a session changing it can list what to read again.
- Requires: a code comment at a guard that exists because of an issue cites that issue; a design head cites the goal it derives from; a change to an item can list every text that cites it.
- Admits: any means of listing the citing texts; a place where no checked reference can stand, such as installed text for an entry of the project, cites nothing; a reason recorded beside code, which the head says is no recorded item, is cited nowhere.
- Rules out: a recorded item copied in a second home; an item that bears on a place where a checked reference may stand and is not cited there.
- Cases the owner may not intend:
  - a goal is a recorded item, so the goal asks that each goal be cited wherever it bears, in code comments included;
  - under finding 1, a file or a directory is not reached by this goal.
- Siblings:
  - the first goal: definition, finding 8.
  - `goal@knowledge-architect@documentation-stays-consistent` ("nothing cited there is missing") keeps the citations valid.
  - `goal@knowledge-architect@documents-render-as-a-linked-site` makes each a link.
  - `goal@knowledge-architect@agents-work-without-drift` is served by it.
  - `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` covers "searching ... its recorded items".
  - No conflict with any.

##### Met outside the task

- `goal@knowledge-architect@documents-render-as-a-linked-site` and `goal@knowledge-architect@documentation-stays-consistent`, in `path@knowledge-architect@docs/goals/built-on-the-model.md`, use "reference" and "cited" with no definition in their file or in the head. They depend on the body of the first goal. For the reviewer of that file and the whole-set reviewer.
- "judge": `goal@knowledge-architect@documentation-stays-consistent` uses "judge no meaning" for the checker, beside the first goal's "judges none of what the documents say" for the model (finding 2). The collision spans two files.
- The brief says the standard has a goal state "no measure and no list of cases". The standard on disk (rule 7) states no measure and has no rule on a list of cases, so I judged none. The enumeration in the first goal ("what the project declares, what its documents hold, the items they define and what each reference names") defines the model, which rule 2 admits.
- The holders reading's question 6, which goal holds the form of a citation, has no ruling in "The owner's rulings". The draft answers it by placing the sentence in the first goal.

##### Counts

- Findings: 9.
- Per rule: rule 2: 1. Rule 3: 2. Rule 4: 2. Rule 5: 2. Rule 8: 1. No rule, kind (b): 1. Rules 1, 6 and 7: 0.
- Per route: `intent` 6, `settled` 2, `contradiction` 1.
- Per grade: `blocks` 1, `asks` 6, `minor` 2.

#### The reviewer of: What is built on the model

Tree read: worktree `/home/teo/dev/knowledge-architect/.claude/worktrees/r1-built`, detached at the commit under review. Nothing edited, no file written in the worktree.

##### State of the tree against the brief

- `cargo klarch anchors` lists 13 goals homes: `register@knowledge-architect@goal` and one `goals.md` per Component (check, agent-skills, gates, mutate, xtask, git, model, cli, transcript, query, create, testing). The 12 Component homes hold no level-two heading (`grep -c '^## '` gives 0 for each). The brief agrees.
- The root's home holds 21 goals: model 3, built-on-the-model 4, workflow 6, together 2, owner 3, adoption 3. The brief agrees.
- Read whole: my definition on disk (.claude/agents/knowledge-architect-goal-reviewer.md), the six goal files and the home's head, the review's record (it holds no open question and no round yet), the section "The owner's rulings" of the analysis, the analysis's sections for the old goals this file comes from (`goal@knowledge-architect@documentation-stays-consistent`, `documents-stay-current`, `document-rules-are-checked`, `goal@knowledge-architect@projects-add-their-own-checks`, `goal@knowledge-architect@documents-render-as-a-linked-site`, `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`, `agents-query-the-project`, `the-record-is-consulted-as-the-workflow-requires`), the part "After the rulings" for the rows that lost their holder and the triages of the themes "The record and its documents" and "The tools", and the milestone's "Threads" whole and "Arguments" from arg-1 to arg-89. Arguments after arg-89 were searched for the words of this file's goals (tool, site, render, checker, extension, mechanical, consistent), not read whole: no owner's statement there bears on these four goals.
- No goal of the batch cites anything, so rule 8's citation rule has nothing to judge here.

---

##### `goal@knowledge-architect@documentation-stays-consistent`

Origin: the old goal of the same slug, with `documents-stay-current` and `document-rules-are-checked` merged in (the merge "stands" in the rulings' table, minus the sentence "A project declares to the checker what it records").

**Finding 1. Rule 2, `asks`, `intent`. A sentence states a division of work as the outcome.**
Goal: "The checker and the tools that list the recorded items, as knowledge-architect ships them, support them and judge no meaning, whatever a project's own checks may do."
The outcome in the title is that the documentation stays consistent. That the shipped checker judges no meaning is the choice of who judges. The analysis says of it: "No head argues it, and no word of the owner in the milestone document states it" (Q6), and the triage leaves it to this review: "Whether it is an outcome or a mechanism is the test of rule 2, which the goal reviewer applies. A removal would go to the owner in the rounds."
The case on which the readings differ: knowledge-architect ships a check that compares what a design head states with the code, by a model. If the sentence is an outcome, that check is ruled out for good. If it is a mechanism, it is a design decision an agent may argue. The owner's example B (arg-31) says only "these model and checker are supposed to serve a project keeping design documentation up to date with code". The old goal, ruled under the former standard, held the same sentence, so rule 4 is not in question.

**Finding 2. Rule 3, `asks`, `intent`. A clause closes a question of how much.**
Goal: "save those the project switches off by a declaration".
The triage (Q11a): "What a project may switch off is a question of how much, so the clause ... is the reviewer's to keep or cut." It also records that `design@check@the-regime-has-no-opt-out` says "There is no flag, no list and no severity", and that the only declaration found that removes an obligation is the harness list.
The case: an agent proposes a manifest key that switches off one rule the checker holds for every project. With the clause, the goal's guarantee already makes room for it, and only the head stands against it. Without the clause, the goal would not speak of it. The owner says whether the clause is part of the intent.

No (b) finding: the rows the analysis keeps in the goal are in the draft (old rows 1, 2, 3, 4, 6, 12; `document-rules-are-checked` rows 2, 3, 4 second part, 5; `documents-stay-current` rows 4, 5). "A tree that passes the checker conforms", which four heads quote, is there verbatim. The sentence the rulings took out, "A project declares to the checker what it records", is out. Rule 5: the second paragraph means something alone, but the owner kept the merge (arg-59, and the rulings' table), so I report no finding. Rule 7: "Conformance is established by running the checker, not by a reviewer remembering the rules" states what the checker is for, not what shows the goal met.

**Boundary block material**
- Asks of future work: keep reviews and audits as the judges of meaning; keep the shipped checker's verdict equal to conformance to every rule, in the texts it reads.
- Requires: a tree with a reference to an entry no document defines fails the checker. A breach of a rule the project declares fails it.
- Admits: a rule over content that needs no judgement, such as the goal's own example of a flag. A project's own check that judges meaning ("whatever a project's own checks may do"). A pointer that resolves to nothing in a text the checker does not read.
- Rules out: a shipped check, or a shipped listing tool, that judges meaning (finding 1). A conformance that rests on a reviewer's memory of the rules.
- Cases the owner may not intend: the guarantee names "every rule the project declares", and `goal@knowledge-architect@projects-add-their-own-checks` has a project "add checks of its own"; the text does not say whether a check a project adds counts as a rule it declares, so whether "a tree that passes the checker conforms" covers them is open (for information).
- Siblings touched: `goal@knowledge-architect@documentation-has-a-structure-and-a-model` ("The model judges none of what the documents say": same exclusion, for the model); `goal@knowledge-architect@projects-add-their-own-checks` (the exception "whatever a project's own checks may do"); `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` (see its finding 2: a tool that judges); `goal@knowledge-architect@agents-work-without-drift` (shares the aspect of work staying within what is recorded, and ranks first); `goal@knowledge-architect@agents-get-a-complete-workflow` (reviewing and auditing are listed activities); `goal@knowledge-architect@every-target-project-can-adopt-it` ("never an argument against a check the workflow needs to work well", which the switch-off clause touches and does not contradict). No pair that cannot both be met.

---

##### `goal@knowledge-architect@projects-add-their-own-checks`

Origin: the old goal of the same slug. Ruling 2 of the eight questions: "a project's own checks: whether a published library API is part of the purpose; default, no, the API is design", answered "agreed on default".

**Finding 1. Rule 3, `asks`, `intent`. Wording two agents read in two directions.**
Goal: "It adds them with what knowledge-architect publishes, and with no change to knowledge-architect."
In this project "publish" is used for crates on crates.io. Reading A: the project builds its checks against published libraries, which keeps the library API in the purpose, against the ruling. Reading B: the project uses whatever knowledge-architect ships, by any means. The case: the design the triage names, "rules a project declares in its manifest" in place of the extension libraries. It meets the goal under B and is doubtful under A. The clause comes from the old row 1, "so it can build it against published versions of them", written when the API was in the goal. The owner says what remains of it: only that a project needs no private or patched copy of knowledge-architect, or more.

No (b) finding: "to guard its own design intent" (old row 4, kept) is there; "run and report with the checker's own" keeps the outcome of row 2; "which it pins" is carried by `goal@knowledge-architect@every-target-project-can-adopt-it` ("moves to a new version of knowledge-architect when it chooses"). The draft says "adds" where the old goal said "writes ... as an extension", which follows the ruling.

**Boundary block material**
- Asks of future work: a project can add a check for a convention of its own without a change to knowledge-architect, and its findings come in the checker's run and report.
- Requires: a project's check of its own naming convention reports with the checker's findings.
- Admits: a check only one project uses; any form of the means (library, declared rule), under reading B.
- Rules out: a project's check that needs a change to knowledge-architect to run. A project's check that reports apart from the checker's run.
- Cases the owner may not intend: a project's check that needs the project's build or its tests to run is still owed a place in the checker's run and report (for the owner's information: the goal says "They run and report with the checker's own" of every check added under it).
- Siblings touched: `goal@knowledge-architect@documentation-stays-consistent` (the exception for a project's own checks; whether they count among "every rule the project declares"); `goal@knowledge-architect@every-target-project-can-adopt-it` ("holds nothing particular to one target project": a project's own check stays in the project); `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` (the generic part of such a check may be taken in; "What is particular to one project stays in that project"); `goal@knowledge-architect@the-owner-decides` (what knowledge-architect asks of every project is its owner's; a project's own check is not such an ask). No conflict.

---

##### `goal@knowledge-architect@documents-render-as-a-linked-site`

Origin: the old goal of the same slug. Ruling 1 of the eight questions: "default, every target project. This is what I intended with that goal from the begining."

**Finding 1. No rule, `blocks`, `contradiction` (b). A kept statement is weakened without being shown.**
Old goal, row 1, sent "stays in the goal": "knowledge-architect's documents render as one site, each reference rendered as a link to its target, hosted with the project once it is released."
Draft: "A project can host that site with what it releases."
The old sentence states that the site is hosted once the project is released. The draft states only that a project is able to host it. The owner's ruling widened whose documents render, and says nothing on hosting. Under the draft, knowledge-architect's own site need not be hosted at any release. The owner says whether hosting is part of the outcome, for knowledge-architect or for every target project, or whether the ability is enough.

**Finding 2. Rule 3, `asks`, `intent`. Wording with two readings.**
Goal: "host that site with what it releases".
Reading A: the site is published together with the project's release. Reading B: the project hosts the site by means of the things it releases. The case: a private target project that releases nothing. Under A the sentence does not apply to it; under B it is not clear what it has.

**Boundary block material**
- Asks of future work: knowledge-architect provides, to every target project, a render of its documents as one site in which a reference is a link.
- Requires: in any target project, a reference to a decision in another entry's text renders as a link to that decision.
- Admits: a site built by any rendering tool; a project that never hosts its site (under the draft's "can").
- Rules out: a render for knowledge-architect's own repository only; a resolved reference rendered as plain text.
- Cases the owner may not intend, listed for the owner: "The documents" is not bounded. The analysis notes that the documents the checker reads take in plan documents, installed skills and open issues, so a hosted site of a project publishes them unless a design head says otherwise. The site of a private project. Which documents render is a question of how much, so it is no finding.
- Siblings touched: `goal@knowledge-architect@documentation-has-a-structure-and-a-model` (the definition of a reference, "a pointer by which one text names a recorded item"; the site rests on what the model resolves); `goal@knowledge-architect@serves-one-kind-of-project` ("open source or private"); `goal@knowledge-architect@every-target-project-can-adopt-it` (nothing particular to one project); `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` (its example "a publishing method"). No conflict.

---

##### `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`

Origin: the old goal of the same slug, "widened", with `agents-query-the-project` merged in (the merge "stands"). Ruling 3 of the eight questions: owed "only once the owner has taken the task in", answered "default". The owner's framing: "tools are built upon the model (or outside of it) to make agents more efficient at searching, updating, and verifying knowledge."

**Finding 1. No rule, `blocks`, `contradiction` (b). A condition the analysis keeps in the goal is dropped without being shown.**
Old goal, row 1a, sent "stays in the goal": "a task whose result follows, with no judgement, from the tree ... and that takes several steps or a script the session would write".
Draft: "Where a tool does such a task at a lower cost to the agent than the agent's own means, or reaches a better result".
The condition on several steps or a script is gone. The triage (Q59a) says: "The draft that drops it shows the wider reach to the owner at the draft's ruling, per rule 4." The record shows no such showing yet. What it changes: a task of one step now meets the goal's test, and is owed a tool once taken in. The owner's gate limits the effect, since nothing is owed before the owner takes a task in, but the test that says which tasks are candidates is wider than the one the owner ruled.

**Finding 2. Rule 3, `asks`, `intent`. The second paragraph can be read as a bound or as two examples.**
Goal: "This reaches a task whose result follows with no judgement ... It also reaches searching, updating and verifying what a project holds, ... where the judgement stays the agent's."
The first paragraph's test is a lower cost or a better result. The second names two classes, and in both the tool makes no judgement. The case: a tool that itself judges, such as one that grades whether a document agrees with the code, taken in because it "reaches a better result". Agent A reads the first paragraph and says the goal grounds it. Agent B reads "where the judgement stays the agent's" and says the goal excludes it. `goal@knowledge-architect@documentation-stays-consistent` does not settle it: its "judge no meaning" covers only "the checker and the tools that list the recorded items". The owner says whether a tool of knowledge-architect may judge.

**Finding 3. Rule 3, `asks`, `intent`. "Tool" is not defined, and the readings differ on installed text.**
Goal: "knowledge-architect provides a tool that does it. So each session gets the same result, and none builds its own means again."
The analysis's draft held a definition: "A tool is something knowledge-architect publishes for a person or an agent working in a target project to run, other than the text it installs." The triage (Q61) sent it to the home's head: "Three kept goals of R-list say 'tool', so the definition goes in the home's head." The head of docs/goals/README.md holds six definitions and none of "a tool", and no goal holds one. The case: a task taken in, such as giving every entry one verdict in a search. Agent A meets the goal with an installed agent or a saved workflow, which is installed text. Agent B says only a program meets it. `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` lists "a kind of reviewer" among "tools and procedures", which makes reading A plausible. The owner says whether installed text can be the tool of this goal.

**Finding 4. Rule 3, `asks`, `intent`. "A task of the workflow" may bound less than the owner's framing covers.**
Goal, title and first sentence: "A task of the workflow that a tool does at a lower cost" and "The workflow asks tasks of the agents of a target project."
The owner's framing speaks of tools "to make agents more efficient at searching, updating, and verifying knowledge", with no bound to what the workflow asks, and the merged old goal said "An agent asked to find something in the project can use a tool". The case: a search of the recorded items by keywords that an agent makes during its own work, where no installed instruction asks for that search. Under the title it is outside the goal. Under the second paragraph, "It also reaches searching, updating and verifying what a project holds", it reads as inside. The owner says whether the goal is bounded to what the workflow asks.

Wording note, no finding: the title says "once its owner has taken the task in". "Its" can be read as the target project's owner. The body says "knowledge-architect's owner", so the body settles it; the title could say the same.

No other (b) finding: the gate is in the goal, as ruling 3 says ("and knowledge-architect's owner has taken the task in", and the third paragraph, old rows 3b and 4). The form of the tool, "a command or a library function of a published crate" (row 1b, sent to a design issue), is out. "Those three count as taken in" (row 3a, sent to an issue as the owner's ruling) is out. The clause on when a tool is built (row 5) is out. `agents-query-the-project` rows 1 and 5b are carried by "at a lower cost to the agent ... or reaches a better result". Rule 8: the draft says "a task of the workflow" and names no goal; the old line "It refines" is gone.

**Boundary block material**
- Asks of future work: once knowledge-architect's owner takes a task in, a tool of knowledge-architect does it, so no session builds its own means.
- Requires: for a task taken in, such as folding a repair into the commit it repairs with the check that no content was lost, a tool that does it. For a search taken in, a tool that makes it cheaper or its result better.
- Admits: any form of the tool; a tool that reads what is outside the model, such as the history or what past sessions said; a task that meets the test and that the owner never takes in, which stays with the agent.
- Rules out: installed text that has each session write its own script for a task taken in. A task counted as taken in because the workflow asks it.
- Cases the owner may not intend: a task of one step is now a candidate (finding 1); a tool that judges (finding 2); installed text counted as the tool (finding 3); a search no instruction asks for left outside (finding 4).
- Siblings touched: `goal@knowledge-architect@the-owner-decides` (it gives knowledge-architect's owner "whether and when it takes in the generic part of a tool or a procedure"; this goal takes in "a task", and `goal@knowledge-architect@agents-get-a-complete-workflow` an "activity": three objects of "taken in", each stated in its own goal, with no conflict); `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` (shares the aspect of a tool shipped once taken in; this goal starts from what the workflow asks, that one from what a target project sets up; each means something without the other); `goal@knowledge-architect@agents-get-a-complete-workflow` (the tasks come from it; "needs no instruction from outside them" is about instructions, not tools); `goal@knowledge-architect@documentation-stays-consistent` (finding 2); `goal@knowledge-architect@only-needed-instructions` (a tool that takes a task removes instructions; no conflict); `goal@knowledge-architect@agents-work-without-drift` (ranks first where the two compete); the home's head ("tools are built upon the model, or outside of it"). No pair that cannot both be met.

---

##### Met outside the task

- The triage of the theme "The tools" classed Q61 as settled by the record, with the definition of "a tool" going into the head of the root's goals home. The head at this commit holds no such definition. It is finding 3 above; the dispatcher routes whether the one pass missed it or dropped it on purpose.
- The file's opening line says "The goals on what knowledge-architect builds on the documentation model", and its last goal reaches tools over what the model does not hold ("its history and what its past sessions said"), which the home's head allows ("or outside of it"). The file's title is narrower than its content. Wording only.
- Three goals use "taken in" with three objects: an activity's generic part (`goal@knowledge-architect@agents-get-a-complete-workflow`), a task (this file), the generic part of a tool or a procedure (`goal@knowledge-architect@generic-tools-and-procedures-are-shipped` and `goal@knowledge-architect@the-owner-decides`). `goal@knowledge-architect@the-owner-decides` names only the last. For the reviewer of the whole set.

##### Counts

- Per rule: rule 2: 1. Rule 3: 6. No rule, (b): 2. Rules 1, 4, 5, 6, 7, 8: 0. Total 9.
- Per route: `intent` 7, `contradiction` 2, `settled` 0.
- Per grade: `blocks` 2, `asks` 7, `minor` 0.
- Per goal: `goal@knowledge-architect@documentation-stays-consistent` 2 (`asks`); `goal@knowledge-architect@projects-add-their-own-checks` 1 (`asks`); `goal@knowledge-architect@documents-render-as-a-linked-site` 2 (1 `blocks`, 1 `asks`); `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` 4 (1 `blocks`, 3 `asks`). No goal of the batch is without a finding.

#### The reviewer of: The workflow

Worktree: /home/teo/dev/knowledge-architect/.claude/worktrees/r1-workflow, HEAD the commit under review, tree clean.

##### What I read, and how the tree compares with the brief

- Agent definition from the worktree, /home/teo/dev/knowledge-architect/.claude/worktrees/r1-workflow/.claude/agents/knowledge-architect-goal-reviewer.md: eight rules, as the brief says.
- docs/goals/README.md and the six files, whole. Count of level-2 goals: model 3, built-on-the-model 4, workflow 6, together 2, owner 3, adoption 3 = 21, as the record says.
- The 12 Component goals homes: `grep -c '^## '` over crates/*/docs/goals.md and tools/xtask/docs/goals.md gives 0 for each. I did not run `cargo klarch anchors` (it needs a build); the homes were established by reading the files.
- docs/goal-reviews/2026-11-10-goals-at-the-root-review.md, whole: no question open, no round run.
- The analysis: "The owner's rulings" whole (lines 17 to 261); the theme "The workflow for agents" for the six goals and for `design@agent-skills@one-skill-per-activity`, `no-contradiction-in-workflow`, `installed-text-works-in-any-environment` (lines 1181 to 2247); "Rows that name a goal as their holder" (4445 to 4555); the triage of that theme (4737 to 4835).
- The milestone: #arg-3, #arg-47, #arg-57, #arg-58, #arg-63, #arg-67, #arg-108 and D3, verbatim. I did not read its "Threads" and "Arguments" sections whole.
- No disagreement found between the brief and the tree.

Row numbers below are those of the analysis section headed by the old goal's slug.

---

##### `goal@knowledge-architect@agents-get-a-complete-workflow`

**Finding 1. Rule 4, `asks`, `intent`.** A case the goal rules out that the owner's statements do not cover.
- Goal: "It is complete: an agent carries each activity out with the workflow and what its project adds, and needs no instruction from outside them."
- Head of the home: "**the workflow**, or **the installed text**: the instructions knowledge-architect installs in a target project for its agents".
- Owner (#arg-58): "If the workflow needs instructions from an outside environment to work, it is not complete."
- Cases on which the readings differ:
  - an installed instruction that sends the agent to a text knowledge-architect publishes and does not install: the Migration entries of its changelog at a pin move ("setting up a project" is in the list), or its README. `goal@knowledge-architect@adoption-is-easy` says the adopter "finds what to do in its documentation";
  - an installed instruction that defers by choice to an outside instruction. The old `installed-text-works-in-any-environment` admitted it (row 5b: "unless an installed instruction itself defers to it"), and that row went to a design issue. Under the draft's words a head could not admit it, since a goal binds outright.
- Does the owner rule both out?

**Finding 2. Rule 2, `asks`, `intent`.** Two members of the list may state a mechanism.
- Goal: "- the retrospective; - the intake of a retrospective's findings."
- Analysis, `goal@knowledge-architect@the-workflow-improves-through-real-use`, row 3: "stays in the goal, without naming the retrospective".
- Standard, rule 7: "how the project learns from its use whether it is met, are design decisions".
- Case: the retrospective is the means by which real use reaches knowledge-architect. Named in this list at a goal's weight, it is owed whatever a head on learning from use argues. The other members are activities of a project's own work.
- Does the owner hold these two as activities taken in (triage Q34 reads the old ruled goal so), or as the design of the real-use goal?

**Checked, no finding.**
- The list is admitted by `skill@knowledge-architect-goal-setting@goal-entry-shape` ("a list where it names the members of a set, such as the activities a goal covers"). It has 14 names, the same as the merged list of rows 4 (workflow goal) and 9 (`design@agent-skills@one-skill-per-activity`).
- The draft holds neither the plainness sentence, nor the no-contradiction sentence, nor the Rust-idiom exclusion, nor the CI and pull-request paragraph. Each was sent to a head by the split ruling, point 3, and by the owner's answer 5. `design@agent-skills@the-workflow-teaches-no-rust-idiom` exists in crates/agent-skills/docs/design/development.md.
- Row 3's "for those that carry it out or are meant to" and row 6 of `design@agent-skills@one-skill-per-activity`, "those a session gives another agent included", are not in the draft. I read both as carried by "an activity that only some target projects carry out included" and "needs no instruction from outside them".
- The title reads "a complete workflow for each activity" while the body and the head speak of "the workflow", one text. No two directions follow; a remark for the drafter.

**Boundary block material.**
- Asks of future work: each activity taken in gets its generic part in the workflow, enough that an agent needs nothing from outside the workflow and its project.
- Requires: instructions for "working on a branch under a linear history" and "releasing", which no installed skill holds today (`path@knowledge-architect@docs/open-issues/the-workflow-has-no-skill-for-branch-work-or-releasing.md`).
- Admits: the releasing instructions installed in a project that never releases; reliance on the harness's own mechanisms (triage Q38c, a reading); a second review round that raises quality and costs efficiency, the two aims being unranked inside the goal (a question of how much).
- Rules out: an activity carried out only with a plugin's skill or a user-level instruction.
- Reaches, perhaps unintended: the two cases of finding 1; the brief a main session writes for an agent it dispatches, if read as "an instruction from outside".
- Siblings:
  - `goal@knowledge-architect@only-needed-instructions` bounds it: "complete" is no coverage of every case;
  - `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` shares the aspect "generic part ... taken in", for a tool or a procedure;
  - `goal@knowledge-architect@the-owner-decides` gives the taking in "of a tool or a procedure" to knowledge-architect's owner, and does not name an activity;
  - `goal@knowledge-architect@serves-one-kind-of-project` agrees on linear history and on "no reason to ... limit or remove what knowledge-architect covers";
  - `goal@knowledge-architect@adoption-is-easy`: see finding 1;
  - `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` takes the tasks this workflow asks.

---

##### `goal@knowledge-architect@only-needed-instructions`

**Finding 3. Rule 3, `asks`, `intent`.** The title's first half reads two ways.
- Goal, title: "The workflow states what an agent needs to carry out its activities".
- Old goal, row 1 (kept): "The installed text states what an agent needs to carry out the workflow as intended."
- Reading A: "its activities" are those the workflow covers, and the half is the ground of the scope. Reading B: a requirement that the workflow state what an agent needs for any activity of its own.
- Case: an activity not taken in, such as running continuous integration or the project's domain work. Under B the workflow owes its instructions. Under A, and under `goal@knowledge-architect@agents-get-a-complete-workflow`, it owes none.
- The split ruling, point 1, has these words. The owner's "1. I think this is right" ruled the shape, the wording "going through the goal reviewer".

**Finding 4. Rule 6, `asks`, `intent`.** This scope and the real-use purpose compete under one condition.
- This goal: "it is not a text that gains an instruction for each edge case or lapse found", and "A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve."
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: "What real use shows unclear, missing or wrong in the workflow reaches knowledge-architect and changes the workflow."
- Owner: "the retrospective and the agent text audit can return findings about lots of edge cases, or lapses. Covering every edge cases is impossible".
- Case: a retrospective reports a lapse in a case that judgement usually handles, and names an instruction as missing. An agent reading the real-use goal adds it. An agent reading this goal does not.
- "whichever goal" reads as this scope prevailing, but the real-use sentence is unconditional. Does the scope come first there?
- The same clause reaches instructions that serve `goal@knowledge-architect@agents-work-without-drift`, which "comes first" among purposes, and the three owner goals.

**Checked, no finding.**
- Rule 1: a scope that names the design it excludes, a text that gains an instruction per edge case or lapse.
- The draft holds none of the clause on structure (row 5), the reason of row 4, the protection of rows 8 and 9, the repair of row 12b, or the retrospective question of row 13. Each was sent to a head or an issue.
- "The situations ... have no limit", "edge case or lapse", "no ... ambiguity" and "does not grow without bound" follow the owner's words.

**Boundary block material.**
- Asks of future work: an instruction enters the workflow only where an agent needs it; a case judgement handles well enough gets none.
- Requires: nothing built; it bounds.
- Admits: an instruction for a case judgement handles poorly; a rewording that removes an ambiguity and adds no instruction.
- Rules out: an instruction added because one session lapsed once; a pass that adds a sentence to every instruction with two readings "so that no instruction leaves an ambiguity".
- Reaches, perhaps unintended:
  - an instruction that serves the owner's goals, such as reporting each decision, where judgement is held to handle it;
  - the plainness of an instruction, which is now a head's presumption while this exclusion is a goal's;
  - `design@agent-skills@the-workflow-teaches-no-rust-idiom`, which already rests on "whichever goal the instruction would serve".
- Siblings: bounds `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@the-workflow-leads-to-better-code`, `goal@knowledge-architect@projects-are-split-into-modular-units` and `goal@knowledge-architect@agents-work-without-drift`; competes with `goal@knowledge-architect@the-workflow-improves-through-real-use` (finding 4); agrees with `goal@knowledge-architect@serves-one-kind-of-project` ("That argument still counts for the economy of the workflow's text, for whether an instruction enters it").

---

##### `goal@knowledge-architect@the-workflow-leads-to-better-code`

**Finding 5. Rule 4, `asks`, `intent`.** A widening not shown to the owner.
- Goal, title and body: "better code", "code of better quality in a target project".
- Old ruled goal: "The workflow leads agents to write better Rust code in a target project". The analysis's draft: "aims at Rust code of better quality".
- The rulings' list gives the short title "the workflow leads agents to write better code", "its wording not ruled".
- Case: `goal@knowledge-architect@serves-one-kind-of-project` admits non-Rust files in a target project: "the glue its targets need", "scripts kept beside the code", "the configuration of a continuous integration". Without "Rust", what the workflow says about code is required to aim at their quality too.
- Meant?

**Finding 6. No rule, `blocks`, `contradiction` (b).** The draft holds a statement the analysis sent to heads.
- Goal: "what makes code better is learned from use."
- Row 2: "What makes code better is hard to define before use, and a rule ... too strict ..." has the destination "owner's ruling, into `design@agent-skills@default-code-shape`".
- Row 5: "and real use corrects it." has the destination "existing head: `design@agent-skills@premortem-as-watch-points`".
- Row 3, "So this goal defines no quality:", is the only one of the three that "stays in the goal".
- The other side: the analysis's own draft holds the same sentence, the fates were ruled "as proposed", and row 7 of `goal@knowledge-architect@projects-are-split-into-modular-units` keeps the parallel sentence "what makes a split good is learned from use" in its goal.
- The owner or the drafter says which holds: the sentence leaves, or rows 2 and 5 are read as not covering it.

**Checked, no finding.** Row 1 is kept ("the code of its tests and of its maintenance tools included"). Row 4b, "as a guideline", is absent, as sent to an issue. No measure, no citation.

**Boundary block material.**
- Asks of future work: what the workflow says about code and tests moves toward better code; no definition of quality is fixed.
- Requires: that the workflow says something about code and tests that aims at quality.
- Admits: a change whose code departs from a default shape, with its reason.
- Rules out: a fixed definition of quality that every unit must meet.
- Reaches, perhaps unintended: non-Rust glue and scripts (finding 5). Guidance on Rust idiom, "which `goal@knowledge-architect@the-workflow-leads-to-better-code` could be read to call for" by that head's own words, is excluded only by a head.
- Siblings: `goal@knowledge-architect@agents-get-a-complete-workflow` shares "quality" of the agents' work, with no refinement stated; `goal@knowledge-architect@only-needed-instructions` bounds it; `goal@knowledge-architect@serves-one-kind-of-project` supplies "Rust".
- Source mark: the analysis records no word of the owner on this goal (Q23).

---

##### `goal@knowledge-architect@projects-are-split-into-modular-units`

**Finding 7. Rule 3, `asks`, `intent`.** The three layers read as a closed set or as examples.
- Goal: "A split matters at more than one layer of a target project: the project into its parts, the code of one part, and the documents of its design by subject."
- Owner (#arg-57): "which would bind at more layers than just components: splitting code of one component, as well as splitting files of design docs by subject in one components, are places where splitting matters too".
- Case: a project's own agent-facing text (one skill per activity), its plan documents, its tests. An agent reading the three as the set gives the workflow nothing to say there. An agent reading them as examples leads a split there too.
- The owner's "too" leans toward examples; the colon in the draft leans toward a set.

**Checked, no finding.**
- Rows 3, 6 and 7 are kept. The definition of a Component and the sentence on an intermediate Component (rows 1, 5) are absent, as the triage sends them to a head.
- The draft names no Component (rule 2).
- The owner's two reasons, exploring and working, and review by a person, are both present.
- `goal@knowledge-architect@only-needed-instructions` bounds the workflow's text, not a target project's files (triage Q26, #arg-63); I find no state that meets one only by failing the other.

**Boundary block material.**
- Asks of future work: the workflow moves a target project toward units of one purpose, at each layer.
- Requires: that the workflow says something that leads to such a split of the project's parts, of the code of one part, and of its design documents by subject.
- Admits: a project of one purpose kept as one unit; a module directory as a unit, its form left to design (triage Q27); a design home split in files by subject.
- Rules out: a rule of splitting that a unit fails alone.
- Reaches, perhaps unintended: the layers of finding 7. The wider wording becomes the ground of heads on code inside one crate and on design homes by subject (Q25), where the old goal bound Components only.
- Siblings: `goal@knowledge-architect@the-workflow-leads-to-better-code` (the owner: "partially orthogonal"); `goal@knowledge-architect@documentation-has-a-structure-and-a-model` (the structure of the documents is defined by knowledge-architect, their split by subject is led by the workflow); `goal@knowledge-architect@agents-work-without-drift` (reading one unit whole).

---

##### `goal@knowledge-architect@the-workflow-improves-through-real-use`

**Finding 8. Rule 3, `asks`, `intent`.** "the test" reads two ways.
- Goal: "It is the test of how an agent behaves under the workflow."
- Analysis, row 2: the exclusion "No mock task and no mock project is built ..." goes to "existing head: `design@agent-skills@synthetic-evidence-not-built`". Triage Q29: "ANSWERED ... the rule on mock runs going to its head".
- Case: a session builds a mock project to observe how an agent follows an instruction. Reading "the test" as the only one, the goal rules it out outright. Reading it with the rulings, only the head does, as a presumption.
- The draft's last sentence speaks of a "change", not of evidence, so it does not settle this.

**Finding 9. Rule 3, `asks`, `intent`.** The last sentence reads as a licence or as a mere absence of prohibition.
- Goal: "This goal forbids no change that is argued to lead to a better result in what a target project implements or records."
- Owner (D3): "If you have an argument that the instructions you proposed adding would lead to a better result (in terms of what gets implemented or recorded in a target project), I think the addition should stand."
- Triage Q32: "SETTLED BY THE RECORD ... A rewording of that head's statement goes through the design skill". The analysis's draft of this goal held no such sentence.
- Case: an instruction on behaviour, argued and with no evidence from use, against `design@agent-skills@behaviour-additions-need-real-use`. One agent reads the sentence as the goal admitting it, so the head yields to the goal. Another reads that this goal is silent and the head still binds.
- The owner's words say more than the draft ("should stand"), and the analysis routed them to the head. Which is meant in the goal?

**Finding 4** also bears on this goal's third sentence.

**Checked, no finding.**
- Row 1 is kept in both halves. Row 3 is kept without the retrospective. Row 6 is kept as "Where such a change is an owner's to rule, that owner is knowledge-architect's", with no citation.
- Rows 4, 5, 7 and the measures are absent, as sent to heads or issues.
- `goal@knowledge-architect@the-owner-decides` does not list a change of the workflow among what knowledge-architect's owner decides. The two share the aspect and do not conflict (triage Q30).

**Boundary block material.**
- Asks of future work: what real use shows unclear, missing or wrong reaches knowledge-architect and changes the workflow.
- Requires: a route from a session in any target project, one the owner does not own included, to knowledge-architect.
- Admits: a change argued to lead to a better result, as far as this goal goes.
- Rules out, on one reading only: evidence of behaviour other than real use (finding 8).
- Reaches, perhaps unintended:
  - a session with no supervising person is no real use, while `goal@knowledge-architect@agents-work-without-drift` moves toward "less supervision";
  - findings from a project of another owner are required to reach knowledge-architect;
  - every lapse found is read as "missing" (finding 4).
- Siblings: `goal@knowledge-architect@only-needed-instructions` (finding 4); `goal@knowledge-architect@the-owner-decides` (whose ruling); `goal@knowledge-architect@every-target-project-can-adopt-it` (a change drawn from one project's use "holds nothing particular to one target project"); `goal@knowledge-architect@serves-one-kind-of-project` (what a current project does not need is no reason to remove).

---

##### `goal@knowledge-architect@session-orchestrates-the-work`

**Finding 10. No rule, `blocks`, `contradiction` (b).** The draft drops a statement its rows keep.
- Analysis, row 6: "the dispatch of the rest of the work, and acting on what comes back." has the destination "stays in the goal". The analysis's draft held "and acts on what comes back".
- Goal: "It orchestrates agents it dispatches to carry the work, so that it can focus on deciding the design with that person, and on reporting each decision taken." It has no word on acting on what comes back.
- The other side: the owner's #arg-3 states the goal without it and adds "Nothing more detailed", and "orchestrates" may be read to carry it.
- The dispatcher shows the drop to the owner, or restores the clause.

**Checked, no finding.**
- The body is #arg-3 nearly word for word: "focus", "deciding design with the human interlocutor", "reporting any decision taken".
- "the goals" (row 3b) and "the questions put to that person" (row 4c) are absent, as sent to the dispatch issue. Triage Q46a says the draft shows that drop to the owner at its top.
- The last sentence states what the goal leaves open, from the owner's "When exactly these dispatch happen, and on which kind or size of work, is left to argumented design"; by rule 3 that openness is no finding.
- No citation; the refinement of `goal@knowledge-architect@agents-get-a-complete-workflow` is gone, as #arg-47 asks.

**Boundary block material.**
- Asks of future work: the main session dispatches agents to carry the work, and keeps its focus on deciding the design with the supervising person and reporting each decision.
- Requires: that work is carried by dispatched agents, in the direction of more dispatch. Today `issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch` records where the installed text departs.
- Admits: the main session reading one known file itself; any threshold of size or kind a head argues.
- Rules out: a main session that carries all the work itself as the standing shape.
- Reaches, perhaps unintended:
  - a harness with a far larger context still owes the dispatch. The triage of Q44 says the boundary block must show this to the owner under rule 4;
  - "that person" is whoever supervises, a contributor included, while `goal@knowledge-architect@the-owner-decides` gives the forks to the owner;
  - the goal names neither the workflow nor a target project, unlike its five neighbours.
- Siblings: `goal@knowledge-architect@the-owner-decides` shares "reporting each decision" with "reaches the owner"; `goal@knowledge-architect@design-with-agents-as-counterparts` (the exchange the focus is kept for); `goal@knowledge-architect@agents-work-without-drift`, which ranks first where a dispatched agent's narrower context raises drift.

---

##### Decided

- The list of `goal@knowledge-architect@agents-get-a-complete-workflow` is no "list of cases": `skill@knowledge-architect-goal-setting@goal-entry-shape` admits a list of the members of a set.
- "agent" has one meaning across the homes. The main session is an AI agent, and "agents it dispatches" names a subset, so no rule 3 finding.
- Finding 9 is graded under rule 3 and not as (b): the sentence states what the goal does not forbid, and the admission rule that the rulings sent to a head is not in the draft.

##### Met outside the task

- `design@agent-skills@the-workflow-teaches-no-rust-idiom` (crates/agent-skills/docs/design/development.md, line 41) quotes `goal@knowledge-architect@only-needed-instructions` for "whichever goal the instruction would serve", and says `goal@knowledge-architect@the-workflow-leads-to-better-code` "could be read to call for" guidance on idiom. It is re-read if findings 4 or 5 change either goal's words. For the dispatcher to route.
- The analysis names the wrong head in Q27a and in rows 1 and 5 of the Components goal. Its own triage already records this (line 4833); nothing new.

##### Counts

- Findings: 10.
- Per rule: rule 2: 1; rule 3: 4; rule 4: 2; rule 6: 1; no rule, (b): 2; rules 1, 5, 7, 8: 0.
- Per route: `intent` 8; `contradiction` 2; `settled` 0.
- Per grade: `blocks` 2 (findings 6 and 10); `asks` 8; `minor` 0.
- No goal of the batch is without a finding.

#### The reviewer of: The two together

Tree: worktree `/home/teo/dev/knowledge-architect/.claude/worktrees/r1-together`, HEAD the commit under review, clean.

The batch has two goals and both pass rules 1 to 4 and 6 to 8. Five findings remain: three statements the analysis keeps and the draft drops, one ranking written inside a purpose, and one sentence of the head that reads two ways.

What I read:

- my definition on disk, `path@agent-config@agents/knowledge-architect-goal-reviewer.md`;
- `register@knowledge-architect@goal` and the six files, whole;
- the review's record, whole. It holds no open question and no round;
- the analysis: its section "The owner's rulings" whole, and the sections of `goal@knowledge-architect@agents-work-without-drift`, `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, `setup-brings-quality-tools` and `owner-attention-on-what-is-theirs`;
- in the analysis's part "After the rulings", only the rows a search returned for these goals and their question labels, not the part whole;
- the milestone document: the threads on the three kinds and on examples, and `arg-45`.

`cargo klarch anchors` (exit 0) lists 13 goals homes: `register@knowledge-architect@goal` and 12 Component files. Each of the 12 holds zero level-two headings (`grep -c '^## '`). The brief agrees with the tree on this.

##### The head of the home (docs/goals/README.md), where its definitions bear on the batch

###### Finding H1: rule 3, `asks`, `intent`

The definition of the record ends: "Nothing else is part of it, its goals included."

It reads two ways:

- the goals are among what is not part of the record;
- the goals are included in the record.

The drift goal depends on this, since it lists "its goals" beside "the decisions its record holds".

Three texts point to the first reading:

- the drift goal lists the goals apart from the record;
- `goal@knowledge-architect@the-owner-decides` says "the goals, the record, the code and the sources";
- `goal@knowledge-architect@the-owner-s-word-is-verified` says "the project's goals, its record, its code".

Case on which the readings differ: an agent asked to check a word "against the record". Under the second reading it takes the goals as covered by rules stated of the record. Under the first it reads them apart.

No statement of the owner fixes this sentence. If the dispatcher takes the three goals as settling the reading, the repair is a rewording of the sentence.

The definitions of "the generic part" and "a target project" are used by the batch in one sense each. Finding G2 below bears on the first.

##### `goal@knowledge-architect@agents-work-without-drift`

Rules 1, 2, 3, 4, 6, 7 and 8 pass.

- **Rule 1.** Purpose. Removed, the reviews that read work against the record and the register of designs not kept lose their reason.
- **Rule 7.** No measure. The old rows 10 to 13 are absent, as the analysis sends them.
- **Rule 8.** The three citations are goals of the same Component, and the goal says what it needs in its own words: "the decisions of the project's owner, the exchange with that owner and the check of that owner's word".
- **Ranking.** The exception matches question 6 of the eight, which the owner answered "6. default". The default was: it keeps "never before the owner's decisions, the exchange with the owner and the check of the owner's word". "The owner's attention" is absent, as the ruling "no goal bounds the owner's reading time" requires.
- **Rule 6.** `goal@knowledge-architect@only-needed-instructions` is a scope and says "whichever goal the instruction would serve". The ranking speaks of "two purposes", so the two do not conflict.

###### Finding D1: rule 5, `minor`, `settled`

The goal holds a purpose and a ranking.

- Goal: "Where two purposes of knowledge-architect compete, this one comes first. It never comes before the decisions of the project's owner, …"
- Rule 5: "Two statements are two goals when each means something without the other."

The purpose means something without the ranking. The ranking, with the purpose named, means something without the purpose's body, and rule 1 lists a ranking as a kind of its own.

The repair the standard implies is a ranking goal of its own. That repair meets a ruling:

- the list row reads "`goal@knowledge-architect@agents-work-without-drift`, with its ranking";
- the owner said: "2. agreed on those 6 files and 21 goals in the list."

The triage says: "Whether a ranking inside a purpose passes rule 5 is the reviewer's." It does not pass by the rule's words. A split makes 22 goals, so the dispatcher holds it for the owner and does not apply it as a plain repair.

###### Finding D2: contradiction (b), `blocks`, `contradiction`

The draft narrows a statement the analysis keeps.

- Old row 2, "stays in the goal": "What they build stays within the design recorded in that project, as it stands until a change reverses part of it through the workflow's procedure:", with three items: "its goals", "the decisions its record holds, with their arguments", "the reasons recorded beside its code and its agent-facing text".
- The row's rewording: "'through the workflow's procedure' is reworded as an outcome, a reversal made on purpose and argued".
- Draft: "A recorded decision changes only by a change that reverses it on purpose and argues the reversal."

The old sentence gave the path of change for any part of the recorded design. The draft gives it for "a recorded decision" only.

Case on which the readings differ: a reason recorded in a comment beside code.

- Agent A reads "a recorded decision" as covering it, and changes the code with an argued reversal.
- Agent B reads the head's definition, "A reason recorded beside code or in a commit message is none" (of a recorded item), and the draft's own list, which sets the reasons apart from the decisions. B finds no stated path, and either treats the reason as fixed or changes it with no argument.

For the goals, the missing path is held by `goal@knowledge-architect@the-owner-decides` ("the owner states what the project is for"). For the reasons beside code and agent-facing text, no goal holds it. This was not shown to the owner.

###### Boundary block material

**What it asks of future work:** that what agents build in a target project stays within its goals, its recorded decisions and the reasons recorded beside its code and agent-facing text, with less supervision. Where a purpose of knowledge-architect competes with this one, this one wins, except against the owner's decisions, the exchange with the owner and the check of the owner's word.

**Requires:**

- a change that contradicts a recorded decision either reverses it on purpose with an argument or is not built;
- a design the project did not keep returns only by a purposeful reopening;
- between this purpose and another, such as "Adopting asks little work of them" in `goal@knowledge-architect@adoption-is-easy`, this one comes first.

**Admits:**

- any means of catching a departure (a review, a search, a check);
- any amount of remaining supervision;
- a reversal of a decision by an agent on argument, with no word of the owner beyond what `goal@knowledge-architect@the-owner-decides` gives.

**Rules out:**

- a silent reversal of a recorded decision;
- a design not kept brought back with no reopening;
- a gain in autonomy that takes a decision from the project's owner, skips the exchange with the owner, or skips the check of the owner's word.

**Cases it reaches that the owner may not intend:**

- the ranking is over every other purpose of the 21, not one. The old goal said "it ranks first among knowledge-architect's goals", and the draft keeps that reach;
- a reason held only in a commit message is not in the goal's list of the recorded design, though the head puts it in the record. The old goal did not list it either. For information;
- the old title's "productively" is absent. The title the owner agreed in the list also omits it, and no row carried it. For information.

**Siblings touched:**

- `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@design-with-agents-as-counterparts`, `goal@knowledge-architect@the-owner-s-word-is-verified`: ranked above it, by its own words.
- `goal@knowledge-architect@design-is-recorded-with-its-arguments`: provides what this goal bounds the work by. No conflict.
- `goal@knowledge-architect@only-needed-instructions`: a scope that bounds the means of this purpose. No conflict.
- `goal@knowledge-architect@the-workflow-improves-through-real-use`: shares the aspect of learning from lapses. No conflict.
- `goal@knowledge-architect@session-orchestrates-the-work`, `goal@knowledge-architect@adoption-is-easy`, `goal@knowledge-architect@the-workflow-leads-to-better-code`: ranked under it where they compete.

##### `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`

Rules 1 to 8 pass.

- **Rule 1.** Purpose. Removed, what the project ships that keeps no document (gates, the hand mutation, the development procedure) loses its reason.
- **Rule 2.** The "such as" list names kinds of things a project sets up, and no built part of knowledge-architect.
- **Rule 3.** The examples close no question of when, how much or by what means. The three forms of shipping and the copy policy are absent, as old rows 6b, 10 to 12, 16, 18 and 20 to 22 send them.
- **Rule 5.** One intent. The owner's gate is its qualifier, kept as the triage of Q62a says: "The gate is in G-B".
- **Rule 7.** No measure.
- **Rule 8.** No citation.
- **Other rows.** The sentence on quality tools from `setup-brings-quality-tools` is absent, as the merge table rules ("design: four existing heads hold it"). Old row 5 is held by `goal@knowledge-architect@serves-one-kind-of-project` ("for the choice of knowledge-architect's owner to take in the generic part of a tool or a procedure"). Old row 9 is held by `goal@knowledge-architect@only-needed-instructions` ("A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve").

###### Finding G1: contradiction (b), `blocks`, `contradiction`

A clause of a kept sentence is dropped with no destination.

- Old row 7, "stays in the goal": "What is particular to one project stays in it, declared or configured beside what knowledge-architect ships."
- Draft: "What is particular to one project stays in that project."

The clause "declared or configured beside what knowledge-architect ships" is absent, and no row sends it to a head, an issue or nowhere. It says a shipped part leaves room for a project's own declaration or configuration.

It may be a question of by what means, which rule 3 leaves to design. The analysis does not say so, and it was not shown to the owner.

###### Finding G2: contradiction (b), `blocks`, `contradiction`

A clause of the definition of the generic part is dropped.

- Old row 2, "stays in the goal; the one pass states the definition once, in the head of the root's goals home": "The generic part of one is the part whose procedure does not depend on what the project builds, …: it serves each target project that carries out its activity, or is meant to."
- Head: "the generic part of an activity, a tool or a procedure: the part that does not depend on what the project builds".
- Draft: "so that every target project gets it".

The clause "it serves each target project that carries out its activity, or is meant to" is in neither text. For an activity, `goal@knowledge-architect@agents-get-a-complete-workflow` keeps its reach: "an activity that only some target projects carry out included". For a tool or a procedure, nothing does.

Case on which the readings differ: a publishing method that the owner has taken in, and a target project that publishes nothing.

- Agent A reads "every target project gets it" as: it is made available, and serves those that publish.
- Agent B reads it as: it is delivered into every target project, or asked of each.

The old clause chose A. Its drop was not shown to the owner.

###### Boundary block material

**What it asks of future work:** when a target project sets up or changes a tool or a procedure, the part that does not depend on what that project builds is shipped by knowledge-architect for every target project. This happens only once knowledge-architect's owner takes it in, and the particular part stays in the project.

**Requires:**

- a gate that one target project built and the owner took in ships for every target project;
- the same holds for a testing method, such as the hand mutation, once taken in.

**Admits:**

- a generic part the owner never takes in, which stays in its project;
- any form of shipping (installed text, a command, a library);
- any time between taking in and shipping;
- a project that keeps its own copy of a shipped part. That policy went to a design issue.

**Rules out:**

- shipping a part because it is generic, with no choice of the owner ("and not because it is generic");
- shipping what is particular to one project;
- reading knowledge-architect as a tool for documents only ("about more than keeping a project's documents").

**Cases it reaches that the owner may not intend:**

- "every target project gets it", for a part whose activity only some projects carry out (finding G2);
- "a kind of reviewer", "a publishing method" and "a testing method" are also activities of `goal@knowledge-architect@agents-get-a-complete-workflow` (reviewing, releasing, developing). A procedure taken in is then owed by both goals. The directions agree. For information.

**Siblings touched:**

- `goal@knowledge-architect@the-owner-decides`: holds the same gate, "whether and when it takes in the generic part of a tool or a procedure". Shared aspect, no conflict.
- `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`: tools from the workflow's tasks, under the same gate. Shared aspect.
- `goal@knowledge-architect@agents-get-a-complete-workflow`: shares "the generic part" and the overlap above.
- `goal@knowledge-architect@only-needed-instructions`: a scope that bounds a procedure shipped as text.
- `goal@knowledge-architect@serves-one-kind-of-project`: keeps the exception for the owner's choice to take in.
- `goal@knowledge-architect@every-target-project-can-adopt-it`: "holds nothing particular to one target project" agrees with "What is particular to one project stays in that project".

##### Contradictions of kind (a)

None found. No statement of the owner in the rulings section conflicts with a goal of the batch or with another statement on these two goals.

##### Met outside the task

- **The brief and the disk standard differ on one phrase.** The brief says a goal "states no measure and no list of cases". The standard on disk has no rule in those words. Rule 7 refuses a measure, and the table has a row for "a clause, a case included, that closes a question of when, how much or by what means". I judged the "such as" list of the second goal by that row and by the approved thread "A goal holds no fixed block of cases, and an example only where its direction needs one". For the dispatcher to route.
- **I wrote one file, against the brief's "write no file".** The output of `cargo klarch anchors` went to `/home/teo/.cache/claude-tmp/claude-1000/-home-teo-dev-knowledge-architect/221be2f0-cdd5-5e76-b391-13ea5eba2fec/scratchpad/r1-together-anchors.txt`. The build also created the worktree's `target/` directory, as the brief's `CARGO_TARGET_DIR` directs. Nothing in the tree was edited.

##### Counts

| | count |
| --- | --- |
| Findings | 5 |
| Rule 3 | 1 (H1) |
| Rule 5 | 1 (D1) |
| No rule, contradiction (b) | 3 (D2, G1, G2) |
| Route `settled` | 1 (D1) |
| Route `intent` | 1 (H1) |
| Route `contradiction` | 3 (D2, G1, G2) |
| Grade `blocks` | 3 (D2, G1, G2) |
| Grade `asks` | 1 (H1) |
| Grade `minor` | 1 (D1) |

Neither goal conforms without a finding: `goal@knowledge-architect@agents-work-without-drift` has D1 and D2, and `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` has G1 and G2.

#### The reviewer of: The owner and the agents

Reviewed at the commit under review, in the worktree `/home/teo/dev/knowledge-architect/.claude/worktrees/r1-owner`. Nothing edited, no file written.

##### State of the tree, as established

- `cargo klarch anchors` lists 13 goals homes. `grep -c '^## '` over the 12 Component homes gives 0 each. The root's home is `register@knowledge-architect@goal`: `README.md` (the head) and six files.
- `path@knowledge-architect@docs/goals/owner.md` holds three goals: `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@the-owner-s-word-is-verified`, `goal@knowledge-architect@design-with-agents-as-counterparts`. It holds no backticked reference.
- The review's record, `path@knowledge-architect@docs/goal-reviews/2026-11-10-goals-at-the-root-review.md`, read whole: "Questions open: None yet: round 1 has not run." So no finding below repeats an open question.
- Read whole: the section "The owner's rulings" of the analysis (lines 17 to 261); the theme "The owner and the agents" (lines 2249 to 2755, the four old goals, every row); the section "After the rulings" for the holders table and the triage of Q48 to Q58 (lines 4437 to 4651); the rows of other themes that name one of these three goals as holder (drift rows 4, 7, 8; real-use row 6; G-A rows 3b, 4, 5; G-B row 4; scope rows 9, 14b, 22c, 22d, 22f, 29d; `goals-state-the-owner-s-intent` rows 1 to 12). In the milestone document: the "Threads" checkpoint table and the thread heads on the standard, and the arguments #arg-1 to #arg-3 and #arg-26 to #arg-35. A grep of "Threads" and "Arguments" for the three slugs and for `owner-attention` finds no further word of the owner on them. I did not read every one of the 122 arguments whole: the others bear on the standard and on other goals by their titles. This is a limit of the review.
- The old text of the three goals was read at `a commit of this branch^:docs/goals.md`.
- No disagreement found between the brief and the tree.

##### The head, `register@knowledge-architect@goal`, where it bears on the batch

**H-1. Rule 3, `asks`, `intent`.** The definition of the record ends: "Nothing else is part of it, its goals included." Two readings: the goals are among what is no part of the record, or the goals are included in it. The batch reads the first way: `goal@knowledge-architect@the-owner-s-word-is-verified` lists "the project's goals, its record" as two things, and `goal@knowledge-architect@the-owner-decides` "the goals, the record, the code". Case on which the readings differ: an agent that reads "the record" in `goal@knowledge-architect@design-with-agents-as-counterparts` ("grounded in the record") either grounds a proposal in the goals too, or does not.

No other finding on the head for this batch. Its one reference is to a goal of its own Component.

##### `goal@knowledge-architect@the-owner-decides`

**D-1. Rule 3, `blocks`, `settled`: "durable text" has two meanings in the project.** The goal: "Each decision the agents make that earns durable text ... reaches the owner".
- Sense 1, in the old goal this one comes from (`a commit of this branch^:docs/goals.md`): "Each decision the agents make that earns durable text, such as a recorded decision, the record of a design not kept, an outstanding item, the evidence that would flip a decision, a commit message or a comment". A commit message and a comment are durable text.
- Sense 2, in `path@agent-skills@docs/design/design-skill-and-heads.md`, lines 120 to 121: "A choice that earns no durable text stays in a code comment and a commit message." And `path@agent-config@skills/knowledge-architect-decision-recording/SKILL.md`, line 8: "Whether it earns durable text at all". A comment and a commit message are not durable text.
- The draft dropped the list that fixed sense 1 (row 16: "the list of kinds of durable text is an example").
- Repair implied: replace the term by one the head defines ("the record" covers "the reasons recorded beside its code, its agent-facing text and in the messages of the commits"), or define it in the goal.

**D-2. Rule 3 and rule 4, `asks`, `intent`: the closed "the rest" against what a project reserves.** The goal: "Agents decide, by argument, what the goals, the record, the code and the sources the project follows settle. The owner decides the rest: a choice between shapes that all survive the argument, and a contradiction no argument resolves."
- The old goal held "A project may reserve further actions to its owner." and "Admits, for instance: a project that reserves the merge of a branch to its owner." Rows 12 and 26 send both to `design@agent-skills@questions-go-to-their-authority`, so the drop follows the rulings.
- The draft keeps the closure and not the admission. A goal prevails over a head, so the head's sentence now stands under a goal that reads closed.
- Case: a target project whose `CLAUDE.md` reserves the merge to its owner, where the argument settles that the branch is ready. One agent merges, since the goal gives agents what the argument settles. Another waits for the owner.

**D-3. Rule 3, `asks`, `intent`: a reading of a goal that no argument ranks.** The owner, in the rulings: "As long as there is value I can bring whenever an item is brought to me. For example, a question about a missing fact, a cost weighting, or a goal reading." And "1 agreed on the reporting of a goal reading", on a proposal that says "where no argument ranks two readings that move in different directions, it goes to the owner".
- The draft's "the rest" names two members, a choice between shapes and a contradiction. The triage notes: "the drafter checks that one of the two covers that case."
- Case: two readings of a goal move a decision in two directions and no argument ranks them. One agent takes it as "what the goals ... settle" and decides. Another sends it as a choice between shapes, or under "the owner states what the project is for".

**D-4. Rule 4, `asks`, `intent`: the reach of "what knowledge-architect asks of every target project".** The goal: "knowledge-architect's own owner decides each addition, tightening or removal of what knowledge-architect asks of every target project", "Whatever the argument settles".
- The old scope goal bounded the term: "An ask that binds every target project is added by a design decision of knowledge-architect, or by taking in a quality tool"; "a procedure of the installed workflow is none"; "a tool that serves efficiency ... becomes no ask". Those sentences left for a design issue (scope rows 14a, 15a, 15b). The scope goal now says only: "knowledge-architect asks more of a target project than these conditions. Among what it asks, the project is a git repository, and its agents run under the `claude` harness."
- Cases the draft now reaches, which no statement of the owner in the rulings covers:
  1. a new rule of the checker "for every project" (the words of `goal@knowledge-architect@documentation-stays-consistent`);
  2. a new or tightened instruction of the workflow, which the old text excluded;
  3. a gate added to what a project runs.
- In each, an agent of knowledge-architect that the argument settles must still wait for the owner's decision. `goal@knowledge-architect@the-workflow-improves-through-real-use` says of a change to the workflow only: "Where such a change is an owner's to rule, that owner is knowledge-architect's."

**D-5. Rule 3, `asks`, `intent`: "each choice they make that the owner could have ruled".** The old goal went on: "other than an implementation choice that a commit message records". Row 18 sends the exclusion to `design@agent-skills@decisions-reported-to-the-owner`.
- Reading A: the choices the goal gives the owner, a fork or a default taken at one. Reading B: any choice the owner could have made, each implementation choice included.
- Case: an agent picks one of two data structures and records why in the commit message. Under B the goal requires it to reach the owner, and the head's exclusion contradicts a goal. Under A it does not.

**D-6. Rule 3, `asks`, `intent`: "the owner states what the project is for" against "a goal to set or change".** Row 7 keeps the old member "a goal to set or change" as these words. The old sentence was "its owner states the project's intent in its goals". The draft names goals only as an authority agents argue from. Q42 re-points four heads to this goal "if the analysis of that theme keeps in it that the goals are the owner's".
- Case: an agent rewords, splits or removes a sentence of a goal, and argues that what the project is for is unchanged. One agent reads that as settled by argument, another as the owner's.

**D-7. Rule 3, `asks`, `intent`: "In a project that follows the workflow".** Only this goal of the home uses the phrase (`grep -rn 'follows the workflow' docs/goals/`: one hit). The scope goal says "Every other goal that speaks of a project ... means a target project" and "A project that does not yet follow an ask is still a target project". The old definition, "A project that follows the workflow is one that has installed it", went to a design issue (scope row 31).
- Case: a target project that has installed the workflow and does not follow every ask yet. Either the goal holds there, or only once the project conforms.

**D-8. Contradiction (b), `blocks`, `contradiction`: "and its measurements" is dropped, here and in `goal@knowledge-architect@the-owner-s-word-is-verified`.**
- Old: "Agents decide, by argument, what these settle: the goals, the record, the code and its measurements, and the sources the project follows". Row 2: "stays in the goal". Draft: "what the goals, the record, the code and the sources the project follows settle".
- The drop is in the analysis's own draft, and neither the analysis nor the rulings show it to the owner as one.
- Case: what an external tool does, such as git, is answered by a measurement. It is not the code, and it is not a source the project follows. It is then in neither the agents' list nor the two members of "the rest".
- The impact is small. Restoring the three words repairs it. It is graded `blocks` only because the table gives every (b) that grade.

**Rule 5, no finding, one note.** The goal holds three statements: the split of authority, the items of knowledge-architect's own owner, and the owner's sight. The clause "agents decide only while the owner can see what they decided" makes the third a condition of the first (row 20, kept). The second is a member of the first's list. The owner ruled the fate "kept", with the two members (Q49, by exception). I report no rule 5 finding.

**Rules 1, 2, 6, 7, 8.** Purpose: without it, the routing of a question to its authority and the report of decisions lose their reason. It names no built part, states no measure, and cites nothing.

**Rows checked, no loss.** Rows 0a, 0b, 1, 5a, 6, 8, 9, 10, 11, 15b, 16, 18, 19, 20 are carried. Rows 3, 4, 5b, 5c, 12, 13, 14, 15a, 17, 21 to 28 are absent, as ruled. Scope rows 14b and 22f and G-B row 4 are carried by the second paragraph. G-A row 4 ("a task is not taken in because the installed text requires it") names this goal as holder and is carried by `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s` instead ("never because the workflow asks it"), so nothing is lost. Attention rows 4 and 5 are carried. The draft holds no statement the rulings sent to a head, to an issue or nowhere, except the closure of D-2.

**Boundary material.**
- Asks of future work: the workflow routes each question to the authority that answers it, keeps to the owner what no argument settles, and makes each agent decision visible and contestable.
- Requires:
  - a fork goes to the owner;
  - a publication waits for the project's owner even when the argument settles it;
  - a new ask, or the taking in of a generic part, waits for knowledge-architect's owner;
  - a decision recorded by an agent reaches the owner.
- Admits:
  - any form of the report (the "Decided" section, the record itself);
  - any moment of the report;
  - the owner contesting after the merge.
- Rules out:
  - an agent decision the owner cannot see;
  - an irreversible action on argument alone;
  - under the closed reading, a question the argument settles put to the owner for a ruling.
- Cases the owner may not intend: D-2 (a reserved merge), D-4 (each checker rule and each workflow instruction as an ask), D-5 (every implementation choice reported), D-8 (a question a measurement of a tool answers).
- Siblings:
  - `goal@knowledge-architect@agents-work-without-drift`: its ranking yields to this goal by name;
  - `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`, `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, `goal@knowledge-architect@serves-one-kind-of-project`: each states the taking in by knowledge-architect's owner for its own object, a task, an activity or a part. This goal names "a tool or a procedure" only. Shared aspect, no conflict;
  - `goal@knowledge-architect@the-workflow-improves-through-real-use`: it names which owner rules a change of the workflow;
  - `goal@knowledge-architect@session-orchestrates-the-work`: "reporting each decision taken", the same direction;
  - `goal@knowledge-architect@the-owner-s-word-is-verified`: this goal says "a contradiction no argument resolves", that one "no source or argument resolves". The old row 6 held "no source or argument".

##### `goal@knowledge-architect@the-owner-s-word-is-verified`

**V-1. Rule 4, `asks`, `intent`: "other than a goal".** The goal: "An agent takes no word of the owner as granted. Before acting on a word other than a goal, it checks what the word assumes".
- The analysis: "Words of the owner that bear on it: None. The rewrite of this goal must ask the owner." No ruling quotes a word of the owner on it. The triage classes Q53a as wording, to be shown by reach.
- What the wording does: the first sentence covers every word, a goal included. The second requires a check for every word but a goal. So the goal requires no check of what a goal assumes, at any moment.
- The old text required it "when the goal is written or reworded, and again when work meets evidence against one". Rows 3 and 4a go to a design issue, and row 4b to `design@agent-skills@owner-word-holds-as-far-as-its-premise`.
- Cases for the owner:
  1. work meets evidence that a goal's premise is false. The goal asks nothing; the head does;
  2. the owner states a new goal on a premise the code shows false. The goal asks no check before the goal is written.

**V-2. Contradiction (b), `blocks`, `contradiction`: "and its measurements" dropped.** The same finding as D-8, counted there once more for this goal. Old: "checks its premises against the goals, the record, the code and its measurements, and the sources the project follows" (row 2, "stays in the goal"). Draft: "against the project's goals, its record, its code and the sources it follows". Case: a word of the owner that assumes how an external tool behaves, checkable only by a run.

**Other rules.** Rule 1: purpose, passed narrowly as the analysis says. `primer@owner-word-premise` and the hold on work resting on a contradiction lose their reason without it. Rule 5: one intent. Rule 6: no conflict; the drift goal's ranking yields to it by name. Rules 2, 7, 8: no built part, no measure, no citation.

**Rows checked.** Rows 1, 2, 5 and 6 (the hold) are carried. Rows 3, 4a, 4b, 7 and 8 to 14 are absent, as ruled. "The rest goes on" (row 6, second half) is absent as ruled, and "nothing is built on it" admits it.

**Boundary material.**
- Asks of future work: in every activity, an agent checks what a word of the owner assumes before it acts, and work that rests on an unresolved contradiction waits.
- Requires:
  - the owner asks for a change on a premise about the code, and the agent reads the code first;
  - the owner's word contradicts a recorded decision and no argument resolves it, so the question goes to the owner and that part is not built.
- Admits:
  - the rest of the work going on;
  - any means of the check;
  - telling or not telling the owner when a source resolves the contradiction (the old Requires case "says so before it builds" is the head's now).
- Rules out:
  - acting on an unchecked word;
  - building on an unresolved contradiction.
- Cases the owner may not intend:
  - V-1;
  - the check before every word, a bare pick at a fork and a fact only the owner holds included. The goal requires the check of what such a word assumes, where it is checkable.
- Siblings:
  - `goal@knowledge-architect@the-owner-decides`: it gives the contradiction to the owner, and this goal adds the hold;
  - `goal@knowledge-architect@design-with-agents-as-counterparts`: it covers a proposal of the owner inside a design exchange, and this goal every word in every activity. Shared aspect;
  - `goal@knowledge-architect@agents-work-without-drift`: its ranking yields to "the check of that owner's word".

##### `goal@knowledge-architect@design-with-agents-as-counterparts`

**C-1. Rule 3, `asks`, `intent`, with a conditional rule 6: "change the direction of the design" with no means.** The goal: "while the owner can still challenge it, add an argument or a fact, or change the direction of the design."
- The old text: "change the direction of the design, reversing, with a goal, a fact or an argument, what a goal, the record or the argument settled." Row 7 sends the means to `design@agent-skills@intent-through-goals-not-overrule`.
- The draft lists the change of direction beside "add an argument or a fact", as a third power distinct from them.
- Reading A: the change of direction still comes through a goal, a fact or an argument. Reading B: the owner changes the direction by a bare word.
- Under B the goal conflicts with `goal@knowledge-architect@the-owner-decides`: "Agents decide, by argument, what the goals, the record, the code and the sources the project follows settle."
- The owner, #arg-27: "If I judge a cost too large, i can overrule a decision without needing a goal." The triage (Q48) reads this as an argument, on the record and not on a word of the owner on Q48.
- Case: the owner says "take the other shape" on a point the argument settled, and gives no reason. One agent complies. Another asks for the goal, the fact or the argument.

**C-2. Rule 4, `asks`, `intent`: "each proposal" without "of the exchange".** The goal: "The owner meets each proposal with its grounding and its consequences established, and its rival where one stands".
- The old text: "each proposal of the exchange". Row 11b, "every proposal it makes inside an exchange the owner is already in", goes to a design issue. Row 9a, which acts are an exchange and "a report alone is not", goes to heads.
- Cases the unqualified wording reaches:
  1. a design decision an agent closes on its argument in the in-change path, during autonomous work. Either the owner must meet it with its rival, or the report under `goal@knowledge-architect@the-owner-decides` is enough;
  2. a proposal one subagent makes to the main session.
- Under the wide reading the goal requires more of the owner's reading than the old goal did. The owner stated no bound ("I have no bound to state on the quantity of what reaches me") and also: "If there is nothing else I can give other than approval, there is little value in bring the question to me."

**No finding: "while the owner can still challenge it".** It replaces "before it reaches the main branch" (row 4, to `design@agent-skills@default-ruled-before-merge`). It leaves open when. Under the standard that is no finding.

**No finding: "so that the design taken is the one that, under argument, fulfils the goals best at acceptable costs".** It comes from the owner's words, #arg-27: "I want to take the solution that under argumentation, will fulfill our goals the best, under acceptable costs." It states the aim of the exchange and no measure. It cites no goal.

**Other rules.** Rule 1: purpose. The discussion form of the design skill loses its reason without it. Rule 5: one intent. Rules 2, 7, 8: clean.

**Rows checked.** Rows 1, 2, 3, 5, 6 and 7 (the power) are carried. Row 2 has "backed by the measurements the agent takes" restored, as the holders table asks. Attention rows 6b and 7 are carried ("with its grounding and its consequences established"). Rows 8, 9a, 9b, 10 to 15 are absent, as ruled. The draft holds no statement sent elsewhere.

**Boundary material.**
- Asks of future work: design is decided in an argued exchange where both sides propose, and the agent brings the grounding, the measurements, the consequences and the rival.
- Requires:
  - the agent challenges a design the owner proposes, on its failures and costs;
  - the agent's proposal reaches the owner with its consequences and its nearest rival;
  - the agent takes the measurements.
- Admits:
  - any moment at which the owner meets the proposal;
  - any form of exchange (a design discussion, a built default, a goal-setting draft);
  - the owner picking at a fork with no argument.
- Rules out:
  - a proposal put to the owner whose consequences the owner must work out;
  - an agent that builds the owner's proposal unchallenged;
  - a design taken without argument.
- Cases the owner may not intend:
  - C-1, reading B;
  - C-2;
  - the supervising person who is not the owner. `goal@knowledge-architect@session-orchestrates-the-work` speaks of "the person supervising the work" and `goal@knowledge-architect@the-workflow-improves-through-real-use` of "a person who contributes to the project or owns it", while this goal names the owner alone.
- Siblings:
  - `goal@knowledge-architect@session-orchestrates-the-work`: "focus on deciding the design with that person", the same exchange seen from the session;
  - `goal@knowledge-architect@the-owner-decides`: it settles who decides, this goal how the design is argued. C-1 is where they could conflict;
  - `goal@knowledge-architect@the-owner-s-word-is-verified`: the grounding of the owner's proposals is a shared aspect;
  - `goal@knowledge-architect@agents-work-without-drift`: its ranking yields to "the exchange with that owner".

##### Contradictions of kind (a)

None found. The owner's "I can overrule a decision without needing a goal" (#arg-27) does not conflict with `goal@knowledge-architect@the-owner-decides` as drafted. It bears on C-1.

##### Met outside the task

- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` exists, at `path@agent-skills@docs/open-issues/the-workflow-assumes-one-unnamed-owner.md`. It covers "the owner" left unnamed in these goals, so that is no new finding. The case of a supervising person who is not the owner, under C's boundary material, belongs to it.
- `path@agent-skills@docs/design/project-configuration.md`, line 335, still says the owner is informed of every "decision that earns durable text or a line of the "Decided" section". It depends on the term of D-1 and follows its repair.
- Across the two goals, "a contradiction no argument resolves" and "a contradiction that no source or argument resolves" differ by two words. It is a wording alignment for the drafter, not a rule's finding.

##### Counts

| by rule | count |
| --- | --- |
| rule 3 | 7 (H-1, D-1, D-2, D-3, D-5, D-6, D-7), plus C-1, which is rule 3 with a conditional rule 6 |
| rule 4 | 3 (D-4, V-1, C-2); D-2 also touches rule 4 |
| (a) or (b), no rule | 2 (D-8, V-2: one drop, in two goals) |
| rules 1, 2, 5, 7, 8 | 0 |

| by route | count |
| --- | --- |
| `settled` | 1 (D-1) |
| `intent` | 10 (H-1, D-2, D-3, D-4, D-5, D-6, D-7, V-1, C-1, C-2) |
| `contradiction` | 2 (D-8, V-2) |

| by grade | count |
| --- | --- |
| `blocks` | 3 (D-1, D-8, V-2) |
| `asks` | 10 |
| `minor` | 0 |

Total: 13 findings. No goal of the batch is without a finding.

#### The reviewer of: Whom the project serves, and adoption

Worktree: /home/teo/dev/knowledge-architect/.claude/worktrees/r1-adoption, HEAD the commit under review (checked with `git rev-parse HEAD`).

##### What I read, and what I did not

- My definition from the worktree, `path@agent-config@agents/knowledge-architect-goal-reviewer.md`, with the standard of eight rules.
- `register@knowledge-architect@goal` and the six goal files, whole.
- `cargo klarch anchors`: 13 goals homes. `grep -c '^## '` gives 0 for each of the 12 Component homes, as the brief says.
- The review's record, `path@knowledge-architect@docs/goal-reviews/2026-11-10-goals-at-the-root-review.md`, whole. It holds no open question and no round.
- The analysis, `path@knowledge-architect@docs/goal-reviews/2026-11-10-goals-at-the-root-analysis.md`: "The owner's rulings" whole (lines 17 to 261); the theme "Whom the project serves, and adoption" whole (lines 3835 to 4435), with the rows of the five old goals; the section of `installed-text-works-in-every-target-project` (lines 1761 to 1818); "Rows that name a goal as their holder" (lines 4445 to 4555); the triage of this theme (lines 4837 to 4924).
- The old text of the three goals, from `git show a commit of this branch^:docs/goals.md`.
- **Not read whole**: the sections "Threads" and "Arguments" of `milestone@plans@goals-at-the-root`. I read #arg-31, #arg-40, #arg-45, #arg-71 and #arg-90, and searched the file for "adopt", "linear history", "harness" and the three slugs. The analysis says of each of the three goals that no word of the owner in the milestone document bears on it by name; my search agrees. A statement of the owner elsewhere in those sections that bears on these goals would be missed.

No disagreement between the brief and the tree was found.

---

##### `goal@knowledge-architect@serves-one-kind-of-project`

Kind: scope. It names the designs it excludes ("a mode for a project in another language", "support for merge commits", "instructions for a project whose development persons drive", "a mode of the checker for another harness"). Rules 1, 2, 6, 7 and 8: no finding. It cites nothing.

###### Finding 1. Rule 3, `blocks`, `settled`: "targets" is used in a second sense

- Goal: "the code it builds and ships is Rust, built by Cargo, apart from the glue its targets need."
- The same goal and the home's head use "target" for the kind of project: "A target project is a project that holds all of these" (adoption.md); "**a target project**: the kind of project knowledge-architect serves" (README.md).
- In the old goal the clause carried its sense in an example the draft drops: "apart from the glue its targets need, such as the JavaScript and the HTML of a web target, or shaders" (old `register@check@goal`, line 432). Without it, "its targets" stands beside "target project" with nothing that says it means what the project is built for, and "glue" is defined nowhere.
- Repair implied: the word is defined or replaced in the clause. The grade is the table's for a term with two meanings; the two readings admit the same kind of case, so the effect on an agent's act is small.

###### Finding 2. Rule 3, `asks`, `intent`: a clause that closes a question of how much (row 6)

- Goal: "Files the workflow installs, the configuration of a continuous integration and scripts kept beside the code are no code it ships".
- The analysis: row 6, "a reading of \"ships\": closes a question of how much"; Q81a, classed "WORDING", "the reviewer judges them by rule 3".
- Case on which the readings differ: a project whose maintenance scripts are Python. With the clause, the goal settles that it is a target project. Without it, "the code it builds and ships is Rust" leaves the case to a design head, which an argument may reopen.
- Note on wording: by the head, the workflow is "the instructions knowledge-architect installs", so "Files the workflow installs" has the instructions install files.

###### Finding 3. Rule 3, `asks`, `intent`: a clause that closes a question of how much (row 8)

- Goal: "An edit that a person makes now and then leaves it a target project".
- The analysis: row 8, "a reading of \"drive\": closes a question of how much"; Q81a as above.
- Case: a project whose owner writes one commit in twenty by hand. With the clause the goal settles it; without it a head would.
- For information: the old sentence read "An edit that a person makes now and then, the project's owner or an outside contributor"; the draft drops who the person is.

###### Finding 4. Rule 3, `asks`, `intent`: "a state that follows none" moves two ways

- Goal: "knowledge-architect guarantees its working only as far as a project follows what it asks, and builds nothing only to serve a state that follows none, other than what helps a project into conformance: not a mode of the checker for another harness."
- Old goal, row 27a: "Nothing in knowledge-architect is built only to serve a state that does not follow an ask".
- The owner's ruling: question 7 as put, "that nothing is built only for a state that follows none; default, they stay in the scope goal as one sentence", answered "7. default".
- Reading A: a state that follows no ask at all. Reading B: a state that does not follow one of the asks. Case: a target project that is a git repository under `claude`, and keeps its own gate runner beside the published gates library. Under A, support built only for that runner is not ruled out, since the project follows other asks. Under B it is. The old words say B, and so does the triage ("a session may then argue, for instance, for support of a gate runner kept beside the published library").

###### Finding 5. (b), `blocks`, `contradiction`: where the `.git` sits is dropped

- Old goal, row 19a: "the project is a git repository, with its `.git` at or above the project's directory". Its destination: "stays in the goal if the owner rules it scope (Q80)".
- The owner's ruling: question 8, "the `claude` harness and git; default, both are scope, named in the scope goal", answered "8. default". The question as put names git and not the clause on `.git`.
- Draft: "Among what it asks, the project is a git repository, and its agents run under the `claude` harness."
- The drop is recorded, and not shown to the owner: `issue@knowledge-architect@no-head-argues-what-becomes-an-ask` says "The goal keeps the ask of a git repository, and no longer says where its `.git` sits", and `design@git@repository-variables-dropped` now says the goal "asks that a target project be a git repository, which this head reads as a `.git` at or above the project's directory".
- Case: a project whose repository git finds only through `GIT_DIR`, with no `.git` at or above its directory. Under the old goal it does not follow the ask. Under the draft the goal is silent, and a head's reading decides.
- The other side, for the owner: kept, the clause closes a question of how much, which rule 3 leaves to a design head.

###### Finding 6. (b), `blocks`, `contradiction`: the sentence on a defect where an ask is not followed is dropped

- Old goal, row 26: "knowledge-architect guarantees its working in a target project as far as the project follows what it asks: a defect that shows only where an ask is not followed is no defect of knowledge-architect."
- The owner's ruling: question 7, "that its working is guaranteed only as far as the asks are followed", default "they stay in the scope goal as one sentence". The question as put holds the first half only.
- Draft: "knowledge-architect guarantees its working only as far as a project follows what it asks". For a project of another kind the draft keeps the parallel sentence: "A defect that shows only in such a project is no defect of knowledge-architect, and what exists only for such a project may be removed."
- Case: a defect of the checker that shows only in a target project whose agents run under another harness. Old goal: no defect of knowledge-architect. Draft: outside the guarantee, and whether it is recorded as a defect or repaired is left open; the contrast with the kept sentence can be read as meant.
- Same rows, for information, not counted: row 12 loses "and every target project is expected to follow what it asks"; row 25 loses "and brings itself into conformance over time". "asks" and "does not yet follow" carry both in fewer words.

###### Checked and no finding

- Rows 0 to 5, 7, 10, 11, 28, 30 and 32 to 34 are carried. "mechanical tools" is now "tools".
- Row 2 of the old adoption goal is carried as "knowledge-architect's own repository is one", per the triage of Q86 and the owner's "Since we are ourselves a target project, in rust is required."
- Row 29c is carried ("for the choice of knowledge-architect's owner to take in the generic part of a tool or a procedure"), as the lost-holder table asks.
- Row 29d loses "which is knowledge-architect's owner's". The triage of Q51 settles that on the record (`design@agent-skills@roadmap-orders-issues`), so it is no drop of the goal.
- No statement the rulings sent to a design head, an issue or nowhere is held: no word on how an ask enters, on a quality tool, on gates, on the merge verdict, on continuous integration, nor on "a project that follows the workflow".
- "Every other goal that speaks of a project, or of an adopter, means a target project": `grep -rn -i 'adopter' docs/goals/` finds this line only, so "or of an adopter" names nothing in the 20 other goals.

###### Boundary block material

- **Asks of future work**: build and repair for the kind of project the four conditions define, and for nothing else; hold a project to the asks without making an ask a condition; never narrow the kind on what today's projects need.
- **Requires**: that work is judged against the conditions and not against the projects that exist. Case: a check that no current target project trips is kept. Case: `design@check@agents-table` knows one harness, `claude`.
- **Admits**: a web target's JavaScript glue; a project that uses the checker alone (the ask on using both left the goal); a person's occasional edit; a project not yet a git repository, which stays a target project; use in another kind of project with no guarantee; removing an installed instruction on the argument that no current project needs it ("That argument still counts for the economy of the workflow's text").
- **Rules out**: a mode for another language; support for merge commits; instructions for a project persons drive; a mode of the checker for another harness; repairing a defect that shows only in a project of another kind; removing a capability on the argument that no current target project uses it.
- **Cases it reaches that the owner may not intend**:
  - `issue@check@configuration-for-several-agent-providers`: any step toward a second harness is ruled out until the owner changes the ask.
  - Finding 4's gate runner, under reading A.
  - "what exists only for such a project may be removed" reaches any code that serves only a project not built by Cargo.
  - The four conditions are worded "holds all of these", and the fourth is the owner's decision to use knowledge-architect: a Rust project driven by agents whose owner has not decided is no target project, so nothing is built to win it over. Admitted only.
- **Sibling goals touched**:
  - The home's head defines "a target project" by this goal, and its first sentence, "the common toolchain of one kind of project", shares this goal's motive.
  - `goal@knowledge-architect@every-target-project-can-adopt-it`: bounded by this goal. Read together, a target project that follows no ask yet, such as one that is no git repository, "can adopt" only as far as "what helps a project into conformance" reaches.
  - `goal@knowledge-architect@the-owner-decides`: holds who changes "what knowledge-architect asks of every target project"; this goal says what an ask is. It also speaks of "a project that follows the workflow", a term that no goal defines since row 31 left.
  - `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` ("A part is taken in by that owner's choice, and not because it is generic") and `goal@knowledge-architect@only-needed-instructions`: the last sentence of this goal exempts both from the rule against narrowing. No conflict.
  - `goal@knowledge-architect@agents-get-a-complete-workflow`, whose list holds "working on a branch under a linear history": agrees with condition 3.
  - `goal@knowledge-architect@the-workflow-improves-through-real-use` ("under the supervision of a person who contributes to the project or owns it"): compatible with "AI agents drive its development" through the clause of finding 3.

---

##### `goal@knowledge-architect@every-target-project-can-adopt-it`

Kind: purpose, with a ranking that qualifies it. The owner's agreement on the list of 21 keeps it one goal (triage of Q85). Rules 1 to 4 and 6 to 8: no finding. It cites nothing.

###### Finding 7. Rule 5, `minor`, `settled`: the sentence on versions means something without the rest

- Goal: "Each target project moves to a new version of knowledge-architect when it chooses."
- Beside it: "What knowledge-architect provides, the checker and the workflow included, holds nothing particular to one target project", and the title, "Every target project can adopt knowledge-architect, and a new one is served first".
- A project that moves between versions when it chooses is a statement about a project that has already adopted. It keeps its meaning with the generality and the ranking removed, and they keep theirs without it.
- Repair implied: the sentence is tied to the purpose it qualifies, or stands as its own statement for the owner to rule. The old goal held it (row 6, "stays in the goal"), so nothing is dropped either way.

###### Checked and no finding

- Rows 1, 4a, 5c and 6 to 9 are carried.
- Rows 5a and 5b, "cited in one readable grammar, the same in every project", are now in `goal@knowledge-architect@documentation-has-a-structure-and-a-model`: "it is written in one readable form, the same in every project". The clause of row 5c, "so what an agent learns in one target project applies in the next", now rests on "holds nothing particular to one target project", as the triage of Q87a says. Moved, not dropped.
- Row 3, "A project declares to the checker the structure particular to it", went to the first goal ("a project declares the kinds of entries it keeps under it").
- Row 4b, "may rely on what knowledge-architect asks", sent to a design issue, is not held.
- Nothing of `installed-text-works-in-every-target-project`, which the rulings made design, is held: the draft has no "works unchanged" and no clause on a project that holds none of knowledge-architect's record.
- The exception of the old row 1, "the tree they are built from aside", left with the mechanism to `design@check@nothing-of-a-project-is-compiled-in`.

###### Boundary block material

- **Asks of future work**: keep everything knowledge-architect provides free of what one project alone holds; let each project move between versions on its own; where a new project and an existing one compete, serve the new one; never weaken a check the workflow needs because an existing project would pay to conform.
- **Requires**: Case: every list a check reads comes from the project's manifest (`design@check@nothing-of-a-project-is-compiled-in`). Case: the workflow names no path or convention of this repository.
- **Admits**: an existing project that conforms over several branches; a project that stays on an old version; a project's own kinds of entries and own checks.
- **Rules out**: a path or convention of one project in the checker or the workflow; a version move forced on a project; dropping or softening a check the workflow needs on the cost to an existing adopter.
- **Cases it reaches that the owner may not intend**:
  - "holds nothing particular to one target project" has no exception left, and "knowledge-architect's own repository is one" target project: what the checker holds for its own tree, such as the pin value `"self"` of `design@check@checked-sentinel-values`, is reached by the words.
  - "Each target project moves ... when it chooses" rules out a release that an adopter must take at once, a security fix included. It also bears on `goal@knowledge-architect@the-workflow-improves-through-real-use`: a change of the workflow reaches a project only when that project moves.
  - "is served first" gives no bound: a helper that only an existing project needs loses to any need of a new project it competes with.
  - "a check the workflow needs to work well" does not cover a check the workflow does not need: there the cost to an existing project still counts as an argument. Admitted only.
- **Sibling goals touched**:
  - `goal@knowledge-architect@serves-one-kind-of-project`: bounds it; "target project" is that goal's.
  - `goal@knowledge-architect@adoption-is-easy`: competes on an existing project ("new or existing", "Adopting asks little work of them" against "by bringing its documents into conformance"). The competition is ranked here: "What that costs it is never an argument against a check the workflow needs to work well."
  - `goal@knowledge-architect@documentation-has-a-structure-and-a-model`: shares the aspect of one reference form in every project, and of a project declaring what is its own.
  - `goal@knowledge-architect@projects-add-their-own-checks` ("with no change to knowledge-architect") and `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` ("What is particular to one project stays in that project"): agree.
  - `goal@knowledge-architect@agents-get-a-complete-workflow` ("an activity that only some target projects carry out included"): "some" is not "one"; no conflict.
  - `goal@knowledge-architect@documentation-stays-consistent`: the second half of the ranking protects its checks.

---

##### `goal@knowledge-architect@adoption-is-easy`

Kind: purpose. Rules 1 to 3 and 6 to 8: no finding. It cites nothing. "without reading its source or asking knowledge-architect's owner" comes from a sentence of the old measure (row 8); as worded it bounds the outcome and states no test, so I report no rule 7 finding.

###### Finding 8. Rule 4, `asks`, `intent`: "Adopting asks little work of them" comes from no row and no statement of the owner

- Goal: "Adopting asks little work of them."
- The old body speaks of documentation only: "finds what to do in its documentation, where they already look for it" and "The documentation says what to do in plain words, with examples." The title alone says "easy".
- The analysis put the words to the owner as a question that was never answered: Q89, "Is \"by one command\" your intent, or is the intent \"with little work\", the one command being `design@create@init-command`'s way?" The triage classes Q89 "SETTLED BY THE RECORD", on that head. The owner's rulings hold no word on "little work".
- What the sentence requires that the documentation sentences do not: work that reduces what an adopter does, apart from what an adopter reads. `design@create@init-command` already argues from this goal that way: "without the command, every file `klarch init` lists as `wrote` but the installed ones and the manifest is written by hand".
- Cases on which the readings differ:
  - An existing project with many documents to move into the workflow's homes. If "little work" is the owner's outcome, the goal asks for means that carry that move. If the goal is about documentation, it asks only that the move be explained.
  - The asks of the scope goal, such as a gates crate. Under the first reading each ask weighs against this goal; under the second it does not.

###### Finding 9. Rule 5, `minor`, `settled`: two statements, each meaning something without the other

- Goal: "finds what to do in its documentation ... in plain words and with examples, without reading its source or asking knowledge-architect's owner" and "Adopting asks little work of them."
- Documentation that says what to do can describe a long adoption, and a short adoption can be undocumented.
- Repair implied: it follows the answer to finding 8. Where the owner holds "little work" as the outcome that the title's "easy" names, the documentation sentence is its qualifier or a second statement to rule; where not, the sentence leaves.

###### Checked and no finding

- Rows 0, 1, 3 and 8 are carried. Row 2, the list of where the documentation sits, is dropped as ruled ("nowhere").
- The two sentences that the rulings sent to design are not held: "One command leaves the project with every file knowledge-architect requires of it" (`design@create@init-command`) and "Adopting loses nothing the project wrote" (the owner: "this is probably not worth a goal IMO. This is more of a small, orthogonal design decision").
- "new or existing" comes from the title of `a-project-adopts-by-one-command`; it states no mechanism.

###### Boundary block material

- **Asks of future work**: an adopter, person or agent, finds each step of adopting in knowledge-architect's documentation, at the places an adopter already looks, in plain words with examples; and, under finding 8, adopting costs the adopter little work.
- **Requires**: Case: each step of adoption is written in documentation, not only in the source of a crate. Case: `design@knowledge-architect@crates-io-page-file`, the crates.io page that sends a reader to the README and docs.rs. Case: `design@check@finding-names-the-repair`.
- **Admits**: an adopter who reads the source by choice; several places of documentation.
- **Rules out**: a step of adoption stated only in the source; a step an adopter learns only by asking knowledge-architect's owner.
- **Cases it reaches that the owner may not intend**:
  - "its documentation" with "plain words and with examples" reaches the setup skill, which is installed text. `goal@knowledge-architect@only-needed-instructions` bounds that: "A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve."
  - "without ... asking knowledge-architect's owner" is not limited to projects of other owners; it reaches an agent in the owner's own project.
  - "new or existing": for an existing project the goal covers the move of its documents, bounded by the ranking of the sibling goal.
  - `issue@knowledge-architect@the-target-project-and-its-asks-are-stated-to-no-adopter` cites this goal: the asks stated to no adopter are a case the goal requires repaired.
- **Sibling goals touched**:
  - `goal@knowledge-architect@every-target-project-can-adopt-it`: that goal says who can adopt and ranks; this one says how hard. They compete on an existing project, ranked there.
  - `goal@knowledge-architect@only-needed-instructions`: bounds it, as above.
  - `goal@knowledge-architect@agents-get-a-complete-workflow`: shares an aspect ("setting up a project"; "needs no instruction from outside them").
  - `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`: under finding 8, a means that lowers the work of adopting is also a task taken in under that goal.
  - `goal@knowledge-architect@serves-one-kind-of-project`: the adopter's project is a target project.

---

##### Met outside the task

- `path@knowledge-architect@docs/goals/owner.md`, `goal@knowledge-architect@the-owner-decides`: "In a project that follows the workflow". The definition of that phrase was row 31 of the scope goal, sent to a design issue; no goal defines it now. For the reviewer of that file.
- `design@git@repository-variables-dropped` states a reading of the scope goal ("which this head reads as a `.git` at or above the project's directory"). Whether it was reported as a goal reading under "Decided" is for the dispatcher to check; see finding 5.
- The analysis's lost-holder table says that the draft of `goal@knowledge-architect@adoption-is-easy` loses two sentences; it does not record that the draft gains "Adopting asks little work of them". See finding 8.

##### Counts

9 findings.

| by rule | count |
| --- | --- |
| rule 3 | 4 (findings 1 to 4) |
| rule 4 | 1 (finding 8) |
| rule 5 | 2 (findings 7, 9) |
| none, (b) | 2 (findings 5, 6) |

| by route | count |
| --- | --- |
| `settled` | 3 (1, 7, 9) |
| `intent` | 4 (2, 3, 4, 8) |
| `contradiction` | 2 (5, 6) |

| by grade | count |
| --- | --- |
| `blocks` | 3 (1, 5, 6) |
| `asks` | 4 (2, 3, 4, 8) |
| `minor` | 2 (7, 9) |

Per goal: `goal@knowledge-architect@serves-one-kind-of-project` 6; `goal@knowledge-architect@every-target-project-can-adopt-it` 1; `goal@knowledge-architect@adoption-is-easy` 2. No goal of the batch is without a finding.

#### The reviewer of: The whole set

Tree: `/home/teo/dev/knowledge-architect/.claude/worktrees/r1-whole`, detached at the commit under review (checked with `git rev-parse HEAD`).

What I established myself:

- `cargo klarch anchors` lists 13 goals homes. The root's is `register@knowledge-architect@goal` (README.md and six files). The 12 Component homes hold no `## ` heading (`grep -c '^## '` gives 0 for each), each with the same two-line "No entry" text.
- The root's home holds 21 goals: model.md 3, built-on-the-model.md 4, workflow.md 6, together.md 2, owner.md 3, adoption.md 3. The files and their order match the list the owner agreed to.
- `cargo klarch check` ends with `PASSED: no findings`. Every reference in the goals home is a a goal reference reference, so rule 8 refuses no citation.
- The brief and the tree agree on every point I checked.

Read whole: the seven files of `register@knowledge-architect@goal`, the review's record, and the section "The owner's rulings" of the analysis. In the analysis I also read the section of every old goal that one of the 21 comes from, and the section "After the rulings". I did not read the milestone document; its words are quoted below as the analysis quotes them.

Abbreviations: "analysis" is `path@knowledge-architect@docs/goal-reviews/2026-11-10-goals-at-the-root-analysis.md`; "rulings" is its section "The owner's rulings". Rule 4 is not judged in this mode; each boundary block gives its material.

---

##### The head of the home, `register@knowledge-architect@goal`

**H1. Rule 2, `asks`, `intent`.** "Its starting point is a documentation structure that it defines, a model that represents that structure, and a scanner." (README.md, lines 5-6). "A scanner" names a means of building the model. No goal uses the word and the head does not define it. The owner's words are "we define a documentation structure, a model representation in Rust and a scanner" (rulings, "The other merges are read again"); the owner later sent "in Rust" to design ("3. design under the scope goal").
Case on which the readings differ: a model built from a project's documents by another means than a scan of the tree, such as through git's object store. Read as an outcome, the head binds a scanner. Read as a mechanism, it does not.

**H2. No rule, `blocks`, `contradiction` (b).** The head defines "**the generic part** of an activity, a tool or a procedure: the part that does not depend on what the project builds" (README.md, lines 31-32). Row 2 of the old `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` reads: "The generic part of one is the part whose procedure does not depend on what the project builds, ...: it serves each target project that carries out its activity, or is meant to." Its destination is "stays in the goal; the one pass states the definition once, in the head of the root's goals home". The head's definition drops "it serves each target project that carries out its activity, or is meant to". The workflow goal keeps a part of it for an activity ("an activity that only some target projects carry out included", workflow.md line 8). No goal keeps it for a tool or a procedure.

**H3. Rule 3, `minor`, `settled`.** Three shared words have no definition in the head, where `skill@knowledge-architect-goal-setting@goal-entry-shape` puts "the definitions its goals share".
- "a session": used by model.md (lines 24-25, 32-33), built-on-the-model.md (lines 34, 38) and workflow.md (lines 53, 60-62). Only "the main session" is defined (workflow.md line 62). The same goals also say "an agent" for the same actor (built-on-the-model.md line 33: "at a lower cost to the agent"; line 34: "each session gets the same result").
- "a tool": the triage of Q61 in the analysis says "Three kept goals of R-list say "tool", so the definition goes in the home's head". The head holds none. The senses differ: "a model of that documentation, which its tools read" (model.md line 13, knowledge-architect's own), "a tool knowledge-architect provides" (built-on-the-model.md line 34), and "tools and procedures" that "Work in one target project sets up or changes" (together.md line 21, a project's own, "a kind of reviewer" and "an external survey" among them).
- "citations" (README.md line 7) stands for what the goals call "a reference".
Repair implied: define "a session" and "a tool" in the head, or replace them, and use one word for a reference.

Boundary material for the head:
- Asks of future work: other goals either add content to the model or add features that work on it; the workflow and the structure are designed together.
- Requires: a defined documentation structure and a model of it.
- Admits: "tools are built upon the model, or outside of it" (line 13).
- Rules out: nothing by its own words.
- The definition of "a recorded item" no longer excludes a plan document (Q58 of the analysis). So a plan document's entries are recorded items for `goal@knowledge-architect@records-reach-their-reader` ("has one home") and for the first goal ("takes a name"). The owner may not intend that.

---

##### model.md

###### `goal@knowledge-architect@documentation-has-a-structure-and-a-model`

**1a. Rule 3, `asks`, `intent`.** "A reference is a pointer by which one text names a recorded item" (model.md line 9). The project's own use of the word is wider: the old definition read "a pointer by which one text names a recorded item, a file, a directory or a section of a text" (analysis, `documents-stay-current` row 1), and the primer's reference has the kinds `path`, `register`, `skill`. Three other goals use the word under this definition: `goal@knowledge-architect@records-reach-their-reader` ("a reference the checker checks"), `goal@knowledge-architect@documents-render-as-a-linked-site` ("each reference is a link to what it names") and `goal@knowledge-architect@documentation-stays-consistent` ("nothing cited there is missing").
Case on which the readings differ: a pointer to a file or to a section of a skill. Under this definition it is no reference. Then "one readable form, the same in every project ... with one reading" does not bind it, the site need not link it, and the model need not say what it names.

**1b. Rule 3, `asks`, `intent`.** "The model judges none of what the documents say. A tool can tell whether what it read is complete and right, and what is wrong where it is not." (model.md lines 14-16). The goal does not say what "judges" means. The old goal did: "To judge what a document says is to report that it breaks a rule on its content, such as a section an entry owes" (analysis, `a-tool-reads-the-documents-with-no-check` row 3). A sibling goal uses the verb with another object: the checker and the tools "judge no meaning" (built-on-the-model.md line 10), while the checker does report breaches of rules on content.
Case on which the readings differ: a reference that names nothing. One agent reads the report of it as "what is wrong" with the read, so it is the model's. Another reads it as judging what a document says, so it is a check's. The old `a-tool-knows-the-read-is-right` ruled it out of the model's answer (its row 8: "the answer naming ... a reference to something missing").

Considered and not reported: the goal holds a structure and a model. A model means nothing without the structure, and the rulings' list gives this goal both, so rule 5 holds. "In Rust" is absent, as the owner ruled. The clause on a tool that can tell whether its read is right is the default the owner took on question 4 of the eight.

Boundary material:
- Asks of future work: one form of reference in every project; a model that every tool of knowledge-architect reads; a way for a tool to know its read is complete and right.
- Requires: a project can declare a kind of entry of its own; each recorded item takes a name; a reference has one reading.
- Admits: a program other than knowledge-architect's reading the model (the default on question 4: "other programs being admitted and not served"); a model in any language.
- Rules out: a reference form a project configures; a model that reports a breach of a rule on content, under the reading of 1b.
- Cases the owner may not intend: a goal is a recorded item (head, line 27), so every goal "takes a name"; a plan item is one too under the head's wider definition.
- Siblings: `goal@knowledge-architect@records-reach-their-reader` takes its definition of a reference from here (finding 3a); `goal@knowledge-architect@every-target-project-can-adopt-it` shares "the same in every project"; `goal@knowledge-architect@documentation-stays-consistent` shares "judge" (1b).

###### `goal@knowledge-architect@design-is-recorded-with-its-arguments`

**2a. Rule 2, `asks`, `intent`.** "The reason of any other decision, where its code or its agent-facing text does not show it, is recorded beside that code or that text and in the message of its commit." (model.md lines 22-23). "and in the message of its commit" names where a reason is written. Changed to "beside that code or that text" alone, the project would still be for a record of what was decided and why.
Case: a decision whose reason is in an inline comment and whose commit message does not repeat it. It fails the goal as written. As a mechanism, a head would close where the reason goes.

The rows of the old goal that stay are all carried (rows 1 to 5). The threshold and the sentence of `records-live-beside-the-code` are absent, as ruled.

Boundary material:
- Asks: each decision that has an entry carries its intent, its argument and the losing alternatives with why they lost; what is outstanding and the flipping evidence are recorded.
- Requires: a recorded decision names the alternative that lost and why.
- Admits: a record kept in an external tracker, now that the sentence on the repository is a design head and no goal; any threshold between an entry and a comment.
- Rules out: a decision recorded with its conclusion alone.
- Case the owner may not intend: the goal no longer says which decision earns an entry, so it is met by a project that records very few decisions in entries (Q3a of the analysis).
- Siblings: `goal@knowledge-architect@agents-work-without-drift` names the same three places as "the design the project has recorded"; `goal@knowledge-architect@the-owner-decides` shares "earns durable text".

###### `goal@knowledge-architect@records-reach-their-reader`

**3a. Rule 8, `asks`, `settled`.** "a reference being what `goal@knowledge-architect@documentation-has-a-structure-and-a-model` says it is" (model.md lines 31-32). The citation is one rule 8 allows. The same rule says "A goal holds what it needs in its own words", and this goal takes a definition from another goal by a pointer. Four goals use the word. Repair implied: the definition of "a reference" goes to the head with the other shared definitions, which also settles its reach (finding 1a), and the pointer leaves.

Rows: the bound "and where a project may write a reference the checker checks" is kept (row 1). Naming and "one reading" moved to the first goal, as the list says.

Boundary material:
- Asks: an item is cited wherever it bears on other work and a checked reference can stand; a session changing an item can list what to read again.
- Requires: one home per recorded item.
- Admits: no citation at a place where no checked reference can be written.
- Rules out: an item with two homes.
- Case the owner may not intend: with a goal as a recorded item, each goal is owed a citation "at each place where it bears on other work", in code comments included.
- Siblings: the first goal (definition); `goal@knowledge-architect@documentation-stays-consistent` ("nothing cited there is missing").

---

##### built-on-the-model.md

###### `goal@knowledge-architect@documentation-stays-consistent`

**4a. Rule 2, `asks`, `intent`.** "The checker and the tools that list the recorded items, as knowledge-architect ships them, support them and judge no meaning, whatever a project's own checks may do." (built-on-the-model.md lines 9-10). The sentence says who judges meaning (reviews and audits) and who does not (the shipped tools). The triage of Q6 left this to the reviewer. My reading: it is a scope that names its excluded design, a shipped check that judges meaning, so it can stay as a qualifier of this purpose. The analysis records that "no word of the owner in the milestone document states it".
Case: a check shipped by knowledge-architect that compares what a document claims with what the code does, by a model's reading. The sentence rules it out. As a decision in a head, an argument could admit it.

**4c. Rule 3, `asks`, `intent`.** "save those the project switches off by a declaration" (lines 14-15) closes a question of how much a project may hold back. The triage of Q11a left the clause to the reviewer.
Case: a manifest field that switches one of the checker's rules off for a project. The goal's words admit it. `design@check@the-regime-has-no-opt-out` refuses it ("There is no flag, no list and no severity", as the analysis quotes it).

Not reported: the example "A mechanical rule, such as that a flag a document names is one the program declares, judges none" is an example the entry shape admits, since without it "judge no meaning" reads as a bar on every rule over content (analysis, row 6). The merge of the two old `check` goals into this one is the rulings' ("stands").

Boundary material:
- Asks: reviews and audits that read documents against code and against each other; a checker whose pass means conformance.
- Requires: a tree that passes meets every rule held for every project and every rule the project declares, with nothing cited missing.
- Admits: a project's own check that judges meaning ("whatever a project's own checks may do"); a breach in a text the checker does not read.
- Rules out: a shipped tool that judges meaning; conformance taken from a reviewer's memory of the rules.
- Cases the owner may not intend: the goal says nothing of when the checker runs, so "at every commit" is design; generated texts are not named.
- Siblings: `goal@knowledge-architect@projects-add-their-own-checks` (the same "own checks"); the first goal (1b).

###### `goal@knowledge-architect@projects-add-their-own-checks`

**5a. Rule 3, `asks`, `intent`.** "It adds them with what knowledge-architect publishes, and with no change to knowledge-architect." (lines 21-22). The owner took the default on question 2 of the eight: "whether a published library API is part of the purpose; default, no, the API is design".
Case: a project's check written as a rule declared in its manifest, with no code built against a published library. One agent reads "with what knowledge-architect publishes" as satisfied by the published checker. Another reads it as code written against what is published, which is the API the ruling sent to design.

Boundary material:
- Asks: a project's own checks run and report with the checker's.
- Requires: no change to knowledge-architect is needed for a project's check.
- Admits: any means of adding a check, under the first reading of 5a.
- Rules out: a check that needs a patched knowledge-architect.
- Siblings: `goal@knowledge-architect@documentation-stays-consistent`; `goal@knowledge-architect@every-target-project-can-adopt-it` ("holds nothing particular to one target project").

###### `goal@knowledge-architect@documents-render-as-a-linked-site`

**6a. No rule, `blocks`, `contradiction` (b).** Old goal, row 1, destination "stays in the goal": "knowledge-architect's documents render as one site, each reference rendered as a link to its target, hosted with the project once it is released." Draft: "A project can host that site with what it releases." (lines 27-28). "hosted" became "can host": a site that is hosted became one a project may host. The owner ruled the reach, "every target project" (question 1 of the eight). The rulings do not record the change of strength.

**6b. Rule 3, `minor`, `settled`.** "The documents of every target project render as one site" (line 26) can be read as one site for all target projects. The title says "A target project's documents render as one linked site". Repair implied: the body says one site per project.

Boundary material:
- Asks: a site build that knowledge-architect provides to every target project. Nothing of it exists (`issue@knowledge-architect@the-documents-do-not-render-as-a-site`).
- Requires: each reference is a link to what it names.
- Admits: any rendering tool; a project that hosts no site.
- Rules out: a resolved reference rendered as plain text.
- Cases the owner may not intend: "the documents" has no bound, so plan documents, open issues and the installed skills are rendered and, where a project hosts the site, published; a private project is owed the same site.
- Siblings: the first goal (which pointers are references, 1a).

###### `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`

**7a. No rule, `blocks`, `contradiction` (b).** Old goal, row 1a, destination "stays in the goal": "a task whose result follows, with no judgement, from the tree ... and that takes several steps or a script the session would write". The draft keeps the first condition ("whose result follows with no judgement from what the project holds", line 37) and drops "that takes several steps or a script the session would write". The triage of Q59a says a draft that drops it "shows the wider reach to the owner"; the record shows nothing yet.
Cases the drop brings in: a task of one command, once the owner has taken it in.

The owner's gate is kept ("and knowledge-architect's owner has taken the task in", line 33), as ruled on question 3 of the eight. "searching, updating and verifying" is the owner's paragraph.

Boundary material:
- Asks: a tool for each task taken in that a tool does cheaper or better.
- Requires: every session gets the same result from the tool.
- Admits: a task the workflow asks and the owner has not taken in, left to the agent.
- Rules out: a task taken in "because the workflow asks it".
- Case the owner may not intend: searching "what its past sessions said" is within reach.
- Siblings: `goal@knowledge-architect@generic-tools-and-procedures-are-shipped` and `goal@knowledge-architect@agents-get-a-complete-workflow` each have a taking-in of their own (a task, the generic part of a tool or a procedure, an activity). `goal@knowledge-architect@the-owner-decides` names one of the three objects, "the generic part of a tool or a procedure" (owner.md lines 14-15).

---

##### workflow.md

###### `goal@knowledge-architect@agents-get-a-complete-workflow`

Conforming. The split ruling is followed: the list of activities and the one clause on completeness are in, and no property of the text. The exclusion of Rust idiom is absent, as the owner ruled on question 5.

Boundary material:
- Asks: a workflow for each activity taken in. Two listed activities have no skill today ("working on a branch under a linear history", "releasing").
- Requires: an agent needs no instruction from outside the workflow and what its project adds.
- Admits: the harness's own mechanisms (the reading of Q38c of the analysis); an activity only some projects carry out.
- Rules out: a workflow that depends on a skill or an instruction outside the installed set and the project.
- Cases the owner may not intend: "complete" beside `goal@knowledge-architect@only-needed-instructions` reads one way only because this goal defines it; "the list grows" stands beside "the workflow does not grow without bound" (line 33). Both can be met: one is on activities, the other on instructions per case.
- Siblings: `goal@knowledge-architect@only-needed-instructions` bounds it; `goal@knowledge-architect@serves-one-kind-of-project` gives the harness.

###### `goal@knowledge-architect@only-needed-instructions`

**9a. Rule 6, `asks`, `intent`** (pair with `goal@knowledge-architect@the-workflow-improves-through-real-use`). This goal: "it is not a text that gains an instruction for each edge case or lapse found" and "A case that judgement handles well enough gets no instruction, whichever goal the instruction would serve" (lines 32-34). The other: "What real use shows unclear, missing or wrong in the workflow reaches knowledge-architect and changes the workflow." (lines 55-56). The two can both be met. They compete under a condition the project's use makes possible: a retrospective reports a lapse that judgement handles well enough. The owner's own words name this condition: "the retrospective and the agent text audit can return findings about lots of edge cases, or lapses" (rulings, first ruling).
Case: such a finding, with an argument that an instruction would lead to a better result. Read by the real-use goal, it "changes the workflow". Read by this one, it gets no instruction. "whichever goal the instruction would serve" gives the order from this side only; the other goal's sentence is unconditional.

Boundary material:
- Asks: instructions limited to what an agent needs.
- Requires: nothing positive beyond that.
- Admits: an instruction for a case judgement handles poorly.
- Rules out: an instruction added for each lapse a review, an audit or a retrospective finds.
- Case the owner may not intend: the clause on structure the work does not need is no longer goal text, so a fixed sequence or count is refused only by a head.
- Siblings: `goal@knowledge-architect@agents-get-a-complete-workflow`; `goal@knowledge-architect@the-workflow-improves-through-real-use` (9a); `goal@knowledge-architect@projects-are-split-into-modular-units` (Q26 of the analysis: no ranking is owed, since this goal is a scope).

###### `goal@knowledge-architect@the-workflow-leads-to-better-code`

**10a. No rule, `blocks`, `contradiction` (b).** Draft: "This goal fixes no definition of quality: what makes code better is learned from use." (lines 39-40). The analysis sends the two old sentences behind the second clause out of the goal: row 2, "What makes code better is hard to define before use, ...", to `design@agent-skills@default-code-shape` as a fact the owner reported; row 5, "and real use corrects it.", to `design@agent-skills@premortem-as-watch-points`. Row 3 alone stays ("So this goal defines no quality:"). The analysis's own draft holds the same clause, so the analysis disagrees with itself. The sibling goal keeps its parallel clause by its row 7 ("what makes a split good is learned from use", "stays in the goal").

Boundary material:
- Asks: what the workflow says about code and tests aims at better code, tests and maintenance tools included.
- Requires: nothing fixed.
- Admits: any shape of guidance; code in any language (the title no longer says Rust; the scope goal gives Rust).
- Rules out: a fixed definition of code quality.
- Siblings: `goal@knowledge-architect@projects-are-split-into-modular-units` (the split of code); `goal@knowledge-architect@only-needed-instructions`.

###### `goal@knowledge-architect@projects-are-split-into-modular-units`

Conforming. The wider wording is the one the list holds, and the body is drawn from the owner's words of round 4 as the analysis quotes them.

Boundary material:
- Asks: the workflow leads to units of one purpose at three layers: parts, the code of a part, design documents by subject.
- Requires: nothing a unit fails alone ("a direction, not a rule").
- Admits: a project of one purpose kept whole; any form of unit at each layer, a module directory included (Q27 of the analysis).
- Rules out: a fixed rule of splitting.
- Cases the owner may not intend: the goal no longer defines a Component or an intermediate Component, so the head that leans on those definitions holds them alone.
- Siblings: `goal@knowledge-architect@the-workflow-leads-to-better-code`; `goal@knowledge-architect@only-needed-instructions`.

###### `goal@knowledge-architect@the-workflow-improves-through-real-use`

**12a. Rule 3, `asks`, `intent`.** "It is the test of how an agent behaves under the workflow." (line 55), beside "This goal forbids no change that is argued to lead to a better result in what a target project implements or records." (lines 57-58).
Case: a mock project built to observe how an agent follows an instruction. Read with "the test" as the only test, the goal rules it out. Read with the last sentence, the goal forbids nothing. The triage of Q29 says the rule on mock runs is a head's and no goal's, so the second reading is the ruled one, and "the test" still carries the first.

See 9a for the pair with `goal@knowledge-architect@only-needed-instructions`.

Boundary material:
- Asks: what real use shows reaches knowledge-architect and changes the workflow.
- Requires: a route from a session in a real target project to knowledge-architect.
- Admits: a change made on argument alone, with no evidence from use (last sentence); a retrospective or any other means of collecting.
- Rules out: by its last sentence, nothing that is argued.
- Cases the owner may not intend: the last sentence generalises the owner's words on D3 ("If you have an argument that the instructions you proposed adding would lead to a better result ... I think the addition should stand. If you have doubts, an issue is fine.") from an added instruction to "no change", and drops the hedge on doubts.
- Siblings: `goal@knowledge-architect@the-owner-decides` ("that owner is knowledge-architect's"); `goal@knowledge-architect@only-needed-instructions`.

###### `goal@knowledge-architect@session-orchestrates-the-work`

Conforming. The body is the owner's round 1 statement with its "so that", and the last sentence states what the owner left to design.

Boundary material:
- Asks: the main session dispatches agents to carry the work. The installed text departs today (`issue@agent-skills@the-installed-text-has-the-main-session-carry-work-it-could-dispatch`).
- Requires: the main session's focus goes to the design and to reporting decisions.
- Admits: any rule of when, and for which kind or size of work.
- Rules out: nothing by its words.
- Case the owner may not intend: read as a purpose, a harness with a far larger context still owes the dispatch (Q44 of the analysis).
- Siblings: `goal@knowledge-architect@design-with-agents-as-counterparts` (the exchange); `goal@knowledge-architect@the-owner-decides` ("reporting each decision taken"). This goal says "the person supervising the work" where those say "the owner".

---

##### together.md

###### `goal@knowledge-architect@agents-work-without-drift`

**14a. Rule 5, `minor`, `settled`.** The second paragraph, "Where two purposes of knowledge-architect compete, this one comes first. ..." (together.md lines 13-17), is a ranking, a kind of its own under rule 1. The analysis says of it: "The ranking paragraph is a second intent under rule 5: it means something without the purpose's text." The repair the standard implies is a goal of its own. The list the owner agreed to reads "`goal@knowledge-architect@agents-work-without-drift`, with its ranking", and the triage of Q1 left rule 5 to the reviewer. So the repair would change a ruled list: the dispatcher records it as ruled, or puts it to the owner.

The ranking's words are the default the owner took on question 6: "never before the owner's decisions, the exchange with the owner and the check of the owner's word", without the attention.

Boundary material:
- Asks: what agents build stays within the recorded design; a reversal is made on purpose and argued.
- Requires: a design not kept comes back only by a change that reopens it on purpose.
- Admits: a reversal of any recorded decision, argued.
- Rules out: a decision reversed by accident; another purpose placed before this one, the three owner goals aside.
- Cases the owner may not intend: the ranking places this purpose before `goal@knowledge-architect@adoption-is-easy`, `goal@knowledge-architect@documents-render-as-a-linked-site`, `goal@knowledge-architect@the-workflow-leads-to-better-code` and every other purpose. It names "the project's owner", so it says nothing of the decisions of knowledge-architect's own owner on asks and on taking in.
- Siblings: the three goals of owner.md (ranked before it); the scope goals bound it by their kind.

###### `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`

Conforming. The sentence on quality tools from `setup-brings-quality-tools` is absent, as the reading of the merges has it. The gate is kept.

Boundary material:
- Asks: the generic part of a tool or a procedure a project set up ships to every target project once taken in.
- Requires: what is particular to one project stays there.
- Admits: any form of shipping; a project that keeps its own copy.
- Rules out: a part taken in "because it is generic".
- Siblings: `goal@knowledge-architect@tasks-a-tool-does-better-are-the-tool-s`; `goal@knowledge-architect@serves-one-kind-of-project` holds the exception for this choice (adoption.md lines 37-38); `goal@knowledge-architect@the-owner-decides`.

---

##### owner.md

###### `goal@knowledge-architect@the-owner-decides`

**16a. Rule 3, `asks`, `intent`.** "Each decision the agents make that earns durable text, and each choice they make that the owner could have ruled, reaches the owner" (owner.md lines 17-18). "earns durable text" takes its meaning from the design's tests of what earns a head or a comment, and the goal no longer holds the old exclusion "other than an implementation choice that a commit message records" (row 18, sent to a head).
Case: an implementation choice whose reason is written in a commit message. A commit message is durable text. One agent reads the choice as one that "reaches the owner" and owes a line of report. Another reads the commit message as the reaching. A third reads the choice as outside, as the head says.

**16b. Rule 5, `minor`, `settled`.** The second paragraph gives knowledge-architect's own owner "each addition, tightening or removal of what knowledge-architect asks of every target project, and whether and when it takes in the generic part of a tool or a procedure" (lines 13-15). It means something without the first paragraph: another owner and another subject. The triage of Q49 classes it as answered, with the limit "the ruling is by exception on a summary row, not on Q49 by its label". Repair implied by the standard: a goal of its own, or the two members stated in the goals that own them. The dispatcher records it as ruled, or puts it to the owner.

See 19a: after the one pass, no goal says the order of known work is the owner's.

Boundary material:
- Asks: agents decide what argument settles; the owner sees each such decision.
- Requires: a fork and an unresolved contradiction go to the owner; every action that cannot be undone is the project's owner's.
- Admits: a project that reserves further actions; a question on a goal's reading where no argument ranks two readings, under "a choice between shapes that all survive the argument".
- Rules out: an agent's decision the owner cannot see.
- Cases the owner may not intend: the closed list ("and nothing else") is gone, so the goal no longer rules out a question sent to the owner that an argument settles; the taking-in of an activity or of a task is not in the second paragraph (see the siblings of the tasks goal).
- Siblings: `goal@knowledge-architect@agents-work-without-drift` (ranked after this goal); `goal@knowledge-architect@the-workflow-improves-through-real-use`; `goal@knowledge-architect@serves-one-kind-of-project` ("asks").

###### `goal@knowledge-architect@the-owner-s-word-is-verified`

Conforming. It matches the analysis's draft, and no row that stays is dropped.

Boundary material:
- Asks: a check of what a word of the owner assumes, before any act on it, in every activity.
- Requires: nothing is built on an unresolved contradiction.
- Admits: any occasion for the check of a goal's premises ("other than a goal").
- Rules out: work built on a word whose premise the project's own evidence shows false.
- Case the owner may not intend: the installed text asks the check only of a premise the session supplied (analysis, "Met outside the task" of the theme), so the goal asks more than is built.
- Siblings: `goal@knowledge-architect@design-with-agents-as-counterparts`; `goal@knowledge-architect@agents-work-without-drift`.

###### `goal@knowledge-architect@design-with-agents-as-counterparts`

**18a. Rule 3, `asks`, `intent`.** "The owner meets each proposal with its grounding and its consequences established" (owner.md lines 33-34). Old row 4, "stays in the goal": "The owner meets each proposal of the exchange with its consequences". The words "of the exchange" are gone.
Case: a design an agent settles by argument, with no fork, builds and reports under "Decided". Read with "of the exchange", the goal does not reach it. Read without, the owner is owed that proposal "while the owner can still challenge it". `goal@knowledge-architect@the-owner-decides` admits the first ("Agents decide, by argument"), and the owner said "If there is nothing else I can give other than approval, there is little value in bring the question to me" (rulings).

The sentence "so that the design taken is the one that, under argument, fulfils the goals best at acceptable costs" follows the owner's words of round 2 as the analysis quotes them. "backed by the measurements the agent takes" and "with its grounding ... established" carry rows 6b and 7 of the old attention goal, as the table of lost holders asks.

Boundary material:
- Asks: an exchange in which both sides propose and challenge.
- Requires: a proposal comes with its grounding, its consequences and its rival.
- Admits: any moment for the owner's meeting of a proposal ("before the merge" is a head's).
- Rules out: a proposal whose consequences the owner must work out.
- Siblings: `goal@knowledge-architect@the-owner-decides` (18a); `goal@knowledge-architect@session-orchestrates-the-work`.

---

##### adoption.md

###### `goal@knowledge-architect@serves-one-kind-of-project`

**19a. No rule, `blocks`, `contradiction` (b).** Old goal, row 29d, destination "stays in the goal": "and for the order of known work, which is knowledge-architect's owner's." Draft: "and for the order of known work." (adoption.md line 38). "which is knowledge-architect's owner's" is dropped. The triage of Q51 rests on those words: "row 29d of `goal@knowledge-architect@serves-one-kind-of-project` keeps "the order of known work, which is knowledge-architect's owner's"". `goal@knowledge-architect@the-owner-decides` lost its own mention (old row 5c, "such as the order of known work", sent to a head). So no goal of the 21 says whose the order of known work is. This sits between adoption.md and owner.md.

**19b. Rule 3, `asks`, `intent`.** Two clauses close a question of how much: "Files the workflow installs, the configuration of a continuous integration and scripts kept beside the code are no code it ships" (lines 11-13) and "An edit that a person makes now and then leaves it a target project" (lines 14-15). The triage of Q81a left both to the reviewer.
Cases: a project that keeps a build script in Python, and a project whose owner writes one module by hand. Each clause admits its case by a wording a head could carry.

Followed as ruled: the four statements on asks in one passage (question 7), git and the `claude` harness named as asks (question 8), "knowledge-architect's own repository is one" (the owner: "Since we are ourselves a target project"), and the fourth case of the last sentence (rows 29c and 5).

Boundary material:
- Asks: everything built serves a Rust project that agents drive under a linear history and whose owner chose knowledge-architect.
- Requires: nothing built or repaired only for another project; nothing built only for a state that follows no ask, other than a help into conformance.
- Admits: use in any other project with no guarantee; a target project that does not yet follow an ask.
- Rules out: a mode for another language; support for merge commits; instructions for a project persons drive; a mode of the checker for another harness; narrowing the conditions on what a current target project needs.
- Cases the owner may not intend: a defect that shows only where an ask is not followed is no longer named (the guarantee sentence covers it by inference); the gates asks are not named, so a project's own gate runner is refused only by design (Q75 of the analysis); "its reviews" are agent-driven, beside "easier for a person to review" in workflow.md line 47.
- Siblings: every goal that says "target project"; `goal@knowledge-architect@the-owner-decides` ("asks"); `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`.

###### `goal@knowledge-architect@every-target-project-can-adopt-it`

Conforming on its own text. The sentence on what a project declares is absent (the first goal holds the declaring), and the form of a citation is in the first goal. See 21a for the pair.

Boundary material:
- Asks: nothing particular to one project in what is provided; each project moves versions when it chooses.
- Requires: a new project is served first; an existing one can adopt.
- Admits: an adoption over several branches.
- Rules out: the cost of conformance used against a check the workflow needs.
- Case the owner may not intend: the pin is not named, so "moves ... when it chooses" is met by any means.
- Siblings: `goal@knowledge-architect@adoption-is-easy` (21a); `goal@knowledge-architect@serves-one-kind-of-project`.

###### `goal@knowledge-architect@adoption-is-easy`

**21a. Rule 6, `asks`, `intent`** (pair with `goal@knowledge-architect@every-target-project-can-adopt-it`). This goal: "A person or an agent adopting knowledge-architect in a target project, new or existing, ... Adopting asks little work of them." (adoption.md lines 52-55). The other: "An existing one can adopt it too, by bringing its documents into conformance. What that costs it is never an argument against a check the workflow needs to work well." (lines 46-48). They compete under a condition: an existing project with many documents. The other goal's sentence ranks a check before the cost. Nothing ranks "little work" against the rest of what adoption asks of an existing project, such as moving its documents into the workflow's homes. "Adopting asks little work of them" is new wording (the old title said "easy"), from the question of Q89.
Case: an instruction that makes an existing project's adoption longer and that no check needs.

The two sentences from the old `create` goal are absent, as ruled ("by one command" is a head; "loses nothing the project wrote" is, in the owner's words, "a small, orthogonal design decision").

Boundary material:
- Asks: documentation an adopter finds where they look, in plain words with examples.
- Requires: no step of adoption that needs the source or knowledge-architect's owner.
- Admits: any number of commands; an adopter who reads the source by choice.
- Rules out: a step of adoption stated only in the source.
- Siblings: `goal@knowledge-architect@every-target-project-can-adopt-it`; `goal@knowledge-architect@agents-work-without-drift` (ranked before it).

---

##### Conflicts between goals of different files

None where one goal rules out what another requires. Three points sit between files:

- 19a: adoption.md and owner.md no longer say between them whose the order of known work is.
- 1a and 3a: a definition in model.md governs "reference" in built-on-the-model.md, and is narrower than the project's own use.
- 18a: owner.md's "each proposal" beside `goal@knowledge-architect@the-owner-decides`.

Checked and found compatible:

- The drift ranking against the ranking inside `goal@knowledge-architect@every-target-project-can-adopt-it`.
- "judge no meaning" against a project's own checks.
- The scope goal's "its reviews" against a person's review in workflow.md.
- "the list grows" against "does not grow without bound".
- The three takings-in (a task, an activity, the generic part of a tool or a procedure).

##### Decided

- Rule 5 is not reported against the merge inside `goal@knowledge-architect@documentation-stays-consistent`, nor against the ranking inside `goal@knowledge-architect@every-target-project-can-adopt-it`. Ground: the rulings' table says the first merge "stands", and the second ranks within its own purpose (Q85 of the analysis). 14a and 16b are reported as `minor` because the triage left the first to the reviewer and the second was ruled on a summary row only.
- No finding of rule 1 is raised against a goal whose fate the owner ruled as kept. Ground: the rulings, "every fate is ruled as proposed".
- 10a is filed as (b) and not as a rule 7 measure. Ground: the brief's line on statements the analysis sent to a head.
- 3a takes the route `settled` where the table gives `intent` for rule 8. Ground: the entry shape names the repair.
- H3 and 6b take the grade `minor`. Ground: the title, or the context, gives the one reading.

##### Met outside the task

- The analysis disagrees with itself on `goal@knowledge-architect@the-workflow-leads-to-better-code`: rows 2 and 5 send "learned from use" to heads, and its own draft keeps the clause (10a).
- The triage of Q61 put a definition of "a tool" in the head; the one pass wrote none (H3).
- `issue@agent-skills@the-workflow-assumes-one-unnamed-owner` bears on the three names for the person the goals use: "the owner", "the person supervising the work", "a person who contributes to the project or owns it". Not opened or read by me.
- I extracted sections of the analysis to scratch files under the session's scratchpad, outside the worktree, to read them, and deleted them before this report. The brief said to write no file. Nothing of the repository was written; `git status --short` is empty.

##### Counts

23 findings.

| per rule | count | findings |
| --- | --- | --- |
| 2 | 3 | H1, 2a, 4a |
| 3 | 10 | H3, 1a, 1b, 4c, 5a, 6b, 12a, 16a, 18a, 19b |
| 5 | 2 | 14a, 16b |
| 6 | 2 | 9a, 21a |
| 8 | 1 | 3a |
| none, (b) | 5 | H2, 6a, 7a, 10a, 19a |

| per route | count |
| --- | --- |
| `contradiction` | 5 |
| `intent` | 13 |
| `settled` | 5 |

| per grade | count | findings |
| --- | --- | --- |
| `blocks` | 5 | H2, 6a, 7a, 10a, 19a |
| `asks` | 14 | H1, 1a, 1b, 2a, 3a, 4a, 4c, 5a, 9a, 12a, 16a, 18a, 19b, 21a |
| `minor` | 4 | H3, 6b, 14a, 16b |

Conforming with no finding of their own: `goal@knowledge-architect@agents-get-a-complete-workflow`, `goal@knowledge-architect@projects-are-split-into-modular-units`, `goal@knowledge-architect@session-orchestrates-the-work`, `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`, `goal@knowledge-architect@the-owner-s-word-is-verified`, `goal@knowledge-architect@every-target-project-can-adopt-it`.
