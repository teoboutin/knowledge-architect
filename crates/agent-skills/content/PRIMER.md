# The knowledge-architect workflow

%% The primer is the part every adopting project's root CLAUDE.md would say alike:
%% `design@agent-skills@primer-limit`.
This project keeps its design record with knowledge-architect. Its documents are checked by
`{{command}} check`: every reference between records resolves, every record has the shape its
register declares, and every generated listing is current. A register is one kind of record, such
as the design decisions or the issues, and its home is the file or the directory that stores its
entries; the two words stay apart because a rule can need both, as "a slug is defined in its
register's home". This primer is installed, and the project's root `CLAUDE.md` imports it. The
project's own rules are in its root `CLAUDE.md`, beside this primer, and add to it.

## Goals bind; decisions bind as a presumption {{slug:goals-bind}}

**The goals are the only statements assumed to come from the owner.** Each Component states them
in its goals home, `register@*@goal`, one heading per goal. A recorded decision was reviewed, but its review can
miss a detail or an implication, more often as the volume of agentic work grows. A decision binds
as a presumption, which a better argument may rebut, and reversing one is an ordinary move with a
procedure (`skill@knowledge-architect-decision-recording`). **When a decision conflicts with a goal,
the likely cause is that the owner missed the conflict: the goal prevails, and the conflict goes to
the owner.**

## A question goes to the authority that answers it {{slug:question-authority}}

What an external specification the project implements says is answered by its text. What the code
or a tool does is answered by a measurement. What follows from the goals and the recorded decisions
is answered by the argument from them. A question one of these answers is answered by the session,
with its source cited, and the owner is told, not asked.

**The owner answers what none of them settles**: a goal to set or change; a fact only the owner
holds, such as a plan or what the owner observed; a contradiction between the owner's
words, the goals and the record that no argument resolves; an action that cannot be undone; and a
fork, where two or more shapes meet every binding constraint and no fact defeats
either, so that choosing between them weighs a cost or a scope that no goal or decision ranks. A
cost alone makes no fork: a shape that fails a goal, or loses to a fact, is out, whatever it would
save. A question put to the owner names the authorities consulted and, for a fork, each surviving
shape and what it trades. **A default put to the owner is built, and the branch that builds it
merges once the owner has ruled on it**: the work goes on with no wait, and the owner rules before
the default reaches the main branch, at its merge at the latest. Which further actions a project
reserves to its owner, such as a merge, is the project's to state. **Where an installed instruction sends to the owner a question this
section gives the session, this section prevails**.

## The owner's word and its premise {{slug:owner-word-premise}}

**A word of the owner holds only as far as its premise.** A ruling given on a premise, stated by
the owner or supplied by the session, does not rule on the case where that premise is false. A
premise the session supplied is the session's to check before the owner rules on it.

**An answer of the owner on a reading or a fact gives way to its source.** Where the owner answered
what a specification's text says, or what the code or a tool does, and the text or a measurement
says otherwise, the source applies at once: the work and the records follow it, and the owner is
told at the top of the turn, with the source cited, and in the turn's "Decided" section, per
`primer@decided-section`. A part of the work that cannot be undone still waits for the owner's
answer. Whether the project follows a specification, and how closely, is the project's own goal or
decision: a departure from a specification is a scope decision the project states, never inferred
from an answer to a reading question.

**A ruling on intent whose premise proves false is a contradiction.** A ruling on intent is a word
of the owner on a question `primer@question-authority` reserves to the owner; a choice at a fork
that a new argument or fact ranks is none, and reopens in the weak form `primer@design-heads`
states. Put the corrected
premise to the owner at the top of the turn, quoting the word it defeats; the part of the work that
rests on it waits for the answer, and the rest goes on.

## Items put to the owner under a label {{slug:labelled-items}}

%% The labels and their records: `design@agent-skills@ruled-items-labelled`.
**An item put to the owner under a label**, `Q<n>` or the prefix a skill gives it, **takes the
next number of its prefix in the record the label reaches, from 1, and that number is never reused
there.** That record is a plan document, a milestone's documents counting as one; a file; or the
commit messages of one branch, counting as one. A label carried later with its item into another
record keeps its number, and that record numbers nothing.
A label that reaches none is numbered within the message that puts it to the owner, wherever the
item's ruling is kept.

**A label numbers what was put to the owner; it never names what decided.** An answer on a question
another authority answers, per `primer@question-authority`, is recorded as a reading or a fact with
its source, never as the owner's ruling. Where the source leaves the point open and the owner chose,
the record says that the owner chose among the readings the source permits.

## Room to judge {{slug:room-to-judge}}

**Where the installed text is silent, judge.** It leaves that room on purpose: it states the
instructions the workflow needs and the reasons for them, and leaves the rest to the session. Room
to judge is never room to act against an instruction.

## Intent and claims {{slug:intent-and-claims}}

- **A design home is built intent**: the design as built and its reasons, and the decisions that
  no work implements and that are not part of any spec, recorded when made. Design that is decided and not built is in a plan
  document until it lands. Check the code against a design home, never the other way. A divergence
  is a defect in one of them: say which. Where the head stands, its argument undefeated, the code is
  the defect: fix it, or open an issue. Where the session finds the head's
  argument defeated, the head's
  reversal goes to `skill@knowledge-architect-design`, and to the owner where reversing it is a
  fork; the part of the task that rests on the point in
  divergence waits for that answer, and the rest of the task goes on. A design home can be wrong,
  and it still prevails over the code until the divergence closes. It closes when
  the code changes to meet the head, or when the head is reversed under
  `skill@knowledge-architect-decision-recording`; it never closes by following the code. Work on
  that point that goes on meanwhile builds on the head.
- **A reason recorded at the code** (an inline comment saying why the code is shaped so, or the
  message of the commit that argued it) is intent at the scale of that code. It binds as a
  presumption, below the design home: a change that defeats it argues against that reason in its
  own commit message, and where it conflicts with a design home, the design home prevails, as in
  any divergence. Between a comment and a commit message, the comment is the current statement.
  **Before removing or reshaping code as unneeded, read its comment and the message of the commit
  that introduced it** (`git log -L`, `git blame`): absence from the design home is not evidence
  that code is superfluous.

## Verify before relying {{slug:verify-before-relying}}

A document can describe the code as it was committed rather than as it is. Confidence is not
correlated with correctness: the more obvious a behaviour looks, the less likely anyone has
checked it. An agent tends to assume that the project and its tools behave as their names, their
documents and its own expectations suggest, and to act on that assumption without checking it.

Before relying on a claim about how anything behaves (an API, a tool, a script, a file format, a
test mechanism, an issue's diagnosis; the list is illustrative, not a boundary):

- **Establish it from the implementation, or from a run you observe.** Names, doc comments and
  prose are leads to check, never evidence. A scoped `CLAUDE.md` invariant, a doc comment and an
  issue's diagnosis go stale.
- **Follow the whole path, not the entry point**: what calls it, what it returns on success _and_
  on failure, and what the caller does with that.
- **One confirming case does not establish a convention.** Widen the check whenever the answer
  generalises.
- **Report the evidence with the conclusion**, and label anything unverified as an assumption. A
  plausible mechanism is not a finding.
- **Re-verify a conclusion before acting on it**, and correct it in place the moment evidence
  contradicts it.
- **A transfer from another project names the property it depends on, and checks that the property
  holds here.** Naming the pattern is not enough. A claim taken from another project's record,
  such as a count of occurrences, is checked against that project's registers as well as its
  history: its rejected designs and its issues often hold what its commit log does not.

**Evidence is textual and cheap**: a verbatim reading of a file, a measurement over a tree, a
failing test. Build it rather than trading intuitions.

### Verify a claim before writing it

**A checkable claim is checked before it is written down**, against the tree at the moment the
sentence is written:

- a number comes from a measurement you can point to, with the machine named; a figure whose
  instrument the same change altered is taken again;
- a count is re-counted: files, lines, entries, references, occurrences;
- "a search returns nothing" and "the only hit in the tree" are run, against the revision the
  sentence describes, so that they match across a line break and across emphasis marks, since
  wrapped prose splits a phrase over two lines and inline markup splits it inside one;
- "X already says this" and "the skill requires that": open X and read it;
- a slug, a path, a section, a commit or a run identifier is resolved;
- "this is fixed" and "that entry is closed" are diffed.

**A commit message is a document for this purpose.** It is durable, and `{{command}} commits`
reads only its pointers, its references, its paths, its skill names and its SHAs: a measurement, a count or a claim about the tree in a message is checked by nobody but
its writer. Before each commit, list every factual claim its message makes: a count, a search and
its result, a section or line number, the name of a file, a slug or a commit, a quotation or an
attribution of the owner, or a statement that something exists, was searched, was found or
changed. Beside each, write the command or the file that checks it; run or open each against the
tree the commit holds, and check a quotation or an attribution against the conversation. A claim no
check supports is removed, or labelled as an assumption. Writing the message from memory of the
work is where the false claims come from.

**The same checks apply to an answer given to the owner in the conversation.**

### Check whether it is already known

Before diagnosing anything as a problem, or reporting one met while doing something else, find
out whether it is already recorded: `{{command}} issues`, `{{command}} tripwires`, and
`{{command}} show <kind>@<anchor>@<id>` for one entry and every reference to it. A recorded entry
usually says more than a fresh diagnosis: the measurement already taken, what was ruled out, and
often why the work was left undone on purpose.

### Read the record of code before changing it

%% Per `design@agent-skills@code-change-reads-its-record`.
**Before proposing a fix or any other change to code, read what the project decided about that
code**: each entry its comments cite; every entry that cites its file, which
`{{command}} show path@<anchor>@<file>` lists; and the heads on its subject in the design home of
the Component that holds it, where each head's title states its decision, with the entries on it in
that Component's `rejected` home. A fix written from the code alone can undo a decision whose reason
the code does not show. A fix that would contradict a head, or bring back a design the `rejected`
home records, is a decision that goes to the design skill, per `primer@design-heads`.

### Precedent is not authority

The instructions that apply are the installed ones, the project's root `CLAUDE.md`, its skills,
and the scoped `CLAUDE.md` next to what is edited. What already exists in the project is evidence
of what was done, not of what should be done.

- **"The existing ones do it this way" is not an argument, and neither is how many of them do.**
  Where an instruction and the surrounding practice disagree, the instruction wins. Where you
  think an instruction is wrong, say so and argue it; changing an instruction is a normal move, and
  silently following the older practice is not.
- **Match local idiom only where the instructions are silent.** Naming and the shape of the
  surrounding prose are worth matching, so a diff stays readable.
- **Do not spread non-conformance while working around it.** Starting from an existing document or
  entry copies its defects with it: read what you copied against the instructions before extending
  it.

## Working prose {{slug:working-prose}}

These rules govern working prose: every text the project's contributors read, such as a
discussion with the owner, a document of the project, a commit message and a code comment. Text
the product's end users read follows the product's own style contract, where literary value can be
a deliberate goal. The test is who reads the
text, not where it is stored.

- Write plain, direct technical English, in short sentences. Readers might be non-native
  speakers: where a simpler word or a shorter sentence says the same thing, use it.
- Use precise technical terms. No idioms, colloquialisms or aphorisms: they read differently to
  different readers, and they carry a qualitative judgement past argument.
- No undefined metaphor: the meaning is carried explicitly. A metaphor that names a defined
  concept is vocabulary once defined.
- Use em dashes and semicolons sparingly. They usually join two sentences that read better apart.
- Prefer explicit quantities, units and invariants over qualitative wording. A qualitative claim
  may state a goal or an observation. Offered as an argument, it is replaced by its checkable form:
  a count, a bound, an invariant, a named failure it prevents, or a test either party could run.
  The test: delete the sentence; if the argument falls, the sentence was the argument, and it is
  replaced by its checkable form, not reworded in other qualitative terms. An owner's ruling needs
  no such form.

## Something met outside the task {{slug:met-outside-the-task}}

Something met while doing other work takes the first of these that applies:

| # | test | outcome |
| --- | --- | --- |
| 1 | it bears on the current work: the work's result, or a decision it rests on, is wrong or incomplete without it | route it as `primer@question-authority` says: what a source, a measurement or the record answers, the session decides and reports; a fork, a goal to set or change, or a fact only the owner holds, goes to the owner at the top of the turn, with a default the work proceeds on unless the owner answers otherwise, a goal standing as it is meanwhile; a contradiction, or an action that cannot be undone, holds the part of the work that rests on it until the owner's answer; a divergence between a design home and the code is routed as `primer@intent-and-claims` says |
| 2 | its fix is checkable from the diff alone: it changes no behaviour, no decision and no test outcome (a typo, a stale pointer, wording that is now false, a broken link) | fix it, in a commit of its own |
| 3 | its `Why it matters` and its `What would close it` can be written | open an issue entry (`skill@knowledge-architect-issue-tracking`) |
| 4 | none of the above | name it, with why it is dropped |

%% The route is `design@agent-skills@installed-fix-goes-upstream`.
**An item whose fix lies in the installed text or in the checker**, not in what the project adds to
them, such as a skill of its own or a check its own build of the checker adds, goes to the
workflow's file of `skill@knowledge-architect-retrospective`, whether or not row 1 also takes it
for the current work: the project edits neither, and the workflow's maintainers read none of its
issue entries. It is never fixed in the project, and it is opened as an issue entry of the project
only where a pointer that no checked form expresses needs one beside it, as
`primer@where-knowledge-goes` says.

**A turn that met anything outside its task ends with a section titled "Met outside the task"**,
listing every item with its outcome: fixed (with the commit), decided (with its ground, also
listed under "Decided"), issue opened (with its id), kept for the retrospective's workflow file, waiting for the
owner's ruling, or dropped (with the reason). A mention inside other prose does not count.
A subagent that edits nothing reports each item at the end of its report, under a heading "Met
outside the task", and the session that dispatched it routes each item by this section.

## Decisions reported at the end of a turn {{slug:decided-section}}

**A turn in which the session decided something the owner could have decided ends with a section
titled "Decided"**, before "Met outside the task" where both are owed: one line per decision, each
with its ground, a reference or a source. A design thread closed on its argument, with the rivals
it rules out, a default the
record answered, a source applied over an answer of the owner, and a premortem cause recorded as a
tripwire or dropped, and a tripwire's wording or scope changed or the tripwire removed, are such decisions; an implementation choice that a commit message records is not. A
subagent reports its decisions at the end of its report, under the same title, and the session that
dispatched it carries them into its own section.

## The project's documents and how they are checked {{slug:checker-model}}

**The manifest**, knowledge-architect.toml at the project's root, declares the Components, the
locations, the registers, and what is exempt from the checks. `{{command}} anchors` lists what it
resolves to: every anchor with its kind and its path, and, under every anchor but a spec or a
milestone, the home of each register it carries.

**An anchor** is a named place that carries registers. A **Component** is a directory the manifest
declares, named by its basename, or the project's root, named by the manifest's project name; it
carries the required documents,
every component register, and each `opt-in` register whose `components` names it. A **location** carries only the registers it declares, their homes
directly under its path, or at its path for a register declared with `dir = "."`. The checker
constructs the anchor `plans` at docs/plans/ and one anchor per spec file, and, under the home of
every directory register, wherever that home sits, one anchor per entry directory, each milestone
included. The anchor words `*` (every Component), `elsewhere` (a
path the tree does not hold) and `plans` are reserved.

**Every Component carries** `path@*@README.md`, `path@*@CLAUDE.md` while the project serves the
`claude` harness, and the home of each component register: `register@*@goal`,
`register@*@design`, `register@*@rejected`, `register@*@tripwire` and `register@*@issue`.

**A register's entries**:

- a **heading register**, such as `design`, `rejected`, `goal` and `tripwire`, keeps its home in
  one file, or in a directory behind a `README.md` that links every subdocument. An entry is a
  heading at the register's level, its statement first and its slug last: level 3 for `design` and
  `rejected`, level 2 for `goal` and `tripwire`;
- a **file register**, such as `issue`, keeps its home in a directory: one file per entry, whose
  basename is the entry's id, beside a hand-written `README.md` and a generated `index.md`;
- a **directory register**, such as `milestone`, keeps its home in a directory: one directory per
  entry, holding a `README.md`, beside a hand-written `README.md` and a generated `index.md`. Each
  entry is an anchor named by its basename, carrying at its own directory the register that a
  declared directory register's `carries` names, so an entry of it is cited `<kind>@<entry>@<id>`;
- a **plan document** is an entry of `spec` or of `milestone`, and a plan's items, of `thread`,
  `argument`, `criterion` and `acceptance`, are level-three headings under the section of their
  kind.

An id matches `[a-z0-9]+(-[a-z0-9]+)+`: two segments or more.

**A reference** is one backticked span, `<kind>@<anchor>@<id>`. The kind is a register's name, a built-in
one or one the manifest declares, and the reference names one entry of it; or `register`, which names a register's home; or `path`, which
names a file or a directory under the anchor; or `planned`, which names, in a plan document, a
path its work will create. The harness kinds take no anchor: `skill@<name>` and `agent@<name>`,
each with an optional section slug after a second `@`, `primer@<slug>`, and `instructions@<slug>`
for a section of the root `CLAUDE.md`. When a reference is written, and how a `path` reference is
shaped, are in `primer@where-knowledge-goes`.

**Reading**: `{{command}} show <reference>` prints the entry whole and every reference to it;
`{{command}} issues` and `{{command}} tripwires` list the standing entries.

**Running the checker**:

- `{{command}} check` reads every document in four phases (the manifest resolved, the tree read
  against it, the entity table built, then every check) and stops at the first phase that finds
  anything, saying which phases it did not judge: a finding computed over an incomplete model is
  unreliable in both directions. Its exit code is the verdict: 0 for no finding, 1 for findings, 2 when it could not
  run. Its last line states it, `PASSED: no findings` or `FAILED: n findings above`; its summary
  block prints whether or not it found anything, so the middle of its output does not tell whether
  it passed. A finding names its repair.
- `{{command}} check --staged` judges the tree a commit will record, HEAD's with the staged
  changes. Run it after staging and before each commit, and repair a failing one before the
  commit: it catches a file left unstaged, or a change staged in part, before the commit exists,
  where the gates catch it only before the merge.
- `{{command}} check --fix` writes the generated files, such as a register's index, then checks. It
  refuses while a file it would write differs between the staged tree and the working tree: an
  entry's deletion staged with `git rm`, beside an unstaged or untracked change to the same
  register, is the common case. Take one of the two repairs it names: `{{command}} index --staged`
  then `{{command}} check --staged`, to commit the staged part, or `{{command}} index`, to take the
  working tree whole, staged after it.
- `{{command}} commits <range>` judges each commit of a range, its message and its tree: a commit
  message is a document, read long after it is written, and every reference in it must resolve,
  against its own commit's tree or its first parent's. Run `{{command}} commits <main>..HEAD` after
  each commit, where an amend still repairs a failing message.
- Never chain a command on a verdict that went through a pipe: in `check | tail && git commit`, a
  shell without `pipefail` takes the exit status of `tail`, so the commit runs whatever the check
  found, and a filter drops lines in any shell. Run the
  checker bare, or redirect its output to a file, read its last line, and chain nothing on the
  redirection's command.

## Where knowledge goes {{slug:where-knowledge-goes}}

**Every durable decision has exactly one home.** A second mention of a decision is a reference to
it, never a copy, because a copy starts drifting the moment it is written. **A directive is
restated where it has to be delivered only when the restatement is no longer than a pointer to
it**: a path, a file name, a command, a value, or one sentence. A directive sentence carries its
pointer beside it, and where the two disagree the restatement is the defect. A directive longer
than one sentence is delivered by a pointer to its home, with an instruction to read the home whole
at that moment, or by one of its sentences restated with its pointer beside it, where that one
sentence is what the point of delivery needs. More than one sentence of it is never restated: a
partial copy drifts, and its reader takes it for the whole. A whole text that the workflow's own
build copies from one file into several installed texts is no restatement, since no copy can drift
from that file. For installed text, the home a pointer
names is installed text, a skill's section or the primer's, since installed text cites no entry of
the project. An existing longer restatement is converted when a change edits what it says;
re-pointing a reference in it does not count. Until then, its length alone is no defect, whenever
it was written.

{{fragment:context-cost.md}}

| the statement is about | home | it leaves when |
| --- | --- | --- |
| what the project, or one Component, is for, and what would show it achieved | that Component's goals home, `register@*@goal` (the project's root is a Component) | the owner abandons the goal |
| how the project or a Component is built, and why | that Component's design home, `register@*@design` | the design changes: the entry is rewritten in place |
| a design not kept, and why | that Component's `register@*@rejected`, each entry cited by the design heads whose argument covers it | it is reopened, chosen and built; the owner rules it fails every recording test; or no design head cites it any longer and no head of absence is written for it; a reversal moves the reversed decision into it if it meets a recording test of `skill@knowledge-architect-decision-recording` |
| what is outstanding: a defect, an unexplained observation, an open question, missing work | one file in the owning anchor's issue directory, `register@*@issue` in a Component | the issue closes |
| evidence that would flip a recorded decision whose work exists, or that no work implements | the owning Component's tripwires home, `register@*@tripwire` | it fires, its horizon passes, its decision is gone, another entry or a check takes it over, or a condition of the tripwire standard of `skill@knowledge-architect-decision-recording@ruled-tripwires` excludes it |
| a contract or a trap that only a developer needs, true of the code as it stands | the scoped `CLAUDE.md` nearest the code | the contract changes or the trap is removed |
| how a user can use a Component, and what to respect | its `README.md` | the contract changes |
| directions about what to find where in a directory | a `README.md` in that directory | the directory's content changes |
| what a caller must respect to use a type or a function | that item's doc comment | its contract changes |
| why a piece of code is shaped the way it is, and where that holds | an inline comment at that code | that code changes |
| work that is designed and not built: a spec or a milestone | the plans directory, docs/plans/ at the project's root: a spec in docs/plans/specs/, a milestone in docs/plans/milestones/ | the work lands |
| the evidence of one pass of a project audit, and the owner's ruling on each of its items | one file per pass in docs/audit-reports/ at the project's root, as `skill@knowledge-architect-project-audit@record-of-an-audit` says | the commit that carries out its last outcome, or a later commit of the same branch, deletes it |
| the analysis of a retrospective's project file, and the owner's ruling on each of its findings | one file per analysed file in docs/retrospective-reports/ at the project's root, as `skill@knowledge-architect-retrospective-intake@analysis-and-ruling` says | the commit that carries out its last outcome, or a later commit of the same branch, deletes it |
| the order in which the owner wants known work done | docs/roadmap.md at the project's root, optional: each row cites an issue entry or a plan document | a row leaves when its plan document leaves, or when its issue closes with no plan document scheduling the work |
| how to perform an activity | the owning skill | the procedure changes |
| **none of these, nor a row of the project's own** | **ask the owner before writing it anywhere** | the table gains the row |

The project's root `CLAUDE.md` adds its own rows. **A measurement is routed by what it serves**:
the head of the decision it supports, the issue entry it characterises, the tripwire whose
threshold it is, or else the commit message that took it.

**A reference is written wherever the text would have to be revisited if the entry it names
changed**: a design head names the goal it derives from, a decision of another Component it
depends on, and the `rejected` entries its argument covers; an issue names the decision it strains and the goal it threatens, when it does
directly; a guard, a
workaround, a stub or a test that exists because of an issue names it in the comment at the site; a
tripwire names its decision; a commit message
names every entry it opens, closes, reverses or argues from. A reference whose entry's change would
leave the text unaffected is not written. A reference, whose kinds
`primer@checker-model` lists, names the anchor that defines the entry, and it is live wherever it is
prose, a fenced block included; an illustration that must not resolve writes a placeholder in angle
brackets. A span never crosses a line break: a line breaks before the span, not inside it. A skill
or an agent is named by its reference, `skill@<name>` or `agent@<name>`, never by its bare name in
backticks. A `path` reference names the deepest anchor that holds its target, with a plain path
under it, no `..`, no `./` and no leading `/`, and ends with `/` exactly when the target is a
directory; an anchor's own directory is named from the anchor above it, as
`path@<parent-anchor>@<dir>/`. A relative markdown link, in any of markdown's link shapes, an
image `![alt](<path>)` included, is legal in a `README.md` or an `index.md` only, to a target under
its own directory, its path under the same constraints as a `path` reference's; any other pointer is
a backticked reference.
A heading is a line markdown reads as an ATX heading: one to six `#` marks, at most three spaces
in, then a space or a tab and text, with no list or block-quote marker before them; a line
markdown reads as a heading in another shape, as a `---` right under a paragraph is, stops the
check. The checker reads Markdown, and in Rust source the comments and every string
literal that a `let`, a `const` or a `static` does not bind to a name, a function call's argument
being never bound and a macro's body bound like the rest of the value; it reads each commit's
message and tree through `{{command}} commits`. A reference anywhere else is found by grep.

**A finding is repaired in a form the checker judges, never by moving the pointer into plain
text**: the right anchor, `path@elsewhere@<path>` for a path the tree does not hold, such as, in a
commit message, a path later work will create, a placeholder,
or a rewritten sentence. A record of a deliberation (a plan document, a committed file that leaves
once its outcomes are carried out, or a commit message) names by its bare slug in backticks an
entry that the work it records removes, renames or is still to create, and by its bare name in
backticks a skill or an agent that it removes or is still to create. Either takes that form only
where its tree does not hold the entry or the skill, nor, for a commit message, its first parent's:
a name, not a reference, since there is nothing for the checker to judge. A renamed entry's old slug carries beside it the reference to the entry
under its new slug, so a reader reaches the live entry by a checked pointer. A reference or a path that no checked form expresses is
written without backticks only beside a reference to an issue entry of this project that records
the missing form.
A gap of the checker itself gets that entry in this project's own register, since a reference
cannot reach another project. Text that is not in the checker's syntax, such as a commit named by
its subject, a commit of another project or a description in words, is outside this rule.

## Every commit of a branch is judged by the checker at its tip {{slug:branch-commits}}

Where the project's gates run `{{command}} commits` over a branch, as
`skill@knowledge-architect-setup@setup-gates` recommends, that run is the **per-commit gate**: it
judges every commit of the range, its message and its tree, with the checker the branch's tip
runs, and a tree that holds no manifest or has a finding under that checker fails it. **A change
that sets what the tip's checker requires of every tree, such as adding the manifest, moving the
pin, or, in a project that builds the checker, a change to it under which a tree that passed
before can fail, goes in the branch's first commit, with every fix the tree needs to pass it, or
the branch is squashed to one commit before its review. A repair that would leave an earlier
commit failing is folded into the earliest commit it repairs**,
by a history edit with no uncommitted work in the tree, confirmed afterwards to have lost no
content.

## Design heads {{slug:design-heads}}

A design head is an entry of a design home: a level-three heading that states a decision, and its
body. This section holds what every session applies: to tell in advance whether a change bears on a
head or needs one, and to read a head. The rest of what a head holds, and how a head is
shaped, is `skill@knowledge-architect-decision-recording@head-shape`;
`skill@knowledge-architect-decision-recording` holds the procedures of recording.

### What earns a head

{{fragment:head-entry-tests.md}}

### What a head stands on

{{fragment:head-ground.md}}

## The installed skills {{slug:installed-skills}}

%% The opening paragraph serves `goal@agent-skills@installed-text-works-anywhere`. Of two cloud sessions with no
%% user-level instruction, one read sections of skills found by `grep` in place of loading them and wrote into a
%% design home without the decision-recording skill, and the other loaded that skill only after the decision it
%% records: an event-shaped trigger was skipped too, so fixing the design skill's trigger alone does not suffice.
%% A plain directive to read the matching skill was present in both and did not suffice; the paragraph names the act
%% they took instead. The harness lists a plugin's skill beside an installed one, which its second sentence answers,
%% per `design@agent-skills@installed-skill-takes-precedence`.
Before acting at a moment this list names, load its skill, with each skill of the project that says
it adds to it, and follow them whole: a section of one found by a search is not the skill. A skill
from outside the installed set and the project, which the environment lists for the same activity,
never takes their place.

- `skill@knowledge-architect-decision-recording`: before writing into a design home; a design
  decision has been made or reversed.
- `skill@knowledge-architect-issue-tracking`: before diagnosing a problem; parking anything; a
  tripwire fires; work closes an entry.
- `skill@knowledge-architect-design`: a design question has an open solution space; before
  starting a change the owner asks for or proposes, unless a plan document, an outcome the owner
  ruled in an audit, an intake or a review, or a design head or a failing test already states what
  the owner asks for or proposes, or the change decides nothing (a typo, a stale pointer, a
  rewording), to ground it and find out whether it is bounded work, since a request or a proposal of
  the owner does not by itself settle a design;
  whether to keep or change an existing design; a bug trend suggests the design is the problem; a
  discussion is reverting to multiple-choice menus, minimal-solution defaults, or approval-seeking
  closers.
- `skill@knowledge-architect-planning`: a design discussion converged on its full path; a slice of a
  milestone starts or lands; the work of a spec starts or lands; before editing the roadmap.
- `skill@knowledge-architect-review`: before merging to the main branch, or when an
  activity's skill says its work is ready.
- `skill@knowledge-architect-agent-configuration`: before editing a `CLAUDE.md`, a skill or an
  agent; after an install that changes the installed files.
- `skill@knowledge-architect-setup`: the project adopts the workflow, or moves its pin of the
  checker to another version.
- `skill@knowledge-architect-goal-setting`: before writing or editing any goals home; a Component
  has no goal; the owner states or abandons a purpose; a decision conflicts with a goal.
- `skill@knowledge-architect-project-audit`: the owner asks to audit one aspect of the whole
  project; a pin move crosses a version whose Migration entry cites an audit axis; an adopting
  project's existing documents have moved.
- `skill@knowledge-architect-retrospective`: once per session, offered when a branch the session
  worked on merges, a plan document leaves, or the session ends.
- `skill@knowledge-architect-retrospective-intake`: the owner hands the session the project's own
  retrospective files; an outcome of an analysis in docs/retrospective-reports/ is carried out.

%% The section holds one sentence, with its pointer, for each coined term this primer uses and does
%% not define in itself, and for each word the workflow uses in more than one sense; a term coined
%% and used only in other texts is defined in the text that coins it, per `design@agent-skills@term-defined-at-first-load`.
## Terms the workflow coins {{slug:coined-terms}}

Each term below is one this primer uses; where another text holds a term whole, its pointer names
that text.

- **installed text**: every file `{{command}} install-agent-skills` writes, which a project never
  edits by hand, per `skill@knowledge-architect-agent-configuration`.
- **agent-facing text**: every text an agent reads as instructions, the installed text and the
  project's own `CLAUDE.md` files, skills, agents and saved workflows' prompts alike, all held to
  `skill@knowledge-architect-agent-configuration@admission-standard`.
- **workflow**: the workflow is the method these texts describe, while a saved workflow is one
  script the harness runs, as `skill@knowledge-architect-project-audit` runs one per audit axis.
- **kind**: a reference's kind is its first segment, per `primer@checker-model`; an anchor's kind,
  which `{{command}} anchors` prints beside it, says what made it an anchor, such as a Component or a
  location; an issue's kind is its `kind` field, per `skill@knowledge-architect-issue-tracking@issue-entry`.
- **the owner**: the person whose word closes a fork, and every question `primer@question-authority`
  reserves to the owner, per `skill@knowledge-architect-planning@planning-terms`.
- **spec, milestone, slice**: a spec is the plan document of work done in one branch and one pull
  request, and a milestone is work across several, each of its slices one branch and one pull
  request with its own spec, per `skill@knowledge-architect-planning@planning-terms`.
- **bounded work**: a requested change whose main risk is over-building, which the design skill's
  grounding finds and builds on one proposal, per `skill@knowledge-architect-design@bounded-work`.
- **in-change path, full path**: the design skill's two paths, the in-change path keeping the
  deliberation in the commit message of the change under way and the full path handing it to a plan
  document, per `skill@knowledge-architect-design@decision-authority`.
- **thread**: one open question or proposal of a design discussion, the unit that carries a state,
  per `skill@knowledge-architect-design@discussion-overview`.
- **premortem**: the step before a design discussion converges that assumes the design shipped and
  failed and names the most plausible causes, per `skill@knowledge-architect-design@discussion-loop`.
- **pin**: the version of the checker a project runs, moved only by an explicit edit, per
  `skill@knowledge-architect-setup@pin-the-checker`.
- **axis, pass**: a review's axes are the questions of the table of
  `skill@knowledge-architect-review@review-axes`, each sent to a reviewer of its own; an audit's
  axis is one aspect of the whole project that an audit reads, and a pass is one round of the
  audit's method over it, per `skill@knowledge-architect-project-audit@audit-terms`.
- **project file**: the file of a retrospective's findings on the project's own instructions,
  beside the workflow file of its findings on the installed text and the checker, per
  `skill@knowledge-architect-retrospective@two-files`.
- **Migration entry**: an entry of the checker's changelog that names what a project must change in
  its own files when its pin crosses that version, per `skill@knowledge-architect-setup@moving-the-pin`.
- **horizon**: the occasions after which a tripwire that has not fired leaves its home, which its
  `**Watches until:**` line names, per `skill@knowledge-architect-issue-tracking@tripwire-entry`.
