---
name: knowledge-architect-planning
description: MUST use when the shapes of a design discussion that expects its full path settle, before its record search, to write the draft the search reads, and when a design discussion has converged and its work needs a spec or a milestone, in the same session as the convergence; whenever a slice of a milestone is about to be implemented or lands; when the work of a spec starts or lands; and before editing the roadmap. Covers the draft the design grounding's search reads, the plan document vocabulary (spec, milestone, plans directory, roadmap), choosing between a spec and a milestone, the roadmap, the directory layout, what each file of a plan document's directory holds and the fixed sections, plan items and their citations, assembly from the discussion's transcript and the committed transcript with its removal step, cutting the work into steps and slices, acceptance criteria, the design audit, the reviews of a plan document, the harvest, the move of a landed slice's spec under its milestone's harvested/, and the deletion that ends a plan document's life.
---

# Planning

%% This skill is one activity.
Scope: turning the converged design of a discussion into a plan document that a later session
implements, then carrying that document through each landing until it leaves the repository. It
begins in the session of a discussion on its full path, once its shapes settle, with the draft its record search
reads, and it re-enters at each slice's audit and at each landing.

**Not covered here**, each named where it lives:

- **The design discussion** that produces the converged design:
  `skill@knowledge-architect-design`. On its full path, it ends at convergence and the premortem,
  its causes named and its acceptance criteria written, and hands off to this skill; its
  in-change path writes no plan document.
- **Writing the code** of a step, its claims and its tests: `skill@knowledge-architect-development`
  for the project's Rust code, and the project's own development procedure for any other code.
- **Dispatching the reviewers**: `skill@knowledge-architect-review`.
- **Auditing one aspect of the whole project**, which the design audit of a slice or a spec is
  not: `skill@knowledge-architect-project-audit`.
- **Recording** what a landing establishes. The procedure is
  `skill@knowledge-architect-decision-recording` for the decisions and the rejected designs, and
  `skill@knowledge-architect-issue-tracking` for the tripwires and the issues.

## Terms {{slug:planning-terms}}

| word | meaning |
| --- | --- |
| **plan document** | a spec, a milestone document, or the spec of a slice, each a directory (`skill@knowledge-architect-planning@plans-layout`). Temporary: a spec and a milestone leave when their work lands; a slice's spec is harvested when its slice lands, and leaves with its milestone |
| **plans directory** | docs/plans/ at the project's root, the one directory where a project keeps its plan documents. The checker fixes the path (`skill@knowledge-architect-planning@plans-layout`) |
| **spec** | the plan document of work done in one branch and one PR: the converged design and a concise implementation sequence. Cited `spec@plans@<id>` |
| **step** | one item of an implementation sequence, in a spec or in a slice |
| **slice** | a part of a milestone that is one branch and one PR, with its own spec, a directory inside the milestone's, cited `spec@<milestone>@<slice>` |
| **milestone** | work across several PRs, with design sessions between them. Its plan documents are one directory: the **milestone document**, its `README.md` with the files beside it, and one spec directory per slice. Cited `milestone@plans@<id>` |
| **harvested** | the state of a slice's spec once its slice has landed: its directory moved under its milestone's `harvested/`, frozen there, until the milestone's directory leaves (`skill@knowledge-architect-planning@plan-document-leaves`) |
| **frozen** | no commit modifies the file |
| **item** | a thread, an argument, a criterion or an acceptance criterion, defined in a plan document as a level-three heading ending with its slug, under the section of its kind (`skill@knowledge-architect-planning@spec-contents`) |
| **roadmap** | docs/roadmap.md at the project's root, optional: the order in which the owner wants known work done (`skill@knowledge-architect-planning@spec-or-milestone`) |
| **transcript** | the harness's log of a session, on disk. It keeps the records from before a compaction. A plan document's committed copy of it is its `transcript.txt` |
| **assembly** | writing a plan document from the discussion's transcript (`skill@knowledge-architect-planning@spec-contents`) |
| **converged** | convergence holds, as loop step 6 of `skill@knowledge-architect-design@discussion-loop` defines it |
| **thread** | one open question or proposal of a design discussion, the unit that carries a state, as `skill@knowledge-architect-design@discussion-overview` defines it; a thread closed `approved` or `argued` is a decision |
| **criterion** | what proposals were judged against: **binding** rules a proposal out, **weighed** makes failing it a cost the owner rules on |
| **acceptance criterion** | a check on a decision of the plan document, a thread closed `approved` or `argued`, that only the work's built code can apply, and whose firing would show the decision wrong |
| **material** | a finding made after a thread closed that defeats a reason the closure gave, a premise it rested on, or a criterion it claimed to meet, as `skill@knowledge-architect-design@decision-authority` defines it, which says how one is answered |
| **audit** | the reading of a slice's entry, or of a spec, against the tree and the design homes before its work is implemented |
| **harvest** | the recording of what a landing established into the project's durable homes |

**The owner** is the person whose word closes a fork, and every question `primer@question-authority` reserves to the owner.
The word "plan" alone never names a document: say spec, milestone document, or plan document.

## When this skill starts {{slug:when-planning-starts}}

%% The draft: `design@agent-skills@design-hands-off-to-planning`; it is no ledger, per
%% `rejected@agent-skills@ledger-written-during-discussion`.
**On the full path of a design discussion, once its shapes settle and before its record search.**
Write the draft the search reads, as loop step 1 of `skill@knowledge-architect-design@discussion-loop`
asks, in your scratch space, with the sections of a spec that carry the design: what the work is,
what is already decided, the threads as they stand, the decided design as proposed, and the losing
alternatives. It is held to fewer constraints than a plan document, since the checker reads no
scratch file. The search's findings revise it, and a later thread extends it, as that loop step
says. It is never read as the discussion's state, and never the source of the plan document.

**The plan document: in the session where the discussion converged, before that session ends.** The discussion's
ledger, every thread with its state and its argument, lives in that conversation and in the
harness's transcript of it, which the assembly of `skill@knowledge-architect-planning@spec-contents` reads. A plan document written from memory in a
later session is written from a summary, and a summary loses exactly the losing arguments and the
conditions of each closure.

## A spec or a milestone {{slug:spec-or-milestone}}

| the work | its plan document |
| --- | --- |
| one branch, one PR | a spec |
| several PRs, with design sessions between them | a milestone |

Propose which one, with the reason, and let the owner rule. A spec whose work turns out to need
several PRs becomes a milestone: at that moment the spec is split into the milestone document and
one spec per slice, by the rule of `skill@knowledge-architect-planning@spec-contents`.

**Work that is known but not designed is not a plan document.** It is a `todo` issue, or a
`deferred` one if an event gates it (the default kinds; a project that declares its own kinds uses
the nearest), in the owning anchor's issue register, with its leads in the
entry. The plan document that schedules it closes that issue in the commit that adds the plan
document, so the work is listed in one place at a time. `{{command}} issues --kind todo` lists
what is known and undesigned. There is no record of landed work: history lists what landed.

**The roadmap orders known work, and holds nothing else.** docs/roadmap.md at the project's root
is optional. Each row cites an issue entry, `issue@<anchor>@<id>`, or a whole plan document,
`spec@plans@<id>` or `milestone@plans@<id>`, in the order the owner wants the work done. An
unordered section may follow. **The order is the owner's**: a row is added or moved on the owner's
word. Three edits need no word, because the row keeps pointing at the same work, or that work is
done:

- the commit that adds a plan document closes the issue it schedules, and rewrites that issue's row
  to cite the plan document;
- the commit that deletes a plan document removes its row: the work landed;
- the commit that closes an issue with no plan document scheduling it removes its row: the work is
  done.

A row whose entry left without its edit above dangles, and `{{command}} check` reports it, so the order
cannot go stale unnoticed. An illustration of the shape:

```markdown
# Roadmap

## In order

1. `milestone@plans@<id>`
2. `issue@<anchor>@<id>`

## Unordered

- `issue@<anchor>@<id>`
```

## Layout {{slug:plans-layout}}

```
docs/plans/
├─ README.md               what the directory holds
├─ specs/
│  ├─ README.md
│  ├─ index.md             generated
│  └─ <id>/                a spec
│     ├─ README.md         the document the searches read
│     ├─ deliberation.md   its Threads and Arguments
│     ├─ search/           the generated lines of its searches
│     └─ transcript.txt    the discussion's trimmed transcript
└─ milestones/
   ├─ README.md
   ├─ index.md             generated
   └─ <id>/                a milestone
      ├─ README.md         the milestone document
      ├─ index.md          generated: its slices, live and harvested
      ├─ deliberation.md
      ├─ search/
      ├─ transcript.txt
      ├─ <slice>/          the spec of one live slice, laid out as a spec
      └─ harvested/
         └─ <slice>/       the spec of a landed slice, frozen
```

%% The layout and the anchors: `design@agent-skills@milestone-is-a-directory`; the files beside a
%% README: `design@agent-skills@discussion-record-apart`; the frozen files under `harvested/`:
%% `design@model@a-harvested-slice-is-frozen`; guarded by the review alone:
%% `design@check@a-frozen-file-is-guarded-by-the-review`.
%% A README holds the document because the checker resolves a relative link only there.
**What each file of a plan document's directory holds**, a slice's directory included:

| file | holds | owed |
| --- | --- | --- |
| `README.md` | the document the searches read: every section of `skill@knowledge-architect-planning@spec-contents` but Threads and Arguments | always |
| `deliberation.md` | the sections Threads and Arguments, whose items record the argument of the discussion | where the document has a thread or an argument |
| `search/discussion.txt` | the generated lines of the discussion's record search, as `skill@knowledge-architect-planning@spec-contents` says | where that search ran for this document |
| `search/design-audit.txt` | the fit lines of its design audit, as `skill@knowledge-architect-planning@working-a-slice` says | where that audit ran |
| `transcript.txt` | the trimmed transcript of the discussion, as `skill@knowledge-architect-planning@spec-contents` says | where the document was assembled from a transcript and the owner approved it |

- The checker refuses any other file or directory in a plan document's directory; a milestone's
  directory admits besides its generated `index.md`, its slice directories and `harvested/`. A
  spec written as one file, `docs/plans/specs/<id>.md`, is a finding whose repair names the
  directory.
- The checker reads `README.md` and `deliberation.md`. It never parses a `.txt` file, so a
  reference, a slug or a commit SHA in `transcript.txt` or in `search/` is no citation and raises no
  finding.
- **Every file under a milestone's `harvested/` is frozen, and out of the walk**: the checker reads
  it for the items it defines alone, so an item it defines still resolves from inside its
  milestone, and a live item that reuses one of its ids is reported. **No commit modifies a frozen
  file, or adds one other than by the move that harvests a slice**, which adds each of its files
  unchanged; a commit may delete one. No check reports such a commit: the review before a merge
  does, by its standing-state axis.
- The checker constructs the anchor `plans` at docs/plans/, one anchor per spec directory and one
  per milestone directory. A slice's directory is no anchor: its milestone is the anchor of every
  file under the milestone's directory. A file or a directory directly under docs/plans/ outside
  this layout is a finding, and so is a directory under milestones/ with no `README.md`.
  `{{command}} index` writes every `index.md`, and `{{command}} check --fix` writes them once the
  first three phases of the check find nothing, then runs every check. For a commit of part of the
  working tree, `{{command}} index --staged` writes them into git's index from what is staged, and
  `{{command}} check --staged` checks what the commit will record.
- **A `README.md` reaches the files beside it by relative navigation links**, such as
  `[deliberation](deliberation.md)` and `[the discussion's search](search/discussion.txt)`, and an
  item of `deliberation.md` by its item reference. The milestone document links each live slice's
  spec as a navigation row, `[<slice title>](<slice>/README.md)`. The checker resolves a relative
  link only in a `README.md` or an `index.md`, to a target under its own directory, so a slice's
  `README.md` reaches its milestone's `deliberation.md` through item references alone. No file of a
  plan document's directory is cited by a `path` reference.
- **The plans directory holds plan documents and nothing else**, except its `README.md` files and
  generated indexes, which keep its homes in the tree while no plan document is open. A document with
  another lifetime, such as a record of how far a subject is implemented or a survey that outlives
  its work, has its own home. If none fits, ask the owner before writing it anywhere.
- **A plan document is cited by its kind**, never by its path: `spec@plans@<id>`,
  `milestone@plans@<id>`, `spec@<milestone>@<slice>`. A `path` citation of one is refused. A whole
  plan document may be cited from anywhere but a goals home, and its citations dangle when it leaves
  (`skill@knowledge-architect-planning@plan-document-leaves`). **A harvested slice is cited from
  inside its milestone's directory only**: a citation of it, or of an item it defines, from
  anywhere else is refused, and its repair names the milestone.
- **A plan document defines items, and no design entry** (`skill@knowledge-architect-planning@spec-contents`). A plan's name is the anchor of its
  items, so it is not the name of a Component, of a location or of a reserved anchor, `plans`,
  `elsewhere` or `*`, and one name is not used under both homes.

## What a spec holds {{slug:spec-contents}}

**Written for a session that did not witness the discussion.** That is the standard every section
is held to, and the reviews of `skill@knowledge-architect-planning@plan-reviews` check it.

**The sections of a spec of specs/ and of a milestone document are fixed: level-two headings with
these titles, in this order**, Threads and Arguments in `deliberation.md`, every other in
`README.md`. A slice's `README.md` owes the five sections of its entry instead, and carries of the
sections below only those its slice fills, "What is already decided" where the slice has entries
to seed its audit's search; its threads and arguments are in its own `deliberation.md`.
`{{command}} check` reports a section a document owes as missing or out of order. A section a document owes and has nothing to hold says
so in one line rather than being omitted, so a reader can tell an empty section from a missing one.

| section | holds |
| --- | --- |
| Status and audience | what the document is for; that it leaves when its work lands; that where it and a design home disagree on a point its section "What is already decided" does not list as reversed or rewritten, the design home wins; that every name it uses is defined in it or exists in the code; that where the owner's word is awaited, `skill@knowledge-architect-planning@working-a-slice` says what waits |
| How the work is done | in a milestone document: a pointer to `skill@knowledge-architect-planning@working-a-slice`, with an instruction to read it whole before each slice, and any addition the milestone's work needs. In a spec: one line naming this skill |
| Names | every project shorthand the document uses, expanded to the file, function or command it names |
| What the work is | what exists today at each site the work touches; what is outside the work and why, each exclusion naming the work or the decision that owns it |
| What is already decided | the recorded decisions the design rests on and does not argue again, as references; and each recorded decision the work reverses or rewrites, with every text that `{{command}} show` lists as referencing it (a tripwire, an issue, a restatement in a `CLAUDE.md` or a skill, a README, a comment), and the step, slice or harvest that judges or updates each; and each entry the discussion's search gave a verdict whose outcome changed something, the design, a thread or the work, with that outcome, and a link to `search/discussion.txt` for the other verdicts |
| Criteria | one item per criterion, ``### <criterion> `##<id>` ``: its kind, its source and its satisfaction |
| Threads | in `deliberation.md`: one item per thread, ``### <resolution> `##<id>` ``: who proposed it and in which round, its final state, the arguments that moved it, the section that carries its shape, the durable home that will harvest it, and the owner's ruling that closed it, attributed, with its round, or, for a thread closed `argued`, the argument that closed it |
| Arguments | in `deliberation.md`: one item per argument of the discussion, ``### <argument> `##arg-<n>` ``: its round, who gave it, the threads it bears on, and its key words, the session's verbatim and the owner's attributed |
| New names, in one place | every new name the design uses (a type, a function, a field, an event, a bound, a counter) in one fenced block with the file it goes in; a name that exists in the code is listed as existing |
| Decided design | one subsection per thread closed `approved` or `argued`: the shape and every condition its closure set, the argument, the nearest rival and the fact that defeated it |
| Mapping tables | one table per total function the code will need, over its whole domain: which existing thing becomes which new thing. Empty when the work needs none |
| Losing alternatives | every ruled-out thread, every thread withdrawn with its defeating reason, and every superseded thread under the thread that absorbed it, each with the thread it lost to and the fact that decided it |
| Readings | where the work reads an external specification the project implements: each reading it makes, and where it is recorded. Empty for work that reads none |
| Premortem | each cause, the thread it stresses, and its verdict: survives into a named claim, criterion or guard; converted into a named clause of the design; named as a cause for the harvest to judge as a tripwire; becomes an acceptance criterion; becomes a claim of the step that builds it; or fired and the thread reopened |
| Acceptance criteria | one item per criterion, ``### <criterion> `##<id>` ``, as `skill@knowledge-architect-planning@acceptance-criteria` says |
| Implementation sequence | in a spec, its steps; in a milestone document, its slices, each linked to its spec, whose own steps it holds (`skill@knowledge-architect-planning@cutting-steps-and-slices`). Concise: what each builds and what it fails alone on, and, in a spec, each step's Claims, as a slice's entry states them |
| Order rationale | one sentence per pair of adjacent steps or slices |
| Defaults awaiting the owner | each default a reviewer's finding or the author's judgement produced that `primer@question-authority` gives the owner, labelled `D<n>`, with the thread it bears on, marked as ruled once the owner rules |
| Harvest | what lands where and when. In a spec, its rows; in a milestone, each slice's row in that slice's spec, and the row of the milestone document itself in it |
| Later consequences | what each later piece of work adds or replaces, so a later reader knows what was deliberately left |

**A milestone's design is split across its documents from the start, by lifetime.** The milestone
document and one spec per slice are written together, in the session that converged. A milestone
document that held every slice's design would be read whole at every slice's grounding, and grows
with every slice the milestone has.

- **A slice's spec is frozen when its slice lands, so it holds what only that slice builds**: the slice's
  entry (`skill@knowledge-architect-planning@cutting-steps-and-slices`) and its steps, then, in the sections above, the items of its own threads and the arguments only
  they use, in its `deliberation.md`, its decided design, its mapping tables, its losing alternatives, the acceptance
  criteria it judges and its harvest row. These sections are not ordered against the entry's.
- **The milestone document holds what crosses slices or outlives one**: the status, how the work is
  done, the names, what the work is, the criteria, every item that more than one slice's
  document cites, in the milestone's `deliberation.md` for a thread or an argument, the premortem, the implementation sequence with each slice linked to its spec,
  the order rationale, the defaults awaiting the owner, the harvest row of the document itself and
  the later consequences. A section whose content lives in the slice specs says so in one line.
- **An item cited from the milestone document, or from more than one slice's spec, lives in the
  milestone document.** A slice's spec that held it is frozen once harvested, so the item could no
  longer be amended in place when a later slice's landing changes it.
- A design session held at a slice's audit writes its design into that slice's spec, and what it
  decides for later slices into theirs.

%% The clause and its reason: `design@agent-skills@discussion-record-apart`.
**Every condition of a decided thread is in the decided design of a document the search reads**,
the `README.md` of the spec, of the milestone or of the slice; `deliberation.md` holds only the
argument. A slice's design audit searches the record with the two `README.md` files as its work
(`skill@knowledge-architect-planning@working-a-slice`), so a condition written only in
`deliberation.md` is one that no search checks the record against.

**Content rules.**

- **No untested code snippet is presented as authority.** An implementer forces an authoritative
  snippet into the code at any cost and copies its comments verbatim. A snippet in a plan document
  is labelled as an illustration of a shape. A detailed step-by-step implementation plan, if one is
  ever written for a less capable implementer, covers a bounded amount of work and opens by saying
  that its content rests on assumptions and may be wrong.
- **A statement about existing code is a claim, not a hypothesis.** It is checked against the tree
  before it is written, as `primer@verify-before-relying` asks, in its subsection "Verify a claim
  before writing it". Only the unbuilt half of a plan document is a
  bet.
- **A correction is applied in place**, with no correction log and no issue: the document leaves
  when its work lands, so a log would be history in a file that keeps none. The commit says what
  was corrected.

**Items.** A thread, an argument, a criterion and an acceptance criterion are items: a
level-three heading under the section of its kind, the statement first and the slug last,
``### <statement> `##<id>` ``. The section gives the kind: Threads `thread`, Arguments `argument`,
Criteria `criterion`, Acceptance criteria `acceptance`. Every level-three heading of those four
sections is an item, and a slug anywhere else in a plan document defines nothing.

%% Arguments are never harvested as entries, so a content slug would cost a name for each of dozens
%% of statements, for nothing; the numbers are assigned at assembly, and the discussion mints none.
%% A milestone's documents and its slices', harvested ones included, share one namespace, per
%% `design@model@plan-items-by-section`.
- An id is in the grammar `[a-z0-9]+(-[a-z0-9]+)+`. A thread keeps the slug the discussion minted.
  An argument is numbered `arg-1`, `arg-2`, …, in order of appearance and never reused, in one sequence
  across the documents of a milestone's directory, its harvested slices included, which share one
  namespace.
- An item is cited `<kind>@<anchor>@<id>`, where the anchor is a spec's id, or the milestone's name for
  an item of the milestone document or of any of its slice specs, live or harvested, and only from inside its own plan document, a milestone's documents counting as one: the spec's directory, or the milestone's directory. From anywhere
  else, a commit message and a design head included, the plan document is cited whole, a harvested
  slice's milestone in its place, and an item or a slice is named beside it as #<id>, a name, per
  `primer@where-knowledge-goes`.

**A path the plan document's work will create** is cited `planned@<anchor>@<path>`, under the rules of a
`path` reference: the anchor that will hold it, a trailing `/` for a directory. The form is legal
in the plans directory only. `{{command}} check` reports it once the target exists, and the commit
that creates the file converts it to `path@<anchor>@<path>` in the same change.

%% Argument boundaries are a judgement made once, at assembly, and the transcript reviewer judges
%% none: a boundary drawn differently loses no argument. The rival, arguments marked in each round
%% of the discussion, makes the extraction exact at a cost paid every round.
**Assembly from the transcript.** The plan documents of one discussion record the whole of it:
each thread in the document that carries it, with its proposer, its final state, the arguments on
each side, every argument it uses included, the owner's rulings with their round, and its
relations. An argument that threads in two plan documents use, a milestone's documents counting as
one, is in both, since an item is cited only from inside its own plan. An answer of the owner on a
reading or a fact is recorded with its source, as `primer@labelled-items` says. It is assembled from the transcript, not from memory. Dispatch a
subagent that extracts every session the discussion spans, a resumed session included, as
`agent@knowledge-architect-transcript-reviewer@extract-transcripts` says, read whole, and from the
extraction takes the delta tables, the owner's messages and the arguments; the design
skill's per-round delta is the draft it reads. The transcript reviewer then checks the document
against the same lines. Where one argument ends and the next begins is decided at
assembly, and the transcript reviewer of `skill@knowledge-architect-planning@plan-reviews` checks that no argument was lost. The status section quotes, for
each session the discussion spans, the message where the discussion begins in it, so that a
reviewer reads the same sessions: each is found by that message, never by a session identifier,
as `skill@knowledge-architect-review` says, and the document names no file of the transcript
directory, whose path holds the session identifier and the machine's home directory. It also
numbers the rounds, each named by the opening words of the owner's message, so that an item cites
its round. Where the harness keeps no
transcript, assemble from the conversation, and say so, and why, in the commit that adds the
document.

%% The verbatim words in a committed transcript, and attributions in the items:
%% `design@agent-skills@committed-transcript` and `design@agent-skills@attribution-replaces-quotation`;
%% the generated lines alone committed: `design@agent-skills@plan-commits-the-generated-lines`.
- **The owner's words are attributed in the items**, as `primer@design-heads` says of a durable
  record, each citing its round; the verbatim words are in `transcript.txt`.
- **The discussion's record search** leaves, of what `{{command}} worklist` writes, only
  `generated-lines.md`, copied after `{{command}} worklist --verify` passes, as
  `search/discussion.txt` beside the `README.md` of the document whose draft the search read. Its
  verdict files, `shown.md`, its agent directories, `agents.txt` and `work.txt` stay in the scratch
  space: their quotations go stale as the record changes, and "What is already decided" carries the
  outcome of every verdict that changed something.
- **The transcript** is committed as `transcript.txt` beside the `README.md` of the milestone
  document or the spec, for each session the discussion spans in order: the output of
  `{{command}} transcript show <session> --from "<opening words>"`, without `--tools`, from the
  message where the discussion begins to the owner's last message before the branch that adds the
  plan document merges, so that the rulings given at its reviews are in it: it is committed after
  the review before that merge.
%% The owner's word for each transcript: `design@agent-skills@transcript-pushed-on-the-owner-s-word`.
- **Before that commit, read the transcript whole and remove** every path under a home directory,
  every e-mail address, every credential and token, and the pasted material the owner marks. Then
  show the owner the transcript, and push no commit that holds it before the owner's word for that
  transcript: once pushed, it cannot be taken back, so a word on committing transcripts in general,
  or on another transcript, does not stand for it. A transcript the owner does not approve is
  removed from the branch before any push, and a commit message of the branch says that none is
  committed.

## Cutting the steps and the slices {{slug:cutting-steps-and-slices}}

The tests below hold for the steps of a spec and of a slice, and for the slices of a milestone,
unless one names the slice alone.

**Each step must be able to fail without another step's new machinery entangled in the
failure.** Every other test here serves that one.

- **The first step lands the mechanism empty and measures it.** No content that uses it, no
  observable change against the instruments the Component already has, and its cost measured against a recorded baseline where the Component has one. Every later failure is
  then a failure of the content and not of the mechanism.
- **Order by dependency.** A step that needs a mechanism comes after the step that proves it.
- **A step touches at most one subsystem it did not build.** Two subsystems in one step are two
  failure surfaces with one name.
- **The last step consumes**: the measurements, the report, the harvest.
- **A step whose trees cannot pass the tip's checker alone** lands in the commit of the step that
  makes them pass, per `primer@branch-commits`. The tests above still decide where the steps are
  cut; only their commits are joined.
- **One branch and one merge per slice**, each merged before the next slice begins, and reviewed
  before the merge. A step is no unit of the history: how many commits it takes, and what other
  commits the branch carries, such as the commit of its own a fix met outside the task takes, is the
  implementing session's to judge.

%% Splits planned before the work: `design@agent-skills@splits-planned-before-work`, the code side
%% found by `design@agent-skills@code-split-scout`, the move committed as
%% `design@agent-skills@move-lands-in-move-only-commits` says, the record side
%% `design@agent-skills@subject-split-standard`, a command with no point of view apart, so no agent,
%% per `design@agent-skills@work-divided-by-judgement`. The notice's threshold is the bound less the
%% work's words, per `design@query@clusters-packed-under-a-word-bound`; a remainder's and a goals
%% group's notices, per `design@query@residual-grouped-by-search-unit`, are no subject the harvest
%% writes into. No CI and no pull request named, per
%% `design@agent-skills@installed-text-requires-no-ci-and-no-pull-request`. The search for old paths
%% is a lead, per `issue@model@a-rust-item-named-in-prose-has-no-checked-form`, since no command
%% performs a move, per `issue@knowledge-architect@no-command-renames-or-moves-an-entry-or-a-component`;
%% it leaves a quotation as quoted.
**A split the work needs is a step before the work or the harvest that needs it**, or, in a
milestone, a slice of its own where the move is large: never left to the implementer, and never an
issue.

- **The record side.** Run `{{command}} worklist --out <dir>`, `<dir>` a new directory of your
  scratch space outside the project, with `--work` each `README.md` of the plan document that its
  design audits will take, since the threshold of its notices is the bound less the words of the
  work. Of the notices it prints, keep each one on the cluster of a design subject the harvest
  writes into, a subject holding a head the harvest names; a notice on a remainder or on the goals
  of a cluster names no subject of the harvest. Each subject so kept, and each subject whose heads
  the harvest will make answer questions about parts that change apart, is split by the subject
  standard of `skill@knowledge-architect-decision-recording@current-design`, in a step before the
  harvest that writes into it.
- **The code side.** A split of a module or of a Component that the discussion decided is a step
  before the work that needs it.
- **A move lands in commits that hold the move alone**, each of which builds and passes the
  project's checks before it is pushed, since a commit before a branch's tip may be built by no
  other run. Then search the prose of the tree for each old path of a module the move changes, and
  re-point what names it: the search is a lead, since a path split over two lines escapes it. Leave
  a quotation as it was quoted.

A slice's entry is five level-two sections of its `README.md`, with these titles, in this order, and
`{{command}} check` reports one missing or out of order. Fixtures, where it applies, sits between
Claims and Audit subjects, and is not checked:

- **Builds**: what the slice lands, by name.
- **Claims**: each with the test that could refute it, and how the test is shown to fail against a
  wrong implementation. A claim whose instrument does not exist names the instrument the slice
  lands first.
- **Fixtures**, where the Component drives its tests with authored content: each written in the
  vocabulary that exists, or naming the addition it makes. A fixture that needs something the
  vocabulary cannot express is a finding against the design, not a note for the implementer.
- **Audit subjects**: the files, decisions and specification passages the slice's audit is already
  known to have to read.
- **Fails alone on**: the one or two observations that tell this slice's failure from every other
  slice's.
- **Premises that expire**: where the slice relies on something a later slice or change ends, what
  ends it, and the criterion or guard that watches it.

## Acceptance criteria {{slug:acceptance-criteria}}

An acceptance criterion is a bet about a decision the work is built on, where a claim is a bet about
the work itself: firing a criterion reopens the decision, not the step. It lives in the plan
document of the work that can judge it, in its acceptance criteria section, and in no file of its
own. One of the in-change path, which has no plan document, is
`skill@knowledge-architect-design@discussion-loop` step 7's. Each acceptance criterion of a plan
document names:

- the decision it guards, as a reference to its thread;
- the step or the slice that judges it;
- the observable that fires it.

**A check proposed before the work has a plan document is a lead, not a criterion.** It stays in
the work's `todo` or `deferred` issue, among its leads, and the design session judges it: it
becomes a criterion of the plan document, or it is dropped. It fires nothing while it is a lead.
Evidence that bears on it before the design session is an issue entry of its own, an
`observation` or a `defect` by what it establishes, which the work's issue names.

%% Written as a criterion in every plan document, the reviews' expected result would become the
%% habitual criterion, in place of a specific one that is harder to find.
**The result a scheduled review is expected to give is not a criterion.** Passing the reviews that
every plan document and every slice owes is the baseline. A criterion names an observable specific to
the decision it guards; listing "the reviews pass" in every plan document would stand in for the
specific criterion that is harder to find.

%% `design@agent-skills@criterion-ruled-at-reopening`; a recurring criterion,
%% `design@agent-skills@acceptance-criteria-in-the-document`; the conditions a criterion meets,
%% `design@agent-skills@criteria-test-silent-premises`; the step claim,
%% `design@agent-skills@completeness-check-is-a-step-claim`.
**An acceptance criterion is admitted only where it meets conditions 1, 3 and 4 of the standard
below.** The standard ranks it, so no criterion is put to the owner, wherever it is
proposed: at the premortem, at the assembly, by a review or at an audit. Its observable is worded
by the agent, and a rewording is listed in the turn's "Decided" section, per
`primer@decided-section`.

**A check whose firing would show the work unfinished is a claim of the step that builds it**, with
its test, which the spec-conformity review judges. It is no criterion, since a review or a test
already reports it.

Where the basis of a number in a criterion leaves a range of values, the choice among them is a
fork: the session lists its value under the defaults awaiting the owner, and
`primer@design-heads` records the owner's approval of it as the session's choice, and a value the
owner sets as the owner's.

- **At each landing**, the messages of the commits that land it report on every criterion judged
  there, one line each, naming it as #<id> beside a citation of the whole plan document, per
  `skill@knowledge-architect-planning@spec-contents`: the decision guarded, fired or not, and the evidence.
- **A criterion that fires** leaves the document at once, as a reopened decision: its firing opens
  a design discussion with the owner on the decision it guards, under
  `skill@knowledge-architect-design`. There, the measurement says whether the decision failed, the
  argument settles the shape that replaces it where it can, and the owner rules its forks. The part
  of the work that rests on that decision waits for the discussion, and the rest goes on. A
  criterion is judged by its observable,
  wherever the observation was made: one that a later review or test shows firing after a landing
  reported it as not fired is reported as fired in the next landing commit, which names the earlier
  report as superseded.
- **When the document leaves**, the landing that deletes it reports on every criterion once more.
  One that did not fire and recurs at later work is judged as a cause by the standard below, and
  written as `skill@knowledge-architect-decision-recording@ruled-tripwires` says, or, where an
  entry already guards the same premise, folded into it. One that is spent, or that the standard
  drops, is deleted with the document.

The standard a recurring criterion is judged by:

{{fragment:tripwire-standard.md}}

## Working a slice, and the work of a spec {{slug:working-a-slice}}

The work of a spec follows this procedure too, as its last paragraph says.

**Where the owner's word is awaited.** A default the plan documents list as awaiting the owner is
built as they write it until the owner rules on it, and its branch merges once the owner has, per
`primer@question-authority`. A scope change listed there does not start
before its ruling. A load-bearing gap of point 2 is not such a default, even where the message
that puts it to the owner names one: it stops the slice, the whole of which rests on the decided
design the gap defeats or on the ruling the documents reserve to the owner.

1. **Ground**: the Component's `CLAUDE.md`, its design home, its `rejected` home; then the
   milestone's `README.md` entire, then the slice's `README.md`, and the items of a
   `deliberation.md` that either cites where a shape's argument is needed. The issues and the
   tripwires, of every anchor, are read at the audit.
%% The fit lines, one per entry given a verdict other than `unrelated`: a judgement that an entry
%% bears on nothing is the claim a later reader cannot check without it, and an entry that did bear
%% and was dismissed is the failure the search exists to prevent. The rival, no record of such an
%% entry, sent the transcript reviewer to report every unrecorded judgement as an item with no
%% outcome. The command writes them, per `design@agent-skills@record-searched-before-the-work`.
%% A fit and a gap, and what the audit reads: `design@agent-skills@design-audit-checks-fit`.
2. **The design audit.** Read the slice's entry and every decided shape it depends on against the
   code as it stands and against the design homes. List every gap: a shape the code refutes, a
   passage of a specification the entry did not read, a name the entry uses that the code does not
   have, a consequence the entry did not see, and **a standing entry the slice's planned code bears on**:
   a tripwire whose firing condition, or a `deferred` issue whose trigger, the planned code meets, and an issue of any kind the slice's code touches, closes, makes worse or depends on. A cause of the milestone document's premortem whose firing condition the planned
   code meets bears on it too: read it in the milestone document, since no register holds it before
   the harvest. Dispatch the
   search of the whole record as the description of `agent@knowledge-architect-record-searcher` says, with
   `--work` the milestone's `README.md` and the slice's `README.md`, and no other file,
   `--subject slice-audit`, `--seeds-from` the slice's `README.md`, and `--since` the commit that
   last changed the slice's `README.md`, `git log -1 --format=%H -- <slice>/README.md`; for a spec,
   `--work` and `--seeds-from` the spec's `README.md`, `--subject spec-audit` and `--since` the
   commit that last changed it. `deliberation.md`, `search/` and `transcript.txt` are never the
   search's work: the decided design carries every condition the search checks. `--seeds-from` takes as
   seeds the design entries and the declared-register entries the `README.md`'s section "What is already
   decided" cites. The search's check writes, beside the verdict file of its subject, `gaps.md`, each verdict other than `constrains` and
   `unrelated`, with its entry's text: read it, and sort each verdict into the sorts below as a gap;
   a `constrains` verdict is a fit. A gap whose entry `gaps.md` marks `changed-since` is on an entry
   the spec's design could not have read in its current text, since its file changed after the
   spec did. The audit's record is `fit-lines.md`, whole, the line of each
   entry given a verdict other than `unrelated`: in the message of the audit's commit, or, where the
   audit applies nothing in place, in the message of the slice's first commit; and, in that same
   commit, copied as `search/design-audit.txt` beside the audited `README.md`, the slice's or the
   spec's. Nothing else the search wrote is committed, `gaps.md` included. A firing found at the audit is
   ruled before the code is written, where one found only by the review of the harvest reopens the
   harvest.
%% The scout and the notice again at each audit, since the code and the record move between the
%% plan's writing and its work: `design@agent-skills@design-audit-checks-fit` and
%% `design@agent-skills@code-split-scout`; the report is no file
%% of a plan document's directory, per `design@model@plan-document-directory`.
   Dispatch `agent@knowledge-architect-code-split-scout`, as its description says, with the
   `README.md` of the slice or of the spec as its work, since it reads the code from a point of view
   apart from yours: it judges the units that work writes into. Read the
   notices of the search's own `{{command}} worklist` run as the record side of
   `skill@knowledge-architect-planning@cutting-steps-and-slices` reads them, over the audit's area.
   The scout reports signals and decides nothing: weigh each signal and each notice, and decide
   whether and how to split as the sorts below decide any gap, since each one the planned steps do
   not take is a gap, listed with the other gaps in the audit's commit message. The scout's report
   stays in your scratch space. Sort each gap:
   - **Applied in place.** The gap has one answer the document's decisions already imply, or is a
     choice among shapes the document rules out all but one of. Apply the answer in place, in the
     milestone document or the slice's spec, wherever the shape it changes is written. Commit the
     amended documents and `search/design-audit.txt` alone, with a subject that names the slice's design audit in those words,
     and a message listing each finding: the gap, the answer and the decision it follows from, and
     the lines of `fit-lines.md`. Where the slice's first change sets what the per-commit gate
     requires, per `primer@branch-commits`, the amended documents land in that change's commit
     instead, and its message carries the audit's subject and list. Earlier audits are found with
     `git log --grep='design audit'`.
     **An answer that relaxes a ruling of the owner, or turns it from its direction, is a scope
     change even when it is the one answer the document implies.** It is applied with the
     others, and also listed in the milestone document as a default awaiting the owner. A premise
     the session supplied to such a default is checked before the owner rules on it, per
     `primer@owner-word-premise`.
   - **A change to the slices.** An audit that finds the work needs another slice, or a slice split
     or reordered, writes each new slice's spec with its harvest row, adds the slice to the
     implementation sequence and the order rationale, and moves into its spec the design it takes
     from other slices, so that each shape keeps one home. It changes a sequence the owner ruled on, so it is a scope
     change, listed and ruled as above.
   - **A choice between shapes.** The gap is a choice between two or more shapes, none of which the
     document rules out, and it defeats no decided design: the rest of the slice does not rest on
     it. It is put to the owner in the form of the paragraph after these sorts, as a default
     awaiting the owner, built as written, and the branch merges once the owner rules, per
     `primer@question-authority`.
   - **Load-bearing.** The gap is material, or needs a ruling the document marks as the owner's.
     Record it in the slice's spec as open at the audit, with the discriminating fact, stop the
     slice, and open a design session with the owner under `skill@knowledge-architect-design`.
     - A gap whose ruling the document marks as the owner's, and whose shapes can be stated in
       full, each with its consequence, takes the form of the paragraph after these sorts, and waits
       for the ruling.
     - A gap that defeats a reason, a premise or a criterion a thread closed `approved` or `argued` rests on needs the
       full session. Its converged design goes into the milestone's documents by the rule of `skill@knowledge-architect-planning@spec-contents`,
       and owes the reviews of `skill@knowledge-architect-planning@plan-reviews`.

     The slice resumes from the ruling or the converged design.

   A choice among shapes that can be stated in full, each with its consequence, is put to the owner
   in one message, with a default; several such gaps go in one question, each under a label,
   `Q<n>`, since each is written in place as a thread with its slug. The slice's documents exist, so
   the ruling is not left to a commit message as the in-change path would leave it: it is written in
   place in the milestone's documents by the rule of
   `skill@knowledge-architect-planning@spec-contents`, as a thread attributing the owner's ruling,
   like the audit's other answers, and the audit's commit lists it among its gaps. It is no
   converged design session, so it owes no new review of
   `skill@knowledge-architect-planning@plan-reviews`.
%% The pointer to the primer's section answers its dilution in a long session, per
%% `rejected@agent-skills@head-rules-whole-in-primer`: implementation starts at this point.
3. **Claims, tests, implementation, gates, commits**, per the project's development procedure. The
   commits name each claim of the spec with its test and its outcome, how the test was shown to fail
   against a wrong implementation, and of any claim whose test cannot yet do so, why not. A claim
   that holds over every site of a corpus is tested by reading the corpus: a phrase search is a lead,
   which misses a phrase split over a line break and a site written in other words. Who carries
   the implementation, the session or an agent it dispatches, is judged as
   `primer@dispatching-work` says.
4. **Review of the work**, per `skill@knowledge-architect-review`. A repair lands as
   `skill@knowledge-architect-review@what-review-leaves` says.
5. **The report** on each acceptance criterion judged at this slice, as
   `skill@knowledge-architect-planning@acceptance-criteria` says.
6. **The harvest**, per the harvest row of the slice's spec: the decisions and the losing
   alternatives under `skill@knowledge-architect-decision-recording`, then the premortem's causes, judged and
   written as `skill@knowledge-architect-decision-recording@ruled-tripwires` says, and the issues under
   `skill@knowledge-architect-issue-tracking`. `skill@knowledge-architect-decision-recording`, which
   the harvest loads, delivers what earns a head and how it is shaped, the thread's slug and its
   pair included. The row names what is judged; the entry tests and the tests for a losing
   alternative decide whether each decision and each alternative earns an entry, the rules on a head's shape
   decide how the decisions are cut into heads, and they govern where the two disagree: an item of the row the tests exclude is named in the harvest's commit,
   with the test it fails. Where a design home is a directory, a new subdocument is
   linked from its README. **The harvest's review is the review before the merge**, as
   `skill@knowledge-architect-review@review-axes` defines it and says what it owes: the harvest
   writes the record the decision-record, routing and standing-state axes judge, so the review of
   point 4 cannot see it.
7. **The slice's spec is harvested** in a commit of its own, after the review before the merge has
   read the tip that holds it, as `skill@knowledge-architect-planning@plan-document-leaves` says. What crosses slices stays in the
   milestone document, amended in place where the landing changed it.

%% A spec of several steps on one branch, with no stated procedure, cannot tell which points of a
%% slice's procedure apply to which part of its work. Its audit reads the design against the tree as
%% it stands and recovers what a session that did not witness the discussion lacks, so a spec
%% implemented by the session that wrote it, on an unmoved main branch, likely gives it nothing to
%% find. The rival, an audit for every spec as for every slice, costs a reading with nothing to find;
%% it is the first candidate of `issue@agent-skills@a-spec-worked-without-its-design-audit-missed-a-critical-gap`,
%% which reopens this condition.
**The work of a spec** takes the same points once, for the whole spec, on its one branch:

- point 1, the grounding;
- point 2, the design audit, only when the work does not start in the session where the discussion
  converged, or when commits other than the spec's own have landed on the main branch since the
  spec was written. Otherwise the session goes straight to the work, since the design was read
  against the tree as it stands;
- point 3, the claims and checks of each step;
- point 4, the review of the work, once;
- points 5 and 6, the report and the harvest, in the commits that land the work;
- point 7, the deletion of the spec's directory, in a commit of its own after the review before the
  merge, as in `skill@knowledge-architect-planning@plan-document-leaves`.

Where a point names the milestone's or the slice's `README.md`, the work of a spec reads the spec's:
its defaults, its threads, its harvest row, and the design audit its commit subject names.

## Reviews of a plan document {{slug:plan-reviews}}

A plan document is committed first, on a branch of its own or on its work's branch, and that
commit is what the reviewers read; a repair lands as
`skill@knowledge-architect-review@what-review-leaves` says. **It may be merged on its own,
whatever the time of its work**: a plan document on the main branch keeps the work done meanwhile
from drifting from it.
**One whose work's first commit changes what the project's per-commit gate checks is merged
before that work**, a spec as well as a milestone document: on one branch,
`primer@branch-commits` would put that commit before the document that plans it, or squash the
two into one. A change that only a check of the working tree sees does not count. It is read
again after its decided design changes, by a design session that converges or by a ruling of the
owner, as at a load-bearing gap of a design audit. A change made in answer to its review, a default written in included, is a
repair, reviewed again only as `skill@knowledge-architect-review@what-review-leaves` says, and an
audit applied in place is no revision. Its reviewers are **fresh, never a fork**: a fork inherits the discussion and reads the document as its
author. Dispatch them through `skill@knowledge-architect-review`, with the invariants that
skill lists, the blind brief included, on every axis of its table whose condition holds, among them
these four:

- `agent@knowledge-architect-cold-implementer-reviewer` reads the document as the implementer of its
  work, or of a milestone's first slice, and reports every place where it cannot act: undefined names,
  shapes without enough detail to write, procedure gaps, ambiguities, and what it would have to
  reconstruct from a conversation it did not see. It also applies the readiness checks below.
- `agent@knowledge-architect-code-claims-reviewer` verifies every statement the document makes about
  the code as it stands, and reports each as confirmed, wrong or imprecise, with the evidence.
%% `design@agent-skills@plan-read-against-the-record`.
- `agent@knowledge-architect-design-conformance-reviewer` reads the document against the project's
  record: the goals, the design heads and the rejected designs of every Component it touches.
  It reports a shape, an acceptance criterion, a default, a step or a harvest row that contradicts
  a goal, that contradicts or widens a head the document does not list as reversed or rewritten,
  or that brings back a rejected design, and reads beside them the verdict files of the discussion's
  record search, whose directory its brief names, and the document's `search/discussion.txt`,
  reporting a verdict the document leaves unanswered. A conflict with a goal goes to the owner.
- `agent@knowledge-architect-transcript-reviewer` reads the discussion's transcripts and checks that
  everything the discussion established that must outlive it is in the document or has another
  durable outcome, and that no ruling of the owner is recorded reversed, relaxed or turned from its
  direction. Dispatch it on every assembled document, in the batch or alone and last
  as `skill@knowledge-architect-review@review-axes` says, and name in its brief the sessions the
  assembly read, with the message where the discussion begins in each.

%% The `D` label of a default: `design@agent-skills@ruled-items-labelled`. The verification of a
%% finding: `design@agent-skills@at-least-one-independent-verification`.
**What their findings become.** Check each finding against the tree, or against the transcript,
before acting on it. A material finding is answered as
`skill@knowledge-architect-design@decision-authority` says, read whole, and the answer is written
into the sections it touches. **A default the record, a measurement or a source settles is the session's decision**: it
is written into the sections it touches with its ground, takes no label, is not listed under the
defaults awaiting the owner, and is reported in the turn's "Decided" section, per
`primer@decided-section`, where the owner can contest it. A
default that `primer@question-authority` gives the owner, a fork among them, is listed under the
defaults awaiting the owner, with the thread it bears on and a label, `D<n>`, numbered as
`primer@labelled-items` says. A ruled default stays in that list,
marked as ruled, with its label, since the sections it was written into cite it by its label. A
finding that is a gap with one answer is applied in place, except that **a gap whose one answer
relaxes a ruling of the owner, or turns it from its direction, is a scope change**, as at the design
audit. A finding that is wrong is dropped, with
the reproduction that showed it wrong kept in the commit message. A scope change is put to the
owner, listed under the defaults awaiting the owner. A default that rewords a goal is drafted as its
own item under `skill@knowledge-architect-goal-setting@drawing-out-intent`, never ruled with the
other defaults. A refinement within a ruling's direction,
stricter, more detailed or reworded, and a wording better than the one the owner was shown, are no
findings, except in a goal, which binds outright: a goal that rules out what the owner has not ruled out knowingly is a finding, per rule 4 of `skill@knowledge-architect-goal-setting@goal-or-not`, and goes to `skill@knowledge-architect-goal-setting@drawing-out-intent`.

**The readiness checks**, applied by the author before dispatch and by the cold implementer after.
This list is their one home; the reviewer reads it here.

- every name the document uses is defined in the names sections or exists in the code;
- every total function the code will need is a mapping table;
- every condition of a decided thread is in the decided design of a `README.md`, never in
  `deliberation.md` alone;
- every fixture is expressible in the vocabulary that exists, or names its addition;
- a milestone document points to `skill@knowledge-architect-planning@working-a-slice`;
- every number is a measurement with its instrument, or a default marked as the owner's;
- every premise a later step or change ends is stated with its guard;
- every statement about the code names the file and the function, and its truth is the code-claims
  reviewer's to establish;
- every acceptance criterion names the decision it guards, the step or slice judging it and the
  observable that fires it;
- every thread maps to a section and to a harvest home;
- every text referencing a decision the work reverses or rewrites is named, with the step, slice or
  harvest that judges or updates it;
- every section the document owes by `skill@knowledge-architect-planning@spec-contents` is
  present, and an empty one says so.

## When a plan document leaves {{slug:plan-document-leaves}}

%% A slice harvested rather than deleted: `design@agent-skills@harvested-slice-kept`; a spec and the
%% milestone's directory leaving whole: `design@agent-skills@spec-or-milestone-leaves-at-landing`.
**A spec or a milestone is deleted, its whole directory, in a commit of its own, once its last
harvest is complete and the review before the merge has read the tip that holds it, and that
commit's message cites it by its kind**, `spec@plans@<id>` or `milestone@plans@<id>`, which
resolves against the commit's parent. The reviewers of that review, the transcript reviewer's
search included, read the document in the tree they judge, and a pair a harvest row states is in a
tree `git log -G` reads.

**A slice's spec is harvested when its slice lands**: its directory is moved under its milestone's
`harvested/`, `git mv <milestone>/<slice> <milestone>/harvested/<slice>`, in a commit of its own
after the review before the merge has read the tip that holds it, whose message cites it as
`spec@<milestone>@<slice>`, which resolves against the commit's parent. The move changes nothing in
the slice's files: from that commit on they are frozen, and they leave with the milestone's
directory. The same commit re-points the milestone document's link to the slice, to
`harvested/<slice>/README.md`, writes the milestone's `index.md` again with
`{{command}} index`, which lists the slice as harvested, and repairs each citation of the slice, or of an item it defines,
from outside the milestone's directory, as the paragraphs below say of a leaving document's
citations, since the checker refuses them once the slice is harvested.

The milestone leaves once its last slice has landed and every default it lists is ruled: in a
commit that deletes its directory, the last slice's spec with it, or, where a default is still
unruled then, after the last slice is harvested, in the commit that records the last ruling.

Before deleting a plan document or harvesting a slice:

- every harvest row of the document is done;
- every acceptance criterion has been reported on, and has become a tripwire or left (`skill@knowledge-architect-planning@acceptance-criteria`);
- what the document established is in the design homes and the registers; what stayed a guess
  leaves with it;
%% Later consequences: the work they hand on is neither a decision built nor a guess, so neither of
%% the two bullets above carries it, and deleting the document would take it out of every tree.
- each of its later consequences is in the issue entry or the plan document of the later work it
  names, or in a `todo` issue opened for it where that work has neither, or a `deferred` one if an
  event gates it;
- every default the document lists as awaiting the owner is ruled, since a design head records such
  a default only once the owner has ruled on it;
- its row of the roadmap, if it has one, is removed (`skill@knowledge-architect-planning@spec-or-milestone`).

**A citation of the leaving document from another plan document dangles**, and so does a
citation of a harvested slice from outside its milestone. The session that meets
it, the one deleting the document or harvesting the slice, or the one rebasing the citing
document's branch onto that commit, does two things in one commit:

- it removes the citation from the citing document;
- it opens a `question` issue in the root's issue register, under
  `skill@knowledge-architect-issue-tracking`: does the citing document still hold now that the
  leaving document's work is built, accounting for deviations or other unplanned happenings? It is
  answered by reading the citing document against what the leaving document harvested. The
  issue cites the citing document, so it cannot outlive it, and its `Why it matters` cites the
  leaving document's harvested design entries, since the leaving document no longer exists. For a
  milestone, its next slice's audit reads the issue.

A milestone's own documents count as one plan document: a harvested slice's spec still resolves
from inside its milestone's directory, so its milestone document and its sibling specs keep their
citations of it, and no `question` issue is opened.

Any other citation of the leaving document, from an issue, a design head or a `CLAUDE.md`, dangles
too, and `{{command}} check` reports it: the deleting or harvesting commit repairs each, as
`skill@knowledge-architect-issue-tracking` says of a sentence whose reference dangles, and an issue whose
subject was the leaving document closes with it.

A reader who needs the deliberation later finds a harvested slice's under its milestone's
`harvested/` while the milestone is open, and every deleted document's in history:

```sh
git log --diff-filter=D --name-only -- docs/plans/         # every deleted plan document's files, with their commit
git show <commit>^:<path of the file>                       # a file as it stood before deletion
```
