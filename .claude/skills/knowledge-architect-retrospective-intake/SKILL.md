---
name: knowledge-architect-retrospective-intake
description: MUST use when the owner hands this session one or more of the project's own retrospective files to act on — a `<YYYY-MM-DD>-<project>-<subject>.md` file, the findings of a retrospective on the project's own instructions — or asks what to do with the findings of such a file; and while an outcome of an analysis in docs/retrospective-reports/ is carried out, for what the analysis owes until it leaves. It runs on the owner's word. Covers checking each finding against the text as it stands, against the registers and the history, against the goals and the design, looking for a better fix than the one proposed, writing the analysis to a file, deciding each finding's outcome or putting it to the owner where it is the owner's, and the life of the analysis until it leaves. Not for running a retrospective, nor for the retrospective's workflow file, which goes to the workflow's repository, nor for the work an outcome rules, which its route's skill covers.
---

# Retrospective intake

Scope: the analysis of the findings of the project's own retrospective files, the outcome of each,
and the life of the analysis file until it leaves.

Not covered here:
- **running a retrospective**, and sending its workflow file to the workflow's repository:
  `skill@knowledge-architect-retrospective`;
- **the work an outcome rules**: an issue is opened under
  `skill@knowledge-architect-issue-tracking`; a finding handled now is handled under the skill that
  item g of `skill@knowledge-architect-retrospective-intake@what-to-establish` names for it. What
  the analysis file owes meanwhile is `skill@knowledge-architect-retrospective-intake@after-the-ruling`.

**The session decides each finding's outcome as `primer@question-authority` routes it**: it
establishes the facts, argues the action, and decides what the goals, the record, the code and the
sources settle. The owner rules on the rest: most often a goal the finding would change, which goes
to `skill@knowledge-architect-goal-setting`; a fact only the owner holds, such as what a session
would have lacked without an addition about how an agent behaves; two fixes that both survive with
nothing ranking them; and a publication, as the outcome "sent upstream" says.

## What it takes `##what-it-takes`

- **In scope: the project's file of a retrospective**, `<YYYY-MM-DD>-<project>-<subject>.md`, any
  number of them at once. It holds the findings on the project's own instructions (P) and the
  project's side of an interaction finding. What each of the retrospective's two files holds is
  `skill@knowledge-architect-retrospective-intake@report-format`.
- **For grounding only: the workflow file of the same retrospective**,
  `<YYYY-MM-DD>-<project>-<subject>-klarch-workflow.md`, when it is present. An interaction finding
  has an id in each file, each naming the other; the workflow file gives the other side. Nothing of
  it is resolved here: its findings are the workflow's. It is written to only under the outcome
  "sent upstream".

A finding is cited by its id and its file's stem, in the form
`skill@knowledge-architect-retrospective@what-it-examines` gives.

## The retrospective's two files and its kinds of defect `##report-format`

Each finding goes to the file of the project whose text or code must change:

| file | holds |
| --- | --- |
| `<YYYY-MM-DD>-<project>-<subject>.md` | what the session did and the version it used, the findings on the project's own instructions, and the project's side of an interaction |
| `<YYYY-MM-DD>-<project>-<subject>-klarch-workflow.md` | what the session did and the version it used, the findings on the installed files and on the checker, the workflow's side of an interaction, and the answers to the standing questions |

`<subject>` names the session's work in a few words, in lowercase joined by hyphens, such as
`pin-move`. An interaction finding whose fix may fall on either side goes in both files, each
written from its own side.

An item is **unclear** when it could be read two ways, **missing** when the session could not
follow the workflow, or could not produce a document the checker accepts, without an instruction
that does not exist, and **wrong** when following it produced a defect or a correction. A decision
the session made by its own judgement where no instruction covers it is not missing: the installed
text leaves that room on purpose, per `primer@room-to-judge`.

## Ground before judging any finding `##ground-before-judging`

1. **Read each file whole.** The session usually did not write it, so it knows of the reported
   session only what the file says.
2. **Note the version each file used**, from its version line: the main commit the session's tree
   contained, and the checker's version. A file whose version line names no commit is dated as
   item b of `skill@knowledge-architect-retrospective-intake@what-to-establish` says. Note the main commit you analyse against,
   `git rev-parse --short <main branch>`. A finding against an older commit or an older version of
   the checker may already be repaired.
3. **Search the whole record with the received files as the work.** One search covers every file
   received together. Dispatch it as the description of `agent@knowledge-architect-record-searcher`
   says, with the files' paths and their subjects as the work, and as the seeds the design heads and
   goals the findings name, or none. A proposed fix on a subject the files did not raise sends the
   search again, with the fix as the work, before its outcome is decided or put to the owner.

   Read whole, in `bears.md` and `may-bear.md`, which the search's check writes, every issue and
   tripwire the search grades `bears` or `may-bear`, every other entry it grades `bears`, and an
   entry graded `may-bear` where a proposal touches its subject, chosen from the table of contents of
   `may-bear.md`; and, with `cargo klarch show <ref>`, every other entry a proposal rests on or would
   change, and each goal the two files give by its grades alone. Never act on an entry from
   its reason line. The analysis lists each other entry graded `may-bear`, with the searcher's
   reason, as not read: a batch of files brings more such entries than a session can read whole and
   still judge. The search finds what a grep per finding
   does not: a finding already recorded, a deferred issue whose trigger the file meets, a tripwire
   whose firing evidence the file carries, a design issue whose re-entry it meets, and a head or a
   goal a proposed fix would contradict or strain.
4. **Cluster the findings.** One defect is often reported in several files, or as two findings of
   one file. A cluster, and a standing entry that several files meet, is analysed once, in the
   analysis of the first file by stem that holds it, and cites every id in it; the analysis of each
   other file holds a section that points there. That a
   finding recurs across sessions is evidence: say how many times.

## What to establish for each finding or cluster `##what-to-establish`

The order is a suggestion; the content is not.

- **a. What it says.** What happened in the session, which text it blames, quoted, and the fix it
  proposes.
- **b. Whether it still applies.** Read the blamed text at the main commit the version line names,
  `git show <commit>:<path>`, then on the main branch as it stands, which holds the reported
  session's branch once it merged, and quote it with its `file:line`. A version line whose commit
  the repository does not hold is read at the commit its `git describe` count gives on the main
  branch, and a file whose version line names no commit is matched by `git log -S '<phrase>'` on
  the instruction it quotes. The report is a claim about the text at its
  version, so it is checked like any claim about the code. A finding may be already repaired: name
  the commit, found with `git log -S '<phrase>'` or `git log --grep`. A report's own "repaired in
  the session" is a claim like any other, and is often wrong: find when the credited wording
  entered, with `git log -L` or `git log -G` on it, and compare with the session's work. A phrase
  searched in a commit message or in wrapped prose is matched across a line break.
- **c. Whether it is already recorded.** An issue or a tripwire, a design head, or a rejected
  design, from the search of `skill@knowledge-architect-retrospective-intake@ground-before-judging`. A proposed fix that already lost is not proposed again: an alternative counts
  as argued and lost only when a recorded reason covers it, per the record test of
  `skill@knowledge-architect-design@threads-and-states`.
- **d. Its kind.**
  - **a defect of the text**: unclear, missing or wrong, as `skill@knowledge-architect-retrospective-intake@report-format` defines the three;
  - **a lapse of the session**: the instruction exists, is clear, and was not followed. What
    repairs it is the lapse rule of `skill@knowledge-architect-retrospective-intake@admission-rules`;
  - **about the owner's behaviour under an installed skill**: judged against that skill's
    expectation set, in `skill@knowledge-architect-retrospective@expectation-sets`, read whole at
    this check. Behaviour the set says the skill assumes otherwise is outside that skill's scope.
    An installed skill with no set has no such test yet, and a finding on the project's own skill
    has none. **A finding that two installed instructions leave no move satisfying both is always
    in scope**, and is never closed by citing an expectation set;
  - **an observation**: a use that went well, or a behaviour not reproduced. It needs no action.
- **e. Its scope.** The goal it serves or threatens, from the project's goals homes,
  `register@*@goal`; a finding on the installed text or on the checker has its scope in the
  installed text or the checker, and no goal of the project's. The
  owner of the fix: the Component or the location that owns the blamed text; or the installed
  text, which the project never edits, per
  `skill@knowledge-architect-agent-configuration@installed-files-never-edited`; or the checker.
  Whether the fix reverses a recorded decision, per
  `skill@knowledge-architect-decision-recording@reversal-check`: if it does, the fix is a reversal
  and goes through that skill. The record check of item f names the heads it would reverse.
- **f. A better fix.** Name the nearest rival to the fix the report proposes, and the fact that
  decides between them. Rivals worth looking for:
  - the same defect fixed at another site: the checker rather than the text, the installed text
    rather than the project's, or the reverse;
  - one fix for a whole cluster;
  - a fix where the instruction is delivered, rather than more instruction;
  - no change.

  The fix is shaped, and read against the record, as `skill@knowledge-architect-retrospective-intake@admission-rules` says: it is the
  standard of every actor that writes or judges agent-facing text.
- **g. Its route.** What handling it needs, which says what "now" would mean:
  - a text edit of the project's own agent configuration whose design is settled:
    `skill@knowledge-architect-agent-configuration`;
  - a decision, or one whose design is not settled: `skill@knowledge-architect-design`;
  - a change to the project's code: the project's own development procedure;
  - an issue only, when the work needs a branch of its own;
  - a fix in the installed text or in the checker: sent upstream;
  - nothing.
- **h. The proposal.** The action, its argument, its default outcome, and who holds it: the
  session, or the owner, with the kind of question that makes it the owner's
  (`skill@knowledge-architect-retrospective-intake@analysis-and-ruling`).

**Every verdict carries its evidence**: a quotation with its file and line, a command and its
output, a commit. A verdict without evidence is labelled an assumption.

## The standard on agent-facing text `##admission-rules`

The four parts of `skill@knowledge-architect-agent-configuration@admission-standard`, with the
rule on a text's size cost that the standard places after the shape of a repair.

### The need test

Every statement in agent-facing text passes one test: **name the act of an agent following the
workflow that would differ without it**: what the agent does, when, where, or a fact it must use.
A statement for which no such act can be named is not written, and an existing one leaves as the
table below gives. The
unit is a sentence, or a clause when only part of a sentence fails.

- **Whose act.** Any agent that reads the text in its work counts, an agent that maintains the text
  or writes a brief from it included. A statement whose only reader is the text's maintainer is at
  most one sentence; in installed text it is a comment of the workflow's source, removed by the
  build, since no installing project maintains the installed text.
- **A reason** passes when it states the mechanism an instruction relies on or the failure it
  prevents, even when the instruction has no exception. A second reason for the same instruction
  passes only when it names another mechanism or failure. A sentence that narrates where an
  instruction came from or how it turned out fails: "learned by getting them wrong", "and it
  worked".
- **A duplicate inside one text fails while its twin stays.** A text is read whole once it is
  loaded, so a second statement of the same act adds none. The finding names the copy that stays:
  the one in the section whose subject it is. The primer and the root `CLAUDE.md` that imports it
  are one text, since both are loaded together, and the primer's copy is the one that stays. Across
  other texts, `primer@where-knowledge-goes` decides.
- **A lead, a transition or an emphasis** fails when the statements it introduces or stresses
  already carry every act. A heading is structure, not a statement.
- **A remark about the text itself** fails unless it tells a reader where to look or what to edit:
  what the text is classified as, or that something is not restated here.
- **A definition** passes when an instruction anywhere in the workflow uses the term. **An example**
  passes when it shows how a definition or an instruction applies.
- **A pointer is needed** when its target is needed to act, or when it names the home of a
  restatement.
- **A statement that can turn false with no change to the project** (the state of another project,
  a tool's version, a person's plan) is suspect: it is written only when an act needs it. "This
  issue was reported upstream" fails, since no act differs.

Wording and style follow `primer@working-prose`; placement across files is judged by other rules.

Current reality only: no history, no dates, no commit hashes, no session records. Examples are kept
minimal.

**The outcomes of a need-test failure.**

| the failing statement | outcome |
| --- | --- |
| it carries no instruction: narration, a remark about the text, outside state no act uses, a duplicate whose twin stays, a lead or an emphasis whose statements carry every act | deleted in the branch: by the session, which applies an audit's outcomes, or as a review's repair |
| it is maintainer-only, in installed text | moved to a comment of the workflow's source |
| it is maintainer-only, in a project's own text, and longer than one sentence | cut to one sentence |
| it is an instruction whose act no agent performs | a decision, since deleting it changes what agents are told, and whether its act is still wanted is the intent of whoever wrote it, which the project's record may not hold |
| a suspect statement that passes | no finding; a suspect statement found false is a factual error |

In a project that installs the workflow, a finding on installed text, of any row, is sent upstream,
as every finding on installed text of an audit is.

### Evidence and kind

**A defect provable by reading needs no other evidence**: a contradiction, a broken trigger, a
factual error, an instruction whose words allow two acts, a provable gap, divergent verdicts that a
review standard and an audit standard give over one text, a statement that fails the need test.

**An instruction about how an agent behaves is added only on evidence from real use**, in the
installed text and in a project's own text alike: a behaviour seen in a real session, produced
unprompted or asked for by the owner, with the owner naming what the session would have lacked
without it, and one sentence stating the mechanism that produced the need. A gap that only
predicts what an agent would do is no edit: it is an issue entry stating what a real session would
have to show. A provable gap is an input an act needs that no text supplies and judgement cannot
derive, a fact fixed elsewhere such as a format the checker enforces, a name or a path; its fill
adds an instruction, which that fact determines.

**A lapse is not repaired by adding text.** An instruction that exists, is clear, and was not
followed stays as it is. Where the lapse recurs at one point of delivery, because the instruction
is read at one moment and needed at another, the fix moves where it is delivered, and the
recurrence is its evidence.

**The kind of a gap weighs it.** A missing capability, a move the agent does not have, is worth
text. A conformance rule, for a move the agent has and might not make, is worth text only where the
default behaviour is systematically wrong. A structure imposed on the work, a fixed sequence, count
or mapping between a unit of the work and a unit of its record, only where the work needs it; a
contradiction a structure causes is answered by removing structure. For an installed skill, a
finding about the owner's behaviour outside the skill's expectation set,
`skill@knowledge-architect-retrospective@expectation-sets`, is no gap; a finding that two installed
instructions leave no move satisfying both is always in scope. An installed agent has no expectation set, since it never works with the owner.

### The shape of a repair

**A finding is repaired at the cause that produced it, not at its site**: a repair written for one
finding adds a rule, and added rules interact at the next pass. A cause is one of four kinds:

- **two homes for one moment**: instructions about one moment or one kind of act stated in more
  than one text;
- **a structure the work did not need**: a fixed sequence, count or mapping that another
  instruction needs to vary;
- **a restatement that drifted from its home**;
- **a rule written for one interaction**: a narrow rule where judgement already decides.

A finding that fits none is a cause of its own, with no kind. A repair is the homes merged into one
and the others pointing to it, the structure removed, the restatement brought back to a pointer, or
the narrow rule deleted and the case left to judgement. It adds an instruction only where none of
these closes the cause. **An instruction** is a sentence that tells an agent to do, or not to do,
an act at a moment; a rewording that makes an existing instruction's intent clearer adds none,
whatever its length. Each repair counts the instructions it adds and removes, and names the nearest
rival to itself and the fact that decides between them. **When the repairs applied together add
more instructions than they remove, the owner is told the net figure**, and the record that applies
them states it.

**The size cost of agent-facing text is counted in the context of the sessions that load it, never
in the text's length.** A statement a session applies to its act costs that session nothing when
the text it loads delivers it, since the session holds it either way. A pointer the session must
follow costs more: the reference resolved, the location found and the target read, each in context
and in time, and the risk that the session acts from memory instead. A size cost exists only for a
session that loads a statement its act does not apply: the primer's text, in every session that
never meets its occasion, or the copy of a text that the build places in several texts, in a text
whose act does not apply it whole.

### The record check

**An edit that changes what an agent is told to do is a decision.** Before it is written, and before
the owner rules on a proposed repair, it is read against the record: every goals home; the design
heads that `cargo klarch show` and a search of the design homes return for the behaviour the edited
text describes; the rejected designs of each Component that owns an edited text; and the
scoped `CLAUDE.md` files and code comments that `skill@knowledge-architect-decision-recording@reversal-check`
searches. The check names each head the edit would reverse, each rejected design it brings
back and each goal it strains, with its quotation. An edit that writes into a design home, or edits
a text whose behaviour a head describes, loads `skill@knowledge-architect-decision-recording` before
it is written; that skill, with `primer@design-heads`, judges whether it contradicts a head, adds a
member its argument does not cover, or earns text at all. One that strains a goal goes to the
owner, under `skill@knowledge-architect-goal-setting`. A rewording that changes no instruction is
not a decision.

## Subagents `##intake-subagents`

The checks a to c read text, and fresh subagents may run them, one per file or per cluster. Brief
each with the findings, the files to read, the entries the search of
`skill@knowledge-architect-retrospective-intake@ground-before-judging` returned, and a scratch
directory of its own, never shared. The agent checks c against those entries, and does not
search the registers again. Each returns, per finding: the quotation,
its file and line, and its verdict. The session itself establishes d to h, and re-checks every
verdict a proposal rests on. No subagent replays the reported session or builds a scenario of agent
behaviour.

## The analysis file and who holds each outcome `##analysis-and-ruling`

Write the analysis in docs/retrospective-reports/ at the project's root, as `<stem>.md`, where
`<stem>` is the received file's stem.

- A head: the received file's name, the version it used, the main commit analysed against, and the
  standing entries the search graded `bears` or `may-bear` that bear on it.
- **One section per finding or cluster**, headed by its ids and a short title, with a bullet per
  item of `skill@knowledge-architect-retrospective-intake@what-to-establish`, in prose. No table:
  the content of each item is too long to read in a cell. A finding that item b shows repaired, or
  item c shows recorded, takes only the bullets that establish it.
- **Every section ends with a `Default` line and an `Outcome` line**, whatever it proposes, no
  change included: a finding, a noted item, and a judgement on a standing entry. A standing entry
  the file may meet gets a section of its own: a tripwire that may have fired, a deferred trigger, a
  design issue's re-entry; whether it fired, and its response, are what its entry and the tripwire
  standard settle. The `Outcome` line holds one of:
  - `decided by the session: <outcome>. Ground: <the argument, with its references>`;
  - `for the owner (<the kind of question>): pending`, then, once the owner rules,
    `for the owner (<the kind of question>): <the choice>. The owner: "<the words, verbatim>"`.

  A section that points to a cluster analysed in another file has neither line: the outcome is
  given where the cluster is analysed, once, so that it cannot be given twice and differ.
- **A span the checker would refuse is quoted in words**, and an entry an outcome removes, renames
  or creates is named as `primer@where-knowledge-goes` says. A finding about a refused form tempts a
  quotation of that form, and the analysis then fails the check it reports on. The project's paths
  and entries are written in the checked forms, since the file is checked like any other.

Then give the owner the full path of each file, and in the conversation one line per section: its
ids, its kind of item d, the proposal in a few words, and who holds the outcome, "decided: <outcome>" or "for you:
<kind>, default <outcome>", or, for a section that points to a cluster, the file that analyses it.
The owner can add a fact, an argument or an intent to any
section, a decided one included, in the file or in the conversation, where the session copies it
into the section verbatim. The session weighs it as any other, decides the outcome again with the
new ground, or makes the section an owner item where the addition opens a question that is the
owner's. **For an owner item, propose that the owner rules by writing its `Outcome` line in the
file**: a ruling written beside its evidence is easier to give than one in the conversation, and it
lands in the record as written; a ruling given in the conversation is copied in verbatim. **Four
outcomes**:

- **handle now**: the finding's work lands in the branch of the intake, its design settled, or
  settled there on the in-change path of `skill@knowledge-architect-design`; it is handled once
  decided, on that branch, under the skill its route names. It needs no issue
  entry, and the proposal does not suggest one;
- **open an issue**: the work needs a branch of its own; name its anchor and its kind;
- **sent upstream**: the fix lies in the installed text or in the checker. Ask the owner whether
  the workflow file of the same retrospective was sent, and how, a fact only the owner holds:
  - not sent yet: the outcome is the session's; add the finding to the file, as a W finding for the
    installed text or a C finding for the checker, with the next number of its letter in that
    file, the two ids each naming the other, as an interaction finding's do. The owner reads the
    whole file before it leaves;
  - sent: the outcome is `for the owner (an action that cannot be undone)`. Write the text: for an
    issue on the workflow's repository, which
    `skill@knowledge-architect-retrospective@what-becomes-of-files` names, the edit of its body and
    a one-line comment naming the added finding, so that those who follow the issue see it; for a
    file sent with no issue, or an issue that cannot be edited, a new issue for the finding, as the
    retrospective opens one. Run `gh issue edit <n> --repo <that repository> --body-file <file>`,
    `gh issue comment <n> --repo <that repository>` or the new issue's command only on the owner's
    word. Where `gh` is absent or fails, give the owner the text and the address. The outcome is
    carried out once it is sent, which the message of the commit that carries out the analysis's
    last outcome says;
- **no change, with the reason**: a lapse, already repaired, out of scope, or an observation.

A question tool is not the way to collect a ruling: it shows a label, not the evidence and the
rival fix the ruling depends on.

## After the analysis is written `##after-the-ruling`

The analysis is committed as soon as it is written, under the project's own branch rules, owner
items pending; the message lists each outcome the session decided, one line each, and the message of
a later commit each outcome it decides or changes. The work proceeds on every default, a decided
outcome and an owner item's alike, except a part that rests on a goal to change, a contradiction or
an action that cannot be undone, which waits for the owner's word. An owner item's outcome counts as
carried out only once it is ruled, so the analysis cannot leave while one is pending.
**Before the branch that holds the analysis merges, list every `Outcome` line still pending**, with
`grep -rn -E 'Outcome.*for the owner \(.*\): pending' docs/retrospective-reports/`, and ask the
owner for those; the branch merges once none is pending.

The issues the outcomes open are opened, and the findings handled now are handled, in the commits
their routes need. The commit that carries out the last outcome, or a later commit of the same
branch, deletes the analysis, and its message names it by its path. History keeps it:
`git log -- docs/retrospective-reports/` lists every analysis.

Then mark each received project file the analysis carried out: a line at its head saying that it
was analysed and every outcome carried out, and that `git log -- docs/retrospective-reports/<stem>.md`
in the project finds the analysis. Where the owner's instructions name a place for handled files,
move it there; otherwise leave it where it lies. Never delete one. A marked file is not handed to an
intake again.

A finding handled now can turn out to need a design discussion and a plan. Its `Outcome` line is
then rewritten to name the plan document or the issue that now carries it, so the analysis can
still leave.

A file whose cluster other files point to leaves after them, or the commit that deletes it rewrites
the pointing sections, since their references would dangle.
