# Agent skills design: The shipped set

## The shipped set

### content/ mirrors the install layout, and the build generates the list `##content-mirrors-the-install-layout`

The directory `path@agent-skills@content/` holds what `install-agent-skills` writes, and nothing
else, except that a line `{{snippet:<file>}}` stands for a file of `path@agent-skills@snippets/`,
which the build inlines there, a line `{{fragment:<file>}}` stands for a file of
`path@agent-skills@fragments/`, which the build copies there indented to the line, per
`design@agent-skills@shared-text-fragment`, a `%%` line is a comment the build removes, per
`design@agent-skills@shipped-text-line-comments`, a delivery placeholder is a literal the build
fills, and a placeholder `{{slug:<id>}}` ending a level-two heading is rendered into that heading's
section slug, so content/ defines no section in this repository's walk and the installed copy does,
per `design@core@installed-entities-from-the-tree`. A slug placeholder anywhere else fails the
build. A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, a saved
workflow of Claude Code's Workflow tool is `workflows/<workflow>.js`, and the primer is `PRIMER.md`;
the build asserts a saved workflow's name, per
`design@agent-skills@saved-workflow-name-is-its-file-stem`. The build script,
`path@agent-skills@build.rs`, walks the directory, renders each file, removing its comments, filling
its delivery substitutions and inlining its snippets, and generates `FILES`, each entry the install
path and the text. It adds the installer's prefix to each skill directory, agent file and workflow
file on the way out, so the installed names are the namespace of
`design@core@owned-namespace-check`. A file the layout does not map fails the build.

The list is generated because a list written by hand drifts from the directory it lists, and the
drift ships: a skill missing from the list is never installed, and nothing reports it. The core's
test `every_shipped_file_is_installed_in_the_owned_namespace` holds the build script to the
namespace, since an install path outside it would write a file the project owns.

### A text several installed texts apply whole is one file the build copies into each of them `##shared-text-fragment`

A standard, or a similar large block of coherent instructions, that several installed texts apply
whole is a file of `path@agent-skills@fragments/`, and each text that applies it names it by a line
`{{fragment:<file>}}`, indented to sit where the host needs it. The build copies it there after the
host's comment and slug passes, so that their failures name the host's own lines, removing the
fragment's own `%%` lines; a fragment holds no heading slug, no other fragment and no build
placeholder but the install's `{{command}}`. The pass is `render_fragments` of
`path@agent-skills@src/render.rs`. A text is a host only where its act applies the whole fragment,
and the test `each_fragment_has_its_hosts` of `path@agent-skills@src/lib.rs` holds the list.

The owner proposed it, against the pointer read whole, so that a standard is written once and still
delivered whole wherever it applies, which `goal@agent-skills@no-contradiction-in-workflow` asks:
a pointer costs the session a decision to fetch it, which the owner believed sessions likely to
skip on the ground that they read the target earlier. The owner ruled that fragments become the main
delivery for standards and similar large blocks read whole at several places. The fragments
today are the tripwire standard and its lifecycle, the rules on design heads, per
`design@agent-skills@one-home-for-head-rules`, the admission standard, per
`design@agent-skills@admission-rules-home`, the retrospective's file format and kinds of defect,
the decision-recording skill's rules on the Component, rejected designs, the deliberation and a
reversal, the audit's outcomes, the rule that a skill is self-sufficient, and the trigger test; and
one rule written as a fragment from the start, the size cost of agent-facing text, per
`design@agent-skills@context-cost-of-agent-text`. The host lists of every fragment but the four of
the admission standard are the owner's: the owner approved each list, the narrowings included, so
a host added to one goes to the owner; the admission standard's four follow from the owner's
approval of its split by subsection, under the rule that a host applies the whole fragment. The
expectation sets stay behind a pointer, since the reader needs one skill's set, which varies per
finding. The pointer it replaces had left the writer of about 35 tripwires of
this repository and thaum without the rule they broke. The snippet pass cannot carry prose: it runs
last, so a snippet keeps its `%%` lines and its placeholders, and every snippet is written into a
new project by `klarch init --rust`. A harness hook delivering pointed sections would bind the
shipped workflow to one harness, against `goal@agent-skills@installed-text-works-anywhere`. A
review of a changed fragment reads it in every host, since a sentence true in one host's context
can be false in another's, as two sentences of the first fragments were.

### The setup skill's blocks for a Rust project are snippets that a test checks and `klarch init --rust` writes `##rust-templates-one-home`

Each block `skill@knowledge-architect-setup@rust-project` shows is a file of
`path@agent-skills@snippets/`, inlined by the build, and exposed whole as `SNIPPETS`: the
workspace's Cargo.toml, the maintenance crate's Cargo.toml, the cargo aliases with the `[env]` tie,
the crate's main and the CI workflow. `klarch init --rust` writes all of them but the CI workflow,
per `design@core@rust-scaffold` and `design@core@init-writes-no-ci-workflow`, so a project gets the text the skill shows. Each is checked by a
test of the workspace: the main is compiled by an example of the maintenance crate, the three TOML
files by a test that builds a project `init --rust` wrote and runs its checker, and the CI
workflow's gates command by a test that parses it with the gates library's flags. A block copied
into a template of the core would be one more text that drifts from the skill, and a block in the
skill alone is checked by nothing, so a change of the gates library or of the checker's command
line could leave it wrong while every gate passes.

### A saved workflow's `meta` name equals its installed file's stem, which the build asserts `##saved-workflow-name-is-its-file-stem`

The harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by
the name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which
the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
installer's namespace" records. So the build fails unless that name equals the installed file's
stem, or the prefix the build adds to the file would namespace nothing, per
`design@core@owned-namespace-check`.

### The shipped text carries comments for this repository, `%%` lines the build removes `##shipped-text-line-comments`

A line of a file of content/ whose first two characters are `%%` is a comment for this
repository's maintainers. It sits beside the instruction it explains, and it may cite design heads
and issues, per `design@agent-skills@content-is-in-the-walk`. The build removes it whole, so no
installed file holds it; a `%%` line inside a fenced block, or one with leading spaces, fails the
build rather than ship. The crate's published source holds the comment lines as written, with their
citations of this repository: no installing project reads that source, only the text the install
writes. It gives the installed text what a comment gives code, the reason at the
site it explains, per `design@agent-skills@local-intent-binds`, so a decision about one instruction
need not take a head, per `design@agent-skills@a-head-is-owed-by-an-entry-test`. The owner's
argument: it "would solve in good part the problem where the design heads of the workflow get
bloated with details that, in a code project, would be served well by a local code comment. It
would also allow to cite the design heads or other items without polluting installed skill
results."

Line comments, not HTML comments: the checker treats an HTML comment as parked text and reads no
reference in it, so a citation there would go unchecked, and the owner judged line comments "saner
for the long term". The marker `%%` has no Markdown meaning, reads as a comment in Mermaid and
Obsidian, and cannot collide with a Rust snippet, whose comments are `//`.

### content/ is in the walk, so the references of its comments are checked against this repository `##content-is-in-the-walk`

The `%%` comments of `design@agent-skills@shipped-text-line-comments` are checked
by putting content/ back in the walk, rather than leaving them unchecked or adding a test in this
repository that would repeat the checker's reference reading. A declaration in the published
checker would cater to this repository's use in what consumers receive. That extends to the checker
what the owner said of the installed files: "I don't want to cater too much to this use case in the
installed files"; the extension is the session's argument, which the owner's choice of content/ in
the walk followed.

The design this decision did not keep:
`rejected@agent-skills@published-checker-reads-percent-lines`.

### The shipped text cites only what every installing project holds, and so no entry of this repository `##shipped-text-cites-no-entry`

The text under content/, with the snippets the build inlines, is read in every project that installs
it, so it names no Component and no convention of this repository, per
`goal@agent-skills@installed-text-works-anywhere`. The one name it holds on purpose is the
workflow's own upstream repository, where a retrospective's findings on the workflow go, per
`design@agent-skills@retrospective-destination`. It cites no entry: a reference to an entry resolves
only against the tree it stands in, and no tree but this one holds this repository's entries. A path
or a home that every conforming project holds may be written as a reference:
`path@*@README.md` names each Component's own copy of a required document, per
`design@core@components-carry-the-same-documents`; `register@*@goal` names each Component's own
goals home, in whichever shape it has, per `design@core@register-home-reference`; and
`path@plans@README.md` and `register@plans@spec` name a file and a home of the one plans directory,
whose path the checker fixes, per `design@core@plans-dir-fixed`. Each is true in every installing
project, and it shows the syntax the checker enforces there. Writing a per-Component path in that
syntax is the owner's proposal: "it promotes the same kind of syntax as what the checker enforces
in projects holding the installed skills". The same argument holds of the plans directory's paths
and homes, since the checker fixes that directory's path, and of a home named by its register,
since the checker refuses a home named by its path. A path that varies by project, and an
illustration, are placeholders in angle brackets. The command a project runs is written as the placeholder that the install fills
with the project's declared command, per `design@core@declared-command`.

**The shipped set may cite its own skills, agents and primer, and their sections**, as
`skill@<name>@<slug>` or `primer@<slug>`: every project serving `claude` holds them, installed, and
they are defined from its installed copies, per `design@core@installed-entities-from-the-tree`. A
test of the core holds every such reference to resolve within the shipped set. It never cites
`instructions@<slug>`, whose slugs are each project's own.

content/ and the snippets are in the walk, so every reference they hold is checked against this
repository. A reference to an entry that resolves here passes the check and would dangle in every
installing project: the reviews of each change, and the release's hand check over the installed
copies, judge that, until a mechanical check exists, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` tracks.

### An installed skill or agent names only the installed set's skills and agents, and the project's own `##no-external-handoff`

An installed skill or agent hands off only to another skill or agent of the set its version ships,
or to the project's own skills. It never names a plugin or a skill from outside the set. A skill
outside the set may be absent in the project, or present at a version that contradicts the
installed one, and either breaks `goal@knowledge-architect@agents-get-a-complete-workflow`, under
which no two parts of the workflow contradict each other.

The set is written across several changes before a version is released, so a skill may name a
member of the set by its installed name before that member lands. No release ships a name its set
does not hold.

### A skill is named by its activity, as a noun of one or two words `##naming-rule`

An installed skill is named by the activity it covers, as an activity noun in common usage, in one
or two words after the installer's prefix: planning, review, issue-tracking, decision-recording.
The unit is the activity because `goal@agent-skills@one-skill-per-activity` gives each activity of
the workflow one skill.
It is never named as an artifact the activity writes, because the text names both: a skill called
"decision-record" would share its name with the design homes and rejected alternatives the
design skill calls the decision record, and one called "open-issues" with the issue directory. One
exception holds: agent-configuration is named like the configuration it edits. Its nearest rival,
agent-setup, reads like the setup skill, and the installer's prefix makes the full name of
`skill@knowledge-architect-agent-configuration` read as a skill; the cost is that the name is also the
artifact's. A name built on a verb form, such as discussing-design-decisions, reads as a sentence
and grows long. That argument is the owner's: "naming an "action" makes the skill names into full
sentences, which is weird and often too long". The rule binds a project's own skills too, through
the agent-configuration skill.

### Bounded work takes a path inside the design skill, after its grounding `##bounded-path-in-design`

Whether a request is a design question or bounded work, a clear requirement whose main risk is
over-building, is known only after the grounding of the design skill's loop step 1, its search for
standing entries included. The work is bounded when it reverses no recorded decision, every
decision it makes fails the entry tests, per `design@agent-skills@a-head-is-owed-by-an-entry-test`,
and no second defensible shape survives the nearest-rival test. The skill then sends one message,
the proposal with its nearest rival, the strongest open reading set aside, the consequences and the
default, which it builds, per `design@agent-skills@derived-decision-proceeds-on-default`; an
argument in the reply returns the work to the loop. The commit that implements it carries the
proposal and, if one came, the owner's answer, attributed, per
`design@agent-skills@attribution-replaces-quotation`. The skill's
description reaches a session before it starts a requested change whose design is not settled.

The owner argued that whether a task is bounded work, which does not change the project's design,
cannot be determined without going through the design skill's grounding steps. The
grounding gives bounded work the search for standing entries that no step gave it, and applying the
entry tests at classification keeps a head from being written for a decision that earns none. The nearest rival, a separate installed skill for
bounded problems, would repeat the grounding to classify at all; it is in the rejected
alternatives. A bounded path names no skill outside the installed set, per
`design@agent-skills@no-external-handoff`.

The design this decision did not keep: `rejected@agent-skills@separate-bounded-problem-skill`.

### The design skill is written for frontier-tier models only `##frontier-tier-only`

The design skill is not simplified for smaller models. A smaller model takes part in the
workflow as an implementer of what the owner and a frontier model decided, not as the owner's
counterpart in the discussion. So "a smaller model would not follow this" is not an argument for or
against any wording of the skill. Its description states "Requires a frontier-tier model
(Opus-class or stronger)": a model below that bar produces the format without the discipline, and
the description is the only text an installer reads before the first run. The evidence is a
scripted four-turn discussion run on three models, described
in `path@agent-skills@README.md`; it is not re-taken, since
`design@agent-skills@synthetic-evidence-not-built` builds no such run, and it stands for the part a
reading of its transcripts confirms: the smaller one reproduced the ledger's format, and it endorsed a weak proposal, invented states outside
the closed set, and dropped open threads between rounds.

The design this decision did not keep: `rejected@agent-skills@design-skill-for-weaker-models`.

### The design skill structures how a discussion is conducted, never what or how much is proposed `##structure-the-flow`

The skill fixes the flow: proposals are argued, threads carry states, a fork waits for the owner's
word and a thread with one shape standing closes on its argument, per
`design@agent-skills@owner-rules-at-a-fork`. It never fixes what may be proposed. The failure it was written against is the
solution quota: a mode that asks for two or three options gets two or three, and where one shape
applies, the others are fabricated to fill the count. A fabricated alternative makes a real
proposal appear to have won a contest that never happened, and spends the round that could have
tested it. The instructions that implement this are "never pad with alternatives to reach a count"
and the nearest-rival test, under which "no rival worth naming" is a claim to test. An edit that
specifies what must be proposed, rather than how a proposal is argued and tracked, works against
this entry.

No bound is set on how much is proposed either: on the number of threads, of proposals, or of
rounds. The ground is the owner's: the owner ruled that a bound on what and how much the design
skill may propose must not be added, since it would go against the core idea of the open design
discussion the skill tries to encourage. Answering a proposed path that would bound a discussion to
one decision met during a task, the owner argued, as the owner's opinion, that a discussion that
starts with one decision should be bounded in neither threads nor rounds, since its size depends
only on what grounding, investigation and experiment bring, which nobody knows in advance. A bound fixed before the discussion decides its size
before its content. A path chosen by the cost of reversal, or by where the work lands, changes where the
deliberation is kept, never how large the discussion may grow.

### A decision met in any activity goes to the design skill only when it creates a head, contradicts one, or adds a member its argument does not cover `##new-or-reshaped-head-needs-design`

A decision goes through the design skill, whatever activity met it, when it creates a design head,
contradicts a statement of one, its argument included, or adds a member the head's argument does
not cover. An addition that answers another question than the head's takes a head of its own, per
`design@agent-skills@one-decision-per-head`. A member the head's argument covers is recorded
directly, routed per `design@agent-skills@extension-follows-the-ground`; an approval the owner gave
it is recorded in the commit, attributed to the owner, not in the head. A change that relocates or rewords recorded decisions, a split of a head included, and adds or
removes none, is not a decision and needs no design skill; the routing and fidelity-of-relocation
review axes judge that it adds or removes none. A choice that earns no durable text stays in a code
comment and a commit message.

The decision-recording skill triggers once a decision has been made, or at the latest when its
text is about to be written, so a new decision met during an issue fix or a review repair otherwise
reaches the record with no argument, no rival weighed,
and none of the design skill's rules for reading the owner's word. This serves
`goal@knowledge-architect@the-owner-decides` and
`goal@knowledge-architect@design-is-recorded-with-its-arguments`. A member inside a head's argument
carries less risk: the decision the head records, and its argument, stand, and the title still
tells a reader of the outline what is decided. The rival, every decision that earns durable
text through the design skill, puts such an addition, and every relocation, to a full discussion
whose outcome the head already records. A test on the size of a change lost: a one-sentence
reversal is the change that most needs the discussion.

Three texts deliver it. The design skill's description names the symptom: it fires while a
decision is still being settled in the conversation, before any write. The backstop, which sends a
decision in one of the three cases, not argued, back to the design skill before its text is
written, is part of the fragment on what earns a head: the primer hosts it, so it reaches every
session at every reading of a head, and the decision-recording skill, loaded before any write into a
design home per `design@agent-skills@design-home-write-loads-recording`, hosts it at the moment of
writing, per `design@agent-skills@one-home-for-head-rules`.
`tripwire@agent-skills@head-created-without-deliberation` watches whether they reach a session in
time.

The design this decision did not keep: `rejected@agent-skills@every-decision-through-design-skill`.

### A head's title states the rule that decided, and its body names the members built `##title-states-the-rule`

When a decision admits members, such as fixes, kinds, verbs, sections or consumers, or picks a
mechanism to meet a requirement, the head's title states the property that admits a member, in its
argument's own terms, and its body names the members built as what the rule admits today. A set
closed on purpose keeps its list in the title, with the sentence that argues the closure in the
body. A title states no more than its argument argues.

A count or a list in a title goes stale at the next member, and every later member then reads as a
change of the decision, with its reversal, its slug rename and its tripwire rename, against
`goal@knowledge-architect@agents-get-a-complete-workflow`, which aims for fewer decisions reversed
by accident. The nearest rival, the instance in the title, has that cost: an audit of this
repository's design heads at commit bc22d4d, reading every head and its history, finds the instance
stated as the rule in 27 of its 189 heads, and its history holds rewrites of such heads that a
member the argument already admitted forced. The figure is re-taken by a fresh read-only audit of
the heads at that commit against this rule. A review repair that moves a member
into a title makes the defect: "the title now states it". The opposite defect, a title
wider than its argument, caused reversals too, which the last sentence of the rule guards. A title
stating a rule alone would let a session add members the owner never saw, so it holds only with the
routing of `design@agent-skills@extension-follows-the-ground`.

The design this decision did not keep: `rejected@agent-skills@title-states-built-instance`.

### A head holds one decision, and parts that lose to different nearest rivals are separate heads `##one-decision-per-head`

Two statements are one decision when they answer one question and lose to the same nearest rival.
A rule's exception, its parameter and its delivery belong to its head, since none means anything
without the rule, unless one loses to a nearest rival of its own. Two statements that lose to different nearest rivals, or whose arguments share no
premise, are two decisions, and each takes a head of its own when it passes an entry test, and a
comment at its code when it passes none. A title joined by "and" over two decisions shows a bundle,
and rewording the title to state both does not make them one. A member of a set the title's rule
admits is no decision, so a member missing from a title is no finding.

The owner raised the question: "I'm not certain there is anything in the decision record skill that
gives a split criteria for design heads." A rule that only asks a head carrying several decisions
to pass "the test for each one, or is split", and rewords a title to state an addition beyond it,
gives no test: an audit of the design heads at commit bc22d4d, re-taken by a fresh read-only audit
of the heads at that commit against this rule, finds about one head in five holding two or more
decisions, and every split in this repository's history up to that commit is one a title false of
part of its body triggered, none a criterion.
The nearest rival, the test
"would reversing one part leave the other standing?", splits a rule from its exception, which always
passes it. The cost is
more heads, not more decisions: the owner's words, "We are not adding content and arguments, only
slugs and independent heads", and the entry tests still judge each part, so the decisions recorded
stay few, per `design@agent-skills@a-head-is-owed-by-an-entry-test`.

The design this decision did not keep: `rejected@agent-skills@head-split-by-independent-reversal`.

### A member that a head's argument covers is recorded directly, and one that the owner's words name goes to the owner `##extension-follows-the-ground`

A new member of a head, which the head's rule admits and its body does not name, is routed by the
head's ground, as `primer@design-heads` states it. Where the part of the head that admits it stands
on its argument, and the argument covers the member, it is recorded directly. Where that part's
ground is the owner's words and they state the rule, the member is within the ruling and is
recorded directly. Where the owner's words name the members, it goes to the owner as one proposal,
which owes no reversal. A member the rule does not admit, or the argument does not cover, is a
change of the decision, per `design@agent-skills@new-or-reshaped-head-needs-design`.

The owner approves an argued decision on its argument, so a member the argument covers is within
what was approved, and a member the owner's own words enumerate is not, per
`goal@knowledge-architect@the-owner-decides`. The head shows its ground, so the route is read from
the head itself. The nearest rival, routing by the scope of the owner's approval, needs that scope
found in the history of each head, often over several commits. Sending every new member through
the design skill costs a reversal for an ordinary extension, which
`design@agent-skills@title-states-the-rule` measured. The risk is a session judging "the argument
covers it" too widely; the design-conformance reviewer reads every plan document against the heads.

The design this decision did not keep: `rejected@agent-skills@member-routed-by-approval-scope`.

### A head that a change touches is brought to the rules on heads in that change `##existing-heads-on-touch`

A head that a change touches is brought, in that change, to the rules of `primer@design-heads` and `skill@knowledge-architect-decision-recording@head-shape`:
its title to the rule it argues, each decision it bundles to a head of its own, its ground to its
argument. Where that would widen what the owner's words in it approved, as a title moved from the
members the owner named to the rule, the change goes to the owner as one proposal.

**Re-pointing a reference in a head does not touch it.** The edit states nothing new in the head,
and a change that re-points references across the record, as a renamed entry or a migrated
reference form does, would otherwise owe the repair of every head it passes through, which is a
project audit's scope, not that change's. Adding a reference, or rewording a sentence, is more
than re-pointing, and so is removing one, on the owner's word.

The rules apply at the moment a session already reads the head and changes what it says, so a head is repaired at
the cost of one read. The whole record is repaired at once only by a project audit, on the
occasions of `design@agent-skills@audit-triggers`, per
`design@agent-skills@audit-repairs-the-whole-record`.

### A write into a design home, or an edit of agent-facing text a head describes, loads the decision-recording skill first, with no condition judged before it `##design-home-write-loads-recording`

A session loads the decision-recording skill before it writes into a design home, whatever its
activity. A session editing agent-facing text, a skill, an agent or a `CLAUDE.md`, also loads it
before an edit of a text whose behaviour a head describes. The skill, with `primer@design-heads`,
then judges whether the edit contradicts a head, adds a member its argument does not cover, or
records a decision at all. The skill's description
states the first trigger, and reaches every session.
`skill@knowledge-architect-agent-configuration@admission-standard` states both, and reaches a
session editing agent-facing text, which is the one activity that asks for a search of the design homes before an edit.

A session can see both conditions without any standard: the file it is about to edit, and the
result of that search. A load conditioned on whether an edit contradicts a head or adds a member
fires only in a session that already applies those tests at the moment of writing; the skill holds
the tests, in `skill@knowledge-architect-decision-recording@what-earns-a-head`, beside the
recording procedures, so they reach the session at the moment of writing, per
`design@agent-skills@one-home-for-head-rules`. This serves `goal@knowledge-architect@the-owner-decides`
and `goal@knowledge-architect@design-is-recorded-with-its-arguments`.

The nearest rival restates the backstop's tests beside the condition at each site. Its copies are
written by hand and drift, where the skill's are a fragment the build writes, and it reaches only a session editing agent-facing
text, not an issue fix or a review repair that writes a head. The cost is one skill load on every
edit of a design home, rewordings included; a rewording records no decision and leaves by the last
sentence of the backstop of `primer@design-heads`.

### The design skill guards the outcome of a discussion, and its tables are a display `##outcome-over-display`

The threads, states and tables exist so the owner can keep track when an agent produces much
content at once. A discussion that reaches a well-argued outcome with an imperfect table has
worked. A finding about the skill is worth an instruction only if it corrupts the outcome: what
gets recorded, what gets built, or what the owner believes was decided. A finding that makes only an
intermediate display imperfect is not, since agent behaviour is not deterministic and an
instruction bought for display fidelity costs more rules than it saves. Together with
`design@agent-skills@expectation-set-bounds-scope`, this entry bounds which findings become text.
