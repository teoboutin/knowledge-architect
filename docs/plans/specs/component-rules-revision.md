# The rules for splitting a project into Components, revised from their first use on this repository, and a guideline that integrated tests drive the project through its public API

## Status and audience

This spec is the plan of a change to the installed text alone. It revises
`skill@knowledge-architect-component-creation`, the fragment the design and development skills
host on when to split, `path@agent-skills@fragments/when-to-split.md`, and the fragment of the
default shape of code, `path@agent-skills@fragments/default-code-shape.md`, which the development
skill and the code-shape reviewer host. It writes no Rust.

It comes from one design discussion, held in one session, which began at the owner's message "New
rules relative to how components and code in projects following knowledge-architect should be split
have just been added." The discussion applied the component-creation skill to this repository, and
compared its result with the owner's own plan. The same discussion produced
`milestone@plans@component-split`, the split of this repository, which this spec's work does not
build. The roadmap orders this spec first and the milestone second, on the owner's word.

- The spec is written for a session that did not witness the discussion. It leaves the repository
  in a commit of its own, after its harvest and the review before the merge.
- Where this spec and a design home disagree on a point "What is already decided" does not list as
  rewritten, the design home wins, and the disagreement is a defect of this spec.
- Every name it uses is defined under Names or New names, or exists in the tree.
- Where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what
  waits. No default awaits the owner today.

## How the work is done

By `skill@knowledge-architect-planning`, the work of a spec. Each edit of the installed text is
written as `path@agent-skills@CLAUDE.md`, "Editing an installed skill or agent", says, read whole,
under `skill@knowledge-architect-agent-configuration`, whose admission standard judges each
statement added.

## Names

- **the component-creation skill**: `path@agent-skills@content/skills/component-creation/SKILL.md`,
  installed as `skill@knowledge-architect-component-creation`. Its sections: `skill@knowledge-architect-component-creation@component-criteria`
  "What a Component is by default", `skill@knowledge-architect-component-creation@split-signals` "Signals against a split", `skill@knowledge-architect-component-creation@component-crates`
  "Components and crates", `skill@knowledge-architect-component-creation@component-shapes` "Shapes beside one crate", `skill@knowledge-architect-component-creation@component-naming`,
  `skill@knowledge-architect-component-creation@creation-procedure`.
- **the fragment on when to split**: `path@agent-skills@fragments/when-to-split.md`, hosted by the
  design skill's section `skill@knowledge-architect-design@when-to-split` and by the development
  skill.
- **the code-shape fragment**: `path@agent-skills@fragments/default-code-shape.md`, hosted by
  `skill@knowledge-architect-development` and `agent@knowledge-architect-code-shape-reviewer`.
- **the module-directory exception**: the paragraph of `skill@knowledge-architect-component-creation@component-crates` that opens "A Component
  may be a module directory inside a crate".
- **the shared test Component**: the first bullet of `skill@knowledge-architect-component-creation@component-shapes`.
- **R1 to R6**: the six rounds of the discussion, each one message of the owner and the reply; the
  owner's words quoted in this document carry their round.
- **hard point 1 to 4**: the four places where the session found the component-creation skill hard
  to apply, R1: the module-directory exception, the test of two changes applied to commands, the
  usable-alone weight, and the shared test Component; threads 1, 2, 3 and 6 below revise them.
- **the table, the checkpoint**: the session's tables of threads, which the owner's words "I agree
  with your table now" (R3) and "Agreed on the table" (R6) answer; each thread's item says which.
- **P1, P2**: the two shapes of where a command's arguments are parsed, argued in
  `milestone@plans@component-split`. P1: the top-level crate parses every command, and the domain
  crates expose a typed API with no clap in it. P2: each domain crate derives clap's types for its
  own commands, and the binary flattens them.

## What the work is

What the texts say today, read in the source at the branch that adds this spec:

- `skill@knowledge-architect-component-creation@component-criteria` lists four default criteria: one purpose; "an interface with a reason to
  change on its own", tested by "two concrete changes someone could plausibly ask for, each
  touching it independently"; its own tests; "takes decisions of its own". Nothing in it says how
  commands or functions that share one purpose and one design are placed, so the test of two
  changes, applied to one command at a time, passes for every command (#commands-sharing-a-purpose).
- `skill@knowledge-architect-component-creation@split-signals` lists three signals: many new dependencies, a cycle, and "every change you expect
  to make crosses the new boundary". The fragment on when to split restates the three in one sentence.
  Nothing says the expected changes include those the goals predict (#expected-changes-from-goals).
- `skill@knowledge-architect-component-creation@component-crates` holds a table whose row for a publishing project ends "weigh it against how
  usable each crate is alone for others", which reads as weighing both ways
  (#usable-alone-one-direction), and the module-directory exception, which applies "where a crate
  of its own would force a publication or a dependency cycle" and does not say what to do when the
  cycle comes only from a recorded decision that can be reversed
  (#cycle-from-a-reversible-decision).
- `skill@knowledge-architect-component-creation@component-shapes` offers the shared test Component "where several crates share testing tools and
  fixtures without depending on each other", and warns that a dev-dependency back onto a dependent
  crate builds the crate twice. It says nothing of integrated tests, or of a crate that supports
  extensions (#test-component-public-api).
- The code-shape fragment states four properties of the default shape and a table of test kinds,
  and says nothing of an integrated test (#test-through-public-api).

Outside this work:

- the split of this repository, `milestone@plans@component-split`, which applies the revised rules;
- the setup skill's mapping of an adopting project's crates to Components, which points to the
  component-creation skill and changes with it alone;
- `design@core@api-facade`, whose sentence on unit-test modules the milestone's testing slice
  rewrites, since the facade is code of this repository, not installed text.

## What is already decided

**Decisions the work rests on, not argued again:**

- `goal@knowledge-architect@projects-are-split-into-components`: the criteria are guidelines that
  real use corrects; this discussion is that use.
- `design@agent-skills@component-skill-delivery`: the skill holds the criteria; the fragment states
  the occasion and the signals; the design and development skills host it. No host is added.
- `design@agent-skills@shared-text-fragment`: a fragment's host list is the owner's; the work adds
  none.
- `design@agent-skills@behaviour-additions-need-real-use`: the evidence of every addition is this
  session, in which the session applied the skill to this repository at the owner's request, and
  the session and the owner named what it lacked; the
  wording of #usable-alone-one-direction allows two acts, a defect provable by reading.
- `design@agent-skills@capabilities-not-structure`, `design@agent-skills@statement-names-an-act`,
  `design@agent-skills@admission-rules-home`, `design@agent-skills@instruction-record-is-minimal`:
  each added statement names an act, imposes no fixed mapping, and keeps its reason in a `%%` line
  unless an entry test gives it a head.
- `design@agent-skills@local-intent-binds`: code an integrated test cannot reach is suspect, never
  deleted on that ground alone.
- `rejected@core@unit-tests-moved-under-tests`: its measurement, 31 files and 349 tests "most over
  private functions", rules out a guideline that moves every unit test to the public API. The
  guideline applies to integrated tests only (#test-through-public-api).
- `design@agent-skills@subject-split-standard`: it defines a subject as a part of a Component, so the
  new criterion uses the word "domain" (#one-main-subject-per-component).

**Decisions the work rewrites**, with every text `cargo klarch show` lists as citing them:

| head | rewritten by | citing texts, and who judges each |
| --- | --- | --- |
| `design@agent-skills@component-crate-mapping` | #usable-alone-one-direction, #cycle-from-a-reversible-decision | the component-creation skill's `%%` line in `skill@knowledge-architect-component-creation@component-crates`, edited by step 2; `path@agent-skills@docs/grounding/crate-boundaries.md`, read at step 2 and left unchanged, since it records Cargo's rules |
| `design@agent-skills@component-split-signals` | #expected-changes-from-goals | the skill's `%%` line in `skill@knowledge-architect-component-creation@split-signals`, step 1 |
| `design@agent-skills@shared-test-component` | #test-component-public-api | the skill's `%%` line in `skill@knowledge-architect-component-creation@component-shapes`, step 3; `path@agent-skills@docs/grounding/crate-boundaries.md`, read at step 3 |
| `design@agent-skills@default-code-shape` | #test-through-public-api, for the property it lists, which cites the new head | its design-home citations in `path@agent-skills@docs/design/components.md`, `path@agent-skills@docs/design/development.md`, `path@agent-skills@docs/design/reviews.md` and `path@agent-skills@docs/design/shipped-set.md`, the fragment's own `%%` line, and `tripwire@agent-skills@shape-findings-fragment-code`, which cites it twice, read at step 4; the tripwire is unchanged in scope |
| `design@agent-skills@test-economy` | #test-through-public-api | its sentence "for a golden test, a test with heavy mocks or a nondeterministic one the writer picks the evidence" gains that a test's kind is decided by what it exercises, so a test of logic run in a mock environment, or through the public API, owes its mutation; citing texts: `agent@knowledge-architect-code-shape-reviewer`'s source, `path@agent-skills@docs/design/reviews.md`, the fragment, and `tripwire@agent-skills@logic-tests-relabelled`, which cites it twice, read at step 4; the harvest judges whether the tripwire's firing clause, which names golden, mocked or external, names the integrated test |

**The discussion's record search** graded 193 entries `bears` and 110 `may-bear`, for this spec and
the milestone together, counted in the files its check assembled, which the tree does not hold. The entries that bear on this spec, with their outcome:

- the goals of agent-skills, `goal@agent-skills@no-contradiction-in-workflow`,
  `goal@agent-skills@one-skill-per-activity`, `goal@agent-skills@only-needed-instructions`,
  `goal@agent-skills@installed-text-works-in-every-target-project` and
  `goal@agent-skills@reviews-judge-by-the-writer-s-criteria`: criteria of this spec;
- `goal@knowledge-architect@agents-get-a-complete-workflow`,
  `goal@knowledge-architect@the-workflow-leads-to-better-code`,
  `goal@knowledge-architect@the-workflow-improves-through-real-use`,
  `goal@knowledge-architect@projects-are-split-into-components`: criteria of this spec;
- `goal@knowledge-architect@agents-work-without-drift`,
  `goal@knowledge-architect@design-is-recorded-with-its-arguments`,
  `goal@knowledge-architect@design-with-agents-as-counterparts`,
  `goal@knowledge-architect@the-owner-decides`, `goal@knowledge-architect@the-owner-s-word-is-verified`:
  they bound how the work is done and recorded, and constrain no thread;
- `goal@knowledge-architect@every-target-project-can-adopt-it`,
  `goal@knowledge-architect@serves-one-kind-of-project`,
  `goal@knowledge-architect@generic-tools-and-procedures-are-shipped`: the text names no convention
  of this repository; "a crate that supports extensions publishes its test tools" is a default shape
  of the skill, no ask;
- the five rewritten heads above and the decisions listed above
  as rested on;
- `design@agent-skills@restatement-size-test` and `rejected@agent-skills@directive-restated-everywhere`:
  the fragment on when to split keeps one sentence of the signals, beside its pointer, and step 1
  rewrites it with the skill;
- `design@agent-skills@term-defined-at-first-load`: "domain" and "integrated test" are defined
  where each text first uses them;
- `design@agent-skills@development-skill`, `design@agent-skills@code-shape-axis`,
  `design@agent-skills@review-judges-additions`, `design@agent-skills@code-axes-rows-home`: the
  guideline reaches the writer and the code-shape reviewer through the fragment; no axis is added;
- `design@agent-skills@premortem-as-watch-points`: the retrospective's standing question on rules
  about code and about splitting watches the revised rules; nothing is added to it;
- `design@agent-skills@a-head-is-owed-by-an-entry-test`, `design@agent-skills@new-or-reshaped-head-needs-design`,
  `design@agent-skills@existing-heads-on-touch`, `design@agent-skills@design-home-write-loads-recording`,
  `design@agent-skills@extension-follows-the-ground`, `design@agent-skills@one-decision-per-head`,
  `design@agent-skills@title-states-the-rule`, `rejected@agent-skills@title-states-built-instance`,
  `design@agent-skills@losing-alternatives-filter`, `design@agent-skills@losing-shape-test`: they
  govern the harvest;
- `design@agent-skills@context-cost-of-agent-text`, `design@agent-skills@shipped-text-cites-no-entry`,
  `design@agent-skills@synthetic-evidence-not-built`: they bound the edits; no conflict;
- `tripwire@agent-skills@agent-text-written-without-the-standard`,
  `tripwire@agent-skills@open-fork-classed-as-settled`, `tripwire@agent-skills@record-search-misses`,
  `tripwire@agent-skills@shape-findings-fragment-code`, `tripwire@agent-skills@logic-tests-relabelled`:
  none fires on this work; the last is a cause of the premortem.

Every other entry the search graded `bears` or `may-bear` is listed, with its outcome, in
`milestone@plans@component-split`; none of them bears on this spec beyond what it lists there. The
two `may-bear` entries read whole are `design@agent-skills@subject-split-standard`, which bears, as
#one-main-subject-per-component says, and `design@agent-skills@losing-shape-test`, which governs the
harvest of a shape rejected with nothing built in its place and meets nothing here.
`design@agent-skills@audit-net-growth` bears: the landing commit counts the instructions the steps
add and remove, and the owner is told the net figure, as the admission standard says.

## Criteria

### The installed text states default criteria for placing work in Components and splitting them, as guidelines that real use corrects `##split-criteria-are-guidelines`

Binding, from `goal@knowledge-architect@projects-are-split-into-components`. Met by every thread:
each revision keeps the criteria guidelines and adds no rule a Component fails alone.

### No two installed instructions leave a session no move that obeys both `##no-contradiction`

Binding, from `goal@agent-skills@no-contradiction-in-workflow`. Met by #cycle-from-a-reversible-decision,
which states when the module-directory exception does not apply, and by step 1, which edits the
fragment on when to split with the skill.

### Each added statement names an act the workflow needs, and imposes no structure the work does not need `##only-needed`

Binding, from `goal@agent-skills@only-needed-instructions`. Met by each thread's statement, judged at
the review on agent-facing text.

### An addition about how an agent behaves rests on real use or on a defect provable by reading `##real-use-evidence`

Binding, from `goal@knowledge-architect@the-workflow-improves-through-real-use`. Met: the discussion
of this spec is the real use; #usable-alone-one-direction is also a wording that allows two acts.

### A rule about code or tests is a default shape, not a rule every unit meets `##guideline-not-rule`

Binding, from `goal@knowledge-architect@the-workflow-leads-to-better-code`. Met by
#test-through-public-api, which states a preference with two branches.

### No guideline moves the unit tests a crate holds over its private functions `##unit-tests-stay`

Binding as a presumption, from `rejected@core@unit-tests-moved-under-tests`. Met by
#test-through-public-api, whose scope is the integrated test.

### The installed text names no convention of this repository `##portable-text`

Binding, from `goal@agent-skills@installed-text-works-in-every-target-project`. Met by every step:
the examples are placeholders or general cases, such as "a crate that supports extensions", never
this repository's crates.

## Threads

### A dependency cycle that comes only from a recorded decision that can be reversed sends the session to reverse that decision, not to the module-directory exception `##cycle-from-a-reversible-decision`

Proposed by the session, R1, as hard point 1 (#arg-22); restated as a revision in R2 (#arg-52).
Final state: approved. The owner, R2: "1. agreed this is not conveyed in a clear way. About the
binary case: I think it is better to have the binary as a "top level" crate, such as the cli crate
you proposed, which pulls the logic from the others and implements the interface, potentially
pulling rendering logic from another crate if that applies (such as a project with a gui). Though
the difficulty here mostly came from our own recorded decision, which is quite old, made for
reasons that no longer apply identically, and can be reversed." R3: "1. Agreed". Arguments:
#arg-20, #arg-22, #arg-32, #arg-33, #arg-52, #arg-60. Shape: "Decided design", the same title. Harvest:
`design@agent-skills@component-crate-mapping`, rewritten.

### Commands or functions that share one purpose and one design are one Component, one module each, and a top-level binary crate holds the interface and pulls its logic from below `##commands-sharing-a-purpose`

Proposed by the session, R1, as hard point 2 (#arg-23); the owner's argument, R2 (#arg-34,
#arg-35); restated in R2 (#arg-53). The sentence on where parsing lives comes from the milestone's
thread #command-parsing-home, closed in R5. Final state: approved. The owner, R2: "2. I think your
cut is good, because crates such as, for example, rewrite is better shaped as one crate with
modules rather than one per functionality, because the subcommands it provides share most of the
design, ideas and have one centralized purpose (rewrite history), and only differ on subject and
application. Same for the subcommands of  the cli binary: grouping the parsing logic, and any
meaningfull verification that can be applied at parsing, is more convenient, while the application
logic should be mostly pulled from other crates (or assembled from high level logic that is
provided by other crates)". R5: "command-parsing-home I think your synthesis is right." Arguments:
#arg-23, #arg-34, #arg-35, #arg-53, #arg-61, #arg-71, #arg-72, #arg-73, #arg-74, #arg-75, #arg-78,
#arg-100. Shape: "Decided design". The thread holds two decisions, cut at the harvest: that
commands sharing one purpose and one design are one Component, stated by the skill alone, takes a
`%%` line, since the `query` head the milestone harvests cites the skill rather than restating the
rule; that a top-level binary crate holds a binary's interface, and parses each command in the
lowest crate every binary offering it depends on, is stated by the skill and by a head of the
milestone's `cli` Component, so it takes a head of agent-skills, which `cli`'s head cites.

### Being usable alone argues for a split, and not being usable alone argues nothing `##usable-alone-one-direction`

Proposed by the session, R1, as hard point 3 (#arg-24); the owner's argument, R2 (#arg-36);
restated in R2 (#arg-54). Final state: approved. The owner, R2: "3. This weight should be considered
minor. It is mostly a lead, and I think it should also be single-direction. It can push in the
direction of splitting if the crate would be useable alone. It should not push in the other
direction if the crate would not be." R3: "3. agreed". Arguments: #arg-24, #arg-36, #arg-54,
#arg-60. Shape: "Decided design". Harvest: `design@agent-skills@component-crate-mapping`, rewritten.

### The split signals are read against the changes the goals predict, as well as the changes made so far `##expected-changes-from-goals`

Proposed by the session, R2 (#arg-59), from the owner's argument on the model's future consumers
(#arg-28, #arg-30). Final state: approved, by the owner's word against the checkpoint, R6: "Agreed
on the table." Arguments: #arg-16, #arg-28, #arg-30, #arg-45, #arg-59. Shape: "Decided design". Harvest:
`design@agent-skills@component-split-signals`, rewritten.

### A Component takes decisions about one main domain, with small side domains allowed, the word "domain" defined where the skill first uses it `##one-main-subject-per-component`

Proposed by the owner, R3 (#arg-66); the word changed by the session, R4, after the record search
(#arg-92). Final state: approved. The owner, R3: "(this is probably a good way to phrase it in the
skill, a crate/component must take decisions about  approximately one main subject, maybe with
smaller side subjects allowed)". R5: "one-main-subject-per-component: agreed on the domain word."
Arguments: #arg-65, #arg-66, #arg-92, #arg-105. Shape: "Decided design". Harvest: a `%%` line in
the skill's `skill@knowledge-architect-component-creation@component-criteria`; no head, since one text states it.

### The shared test Component holds the integrated tests that drive the Components through their public API, and a crate that supports extensions publishes its test tools `##test-component-public-api`

Proposed by the session, R1, as hard point 4 (#arg-25); the owner's argument, R2 (#arg-37 to
#arg-41); restated in R2 (#arg-55, #arg-58). Final state: approved, by the owner's word against the
checkpoint, R6: "Agreed on the table." The owner, R3: "4. agreed." Arguments: #arg-25, #arg-37,
#arg-38, #arg-39, #arg-40, #arg-41, #arg-55, #arg-57, #arg-58, #arg-60, #arg-110. Shape: "Decided
design". Harvest: `design@agent-skills@shared-test-component`, rewritten.

### An integrated test drives the project through its public API, as a consumer would: what it cannot observe there is hidden from consumers too, and what no consumer could observe is a unit test's claim `##test-through-public-api`

Proposed by the owner, R3 (#arg-62, #arg-63); tested by the session against the record in R4
(#arg-79, #arg-83 to #arg-86); scoped by the owner in R5 and R6. Final state: approved. The owner,
R3: "I think this is a larger principle that could be recorded in the development skill as a
guideline: testing through public API when possible because it drives improvement of said public
API, shows were it is lacking, and benefits consumers. Also, it shows if some code is unreachable or
has unobervable effects from public API (that cannot be unit tested internally either), and this is
probably a smell of bad design or dead code. We will need to come up with a proper
principle/guideline around this." R5: "What I meant is an integrated tests, that mocks real world
application. Such a test uses the public API just like a consumer would. If it cannot examine the
returned objects fromthe public API in sufficient detail, this means that things are being hidden
from public consumers, which is a problem. If the test is trying to reach into an internal that
would have no observable effect to a public consumer, then the test is in the wrong place and
should not be an integrated test using mock proejcts." R6: "About test-through-public-api: your
wording uses mock project, but not all projects are meant to apply to other projects, and not all
use mocks. The important point is that the integrated test drives the project through public API,
as a consumer would. It is often associated with a mocking environment, but not necessarily."
Arguments: #arg-37, #arg-39, #arg-57, #arg-62, #arg-63, #arg-79, #arg-83, #arg-84, #arg-85,
#arg-86, #arg-101, #arg-102, #arg-107, #arg-108, #arg-109, #arg-117, #arg-123, #arg-124. Shape:
"Decided design". Harvest: a new head of agent-skills, which `design@agent-skills@default-code-shape`
cites, and `design@agent-skills@test-economy` rewritten, as the harvest table says.

## Arguments

The arguments of the discussion that bear on this spec's threads, numbered as the discussion's
extraction numbered them, so this spec and `milestone@plans@component-split` name one argument by
one number; an argument that bears on neither document's threads alone appears in both. Each
quotes its key words verbatim. An argument's Threads line is the extraction's reading of the threads
named when it was made; a thread's Arguments line, written at assembly, lists every argument that
moved it, so the two lists need not match.

### A new register kind or a new reference form touches both, so almost every expected change would cross the (cut) `##arg-16`

R1, the session. Threads: [model apart from checks] #model-crate.

"A new register kind or a new reference form touches both, so almost every expected change would cross the boundary. That is the third signal against a split." (retracted in #arg-45)

### The premise of the one-crate decision, that a separate binary crate saves its users no dependency, fails once the commands span crates `##arg-20`

R1, the session. Threads: #cli-crate (reversal).

"`design@core@single-crate` argued that a separate binary crate "would save its users no dependency". That premise fails once the command set spans crates."

### It applies "where a crate would force a cycle `##arg-22`

R1, the session. Threads: [hard point 1] #cycle-from-a-reversible-decision.

"It applies "where a crate would force a cycle". Here the cycle comes only from a recorded decision (binary in core). The rules don't say whether to move the binary or use a module directory."

### The "two concrete changes" test passes for nearly every subcommand `##arg-23`

R1, the session. Threads: [hard point 2] #commands-sharing-a-purpose.

"The "two concrete changes" test passes for nearly every subcommand. The only brake is "many new dependencies", which the skill gives no number for."

### Usable alone for others `##arg-24`

R1, the session. Threads: [hard point 3] #usable-alone-one-direction.

""Usable alone for others". This is the publishing-cost weight, and it barely bears on `git` or `create`. In practice it didn't change any cut."

### The shape can't serve a crate whose own unit tests use the fixtures, which is exactly the case here `##arg-25`

R1, the session. Threads: [hard point 4] #test-component-public-api.

"The shape can't serve a crate whose own unit tests use the fixtures, which is exactly the case here."

### And your argument that they basically always impact each other `##arg-28`

R2, the owner. Threads: #model-crate.

"And your argument that they basically always impact each other. However, this would change if we start adding more cli tools to explore or postprocess the scanned model."

### Leads are a command to perform searches in existing entries by list of keywords and report closest matches (cut) `##arg-30`

R2, the owner. Threads: #query-crate, #model-crate.

"Leads are a command to perform searches in existing entries by list of keywords and report closest matches (more efficient than grepping the project, a true search algorithm), mechanical worklists for the audit methodology" / "This is bound to increase, because our goals say that mechanical tasks agent perform due to the workflow have to be provided by klarch"

### I think it is better to have the binary as a "top level" crate, such as the cli crate you proposed, which (cut) `##arg-32`

R2, the owner. Threads: #cli-crate, #cycle-from-a-reversible-decision.

"I think it is better to have the binary as a "top level" crate, such as the cli crate you proposed, which pulls the logic from the others and implements the interface, potentially pulling rendering logic from another crate if that applies (such as a project with a gui)."

### The difficulty here mostly came from our own recorded decision, which is quite old, made for reasons that (cut) `##arg-33`

R2, the owner. Threads: #cycle-from-a-reversible-decision, #cli-crate.

"the difficulty here mostly came from our own recorded decision, which is quite old, made for reasons that no longer apply identically, and can be reversed."

### Rewrite is better shaped as one crate with modules rather than one per functionality, because the (cut) `##arg-34`

R2, the owner. Threads: #commands-sharing-a-purpose, #rewrite-crate.

"rewrite is better shaped as one crate with modules rather than one per functionality, because the subcommands it provides share most of the design, ideas and have one centralized purpose (rewrite history), and only differ on subject and application."

### Grouping the parsing logic, and any meaningfull verification that can be applied at parsing, is more (cut) `##arg-35`

R2, the owner. Threads: #command-parsing-home, #cli-crate.

"grouping the parsing logic, and any meaningfull verification that can be applied at parsing, is more convenient, while the application logic should be mostly pulled from other crates (or assembled from high level logic that is provided by other crates)"

### This weight should be considered minor `##arg-36`

R2, the owner. Threads: #usable-alone-one-direction.

"This weight should be considered minor. It is mostly a lead, and I think it should also be single-direction. It can push in the direction of splitting if the crate would be useable alone. It should not push in the other direction if the crate would not be."

### In practice, the condition on unit tests using fixtures you mention would always apply, no ? `##arg-37`

R2, the owner. Threads: #test-component-public-api, #test-through-public-api.

"in practice, the condition on unit tests using fixtures you mention would always apply, no ? My point on that is that there is value in moving integrated testing with mocks or similar out of unit tests, and sometimes out of the crate themselves, so that they focus on testing the actual public API, and not internals"

### (thaum had a different argument for the split which was a dev dependency cycle, but that does not apply (cut) `##arg-38`

R2, the owner. Threads: #testing-crate.

"(thaum had a different argument for the split which was a dev dependency cycle, but that does not apply here I think)"

### I believe that here, unit tests using mock projects is already quite a design smell on the testing (cut) `##arg-39`

R2, the owner. Threads: #test-through-public-api.

"I believe that here, unit tests using mock projects is already quite a design smell on the testing methodology. Though it can be argued that it would be to hard and inefficient to build scanned models without mock projects."

### Our mocking tools need to be published, because we support writing extensions, which needs the mocking (cut) `##arg-40`

R2, the owner. Threads: #testing-crate.

"our mocking tools need to be published, because we support writing extensions, which needs the mocking tools publsished. It could be published as its own crate too, though."

### Centralizing mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in (cut) `##arg-41`

R2, the owner. Threads: #testing-crate.

"centralizing mock testing in one crate here might be meaningfull, to avoid duplication of mock projects in multiple crates that need them, or crates reaching into mock projects of others. This problem did not exists with the project as a single core crate, but it will surface under the split."

### My claim that "every change crosses the boundary" was overstated `##arg-45`

R2, the session. Threads: #model-crate.

"My claim that "every change crosses the boundary" was overstated." / "So 44 of the 99 commits that touch either side touch both. More than half stay on one side. Some of the "both" commits are explained by fact 2, since resolution currently sits in `check/`."

"Fact 2" is the second fact the session listed in R2: the resolver and the lints are one function today.

### The skill's module-directory exception should say that a cycle caused by a decision that can be reversed (cut) `##arg-52`

R2, the session. Threads: #cycle-from-a-reversible-decision.

"The skill's module-directory exception should say that a cycle caused by a decision that can be reversed sends you to reverse it, not to use a module directory."

### Its "two concrete changes" test, applied to one subcommand at a time, passes every time `##arg-53`

R2, the session. Threads: #commands-sharing-a-purpose, #command-parsing-home.

"Its "two concrete changes" test, applied to one subcommand at a time, passes every time. The skill should say that commands sharing one purpose and one design are one Component, with one module each. A top-level binary crate holds the parsing, the checks that can be done at parse time, and the interface, and pulls its logic from below."

### The table now says "weigh it against how usable each crate is alone", which reads as working both ways `##arg-54`

R2, the session. Threads: #usable-alone-one-direction.

"The table now says "weigh it against how usable each crate is alone", which reads as working both ways. It should say: being usable alone argues for a split, and not being usable alone argues nothing."

### 'mock_projects.rs' is a '[cfg(test)]' module that tests internals `##arg-55`

R2, the session. Threads: #test-component-public-api, #testing-crate.

"`mock_projects.rs` is a `#[cfg(test)]` module that tests internals" / "`path@core@tests/binary.rs` (7,026 lines) already tests only through the binary, so after the split it goes naturally to `cli`'s tests." / "So the shared test Component works here if one rule holds: no crate's unit tests use a mock project."

### The cost: the mock tests that now read internals must be rewritten against the public API `##arg-57`

R2, the session. Threads: #testing-crate, #test-through-public-api.

"The cost: the mock tests that now read internals must be rewritten against the public API. That either shows the API exposes enough, or forces an addition to it."

### It should state the rule above as how to get there, and give the published case: a crate that supports (cut) `##arg-58`

R2, the session. Threads: #test-component-public-api.

"It should state the rule above as how to get there, and give the published case: a crate that supports extensions publishes its test tools."

### The skill weighs the split between the model and its consumers by today's consumers, when the goals (cut) `##arg-59`

R2, the session. Threads: #expected-changes-from-goals.

"The skill weighs the split between the model and its consumers by today's consumers, when the goals already announce new ones. A signal like "every change crosses the boundary" should be read against the changes the goals predict, not only against past commits."

### The owner agrees with hard points 1, 3 and 4 `##arg-60`

R3, the owner. Threads: #cycle-from-a-reversible-decision, #usable-alone-one-direction, #test-component-public-api.

ruling, no argument: "1. Agreed" / "3. agreed" / "4. agreed."

### Try to challenge my statement about "A top-level binary crate holds the parsing, the checks that can be (cut) `##arg-61`

R3, the owner. Threads: #command-parsing-home.

"try to challenge my statement about "A top-level binary crate holds the parsing, the checks that can be done at parse time, and the interface, and pulls its logic from below". Does it fit the shape you proposed, were each crate implements its own command parsing for what it provides, and the top level cli flattens it. I think this is quite a good shape too."

### I think that the cost of rewriting the mock tests to use the public API is a good thing `##arg-62`

R3, the owner. Threads: #test-through-public-api.

"I think that the cost of rewriting the mock tests to use the public API is a good thing. If the API does not exposes enough, we will see it clearly. This will drive us to improve this public API, and thus also benefit consumenrs who write extensions."

### It shows if some code is unreachable or has unobervable effects from public API (that cannot be unit (cut) `##arg-63`

R3, the owner. Threads: #test-through-public-api.

"it shows if some code is unreachable or has unobervable effects from public API (that cannot be unit tested internally either), and this is probably a smell of bad design or dead code. We will need to come up with a proper principle/guideline around this."

### I agree render is out of it `##arg-65`

R3, the owner. Threads: #render-crate.

"I agree render is out of it. It has a different purposes, and takes decisions about a different subject"

### (this is probably a good way to phrase it in the skill, a crate/component must take decisions about (cut) `##arg-66`

R3, the owner. Threads: #one-main-subject-per-component.

"(this is probably a good way to phrase it in the skill, a crate/component must take decisions about approximately one main subject, maybe with smaller side subjects allowed)"

### Two goals of the discussion: plannning the component split, and converging on a better design for the (cut) `##arg-67`

R3, the owner. Threads: (scope of the discussion).

"Two goals of the discussion: plannning the component split, and converging on a better design for the component creation skill. As well as the smaller development guideline item I mentionned"

### The binary that offers 'gates' and 'mutate' is each project's own xtask, written from a snippet `##arg-71`

R4, the session. Threads: #command-parsing-home.

"The binary that offers `gates` and `mutate` is each project's own xtask, written from a snippet. No shared interface crate sits above it. Under P1, every project's xtask would declare the flags again, and the copies would drift. So P1 cannot apply to `gates` and `mutate`."

### Two kinds of binary offer them: 'klarch' itself, and each project's extension binary `##arg-72`

R4, the session. Threads: #command-parsing-home.

"Two kinds of binary offer them: `klarch` itself, and each project's extension binary" / "Under P1 both depend on `cli`, so each command still has one spelling."

### A frontend that isn't a CLI gets a usable API `##arg-73`

R4, the session. Threads: #command-parsing-home.

"A frontend that isn't a CLI gets a usable API. Your GUI case, or `render` driven from another tool, wants `render::build(&model, &RenderOptions)`, not a clap struct. That typed API is also what #test-through-public-api would test against."

### Decisions about the whole interface get one home `##arg-74`

R4, the session. Threads: #command-parsing-home.

"Decisions about the whole interface get one home. `design@core@exit-code-ladder`, flags shared across commands (`--staged`, `--out`), and output conventions apply to every command. Under P2 they are restated in each domain crate."

### Clap leaves the domain crates' interface `##arg-75`

R4, the session. Threads: #command-parsing-home.

"clap leaves the domain crates' interface. `design@core@arguments-parse-through-clap` says "clap is part of the core's public interface". Under P1 only `cli` has that cost."

### Argument parsing lives in the lowest crate that every binary offering the command depends on `##arg-78`

R4, the session. Threads: #command-parsing-home.

"argument parsing lives in the lowest crate that every binary offering the command depends on. For klarch's commands that is `cli`, which is your shape. For `gates` and `mutate` it is the crate itself, since no shared crate exists above them."

### Your guideline agrees with the first sentence: nothing is made public for a test `##arg-79`

R4, the session. Threads: #test-through-public-api.

"Your guideline agrees with the first sentence: nothing is made public for a test. It goes against the second: the guideline says a test that needs a private item is first a sign that the API lacks something, and a unit-test module is the last resort. The record I've read gives no reason to prefer unit-test modules."

### The testing crate moves the 36 mock-based tests `##arg-83`

R4, the session. Threads: #test-through-public-api, #testing-crate.

finding 2: "#testing-crate moves the 36 mock-based tests. That doesn't bring back the rejected entry, which was about moving all unit tests." / "The guideline must be an order of preference" / "As a rule to move every test, it is the rejected design."

### 5 of the 36 build 'Inputs' by hand `##arg-84`

R4, the session. Threads: #testing-crate, #test-through-public-api.

"5 of the 36 build `Inputs` by hand." / "Through the public API, each needs a mock project whose tree really has that state. Making `Inputs` constructible would bring back `rejected@core@inputs-struct-literal`. Default: rewrite the 5 over mock projects."

Made precise at the plan's reviews: 5 struct literals of `Inputs`, two of them in the shared helpers `findings_with`, which 17 tests reach, and `phase_two`, which 1 test reaches; two of the five inject a state, an empty map of committed files.

### The entry binds the "unreachable code is a sign of dead code" half `##arg-85`

R4, the session. Threads: #test-through-public-api.

"`design@agent-skills@local-intent-binds` binds the "unreachable code is a sign of dead code" half" / "The guideline says the code is suspect, not that it may be deleted."

### The entry loses the sentence "A test that needs a private item is a unit-test module inside the crate `##arg-86`

R4, the session. Threads: #test-through-public-api.

"`design@core@api-facade` loses the sentence "A test that needs a private item is a unit-test module inside the crate"" / "It is rewritten to the preference order."

### The entry defines a subject as "the heads a piece of work reads together, one part of the Component", and (cut) `##arg-92`

R4, the session. Threads: #one-main-subject-per-component.

finding 6: "`design@agent-skills@subject-split-standard` defines a subject as "the heads a piece of work reads together, one part of the Component", and a Component holds several. "One main subject per Component" would collide with that."

### This session is the real use the skill revisions rest on `##arg-99`

R4, the session. Threads: skill-revision threads, #test-through-public-api (Criteria).

"`goal@knowledge-architect@the-workflow-improves-through-real-use` ... this session is the real use, and #usable-alone-one-direction is a wording that allows two readings, provable by reading" / "`goal@knowledge-architect@the-workflow-leads-to-better-code`: a guideline, not a rule"

### Command-parsing-home I think your synthesis is right `##arg-100`

R5, the owner. Threads: #command-parsing-home.

ruling, no argument: "command-parsing-home I think your synthesis is right."

### What I meant is an integrated tests, that mocks real world application `##arg-101`

R5, the owner. Threads: #test-through-public-api.

"What I meant is an integrated tests, that mocks real world application. Such a test uses the public API just like a consumer would. If it cannot examine the returned objects fromthe public API in sufficient detail, this means that things are being hidden from public consumers, which is a problem."

### If the test is trying to reach into an internal that would have no observable effect to a public consumer, (cut) `##arg-102`

R5, the owner. Threads: #test-through-public-api.

"If the test is trying to reach into an internal that would have no observable effect to a public consumer, then the test is in the wrong place and should not be an integrated test using mock proejcts."

### One-main-subject-per-component: agreed on the domain word `##arg-105`

R5, the owner. Threads: #one-main-subject-per-component.

ruling, no argument: "one-main-subject-per-component: agreed on the domain word."

### Your clarification of the thread on testing through the public API settles the conflict with the entry `##arg-107`

R5, the session. Threads: #test-through-public-api.

"Your clarification of #test-through-public-api settles the conflict with `rejected@core@unit-tests-moved-under-tests`. The rule applies to integrated tests over mock projects, not to unit tests." (the restated guideline is the shape of #test-through-public-api in `spec@plans@component-rules-revision`)

### This doesn't touch the 437 unit tests `##arg-108`

R5, the session. Threads: #test-through-public-api, #testing-crate.

"This doesn't touch the 437 unit tests. The 5 mock tests that build `Inputs` by hand each go one of two ways: the state they inject becomes a real mock tree, such as one with no committed file, or the claim becomes a unit test over an in-memory model, which `design@core@model-then-checks` already provides for."

Made precise at the plan's review: see the note of #arg-84.

### When an integrated test is blocked by a missing observation, the entry fires, and its response already (cut) `##arg-109`

R5, the session. Threads: #test-through-public-api.

"When an integrated test is blocked by a missing observation, `tripwire@core@private-item-needed` fires, and its response already routes the fix."

### You didn't address the last three findings directly `##arg-110`

R5, the session. Threads: #model-crate, #testing-crate, #test-component-public-api.

"You didn't address the last three findings directly. Your next word against this table settles them."

### In a target project, sessions read "one purpose" or "one domain" broadly and merge functions that change (cut) `##arg-116`

R5, the session. Threads: #commands-sharing-a-purpose, #one-main-subject-per-component (C6).

"in a target project, sessions read "one purpose" or "one domain" broadly and merge functions that change apart into one crate" / "already watches this; no tripwire"

### Sessions widen a public API to serve an integrated test, for an effect no consumer observes, instead of (cut) `##arg-117`

R5, the session. Threads: #test-through-public-api (C7).

"sessions widen a public API to serve an integrated test, for an effect no consumer observes, instead of moving the claim to a unit test" / "the code-shape reviewer hosts the fragment and reports it"

### A spec for the skill revisions `##arg-120`

R5, the session. Threads: (hand-off).

"A spec for the skill revisions" / "the fragment on when to split, which restates the split signals and must change with the skill" / "Text only, independent of the code." / "A milestone for the split, in slices" / "`bundle`, `render` and `rewrite` are each created by the work that builds them."

### Your wording uses mock project, but not all projects are meant to apply to other projects, and not all use (cut) `##arg-123`

R6, the owner. Threads: #test-through-public-api.

"your wording uses mock project, but not all projects are meant to apply to other projects, and not all use mocks."

### The important point is that the integrated test drives the project through public API, as a consumer would `##arg-124`

R6, the owner. Threads: #test-through-public-api.

"The important point is that the integrated test drives the project through public API, as a consumer would. It is often associated with a mocking environment, but not necessarily."

### Agreed on the table `##arg-125`

R6, the owner. Threads: every thread, (planning shape).

ruling, no argument: "Agreed on the table. Agreed on the planning shape too, spec for skill revisions, and one milestone for the split."

## New names, in one place

```text
domain             the component-creation skill, `skill@knowledge-architect-component-creation@component-criteria`: what a Component's decisions are about
integrated test    the code-shape fragment: a test that drives the project through its public API, as a consumer would
top-level binary crate   the component-creation skill, `skill@knowledge-architect-component-creation@component-shapes`: the crate that holds a binary's interface
```

## Decided design

### A dependency cycle that comes only from a recorded decision that can be reversed sends the session to reverse that decision, not to the module-directory exception

**Shape.** The module-directory exception gains one sentence: where the cycle that a crate of its own
would create comes only from a recorded decision, such as where a binary lives, the decision is
reopened under the design skill first, and the exception applies only to a cycle Cargo's rules force
whatever the decisions. The table of `skill@knowledge-architect-component-creation@component-crates` gains no row.

**Argument.** The exception is for the two facts of Cargo the grounding file reads: a published crate
cannot depend on a path crate, and the resolver refuses a cycle. A cycle made by a decision is no
such fact: here it came from `design@core@single-crate`, whose premise, that a separate binary crate
saves its users no dependency, failed once the commands spanned several crates (#arg-20, #arg-33).

**Nearest rival and the fact that defeated it.** The exception left as written: it let the session
reach for a module directory where reversing the decision gave every crate its own boundary, and the
owner judged the text unclear on the case (#arg-22).

### Commands or functions that share one purpose and one design are one Component, one module each, and a top-level binary crate holds the interface and pulls its logic from below

**Shape.** `skill@knowledge-architect-component-creation@component-criteria` gains, after the test of two changes: commands, subcommands or
functions that share one purpose and one design are one Component, with one module each; the test
of two changes is applied to the Component, not to each command. `skill@knowledge-architect-component-creation@component-shapes` gains a bullet,
**a top-level binary crate**: where several Components contribute commands to one binary, the binary
and its interface sit in a crate above them, which parses the arguments, applies the checks that can
be made while parsing, and calls the logic the Components expose as a typed API; the parsing of a
command lives in the lowest crate that every binary offering the command depends on, so a library
whose commands are flattened into each adopting project's own binary keeps its own parsing.

**Argument.** The test of two changes passes for every command taken alone (#arg-23); commands that
share their design change together where their design changes, and apart only where their subject
differs (#arg-34). Parsing grouped in one crate gives the decisions about the whole interface one
home, keeps the parser's library out of the domain crates' interface, and gives a frontend that is no
command line a typed API (#arg-73 to #arg-75); a library flattened into binaries no shared crate sits
above must keep its parsing, or each binary declares it again (#arg-71).

**Nearest rival and the fact that defeated it.** For the first decision, the test of two changes
applied to each command: it passes for every command and so splits a design that changes as one
(#arg-23). For the second, each Component parsing its own commands and the binary flattening them,
P2: it serves the libraries flattened into adopting projects' binaries, which the shape keeps, and
for the rest it restates the interface's decisions in each domain crate (#arg-74).

### Being usable alone argues for a split, and not being usable alone argues nothing

**Shape.** The table row for a publishing project ends: "a crate that others could use alone is an
argument for the split; a crate no one would use alone is no argument against it", and calls the
weight minor, a lead: the owner's words, "This weight should be considered minor. It is mostly a
lead" (#arg-36). The owner chose a minor weight over a weight equal to the default criteria, which
no goal ranks.

**Argument.** The owner's (#arg-36): it is a lead, and only in one direction. Read both ways, the
row would keep a Component inside another on the ground that no one outside would use it, which no
criterion of `skill@knowledge-architect-component-creation@component-criteria` asks (#arg-54).

**Nearest rival and the fact that defeated it.** The row as written, weighed both ways: its words
allow two acts, a defect provable by reading.

### The split signals are read against the changes the goals predict, as well as the changes made so far

**Shape.** The third signal reads: "every change you expect to make crosses the new boundary,
counting the changes the goals already predict, not only those of the history". The fragment on when
to split's sentence of the signals is rewritten to match, in step 1.

**Argument.** In this repository, the history alone showed 44 of 99 commits crossing the line between
the model and the checks, and the goals predicted new consumers of the model needing no check
(#arg-28, #arg-30, #arg-45). A signal read against the past alone kept the model inside the checker
(#arg-16, #arg-59).

**Nearest rival and the fact that defeated it.** The signal as written: "you expect" admits the
goals' predictions, but the session that applied it counted commits only, so the wording led to the
narrower reading in real use.

### A Component takes decisions about one main domain, with small side domains allowed

**Shape.** The fourth criterion of `skill@knowledge-architect-component-creation@component-criteria` reads: "takes decisions of its own, about one
main domain, with small side domains allowed, so it has a design of its own". The word is defined
there: a domain is what a Component's decisions are about, such as the layout of a rendered site or
the command line; a Component's design home may hold several subjects of one domain.

**Argument.** The owner's (#arg-65, #arg-66): render was kept out of the query crate because it
takes decisions about a different matter. The word "subject" is taken: a subject is a part of a
Component's design home, per `design@agent-skills@subject-split-standard` (#arg-92).

**Nearest rival and the fact that defeated it.** "One main subject", the owner's first wording: it
collides with the defined term.

### The shared test Component holds the integrated tests that drive the Components through their public API, and a crate that supports extensions publishes its test tools

**Shape.** The shared test Component's bullet states: it holds the integrated tests that cross the
Components it depends on, with their fixtures; the tested Components' unit tests use none of those
fixtures, so no Component dev-depends on it from below. The double-build warning stays. A second
decision, stated by the skill alone and so carried by a `%%` line: a Component that supports
extensions publishes the tools an extension's tests need, its harness; whether the harness is a
module of that Component or a crate of its own is the project's to choose, and the fixtures of the
shared test Component stay outside every package.

**Argument.** The owner's (#arg-37, #arg-41): integrated tests out of unit tests and out of the
crates make them test the public API, and one home for the fixtures keeps crates from duplicating
them or reaching into another's. The condition the bullet stated, crates "without depending on each
other", nearly always fails for a crate whose own unit tests use the fixtures (#arg-25, #arg-37); the
rule that no unit test uses them is how the shape becomes reachable (#arg-55).

**Nearest rival and the fact that defeated it.** The bullet as written: its condition excluded the
case met here, a crate whose unit tests use the fixtures.

### An integrated test drives the project through its public API, as a consumer would

**Shape.** The code-shape fragment gains a property, after "The logic tested without the side
effects": **an integrated test drives the project through its public API, as a consumer would**, in
a mock environment or not. Where it cannot observe what it needs in what that API returns, the API
hides it from consumers too, and the API is what is fixed; where what it would observe has no effect
a consumer could see, the claim is a unit test's, and an integrated test is the wrong place for it.
Code no integrated test can reach and no consumer can observe is suspect, and its comment and the
commit that added it are read before anything is removed. The test-kind table gains a row:

| test kind | what a mutation tells | the economy question |
| --- | --- | --- |
| integrated, through the public API | a mutation of the logic it drives, which it owes as a test of logic owes where its claim is one of logic | does it observe the claim through what the API returns, or reach for an internal |

A test's kind is still decided by what it exercises; "integrated" names how it drives the code, so
it exempts no test of logic from its mutation.

**Argument.** The owner's (#arg-62, #arg-63, #arg-101, #arg-102, #arg-124): the test drives the API's
improvement, shows where it lacks, benefits its consumers, and shows code with no observable effect.
The scope is the integrated test (#arg-107): a guideline that moved every unit test to the public API
is `rejected@core@unit-tests-moved-under-tests` (#arg-83). The reading before removal is
`design@agent-skills@local-intent-binds` (#arg-85).

**Nearest rival and the fact that defeated it.** The session's reading of the owner's first
phrasing, R3, "testing through public API when possible", as applying to every test: it is the
rejected design, whose measurement shows most unit tests run over private functions; the owner then
scoped the phrasing to the integrated test, R5. The wording "over a mock project" (R5) lost to the
owner's R6: not every project uses mocks, and not every project serves others.

## Mapping tables

Empty: the work needs no total function over a domain.

## Losing alternatives

- The module-directory exception left as written, lost to #cycle-from-a-reversible-decision: the
  owner judged it unclear on a cycle a decision makes.
- Each Component parsing its own commands and the binary flattening them, lost to
  #commands-sharing-a-purpose for every command whose binaries share a crate above it; it stays the
  shape for a library flattened into each adopting project's binary.
- The usable-alone weight read both ways, lost to #usable-alone-one-direction.
- "One main subject", lost to #one-main-subject-per-component: the word collides with
  `design@agent-skills@subject-split-standard`.
- The session's reading of the guideline as testing every unit through the public API, lost to
  #test-through-public-api on `rejected@core@unit-tests-moved-under-tests`; the owner's own phrasing
  was scoped to the integrated test, R5.
- "An integrated test over a mock project", lost to #test-through-public-api on the owner's R6.

## Readings

Empty: the work reads no external specification.

## Premortem

| cause | thread it stresses | verdict |
| --- | --- | --- |
| C6 of the discussion: in a target project, sessions read "one purpose" or "one domain" broadly and merge functions that change apart into one crate | #commands-sharing-a-purpose, #one-main-subject-per-component | named as a cause; `design@agent-skills@premortem-as-watch-points` already asks whether a splitting rule was followed against the work's interest, so no tripwire |
| C7 of the discussion: sessions widen a public API to serve an integrated test, for an effect no consumer observes, instead of moving the claim to a unit test | #test-through-public-api | converted into a clause of the design: the second branch of the guideline; the code-shape reviewer, which hosts the fragment, reports it |
| a test of logic is called integrated to skip its recorded mutation | #test-through-public-api | converted into a clause of the design: the new row says an integrated test of a claim of logic owes its mutation; `tripwire@agent-skills@logic-tests-relabelled` already watches the case, and the harvest judges whether its firing clause, which names golden, mocked or external, names the new row |
| the fragment on when to split and the skill's signals drift, one revised and not the other | #expected-changes-from-goals | a claim of step 1, checked by reading both installed copies |

## Acceptance criteria

None: every check of this work's decisions is a claim of a step, which the review judges, or a cause
the retrospective's standing question already watches.

## Implementation sequence

1. **The signals and the criteria.** `skill@knowledge-architect-component-creation@component-criteria` gains the sentence of
   #commands-sharing-a-purpose and the domain criterion; `skill@knowledge-architect-component-creation@split-signals` the third signal's wording;
   the fragment on when to split its sentence. Claims: the installed copies of the skill, the design
   skill and the development skill state the same three signals; test: each installed copy, read
   after `cargo klarch install-agent-skills`, holds the new clause of the third signal,
   `grep -c 'the changes the goals already predict' .claude/skills/knowledge-architect-component-creation/SKILL.md .claude/skills/knowledge-architect-design/SKILL.md .claude/skills/knowledge-architect-development/SKILL.md`
   counting one in each file; shown to fail on the copies before the install, which count none. "Domain" is defined in the sentence that first uses it;
   test: the first sentence of the installed skill holding the word defines it, read. Fails alone on:
   a host of the fragment that states the old signal.
2. **The crates.** `skill@knowledge-architect-component-creation@component-crates`: the usable-alone row and the cycle sentence. Claims: the
   module-directory exception names the case of a cycle a decision makes, and the row weighs in one
   direction; test: the installed section, read, holds both sentences, and the old "weigh it
   against how usable each crate is alone" is gone, `grep -n 'how usable' .claude/skills/knowledge-architect-component-creation/SKILL.md`;
   shown to fail on the copy before the install. Fails alone on: the row still read both ways.
3. **The shapes.** `skill@knowledge-architect-component-creation@component-shapes`: the shared test Component's bullet and the top-level binary
   crate's bullet. The bullet of the shared test Component defines "integrated test" in the sentence
   that first uses it, in the words of the code-shape fragment. Claims: the bullets name no crate of
   this repository, and the condition "without depending on each other" is gone; test: the
   installed section, read, names no package of this repository, and
   `grep -n 'without depending on each other' .claude/skills/knowledge-architect-component-creation/SKILL.md`
   lists nothing; shown to fail on the copy before the install, which lists the condition. Fails
   alone on: the condition left.
4. **The code-shape fragment.** The integrated-test property and its table row. Claim: both hosts'
   installed copies hold it; test: `grep -ln 'integrated, through the public API' .claude/skills/knowledge-architect-development/SKILL.md .claude/agents/knowledge-architect-code-shape-reviewer.md`
   lists both files; shown to fail before the install. Fails alone on: the reviewer's copy without
   the property.
5. **The harvest.**

## Order rationale

- Step 1 before step 2: the criteria the crates section weighs are stated first.
- Step 2 before step 3: the shapes section's top-level binary crate rests on the cycle sentence.
- Step 3 before step 4: the fragment's integrated test is the test the shared test Component holds.
- Step 4 before step 5: the harvest records what the four steps wrote.

## Defaults awaiting the owner

None.

## Harvest

| what | where | when |
| --- | --- | --- |
| #cycle-from-a-reversible-decision, #usable-alone-one-direction | `design@agent-skills@component-crate-mapping`, rewritten in place | at the landing |
| #expected-changes-from-goals | `design@agent-skills@component-split-signals`, rewritten | at the landing |
| #test-component-public-api | `design@agent-skills@shared-test-component`, rewritten | at the landing |
| #test-through-public-api | a new head of `register@agent-skills@design`, in its subdocument on developing, for the guideline, `#test-through-public-api → integrated-test-through-public-api`, since the thread's name read alone states the rival: it answers how an integrated test reaches the code, and loses to its own rival, every test through the public API; `design@agent-skills@default-code-shape` cites it for the property it lists, and `design@agent-skills@test-economy` is rewritten as the table above says | at the landing |
| #commands-sharing-a-purpose, its first decision; #one-main-subject-per-component; the publication of the test tools, the second decision of #test-component-public-api, which the `testing` head the milestone harvests does not restate, since that head records this repository's choice of a crate, not the rule | `%%` lines at their instructions, no head | at the landing |
| #commands-sharing-a-purpose, its second decision: a top-level binary crate holds a binary's interface, and a command's arguments are parsed in the lowest crate every binary offering it depends on | two `%%` lines at its two instructions, since the two parts lose to different rivals, the binary inside a domain crate and P2, and no head while the component-creation skill alone states them, per entry test 2 of `primer@design-heads`; the slice of `milestone@plans@component-split` whose `cli` head states them is their second text, and writes the two heads of `register@agent-skills@design` that its head cites; its rivals are each judged as a losing alternative | at the landing |
| #cycle-from-a-reversible-decision | the title of `design@agent-skills@component-crate-mapping` narrowed: the module-directory exception holds where a crate of its own would force a publication or a cycle that no reversible decision causes | at the landing |
| the losing alternatives | judged by `skill@knowledge-architect-decision-recording@losing-alternatives`; those that fail stay in this spec's history | at the landing |
| the premortem's causes | judged by `skill@knowledge-architect-decision-recording@ruled-tripwires` | at the landing |
| a CHANGELOG.md entry under Workflow, if a session watching would observe a new act | `Next release` | at the landing |
| the net count of instructions the steps add and remove | the landing commit's message, and the owner told | at the landing |

## Later consequences

- `milestone@plans@component-split` applies the revised rules to this repository; its testing slice
  rewrites `design@core@api-facade` to the guideline's branches.
