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
per `design@model@installed-entities-from-the-tree`. A slug placeholder anywhere else fails the
build. A skill is `skills/<skill>/` with its files, an agent is `agents/<agent>.md`, a saved
workflow of Claude Code's Workflow tool is `workflows/<workflow>.js`, and the primer is `PRIMER.md`;
the build asserts a saved workflow's name, per
`design@agent-skills@saved-workflow-name-is-its-file-stem`. The build script,
`path@agent-skills@build.rs`, walks the directory, renders each file, removing its comments, filling
its delivery substitutions and inlining its snippets, and generates `FILES`, each entry the install
path and the text. It adds the installer's prefix to each skill directory, agent file and workflow
file on the way out, so the installed names are the namespace of
`design@check@owned-namespace-check`. A file the layout does not map fails the build.

The list is generated because a list written by hand drifts from the directory it lists, and the
drift ships: a skill missing from the list is never installed, and nothing reports it. `check`'s
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
delivered whole wherever it applies: a
pointer costs the session a decision to fetch it, which the owner believed sessions likely to skip
on the ground that they read the target earlier. The owner ruled that fragments become the main
delivery for standards and similar large blocks read whole at several places. One file copied by
the build also leaves no two statements of a standard to contradict, per
`design@agent-skills@no-two-installed-instructions-contradict`. The fragments today
are the tripwire standard and its lifecycle, the rules on design heads, per
`design@agent-skills@one-home-for-head-rules`, the admission standard, per
`design@agent-skills@admission-rules-home`, the retrospective's file format and kinds of defect, the
decision-recording skill's rules on the Component, rejected designs, the deliberation and a
reversal, the audit's outcomes, the rule that a skill is self-sufficient, and the trigger test; the
goal standard, per `design@agent-skills@goal-standard-rules`, hosted by the goal-setting skill
and the goal reviewer; and one rule written as a fragment from the start, the size cost of
agent-facing text, per `design@agent-skills@context-cost-of-agent-text`; the default shape of code,
per `design@agent-skills@default-code-shape`; the module guidelines, per
`design@agent-skills@module-split-guidelines`, hosted by the development skill, the code-shape
reviewer and the code-split scout; the rule on when a subject of a design home splits;
the warning that a shallow history miscounts a tripwire's horizon; and two blocks smaller than a
standard, per `design@agent-skills@component-skill-delivery`: the occasion of a Component's creation
or split, hosted by the design skill, the development skill and the code-split scout, and how to
split, the load of the component-creation skill with the signals of a wrong cut, hosted by the two
skills alone. The occasion is a fragment because the owner proposed a fragment on when to split
for the design skill; the rule admits no other block smaller than a standard, and the session
extended it to the development skill. How to split is a fragment of its own, at the cost of one more
fragment, on the owner's argument that a fragment named for when to split holds no content on how
to split. Its two rivals each leave content on how to split in the fragment on when to split: the
session's default, the scout dropped from the fragment's hosts, and the scout kept a host of the
whole, which besides loads into every run of it two sentences no act of it applies, the cost
`design@agent-skills@context-cost-of-agent-text` counts, and which its own text overrides. The
module guidelines are a fragment of their own, at the cost of one more fragment, because the scout
applies them and none of the default shape's test rules: the scout hosting the whole default shape
would load those rules into every run of it, the same cost, under a host rule narrowed to admit it,
which weakens the rule for every fragment; and the scout reading the default shape by a pointer
loses to this head's argument. Who decides each fragment's host list is
`design@agent-skills@host-lists-are-the-session-s`; the admission standard's four fragments
follow from its split by subsection, under the host rule. The expectation sets stay behind a pointer, since the reader needs one skill's set, which
varies per finding. The pointer it replaces had left the writer of about 35 tripwires of this
repository and thaum without the rule they broke. The snippet pass cannot carry prose: it runs last,
so a snippet keeps its `%%` lines and its placeholders, and every snippet is written into a new
project by `klarch init --rust`. A harness hook delivering pointed sections was defeated on its making
the shipped workflow depend on a hook one harness runs, which an environment of another harness
lacks; since `goal@serves-one-kind-of-project` asks every target project's
agents to run under the `claude` harness, that defeat is to be argued again, per
`issue@agent-skills@heads-argued-from-another-harness`.

### A fragment's host list is the session's, decided by the rule that a host applies the whole fragment, each change reported under "Decided" `##host-lists-are-the-session-s`

The session decides which texts host a fragment, by the rule of
`design@agent-skills@shared-text-fragment` that a text is a host only where its act applies the
whole fragment, and reports each change of a list under the turn's "Decided" section, per
`primer@decided-section`, where the owner can contest it. `FRAGMENT_HOSTS` of
`path@agent-skills@src/lib.rs` holds the lists, and its test fails while a list and the shipped texts
disagree.

The owner chose the lists decided by the session over lists reserved to the owner, which no goal
ranks. The rival, lists reserved to the owner, costs a question to the owner at
every change of a list. The shape chosen lets a list change before the owner has seen it, and
leaves the owner the report of each change and the right to contest it. The rule decides what a list holds, so a change the session
makes is checkable against the rule by any reader.

### Code that decides which lines are inlined reads each kind of placeholder through one reader `##one-placeholder-reader-per-kind`

A fragment placeholder is read by `fragment_placeholder` and a snippet placeholder by
`snippet_placeholder`, both of `path@agent-skills@src/render.rs`. Every code that decides which
lines are inlined finds them through its reader: the fragment pass, `render_fragments`; the snippet
pass of `path@agent-skills@build.rs`; the source view, `source_view`, which
`design@xtask@source-view-in-the-tool` prints; and the test
`every_host_ships_each_fragment_it_names` of `path@agent-skills@src/lib.rs`, which finds each
fragment's hosts. The view then inlines exactly the lines the build inlines, so a maintainer reading
it reads what ships. The checks that a fragment nests no placeholder, and that a view keeps none,
test a prefix alone, since they refuse anything that opens like one.

The rival, a reading of the grammar written again at each site, differs from the build's on any
line no test holds, and a tab before a placeholder or a space after it is such a line: the build
ships both as written. The test `a_line_the_build_does_not_read_as_a_placeholder_stays_as_written`
holds those two lines for the fragment pass and the view; a reading of its own in the view, with
none of those lines tested, passes every other test, as the commit "Repairs of the decision-record
review of the shared readers: a default awaiting the owner leaves the design home" records.

### The setup skill's blocks for a Rust project are snippets that a test checks and `klarch init --rust` writes `##rust-templates-one-home`

Each block `skill@knowledge-architect-setup@rust-project` shows is a file of
`path@agent-skills@snippets/`, inlined by the build, and exposed whole as `SNIPPETS`: the
workspace's Cargo.toml, the maintenance crate's Cargo.toml, the cargo aliases with the `[env]` tie,
the crate's main and the CI workflow. `klarch init --rust` writes all of them but the CI workflow,
per `design@create@rust-scaffold` and `design@create@init-writes-no-ci-workflow`, so a project gets the
text the skill shows. Each is checked by a test of the workspace: the main is compiled by an example
of the maintenance crate, the three TOML files by a test that builds a project `init --rust` wrote
and runs its checker, and the CI workflow's gates command by a test that parses it with the gates
library's flags. A block copied into a template of `check` would be one more text that drifts from
the skill, and a block in the skill alone is checked by nothing, so a change of the gates library or
of the checker's command line could leave it wrong while every gate passes.

### A saved workflow's `meta` name equals its installed file's stem, which the build asserts `##saved-workflow-name-is-its-file-stem`

The harness reads saved workflows directly under .claude/workflows/, no deeper, and calls each by
the name its `meta` declares, never by its file's name, as a probe of Claude Code measured, which
the message of the commit "The agentic-workflow axis's script ships as a saved workflow in the
installer's namespace" records. So the build fails unless that name equals the installed file's
stem, or the prefix the build adds to the file would namespace nothing, per
`design@check@owned-namespace-check`.

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
it, so it names no Component and no convention of this repository:
`goal@every-target-project-can-adopt-it` has the workflow hold nothing
particular to one target project. The one name it holds on purpose is the
workflow's own upstream repository, where a retrospective's findings on the workflow go, per
`design@agent-skills@retrospective-destination`. It cites no entry: a reference to an entry resolves
only against the tree it stands in, and no tree but this one holds this repository's entries. A path
or a home that every conforming project holds may be written as a reference: `path@*@README.md`
names each Component's own copy of a required document, per
`design@model@components-carry-the-same-documents`; `register@goal` names the root's goals home,
in whichever shape it has, in every installing project, since the root alone carries the register
and its home is named with no anchor, per `design@model@root-register-reference-has-no-anchor`; and
`path@plans@README.md` and `register@plans@spec` name a file and a home of the one plans directory,
whose path the checker fixes, per `design@model@plans-dir-fixed`. Each is true in every installing
project, and it shows the syntax the checker enforces there. Writing a per-Component path in that
syntax is the owner's proposal, on the argument that it promotes the same kind of syntax as what the
checker enforces in projects holding the installed skills. The same argument holds of the plans directory's paths and
homes, since the checker fixes that directory's path, and of a home named by its register, since the
checker refuses a home named by its path. A path that varies by project, and an illustration, are
placeholders in angle brackets. The command a project runs is written as the placeholder that the
install fills with the project's declared command, per `design@check@declared-command`.

**The shipped set may cite its own skills, agents and primer, and their sections**, as
`skill@<name>@<slug>` or `primer@<slug>`: every project serving `claude` holds them, installed, and
they are defined from its installed copies, per `design@model@installed-entities-from-the-tree`. A
test of the checker holds every such reference to resolve within the shipped set. It never cites
`instructions@<slug>`, whose slugs are each project's own.

content/ and the snippets are in the walk, so every reference they hold is checked against this
repository. A reference to an entry that resolves here passes the check and would dangle in every
installing project: the reviews of each change, and the release's hand check over the installed
copies, judge that, until a mechanical check exists, which
`issue@agent-skills@shipped-text-citing-an-entry-is-unchecked` tracks.

### Each installed instruction can be followed, and gives the result it intends, with what the project that installs it holds `##installed-text-works-with-what-an-installing-project-holds`

A target project other than this repository that installs the workflow holds none of this
repository's goals and record: none of the reasons recorded beside its code and its agent-facing
text, the `%%` comments of the installed text's source included, and none of its commit messages.
Each installed instruction can be followed, and gives the result it intends, with what such a
project holds: the installed text, the project's own instructions, goals and record, its code and
its measurements, the sources it follows, knowledge-architect's published documentation among them,
and its owner. That holds where the project's own goals and record say nothing on the instruction's
subject, and as far as the project follows what knowledge-architect asks. An edit of the installed
text is argued from the side of such a project, as `path@agent-skills@CLAUDE.md` says.

`goal@every-target-project-can-adopt-it` has the workflow hold nothing
particular to one target project, and this repository is one target project, so an instruction that
needs this repository's record holds something particular to it. The nearest rival is an
instruction that reads rightly only beside a reason this repository records, in a design head or in
a `%%` comment of its source: the session that edits the source holds that reason in its context,
and the instruction reads complete to it. An installing project holds neither: the build removes
the `%%` lines, per `design@agent-skills@shipped-text-line-comments`, and no tree but this one holds
this repository's entries, per `design@agent-skills@shipped-text-cites-no-entry`, which states what
the shipped text may name and cite.

### An installed skill or agent names only the installed set's skills and agents, and the project's own `##no-external-handoff`

An installed skill or agent hands off only to another skill or agent of the set its version ships,
or to the project's own skills. It never names a plugin or a skill from outside the set. A skill
outside the set may be absent in the project, or present at a version that contradicts the installed
one, and either makes the installed text depend on a skill from outside what knowledge-architect
provides and what the project adds, where `goal@agents-get-a-complete-workflow`
has an agent need no instruction from outside them.

The set is written across several changes before a version is released, so a skill may name a
member of the set by its installed name before that member lands. No release ships a name its set
does not hold.

### Each activity the workflow covers has one installed skill, cut by methodology, from which an agent carries the activity out `##one-skill-per-activity`

The installed set holds one skill for each activity the workflow covers, and no skill for two of
them. An activity is the scope over which a complete set of procedures makes sense, a sub-activity
included. Two tasks whose methodologies differ, their steps taken as a whole, are two activities,
even where they share part of a name or read the same texts; where the two tests disagree, the
methodologies decide. Each skill is written so that an agent carries its activity out from it, and
reads another installed skill only at a step that points to it, a moment at which the installed
text says to load that skill included: a skill per activity serves only where the activity is
carried out from it. `path@agent-skills@fragments/skill-self-sufficient.md` holds the test of each
pointer. An installed agent is no skill, so one that several skills dispatch stays one file of the
set.

`goal@agents-get-a-complete-workflow` has an agent carry each activity out with
the workflow and what its project adds, so an agent that meets an activity finds its instructions
in one place it can name, without knowing where else to look. The nearest rivals cut a skill by
another unit, by what it reads or by the artifact it writes, or put two activities in one skill. A
skill loads whole, so a skill that holds two activities loads the procedure of one into every
session of the other: the Component procedure written into the design skill would load into every
design discussion, most of which create no Component, per
`design@agent-skills@component-skill-delivery`. A skill cut by what it reads bundles two
methodologies: the review of the whole goal set reads a whole project, as an audit does, and no
goal merges from it on the session's word, where an audit's settled repairs do, per
`design@agent-skills@goal-set-review-in-goal-setting`.

### A skill is named by its activity, as a noun of one or two words `##naming-rule`

An installed skill is named by the activity it covers, as an activity noun in common usage, in one
or two words after the installer's prefix: planning, review, issue-tracking, decision-recording. The
unit is the activity because `design@agent-skills@one-skill-per-activity` gives each activity of the
workflow one skill. It is never named as an artifact the activity writes, because the text names
both: a skill called "decision-record" would share its name with the design homes and rejected
alternatives the design skill calls the decision record, and one called "open-issues" with the issue
directory. One exception holds: agent-configuration is named like the configuration it edits. Its
nearest rival, agent-setup, reads like the setup skill, and the installer's prefix makes the full
name of `skill@knowledge-architect-agent-configuration` read as a skill; the cost is that the name
is also the artifact's. A name built on a verb form, such as discussing-design-decisions, reads as a
sentence and grows long. That argument is the owner's: "naming an "action" makes the skill names
into full sentences, which is weird and often too long". The rule binds a project's own skills too,
through the agent-configuration skill.
